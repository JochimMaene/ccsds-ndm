use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::{Ndm, Validate};

const OPM: &str = include_str!("../../data/kvn/opm_g1.kvn");

#[test]
fn opm_validation_collects_independent_errors_in_model_order() {
    let mut opm = Opm::from_kvn(OPM).expect("fixture should parse");
    opm.id = Some("NOT_AN_OPM".into());
    opm.header.originator.clear();
    opm.body.segment.metadata.object_name.clear();
    opm.body.segment.data.state_vector.x.value = f64::NAN;

    let errors = opm
        .validation_errors()
        .expect("validation should not fail mechanically");
    let diagnostics: Vec<_> = errors
        .iter()
        .map(|error| (error.code(), error.field_path()))
        .collect();

    assert_eq!(
        diagnostics,
        vec![
            (Some("validation.invalid_value"), Some("id".to_owned())),
            (
                Some("validation.missing_required_field"),
                Some("header.originator".to_owned())
            ),
            (
                Some("validation.missing_required_field"),
                Some("body.segment.metadata.object_name".to_owned())
            ),
            (
                Some("validation.invalid_value"),
                Some("body.segment.data.state_vector.x".to_owned())
            ),
        ]
    );
}

#[test]
fn fail_fast_validation_returns_the_first_error_from_the_same_order() {
    let mut opm = Opm::from_kvn(OPM).expect("fixture should parse");
    opm.header.originator.clear();
    opm.body.segment.metadata.object_name.clear();

    let first = opm.validate().expect_err("model should be invalid");
    assert_eq!(first.field_path().as_deref(), Some("header.originator"));
    assert_eq!(
        opm.validation_errors().unwrap()[0].field_path().as_deref(),
        Some("header.originator")
    );
}
