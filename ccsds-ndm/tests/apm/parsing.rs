use crate::common::{mutated, mutated_once};
use crate::{sample_apm_kvn, KVN, XML};
use ccsds_ndm::error::{CcsdsNdmError, ValidationError};
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::Ndm;
fn sample_apm_header() -> String {
    r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
    .to_string()
}

fn sample_apm_meta() -> String {
    r#"META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
"#
    .to_string()
}

#[test]
fn test_parse_apm_version_error() {
    let input = r#"CCSDS_APM_VERS = 3.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2022-11-04T17:22:31
QUAT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
Q1 = 0.5
Q2 = 0.5
Q3 = 0.5
QC = 0.5
QUAT_STOP
"#;
    let err = Apm::from_kvn(input).unwrap_err();
    match err {
        CcsdsNdmError::UnsupportedInputVersion { version, .. } => {
            assert_eq!(version, "3.0");
        }
        _ => panic!("Expected unsupported input version, got {:?}", err),
    }
}

#[test]
fn test_apm_quaternion_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nQUAT_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nQ1 = 0.0\nQ2 = 0.0\nQ3 = 0.0\nQC = 1.0\nQ1_DOT = 0.01\nQ2_DOT = 0.02\nQ3_DOT = 0.03\nQC_DOT = 0.04\nQUAT_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let q = &apm.body.segment.data.quaternion_state[0];
    assert_eq!(q.quaternion.q1, 0.0);
    assert_eq!(q.quaternion_dot.as_ref().unwrap().q1_dot.value, 0.01);
}

#[test]
fn test_apm_euler_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nEULER_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nEULER_ROT_SEQ = ZYX\nANGLE_1 = 10\nANGLE_2 = 20\nANGLE_3 = 30\nEULER_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let e = &apm.body.segment.data.euler_angle_state[0];
    assert_eq!(e.angle_1.value, 10.0);
    assert_eq!(e.euler_rot_seq, ccsds_ndm::types::RotSeq::ZYX);
}

#[test]
fn test_apm_spin_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nSPIN_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nSPIN_ALPHA = 10\nSPIN_DELTA = 20\nSPIN_ANGLE = 30\nSPIN_ANGLE_VEL = 0.1\nSPIN_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let s = &apm.body.segment.data.spin[0];
    assert_eq!(s.spin_alpha.value, 10.0);
}

#[test]
fn test_apm_inertia_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nINERTIA_START\nINERTIA_REF_FRAME = A\nIXX = 100\nIYY = 200\nIZZ = 300\nIXY = 10\nIXZ = 20\nIYZ = 30\nINERTIA_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let i = &apm.body.segment.data.inertia[0];
    assert_eq!(i.ixx.value, 100.0);
}

#[test]
fn test_apm_man_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nMAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let m = &apm.body.segment.data.maneuver_parameters[0];
    assert_eq!(m.man_duration.value, 10.0);
}

#[test]
fn test_apm_multiple_blocks_mixed() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nQUAT_START\nREF_FRAME_A=A\nREF_FRAME_B=B\nQ1=0\nQ2=0\nQ3=0\nQC=1\nQUAT_STOP\nINERTIA_START\nINERTIA_REF_FRAME=A\nIXX=1\nIYY=2\nIZZ=3\nIXY=0\nIXZ=0\nIYZ=0\nINERTIA_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    assert_eq!(apm.body.segment.data.inertia.len(), 1);
}

#[test]
fn test_parse_apm_angvel_block() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nANGVEL_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nANGVEL_FRAME = B\nANGVEL_X = 1.0\nANGVEL_Y = 2.0\nANGVEL_Z = 3.0\nANGVEL_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let av = &apm.body.segment.data.angular_velocity[0];
    assert_eq!(av.angvel_z.value, 3.0);
    assert_eq!(av.angvel_frame.0, "B");
}

#[test]
fn test_parse_apm_spin_full() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nSPIN_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nSPIN_ALPHA = 10\nSPIN_DELTA = 20\nSPIN_ANGLE = 30\nSPIN_ANGLE_VEL = 0.1\nNUTATION = 5.0\nNUTATION_PER = 100.0\nNUTATION_PHASE = 45.0\nSPIN_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let s = &apm.body.segment.data.spin[0];
    assert_eq!(s.nutation.as_ref().unwrap().value, 5.0);
    assert!(s.momentum_delta.is_none());
}

#[test]
fn test_parse_apm_spin_conflicting_choice() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nSPIN_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nSPIN_ALPHA = 10\nSPIN_DELTA = 20\nSPIN_ANGLE = 30\nSPIN_ANGLE_VEL = 0.1\nNUTATION = 5.0\nNUTATION_PER = 100.0\nNUTATION_PHASE = 45.0\nMOMENTUM_ALPHA = 1.0\nMOMENTUM_DELTA = 2.0\nNUTATION_VEL = 0.05\nSPIN_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let err = Apm::from_kvn(&input).unwrap_err();
    match err {
        CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
            ValidationError::Conflict { fields, .. } => {
                assert!(fields.iter().any(|f| f == "NUTATION"));
                assert!(fields.iter().any(|f| f == "MOMENTUM_ALPHA"));
            }
            _ => panic!("Expected conflict error, got {:?}", boxed_err),
        },
        _ => panic!("Expected Validation error, got {:?}", err),
    }
}

#[test]
fn test_parse_apm_euler_derivatives() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nEULER_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nEULER_ROT_SEQ = ZYX\nANGLE_1 = 10\nANGLE_2 = 20\nANGLE_3 = 30\nANGLE_1_DOT = 0.1\nANGLE_2_DOT = 0.2\nANGLE_3_DOT = 0.3\nEULER_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let e = &apm.body.segment.data.euler_angle_state[0];
    assert_eq!(e.angle_3_dot.as_ref().unwrap().value, 0.3);
}

#[test]
fn test_parse_apm_maneuver_delta_mass() {
    let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nMAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_DELTA_MASS = -1.5\nMAN_STOP\n",
        sample_apm_header(), sample_apm_meta());
    let apm = Apm::from_kvn(&input).unwrap();
    let m = &apm.body.segment.data.maneuver_parameters[0];
    assert_eq!(m.man_delta_mass.as_ref().unwrap().value, -1.5);
}

#[test]
fn apm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = TRMM";
    let object_id = "OBJECT_ID = 1997-074A";
    let q1 = "Q1 = 0.00005";
    for (label, source) in [
        (
            "duplicate top-level keyword",
            mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            mutated(
                KVN,
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "duplicate logical-block keyword",
            mutated(KVN, q1, &format!("{q1}\n{q1}")),
        ),
        (
            "unknown logical-block keyword",
            mutated(KVN, q1, &format!("{q1}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            mutated(KVN, q1, &format!("{q1}\nCOMMENT misplaced")),
        ),
        ("unknown block", mutated(KVN, "QUAT_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            mutated(KVN, "QUAT_STOP", "SPIN_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object_name, &format!("{object_name} €")),
        ),
    ] {
        let error = Apm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn apm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2003-09-30T14:28:15.1172</EPOCH>";
    let q1 = "<Q1>0.00005</Q1>";
    for (label, source) in [
        (
            "unknown quaternion-state child",
            mutated(
                XML,
                "<quaternionState>",
                "<quaternionState><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown quaternion child",
            mutated(XML, "<quaternion>", "<quaternion><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            mutated(
                XML,
                "<quaternionState>",
                "<quaternionState unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            mutated(XML, q1, "<Q1 unexpected=\"value\">0.00005</Q1>"),
        ),
        (
            "duplicate epoch",
            mutated(XML, epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered quaternion components",
            mutated(
                XML,
                "<Q1>0.00005</Q1>\n<Q2>0.87543</Q2>",
                "<Q2>0.87543</Q2>\n<Q1>0.00005</Q1>",
            ),
        ),
    ] {
        let error = Apm::from_xml(&source).unwrap_err();
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
fn parse_apm_success() {
    let kvn = sample_apm_kvn();
    let apm = Apm::from_kvn(&kvn).expect("APM parse failed");

    assert_eq!(apm.version, "2.0");
    assert_eq!(
        apm.body.segment.metadata.object_name,
        "MARS GLOBAL SURVEYOR"
    );
    assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    assert_eq!(apm.body.segment.data.quaternion_state[0].quaternion.q1, 0.5);
}

/// The two rules that reject an otherwise well-formed APM, as mutations of a shipped fixture.
///
/// `apm_g2.kvn` is the baseline because its `OBJECT_NAME` is not preceded by comments:
/// dropping that line from `apm_g1.kvn` leaves a `COMMENT` at the head of the metadata and the
/// message is refused for comment placement instead, which is not the rule under test.
#[test]
fn apm_rejects_missing_object_name_or_all_data_blocks() {
    const FIXTURE: &str = include_str!("../../data/kvn/apm_g2.kvn");
    Apm::from_kvn(FIXTURE).expect("baseline fixture must satisfy both rules");

    let attitude_at = FIXTURE.find("EULER_START").unwrap();
    for (mutated, expected) in [
        (
            FIXTURE[..attitude_at].to_owned(),
            "Missing required field: At least one logical block in block APM Data",
        ),
        (
            mutated(FIXTURE, "OBJECT_NAME = GOES-P\n", ""),
            "Missing required field: OBJECT_NAME in block APM Metadata",
        ),
    ] {
        let error = Apm::from_kvn(&mutated).expect_err("mutation accepted");
        assert!(
            error.to_string().contains(expected),
            "diagnostic did not name {expected}: {error}"
        );
    }
}

#[test]
fn apm_multiple_blocks() {
    let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
EULER_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
EULER_ROT_SEQ = XYZ
ANGLE_1 = 10 [deg]
ANGLE_2 = 20 [deg]
ANGLE_3 = 30 [deg]
EULER_STOP
"#;
    let apm = Apm::from_kvn(kvn).unwrap();
    assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    assert_eq!(apm.body.segment.data.euler_angle_state.len(), 1);
}

#[test]
fn dimensionless_kvn_values_reject_spurious_units() {
    Apm::from_kvn(KVN).expect("baseline fixture must parse");
    let input = mutated_once(KVN, "Q1 = 0.00005", "Q1 = 0.00005 [deg]");
    let error = Apm::from_kvn(&input).expect_err("quaternion components are dimensionless");
    let parse = crate::common::kvn_parse_error(&error).expect("KVN syntax diagnostic");
    assert!(
        parse
            .contexts
            .contains(&"Units are not allowed for this dimensionless or implicitly scaled field"),
        "{error}"
    );
}
