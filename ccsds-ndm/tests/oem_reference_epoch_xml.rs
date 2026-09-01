use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::Epoch;

const OEM_KVN: &str = include_str!("../data/kvn/oem_g11.kvn");
const OEM_XML: &str = include_str!("../data/xml/oem_g14.xml");

#[test]
fn xml_reference_frame_epoch_requires_calendar_form() {
    let kvn = OEM_KVN.replacen(
        "REF_FRAME = EME2000\n",
        "REF_FRAME = EME2000\nREF_FRAME_EPOCH = 2000-01-01T12:00:00\n",
        1,
    );
    let oem = Oem::from_kvn(&kvn).expect("calendar frame epoch should parse");
    let xml = oem.to_xml().expect("valid OEM should generate");
    assert!(xml.contains("<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>"));

    let numeric = xml.replace(
        "<REF_FRAME_EPOCH>2000-01-01T12:00:00</REF_FRAME_EPOCH>",
        "<REF_FRAME_EPOCH>123.5</REF_FRAME_EPOCH>",
    );
    assert!(Oem::from_xml(&numeric).is_err());
}

#[test]
fn xml_contextual_epoch_fields_reject_invalid_values() {
    for (needle, replacement) in [
        (
            "<START_TIME>2019-12-18T12:00:00.331</START_TIME>",
            "<START_TIME>2023-02-29T12:00:00</START_TIME>",
        ),
        (
            "<USEABLE_START_TIME>2019-12-18T12:10:00.331</USEABLE_START_TIME>",
            "<USEABLE_START_TIME>+</USEABLE_START_TIME>",
        ),
        ("<EPOCH>2019-12-18T12:00:00.331</EPOCH>", "<EPOCH>+</EPOCH>"),
        ("<EPOCH>2019-12-28T22:28:00.331</EPOCH>", "<EPOCH>.</EPOCH>"),
    ] {
        let invalid = OEM_XML.replacen(needle, replacement, 1);
        assert!(
            Oem::from_xml(&invalid).is_err(),
            "accepted invalid contextual epoch replacement {replacement:?}"
        );
    }
}

#[test]
fn generation_rejects_mutated_contextual_epochs() {
    let mut oem = Oem::from_xml(OEM_XML).expect("fixture should parse");
    oem.body.segment[0].metadata.start_time = Epoch::new("+").unwrap();
    assert!(oem.to_xml().is_err());
    assert!(oem.to_kvn().is_err());

    let mut oem = Oem::from_xml(OEM_XML).expect("fixture should parse");
    oem.body.segment[0].data.state_vector[0].epoch = Epoch::new("2023-02-29T00:00:00").unwrap();
    assert!(oem.to_xml().is_err());
}
