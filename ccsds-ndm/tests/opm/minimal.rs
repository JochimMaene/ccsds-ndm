use crate::common::mutated;
use crate::MINIMAL;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;

#[test]
fn builder_defaults_validate_and_roundtrip() {
    let parsed = Opm::from_kvn(MINIMAL).unwrap();
    let built = Opm::builder()
        .header(parsed.header.clone())
        .body(parsed.body.clone())
        .build();
    assert_eq!(built, parsed);
    ccsds_ndm::Validate::validate(&built).unwrap();
    assert_eq!(Opm::from_kvn(&built.to_kvn().unwrap()).unwrap(), built);
}

#[test]
fn rejects_invalid_units_kvn() {
    let input = mutated(MINIMAL, "X = 1000 [km]", "X = 1000 [m]");
    let error = Opm::from_kvn(&input).unwrap_err();
    assert!(
        matches!(error.as_format_error(), Some(ccsds_ndm::error::FormatError::Enum(cause)) if cause.field == "X" && cause.value == "m"),
        "{error}"
    );
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(MINIMAL, "X = 1000 [km]", "X = BAD [km]");
    let error = Opm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}
