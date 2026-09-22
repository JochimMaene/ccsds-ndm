use crate::common::mutated;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1 [km]
Y = 2 [km]
Z = 3 [km]
X_DOT = 4 [km/s]
Y_DOT = 5 [km/s]
Z_DOT = 6 [km/s]
"#;

#[test]
fn minimal_kvn_parses() {
    Opm::from_kvn(KVN).unwrap();
}

#[test]
fn rejects_invalid_units_kvn() {
    let input = mutated(KVN, "X = 1 [km]", "X = 1 [m]");
    let error = Opm::from_kvn(&input).unwrap_err();
    assert!(
        matches!(error.as_format_error(), Some(ccsds_ndm::error::FormatError::Enum(cause)) if cause.field == "unit" && cause.value == "m"),
        "{error}"
    );
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "REF_FRAME", "BAD_KEY");
    let error = Opm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "X = 1 [km]", "X = BAD [km]");
    let error = Opm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}
