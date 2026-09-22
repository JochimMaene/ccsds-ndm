use crate::common::mutated;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::Ndm;

const APM_KVN: &str = include_str!("../../data/kvn/apm_g1.kvn");
const APM_XML: &str = include_str!("../../data/xml/apm_g10.xml");

#[test]
fn apm_epoch_fields_require_calendar_or_ordinal_form() {
    let valid = Apm::from_kvn(APM_KVN).expect("APM fixture should parse");
    assert_eq!(
        valid.body.segment.data.epoch.as_str(),
        "2003-09-30T14:28:15.1172"
    );

    let invalid = mutated(APM_KVN, "EPOCH = 2003-09-30T14:28:15.1172", "EPOCH = 123.5");
    crate::common::assert_invalid_epoch(&Apm::from_kvn(&invalid).unwrap_err(), "123.5");

    let with_maneuver = mutated(APM_KVN, "QUAT_START",
        "MAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_STOP\nQUAT_START",
    );
    Apm::from_kvn(&with_maneuver).expect("valid maneuver control");
    let invalid = mutated(
        &with_maneuver,
        "MAN_EPOCH_START = 2023-01-01T01:00:00",
        "MAN_EPOCH_START = 123.5",
    );
    crate::common::assert_invalid_epoch(&Apm::from_kvn(&invalid).unwrap_err(), "123.5");

    let invalid_xml = mutated(
        APM_XML,
        "<EPOCH>2003-09-30T14:28:15.1172</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Apm::from_xml(&invalid_xml).unwrap_err(), "123.5");
}
