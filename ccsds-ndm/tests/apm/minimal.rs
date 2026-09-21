use crate::common::mutated;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = SC_BODY_1
REF_FRAME_B = GCRF
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
"#;

#[test]
fn minimal_kvn_parses() {
    Apm::from_kvn(KVN).unwrap();
}

#[test]
fn rejects_missing_frame_kvn() {
    let input = mutated(KVN, "REF_FRAME_A = SC_BODY_1\n", "");
    let error = Apm::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
    crate::common::assert_validation_field(&error, "REF_FRAME_A");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "META_STOP", "BAD_KEY = 1\nMETA_STOP");
    let error = Apm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "Q1 = 0", "Q1 = BAD");
    let error = Apm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_invalid_euler_seq_kvn() {
    let mut input = KVN.to_owned();
    input.push_str(
        "EULER_START\nREF_FRAME_A = SC_BODY_1\nREF_FRAME_B = GCRF\nEULER_ROT_SEQ = BAD\nANGLE_1 = 0\nANGLE_2 = 0\nANGLE_3 = 0\nEULER_STOP\n",
    );
    let error = Apm::from_kvn(&input).unwrap_err();
    assert!(
        matches!(error.as_format_error(), Some(ccsds_ndm::error::FormatError::Enum(cause)) if cause.field == "EULER_ROT_SEQ" && cause.value == "BAD"),
        "{error}"
    );
}
