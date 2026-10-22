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
    assert_eq!(diagnostic.expected, None);
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
fn blank_mandatory_value_is_missing_at_its_record() {
    let object_name = KVN_FIXTURES[0]
        .lines()
        .find(|line| line.trim_start().starts_with("OBJECT_NAME"))
        .unwrap();
    let input = mutated(KVN_FIXTURES[0], object_name, "OBJECT_NAME =");
    let error = Oem::from_kvn(&input).expect_err("empty object name should fail");
    let diagnostic = error.diagnostic().expect("parse context should be present");
    assert_eq!(diagnostic.code, Some("validation.missing_required_field"));
    // A blank mandatory value is absent under the shared KVN rule; the error names its record.
    let line = KVN_FIXTURES[0]
        .lines()
        .position(|line| line == object_name)
        .unwrap()
        + 1;
    assert!(matches!(
        crate::common::validation_error_source(error.as_validation_error().unwrap()),
        ccsds_ndm::error::ValidationError::MissingRequiredField { line: Some(found), .. } if *found == line
    ));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[0].metadata.object_name")
    );

    // The path names the segment that holds the blank record.
    let second = KVN_FIXTURES[0].rfind(object_name).unwrap();
    let input = format!(
        "{}OBJECT_NAME ={}",
        &KVN_FIXTURES[0][..second],
        &KVN_FIXTURES[0][second + object_name.len()..]
    );
    let error = Oem::from_kvn(&input).unwrap_err();
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[1].metadata.object_name")
    );
    assert!(
        error.to_string().contains("in block OEM Metadata"),
        "{error}"
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
