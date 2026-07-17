use ccsds_ndm::messages::{cdm::Cdm, rdm::Rdm, tdm::Tdm};
use ccsds_ndm::traits::Ndm;

const CDM_KVN: &str = include_str!("../../data/kvn/cdm_362.kvn");
const CDM_XML: &str = include_str!("../../data/xml/cdm_44.xml");
const TDM_KVN: &str = include_str!("../../data/kvn/tdm_e1.kvn");
const TDM_XML: &str = include_str!("../../data/xml/tdm_e21.xml");
const RDM_KVN: &str = include_str!("../../data/kvn/rdm_c2.kvn");
const RDM_XML: &str = include_str!("../../data/xml/rdm_c4.xml");

#[test]
fn cdm_epoch_fields_require_calendar_or_ordinal_form() {
    Cdm::from_kvn(CDM_KVN).expect("CDM fixture should parse");

    for (needle, replacement) in [
        (
            "CREATION_DATE = 2010-03-12T22:31:12.000",
            "CREATION_DATE = 123.5",
        ),
        ("TCA = 2010-03-13T22:37:52.618", "TCA = 123.5"),
    ] {
        let invalid = CDM_KVN.replace(needle, replacement);
        assert!(
            Cdm::from_kvn(&invalid).is_err(),
            "accepted numeric CDM epoch"
        );
    }

    let invalid = CDM_XML.replace(
        "<CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>",
        "<CREATION_DATE>123.5</CREATION_DATE>",
    );
    assert!(Cdm::from_xml(&invalid).is_err());
}

#[test]
fn tdm_epoch_fields_require_calendar_or_ordinal_form() {
    Tdm::from_kvn(TDM_KVN).expect("TDM fixture should parse");

    for (needle, replacement) in [
        (
            "CREATION_DATE = 2005-160T20:15:00Z",
            "CREATION_DATE = 123.5",
        ),
        ("2005-159T17:41:00 32023442781.733", "123.5 32023442781.733"),
    ] {
        let invalid = TDM_KVN.replace(needle, replacement);
        assert!(
            Tdm::from_kvn(&invalid).is_err(),
            "accepted numeric TDM epoch"
        );
    }

    let invalid = TDM_XML.replace(
        "<EPOCH>2007-069T15:22:22.000</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    assert!(Tdm::from_xml(&invalid).is_err());
}

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
        let invalid = RDM_KVN.replace(needle, replacement);
        assert!(
            Rdm::from_kvn(&invalid).is_err(),
            "accepted numeric RDM epoch"
        );
    }

    let no_next = RDM_KVN.replace(
        "NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00",
        "NEXT_MESSAGE_EPOCH = N/A",
    );
    let parsed = Rdm::from_kvn(&no_next).expect("RDM N/A next epoch should be nullable");
    assert!(parsed.body.segment.metadata.next_message_epoch.is_none());

    let invalid = RDM_XML.replace(
        "<EPOCH_TZERO>2018-04-22T09:00:00.00</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    assert!(Rdm::from_xml(&invalid).is_err());
}
