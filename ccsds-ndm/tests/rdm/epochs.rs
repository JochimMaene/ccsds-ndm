use crate::common::mutated;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::Ndm;

const RDM_KVN: &str = include_str!("../../data/kvn/rdm_c2.kvn");
const RDM_XML: &str = include_str!("../../data/xml/rdm_c4.xml");

#[test]
fn rdm_epoch_fields_require_calendar_or_ordinal_form() {
    Rdm::from_kvn(RDM_KVN).expect("RDM fixture should parse");

    for (needle, replacement) in [
        (
            "CREATION_DATE = 2018-04-22T09:31:34.00",
            "CREATION_DATE = 123.5",
        ),
        (
            "EPOCH_TZERO = 2018-04-22T09:00:00.00",
            "EPOCH_TZERO = 123.5",
        ),
        (
            "NOMINAL_REENTRY_EPOCH = 2018-04-27T19:45:33",
            "NOMINAL_REENTRY_EPOCH = 123.5",
        ),
    ] {
        let invalid = mutated(RDM_KVN, needle, replacement);
        crate::common::assert_invalid_epoch(&Rdm::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let no_next = mutated(
        RDM_KVN,
        "NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00",
        "NEXT_MESSAGE_EPOCH = N/A",
    );
    let parsed = Rdm::from_kvn(&no_next).expect("RDM N/A next epoch should be nullable");
    assert!(parsed.body.segment.metadata.next_message_epoch.is_none());

    let invalid = mutated(
        RDM_XML,
        "<EPOCH_TZERO>2018-04-22T09:00:00.00</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    crate::common::assert_invalid_epoch(&Rdm::from_xml(&invalid).unwrap_err(), "123.5");
}
