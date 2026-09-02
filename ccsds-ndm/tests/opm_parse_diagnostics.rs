use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::validation::MessageKind;

const KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

#[test]
fn kvn_parse_diagnostic_is_located_bounded_and_machine_readable() {
    let input = KVN.replace(
        "OBJECT_NAME = OSPREY 5",
        "OBJECT_NAME = OSPREY 5\nUNKNOWN_KEY = value",
    );
    let error = Opm::from_kvn(&input).expect_err("unknown keyword should fail");
    let diagnostic = error
        .diagnostic()
        .expect("strict parser should attach diagnostic context");

    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
    assert_eq!(diagnostic.message_kind, MessageKind::Opm);
    assert_eq!(diagnostic.source_edition, Some("3.0"));
    assert_eq!(diagnostic.code, Some("parse.kvn.syntax"));
    assert_eq!(diagnostic.source_location, Some((6, 1)));
    assert_eq!(diagnostic.original_token, Some("UNKNOWN_KEY = value"));
    assert_eq!(diagnostic.expected, Some("strict OPM KVN"));
}

#[test]
fn parse_token_excerpt_is_bounded_and_semantic_paths_survive_the_context_wrapper() {
    let long = KVN.replace(
        "OBJECT_NAME = OSPREY 5",
        &format!("OBJECT_NAME = {}", "x".repeat(300)),
    );
    let error = Opm::from_kvn(&long).expect_err("overlong line should fail");
    assert!(
        error
            .diagnostic()
            .and_then(|diagnostic| diagnostic.original_token)
            .expect("long token should have an excerpt")
            .len()
            <= 128
    );

    let empty_name = KVN.replace("OBJECT_NAME = OSPREY 5", "OBJECT_NAME =");
    let error = Opm::from_kvn(&empty_name).expect_err("empty required field should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.code, Some("validation.missing_required_field"));
    assert_eq!(
        diagnostic.field_path.as_deref(),
        Some("body.segment.metadata.object_name")
    );
}

#[test]
fn xml_parse_diagnostic_identifies_input_notation_without_inventing_a_location() {
    let xml = Opm::from_kvn(KVN)
        .expect("fixture should parse")
        .to_xml()
        .expect("fixture should generate XML");
    let wrong_root = xml.replace("<opm ", "<omm ").replace("</opm>", "</omm>");
    let error = Opm::from_xml(&wrong_root).expect_err("wrong root should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");

    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Xml);
    assert_eq!(diagnostic.message_kind, MessageKind::Opm);
    assert_eq!(diagnostic.code, Some("parse.xml.syntax"));
    assert_eq!(diagnostic.source_location, None);
    assert_eq!(diagnostic.original_token, None);
}
