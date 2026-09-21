use crate::common::{fixtures, mutated_once, validate_xml};

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::{Message, Ndm, Validate};

const OPM_KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");

#[test]
fn every_shipped_combined_fixture_preserves_children_and_generates_valid_xml() {
    for (name, source) in fixtures("ndm", "xml") {
        if name == "ndm_g22.xml" {
            continue;
        }
        let message = CombinedNdm::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        let reparsed = CombinedNdm::from_xml(&xml).unwrap();
        assert_eq!(reparsed, message, "{name} typed model");
        validate_xml(&name, &xml);
    }
}

#[test]
fn shipped_g22_is_schema_valid_but_rejected_by_the_verified_opm_semantic_gate() {
    let source = include_str!("../../data/xml/ndm_g22.xml");
    validate_xml("ndm_g22.xml source", source);
    let error = CombinedNdm::from_xml(source).unwrap_err();
    assert!(error.to_string().contains("MASS"));
}

#[test]
fn combined_xml_rejects_illegal_root_and_constituent_attributes() {
    let source = include_str!("../../data/xml/ndm_g12.xml");
    for (label, xml) in [
        (
            "root id",
            mutated_once(source, "<ndm ", "<ndm id=\"not-allowed\" "),
        ),
        (
            "unknown root attribute",
            mutated_once(source, "<ndm ", "<ndm unexpected=\"value\" "),
        ),
        (
            "unknown constituent attribute",
            mutated_once(source, "<apm id=", "<apm unexpected=\"value\" id="),
        ),
        (
            "missing constituent version",
            mutated_once(source, " version=\"2.0\"", ""),
        ),
    ] {
        assert!(CombinedNdm::from_xml(&xml).is_err(), "accepted {label}");
    }
}

#[test]
fn streaming_generation_matches_string_generation() {
    let opm = Opm::from_kvn(OPM_KVN).unwrap();
    let message = CombinedNdm {
        id: None,
        comments: vec!["two OPM messages".into()],
        messages: vec![Message::Opm(opm.clone()), Message::Opm(opm)],
    };

    let xml = message.to_xml().unwrap();
    let mut xml_output = Vec::new();
    message.write_xml_to(&mut xml_output).unwrap();
    assert_eq!(xml_output, xml.as_bytes());
}

#[test]
fn loss_or_non_schema_model_states_are_rejected() {
    let nested = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Ndm(CombinedNdm {
            id: None,
            comments: Vec::new(),
            messages: Vec::new(),
        })],
    };
    crate::common::assert_validation_field(&nested.validate().unwrap_err(), "ndm");
    crate::common::assert_validation_field(&nested.to_xml().unwrap_err(), "ndm");
}

#[test]
fn combined_xml_string_generation_identifies_invalid_envelope_fields() {
    let mut message =
        ccsds_ndm::messages::ndm::CombinedNdm::from_xml(include_str!("../../data/xml/ndm_g12.xml"))
            .unwrap();
    message.id = Some("bad\u{1}id".into());
    let error = message.to_xml().unwrap_err();
    assert!(error.to_string().contains("MESSAGE_ID"));
}
