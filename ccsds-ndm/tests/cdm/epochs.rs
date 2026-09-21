use crate::common::mutated;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;

const CDM_KVN: &str = include_str!("../../data/kvn/cdm_362.kvn");
const CDM_XML: &str = include_str!("../../data/xml/cdm_44.xml");

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
        let invalid = mutated(CDM_KVN, needle, replacement);
        crate::common::assert_invalid_epoch(&Cdm::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let invalid = mutated(
        CDM_XML,
        "<CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>",
        "<CREATION_DATE>123.5</CREATION_DATE>",
    );
    crate::common::assert_invalid_epoch(&Cdm::from_xml(&invalid).unwrap_err(), "123.5");
}
