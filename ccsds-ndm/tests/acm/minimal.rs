use crate::common::mutated;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_DESIGNATOR = 2020-001A
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

#[test]
fn minimal_kvn_parses() {
    Acm::from_kvn(KVN).unwrap();
}

#[test]
fn frame_without_optional_cp_is_preserved() {
    let mut input = KVN.to_owned();
    input.push_str("PHYS_START\nCP_REF_FRAME = SC_BODY\nPHYS_STOP\n");
    let message = Acm::from_kvn(&input).unwrap();
    assert_eq!(Acm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
}

#[test]
fn rejects_spurious_metadata_kvn() {
    let input = mutated(KVN, "META_STOP", "BAD_KEY = 1\nMETA_STOP");
    let error = Acm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "0.0 0 0 0 1", "NOT_A_DATE 0 0 0 1");
    let error = Acm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_duplicate_sensor_kvn() {
    let mut input = KVN.to_owned();
    input.push_str("AD_START\nATTITUDE_STATES = QUATERNION\nREF_FRAME_A = GCRF\nREF_FRAME_B = SC_BODY\nSENSOR_START\nSENSOR_NUMBER = 1\nSENSOR_USED = STAR_TRACKER\nSENSOR_STOP\nSENSOR_START\nSENSOR_NUMBER = 1\nSENSOR_USED = GYRO\nSENSOR_STOP\nAD_STOP\n");
    Acm::from_kvn(&input.replace(
        "SENSOR_NUMBER = 1\nSENSOR_USED = GYRO",
        "SENSOR_NUMBER = 2\nSENSOR_USED = GYRO",
    ))
    .expect("valid control");
    let error = Acm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"), "{error}");
    crate::common::assert_validation_field(&error, "SENSOR_NUMBER");
}

#[test]
fn rejects_conflict_kvn() {
    let mut input = KVN.to_owned();
    input.push_str("MAN_START\nMAN_PURPOSE = SCI_OBJ\nMAN_BEGIN_TIME = 0\nMAN_END_TIME = 10\nMAN_DURATION = 10\nMAN_STOP\n");
    Acm::from_kvn(&mutated(&input, "MAN_DURATION = 10\n", "")).expect("valid control");
    let error = Acm::from_kvn(&input).unwrap_err();
    assert!(
        matches!(error.as_validation_error(), Some(ccsds_ndm::error::ValidationError::Conflict { fields, .. }) if fields == &["MAN_END_TIME", "MAN_DURATION"]),
        "{error}"
    );
}
