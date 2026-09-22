use crate::common::mutated;
use crate::{KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, FormatError};
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::types::ControlledType;
use ccsds_ndm::Ndm;
#[test]
fn rdm_root_attributes() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(rdm.id, Some("CCSDS_RDM_VERS".to_string()));
    assert_eq!(rdm.version, "1.0");
}

#[test]
fn rdm_header_mandatory_fields() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = ESA
MESSAGE_ID = ESA-20231113-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.header.message_id, "ESA-20231113-001");
}

#[test]
fn rdm_header_optional_comments() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert!(rdm.header.comment.is_empty());
}

#[test]
fn rdm_metadata_mandatory_fields() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = SENTINEL-1A
INTERNATIONAL_DESIGNATOR = 2014-016A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let meta = &rdm.body.segment.metadata;
    assert_eq!(meta.object_name, "SENTINEL-1A");
    assert_eq!(meta.international_designator, "2014-016A");
    assert_eq!(meta.center_name, "EARTH");
    assert_eq!(meta.time_system, "TAI");
}

#[test]
fn rdm_controlled_type_values() {
    for (val, expected) in [
        ("YES", ControlledType::Yes),
        ("yes", ControlledType::Yes),
        ("NO", ControlledType::No),
        ("no", ControlledType::No),
        ("UNKNOWN", ControlledType::Unknown),
        ("unknown", ControlledType::Unknown),
    ] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = {}
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
            val
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(rdm.body.segment.metadata.controlled_reentry, expected);
    }
}

#[test]
fn rdm_object_type_enum() {
    for obj_type in ["PAYLOAD", "ROCKET BODY", "DEBRIS", "UNKNOWN", "OTHER"] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
OBJECT_TYPE = {}
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
            obj_type
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.object_type.unwrap().to_string(),
            obj_type
        );
    }
}

#[test]
fn rdm_intrack_thrust_yesno() {
    for val in ["YES", "NO"] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
INTRACK_THRUST = {}
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
            val
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(
            rdm.body
                .segment
                .metadata
                .intrack_thrust
                .unwrap()
                .to_string(),
            val
        );
    }
}

#[test]
fn rdm_metadata_optional_fields() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
OBJECT_OWNER = ESA
OBJECT_OPERATOR = EUMETSAT
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
EPHEMERIS_NAME = NONE
GRAVITY_MODEL = EGM-96: 36D 360
ATMOSPHERIC_MODEL = NRLMSISE-00
SOLAR_FLUX_PREDICTION = PREDICTED
N_BODY_PERTURBATIONS = MOON, SUN
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
DRAG_PARAMETERS_SOURCE = OD
DRAG_PARAMETERS_ALTITUDE = 200 [km]
PREVIOUS_MESSAGE_ID = PREV-001
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let meta = &rdm.body.segment.metadata;
    assert_eq!(meta.catalog_name, Some("SATCAT".to_string()));
    assert_eq!(meta.object_designator, Some("12345".to_string()));
    assert_eq!(meta.object_owner, Some("ESA".to_string()));
    assert_eq!(meta.ref_frame, Some("EME2000".to_string()));
    assert_eq!(meta.gravity_model, Some("EGM-96: 36D 360".to_string()));
    assert_eq!(meta.atmospheric_model, Some("NRLMSISE-00".to_string()));
}

#[test]
fn rdm_atmospheric_mandatory_fields() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 23.5 [d]
REENTRY_ALTITUDE = 150.0 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
    assert!((atmos.orbit_lifetime.value - 23.5).abs() < 1e-9);
    assert!((atmos.reentry_altitude.value - 150.0).abs() < 1e-9);
}

#[test]
fn rdm_day_interval_units_required() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_WINDOW_START = 4.0 [d]
ORBIT_LIFETIME_WINDOW_END = 7.0 [d]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
    assert!(atmos.orbit_lifetime_window_start.is_some());
    assert!(atmos.orbit_lifetime_window_end.is_some());
}

#[test]
fn rdm_percentage_type() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_CONFIDENCE_LEVEL = 95.0 [%]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
    assert!(atmos.orbit_lifetime_confidence_level.is_some());
}

#[test]
fn rdm_probability_type_range() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.5
PROBABILITY_OF_BURN_UP = 0.0
PROBABILITY_OF_CASUALTY = 1.0
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let ground = rdm
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap();
    assert!((ground.probability_of_impact.as_ref().unwrap().value - 0.5).abs() < 1e-9);
    assert!((ground.probability_of_burn_up.as_ref().unwrap().value - 0.0).abs() < 1e-9);
    assert!((ground.probability_of_casualty.as_ref().unwrap().value - 1.0).abs() < 1e-9);
}

#[test]
fn rdm_latitude_range() {
    for lat in ["-90.0", "0.0", "45.5", "90.0"] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_REF_FRAME = ITRF
NOMINAL_IMPACT_LON = 0
NOMINAL_IMPACT_LAT = {}
"#,
            lat
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
    }
}

#[test]
fn rdm_longitude_range() {
    for lon in ["-180.0", "-45.5", "0.0", "90.0", "180.0"] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_REF_FRAME = ITRF
NOMINAL_IMPACT_LON = {}
NOMINAL_IMPACT_LAT = 0
"#,
            lon
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
    }
}

#[test]
fn rdm_altitude_range() {
    for alt in ["-430.0", "0.0", "1000.0", "8000.0"] {
        let kvn = format!(
            r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_REF_FRAME = ITRF
NOMINAL_IMPACT_LON = 0
NOMINAL_IMPACT_LAT = 0
NOMINAL_IMPACT_ALT = {}
"#,
            alt
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
    }
}

#[test]
fn rdm_impact_confidence_intervals() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_1_CONFIDENCE = 50.0 [%]
IMPACT_1_START_LON = -10.0
IMPACT_1_START_LAT = 40.0
IMPACT_1_STOP_LON = 10.0
IMPACT_1_STOP_LAT = 45.0
IMPACT_1_CROSS_TRACK = 100.0 [km]
IMPACT_2_CONFIDENCE = 90.0 [%]
IMPACT_2_START_LON = -15.0
IMPACT_2_START_LAT = 38.0
IMPACT_2_STOP_LON = 15.0
IMPACT_2_STOP_LAT = 47.0
IMPACT_2_CROSS_TRACK = 200.0 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let ground = rdm.body.segment.data.ground_impact_parameters.unwrap();
    assert!(ground.impact_1_confidence.is_some());
    assert!(ground.impact_2_confidence.is_some());
}

#[test]
fn rdm_state_vector_type() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let sv = rdm.body.segment.data.state_vector.as_ref().unwrap();
    assert!((sv.x.value - 7000.0).abs() < 1e-9);
}

#[test]
fn rdm_covariance_matrix_type() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
COV_REF_FRAME = RTN
CX_X = 1.0e-4 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0e-4 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0e-4 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 1.0e-6 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 1.0e-6 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 1.0e-6 [km**2/s**2]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let cov = rdm.body.segment.data.covariance_matrix.as_ref().unwrap();
    assert_eq!(cov.cov_ref_frame, Some("RTN".to_string()));
    assert!((cov.cx_x.value - 1.0e-4).abs() < 1e-15);
}

#[test]
fn rdm_spacecraft_parameters() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
WET_MASS = 3500 [kg]
DRY_MASS = 2000 [kg]
HAZARDOUS_SUBSTANCES = Hydrazine, Nuclear
SOLAR_RAD_AREA = 25.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 20.0 [m**2]
DRAG_COEFF = 2.2
RCS = 15.0 [m**2]
BALLISTIC_COEFF = 150.0
THRUST_ACCELERATION = 0.001
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let sp = rdm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert!((sp.wet_mass.as_ref().unwrap().value - 3500.0).abs() < 1e-9);
    assert!((sp.dry_mass.as_ref().unwrap().value - 2000.0).abs() < 1e-9);
    assert_eq!(
        sp.hazardous_substances,
        Some("Hydrazine, Nuclear".to_string())
    );
    assert!(sp.ballistic_coeff.is_some());
    assert!(sp.thrust_acceleration.is_some());
}

#[test]
fn rdm_coefficients_nonnegative() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
SOLAR_RAD_COEFF = 0.0
DRAG_COEFF = 0.0
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let sp = rdm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert!((sp.solar_rad_coeff.unwrap().value - 0.0).abs() < 1e-9);
    assert!((sp.drag_coeff.unwrap().value - 0.0).abs() < 1e-9);
}

#[test]
fn rdm_od_parameters() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
TIME_LASTOB_START = 2022-12-31T00:00:00
TIME_LASTOB_END = 2022-12-31T23:59:59
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 20
TRACKS_USED = 18
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.234
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let od = rdm.body.segment.data.od_parameters.as_ref().unwrap();
    assert!(od.time_lastob_start.is_some());
    assert!(od.time_lastob_end.is_some());
    assert_eq!(od.obs_available, Some(100.into()));
    assert_eq!(od.obs_used, Some(95.into()));
    assert_eq!(od.tracks_available, Some(20.into()));
    assert_eq!(od.tracks_used, Some(18.into()));
}

#[test]
fn rdm_sample_c1_kvn() {
    let kvn = include_str!("../../data/kvn/rdm_c1.kvn");
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
}

#[test]
fn rdm_sample_c2_kvn() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2018-04-22T09:31:34.00
ORIGINATOR = ESA
MESSAGE_ID = ESA/20180422-001
OBJECT_NAME = SPACEOBJECT
INTERNATIONAL_DESIGNATOR = 2018-099B
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 81594
OBJECT_TYPE = ROCKET BODY
OBJECT_OWNER = ESA
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2018-04-22T09:00:00.00
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96: 36D 36O
ATMOSPHERIC_MODEL = NRLMSISE-00
N_BODY_PERTURBATIONS = MOON
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
INTRACK_THRUST = NO
REENTRY_DISINTEGRATION = MASS-LOSS + BREAK-UP
PREVIOUS_MESSAGE_ID = ESA/20180421-007
NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2018-04-27T19:45:33
REENTRY_WINDOW_START = 2018-04-27T11:45:33
REENTRY_WINDOW_END = 2018-04-27T22:12:56
PROBABILITY_OF_IMPACT = 0.0
PROBABILITY_OF_BURN_UP = 1.0
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(
        rdm.body.segment.metadata.catalog_name,
        Some("SATCAT".to_string())
    );
    assert!(rdm
        .body
        .segment
        .data
        .atmospheric_reentry_parameters
        .nominal_reentry_epoch
        .is_some());
}

/// Every mandatory keyword, dropped one at a time from a shipped fixture.
///
/// This was eleven copies of the same minimal message, each missing one line. `rdm_c1.kvn`
/// carries exactly those keywords and nothing else, so dropping one line from it is the same
/// case without the copies — and the baseline parse proves the other ten are still present.
#[test]
fn every_mandatory_rdm_keyword_is_reported_by_name() {
    const FIXTURE: &str = include_str!("../../data/kvn/rdm_c1.kvn");
    Rdm::from_kvn(FIXTURE).expect("baseline fixture must carry every mandatory keyword");

    for (keyword, block) in [
        ("CREATION_DATE", "Header"),
        ("ORIGINATOR", "Header"),
        ("MESSAGE_ID", "Header"),
        ("OBJECT_NAME", "Metadata"),
        ("INTERNATIONAL_DESIGNATOR", "Metadata"),
        ("CONTROLLED_REENTRY", "Metadata"),
        ("CENTER_NAME", "Metadata"),
        ("TIME_SYSTEM", "Metadata"),
        ("EPOCH_TZERO", "Metadata"),
        ("ORBIT_LIFETIME", "Atmospheric Reentry"),
        ("REENTRY_ALTITUDE", "Atmospheric Reentry"),
    ] {
        let kept: Vec<&str> = FIXTURE
            .lines()
            .filter(|line| !line.starts_with(&format!("{keyword} =")))
            .collect();
        assert_eq!(
            kept.len(),
            FIXTURE.lines().count() - 1,
            "{keyword} matched no line, so nothing was dropped"
        );

        let error = Rdm::from_kvn(&format!("{}\n", kept.join("\n")))
            .expect_err("message with a mandatory keyword removed was accepted");
        // ponytail: matching the rendered diagnostic rather than the typed variant. Only
        // `MissingRequiredField` renders this way, so it is exact today; the ceiling is that
        // rewording the `Display` impl breaks these tests. Match on the variant if that lands.
        let expected = format!("Missing required field: {keyword} in block {block}");
        assert!(
            error.to_string().contains(&expected),
            "dropping {keyword} reported {error}"
        );
    }
}
#[test]
fn test_rdm_empty_file_error() {
    let error = Rdm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!((parse.line, parse.column), (1, 1));
}

#[test]
fn test_rdm_version_not_first_error() {
    let kvn = r#"OBJECT_NAME = TEST
CCSDS_RDM_VERS = 1.0
"#;
    let err = Rdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err
                    .message
                    .to_lowercase()
                    .contains("duplicate or out-of-order rdm keyword"));
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected version-not-first error, got: {:?}", err),
    }
}

#[test]
fn rdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = SPACEOBJECT";
    let designator = "INTERNATIONAL_DESIGNATOR = 2018-099B";
    let lifetime = "ORBIT_LIFETIME = 5.5 [d]";
    for (label, source) in [
        (
            "duplicate keyword",
            mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            mutated(
                KVN,
                &format!("{object_name}\n{designator}"),
                &format!("{designator}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            mutated(KVN, lifetime, &format!("{lifetime}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            mutated(KVN, lifetime, &format!("{lifetime}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object_name, &format!("{object_name} €")),
        ),
    ] {
        let error = Rdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn rdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let lifetime = "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>";
    let altitude = "<REENTRY_ALTITUDE units=\"km\">80.0</REENTRY_ALTITUDE>";
    for (label, source) in [
        (
            "unknown atmospheric child",
            mutated(
                XML,
                "<atmosphericReentryParameters>",
                "<atmosphericReentryParameters><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown covariance child",
            mutated(
                XML,
                "<covarianceMatrix>",
                "<covarianceMatrix><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown container attribute",
            mutated(
                XML,
                "<groundImpactParameters>",
                "<groundImpactParameters unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                lifetime,
                "<ORBIT_LIFETIME units=\"d\" unexpected=\"value\">5.5</ORBIT_LIFETIME>",
            ),
        ),
        (
            "duplicate element",
            mutated(XML, lifetime, &format!("{lifetime}{lifetime}")),
        ),
        (
            "reordered elements",
            mutated(
                XML,
                &format!("{lifetime}\n{altitude}"),
                &format!("{altitude}\n{lifetime}"),
            ),
        ),
    ] {
        let error = Rdm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

/// Parse official RDM XML example C-3 (minimal)
#[test]
fn rdm_sample_c3_xml() {
    let xml = include_str!("../../data/xml/rdm_c3.xml");
    let rdm = Rdm::from_xml(xml).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
}

/// Parse official RDM XML example C-4 (comprehensive)
#[test]
fn rdm_sample_c4_xml() {
    let xml = include_str!("../../data/xml/rdm_c4.xml");
    let rdm = Rdm::from_xml(xml).unwrap();
    assert_eq!(rdm.header.message_id, "ESA/20180422-001");
    assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
    assert!(rdm.body.segment.data.state_vector.is_some());
    assert!(rdm.body.segment.data.covariance_matrix.is_some());
    assert!(rdm.body.segment.data.spacecraft_parameters.is_some());
    assert!(rdm.body.segment.data.od_parameters.is_some());
}

/// The conditional RDM rules, as single mutations of a shipped fixture.
#[test]
fn rdm_conditional_rules_name_the_offending_field() {
    const FIXTURE: &str = include_str!("../../data/kvn/rdm_c2.kvn");
    const STATE_VECTOR: &str = "COMMENT State vector at the last OD epoch";
    const COVARIANCE: &str = "COMMENT Position/velocity covariance matrix at last OD epoch";
    Rdm::from_kvn(FIXTURE).expect("baseline fixture must satisfy every rule");

    let state_vector_at = FIXTURE.find(STATE_VECTOR).unwrap();
    let covariance_at = FIXTURE.find(COVARIANCE).unwrap();
    // Neither block is mandatory, so the truncation is a valid message in its own right.
    let no_state_or_covariance = &FIXTURE[..state_vector_at];
    Rdm::from_kvn(no_state_or_covariance)
        .expect("a message with neither optional block must still parse");
    let covariance_without_state =
        format!("{}{}", no_state_or_covariance, &FIXTURE[covariance_at..]);

    for (mutated, expected) in [
            (
                mutated(FIXTURE, "REENTRY_ALTITUDE = 80.0 [km]",
                    "REENTRY_ALTITUDE = 80.0 [km]\nORBIT_LIFETIME_WINDOW_START = 6.0 [d]\nORBIT_LIFETIME_WINDOW_END = 5.0 [d]",
                ),
                "ORBIT_LIFETIME_WINDOW_START must be <= ORBIT_LIFETIME_WINDOW_END",
            ),
            (
                mutated(FIXTURE, "REF_FRAME = EME2000\n", ""),
                "Missing required field: REF_FRAME (required when state vector is provided)",
            ),
            (
                covariance_without_state,
                "Missing required field: stateVector (required when covarianceMatrix is provided)",
            ),
            (
                mutated(FIXTURE, "OBJECT_NAME = SPACEOBJECT", "OBJECT_NAME ="),
                "Missing required field: OBJECT_NAME",
            ),
        ] {
            let error = Rdm::from_kvn(&mutated).expect_err("mutation accepted");
            assert!(
                error.to_string().contains(expected),
                "diagnostic did not name {expected}: {error}"
            );
        }
}
