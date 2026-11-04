use crate::common::mutated;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;

#[test]
fn minimal_kvn_parses() {
    Ocm::from_kvn(KVN).unwrap();
}

#[test]
fn rejects_missing_tzero_kvn() {
    let input = mutated(KVN, "EPOCH_TZERO = 2023-01-01T00:00:00\n", "");
    let error = Ocm::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{error}"
    );
    crate::common::assert_validation_field(&error, "EPOCH_TZERO");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "TIME_SYSTEM", "BAD_KEY");
    let error = Ocm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn trajectory_basis_is_open_book_text_but_schema_enum_in_xml() {
    let input = mutated(KVN, "TRAJ_START\n", "TRAJ_START\nTRAJ_BASIS = INVALID\n");
    Ocm::from_kvn(&mutated(
        &input,
        "TRAJ_BASIS = INVALID",
        "TRAJ_BASIS = PREDICTED",
    ))
    .expect("valid control");
    let message = Ocm::from_kvn(&input).unwrap();
    assert_eq!(
        message.body.segment.data.traj[0].traj_basis.as_deref(),
        Some("INVALID")
    );
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    assert!(message.to_xml().is_err());
}

#[test]
fn rejects_invalid_units_kvn() {
    let input = mutated(
        KVN,
        "TRAJ_TYPE = CARTPV",
        "TRAJ_TYPE = CARTPV\nTRAJ_UNITS = [m, m, m, m/s, m/s, m/s]",
    );
    let error = Ocm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"), "{error}");
    crate::common::assert_validation_field(&error, "TRAJ_UNITS");
}
