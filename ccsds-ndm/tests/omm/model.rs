use ccsds_ndm::error::ValidationError;
use ccsds_ndm::messages::omm::{
    BStar, InvErUnits, MeanMotionDDot, MeanMotionDot, RevPerDay2Units, RevPerDay3Units,
    RevPerDayUnits, TleParameters,
};
use ccsds_ndm::types::M2kg;

/// `TleParameters::validate` is public, so its theory dispatch is checked directly: the
/// error variants the KVN table only sees as text, the theories that impose no requirement,
/// and the satisfied SGP4-XP case.
#[test]
fn tle_parameters_validate_dispatches_on_theory() {
    let base = || {
        TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build()
    };

    let mut conflicting = base();
    conflicting.bstar = Some(BStar::new(0.0001, None));
    conflicting.bterm = Some(M2kg::new(0.01, None));
    let error = conflicting.validate("SGP4").unwrap_err();
    assert!(error.as_validation_error().is_some_and(|error| {
        matches!(error, ValidationError::Conflict { fields, .. }
                if fields.iter().any(|field| field.as_ref() == "BSTAR")
                    && fields.iter().any(|field| field.as_ref() == "BTERM"))
    }));

    let mut conflicting = base();
    conflicting.mean_motion_ddot = Some(MeanMotionDDot::new(0.0, None));
    conflicting.agom = Some(M2kg::new(0.001, None));
    let error = conflicting.validate("SGP").unwrap_err();
    assert!(error.as_validation_error().is_some_and(|error| {
        matches!(error, ValidationError::Conflict { fields, .. }
                if fields.iter().any(|field| field.as_ref() == "MEAN_MOTION_DDOT")
                    && fields.iter().any(|field| field.as_ref() == "AGOM"))
    }));

    // An unrecognised theory imposes nothing beyond the conflict rules.
    assert!(base().validate("UNKNOWN").is_ok());

    let mut satisfied = base();
    satisfied.bterm = Some(M2kg::new(0.01, None));
    satisfied.agom = Some(M2kg::new(1.0, None));
    satisfied
        .validate("SGP4-XP")
        .expect("BTERM and AGOM satisfy SGP4-XP");
}

#[test]
fn rev_per_day_units_display_all() {
    assert_eq!(format!("{}", RevPerDayUnits::RevPerDay), "rev/day");
    assert_eq!(format!("{}", RevPerDay2Units::RevPerDay2), "rev/day**2");
    assert_eq!(format!("{}", RevPerDay3Units::RevPerDay3), "rev/day**3");
}

#[test]
fn omm_units_parsing() {
    use std::str::FromStr;
    assert!(InvErUnits::from_str("1/ER").is_ok());
    let error = InvErUnits::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDayUnits::from_str("rev/day").is_ok());
    assert!(RevPerDayUnits::from_str("REV/DAY").is_ok());
    let error = RevPerDayUnits::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDay2Units::from_str("rev/day**2").is_ok());
    assert!(RevPerDay2Units::from_str("REV/DAY**2").is_ok());
    let error = RevPerDay2Units::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDay3Units::from_str("rev/day**3").is_ok());
    assert!(RevPerDay3Units::from_str("REV/DAY**3").is_ok());
    let error = RevPerDay3Units::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");
}
