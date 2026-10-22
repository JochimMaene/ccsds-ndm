use crate::common::mutated;
use crate::{ATT_KVN, COV_KVN};
use ccsds_ndm::error::{CcsdsNdmError, ValidationError};
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::types::AcmAttitudeType;
use ccsds_ndm::Ndm;
fn sample_acm_header() -> String {
    r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
    .to_string()
}

fn sample_acm_meta() -> String {
    r#"META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
"#
    .to_string()
}

#[test]
fn test_parse_acm_minimal() {
    let input = format!("{}{}\nATT_START\nREF_FRAME_A = EME2000\nREF_FRAME_B = SC_BODY_1\nNUMBER_STATES = 4\nATT_TYPE = QUATERNION\n0.0 0.5 0.5 0.5 0.5\nATT_STOP\n",
        sample_acm_header(), sample_acm_meta());
    let acm = Acm::from_kvn(&input).unwrap();
    assert_eq!(acm.version, "2.0");
    assert_eq!(acm.header.originator, "NASA/JPL");
    assert_eq!(
        acm.body.segment.metadata.object_name,
        "MARS GLOBAL SURVEYOR"
    );
    assert_eq!(acm.body.segment.data.att.len(), 1);
    assert_eq!(acm.body.segment.data.att[0].att_lines.len(), 1);
}

#[test]
fn test_parse_acm_version_error() {
    let input = r#"CCSDS_ACM_VERS = 3.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0.5 0.5 0.5 0.5
ATT_STOP
"#;
    let err = Acm::from_kvn(input).unwrap_err();
    match err {
        CcsdsNdmError::UnsupportedInputVersion { version, .. } => {
            assert_eq!(version, "3.0");
        }
        _ => panic!("Expected unsupported input version, got {:?}", err),
    }
}

#[test]
fn test_acm_missing_mandatory_metadata() {
    let input = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
"#;
    let err = Acm::from_kvn(input).unwrap_err();
    match err {
        CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
            ValidationError::MissingRequiredField { field, .. } => {
                assert_eq!(field, "OBJECT_NAME");
            }
            _ => panic!("Expected missing field error, got {:?}", boxed_err),
        },
        _ => panic!("Expected Validation error, got {:?}", err),
    }
}

#[test]
fn test_acm_att_block_variants() {
    let att_block = r#"ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 3
ATT_TYPE = EULER_ANGLES
EULER_ROT_SEQ = ZYX
0.0 10.0 20.0 30.0
ATT_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), att_block);
    let acm = Acm::from_kvn(&input).unwrap();
    let att = &acm.body.segment.data.att[0];
    assert_eq!(att.att_type, AcmAttitudeType::EulerAngles);
}

#[test]
fn test_acm_phys_block_full() {
    let phys_block = r#"PHYS_START
COMMENT Phys comment
DRAG_COEFF = 2.2
WET_MASS = 1500.0 [kg]
DRY_MASS = 1000.0 [kg]
CP_REF_FRAME = SC_BODY_1
CP = 0.1 0.2 0.3 [m]
INERTIA_REF_FRAME = SC_BODY_1
IXX = 1000.0 [kg*m**2]
IYY = 2000.0 [kg*m**2]
IZZ = 3000.0 [kg*m**2]
IXY = 10.0 [kg*m**2]
IXZ = 20.0 [kg*m**2]
IYZ = 30.0 [kg*m**2]
PHYS_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), phys_block);
    let acm = Acm::from_kvn(&input).unwrap();
    let phys = acm.body.segment.data.phys.as_ref().unwrap();
    assert_eq!(phys.drag_coeff, Some(2.2));
    assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1500.0);
    assert_eq!(phys.cp.as_ref().unwrap().elements[0], 0.1);
    assert_eq!(phys.ixx.as_ref().unwrap().value, 1000.0);
    assert!(phys.comment.contains(&"Phys comment".to_string()));
}

#[test]
fn test_acm_cov_block() {
    let cov_block = r#"COV_START
COV_BASIS = DETERMINED_OBC
COV_REF_FRAME = EME2000
COV_TYPE = ANGLE
0.0 1.0e-6 2.0e-6 3.0e-6
COV_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), cov_block);
    let acm = Acm::from_kvn(&input).unwrap();
    let cov = &acm.body.segment.data.cov[0];
    assert_eq!(
        cov.cov_basis.as_ref().unwrap().to_string(),
        "DETERMINED_OBC"
    );
    assert_eq!(cov.cov_lines[0].values[0], 1.0e-6);
}

#[test]
fn test_acm_man_block() {
    let man_block = r#"MAN_START
MAN_ID = MAN_001
MAN_PURPOSE = MOM_DESAT
MAN_BEGIN_TIME = 100.0
MAN_DURATION = 100.0 [s]
ACTUATOR_USED = THRUSTER_1
TARGET_MOMENTUM = 0.1 0.2 0.3 [N*m*s]
TARGET_MOM_FRAME = J2000
MAN_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), man_block);
    let acm = Acm::from_kvn(&input).unwrap();
    let man = &acm.body.segment.data.man[0];
    assert_eq!(man.man_id.as_deref(), Some("MAN_001"));
    assert_eq!(man.man_duration.as_ref().unwrap().value, 100.0);
    assert_eq!(man.target_momentum.as_ref().unwrap().elements[0], 0.1);
}

#[test]
fn test_acm_ad_block_with_sensors() {
    let ad_block = r#"AD_START
AD_ID = AD_001
AD_METHOD = EKF
ATTITUDE_SOURCE = OBC
ATTITUDE_STATES = QUATERNION
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
SENSOR_START
SENSOR_NUMBER = 1
SENSOR_USED = STAR_TRACKER
SENSOR_NOISE_STDDEV = 0.01 [deg]
SENSOR_FREQUENCY = 10.0
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 2
SENSOR_USED = GYRO
SENSOR_STOP
AD_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), ad_block);
    let acm = Acm::from_kvn(&input).unwrap();
    let ad = &acm.body.segment.data.ad.as_ref().unwrap();
    assert_eq!(ad.ad_id.as_deref(), Some("AD_001"));
    assert_eq!(ad.ad_method.as_deref(), Some("EKF"));
    assert_eq!(ad.sensors.len(), 2);
}

#[test]
fn test_acm_multiple_blocks_mixed() {
    let blocks = r#"ATT_START
REF_FRAME_A = A
REF_FRAME_B = B
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0 0 0 0 1
ATT_STOP
ATT_START
REF_FRAME_A = A
REF_FRAME_B = B
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
10 0 0 0 1
ATT_STOP
PHYS_START
DRAG_COEFF = 1.0
PHYS_STOP
"#;
    let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), blocks);
    let acm = Acm::from_kvn(&input).unwrap();
    assert_eq!(acm.body.segment.data.att.len(), 2);
    assert!(acm.body.segment.data.phys.is_some());
}

#[test]
fn acm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object = "OBJECT_NAME = SDO";
    let designator = "INTERNATIONAL_DESIGNATOR = 2010-005A";
    let first_state = "0.000000 0.1153 -0.1424 0.8704 0.4571 2.271e-06 -4.405e-06 -3.785e-06";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            mutated(ATT_KVN, object, &format!("{object}\n{object}")),
        ),
        (
            "reordered metadata",
            mutated(
                ATT_KVN,
                &format!("{object}\n{designator}"),
                &format!("{designator}\n{object}"),
            ),
        ),
        (
            "unknown attitude keyword",
            mutated(
                ATT_KVN,
                "ATT_TYPE = QUATERNION",
                "ATT_TYPE = QUATERNION\nUNKNOWN = 1",
            ),
        ),
        (
            "comment after history",
            mutated(
                ATT_KVN,
                first_state,
                &format!("{first_state}\nCOMMENT misplaced"),
            ),
        ),
        (
            "assignment after history",
            mutated(
                ATT_KVN,
                first_state,
                &format!("{first_state}\nRATE_TYPE = GYRO_BIAS"),
            ),
        ),
        (
            "sensor outside AD",
            mutated(ATT_KVN, "ATT_STOP", "ATT_STOP\nSENSOR_START\nSENSOR_STOP"),
        ),
        ("mismatched block", mutated(ATT_KVN, "ATT_STOP", "COV_STOP")),
        (
            "unknown block",
            mutated(ATT_KVN, "ATT_START", "UNKNOWN_START"),
        ),
        ("trailing assignment", format!("{ATT_KVN}UNKNOWN = value\n")),
    ] {
        let error = Acm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn acm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let xml = Acm::from_kvn(COV_KVN).unwrap().to_xml().unwrap();
    let object = "<OBJECT_NAME>LRO</OBJECT_NAME>";
    let designator = "<INTERNATIONAL_DESIGNATOR>2009-031A</INTERNATIONAL_DESIGNATOR>";
    for (label, source) in [
        (
            "unknown covariance child",
            mutated(&xml, "<cov>", "<cov><UNKNOWN/>"),
        ),
        (
            "illegal covariance-line attribute",
            mutated(&xml, "<covLine>", "<covLine units=\"1\">"),
        ),
        (
            "unknown sensor attribute",
            mutated(&xml, "<sensorData>", "<sensorData unexpected=\"value\">"),
        ),
        (
            "duplicate covariance type",
            mutated(&xml, "</COV_TYPE>", "</COV_TYPE><COV_TYPE>ANGLE</COV_TYPE>"),
        ),
        (
            "reordered metadata",
            mutated(
                &xml,
                &format!("{object}{designator}"),
                &format!("{designator}{object}"),
            ),
        ),
    ] {
        let error = Acm::from_xml(&source).unwrap_err();
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
fn acm_multiple_att_blocks() {
    let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = INSTRUMENT
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
"#;
    let acm = Acm::from_kvn(kvn).unwrap();
    assert_eq!(acm.body.segment.data.att.len(), 2);
}

#[test]
fn acm_physical_block() {
    let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
WET_MASS = 1000 [kg]
DRY_MASS = 500 [kg]
PHYS_STOP
"#;
    let acm = Acm::from_kvn(kvn).unwrap();
    let phys = acm.body.segment.data.phys.as_ref().unwrap();
    assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1000.0);
}

#[test]
fn euler_history_preserves_unwrapped_finite_angles() {
    let input = format!("{}{}ATT_START\nREF_FRAME_A = ICRF\nREF_FRAME_B = SC_BODY_1\nNUMBER_STATES = 3\nATT_TYPE = EULER_ANGLES\nEULER_ROT_SEQ = ZYX\n0.0 999999.0 0.0 0.0\nATT_STOP\n", sample_acm_header(), sample_acm_meta());
    let message = Acm::from_kvn(&input).unwrap();
    assert_eq!(
        message.body.segment.data.att[0].att_lines[0].values[0],
        999999.0
    );
    let xml = message.to_xml().unwrap();
    crate::common::validate_xml("unwrapped Euler history", &xml);
    assert_eq!(Acm::from_xml(&xml).unwrap(), message);
    assert_eq!(Acm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
}

#[test]
fn history_row_errors_name_the_problem_at_the_row() {
    let row = "2.000000 0.1153 -0.1424 0.8704 0.4571 2.271e-06 -4.405e-06 -3.785e-06";
    let error =
        Acm::from_kvn(&mutated(ATT_KVN, row, &row.replace("0.1153", "0.1x53"))).unwrap_err();
    let kvn = crate::common::kvn_parse_error(&error).expect("a KVN parse error");
    assert_eq!(
        (kvn.line, kvn.message.as_str()),
        (
            19,
            "Invalid ATT data line: expected a time tag followed by numbers"
        )
    );
}

#[test]
fn orekit_euler_rot_seq_before_number_states_is_read_and_written_in_table_order() {
    // Orekit writes EULER_ROT_SEQ before NUMBER_STATES in ATT and AD; the order is lossless.
    let att = mutated(
        include_str!("../../data/kvn/acm_g6.kvn"),
        "NUMBER_STATES = 4\nATT_TYPE = QUATERNION\n0.0 0.73566 -0.50547 0.41309 0.180707\n0.25 0.73529 -0.50531 0.41375 0.181158\n0.50 0.73492 -0.50515 0.41441 0.181610",
        "EULER_ROT_SEQ = XYZ\nNUMBER_STATES = 3\nATT_TYPE = EULER_ANGLES\n0.0 1.0 2.0 3.0",
    );
    let ad = mutated(
        COV_KVN,
        "ATTITUDE_SOURCE = OBC\n",
        "ATTITUDE_SOURCE = OBC\nEULER_ROT_SEQ = ZXZ\n",
    );
    for (source, canonical) in [
        (att, "ATT_TYPE = EULER_ANGLES\nEULER_ROT_SEQ = XYZ"),
        (ad, "ATTITUDE_STATES = QUATERNION\nEULER_ROT_SEQ = ZXZ"),
    ] {
        let message = Acm::from_kvn(&source).unwrap();
        let generated = message.to_kvn().unwrap();
        let compact: String = generated
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(compact.contains(canonical), "{generated}");
        assert_eq!(Acm::from_kvn(&generated).unwrap(), message);
        let twice = source.replacen("NUMBER_STATES", "EULER_ROT_SEQ = XYZ\nNUMBER_STATES", 1);
        let anchor = canonical.lines().next().unwrap();
        let canonical_too = mutated(&source, anchor, &format!("{anchor}\nEULER_ROT_SEQ = XYZ"));
        for duplicate in [twice, canonical_too] {
            let error = Acm::from_kvn(&duplicate).unwrap_err().to_string();
            assert!(error.contains("duplicate"), "{error}");
        }
    }
}

#[test]
fn user_comments_may_precede_any_parameter_and_keep_their_position() {
    // ADM 5.3.10.5: each user-defined parameter line may be preceded by comment lines.
    use ccsds_ndm::types::UserDefinedEntry;
    let kvn = mutated(
        include_str!("../../data/kvn/acm_g6.kvn"),
        "ATT_STOP",
        "ATT_STOP\nUSER_START\nCOMMENT first\nUSER_DEFINED_A = 1\nCOMMENT second\nUSER_DEFINED_B = 2\nUSER_STOP",
    );
    let message = Acm::from_kvn(&kvn).unwrap();
    let entries = &message.body.segment.data.user.as_ref().unwrap().entries;
    assert!(matches!(&entries[2], UserDefinedEntry::Comment(text) if text == "second"));
    let generated = message.to_kvn().unwrap();
    let compact: String = generated
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" ") + "\n")
        .collect();
    assert!(
        compact.contains("USER_DEFINED_A = 1\nCOMMENT second\n"),
        "{generated}"
    );
    assert_eq!(Acm::from_kvn(&generated).unwrap(), message);
    assert_eq!(Acm::from_xml(&message.to_xml().unwrap()).unwrap(), message);

    let empty = mutated(&kvn, "USER_DEFINED_A = 1", "USER_DEFINED_ = 1");
    let error = Acm::from_kvn(&empty).unwrap_err().to_string();
    assert!(error.contains("parameter name suffix"), "{error}");
}

#[test]
fn ad_cov_type_none_normalizes_to_omission() {
    // ADM table 5-8 lists NONE for AD COV_TYPE; XSD acmCovarianceLineType has no such token.
    let original = Acm::from_kvn(&mutated(
        COV_KVN,
        "COV_TYPE = ANGLE_GYROBIAS\nREF_FRAME_A",
        "REF_FRAME_A",
    ))
    .unwrap();
    for token in ["NONE", "none"] {
        let kvn = mutated(
            COV_KVN,
            "COV_TYPE = ANGLE_GYROBIAS\nREF_FRAME_A",
            &format!("COV_TYPE = {token}\nREF_FRAME_A"),
        );
        assert_eq!(Acm::from_kvn(&kvn).unwrap(), original);
    }
    assert!(Acm::from_kvn(&mutated(
        COV_KVN,
        "COV_TYPE = ANGLE_GYROBIAS\nREF_FRAME_A",
        "COV_TYPE = None\nREF_FRAME_A"
    ))
    .is_err());
    let xml = mutated(
        &original.to_xml().unwrap(),
        "</ATTITUDE_STATES>",
        "</ATTITUDE_STATES><COV_TYPE>NONE</COV_TYPE>",
    );
    assert!(Acm::from_xml(&xml).is_err());
}

#[test]
fn acm_specific_rejections_name_the_keyword() {
    let classification = mutated(
        ATT_KVN,
        "TIME_SYSTEM",
        "CLASSIFICATION = PUBLIC\nTIME_SYSTEM",
    );
    let error = Acm::from_kvn(&classification).unwrap_err().to_string();
    assert!(
        error.contains("CLASSIFICATION is supported only in the ACM header"),
        "{error}"
    );
    let duration = mutated(ATT_KVN, "MAN_DURATION = 450.0", "MAN_DURATION = -1.0");
    let error = Acm::from_kvn(&duration).unwrap_err().to_string();
    assert!(error.contains("MAN_DURATION"), "{error}");
    for (from, to, keyword) in [
        (
            "MAN_DURATION = 450.0",
            "MAN_DURATION = 450.0 [min]",
            "MAN_DURATION",
        ),
        (
            "TARGET_MOMENTUM = 1.30 -16.400 -11.350",
            "TARGET_MOMENTUM = 1.30 -16.400 -11.350 [Nms]",
            "TARGET_MOMENTUM",
        ),
    ] {
        let error = Acm::from_kvn(&mutated(ATT_KVN, from, to))
            .unwrap_err()
            .to_string();
        assert!(error.contains(keyword), "{error}");
    }
}

#[test]
fn blank_lines_are_insignificant_inside_histories_and_between_sensors() {
    for source in [crate::ATT_KVN, crate::COV_KVN] {
        let original = Acm::from_kvn(source).unwrap();
        assert_eq!(
            Acm::from_kvn(&source.replace('\n', "\n\n")).unwrap(),
            original
        );
    }
}

#[test]
fn xml_history_values_refuse_nonzero_underflow_and_keep_subnormals() {
    let mut message = Acm::from_kvn(include_str!("../../data/kvn/acm_g6.kvn")).unwrap();
    for line in &mut message.body.segment.data.att[0].att_lines {
        line.values = vec![0.0, 0.0, 0.0, 1.0];
    }
    let xml = message.to_xml().unwrap();
    let invalid = mutated(&xml, " 0 0 0 1</attLine>", " 1e-400 0 0 1</attLine>");
    let error = Acm::from_xml(&invalid).unwrap_err();
    assert!(error.to_string().contains("underflow"), "{error}");
    let valid = invalid.replace("1e-400", "1e-320");
    let parsed = Acm::from_xml(&valid).unwrap();
    assert!(parsed.body.segment.data.att[0].att_lines[0].values[0] > 0.0);
}
