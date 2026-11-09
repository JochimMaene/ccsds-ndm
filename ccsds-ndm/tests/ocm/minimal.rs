use crate::common::mutated;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::Ndm;

pub(super) const HEADER: &str = "CCSDS_OCM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nMETA_START\nTIME_SYSTEM = UTC\nEPOCH_TZERO = 2023-01-01T00:00:00\nMETA_STOP\n";

pub(super) const KVN: &str = r#"CCSDS_OCM_VERS = 3.0
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
    let message = Ocm::from_kvn(KVN).unwrap();
    assert_eq!(message.body.segment.metadata.time_system, "UTC");
    assert!(message.body.segment.metadata.sclk_offset_at_epoch.is_none());
    assert!(message.body.segment.metadata.sclk_sec_per_si_sec.is_none());
    assert_eq!(message.body.segment.data.traj.len(), 1);
    assert_eq!(
        message.body.segment.data.traj[0].traj_lines[0].values.len(),
        6
    );
    assert_eq!(Ocm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
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
