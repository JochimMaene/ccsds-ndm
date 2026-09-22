use crate::common::{assert_rejects, assert_validation_field, mutated};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata};
use ccsds_ndm::types::{Epoch, InterpolationDegree};
use ccsds_ndm::{Ndm, Validate};
use std::num::NonZeroU32;
#[test]
fn covariance_validation_accepts_negative_values_but_rejects_non_finite_values() {
    let negative_variance = mutated(
        include_str!("../../data/kvn/oem_g13.kvn"),
        "3.3313494e-04",
        "-3.3313494e-04",
    );
    let mut oem = Oem::from_kvn(&negative_variance)
        .expect("OEM does not specify a sign constraint for covariance values");
    oem.to_kvn()
        .expect("negative covariance values must generate");

    oem.body.segment[0].data.covariance_matrix[0].cx_x.value = f64::NAN;
    assert_rejects(&oem, "CX_X");
}

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
