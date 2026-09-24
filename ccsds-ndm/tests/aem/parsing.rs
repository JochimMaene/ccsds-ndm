use ccsds_ndm::common::AemAttitudeState;
use ccsds_ndm::error::{CcsdsNdmError, FormatError, ValidationError};
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::types::AttitudeTypeType;
use ccsds_ndm::Ndm;

use crate::common::{mutated, mutated_once};
use crate::{KVN, SPIN_KVN, XML};

#[test]
fn aem_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object = "OBJECT_NAME = MARS GLOBAL SURVEYOR";
    let object_id = "OBJECT_ID = 1996-062A";
    let first_state = "1996-11-28T21:29:07.2555 0.56748 0.03146 0.45689 0.68427";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            mutated(KVN, object, &format!("{object}\n{object}")),
        ),
        (
            "reordered metadata",
            mutated(
                KVN,
                &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
        (
            "unknown metadata",
            mutated(KVN, object, &format!("{object}\nUNKNOWN = value")),
        ),
        (
            "comment after history begins",
            mutated(
                KVN,
                first_state,
                &format!("{first_state}\nCOMMENT misplaced"),
            ),
        ),
        ("mismatched block", mutated(KVN, "META_STOP", "DATA_STOP")),
        ("unknown block", mutated(KVN, "DATA_START", "UNKNOWN_START")),
        (
            "XML-only metadata keyword in KVN",
            mutated(
                KVN,
                "ATTITUDE_TYPE = QUATERNION",
                "ATTITUDE_TYPE = QUATERNION\nANGVEL_FRAME = A",
            ),
        ),
        (
            "zero interpolation degree",
            mutated(
                SPIN_KVN,
                "ATTITUDE_TYPE = SPIN",
                "ATTITUDE_TYPE = SPIN\nINTERPOLATION_DEGREE = 0",
            ),
        ),
        (
            "missing interpolation degree",
            mutated(KVN, "INTERPOLATION_DEGREE = 7\n", ""),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
    ] {
        let error = Aem::from_kvn(&source).unwrap_err();
        match label {
            "missing interpolation degree" => crate::common::assert_validation_field(
                &error,
                "INTERPOLATION_DEGREE (required when INTERPOLATION_METHOD is present)",
            ),
            _ => assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}"),
        }
    }
}

#[test]
fn aem_kvn_rejects_non_ccsds_history_number_spellings() {
    for value in [".5", "1e0", "12.5e3"] {
        let source = mutated_once(SPIN_KVN, "2.6862511e+002", value);
        let error = Aem::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{value}: {error}");
    }
    // Reading does not apply the book's 16-digit cap: 17-digit doubles are common and lossless.
    let source = mutated_once(SPIN_KVN, "2.6862511e+002", "268.62511000000001");
    Aem::from_kvn(&source).unwrap();
}

#[test]
fn aem_xml_rejects_unknown_choice_content_attributes_and_ordering_errors() {
    let object = "<OBJECT_NAME>TEST</OBJECT_NAME>";
    let object_id = "<OBJECT_ID>2000-999Z</OBJECT_ID>";
    let epoch = "<EPOCH>2000-100T00:00:00.000</EPOCH>";
    for (label, source) in [
        (
            "unknown attitude state choice",
            mutated(XML, "<attitudeState>", "<attitudeState><UNKNOWN/>"),
        ),
        (
            "two attitude state choices",
            mutated(XML, "</quaternionEphemeris>",
                "</quaternionEphemeris><spin><EPOCH>2000-100T00:00:00.000</EPOCH><SPIN_ALPHA>1</SPIN_ALPHA><SPIN_DELTA>2</SPIN_DELTA><SPIN_ANGLE>3</SPIN_ANGLE><SPIN_ANGLE_VEL>4</SPIN_ANGLE_VEL></spin>",
            ),
        ),
        (
            "unknown container attribute",
            mutated(XML, "<attitudeState>", "<attitudeState unexpected=\"value\">"),
        ),
        (
            "illegal quaternion units",
            mutated(XML, "<Q1>-0.005068</Q1>", "<Q1 units=\"1\">-0.005068</Q1>"),
        ),
        (
            "missing Euler rotation sequence",
            mutated(XML, "<EULER_ROT_SEQ>XYZ</EULER_ROT_SEQ>", ""),
        ),
        (
            "missing angular velocity frame",
            mutated(XML, "<ANGVEL_FRAME>REF_FRAME_B</ANGVEL_FRAME>", ""),
        ),
        (
            "duplicate epoch",
            mutated(XML, epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered metadata",
            mutated(XML, &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
    ] {
        let error = Aem::from_xml(&source).unwrap_err();
        match label {
            "two attitude state choices" => {
                assert_eq!(error.code(), Some("parse.xml.syntax"), "{error}");
                assert!(error.to_string().contains("attitudeState requires exactly one attitude choice"), "{error}");
            }
            "missing Euler rotation sequence" => crate::common::assert_validation_field(&error, "EULER_ROT_SEQ (required for EULER_ANGLE types)"),
            "missing angular velocity frame" => crate::common::assert_validation_field(&error, "ANGVEL_FRAME (required for ANGVEL types)"),
            _ => assert!(matches!(error.as_format_error(), Some(FormatError::InvalidFormat(_))), "{label}: {error}"),
        }
    }
}

#[test]
fn aem_kvn_accepts_carriage_return_only_line_endings() {
    let cr_only = mutated(&KVN.replace("\r\n", "\n"), "\n", "\r");
    assert!(!cr_only.contains('\n'), "fixture still holds line feeds");

    let from_cr = Aem::from_kvn(&cr_only).unwrap();
    let from_lf = Aem::from_kvn(KVN).unwrap();
    assert_eq!(from_cr, from_lf);
}

fn sample_aem_header() -> String {
    r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
    .to_string()
}

fn sample_aem_meta() -> String {
    r#"META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2002-11-04T17:22:31
STOP_TIME = 2002-11-04T17:25:31
ATTITUDE_TYPE = QUATERNION
META_STOP
"#
    .to_string()
}

#[test]
fn test_parse_aem_minimal() {
    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\nDATA_STOP\n",
        sample_aem_header(),
        sample_aem_meta()
    );
    let aem = Aem::from_kvn(&input).unwrap();
    assert_eq!(aem.version, "2.0");
    assert_eq!(aem.header.originator, "NASA/JPL");
    assert_eq!(aem.body.segment.len(), 1);
}

#[test]
fn test_parse_aem_version_error() {
    let input = r#"CCSDS_AEM_VERS = 3.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2002-11-04T17:22:31
STOP_TIME = 2002-11-04T17:25:31
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2002-11-04T17:22:31 0.5 0.5 0.5 0.5
DATA_STOP
"#;
    let err = Aem::from_kvn(input).unwrap_err();
    match err {
        CcsdsNdmError::UnsupportedInputVersion { version, .. } => {
            assert_eq!(version, "3.0");
        }
        _ => panic!("Expected unsupported input version, got {:?}", err),
    }
}

#[test]
fn test_aem_missing_mandatory_metadata() {
    // Missing OBJECT_NAME
    let input = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_ID = SAT1
CENTER_NAME = EARTH
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
    let err = Aem::from_kvn(input).unwrap_err();
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

fn parse_attitude_state(attitude_type: &str, values: &str) -> AemAttitudeState {
    let mut meta = sample_aem_meta().replace("QUATERNION", attitude_type);
    let mut extra = String::new();
    if attitude_type.starts_with("EULER") {
        extra.push_str("EULER_ROT_SEQ = ZYX\n");
    }
    if attitude_type.contains("ANGVEL") {
        extra.push_str("RATE_FRAME = SC_BODY_1\n");
    }
    if !extra.is_empty() {
        meta = meta.replace("META_STOP", &format!("{extra}META_STOP"));
    }
    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 {values}\nDATA_STOP\n",
        sample_aem_header(),
        meta
    );
    Aem::from_kvn(&input).unwrap().body.segment[0]
        .data
        .attitude_states[0]
        .clone()
}

// ADM table 4-4: compare every named component in its KVN column order.
fn parsed_type_and_values(state: &AemAttitudeState) -> (AttitudeTypeType, Vec<f64>) {
    match state {
        AemAttitudeState::QuaternionEphemeris(value) => (
            AttitudeTypeType::Quaternion,
            vec![
                value.quaternion.q1,
                value.quaternion.q2,
                value.quaternion.q3,
                value.quaternion.qc,
            ],
        ),
        AemAttitudeState::QuaternionDerivative(value) => (
            AttitudeTypeType::QuaternionDerivative,
            vec![
                value.quaternion.q1,
                value.quaternion.q2,
                value.quaternion.q3,
                value.quaternion.qc,
                value.quaternion_dot.q1_dot.value,
                value.quaternion_dot.q2_dot.value,
                value.quaternion_dot.q3_dot.value,
                value.quaternion_dot.qc_dot.value,
            ],
        ),
        AemAttitudeState::QuaternionAngVel(value) => (
            AttitudeTypeType::QuaternionAngVel,
            vec![
                value.quaternion.q1,
                value.quaternion.q2,
                value.quaternion.q3,
                value.quaternion.qc,
                value.ang_vel.angvel_x.value,
                value.ang_vel.angvel_y.value,
                value.ang_vel.angvel_z.value,
            ],
        ),
        AemAttitudeState::EulerAngle(value) => (
            AttitudeTypeType::EulerAngle,
            vec![
                value.angle_1.value,
                value.angle_2.value,
                value.angle_3.value,
            ],
        ),
        AemAttitudeState::EulerAngleDerivative(value) => (
            AttitudeTypeType::EulerAngleDerivative,
            vec![
                value.angle_1.value,
                value.angle_2.value,
                value.angle_3.value,
                value.angle_1_dot.value,
                value.angle_2_dot.value,
                value.angle_3_dot.value,
            ],
        ),
        AemAttitudeState::EulerAngleAngVel(value) => (
            AttitudeTypeType::EulerAngleAngVel,
            vec![
                value.angle_1.value,
                value.angle_2.value,
                value.angle_3.value,
                value.angvel_x.value,
                value.angvel_y.value,
                value.angvel_z.value,
            ],
        ),
        AemAttitudeState::Spin(value) => (
            AttitudeTypeType::Spin,
            vec![
                value.spin_alpha.value,
                value.spin_delta.value,
                value.spin_angle.value,
                value.spin_angle_vel.value,
            ],
        ),
        AemAttitudeState::SpinNutation(value) => (
            AttitudeTypeType::SpinNutation,
            vec![
                value.spin_alpha.value,
                value.spin_delta.value,
                value.spin_angle.value,
                value.spin_angle_vel.value,
                value.nutation.value,
                value.nutation_per.value,
                value.nutation_phase.value,
            ],
        ),
        AemAttitudeState::SpinNutationMom(value) => (
            AttitudeTypeType::SpinNutationMom,
            vec![
                value.spin_alpha.value,
                value.spin_delta.value,
                value.spin_angle.value,
                value.spin_angle_vel.value,
                value.momentum_alpha.value,
                value.momentum_delta.value,
                value.nutation_vel.value,
            ],
        ),
    }
}

#[test]
fn test_parse_aem_attitude_types() {
    let cases = [
        (
            "QUATERNION",
            "0.0 0.36 0.48 0.8",
            AttitudeTypeType::Quaternion,
        ),
        (
            "QUATERNION/DERIVATIVE",
            "0.0 0.36 0.48 0.8 0.5 0.6 0.7 0.9",
            AttitudeTypeType::QuaternionDerivative,
        ),
        (
            "QUATERNION/ANGVEL",
            "0.0 0.36 0.48 0.8 0.01 0.02 0.03",
            AttitudeTypeType::QuaternionAngVel,
        ),
        (
            "EULER_ANGLE",
            "10.0 20.0 30.0",
            AttitudeTypeType::EulerAngle,
        ),
        (
            "EULER_ANGLE/DERIVATIVE",
            "10.0 20.0 30.0 0.1 0.2 0.3",
            AttitudeTypeType::EulerAngleDerivative,
        ),
        (
            "EULER_ANGLE/ANGVEL",
            "10.0 20.0 30.0 0.1 0.2 0.3",
            AttitudeTypeType::EulerAngleAngVel,
        ),
        ("SPIN", "10.0 20.0 30.0 0.1", AttitudeTypeType::Spin),
        (
            "SPIN/NUTATION",
            "10.0 20.0 30.0 0.1 5.0 100.0 45.0",
            AttitudeTypeType::SpinNutation,
        ),
        (
            "SPIN/NUTATION_MOM",
            "10.0 20.0 30.0 0.1 5.0 6.0 0.05",
            AttitudeTypeType::SpinNutationMom,
        ),
    ];

    for (attitude_type, values, expected_type) in cases {
        let state = parse_attitude_state(attitude_type, values);
        let (actual_type, actual_values) = parsed_type_and_values(&state);
        let expected_values: Vec<f64> = values
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        assert_eq!(actual_type, expected_type);
        assert_eq!(
            actual_values, expected_values,
            "{attitude_type} column order"
        );
    }
}

#[test]
fn test_parse_aem_quaternion_angvel_requires_angvel_frame() {
    let qr_meta = sample_aem_meta().replace("QUATERNION", "QUATERNION/ANGVEL");
    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.1 0.2 0.3 0.4 0.01 0.02 0.03\nDATA_STOP\n",
        sample_aem_header(),
        qr_meta
    );
    let err = Aem::from_kvn(&input).unwrap_err();
    match err {
        CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
            ValidationError::MissingRequiredField { field, .. } => {
                assert_eq!(field, "ANGVEL_FRAME (required for ANGVEL types)");
            }
            _ => panic!("Expected missing field error, got {:?}", boxed_err),
        },
        _ => panic!("Expected Validation error, got {:?}", err),
    }
}

#[test]
fn test_parse_aem_invalid_lines() {
    let cases = [
        (
            "wrong quaternion columns",
            sample_aem_meta(),
            "2002-11-04T17:22:31 0.1 0.2 0.3",
        ),
        (
            "out-of-range normalized quaternion",
            sample_aem_meta(),
            "2002-11-04T17:22:31 1.0001 0 0 0",
        ),
        (
            "wrong derivative columns",
            sample_aem_meta().replace("QUATERNION", "QUATERNION/DERIVATIVE"),
            "2002-11-04T17:22:31 0.1 0.2 0.3 0.4",
        ),
        (
            "wrong Euler columns",
            sample_aem_meta()
                .replace("QUATERNION", "EULER_ANGLE")
                .replace("META_STOP", "EULER_ROT_SEQ = ZYX\nMETA_STOP"),
            "2002-11-04T17:22:31 10.0 20.0",
        ),
        (
            "invalid epoch",
            sample_aem_meta(),
            "NOT_A_DATE 0.1 0.2 0.3 0.4",
        ),
        (
            "invalid float",
            sample_aem_meta(),
            "2002-11-04T17:22:31 0.1 NOT_A_NUMBER 0.3 0.4",
        ),
    ];

    for (label, metadata, values) in cases {
        let input = format!(
            "{}{}\nDATA_START\n{values}\nDATA_STOP\n",
            sample_aem_header(),
            metadata
        );
        let error = Aem::from_kvn(&input).unwrap_err();
        if label == "out-of-range normalized quaternion" {
            crate::common::assert_validation_field(&error, "Q1");
        } else if label == "invalid epoch" {
            crate::common::assert_invalid_epoch(&error, "NOT_A_DATE");
        } else {
            assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
        }
    }

    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.1 0.2 0.3 0.4\n",
        sample_aem_header(),
        sample_aem_meta()
    );
    let error = Aem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn test_aem_value_error_reports_record_line() {
    let bad_record = "2002-11-04T17:23:00 1.5 0 0 0";
    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\n{bad_record}\nDATA_STOP\n",
        sample_aem_header(),
        sample_aem_meta()
    );
    let expected_line = input[..input.find(bad_record).unwrap()]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;

    let CcsdsNdmError::Validation(error) = Aem::from_kvn(&input).unwrap_err() else {
        panic!("expected validation error");
    };
    let ValidationError::OutOfRange { name, line, .. } = *error else {
        panic!("expected quaternion component range error");
    };
    assert_eq!(name, "Q1");
    assert_eq!(line, Some(expected_line));
}

#[test]
fn test_parse_aem_multiple_segments() {
    let seg1 = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\nDATA_STOP\n",
        "",
        sample_aem_meta()
    );
    let seg2 = format!(
        "{}\nDATA_START\n2002-11-04T17:23:00 0.5 0.5 0.5 0.5\nDATA_STOP\n",
        sample_aem_meta()
    ); // Re-use meta for simplicity

    let input = format!("{}{}{}", sample_aem_header(), seg1, seg2);
    let aem = Aem::from_kvn(&input).unwrap();
    assert_eq!(aem.body.segment.len(), 2);
}

#[test]
fn test_parse_aem_comments() {
    let input = r#"CCSDS_AEM_VERS = 2.0
COMMENT Header comment
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
COMMENT Meta comment
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP

DATA_START
COMMENT Data comment
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
    let aem = Aem::from_kvn(input).unwrap();
    assert!(aem.header.comment.contains(&"Header comment".to_string()));
    assert!(aem.body.segment[0]
        .metadata
        .comment
        .contains(&"Meta comment".to_string()));
    assert!(aem.body.segment[0]
        .data
        .comment
        .contains(&"Data comment".to_string()));
}
#[test]
fn test_parse_aem_rate_frame_alias() {
    let meta = sample_aem_meta()
        .replace("QUATERNION", "QUATERNION/ANGVEL")
        .replace("META_STOP", "RATE_FRAME = SC_BODY_1\nMETA_STOP");

    // QUATERNION/ANGVEL needs 7 columns
    let input = format!(
        "{}{}\nDATA_START\n2002-11-04T17:22:31 0.0 0.0 0.0 1.0 0.01 0.02 0.03\nDATA_STOP\n",
        sample_aem_header(),
        meta
    );

    let aem = Aem::from_kvn(&input).unwrap();
    // Check that proper parsing happened (RATE_FRAME maps to angvel_frame)
    assert_eq!(
        aem.body.segment[0].metadata.angvel_frame.as_deref(),
        Some("SC_BODY_1")
    );
}

#[test]
fn test_aem_no_segments() {
    // Valid header but no body segments
    let input = sample_aem_header().to_string();
    let err = Aem::from_kvn(&input).unwrap_err();
    match err {
        CcsdsNdmError::Format(boxed_err) => match *boxed_err {
            FormatError::Kvn(e) => {
                assert!(format!("{:?}", e).contains("At least one segment required"));
            }
            _ => panic!("Expected Kvn format error, got {:?}", boxed_err),
        },
        _ => panic!("Expected Format error, got {:?}", err),
    }
}
