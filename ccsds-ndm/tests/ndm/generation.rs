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
        id: Some("TEST_ID_123".into()),
        comments: vec!["Global NDM comment".into()],
        messages: messages.clone(),
    };
    ccsds_ndm::Validate::validate(&combined).unwrap();

    let xml = Message::Ndm(combined.clone()).to_xml().unwrap();
    let Message::Ndm(reparsed) = ccsds_ndm::from_str(&xml).unwrap() else {
        panic!("combined envelope did not round-trip");
    };
    assert_eq!(reparsed, combined);
}

#[test]
fn combined_schema_refuses_standalone_odm2_constituents_before_output() {
    for source in [
        include_str!("../../data/kvn/opm_g1.kvn"),
        include_str!("../../data/kvn/omm_g9.kvn"),
        include_str!("../../data/kvn/oem_g13.kvn"),
    ] {
        let mut child = ccsds_ndm::from_str(source).unwrap();
        let (version, header) = match &mut child {
            Message::Opm(value) => (&mut value.version, &mut value.header),
            Message::Omm(value) => (&mut value.version, &mut value.header),
            Message::Oem(value) => (&mut value.version, &mut value.header),
            _ => unreachable!(),
        };
        *version = "2.0".into();
        header.classification = None;
        header.message_id = None;
        let standalone = child.to_xml().unwrap();
        let combined = CombinedNdm {
            id: None,
            comments: vec![],
            messages: vec![child],
        };
        combined.validate().unwrap();
        assert!(combined
            .to_xml()
            .unwrap_err()
            .to_string()
            .contains("NDM/XML 4.0.0"));
        let mut output = Vec::new();
        assert!(combined.write_xml_to(&mut output).is_err());
        assert!(output.is_empty());
        let document = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ndm>{}</ndm>",
            standalone.split_once('\n').unwrap().1
        );
        assert_eq!(CombinedNdm::from_xml(&document).unwrap(), combined);
    }
}

#[test]
fn combined_schema_refuses_legacy_tdm_but_standalone_input_remains_supported() {
    use ccsds_ndm::messages::tdm::Tdm;
    let source = include_str!("../../data/legacy/tdm_1_0_b9.xml");
    let child = Tdm::from_xml(source).unwrap();
    assert_eq!(child.version, "1.0");
    let body = &source[source.find("<header>").unwrap()..source.rfind("</tdm>").unwrap()];
    let xml = format!("<ndm><tdm id=\"CCSDS_TDM_VERS\" version=\"1.0\">{body}</tdm></ndm>");
    assert!(CombinedNdm::from_xml(&xml).is_ok());
    assert!(ccsds_ndm::from_str(&xml).is_ok());
    let combined = CombinedNdm {
        id: None,
        comments: vec![],
        messages: vec![Message::Tdm(child)],
    };
    combined.validate().unwrap();
    assert!(combined.to_xml().is_err());
    let mut output = Vec::new();
    assert!(combined.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn combined_xml_output_applies_cdm_available_value_checks() {
    let mut cdm =
        ccsds_ndm::messages::cdm::Cdm::from_kvn(include_str!("../../data/kvn/cdm_363.kvn"))
            .unwrap();
    cdm.header.message_for = Some("N/A".into());
    assert!(cdm.to_xml().is_err());
    let combined = CombinedNdm {
        id: None,
        comments: vec![],
        messages: vec![Message::Cdm(cdm)],
    };
    assert!(combined.to_xml().is_err());
    let mut output = Vec::new();
    assert!(combined.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn shipped_g22_ocm_constituent_violates_man_composition_order() {
    let source = include_str!("../../data/xml/ndm_g22.xml");
    let opm = &source[source.find("<opm ").unwrap()..source.find("</opm>").unwrap() + 6];
    let without_opm = source.replacen(opm, "", 1);
    let error = CombinedNdm::from_xml(&without_opm).unwrap_err().to_string();
    assert!(error.contains("MAN_COMPOSITION"), "{error}");
    let ocm = &without_opm[without_opm.find("<ocm ").unwrap()..];
    let ocm = &ocm[..ocm.find("</ocm>").unwrap() + 6];
    let without_both = without_opm.replacen(ocm, "", 1);
    assert_eq!(
        CombinedNdm::from_xml(&without_both).unwrap().messages.len(),
        2
    );
}

#[test]
fn constituent_namespace_declarations_are_namespace_scope_not_attributes() {
    let source = include_str!("../../data/xml/ndm_g12.xml");
    let expected = CombinedNdm::from_xml(source).unwrap();
    let child = &source[source.find("<apm").unwrap()..];
    let tag = &child[..child.find('>').unwrap()];
    for declaration in [
        " xmlns:local=\"urn:example:local\"",
        " xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"",
    ] {
        let declared = source.replacen(tag, &format!("{tag}{declaration}"), 1);
        assert_eq!(
            CombinedNdm::from_xml(&declared).unwrap(),
            expected,
            "{declaration}"
        );
    }
    // A schema-location hint is an attribute, not a declaration (ODM 8.12.7).
    let hinted = source.replacen(
        tag,
        &format!("{tag} xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"x.xsd\""),
        1,
    );
    assert!(CombinedNdm::from_xml(&hinted).is_err());
}

#[test]
fn empty_combined_envelope_is_accepted_and_generated_despite_the_should() {
    for source in ["<ndm/>", "<ndm></ndm>"] {
        let empty = CombinedNdm::from_xml(source).unwrap();
        assert!(empty.messages.is_empty());
        let xml = empty.to_xml().unwrap();
        validate_xml("empty combined NDM", &xml);
        assert_eq!(CombinedNdm::from_xml(&xml).unwrap(), empty);
    }
}
