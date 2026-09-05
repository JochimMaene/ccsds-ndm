use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::{from_str_with_options, Message, Notation, ParseOptions};
use ccsds_ndm::{Ndm, Validate};
use tempfile::NamedTempFile;

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");
const OPM_WITH_MANEUVERS_KVN: &str = include_str!("../data/kvn/opm_g2.kvn");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn every_shipped_combined_fixture_preserves_children_and_generates_valid_xml() {
    for name in ["ndm_g12.xml", "ndm_g21.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
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
    let source = fs::read_to_string(repository_path("data/xml/ndm_g22.xml")).unwrap();
    validate_xml("ndm_g22.xml source", &source);
    let error = CombinedNdm::from_xml(&source).unwrap_err();
    assert!(error.to_string().contains("MASS"));
}

#[test]
fn combined_xml_rejects_illegal_root_and_constituent_attributes() {
    let source = fs::read_to_string(repository_path("data/xml/ndm_g12.xml")).unwrap();
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
fn aggregate_parse_limits_apply_to_direct_combined_entry_points() {
    let source = fs::read_to_string(repository_path("data/xml/ndm_g21.xml")).unwrap();
    let error = from_str_with_options(
        &source,
        Some(Notation::Xml),
        &ParseOptions::default().with_max_input_bytes(source.len() - 1),
    )
    .unwrap_err();
    assert_eq!(error.code(), Some("resource.input_limit_exceeded"));
    assert!(from_str_with_options(
        &source,
        Some(Notation::Xml),
        &ParseOptions::default().with_max_xml_depth(1)
    )
    .is_err());

    let aem = Aem::from_xml(include_str!("../data/xml/aem_g11.xml")).unwrap();
    let two_children = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Aem(aem.clone()), Message::Aem(aem)],
    }
    .to_xml()
    .unwrap();
    let error = from_str_with_options(
        &two_children,
        Some(Notation::Xml),
        &ParseOptions::default().with_max_records(1),
    )
    .unwrap_err();
    assert_eq!(
        error.code(),
        Some("resource.record_limit_exceeded"),
        "{error:?}"
    );
}

#[test]
fn opm_maneuvers_are_not_history_records_in_standalone_or_combined_messages() {
    let options = ParseOptions::default().with_max_records(0);
    let Message::Opm(opm) =
        from_str_with_options(OPM_WITH_MANEUVERS_KVN, Some(Notation::Kvn), &options).unwrap()
    else {
        unreachable!()
    };
    assert!(!opm.body.segment.data.maneuver_parameters.is_empty());

    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Opm(opm)],
    };
    let xml = combined.to_xml().unwrap();
    from_str_with_options(&xml, Some(Notation::Xml), &options).unwrap();
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

fn validate_xml(label: &str, xml: &str) {
    let document = NamedTempFile::new().unwrap();
    fs::write(document.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(repository_path("data/xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
