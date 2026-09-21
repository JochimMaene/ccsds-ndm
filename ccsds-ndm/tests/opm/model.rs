use crate::common::{assert_validation_field, mutated};
use ccsds_ndm::common::StateVector;
use ccsds_ndm::error::ValidationError;
use ccsds_ndm::messages::opm::{KeplerianElements, Opm, OpmBody, OpmData, OpmMetadata, OpmSegment};
use ccsds_ndm::types::{
    Angle, AngleUnits, Distance, Gm, GmUnits, Inclination, NonNegativeDouble, PositionUnits,
    Velocity,
};
use ccsds_ndm::{Ndm, Validate};
const MINIMAL: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1000 [km]
Y = 2000 [km]
Z = 3000 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
"#;

#[test]
fn required_metadata_and_state_fields_cannot_be_omitted() {
    Opm::from_kvn(MINIMAL).unwrap();
    for key in [
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME",
        "TIME_SYSTEM",
        "EPOCH",
        "X",
        "Y",
        "Z",
        "X_DOT",
        "Y_DOT",
        "Z_DOT",
    ] {
        let line = MINIMAL
            .lines()
            .find(|line| line.starts_with(&format!("{key} =")))
            .unwrap();
        let invalid = mutated(MINIMAL, &format!("{line}\n"), "");
        assert_validation_field(&Opm::from_kvn(&invalid).unwrap_err(), key);
    }
}

#[test]
fn minimal_message_omits_optional_blocks_and_roundtrips() {
    let message = Opm::from_kvn(MINIMAL).unwrap();
    let data = &message.body.segment.data;
    assert!(data.keplerian_elements.is_none());
    assert!(data.spacecraft_parameters.is_none());
    assert!(data.covariance_matrix.is_none());
    assert!(data.maneuver_parameters.is_empty());
    assert_eq!(Opm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Opm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
}

fn sample_opm_kvn() -> String {
    r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
MESSAGE_ID = OPM 201113719185
COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED
OBJECT_NAME = OSPREY 5
OBJECT_ID = 2022-999A
CENTER_NAME = EARTH
REF_FRAME = ITRF1997
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514000 [km]
Y = 1239.647000 [km]
Z = -717.490000 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
MASS = 3000.000000 [kg]
SOLAR_RAD_AREA = 18.770000 [m**2]
SOLAR_RAD_COEFF = 1.000000
DRAG_AREA = 18.770000 [m**2]
DRAG_COEFF = 2.500000
"#
    .to_string()
}

#[test]
fn parse_opm_success() {
    let kvn = sample_opm_kvn();
    let opm = Opm::from_kvn(&kvn).expect("OPM parse failed");

    assert_eq!(opm.version, "3.0");
    assert_eq!(opm.header.originator, "JAXA");
    assert_eq!(opm.body.segment.metadata.object_name, "OSPREY 5");
    assert_eq!(opm.body.segment.data.state_vector.x.value, 6503.514);
    assert_eq!(
        opm.body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap()
            .mass
            .as_ref()
            .unwrap()
            .value,
        3000.0
    );
}

#[test]
fn parse_opm_with_maneuvers() {
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2000-06-03T05:33:00
ORIGINATOR = NASA
OBJECT_NAME = EUTELSAT W4
OBJECT_ID = 2000-028A
CENTER_NAME = EARTH
REF_FRAME = TOD
TIME_SYSTEM = UTC
EPOCH = 2000-06-03T00:00:00.000
X = 6655.9942 [km]
Y = -40218.5751 [km]
Z = -82.9177 [km]
X_DOT = 3.11548207 [km/s]
Y_DOT = 0.47042605 [km/s]
Z_DOT = -0.00101490 [km/s]
MASS = 1000.0 [kg]
MAN_EPOCH_IGNITION = 2000-06-03T04:23:00
MAN_DURATION = 1500.0 [s]
MAN_DELTA_MASS = -10.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 10.5 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2000-06-05T06:00:00
MAN_DURATION = 1500.0 [s]
MAN_DELTA_MASS = -10.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = -10.5 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#;
    let opm = Opm::from_kvn(kvn).expect("OPM maneuver parse failed");
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 2);
    assert_eq!(
        opm.body.segment.data.maneuver_parameters[0].man_dv_1.value,
        10.5
    );
    assert_eq!(
        opm.body.segment.data.maneuver_parameters[1].man_dv_1.value,
        -10.5
    );
}

#[test]
fn opm_maneuver_requires_mass_in_strict_mode() {
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514
Y = 1239.647
Z = -717.490
X_DOT = -0.873160
Y_DOT = 8.740420
Z_DOT = -4.191076
MAN_EPOCH_IGNITION = 2023-01-01T00:00:00
MAN_DURATION = 10.0
MAN_DELTA_MASS = -1.0
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1
MAN_DV_2 = 0.0
MAN_DV_3 = 0.0
"#;
    let err = Opm::from_kvn(kvn).unwrap_err();
    let ok = err.as_validation_error().is_some_and(|e| {
        matches!(
            e,
            ValidationError::MissingRequiredField { block, field, .. }
            if block.as_ref() == "Spacecraft Parameters" && field.as_ref() == "MASS"
        )
    });
    assert!(ok, "expected MASS missing validation error, got {err}");
}

#[test]
fn metadata_optional_ref_frame_epoch() {
    // XSD: REF_FRAME_EPOCH has minOccurs="0" - it's optional
    let kvn_without = MINIMAL;
    let opm = Opm::from_kvn(kvn_without).unwrap();
    assert!(opm.body.segment.metadata.ref_frame_epoch.is_none());

    let kvn_with = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1000 [km]
Y = 2000 [km]
Z = 3000 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
"#;
    let opm = Opm::from_kvn(kvn_with).unwrap();
    assert!(opm.body.segment.metadata.ref_frame_epoch.is_some());
}

#[test]
fn state_vector_all_mandatory() {
    // XSD: stateVectorType requires all position and velocity components
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 6503.514 [km]
Y = 1239.647 [km]
Z = -717.490 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
"#;
    let opm = Opm::from_kvn(kvn).unwrap();
    let sv = &opm.body.segment.data.state_vector;
    assert_eq!(sv.x.value, 6503.514);
    assert_eq!(sv.y.value, 1239.647);
    assert_eq!(sv.z.value, -717.490);
    assert_eq!(sv.x_dot.value, -0.873160);
    assert_eq!(sv.y_dot.value, 8.740420);
    assert_eq!(sv.z_dot.value, -4.191076);
}

#[test]
fn keplerian_with_true_anomaly() {
    // XSD: keplerianElementsType choice: TRUE_ANOMALY path
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
TRUE_ANOMALY = 270 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert!(kep.true_anomaly.is_some());
    assert!(kep.mean_anomaly.is_none());
    assert_eq!(kep.true_anomaly.as_ref().unwrap().value, 270.0);
}

#[test]
fn keplerian_with_mean_anomaly() {
    // XSD: keplerianElementsType choice: MEAN_ANOMALY path
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
MEAN_ANOMALY = 120 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert!(kep.mean_anomaly.is_some());
    assert!(kep.true_anomaly.is_none());
    assert_eq!(kep.mean_anomaly.as_ref().unwrap().value, 120.0);
}

#[test]
fn keplerian_eccentricity_zero_valid() {
    // XSD: nonNegativeDouble - minInclusive=0.0 (circular orbit)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.0
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.eccentricity, NonNegativeDouble::new(0.0).unwrap());
}

#[test]
fn keplerian_inclination_boundaries() {
    // XSD: inclinationType - 0 to 180 degrees inclusive
    let kvn_zero = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 0 [deg]
RA_OF_ASC_NODE = 0 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn_zero).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.inclination.angle.value, 0.0);

    let kvn_180 = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 180 [deg]
RA_OF_ASC_NODE = 0 [deg]
ARG_OF_PERICENTER = 0 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn_180).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.inclination.angle.value, 180.0);
}

#[test]
fn keplerian_angle_range_negative() {
    // XSD: angleRange - can be negative (minInclusive=-360.0)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = -180 [deg]
ARG_OF_PERICENTER = -90 [deg]
TRUE_ANOMALY = -45 [deg]
GM = 398600.4 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.ra_of_asc_node.value, -180.0);
    assert_eq!(kep.arg_of_pericenter.value, -90.0);
    assert_eq!(kep.true_anomaly.as_ref().unwrap().value, -45.0);
}

#[test]
fn keplerian_gm_positive() {
    // XSD: positiveDouble for GM - minExclusive=0.0
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"SEMI_MAJOR_AXIS = 7000 [km]
ECCENTRICITY = 0.001
INCLINATION = 45 [deg]
RA_OF_ASC_NODE = 90 [deg]
ARG_OF_PERICENTER = 180 [deg]
TRUE_ANOMALY = 0 [deg]
GM = 0.001 [km**3/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let kep = opm.body.segment.data.keplerian_elements.as_ref().unwrap();
    assert_eq!(kep.gm.value, 0.001);
}

#[test]
fn spacecraft_parameters_with_all_fields() {
    // XSD: spacecraftParametersType has MASS, SOLAR_RAD_AREA, SOLAR_RAD_COEFF, DRAG_AREA, DRAG_COEFF
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 500 [kg]
SOLAR_RAD_AREA = 10.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 8.0 [m**2]
DRAG_COEFF = 2.2
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let sp = opm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert_eq!(sp.mass.as_ref().unwrap().value, 500.0);
    assert_eq!(sp.solar_rad_area.as_ref().unwrap().value, 10.0);
    assert_eq!(
        sp.solar_rad_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(1.2).unwrap()
    );
    assert_eq!(sp.drag_area.as_ref().unwrap().value, 8.0);
    assert_eq!(
        sp.drag_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(2.2).unwrap()
    );
}

#[test]
fn spacecraft_zero_coefficients() {
    // XSD: nonNegativeDouble allows 0 for coefficients
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 100 [kg]
SOLAR_RAD_COEFF = 0.0
DRAG_COEFF = 0.0
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let sp = opm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert_eq!(
        sp.solar_rad_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(0.0).unwrap()
    );
    assert_eq!(
        sp.drag_coeff.as_ref().unwrap(),
        &NonNegativeDouble::new(0.0).unwrap()
    );
}

#[test]
fn covariance_matrix_present() {
    // XSD: covarianceMatrixType when present
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"COV_REF_FRAME = RSW
CX_X = 1.0e-6 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0e-6 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0e-6 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 1.0e-9 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 1.0e-9 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 1.0e-9 [km**2/s**2]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let cov = opm.body.segment.data.covariance_matrix.as_ref().unwrap();
    assert!(cov.cov_ref_frame.is_some());
}

#[test]
fn single_maneuver() {
    // XSD: maneuverParametersType with mandatory fields
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 1);
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_duration.value, 100.0);
    assert_eq!(man.man_delta_mass.value, -5.0);
}

#[test]
fn multiple_maneuvers_unbounded() {
    // XSD: maxOccurs="unbounded" allows multiple maneuvers
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2023-01-03T00:00:00
MAN_DURATION = 50 [s]
MAN_DELTA_MASS = -2.5 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.05 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
MAN_EPOCH_IGNITION = 2023-01-04T00:00:00
MAN_DURATION = 75 [s]
MAN_DELTA_MASS = -3.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.0 [km/s]
MAN_DV_2 = 0.1 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 3);
}

#[test]
fn maneuver_delta_mass_zero_allowed() {
    // XSD: deltamassTypeZ is nonPositiveDouble (≤0), so zero is allowed
    // This represents attitude maneuvers that don't use propellant
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = 0.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    // XSD allows zero for attitude maneuvers
    let opm = Opm::from_kvn(kvn).unwrap();
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_delta_mass.value, 0.0);
}

#[test]
fn maneuver_delta_mass_positive_rejected() {
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = 5.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    // Should fail - positive MAN_DELTA_MASS is not allowed (must be <= 0)
    crate::common::assert_validation_field(&Opm::from_kvn(kvn).unwrap_err(), "DeltaMassZ");
}

#[test]
fn maneuver_delta_mass_negative() {
    // XSD: deltamassTypeZ - negative values are valid (mass loss)
    let kvn = &format!(
        "{MINIMAL}{}",
        r#"MASS = 3000.000000 [kg]
MAN_EPOCH_IGNITION = 2023-01-02T00:00:00
MAN_DURATION = 100 [s]
MAN_DELTA_MASS = -100.0 [kg]
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1 [km/s]
MAN_DV_2 = 0.0 [km/s]
MAN_DV_3 = 0.0 [km/s]
"#
    );
    let opm = Opm::from_kvn(kvn).unwrap();
    let man = &opm.body.segment.data.maneuver_parameters[0];
    assert_eq!(man.man_delta_mass.value, -100.0);
}

#[test]
fn kvn_roundtrip() {
    let kvn = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 6503.514 [km]
Y = 1239.647 [km]
Z = -717.490 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
"#;
    let opm = Opm::from_kvn(kvn).unwrap();
    let output = opm.to_kvn().unwrap();

    // Parse output again
    let opm2 = Opm::from_kvn(&output).unwrap();
    assert_eq!(opm2, opm);
}

#[test]
fn keplerian_elements_validation() {
    use ccsds_ndm::Validate;
    let mut kep = KeplerianElements::builder()
        .semi_major_axis(Distance::new(7000.0, Some(PositionUnits::Km)))
        .eccentricity(NonNegativeDouble::new(0.001).unwrap())
        .inclination(Inclination::new(45.0, Some(AngleUnits::Deg)).unwrap())
        .ra_of_asc_node(Angle::new(90.0, Some(AngleUnits::Deg)).unwrap())
        .arg_of_pericenter(Angle::new(180.0, Some(AngleUnits::Deg)).unwrap())
        .gm(Gm::new(398600.44, Some(GmUnits::Km3PerS2)).unwrap())
        .build();

    // Neither anomaly
    assert_validation_field(&kep.validate().unwrap_err(), "TRUE_ANOMALY");

    // Both anomalies
    kep.true_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    kep.mean_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    assert_validation_field(&kep.validate().unwrap_err(), "TRUE_ANOMALY");

    // Exactly one (true)
    kep.mean_anomaly = None;
    assert!(kep.validate().is_ok());

    // Exactly one (mean)
    kep.true_anomaly = None;
    kep.mean_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    assert!(kep.validate().is_ok());
}

#[test]
fn opm_data_validation() {
    use ccsds_ndm::Validate;
    let mut data = OpmData::builder()
        .state_vector(
            StateVector::builder()
                .epoch("2023-01-01T00:00:00".parse().unwrap())
                .x(Distance::new(1.0, None))
                .y(Distance::new(1.0, None))
                .z(Distance::new(1.0, None))
                .x_dot(Velocity::new(1.0, None))
                .y_dot(Velocity::new(1.0, None))
                .z_dot(Velocity::new(1.0, None))
                .build(),
        )
        .build();

    assert!(data.validate().is_ok());

    // With invalid KeplerianElements
    data.keplerian_elements = Some(
        KeplerianElements::builder()
            .semi_major_axis(Distance::new(7000.0, Some(PositionUnits::Km)))
            .eccentricity(NonNegativeDouble::new(0.001).unwrap())
            .inclination(Inclination::new(45.0, Some(AngleUnits::Deg)).unwrap())
            .ra_of_asc_node(Angle::new(90.0, Some(AngleUnits::Deg)).unwrap())
            .arg_of_pericenter(Angle::new(180.0, Some(AngleUnits::Deg)).unwrap())
            .gm(Gm::new(398600.44, Some(GmUnits::Km3PerS2)).unwrap())
            .build(),
    );
    assert_validation_field(&data.validate().unwrap_err(), "TRUE_ANOMALY");
}

#[test]
fn opm_serialization_gaps() {
    use ccsds_ndm::common::OdmHeader;
    let opm = Opm::builder()
        .version("3.0")
        .header(
            OdmHeader::builder()
                .creation_date("2023-01-01T00:00:00".parse().unwrap())
                .originator("TEST")
                .build(),
        )
        .body(
            OpmBody::builder()
                .segment(
                    OpmSegment::builder()
                        .metadata(
                            OpmMetadata::builder()
                                .object_name("SAT")
                                .object_id("1")
                                .center_name("EARTH")
                                .ref_frame("GCRF")
                                .ref_frame_epoch("2000-01-01T12:00:00".parse().unwrap())
                                .time_system("UTC")
                                .build(),
                        )
                        .data(
                            OpmData::builder()
                                .state_vector(
                                    StateVector::builder()
                                        .epoch("2023-01-01T00:00:00".parse().unwrap())
                                        .x(Distance::new(1.0, None))
                                        .y(Distance::new(1.0, None))
                                        .z(Distance::new(1.0, None))
                                        .x_dot(Velocity::new(1.0, None))
                                        .y_dot(Velocity::new(1.0, None))
                                        .z_dot(Velocity::new(1.0, None))
                                        .build(),
                                )
                                .keplerian_elements(
                                    KeplerianElements::builder()
                                        .semi_major_axis(Distance::new(7000.0, None))
                                        .eccentricity(NonNegativeDouble::new(0.0).unwrap())
                                        .inclination(Inclination::new(0.0, None).unwrap())
                                        .ra_of_asc_node(Angle::new(0.0, None).unwrap())
                                        .arg_of_pericenter(Angle::new(0.0, None).unwrap())
                                        .mean_anomaly(Angle::new(0.0, None).unwrap())
                                        .gm(Gm::new(398600.44, None).unwrap())
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    let kvn = opm.to_kvn().unwrap();
    assert!(kvn.contains("REF_FRAME_EPOCH"));
    assert!(kvn.contains("2000-01-01T12:00:00"));
    assert!(kvn.contains("MEAN_ANOMALY"));
}

#[test]
fn opm_minimal_data_gaps() {
    use ccsds_ndm::common::OdmHeader;
    // Minimal OPM without Keplerian Elements or optional Spacecraft Params
    let opm = Opm::builder()
        .version("3.0")
        .header(
            OdmHeader::builder()
                .creation_date("2023-01-01T00:00:00".parse().unwrap())
                .originator("TEST")
                .build(),
        )
        .body(
            OpmBody::builder()
                .segment(
                    OpmSegment::builder()
                        .metadata(
                            OpmMetadata::builder()
                                .object_name("SAT")
                                .object_id("1")
                                .center_name("EARTH")
                                .ref_frame("GCRF")
                                .time_system("UTC")
                                .build(),
                        )
                        .data(
                            OpmData::builder()
                                .state_vector(
                                    StateVector::builder()
                                        .epoch("2023-01-01T00:00:00".parse().unwrap())
                                        .x(Distance::new(1.0, None))
                                        .y(Distance::new(1.0, None))
                                        .z(Distance::new(1.0, None))
                                        .x_dot(Velocity::new(1.0, None))
                                        .y_dot(Velocity::new(1.0, None))
                                        .z_dot(Velocity::new(1.0, None))
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    let kvn = opm.to_kvn().unwrap();
    assert!(!kvn.contains("SEMI_MAJOR_AXIS"));
    assert!(opm.validate().is_ok());
}
