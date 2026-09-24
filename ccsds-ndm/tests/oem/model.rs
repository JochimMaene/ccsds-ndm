use crate::common::assert_validation_field;
use ccsds_ndm::messages::oem::{OemBody, OemData, OemMetadata};
use ccsds_ndm::types::{Epoch, InterpolationDegree};
use ccsds_ndm::Validate;
use std::num::NonZeroU32;

#[test]
fn validates_required_model_invariants() {
    let mut metadata = OemMetadata::builder()
        .object_name("SAT")
        .object_id("1")
        .center_name("EARTH")
        .ref_frame("GCRF")
        .time_system("UTC")
        .start_time(Epoch::new("2023-01-01T12:00:00").unwrap())
        .stop_time(Epoch::new("2023-01-01T13:00:00").unwrap())
        .build();

    metadata.interpolation = Some("LAGRANGE".into());
    assert_validation_field(
        &metadata.validate().unwrap_err(),
        "INTERPOLATION_DEGREE (required when INTERPOLATION is present)",
    );

    metadata.interpolation_degree = Some(InterpolationDegree::from(NonZeroU32::new(5).unwrap()));
    assert!(metadata.validate().is_ok());
    assert_validation_field(
        &OemData::builder().build().validate().unwrap_err(),
        "stateVector (at least one required)",
    );
    assert_validation_field(
        &OemBody { segment: vec![] }.validate().unwrap_err(),
        "segment (at least one required)",
    );
}
