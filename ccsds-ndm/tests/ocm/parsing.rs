use crate::common::mutated;
use crate::{KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, FormatError, ValidationError};
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::Ndm;
/// Each numeric parser is reached with an otherwise valid, correctly ordered block.
#[test]
fn malformed_numbers_identify_the_mutated_record() {
    const HEADER: &str = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\n";
    let blocks: &[(&str, &str, &[&str])] = &[
        ("TRAJ_START\n", "CENTER_NAME = EARTH\nTRAJ_REF_FRAME = GCRF\nTRAJ_TYPE = CARTPV\n0 1 2 3 4 5 6\nTRAJ_STOP\n", &["INTERPOLATION_DEGREE"]),
        ("TRAJ_START\nCENTER_NAME = EARTH\nTRAJ_REF_FRAME = GCRF\n", "ORB_REVNUM_BASIS = 0\nTRAJ_TYPE = CARTPV\n0 1 2 3 4 5 6\nTRAJ_STOP\n", &["ORB_REVNUM"]),
        ("PHYS_START\n", "PHYS_STOP\n", &["DRAG_COEFF_NOM", "OEB_Q1", "OEB_Q2", "OEB_Q3", "OEB_QC", "SOLAR_RAD_COEFF", "VM_ABSOLUTE", "VM_APPARENT_MIN", "VM_APPARENT", "VM_APPARENT_MAX", "REFLECTANCE"]),
        ("COV_START\nCOV_REF_FRAME = GCRF\n", "COV_TYPE = CARTP\nCOV_ORDERING = LTM\n0 1 0 1 0 0 1\nCOV_STOP\n", &["COV_SCALE_MIN", "COV_SCALE_MAX"]),
        ("MAN_START\nMAN_ID = MAN1\nMAN_DEVICE_ID = DEV1\nMAN_REF_FRAME = TNW\n", "MAN_COMPOSITION = TIME_ABSOLUTE, THR_X, THR_Y, THR_Z\n2023-01-01T00:00:00 0.1 0.2 0.3\nMAN_STOP\n", &["DC_MIN_CYCLES", "DC_MAX_CYCLES"]),
        ("PERT_START\n", "PERT_STOP\n", &["OBLATE_FLATTENING", "ALBEDO_GRID_SIZE"]),
        ("PERT_START\nPERT_STOP\nOD_START\nOD_ID = OD1\nOD_METHOD = LS\nOD_EPOCH = 2023-01-01T00:00:00\n", "OD_STOP\n", &["OBS_AVAILABLE", "OBS_USED", "TRACKS_AVAILABLE", "TRACKS_USED", "GDOP", "SOLVE_N", "CONSIDER_N", "SENSORS_N", "WEIGHTED_RMS"]),
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
    // The CARTPV default requires a full six-dimensional covariance.
    let kvn = kvn.replace(
        "1e-6 0 1e-6 0 0 1e-6",
        "1 0 1 0 0 1 0 0 0 1 0 0 0 0 1 0 0 0 0 0 1",
    );
    let parsed = Ocm::from_kvn(&kvn).unwrap();
    assert_eq!(parsed.body.segment.data.cov[0].cov_type, "CARTPV");
    crate::common::validate_xml("default covariance type", &parsed.to_xml().unwrap());
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
TRAJ_START
TRAJ_ID = TRAJECTORY
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
0 1 2 3 4 5 6
TRAJ_STOP
COV_START
TRAJ_ID = TRAJECTORY
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 0.000001 0 0.000001 0 0 0.000001 0 0 0 0.000001 0 0 0 0 0.000001 0 0 0 0 0 0.000001
COV_STOP
COV_START
TRAJ_ID = TRAJECTORY
COV_REF_FRAME = TNW
COV_TYPE = KEPLERIAN
COV_ORDERING = UTM
2023-01-01T01:00:00 0.000001 0 0 0 0 0 0.000001 0 0 0 0 0.000001 0 0 0 0.000001 0 0 0.000001 0 0.000001
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.cov.len(), 2);
    assert_eq!(ocm.body.segment.data.cov[0].cov_type, "CARTPV");
    assert_eq!(ocm.body.segment.data.cov[1].cov_type, "KEPLERIAN");
}

// XSD: MAN_ID, MAN_DEVICE_ID, MAN_REF_FRAME, DC_TYPE, MAN_COMPOSITION mandatory
// XSD: manLine minOccurs="1" maxOccurs="unbounded"
// XSD: man minOccurs="0" maxOccurs="unbounded" in ocmData

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
    assert_eq!(user.parameters().count(), 2);
    assert_eq!(user.parameters().next().unwrap().parameter, "CUSTOM_PARAM");
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
    let err = crate::common::parse_error_cause(err);
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
    assert!(
        parse.message.contains("Expected META_START")
            || parse.contexts.contains(&"Expected META_START"),
        "{error}"
    );
    assert_eq!(parse.line, 4, "diagnostic did not point past the header");
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
        .comments()
        .any(|value| value == "inside user block"));
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
    let err = crate::common::parse_error_cause(err);
    match err {
        CcsdsNdmError::Format(format_err) => match *format_err {
            FormatError::Kvn(ref err) => {
                assert!(
                    err.message.contains("TRAJ data line")
                        || err.contexts.iter().any(|c| c.contains("TRAJ data line"))
                );
            }
            _ => panic!("unexpected format error: {:?}", format_err),
        },
        _ => panic!("Expected trajLine missing error, got: {:?}", err),
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
2023-01-01T00:00:00 0.000001 0 0.000001 0 0 0.000001 0 0 0 0.000001 0 0 0 0 0.000001 0 0 0 0 0 0.000001
COV_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert!(ocm.body.segment.data.cov[0]
        .comment
        .contains(&"inside cov block".to_string()));
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
    let error = Ocm::from_kvn(kvn).unwrap_err();
    assert!(
        error.to_string().contains("COMMENT is only allowed"),
        "{error}"
    );
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
fn lowercase_maneuver_units_survive_both_notations() {
    let source = format!(
        "{}MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
MAN_REF_FRAME = TNW
MAN_COMPOSITION = TIME_ABSOLUTE, THR_X, THR_Y, THR_Z
MAN_UNITS = n, n, n
2023-01-01T00:00:00.000 0.1 0.2 0.3
MAN_STOP
",
        super::minimal::KVN
    );
    let message = Ocm::from_kvn(&source).unwrap();
    assert_eq!(
        message.body.segment.data.man[0].man_units.as_deref(),
        Some("[n,n,n]")
    );
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert_eq!(Ocm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
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
    let err = crate::common::parse_error_cause(err);
    match err {
        CcsdsNdmError::Validation(val_err) => match *val_err {
            ValidationError::OutOfRange { name, value, .. } => {
                assert_eq!(name, "WET_MASS");
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

    let Some(format_error) = err.as_format_error() else {
        panic!("Expected FormatError");
    };
    if let ccsds_ndm::error::FormatError::Enum(enum_err) = format_error {
        assert_eq!(enum_err.value, "INVALID_ORDERING");
        assert_eq!(enum_err.field, "COV_ORDERING");
    } else {
        panic!("Expected EnumParseError, got {format_error:?}");
    }
}

#[test]
fn trajectory_numbers_accept_producer_spellings_without_underflow() {
    let source = include_str!("../../data/kvn/ocm_g15.kvn");
    let message = Ocm::from_kvn(source).unwrap();
    let generated = message.to_kvn().unwrap();
    let needle = generated
        .lines()
        .find(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|value| value.parse::<f64>().is_ok())
        })
        .unwrap();
    let mut tokens: Vec<_> = needle.split_whitespace().collect();
    tokens[1] = "1e3";
    let legal = generated.replacen(needle, &tokens.join(" "), 1);
    assert!(Ocm::from_kvn(&legal).is_ok());
    tokens[1] = "1e-9999";
    let underflow = generated.replacen(needle, &tokens.join(" "), 1);
    assert!(Ocm::from_kvn(&underflow).is_err());
}

#[test]
fn producer_defaults_open_tokens_and_exponent_tags_round_trip() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nOBJECT_TYPE = ROCKET_BODY\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nTRAJ_START\nTRAJ_BASIS = SIMULATION\n1e1 1 2 3 4 5 6\nTRAJ_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();
    assert_eq!(message.body.segment.metadata.time_system, "UTC");
    assert_eq!(
        message.body.segment.metadata.object_type.as_deref(),
        Some("ROCKET BODY")
    );
    let trajectory = &message.body.segment.data.traj[0];
    assert_eq!(trajectory.traj_type, "CARTPV");
    assert_eq!(trajectory.traj_basis.as_deref(), Some("SIMULATION"));
    assert_eq!(trajectory.traj_lines[0].epoch.as_str(), "10");
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert!(
        message.to_xml().is_err(),
        "book free-text basis is outside the XML enumeration"
    );
    let mut message = message;
    message.body.segment.data.traj[0].traj_basis = None;
    crate::common::validate_xml("OCM producer defaults", &message.to_xml().unwrap());
    for tag in ["1e-2147483648", "1e999999999999", "1e-400"] {
        assert!(Ocm::from_kvn(&source.replace("1e1 1", &format!("{tag} 1"))).is_err());
    }
}

#[test]
fn trailing_comments_are_refused_without_losing_them() {
    let message = Ocm::from_kvn(KVN).unwrap();
    let canonical = message.to_kvn().unwrap();
    let error = Ocm::from_kvn(&format!("{canonical}\nCOMMENT cannot be dropped\n")).unwrap_err();
    assert!(error.to_string().contains("COMMENT"), "{error}");
}

#[test]
fn xml_history_numbers_reject_underflow_and_tag_only_unknown_compositions() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nTRAJ_START\nTRAJ_TYPE = FUTURE_TYPE\n0 1\nTRAJ_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();
    let xml = message.to_xml().unwrap();
    assert!(Ocm::from_xml(
        &xml.replace("<trajLine>0 1</trajLine>", "<trajLine>0 1e-400</trajLine>")
    )
    .is_err());
    assert!(
        Ocm::from_xml(&xml.replace("<trajLine>0 1</trajLine>", "<trajLine>0</trajLine>")).is_err()
    );
    let mut message = message;
    message.body.segment.data.traj[0].traj_lines[0]
        .values
        .clear();
    crate::common::assert_rejects(&message, "TRAJ data line 1");
}

#[test]
fn optional_absence_survives_producer_kvn() {
    for absent in ["", "N/A", "n/a"] {
        let source = mutated(
            KVN,
            "INTERNATIONAL_DESIGNATOR = 2022-999A",
            &format!("INTERNATIONAL_DESIGNATOR = {absent}"),
        );
        assert!(Ocm::from_kvn(&source)
            .unwrap()
            .body
            .segment
            .metadata
            .international_designator
            .is_none());
    }
}

#[test]
fn xml_covariance_history_rejects_underflow_and_empty_unknown_rows() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_TYPE = FUTURE_TYPE\nCOV_ORDERING = LTM\n0 1\nCOV_STOP\n";
    let message = Ocm::from_kvn(source).unwrap();
    let xml = message.to_xml().unwrap();
    for replacement in ["<covLine>0 1e-400</covLine>", "<covLine>0</covLine>"] {
        let invalid = mutated(&xml, "<covLine>0 1</covLine>", replacement);
        assert!(Ocm::from_xml(&invalid).is_err());
    }
}

#[test]
fn schema_valid_xml_covariances_do_not_require_tc1_linkage() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = GCRF\nCOV_TYPE = CARTP\nCOV_ORDERING = LTM\n0 1 0 1 0 0 1\nCOV_STOP\n";
    let base = Ocm::from_kvn(source).unwrap().to_xml().unwrap();
    let start = base.find("<cov>").expect("fixture covariance");
    let end = start + base[start..].find("</cov>").unwrap() + "</cov>".len();
    let source = format!("{}{}{}", &base[..end], &base[start..end], &base[end..]);
    crate::common::validate_xml("OCM repeated covariance without XML TRAJ_ID", &source);
    let message = Ocm::from_xml(&source).unwrap();
    assert_eq!(message.body.segment.data.cov.len(), 2);
    assert!(message
        .to_kvn()
        .unwrap_err()
        .to_string()
        .contains("TRAJ_ID"));
    assert!(ccsds_ndm::Validate::validate(&message).is_err());
    let xml = message.to_xml().unwrap();
    crate::common::validate_xml("OCM repeated covariance round trip", &xml);
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    let mut streamed = Vec::new();
    message.write_xml_to(&mut streamed).unwrap();
    assert_eq!(streamed, xml.as_bytes());
    let combined = ccsds_ndm::messages::ndm::CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![ccsds_ndm::Message::Ocm(message)],
    };
    let xml = combined.to_xml().unwrap();
    crate::common::validate_xml("combined OCM repeated covariance", &xml);
    assert_eq!(
        ccsds_ndm::messages::ndm::CombinedNdm::from_xml(&xml).unwrap(),
        combined
    );
}

#[test]
fn malformed_covariance_rows_never_disappear() {
    let source = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\nCOV_START\nCOV_REF_FRAME = GCRF\nCOV_TYPE = CARTP\nCOV_ORDERING = LTM\n0 1 0 1 0 0 1\n10 1 0 1 0 0 1\n20 1 0 1 0 0 1\nCOV_STOP\n";
    let cov = Ocm::from_kvn(source).unwrap().body.segment.data.cov;
    assert_eq!(cov[0].cov_lines.len(), 3);
    for bad in [
        "abc",
        "10 1 0 abc 0 0 1",
        "10 1 0 1e-400 0 0 1",
        "10 1 0 1 0 0 1 x",
        "10 1 0 1 0 0 1 trailing",
        "10 abc",
    ] {
        let error = Ocm::from_kvn(&mutated(source, "10 1 0 1 0 0 1\n", &format!("{bad}\n")))
            .unwrap_err()
            .to_string();
        assert!(error.contains("line 13"), "{bad}: {error}");
    }
    assert!(Ocm::from_kvn(&mutated(source, "COV_STOP\n", "")).is_err());
}

#[test]
fn maneuver_required_fields_name_the_missing_assignment() {
    let source = format!(
        "{}MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
",
        super::minimal::KVN
    );
    Ocm::from_kvn(&source).unwrap();
    for assignment in [
        "MAN_ID = MAN1\n",
        "MAN_DEVICE_ID = DEV1\n",
        "MAN_COMPOSITION = TIME_ABSOLUTE, DV_X, DV_Y, DV_Z\n",
    ] {
        let input = mutated(&source, assignment, "");
        let error = Ocm::from_kvn(&input).unwrap_err();
        assert_eq!(
            error.code(),
            Some("validation.missing_required_field"),
            "{error}"
        );
        crate::common::assert_validation_field(&error, assignment.split_once(" = ").unwrap().0);
    }
}

#[test]
fn unknown_keys_name_each_logical_block() {
    let covariance = include_str!("../../data/kvn/ocm_g19.kvn");
    for (source, marker, name) in [
        (KVN, "TRAJ_START", "Trajectory"),
        (KVN, "PHYS_START", "Physical"),
        (KVN, "OD_START", "Orbit Determination"),
        (covariance, "COV_START", "Covariance"),
    ] {
        Ocm::from_kvn(source).unwrap();
        let input = mutated(source, marker, &format!("{marker}\nUNKNOWN_KEY = value"));
        let error = Ocm::from_kvn(&input).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{name}: {error}");
        assert!(
            error
                .to_string()
                .contains(&format!("Unexpected OCM {name} key")),
            "{error}"
        );
    }
}
