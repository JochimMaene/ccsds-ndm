use std::fs;

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Message;
use ccsds_ndm::{Ndm, Validate};

mod common;
use common::{data_dir, validate_xml};

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

#[test]
fn every_shipped_combined_fixture_preserves_children_and_generates_valid_xml() {
    for name in ["ndm_g12.xml", "ndm_g21.xml"] {
        let source = fs::read_to_string(data_dir().join("xml").join(name)).unwrap();
        let message = CombinedNdm::from_xml(&source).unwrap();
        let kinds: Vec<_> = message.messages.iter().map(Message::kind).collect();
        let xml = message.to_xml().unwrap();
        let reparsed = CombinedNdm::from_xml(&xml).unwrap();
        assert_eq!(reparsed, message, "{name} typed model");
        assert_eq!(
            reparsed
                .messages
                .iter()
                .map(Message::kind)
                .collect::<Vec<_>>(),
            kinds,
            "{name} child order"
        );
        validate_xml(name, &xml);
    }
}

#[test]
fn shipped_g22_is_schema_valid_but_rejected_by_the_verified_opm_semantic_gate() {
    let source = fs::read_to_string(data_dir().join("xml/ndm_g22.xml")).unwrap();
    validate_xml("ndm_g22.xml source", &source);
    let error = CombinedNdm::from_xml(&source).unwrap_err();
    assert!(error.to_string().contains("MASS"));
}

#[test]
fn combined_xml_rejects_illegal_root_and_constituent_attributes() {
    let source = fs::read_to_string(data_dir().join("xml/ndm_g12.xml")).unwrap();
    for (label, xml) in [
        (
            "root id",
            source.replacen("<ndm ", "<ndm id=\"not-allowed\" ", 1),
        ),
        (
            "unknown root attribute",
            source.replacen("<ndm ", "<ndm unexpected=\"value\" ", 1),
        ),
        (
            "unknown constituent attribute",
            source.replacen("<apm id=", "<apm unexpected=\"value\" id=", 1),
        ),
        (
            "missing constituent version",
            source.replacen(" version=\"2.0\"", "", 1),
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
    assert!(nested.validate().is_err());
    assert!(nested.to_xml().is_err());
}
