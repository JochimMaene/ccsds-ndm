use crate::common::{fixtures, validate_xml};
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

#[test]
fn combined_generation_preserves_child_checks() {
    let opm = Opm::from_kvn(OPM_KVN).unwrap();
    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Opm(opm.clone())],
    };
    let message = Message::Ndm(combined.clone());

    message.to_xml().unwrap();

    let mut invalid_xml = opm;
    invalid_xml.header.originator = "control \u{1}".into();
    let invalid_xml = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Opm(invalid_xml)],
    };
    let error = invalid_xml.to_xml().unwrap_err();
    assert_eq!(
        error.diagnostic().unwrap().message_kind,
        ccsds_ndm::validation::MessageKind::Opm
    );
}

/// The combined envelope dispatches per-family validation and generation for every constituent.
#[test]
fn combined_envelope_carries_one_message_of_every_family() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/kvn");
    let messages: Vec<Message> = [
        "acm_g6.kvn",
        "aem_g4.kvn",
        "apm_g1.kvn",
        "cdm_362.kvn",
        "ocm_g15.kvn",
        "oem_g11.kvn",
        "omm_g7.kvn",
        "opm_g1.kvn",
        "rdm_c1.kvn",
        "tdm_e1.kvn",
    ]
    .iter()
    .map(|fixture| ccsds_ndm::from_file(root.join(fixture)).unwrap())
    .collect();

    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: messages.clone(),
    };
    ccsds_ndm::Validate::validate(&combined).unwrap();

    let xml = Message::Ndm(combined).to_xml().unwrap();
    let Message::Ndm(reparsed) = ccsds_ndm::from_str(&xml).unwrap() else {
        panic!("combined envelope did not round-trip");
    };
    assert_eq!(reparsed.messages, messages);
}
