use crate::common::mutated;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0
ECCENTRICITY = 0.001
INCLINATION = 98.0
RA_OF_ASC_NODE = 10.0
ARG_OF_PERICENTER = 20.0
MEAN_ANOMALY = 30.0
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;

#[test]
fn minimal_kvn_parses() {
    Omm::from_kvn(KVN).unwrap();
}

#[test]
fn rejects_missing_object_id_kvn() {
    let input = mutated(KVN, "OBJECT_ID = 2020-001A\n", "");
    let error = Omm::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
    crate::common::assert_validation_field(&error, "OBJECT_ID");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "MEAN_ELEMENT_THEORY", "BAD_KEY");
    let error = Omm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "MEAN_MOTION = 15.0", "MEAN_MOTION = BAD");
    let error = Omm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}
