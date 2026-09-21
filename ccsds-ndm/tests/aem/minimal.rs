use crate::common::mutated;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T00:10:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;

#[test]
fn minimal_kvn_parses() {
    Aem::from_kvn(KVN).unwrap();
}

#[test]
fn rejects_missing_attitude_type_kvn() {
    let input = mutated(KVN, "ATTITUDE_TYPE = QUATERNION\n", "");
    let error = Aem::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
    crate::common::assert_validation_field(&error, "ATTITUDE_TYPE");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "META_STOP", "BAD_KEY = 1\nMETA_STOP");
    let error = Aem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "0 0 0 1", "0 0 BAD 1");
    let error = Aem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_inconsistent_data_kvn() {
    let input = mutated(KVN, "0 0 0 1", "0 0 0");
    let error = Aem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}
