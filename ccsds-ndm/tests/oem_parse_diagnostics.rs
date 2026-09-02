use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::validation::MessageKind;

const KVN: &str = include_str!("../data/kvn/oem_g11.kvn");
const XML: &str = include_str!("../data/xml/oem_g14.xml");

#[test]
fn kvn_syntax_diagnostics_are_located_and_machine_readable() {
    let object_name = KVN
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = KVN.replace(object_name, &format!("{object_name}\nUNKNOWN = value"));
    let error = Oem::from_kvn(&input).expect_err("unknown keyword should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");

    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
    assert_eq!(diagnostic.message_kind, MessageKind::Oem);
    assert_eq!(diagnostic.source_edition, Some("3.0"));
    assert_eq!(diagnostic.code, Some("parse.kvn.syntax"));
    assert_eq!(diagnostic.original_token, Some("UNKNOWN = value"));
    assert_eq!(diagnostic.expected, Some("strict OEM KVN"));
    assert!(diagnostic.source_location.is_some());
}

#[test]
fn carriage_return_diagnostics_keep_the_edition_and_excerpt_to_one_record() {
    let object_name = KVN
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = KVN
        .replace(object_name, &format!("{object_name}\nUNKNOWN = value"))
        .replace('\n', "\r");
    let error = Oem::from_kvn(&input).expect_err("unknown keyword should fail");
    let diagnostic = error.diagnostic().unwrap();

    assert_eq!(diagnostic.source_edition, Some("3.0"));
    assert_eq!(diagnostic.original_token, Some("UNKNOWN = value"));
}

#[test]
fn semantic_paths_survive_parse_context() {
    let object_name = KVN
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = KVN.replace(object_name, "OBJECT_NAME =");
    let error = Oem::from_kvn(&input).expect_err("empty object name should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.code, Some("validation.missing_required_field"));
    assert_eq!(
        diagnostic.field_path.as_deref(),
        Some("body.segment[0].metadata.object_name")
    );
}

#[test]
fn xml_syntax_diagnostics_identify_the_notation_without_inventing_a_location() {
    let input = XML.replace("<oem ", "<omm ").replace("</oem>", "</omm>");
    let error = Oem::from_xml(&input).expect_err("wrong root should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Xml);
    assert_eq!(diagnostic.message_kind, MessageKind::Oem);
    assert_eq!(diagnostic.code, Some("parse.xml.syntax"));
    assert_eq!(diagnostic.source_location, None);
}
