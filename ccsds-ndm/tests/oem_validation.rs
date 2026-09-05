use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::types::Epoch;
use ccsds_ndm::{Ndm, Validate};

const MULTI_SEGMENT: &str = include_str!("../data/kvn/oem_g11.kvn");
const XML: &str = include_str!("../data/xml/oem_g14.xml");

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
fn consecutive_useable_spans_may_touch_but_not_overlap() {
    let mut message = Oem::from_kvn(MULTI_SEGMENT).unwrap();
    let second = &mut message.body.segment[1].metadata;
    second.start_time = epoch("2019-12-28T21:00:00.000");
    message
        .validate()
        .expect("total spans may overlap when useable spans do not");

    message.body.segment[1].metadata.useable_start_time = Some(epoch("2019-12-28T21:23:00.331"));
    message
        .validate()
        .expect("a shared useable-span endpoint is allowed");

    message.body.segment[1].metadata.useable_start_time = Some(epoch("2019-12-28T21:22:00.331"));
    let error = message
        .validate()
        .expect_err("consecutive useable spans must not overlap");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[1].metadata.useable_start_time")
    );
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
    let error = message.validate().unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert!(error.to_string().contains("START_TIME"));

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
    message.body.segment[0].data.state_vector[2].epoch = epoch("2019-12-18T12:00:30.331");
    let error = message.validate().unwrap_err();
    assert!(error.to_string().contains("nondecreasing"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.state_vector[2].epoch")
    );
}

#[test]
fn covariance_time_tags_are_strictly_increasing() {
    let mut message = Oem::from_xml(XML).unwrap();
    let covariance = message.body.segment[0].data.covariance_matrix[0].clone();
    message.body.segment[0]
        .data
        .covariance_matrix
        .push(covariance);
    let error = message.validate().unwrap_err();
    assert!(error.to_string().contains("strictly increasing"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].data.covariance_matrix[1].epoch")
    );
}

#[test]
fn validation_stops_at_the_first_failure() {
    let mut message = Oem::from_xml(XML).unwrap();
    let metadata = &mut message.body.segment[0].metadata;
    metadata.object_name.clear();
    metadata.useable_start_time = Some(epoch("2019-12-01T00:00:00"));
    metadata.useable_stop_time = Some(epoch("2019-11-01T00:00:00"));
    message.body.segment[0].data.state_vector[0].x.value = f64::NAN;

    let first = message.validate().unwrap_err();
    assert_eq!(first.code(), Some("validation.missing_required_field"));
    assert_eq!(
        first.field_path().as_deref(),
        Some("body.segment[0].metadata.object_name")
    );
}
