use crate::common::mutated;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;

const AEM_KVN: &str = include_str!("../../data/kvn/aem_g4.kvn");
const AEM_XML: &str = include_str!("../../data/xml/aem_g11.xml");

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
        let invalid = mutated(AEM_KVN, needle, replacement);
        crate::common::assert_invalid_epoch(&Aem::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let invalid_metadata = mutated(
        AEM_XML,
        "<START_TIME>2006-090T05:00:00.071</START_TIME>",
        "<START_TIME>123.5</START_TIME>",
    );
    crate::common::assert_invalid_epoch(&Aem::from_xml(&invalid_metadata).unwrap_err(), "123.5");

    let invalid_state = mutated(
        AEM_XML,
        "<EPOCH>2006-090T05:00:00.071</EPOCH>",
        "<EPOCH>123.5</EPOCH>",
    );
    crate::common::assert_invalid_epoch(&Aem::from_xml(&invalid_state).unwrap_err(), "123.5");
}

#[test]
fn xml_parsing_rejects_non_calendar_adm_creation_date() {
    let invalid = mutated(
        AEM_XML,
        "<CREATION_DATE>2008-071T17:09:49</CREATION_DATE>",
        "<CREATION_DATE>12345</CREATION_DATE>",
    );
    crate::common::assert_invalid_epoch(&Aem::from_xml(&invalid).unwrap_err(), "12345");
}
