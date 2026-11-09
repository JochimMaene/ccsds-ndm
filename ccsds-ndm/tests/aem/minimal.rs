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
fn rejects_missing_required_metadata_kvn() {
    Aem::from_kvn(KVN).unwrap();
    for (record, field) in [
        ("ATTITUDE_TYPE = QUATERNION\n", "ATTITUDE_TYPE"),
        ("OBJECT_NAME = TEST\n", "OBJECT_NAME"),
    ] {
        let input = mutated(KVN, record, "");
        let error = Aem::from_kvn(&input).unwrap_err();
        assert_eq!(
            error.code(),
            Some("validation.missing_required_field"),
            "{error}"
        );
        crate::common::assert_validation_field(&error, field);
    }
}
