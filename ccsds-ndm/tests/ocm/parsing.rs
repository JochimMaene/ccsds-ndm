use crate::common::mutated;
use crate::{KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, FormatError, ValidationError};
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::types::CovOrder;
use ccsds_ndm::Ndm;
/// Each numeric parser is reached with an otherwise valid, correctly ordered block.
#[test]
fn malformed_numbers_identify_the_mutated_record() {
    const HEADER: &str = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\n";
    let blocks: &[(&str, &str, &[&str])] = &[
        ("TRAJ_START\n", "CENTER_NAME = EARTH\nTRAJ_REF_FRAME = GCRF\nTRAJ_TYPE = CARTPV\n0 1 2 3 4 5 6\nTRAJ_STOP\n", &["INTERPOLATION_DEGREE"]),
        ("TRAJ_START\nCENTER_NAME = EARTH\nTRAJ_REF_FRAME = GCRF\n", "TRAJ_TYPE = CARTPV\n0 1 2 3 4 5 6\nTRAJ_STOP\n", &["ORB_REVNUM"]),
        ("PHYS_START\n", "PHYS_STOP\n", &["OEB_Q2", "OEB_Q3", "OEB_QC", "SOLAR_RAD_COEFF", "VM_ABSOLUTE", "VM_APPARENT_MIN", "VM_APPARENT", "VM_APPARENT_MAX", "REFLECTANCE"]),
        ("COV_START\nCOV_REF_FRAME = GCRF\n", "COV_TYPE = CARTP\nCOV_ORDERING = LTM\n0 1 0 1 0 0 1\nCOV_STOP\n", &["COV_SCALE_MIN", "COV_SCALE_MAX"]),
        ("MAN_START\nMAN_ID = MAN1\nMAN_DEVICE_ID = DEV1\nMAN_REF_FRAME = TNW\n", "MAN_COMPOSITION = TIME_ABSOLUTE\n2023-01-01T00:00:00 0.1 0.2 0.3\nMAN_STOP\n", &["DC_MIN_CYCLES", "DC_MAX_CYCLES"]),
        ("PERT_START\n", "PERT_STOP\n", &["OBLATE_FLATTENING", "ALBEDO_GRID_SIZE"]),
        ("OD_START\nOD_ID = OD1\nOD_METHOD = LS\nOD_EPOCH = 2023-01-01T00:00:00\n", "OD_STOP\n", &["OBS_AVAILABLE", "OBS_USED", "TRACKS_AVAILABLE", "TRACKS_USED", "GDOP", "SOLVE_N", "CONSIDER_N", "SENSORS_N", "WEIGHTED_RMS"]),
    ];
    for (prefix, suffix, fields) in blocks {
        for field in *fields {
            let source = format!("{HEADER}{prefix}{field} = 1\n{suffix}");
            Ocm::from_kvn(&source).unwrap_or_else(|error| panic!("{field} baseline: {error}"));
            let invalid = crate::common::mutated(
                &source,
                &format!("{field} = 1"),
                &format!("{field} = NOT_A_NUMBER"),
            );
            let error = Ocm::from_kvn(&invalid).unwrap_err();
            let parse = crate::common::kvn_parse_error(&error)
                .unwrap_or_else(|| panic!("{field}: {error}"));
            assert_eq!(
                parse.line,
                HEADER.lines().count() + prefix.lines().count() + 1,
                "{field}: {error}"
            );
            assert!(!parse.message.contains("out-of-order"), "{field}: {error}");
        }
    }
}

#[test]
fn metadata_mandatory_time_system() {
    // XSD: TIME_SYSTEM is mandatory
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
}

#[test]
fn metadata_mandatory_epoch_tzero() {
    // XSD: EPOCH_TZERO is mandatory (no minOccurs="0" attribute)
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "EPOCH_TZERO")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => assert!(err.message.contains("EPOCH_TZERO")),
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn metadata_all_optional_fields() {
    // XSD: Most metadata fields are minOccurs="0" - verify they can all be set
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SATELLITE-1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = NORAD
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT1
ORIGINATOR_POC = John Doe
OPERATOR = OPERATOR_A
OWNER = OWNER_B
COUNTRY = USA
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(
        ocm.body.segment.metadata.object_name,
        Some("SATELLITE-1".into())
    );
    assert_eq!(
        ocm.body.segment.metadata.international_designator,
        Some("2023-001A".into())
    );
    assert_eq!(
        ocm.body.segment.metadata.operator,
        Some("OPERATOR_A".into())
    );
    assert_eq!(ocm.body.segment.metadata.country, Some("USA".into()));
}

#[test]
fn test_metadata_sclk_parameters_remain_optional_outside_sclk() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.metadata.sclk_offset_at_epoch.is_none());
    assert!(ocm.body.segment.metadata.sclk_sec_per_si_sec.is_none());
}

// XSD: CENTER_NAME, TRAJ_REF_FRAME, TRAJ_TYPE mandatory
// XSD: trajLine minOccurs="1" maxOccurs="unbounded"
// XSD: traj minOccurs="0" maxOccurs="unbounded" in ocmData
// Note: Library applies defaults for some mandatory fields

#[test]
fn data_blocks_are_optional() {
    // XSD: traj minOccurs="0" - OCM can exist without trajectory block
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let data = &ocm.body.segment.data;
    assert!(data.traj.is_empty());
    assert!(data.cov.is_empty());
    assert!(data.man.is_empty());
    assert!(data.phys.is_none());
    assert!(data.pert.is_none());
    assert!(data.od.is_none());
    assert!(data.user.is_none());
}

#[test]
fn traj_mandatory_traj_type() {
    // XSD: TRAJ_TYPE is mandatory (no minOccurs="0")
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "TRAJ_TYPE")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => assert!(err.message.contains("TRAJ_TYPE")),
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn traj_multiple_blocks_unbounded() {
    // XSD: maxOccurs="unbounded" allows multiple trajectory blocks
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
TRAJ_START
CENTER_NAME = MOON
TRAJ_REF_FRAME = ICRF
TRAJ_TYPE = KEPLERIAN
2023-01-01T01:00:00 7000 0.001 28 0 0 0
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj.len(), 2);
    assert_eq!(ocm.body.segment.data.traj[0].center_name, "EARTH");
    assert_eq!(ocm.body.segment.data.traj[1].center_name, "MOON");
}

#[test]
fn traj_multiple_lines() {
    // XSD: trajLine maxOccurs="unbounded" - multiple trajectory data lines
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
2023-01-01T01:00:00 1.1 2.1 3.1 4.1 5.1 6.1
2023-01-01T02:00:00 1.2 2.2 3.2 4.2 5.2 6.2
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 3);
}

#[test]
fn traj_optional_fields() {
    // XSD: Many trajectory fields are minOccurs="0"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
TRAJ_ID = TRAJECTORY_1
TRAJ_PREV_ID = TRAJECTORY_0
TRAJ_NEXT_ID = TRAJECTORY_2
TRAJ_BASIS = DETERMINED
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 100
TRAJ_TYPE = CARTPV
ORB_AVERAGING = OSCULATING
TRAJ_UNITS = km km km km/s km/s km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let traj = &ocm.body.segment.data.traj[0];
    assert_eq!(traj.traj_id, Some("TRAJECTORY_1".into()));
    assert_eq!(traj.interpolation, Some("LAGRANGE".into()));
    assert_eq!(traj.interpolation_degree, Some(7));
    assert_eq!(traj.propagator, Some("SGP4".into()));
}

// XSD: COV_REF_FRAME, COV_TYPE, COV_ORDERING mandatory
// XSD: covLine minOccurs="1" maxOccurs="unbounded"
// XSD: cov minOccurs="0" maxOccurs="unbounded" in ocmData

#[test]
fn cov_mandatory_type() {
    // XSD: COV_TYPE is mandatory
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "COV_TYPE")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => assert!(err.message.contains("COV_TYPE")),
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn cov_multiple_blocks_unbounded() {
    // XSD: maxOccurs="unbounded" allows multiple covariance blocks
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COV_START
COV_REF_FRAME = TNW
COV_TYPE = KEPLERIAN
COV_ORDERING = UTM
2023-01-01T01:00:00 1e-5 0 1e-5 0 0 1e-5
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.cov.len(), 2);
    assert_eq!(ocm.body.segment.data.cov[0].cov_type, "CARTPV");
    assert_eq!(ocm.body.segment.data.cov[1].cov_type, "KEPLERIAN");
}

#[test]
fn cov_multiple_lines() {
    // XSD: covLine maxOccurs="unbounded"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
2023-01-01T01:00:00 1.1e-6 0 1.1e-6 0 0 1.1e-6
2023-01-01T02:00:00 1.2e-6 0 1.2e-6 0 0 1.2e-6
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.cov[0].cov_lines.len(), 3);
}

#[test]
fn cov_optional_fields() {
    // XSD: Many covariance fields are minOccurs="0"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_ID = COVARIANCE_1
COV_PREV_ID = COVARIANCE_0
COV_NEXT_ID = COVARIANCE_2
COV_BASIS = DETERMINED
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = km**2 km**2 km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let cov = &ocm.body.segment.data.cov[0];
    assert_eq!(cov.cov_id, Some("COVARIANCE_1".into()));
    assert_eq!(cov.cov_scale_min, Some(0.5));
    assert_eq!(cov.cov_scale_max, Some(2.0));
}

// XSD: MAN_ID, MAN_DEVICE_ID, MAN_REF_FRAME, DC_TYPE, MAN_COMPOSITION mandatory
// XSD: manLine minOccurs="1" maxOccurs="unbounded"
// XSD: man minOccurs="0" maxOccurs="unbounded" in ocmData

#[test]
fn man_mandatory_man_id() {
    // XSD: MAN_ID is mandatory
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "MAN_ID")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => assert!(err.message.contains("MAN_ID")),
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn man_mandatory_device_id() {
    // XSD: MAN_DEVICE_ID is mandatory
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "MAN_DEVICE_ID")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("MAN_DEVICE_ID"));
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn man_mandatory_composition() {
    // XSD: MAN_COMPOSITION is mandatory
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::MissingRequiredField { ref field, .. } => {
                assert_eq!(field, "MAN_COMPOSITION")
            }
            _ => panic!("unexpected validation error: {:?}", val_err),
        },
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(err.message.contains("MAN_COMPOSITION"));
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn man_multiple_blocks_unbounded() {
    // XSD: maxOccurs="unbounded" allows multiple maneuver blocks
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
MAN_START
MAN_ID = MANEUVER_2
MAN_DEVICE_ID = THRUSTER_2
MAN_REF_FRAME = TNW
DC_TYPE = TIME
DC_WIN_OPEN = 2023-01-02T00:00:00
DC_WIN_CLOSE = 2023-01-02T02:00:00
DC_EXEC_START = 2023-01-02T00:30:00
DC_EXEC_STOP = 2023-01-02T01:30:00
DC_REF_TIME = 2023-01-02T01:00:00
DC_TIME_PULSE_DURATION = 60 [s]
DC_TIME_PULSE_PERIOD = 120 [s]
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-02T00:00:00 0 0.1 0
MAN_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.man.len(), 2);
    assert_eq!(ocm.body.segment.data.man[0].man_id, "MANEUVER_1");
    assert_eq!(ocm.body.segment.data.man[1].man_id, "MANEUVER_2");
}

#[test]
fn man_optional_fields() {
    // XSD: Many maneuver fields are minOccurs="0"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_PREV_ID = MANEUVER_0
MAN_NEXT_ID = MANEUVER_2
MAN_BASIS = CANDIDATE
MAN_DEVICE_ID = THRUSTER_1
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
DC_TYPE = CONTINUOUS
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T01:00:00
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let man = &ocm.body.segment.data.man[0];
    assert_eq!(man.man_prev_id, Some("MANEUVER_0".into()));
    assert_eq!(man.man_purpose, Some("ORBIT_RAISING".into()));
    assert!(man.dc_win_open.is_some());
}

// XSD: phys, pert, od all minOccurs="0" (optional)
// XSD: All fields within these blocks are optional (minOccurs="0")

#[test]
fn phys_all_optional_fields() {
    // XSD: All physical description fields are minOccurs="0"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME_CORP
BUS_MODEL = LEO_BUS
DRAG_CONST_AREA = 10.0 [m**2]
DRAG_COEFF_NOM = 2.2
WET_MASS = 500 [kg]
DRY_MASS = 400 [kg]
RCS = 1.0 [m**2]
SRP_CONST_AREA = 8.0 [m**2]
SOLAR_RAD_COEFF = 1.2
MAX_THRUST = 0.1 [N]
DV_BOL = 0.3 [km/s]
DV_REMAINING = 0.15 [km/s]
PHYS_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let phys = ocm.body.segment.data.phys.as_ref().unwrap();
    assert_eq!(phys.manufacturer, Some("ACME_CORP".into()));
    assert!(phys.wet_mass.is_some());
    assert!(phys.dry_mass.is_some());
}

#[test]
fn phys_inertia_tensor() {
    // XSD: momentType for IXX, IYY, IZZ, IXY, IXZ, IYZ
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
IXX = 100 [kg*m**2]
IYY = 200 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 10 [kg*m**2]
IXZ = 5 [kg*m**2]
IYZ = 8 [kg*m**2]
PHYS_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let phys = ocm.body.segment.data.phys.as_ref().unwrap();
    assert!(phys.ixx.is_some());
    assert!(phys.iyy.is_some());
    assert!(phys.izz.is_some());
}

#[test]
fn pert_all_optional_fields() {
    // XSD: All perturbation fields are minOccurs="0"
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
PERT_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let pert = ocm.body.segment.data.pert.as_ref().unwrap();
    assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".into()));
    assert!(pert.gravity_model.is_some());
    assert!(pert.gm.is_some());
}

#[test]
fn od_all_optional_fields() {
    // XSD: OD has some mandatory fields (OD_ID, OD_METHOD, OD_EPOCH) and many optional
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD_1
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
WEIGHTED_RMS = 1.5
OD_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let od = ocm.body.segment.data.od.as_ref().unwrap();
    assert_eq!(od.od_method, "BATCH_LS");
    assert!(!od.od_epoch.to_string().is_empty());
}

#[test]
fn user_defined_optional() {
    // XSD: user minOccurs="0" - user defined block is optional
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
USER_DEFINED_CUSTOM_PARAM = custom_value
USER_DEFINED_ANOTHER_PARAM = another_value
USER_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let user = ocm.body.segment.data.user.as_ref().unwrap();
    assert_eq!(user.user_defined.len(), 2);
    assert_eq!(user.user_defined[0].parameter, "CUSTOM_PARAM");
}

#[test]
fn complex_ocm_all_blocks() {
    // OCM with trajectory, covariance, maneuver blocks
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST_SATELLITE
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj.len(), 1);
    assert_eq!(ocm.body.segment.data.cov.len(), 1);
    assert_eq!(ocm.body.segment.data.man.len(), 1);
}

#[test]
fn test_ocm_parsing_errors() {
    // Empty file
    let error = Ocm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.contexts, vec!["Empty file"]);

    // Wrong first keyword
    let err = Ocm::from_kvn("CREATION_DATE = 2023-01-01T00:00:00").unwrap_err();
    match err {
        CcsdsNdmError::Validation(val_err) if matches!(*val_err, ValidationError::MissingRequiredField { ref field, .. } if field.contains("first keyword")) =>
            {}
        CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(_)) => {}
        _ => panic!("Expected first keyword error, got: {:?}", err),
    }

    // Comments before version cannot be retained and are rejected rather than discarded.
    let kvn = r#"
COMMENT leading comment
CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
    let error = Ocm::from_kvn(kvn).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");

    // Unexpected segment start
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
TRAJ_START
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    if let Some(err) = crate::common::kvn_parse_error(&err) {
        assert!(
            err.message.contains("Expected META_START")
                || err.contexts.contains(&"Expected META_START")
        );
    } else {
        panic!("Expected KVN parse error, got: {:?}", err);
    }
    // Metadata unexpected key
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
BAD_KEY = VAL
META_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    if let Some(err) = crate::common::kvn_parse_error(&err) {
        assert!(
            err.message.contains("Expected META_STOP")
                || err.message.contains("Unexpected OCM Data key")
                || err.message.contains("Unexpected OCM Metadata key")
                || err.contexts.contains(&"Expected META_STOP")
                || err.contexts.contains(&"Unexpected OCM Data key")
                || err.contexts.contains(&"Unexpected OCM Metadata key")
        );
    } else {
        panic!("Expected KVN parse error, got: {:?}", err);
    }
}

#[test]
fn test_ocm_data_loop_break_and_comments() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT Trailing comment
UNEXPECTED_KEY = value
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("Unexpected OCM Data key")
                        || err.contexts.contains(&"Unexpected OCM Data key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got: {:?}", err),
    }
}

#[test]
fn test_ocm_user_defined_unexpected_token() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
META_START
USER_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    if let Some(err) = crate::common::kvn_parse_error(&err) {
        assert!(
            err.message.contains("Expected META_START")
                || err.message.contains("Expected TRAJ_START")
                || err.contexts.contains(&"Expected META_START")
                || err.contexts.contains(&"Expected TRAJ_START")
                || err.message.contains("Unexpected key in USER block")
                || err.contexts.contains(&"Unexpected key in USER block")
        );
    } else {
        panic!("Expected KVN parse error, got: {:?}", err);
    }
}

#[test]
fn test_phys_parsing_errors() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
DRAG_COEFF_NOM = NOT_A_FLOAT
PHYS_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    assert!(err.as_format_error().is_some());

    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q1 = NOT_A_FLOAT
PHYS_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    assert!(err.as_format_error().is_some());
}

#[test]
fn test_pert_all_fields_robust() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = SUN MOON JUPITER
OCEAN_TIDES_MODEL = GOT
SOLID_TIDES_MODEL = IERS
REDUCTION_THEORY = IAU
PERT_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let pert = ocm.body.segment.data.pert.as_ref().unwrap();
    assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".to_string()));
    assert!(pert.gm.is_some());
}

#[test]
fn test_od_all_fields_robust() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 10 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 10
TRACKS_USED = 9
WEIGHTED_RMS = 0.5
OD_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let od = ocm.body.segment.data.od.as_ref().unwrap();
    assert_eq!(od.od_id, "OD1");
    assert_eq!(od.obs_available, Some(100));
}

#[test]
fn test_cov_ordering_wcc() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTMWCC
2023-01-01T00:00:00 1e-6 1e-6 1e-6 1e-6 1e-6 1e-6
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.cov[0].cov_ordering, CovOrder::LtmWcc);

    // Also test UTMWCC
    let kvn2 = kvn.replace("LTMWCC", "UTMWCC");
    let ocm2 = Ocm::from_kvn(&kvn2).unwrap();
    assert_eq!(ocm2.body.segment.data.cov[0].cov_ordering, CovOrder::UtmWcc);

    // Also test FULL
    let kvn3 = kvn.replace("LTMWCC", "FULL");
    let ocm3 = Ocm::from_kvn(&kvn3).unwrap();
    assert_eq!(ocm3.body.segment.data.cov[0].cov_ordering, CovOrder::Full);
}

/// A header with no segment. `UnexpectedEof` is raised only by format detection and the
/// combined-NDM reader, so the OCM parser reports the block it was looking for instead.
#[test]
fn test_eof_after_meta_start() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
    let error = Ocm::from_kvn(kvn).unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!(parse.contexts, vec!["Expected META_START"]);
    assert_eq!(parse.line, 4, "diagnostic did not point past the header");
}

#[test]
fn test_empty_lines_in_metadata() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START

TIME_SYSTEM = UTC

EPOCH_TZERO = 2023-01-01T00:00:00

META_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
}

#[test]
fn test_empty_lines_in_data_section() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP

TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP

"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj.len(), 1);
}

#[test]
fn test_comments_before_blocks() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT Comment before TRAJ
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT Comment before COV
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COMMENT Comment before MAN
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
COMMENT Comment before PERT
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
COMMENT Comment before OD
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
COMMENT Comment before USER
USER_START
USER_DEFINED_PARAM = VAL
USER_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();

    // Verify comments were captured in each block
    assert!(ocm.body.segment.data.traj[0]
        .comment
        .contains(&"Comment before TRAJ".to_string()));
    assert!(ocm.body.segment.data.cov[0]
        .comment
        .contains(&"Comment before COV".to_string()));
    assert!(ocm.body.segment.data.man[0]
        .comment
        .contains(&"Comment before MAN".to_string()));
    assert!(ocm
        .body
        .segment
        .data
        .pert
        .as_ref()
        .unwrap()
        .comment
        .contains(&"Comment before PERT".to_string()));
    assert!(ocm
        .body
        .segment
        .data
        .od
        .as_ref()
        .unwrap()
        .comment
        .contains(&"Comment before OD".to_string()));
    assert!(ocm
        .body
        .segment
        .data
        .user
        .as_ref()
        .unwrap()
        .comment
        .contains(&"Comment before USER".to_string()));
}

#[test]
fn test_user_comment_inside_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
COMMENT inside user block
USER_DEFINED_PARAM = VAL
USER_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm
        .body
        .segment
        .data
        .user
        .as_ref()
        .unwrap()
        .comment
        .contains(&"inside user block".to_string()));
}

#[test]
fn test_user_empty_line_inside_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
USER_DEFINED_PARAM1 = VAL1

USER_DEFINED_PARAM2 = VAL2
USER_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(
        ocm.body
            .segment
            .data
            .user
            .as_ref()
            .unwrap()
            .user_defined
            .len(),
        2
    );
}

#[test]
fn test_traj_empty_line_inside_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF

TRAJ_TYPE = CARTPV

2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 1);
}

#[test]
fn test_phys_empty_line_inside_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME

WET_MASS = 500 [kg]
PHYS_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.phys.is_some());
}

#[test]
fn test_od_empty_line_inside_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1

OD_METHOD = LS

OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.od.is_some());
}

#[test]
fn test_traj_missing_lines() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
TRAJ_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("trajLine")
                        || err.contexts.iter().any(|c| c.contains("trajLine"))
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected trajLine missing error, got: {:?}", err),
    }
}

#[test]
fn test_traj_unknown_key_error() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
UNKNOWN_KEY = IGNORED_VALUE
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    // Should FAIL, not ignoring unknown keys
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("Unexpected OCM Trajectory key")
                        || err.contexts.contains(&"Unexpected OCM Trajectory key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got {:?}", err),
    }
}

#[test]
fn test_phys_unknown_key_error() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
UNKNOWN_PHYS_KEY = IGNORED
PHYS_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("Unexpected OCM Physical key")
                        || err.contexts.contains(&"Unexpected OCM Physical key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got {:?}", err),
    }
}

#[test]
fn test_cov_comment_and_empty_line() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT inside cov block

COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.cov[0]
        .comment
        .contains(&"inside cov block".to_string()));
}

#[test]
fn test_comment_before_cov_block() {
    // ...
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT This comment should be prepended to COV
COV_START
COV_ID = COV1
COV_TYPE = ANGLE
COV_ORDERING = LTM
UNKNOWN_COV_KEY = some_value
CX_X = 1.0
COV_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("Unexpected OCM Covariance key")
                        || err.contexts.contains(&"Unexpected OCM Covariance key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got {:?}", err),
    }
}

#[test]
fn test_comment_before_phys_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT This comment should be prepended to PHYS
PHYS_START
WET_MASS = 1000.0 [kg]
PHYS_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.phys.is_some());
    let phys = ocm.body.segment.data.phys.unwrap();
    assert!(phys.comment.iter().any(|c| c.contains("prepended to PHYS")));
}

#[test]
fn test_missing_meta_start() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
CENTER_NAME = EARTH
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    assert!(
        crate::common::kvn_parse_error(&err).is_some_and(|err| {
            err.message.contains("Expected META_START")
                || err
                    .contexts
                    .iter()
                    .any(|context| context.contains("Expected META_START"))
        }),
        "Expected 'expected meta' error, got: {:?}",
        err
    );
}

#[test]
fn test_empty_line_in_man_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1

MAN_REF_FRAME = TNW
MAN_COMPOSITION = TIME_ABSOLUTE
MAN_UNITS = km/s
2023-01-01T00:00:00.000 0.1 0.2 0.3
MAN_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(!ocm.body.segment.data.man.is_empty());
}

#[test]
fn test_empty_line_in_pert_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSIS00

GRAVITY_MODEL = EGM-96
PERT_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.pert.is_some());
}

#[test]
fn test_unknown_key_in_od_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
UNKNOWN_OD_KEY = some_value
OD_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message
                        .contains("Unexpected OCM Orbit Determination key")
                        || err
                            .contexts
                            .contains(&"Unexpected OCM Orbit Determination key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got {:?}", err),
    }
}

#[test]
fn test_unknown_key_in_cov_block() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
UNKNOWN_COV_KEY = some_value
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
    let err = Ocm::from_kvn(kvn).unwrap_err();
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("Unexpected OCM Covariance key")
                        || err.contexts.contains(&"Unexpected OCM Covariance key")
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected KvnParse error, got {:?}", err),
    }
}

#[test]
fn ocm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let center = "CENTER_NAME = EARTH";
    let frame = "TRAJ_REF_FRAME = TOD_EARTH";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            mutated(
                KVN,
                "TIME_SYSTEM = UTC",
                "TIME_SYSTEM = UTC\nTIME_SYSTEM = UTC",
            ),
        ),
        (
            "reordered trajectory keywords",
            mutated(
                KVN,
                &format!("{center}\n{frame}"),
                &format!("{frame}\n{center}"),
            ),
        ),
        (
            "unknown trajectory keyword",
            mutated(KVN, center, &format!("{center}\nUNKNOWN = value")),
        ),
        (
            "comment after trajectory content",
            mutated(KVN, center, &format!("{center}\nCOMMENT misplaced")),
        ),
        ("unknown block", mutated(KVN, "TRAJ_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            mutated(KVN, "TRAJ_STOP", "COV_STOP"),
        ),
        (
            "out-of-order logical block",
            mutated(
                &mutated(KVN, "PHYS_START", "PERT_START"),
                "PHYS_STOP",
                "PERT_STOP",
            ),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII assignment",
            mutated(KVN, center, &format!("{center} €")),
        ),
    ] {
        let error = Ocm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn ocm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let center = "<CENTER_NAME>EARTH</CENTER_NAME>";
    let time_system = "<TIME_SYSTEM>UT1</TIME_SYSTEM>";
    for (label, source) in [
        (
            "unknown data child",
            mutated(XML, "<data>", "<data><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown trajectory child",
            mutated(XML, "<traj>", "<traj><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown leaf attribute",
            mutated(XML, center, "<CENTER_NAME unexpected=\"value\">EARTH</CENTER_NAME>"),
        ),
        (
            "duplicate metadata child",
            mutated(XML, time_system, &format!("{time_system}{time_system}")),
        ),
        (
            "reordered metadata children",
            mutated(XML, "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>\n<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>",
                "<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>\n<OBJECT_NAME>OSPREY 5</OBJECT_NAME>",
            ),
        ),
    ] {
        let error = Ocm::from_xml(&source).unwrap_err();
        assert!(matches!(error.as_format_error(), Some(ccsds_ndm::error::FormatError::InvalidFormat(_))), "{label}: {error}");
    }
}

#[test]
fn parse_simple_ocm() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj.len(), 1);
    assert_eq!(ocm.body.segment.data.traj[0].traj_lines[0].values.len(), 6);
}

#[test]
fn malformed_ocm_records_are_never_silent() {
    let invalid = crate::common::mutated_once(
        include_str!("../../data/kvn/ocm_g15.kvn"),
        "120.0 5478.6",
        "MALFORMED TRAJECTORY RECORD\n120.0 5478.6",
    );

    assert_eq!(
        Ocm::from_kvn(&invalid).unwrap_err().code(),
        Some("parse.kvn.syntax")
    );
}

#[test]
fn missing_ocm_record_block_stops_fail_at_eof() {
    let trajectory_without_stop = include_str!("../../data/kvn/ocm_g15.kvn")
        .strip_suffix("TRAJ_STOP\n")
        .expect("fixture ends with TRAJ_STOP");
    assert_eq!(
        Ocm::from_kvn(trajectory_without_stop).unwrap_err().code(),
        Some("parse.kvn.syntax")
    );

    let maneuver_without_stop = include_str!("../../data/kvn/ocm_g17.kvn")
        .split_once("\nMAN_STOP")
        .expect("fixture contains MAN_STOP")
        .0;
    assert_eq!(
        Ocm::from_kvn(maneuver_without_stop).unwrap_err().code(),
        Some("parse.kvn.syntax")
    );
}

#[test]
fn validation_error_is_preserved() {
    // WET_MASS negative -> OutOfRange Validation Error
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nPHYS_START\nWET_MASS = -100 [kg]\nPHYS_STOP\n";
    let err = Ocm::from_kvn(kvn).unwrap_err();

    // Should be CcsdsNdmError::Validation
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::OutOfRange { name, value, .. } => {
                assert_eq!(name, "Mass");
                assert_eq!(value, "-100");
            }
            _ => panic!("Expected OutOfRange, got {:?}", val_err),
        },
        _ => panic!("Expected CcsdsNdmError::Validation, got {:?}", err),
    }
}

#[test]
fn enum_error_is_preserved() {
    // CENTER_NAME and TIME_SYSTEM are Strings. OBJECT_TYPE has a fallback.
    // COV_ORDERING is a strict enum.
    let kvn = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nOBJECT_TYPE = PAYLOAD\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = EME2000\nCOV_TYPE = POSITION_COVARIANCE\nCOV_ORDERING = INVALID_ORDERING\nCOV_STOP\n";
    let err = Ocm::from_kvn(kvn).unwrap_err();

    let CcsdsNdmError::Format(format_error) = err else {
        panic!("Expected FormatError");
    };
    if let ccsds_ndm::error::FormatError::Enum(enum_err) = *format_error {
        assert_eq!(enum_err.value, "INVALID_ORDERING");
        assert_eq!(enum_err.field, "COV_ORDERING");
    } else {
        panic!("Expected EnumParseError, got {format_error:?}");
    }
}
