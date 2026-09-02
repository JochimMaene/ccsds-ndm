use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::{Ndm, Validate};

const OPM: &str = include_str!("../data/kvn/opm_g1.kvn");

#[test]
fn opm_validation_returns_the_first_error_in_model_order() {
    let mut opm = Opm::from_kvn(OPM).expect("fixture should parse");
    opm.id = Some("NOT_AN_OPM".into());
    opm.header.originator.clear();
    opm.body.segment.metadata.object_name.clear();
    opm.body.segment.data.state_vector.x.value = f64::NAN;

    assert_eq!(
        opm.validate().unwrap_err().field_path().as_deref(),
        Some("id")
    );
}

#[test]
fn fail_fast_validation_returns_the_first_error_from_the_same_order() {
    let mut opm = Opm::from_kvn(OPM).expect("fixture should parse");
    opm.header.originator.clear();
    opm.body.segment.metadata.object_name.clear();

    let first = opm.validate().expect_err("model should be invalid");
    assert_eq!(first.field_path().as_deref(), Some("header.originator"));
}

#[test]
fn maneuver_diagnostics_identify_the_offending_maneuver() {
    const TWO_MANEUVERS: &str = include_str!("../data/kvn/opm_g2.kvn");

    let mut opm = Opm::from_kvn(TWO_MANEUVERS).expect("fixture should parse");
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 2);
    opm.body.segment.data.maneuver_parameters[1]
        .man_delta_mass
        .value = 1.0;

    let error = opm
        .validate()
        .expect_err("a positive delta mass is invalid");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters[1].man_delta_mass")
    );

    let mut opm = Opm::from_kvn(TWO_MANEUVERS).expect("fixture should parse");
    opm.body.segment.data.maneuver_parameters[1]
        .man_ref_frame
        .push('\u{1}');
    let error = opm
        .to_kvn()
        .expect_err("a control character is not KVN-representable");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters[1].man_ref_frame")
    );
}
