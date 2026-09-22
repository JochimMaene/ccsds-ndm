use crate::common::mutated;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;

const TDM_KVN: &str = include_str!("../../data/kvn/tdm_e1.kvn");
const TDM_XML: &str = include_str!("../../data/xml/tdm_e21.xml");

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
        let invalid = mutated(TDM_KVN, needle, replacement);
        crate::common::assert_invalid_epoch(&Tdm::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let invalid = mutated(
        TDM_XML,
        "<EPOCH>2007-069T15:22:22.000</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Tdm::from_xml(&invalid).unwrap_err(), "123.5");
}
