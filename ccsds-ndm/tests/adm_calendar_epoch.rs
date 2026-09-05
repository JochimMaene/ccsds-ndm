use ccsds_ndm::messages::{aem::Aem, apm::Apm};
use ccsds_ndm::Ndm;

const APM_KVN: &str = include_str!("../data/kvn/apm_g1.kvn");
const AEM_KVN: &str = include_str!("../data/kvn/aem_g4.kvn");
const APM_XML: &str = include_str!("../data/xml/apm_g10.xml");
const AEM_XML: &str = include_str!("../data/xml/aem_g11.xml");

#[test]
fn apm_epoch_fields_require_calendar_or_ordinal_form() {
    let valid = Apm::from_kvn(APM_KVN).expect("APM fixture should parse");
    assert_eq!(
        valid.body.segment.data.epoch.as_str(),
        "2003-09-30T14:28:15.1172"
    );

    let invalid = APM_KVN.replace("EPOCH = 2003-09-30T14:28:15.1172", "EPOCH = 123.5");
    assert!(
        Apm::from_kvn(&invalid).is_err(),
        "accepted numeric APM epoch"
    );

    let with_maneuver = APM_KVN.replace(
        "QUAT_START",
        "MAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_STOP\nQUAT_START",
    );
    let invalid = with_maneuver.replace(
        "MAN_EPOCH_START = 2023-01-01T01:00:00",
        "MAN_EPOCH_START = 123.5",
    );
    assert!(
        Apm::from_kvn(&invalid).is_err(),
        "accepted numeric maneuver epoch"
    );

    let invalid_xml = APM_XML.replace(
        "<EPOCH>2003-09-30T14:28:15.1172</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    assert!(Apm::from_xml(&invalid_xml).is_err());
}

#[test]
fn aem_epoch_fields_require_calendar_or_ordinal_form() {
    let valid = Aem::from_kvn(AEM_KVN).expect("AEM fixture should parse");
    assert_eq!(
        valid.body.segment[0].metadata.start_time.as_str(),
        "1996-11-28T21:29:07.2555"
    );

    for (needle, replacement) in [
        (
            "START_TIME = 1996-11-28T21:29:07.2555",
            "START_TIME = 123.5",
        ),
        ("1996-11-28T21:29:07.2555 0.56748", "123.5 0.56748"),
    ] {
        let invalid = AEM_KVN.replace(needle, replacement);
        assert!(
            Aem::from_kvn(&invalid).is_err(),
            "accepted numeric AEM epoch"
        );
    }

    let invalid_metadata = AEM_XML.replace(
        "<START_TIME>2006-090T05:00:00.071</START_TIME>",
        "<START_TIME>123.5</START_TIME>",
    );
    assert!(Aem::from_xml(&invalid_metadata).is_err());

    let invalid_state = AEM_XML.replace(
        "<EPOCH>2006-090T05:00:00.071</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    assert!(Aem::from_xml(&invalid_state).is_err());
}
