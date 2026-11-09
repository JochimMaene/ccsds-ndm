use crate::common::mutated;
use crate::sample_cdm_kvn;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;

#[test]
fn minimal_kvn_parses() {
    Cdm::from_kvn(sample_cdm_kvn().trim_start()).unwrap();
}

#[test]
fn rejects_missing_tca_kvn() {
    let input = mutated(
        sample_cdm_kvn().trim_start(),
        "TCA = 2025-01-02T12:00:00\n",
        "",
    );
    let error = Cdm::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
    crate::common::assert_validation_field(&error, "TCA");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(sample_cdm_kvn().trim_start(), "MISS_DISTANCE", "BAD_KEY");
    let error = Cdm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(
        sample_cdm_kvn().trim_start(),
        "MISS_DISTANCE = 100.0",
        "MISS_DISTANCE = BAD",
    );
    let error = Cdm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_invalid_tca_kvn() {
    let input = mutated(
        sample_cdm_kvn().trim_start(),
        "TCA = 2025-01-02T12:00:00",
        "TCA = NOT_A_DATE",
    );
    let error = Cdm::from_kvn(&input).unwrap_err();
    // A bad KVN time tag is located like any other bad value.
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
    let kvn = crate::common::kvn_parse_error(&error).expect("a located KVN error");
    assert_eq!((kvn.line, kvn.column), (7, 7), "{error}");
    assert!(error.to_string().contains("NOT_A_DATE"));
}
