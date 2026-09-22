use crate::common::{mutated, mutated_once};
use crate::{sample_cdm_kvn, KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, FormatError, ValidationError};
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::types::{
    CdmObjectType, ManeuverableType, ObjectDescription, ReferenceFrameType, ScreenVolumeFrameType,
    ScreenVolumeShapeType, YesNo,
};
use ccsds_ndm::Ndm;
// From CCSDS Blue Book 508.0-B-1 Annex D (modified for KVN)
const CDM_BLUE_BOOK_SAMPLE: &str = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2010-03-12T22:31:12.000
ORIGINATOR = JSPOC
MESSAGE_FOR = SATELLITE A
MESSAGE_ID = 201113719185

TCA = 2010-03-13T22:31:12.000
MISS_DISTANCE = 123.4 [m]
RELATIVE_SPEED = 12.3 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = 20.0 [m]
RELATIVE_POSITION_N = 30.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = 0.2 [m/s]
RELATIVE_VELOCITY_N = 0.3 [m/s]
SCREEN_VOLUME_SHAPE = ELLIPSOID

OBJECT = OBJECT1
OBJECT_DESIGNATOR = 12345
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT A
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1000.0 [km]
Y = 2000.0 [km]
Z = 3000.0 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 67890
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT B
INTERNATIONAL_DESIGNATOR = 2000-001A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1500.0 [km]
Y = 2500.0 [km]
Z = 3500.0 [km]
X_DOT = 1.5 [km/s]
Y_DOT = 2.5 [km/s]
Z_DOT = 3.5 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"###;

const DRAG_COVARIANCE_ROW: &str = "\
CDRG_R = 0.001 [m**3/kg]
CDRG_T = 0.002 [m**3/kg]
CDRG_N = 0.003 [m**3/kg]
CDRG_RDOT = 0.0001 [m**3/(kg*s)]
CDRG_TDOT = 0.0002 [m**3/(kg*s)]
CDRG_NDOT = 0.0003 [m**3/(kg*s)]
CDRG_DRG = 0.00001 [m**4/kg**2]";

const SRP_COVARIANCE_ROW: &str = "\
CSRP_R = 0.001 [m**3/kg]
CSRP_T = 0.002 [m**3/kg]
CSRP_N = 0.003 [m**3/kg]
CSRP_RDOT = 0.0001 [m**3/(kg*s)]
CSRP_TDOT = 0.0002 [m**3/(kg*s)]
CSRP_NDOT = 0.0003 [m**3/(kg*s)]
CSRP_DRG = 0.00001 [m**4/kg**2]
CSRP_SRP = 0.00002 [m**4/kg**2]";

const THRUST_COVARIANCE_ROW: &str = "\
CTHR_R = 0.001 [m**2/s**2]
CTHR_T = 0.002 [m**2/s**2]
CTHR_N = 0.003 [m**2/s**2]
CTHR_RDOT = 0.0001 [m**2/s**3]
CTHR_TDOT = 0.0002 [m**2/s**3]
CTHR_NDOT = 0.0003 [m**2/s**3]
CTHR_DRG = 0.00001 [m**3/(kg*s**2)]
CTHR_SRP = 0.00002 [m**3/(kg*s**2)]
CTHR_THR = 0.000001 [m**2/s**4]";

fn with_optional_covariance_rows(kvn: &str, rows: &str) -> String {
    kvn.replace(
        "CNDOT_NDOT = 1.0 [m**2/s**2]",
        &format!("CNDOT_NDOT = 1.0 [m**2/s**2]\n{rows}"),
    )
}

#[test]
fn test_parse_cdm_blue_book_example() {
    let result = Cdm::from_kvn(CDM_BLUE_BOOK_SAMPLE);
    assert!(result.is_ok(), "Failed to parse CDM: {:?}", result.err());

    let cdm = result.unwrap();
    assert_eq!(cdm.version, "1.0");
    assert_eq!(cdm.header.originator, "JSPOC");
    assert_eq!(cdm.body.segments.len(), 2);
    assert_eq!(cdm.body.segments[0].metadata.object_name, "SAT A");
    assert_eq!(cdm.body.segments[1].metadata.object_name, "SAT B");
    assert_eq!(cdm.body.relative_metadata_data.miss_distance.value, 123.4);
}

#[test]
fn parse_cdm_kvn_success() {
    let kvn = sample_cdm_kvn();
    let cdm = Cdm::from_kvn(&kvn).expect("CDM should parse");
    assert_eq!(cdm.version, "1.0");
    assert_eq!(cdm.header.originator, "TEST");
    assert_eq!(cdm.body.segments.len(), 2);
    assert!(cdm
        .body
        .relative_metadata_data
        .relative_state_vector
        .is_some());
    assert_eq!(
        cdm.body.relative_metadata_data.screen_volume_frame,
        Some(ScreenVolumeFrameType::Rtn)
    );
    assert_eq!(
        cdm.body.relative_metadata_data.screen_volume_shape,
        Some(ScreenVolumeShapeType::Box)
    );
}

#[test]
fn header_missing_fields_error() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
MESSAGE_FOR = SAT
MESSAGE_ID = MSG-001
"###;
    let err = Cdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "CREATION_DATE");
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

/// A CDM describes exactly two objects, so a message truncated before the second one must be
/// refused. Truncating a shipped fixture keeps the first segment byte-for-byte valid, leaving
/// the missing second segment as the only reason the parse can fail.
#[test]
fn validate_exactly_two_segments() {
    const FIXTURE: &str = include_str!("../../data/kvn/cdm_362.kvn");
    let first_segment_only: Vec<&str> = FIXTURE
        .lines()
        .take_while(|line| !line.starts_with("OBJECT = OBJECT2"))
        .collect();

    Cdm::from_kvn(FIXTURE).expect("fixture with both segments must parse");
    let error = Cdm::from_kvn(&format!("{}\n", first_segment_only.join("\n")))
        .expect_err("single-segment CDM accepted");
    assert!(
        matches!(
            &error,
            CcsdsNdmError::Validation(validation)
                if matches!(&**validation, ValidationError::MissingRequiredField { field, .. } if field == "OBJECT")
        ),
        "single-segment CDM was refused for an unrelated reason: {error}"
    );
}

#[test]
fn relative_state_vector_must_be_complete() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = SAT
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
SCREEN_VOLUME_SHAPE = BOX
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 1
CATALOG_NAME = CAT
OBJECT_NAME = O1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
OBJECT = OBJECT2
OBJECT_DESIGNATOR = 2
CATALOG_NAME = CAT
OBJECT_NAME = O2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
"###;
    let err = Cdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "RELATIVE_VELOCITY_N");
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn covariance_missing_required_field() {
    // Remove CR_R from first segment to trigger error
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("CR_R = 1.0 [m**2]", "");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField {
                ref block,
                ref field,
                ..
            } => {
                assert_eq!(block, "Covariance Matrix");
                assert_eq!(field, "CR_R");
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn screen_frame_shape_validation() {
    // Invalid SCREEN_VOLUME_FRAME
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("SCREEN_VOLUME_FRAME = RTN", "SCREEN_VOLUME_FRAME = BAD");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }

    // Invalid SCREEN_VOLUME_SHAPE
    let mut kvn2 = sample_cdm_kvn();
    kvn2 = kvn2.replace("SCREEN_VOLUME_SHAPE = BOX", "SCREEN_VOLUME_SHAPE = BALL");
    let err2 = Cdm::from_kvn(&kvn2).unwrap_err();
    match err2 {
        e if matches!(e.as_format_error(), Some(FormatError::Enum(_))) => {}
        _ => panic!("unexpected error: {:?}", err2),
    }
}

#[test]
fn version_must_be_first() {
    let kvn = r#"CREATION_DATE = 2025-01-01T00:00:00
CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
"#;
    let err = Cdm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message
                        .to_lowercase()
                        .contains("expected ccsds_cdm_vers")
                        || err
                            .contexts
                            .iter()
                            .any(|c| c.to_lowercase().contains("ccsds_cdm_vers"))
                )
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn empty_file_error() {
    let error = Cdm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.message, "missing CCSDS_CDM_VERS");
    assert_eq!((parse.line, parse.column), (1, 1));
}

#[test]
fn header_with_message_for() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("MESSAGE_FOR = OPERATOR", "MESSAGE_FOR = NEW_OPERATOR");
    let cdm = Cdm::from_kvn(&kvn).expect("should parse with MESSAGE_FOR");
    assert_eq!(cdm.header.message_for, Some("NEW_OPERATOR".to_string()));
}

#[test]
fn relative_metadata_with_screen_periods() {
    let mut kvn = sample_cdm_kvn();
    // Insert screen period fields
    kvn = kvn.replace(
        "SCREEN_VOLUME_FRAME = RTN",
        "START_SCREEN_PERIOD = 2025-01-02T11:00:00\nSTOP_SCREEN_PERIOD = 2025-01-02T13:00:00\nSCREEN_VOLUME_FRAME = RTN",
    );
    kvn = kvn.replace(
        "COLLISION_PROBABILITY = 0.001",
        "SCREEN_ENTRY_TIME = 2025-01-02T11:30:00\nSCREEN_EXIT_TIME = 2025-01-02T12:30:00\nCOLLISION_PROBABILITY = 0.001\nCOLLISION_PROBABILITY_METHOD = FOSTER-1992",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with screen period");
    assert!(cdm
        .body
        .relative_metadata_data
        .start_screen_period
        .is_some());
    assert!(cdm.body.relative_metadata_data.stop_screen_period.is_some());
    assert!(cdm.body.relative_metadata_data.screen_entry_time.is_some());
    assert!(cdm.body.relative_metadata_data.screen_exit_time.is_some());
    assert_eq!(
        cdm.body.relative_metadata_data.collision_probability_method,
        Some("FOSTER-1992".to_string())
    );
}

#[test]
fn relative_metadata_collision_probability_parse_error() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace(
        "COLLISION_PROBABILITY = 0.001",
        "COLLISION_PROBABILITY = INVALID",
    );

    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn relative_metadata_unexpected_field() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace(
        "COLLISION_PROBABILITY = 0.001",
        "COLLISION_PROBABILITY = 0.001\nUNKNOWN_FIELD = VALUE",
    );

    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("unknown CDM keyword"))
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_with_optional_fields() {
    let mut kvn = sample_cdm_kvn();
    // Add optional metadata fields
    kvn = kvn.replace(
        "OBJECT_TYPE = PAYLOAD\nEPHEMERIS_NAME = EPH1\nCOVARIANCE_METHOD = CALCULATED\nMANEUVERABLE = YES\nREF_FRAME = EME2000",
        "OBJECT_TYPE = PAYLOAD\nOPERATOR_CONTACT_POSITION = Flight Director\nOPERATOR_ORGANIZATION = NASA\nOPERATOR_PHONE = +1-555-1234\nOPERATOR_EMAIL = contact@nasa.gov\nEPHEMERIS_NAME = EPH1\nCOVARIANCE_METHOD = CALCULATED\nMANEUVERABLE = YES\nORBIT_CENTER = EARTH\nREF_FRAME = EME2000\nGRAVITY_MODEL = EGM-96\nATMOSPHERIC_MODEL = JACCHIA 70 DCA\nN_BODY_PERTURBATIONS = MOON, SUN\nSOLAR_RAD_PRESSURE = YES\nEARTH_TIDES = YES\nINTRACK_THRUST = YES",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with optional metadata");
    let seg1 = &cdm.body.segments[0];
    assert_eq!(seg1.metadata.object_type, Some(ObjectDescription::Payload));
    assert_eq!(
        seg1.metadata.operator_contact_position,
        Some("Flight Director".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_organization,
        Some("NASA".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_phone,
        Some("+1-555-1234".to_string())
    );
    assert_eq!(
        seg1.metadata.operator_email,
        Some("contact@nasa.gov".to_string())
    );
    assert_eq!(seg1.metadata.orbit_center, Some("EARTH".to_string()));
    assert_eq!(seg1.metadata.gravity_model, Some("EGM-96".to_string()));
    assert_eq!(
        seg1.metadata.atmospheric_model,
        Some("JACCHIA 70 DCA".to_string())
    );
    assert_eq!(
        seg1.metadata.n_body_perturbations,
        Some("MOON, SUN".to_string())
    );
    assert_eq!(seg1.metadata.solar_rad_pressure, Some(YesNo::Yes));
    assert_eq!(seg1.metadata.earth_tides, Some(YesNo::Yes));
    assert_eq!(seg1.metadata.intrack_thrust, Some(YesNo::Yes));
}

#[test]
fn metadata_object_types() {
    // Test ROCKET BODY
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = ROCKET BODY");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.object_type,
        Some(ObjectDescription::RocketBody)
    );

    // Test DEBRIS
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = DEBRIS");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.object_type,
        Some(ObjectDescription::Debris)
    );

    // Test UNKNOWN
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = UNKNOWN");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.object_type,
        Some(ObjectDescription::Unknown)
    );

    // Test OTHER
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = OTHER");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.object_type,
        Some(ObjectDescription::Other)
    );

    // Test fallback to OTHER for unknown values
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = SATELLITE");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.object_type,
        Some(ObjectDescription::Other)
    );
}

#[test]
fn metadata_invalid_object() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("OBJECT = OBJECT1", "OBJECT = OBJECT3");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_invalid_covariance_method() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace(
        "COVARIANCE_METHOD = CALCULATED",
        "COVARIANCE_METHOD = INVALID",
    );
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_maneuverable_na() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("MANEUVERABLE = YES", "MANEUVERABLE = N/A");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.maneuverable,
        ManeuverableType::NA
    );
}

#[test]
fn metadata_invalid_maneuverable() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("MANEUVERABLE = YES", "MANEUVERABLE = MAYBE");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_ref_frames() {
    // Test GCRF
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = GCRF");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.ref_frame,
        ReferenceFrameType::Gcrf
    );

    // Test ITRF
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = ITRF");
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    assert_eq!(
        cdm.body.segments[0].metadata.ref_frame,
        ReferenceFrameType::Itrf
    );
}

#[test]
fn metadata_invalid_ref_frame() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = INVALID");
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_unknown_key() {
    let mut kvn = sample_cdm_kvn();
    kvn = kvn.replace(
        "REF_FRAME = EME2000",
        "REF_FRAME = EME2000\nUNKNOWN_META_KEY = VALUE",
    );
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("unknown CDM keyword"))
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn data_with_od_parameters() {
    let mut kvn = sample_cdm_kvn();
    // Insert OD parameters before state vector
    kvn = kvn.replace(
        "X = 1.0 [km]",
        "TIME_LASTOB_START = 2025-01-01T00:00:00\nTIME_LASTOB_END = 2025-01-02T00:00:00\nRECOMMENDED_OD_SPAN = 7.0 [d]\nACTUAL_OD_SPAN = 5.0 [d]\nOBS_AVAILABLE = 100\nOBS_USED = 95\nTRACKS_AVAILABLE = 50\nTRACKS_USED = 48\nRESIDUALS_ACCEPTED = 95.5 [%]\nWEIGHTED_RMS = 1.23\nX = 1.0 [km]",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with OD parameters");
    let od = cdm.body.segments[0].data.od_parameters.as_ref().unwrap();
    assert!(od.time_lastob_start.is_some());
}

#[test]
fn data_with_additional_parameters() {
    let mut kvn = sample_cdm_kvn();
    // Insert additional parameters
    kvn = kvn.replace(
        "X = 1.0 [km]",
        "AREA_PC = 10.0 [m**2]\nAREA_DRG = 12.0 [m**2]\nAREA_SRP = 15.0 [m**2]\nMASS = 1000.0 [kg]\nCD_AREA_OVER_MASS = 0.012 [m**2/kg]\nCR_AREA_OVER_MASS = 0.015 [m**2/kg]\nTHRUST_ACCELERATION = 0.001 [m/s**2]\nSEDR = 0.05 [W/kg]\nX = 1.0 [km]",
    );

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with additional parameters");
    let ap = cdm.body.segments[0]
        .data
        .additional_parameters
        .as_ref()
        .unwrap();
    assert!(ap.area_pc.is_some());
    assert!(ap.area_drg.is_some());
    assert!(ap.area_srp.is_some());
    assert!(ap.mass.is_some());
    assert!(ap.cd_area_over_mass.is_some());
    assert!(ap.cr_area_over_mass.is_some());
    assert!(ap.thrust_acceleration.is_some());
    assert!(ap.sedr.is_some());
}

#[test]
fn covariance_with_drag_fields() {
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), DRAG_COVARIANCE_ROW);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CDRG fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.cdrg_r.is_some());
    assert!(cov.cdrg_t.is_some());
    assert!(cov.cdrg_n.is_some());
    assert!(cov.cdrg_rdot.is_some());
    assert!(cov.cdrg_tdot.is_some());
    assert!(cov.cdrg_ndot.is_some());
    assert!(cov.cdrg_drg.is_some());
}

#[test]
fn covariance_with_srp_fields() {
    let rows = format!("{DRAG_COVARIANCE_ROW}\n{SRP_COVARIANCE_ROW}");
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), &rows);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CSRP fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.csrp_r.is_some());
    assert!(cov.csrp_t.is_some());
    assert!(cov.csrp_n.is_some());
    assert!(cov.csrp_rdot.is_some());
    assert!(cov.csrp_tdot.is_some());
    assert!(cov.csrp_ndot.is_some());
    assert!(cov.csrp_drg.is_some());
    assert!(cov.csrp_srp.is_some());
}

#[test]
fn covariance_with_thrust_fields() {
    let rows = format!("{DRAG_COVARIANCE_ROW}\n{SRP_COVARIANCE_ROW}\n{THRUST_COVARIANCE_ROW}");
    let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), &rows);

    let cdm = Cdm::from_kvn(&kvn).expect("should parse with CTHR fields");
    let cov = &cdm.body.segments[0]
        .data
        .covariance_matrix
        .as_ref()
        .expect("Covariance matrix missing");
    assert!(cov.cthr_r.is_some());
    assert!(cov.cthr_t.is_some());
    assert!(cov.cthr_n.is_some());
    assert!(cov.cthr_rdot.is_some());
    assert!(cov.cthr_tdot.is_some());
    assert!(cov.cthr_ndot.is_some());
    assert!(cov.cthr_drg.is_some());
    assert!(cov.cthr_srp.is_some());
    assert!(cov.cthr_thr.is_some());
}

#[test]
fn covariance_rejects_optional_rows_without_preceding_rows() {
    for (label, rows, missing) in [
        ("SRP without drag", SRP_COVARIANCE_ROW, "CDRG_R"),
        (
            "thrust without drag or SRP",
            THRUST_COVARIANCE_ROW,
            "CDRG_R",
        ),
    ] {
        let kvn = with_optional_covariance_rows(&sample_cdm_kvn(), rows);
        let error = Cdm::from_kvn(&kvn).expect_err(label);
        assert_eq!(error.code(), Some("validation.missing_required_field"));
        assert!(error.to_string().contains(missing), "{label}: {error}");
    }
}

#[test]
fn covariance_unknown_field_error() {
    let mut kvn = sample_cdm_kvn();
    // Add unknown covariance field
    kvn = kvn.replace(
        "CNDOT_NDOT = 1.0 [m**2/s**2]",
        "CNDOT_NDOT = 1.0 [m**2/s**2]\nUNKNOWN_COV = 0.001",
    );
    let err = Cdm::from_kvn(&kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("unknown CDM keyword"))
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_object_default_fallback() {
    // Test what happens if OBJECT isn't OBJECT1 or OBJECT2 in serialization
    let kvn = sample_cdm_kvn();
    let cdm = Cdm::from_kvn(&kvn).expect("parse");
    // Verify both objects are parsed correctly
    assert_eq!(cdm.body.segments[0].metadata.object, CdmObjectType::Object1);
    assert_eq!(cdm.body.segments[1].metadata.object, CdmObjectType::Object2);
}

#[test]
fn unexpected_segment_start() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
META_START
"###;
    let error = Cdm::from_kvn(kvn).unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.message, "expected exactly one CDM assignment");
    assert_eq!(parse.line, 8, "diagnostic did not point at META_START");
}

/// A message that ends inside the relative metadata block. The parser does not report this as
/// an end-of-input condition — `UnexpectedEof` is raised only by format detection and by the
/// combined-NDM reader — it reports the first mandatory field the truncated block never
/// supplied, which is the diagnostic a producer can act on.
#[test]
fn truncated_relative_metadata_names_the_missing_field() {
    let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
TCA = 2025-01-02T12:00:00
"###;
    let error = Cdm::from_kvn(kvn).unwrap_err();
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("expected a validation error, got {error:?}"));
    assert!(
        matches!(
            validation,
            ValidationError::MissingRequiredField { block, field, .. }
                if block == "Relative Metadata" && field == "MISS_DISTANCE"
        ),
        "unexpected diagnostic: {validation:?}"
    );
}
#[test]
fn test_parse_cdm_missing_relative_metadata() {
    let input = r#"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
SCREEN_VOLUME_FRAME = RTN
"#;
    let err = Cdm::from_kvn(input).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { field, .. } => {
                assert_eq!(field, "RELATIVE_POSITION_T");
            }
            _ => panic!("Expected missing field error, got {:?}", val_err),
        },
        _ => panic!("Expected Validation error, got {:?}", err),
    }
}

#[test]
fn test_parse_cdm_missing_covariance() {
    // Verify we can parse CDM without covariance matrix (it is optional)
    let cdm_no_cov = r#"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = 20.0 [m]
RELATIVE_POSITION_N = 30.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = 0.2 [m/s]
RELATIVE_VELOCITY_N = 0.3 [m/s]

OBJECT = OBJECT1
OBJECT_DESIGNATOR = 12345
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT A
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1000.0 [km]
Y = 2000.0 [km]
Z = 3000.0 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 67890
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT B
INTERNATIONAL_DESIGNATOR = 2000-001A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1500.0 [km]
Y = 2500.0 [km]
Z = 3500.0 [km]
X_DOT = 1.5 [km/s]
Y_DOT = 2.5 [km/s]
Z_DOT = 3.5 [km/s]
"#;
    let err = Cdm::from_kvn(cdm_no_cov).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { field, .. } => {
                assert_eq!(field, "covarianceMatrix");
            }
            _ => panic!("Expected missing field error, got {:?}", val_err),
        },
        _ => panic!("Expected Validation error, got {:?}", err),
    }
}

#[test]
fn cdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let tca = "TCA = 2010-03-13T22:37:52.618";
    let miss = "MISS_DISTANCE = 715 [m]";
    let object = "OBJECT = OBJECT1";
    for (label, source) in [
        (
            "duplicate relative keyword",
            mutated(KVN, miss, &format!("{miss}\n{miss}")),
        ),
        (
            "reordered relative keywords",
            mutated(KVN, &format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "unknown metadata keyword",
            mutated(KVN, object, &format!("{object}\nUNKNOWN = value")),
        ),
        (
            "comment inside relative block",
            mutated(KVN, miss, &format!("{miss}\nCOMMENT misplaced")),
        ),
        ("unknown marked block", mutated(KVN, object, "META_START")),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object, "OBJECT = OBJECT1 €"),
        ),
    ] {
        let error = Cdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn cdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let tca = "<TCA>2010-03-13T22:37:52.618</TCA>";
    let miss = "<MISS_DISTANCE units=\"m\">715</MISS_DISTANCE>";
    let object = "<OBJECT>OBJECT1</OBJECT>";
    for (label, source) in [
        (
            "unknown data child",
            mutated(XML, "<data>", "<data><UNKNOWN>1</UNKNOWN>"),
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
            mutated(XML, "<segment>", "<segment unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                miss,
                "<MISS_DISTANCE units=\"m\" unexpected=\"value\">715</MISS_DISTANCE>",
            ),
        ),
        (
            "duplicate metadata child",
            mutated(XML, object, &format!("{object}{object}")),
        ),
        (
            "reordered relative children",
            mutated(XML, &format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "illegal nil attribute",
            mutated(XML, tca, "<TCA nil=\"true\"/>"),
        ),
    ] {
        let error = Cdm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

#[test]
fn cdm_kvn_rejects_a_state_vector_with_metre_units() {
    let source = include_str!("../../data/kvn/cdm_364.kvn");
    Cdm::from_kvn(source).expect("baseline fixture must parse");
    let input = mutated_once(
        source,
        "X = -41600.46272465 [km]",
        "X = -41600.46272465 [m]",
    );
    let error = Cdm::from_kvn(&input)
        .expect_err("CDM state-vector positions are kilometres, not arbitrary length units");
    assert!(
        matches!(error.as_format_error(), Some(FormatError::Enum(cause))
            if cause.field == "unit" && cause.value == "m" && cause.expected == "\"km\""),
        "{error}"
    );
}

#[test]
fn xml_required_unit_attributes_are_not_inferred() {
    Cdm::from_xml(XML).expect("baseline fixture must parse");
    let input = mutated_once(XML, "<X units=\"km\">", "<X>");
    let error = Cdm::from_xml(&input).expect_err("CDM XML state-vector positions require units");
    let diagnostic = error.to_string();
    assert!(
        diagnostic.contains("missing") && diagnostic.contains("units"),
        "{error}"
    );
}
