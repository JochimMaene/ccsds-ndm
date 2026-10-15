use crate::common::mutated;
use crate::{KVN_FIXTURES, XML};
use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::Ndm;

#[test]
fn kvn_syntax_diagnostics_are_located_and_machine_readable() {
    let object_name = KVN_FIXTURES[0]
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = mutated(
        KVN_FIXTURES[0],
        object_name,
        &format!("{object_name}\nUNKNOWN = value"),
    );
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
    let object_name = KVN_FIXTURES[0]
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = mutated(
        &mutated(
            KVN_FIXTURES[0],
            object_name,
            &format!("{object_name}\nUNKNOWN = value"),
        ),
        "\n",
        "\r",
    );
    let error = Oem::from_kvn(&input).expect_err("unknown keyword should fail");
    let diagnostic = error.diagnostic().unwrap();

    assert_eq!(diagnostic.source_edition, Some("3.0"));
    assert_eq!(diagnostic.original_token, Some("UNKNOWN = value"));
}

#[test]
fn semantic_paths_survive_parse_context() {
    let object_name = KVN_FIXTURES[0]
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = mutated(KVN_FIXTURES[0], object_name, "OBJECT_NAME =");
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
    let input = mutated(&mutated(XML, "<oem ", "<omm "), "</oem>", "</omm>");
    let error = Oem::from_xml(&input).expect_err("wrong root should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Xml);
    assert_eq!(diagnostic.message_kind, MessageKind::Oem);
    assert_eq!(diagnostic.code, Some("parse.xml.syntax"));
    assert_eq!(diagnostic.source_location, None);
}

#[test]
fn missing_xml_covariance_epoch_reports_where_deserialization_failed() {
    let start = XML.find("<covarianceMatrix>").unwrap();
    let epoch_start = start + XML[start..].find("<EPOCH>").unwrap();
    let epoch_end = epoch_start + XML[epoch_start..].find("</EPOCH>").unwrap() + "</EPOCH>".len();
    let missing = format!("{}{}", &XML[..epoch_start], &XML[epoch_end..]);
    for input in [
        missing.clone(),
        missing.replacen(
            "<covarianceMatrix>",
            "<covarianceMatrix xmlns:extra=\n'urn:extra'>",
            1,
        ),
    ] {
        let end = input.find("</covarianceMatrix>").unwrap() + "</covarianceMatrix>".len();
        let line = input[..end].bytes().filter(|&byte| byte == b'\n').count() + 1;
        let error = Oem::from_xml(&input).unwrap_err();
        assert!(
            error.to_string().contains("missing field `EPOCH`"),
            "{error}"
        );
        assert!(
            error.to_string().contains(&format!("near line {line},")),
            "{error}"
        );
    }
}
