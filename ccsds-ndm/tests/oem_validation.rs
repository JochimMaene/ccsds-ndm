use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::types::Epoch;

const MULTI_SEGMENT: &str = include_str!("../../data/kvn/oem_g11.kvn");
const XML: &str = include_str!("../../data/xml/oem_g14.xml");

fn epoch(value: &str) -> Epoch {
    value.parse().unwrap()
}

#[test]
fn all_segments_must_describe_one_object_and_one_time_system() {
    let mut message = Oem::from_kvn(MULTI_SEGMENT).unwrap();
    assert!(message.body.segment.len() > 1);
    message.body.segment[1].metadata.time_system = "TAI".into();
    assert_eq!(
        message.validate().unwrap_err().field_path().as_deref(),
        Some("body.segment[1].metadata.time_system")
    );

    let mut message = Oem::from_kvn(MULTI_SEGMENT).unwrap();
    message.body.segment[1].metadata.object_id = "DIFFERENT".into();
    assert!(message.validate().is_err());
}

#[test]
fn oem_time_tags_are_absolute_and_metadata_ranges_are_consistent() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[0].epoch = epoch("123.5");
    assert_eq!(
        message.validate().unwrap_err().field_path().as_deref(),
        Some("body.segment[0].data.state_vector[0].epoch")
    );

    let mut message = Oem::from_xml(XML).unwrap();
    let metadata = &mut message.body.segment[0].metadata;
    std::mem::swap(&mut metadata.start_time, &mut metadata.stop_time);
    let errors = message.validation_errors().unwrap();
    assert!(errors.iter().any(|error| {
        error.code() == Some("validation.invalid_value") && error.to_string().contains("START_TIME")
    }));

    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].metadata.useable_start_time = Some(epoch("2019-12-01T00:00:00"));
    assert!(message.validate().is_err());
}

#[test]
fn ephemeris_records_are_in_span_and_nondecreasing() {
    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[0].epoch = epoch("2019-12-01T00:00:00");
    assert!(message.validate().is_err());

    let mut message = Oem::from_xml(XML).unwrap();
    message.body.segment[0].data.state_vector[1].epoch = epoch("2019-12-17T00:00:00");
    let errors = message.validation_errors().unwrap();
    assert!(errors
        .iter()
        .any(|error| error.to_string().contains("nondecreasing")));
    assert!(errors.iter().any(|error| {
        error.field_path().as_deref() == Some("body.segment[0].data.state_vector[1].epoch")
    }));
}

#[test]
fn covariance_time_tags_are_strictly_increasing() {
    let mut message = Oem::from_xml(XML).unwrap();
    let covariance = message.body.segment[0].data.covariance_matrix[0].clone();
    message.body.segment[0]
        .data
        .covariance_matrix
        .push(covariance);
    let errors = message.validation_errors().unwrap();
    assert!(errors
        .iter()
        .any(|error| error.to_string().contains("strictly increasing")));
    assert!(errors.iter().any(|error| {
        error.field_path().as_deref() == Some("body.segment[0].data.covariance_matrix[1].epoch")
    }));
}

#[test]
fn validation_collects_independent_failures_without_stopping_at_the_first() {
    let mut message = Oem::from_xml(XML).unwrap();
    let metadata = &mut message.body.segment[0].metadata;
    metadata.object_name.clear();
    metadata.useable_start_time = Some(epoch("2019-12-01T00:00:00"));
    metadata.useable_stop_time = Some(epoch("2019-11-01T00:00:00"));
    message.body.segment[0].data.state_vector[0].x.value = f64::NAN;

    let errors = message.validation_errors().unwrap();
    assert!(
        errors.len() >= 4,
        "independent errors were lost: {errors:?}"
    );
    assert_eq!(
        errors[0].code(),
        Some("validation.missing_required_field"),
        "errors should remain in model order"
    );
    assert_eq!(
        errors[0].field_path().as_deref(),
        Some("body.segment[0].metadata.object_name")
    );

    let first = message.validate().unwrap_err();
    assert_eq!(first.code(), errors[0].code());
    assert_eq!(first.field_path(), errors[0].field_path());
}
