use ccsds_ndm::error::{
    CcsdsNdmError, DiagnosticNotation, DiagnosticOperation, DiagnosticSeverity,
};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::{GenerateOptions, MessageType, VersionedNdm};

type Snapshot = (
    DiagnosticSeverity,
    DiagnosticOperation,
    DiagnosticNotation,
    MessageKind,
    String,
    String,
    Option<&'static str>,
    Option<String>,
);

fn snapshot(error: CcsdsNdmError) -> Snapshot {
    let diagnostic = error
        .diagnostic()
        .unwrap_or_else(|| panic!("generation error omitted structured context: {error}"));
    assert_eq!(diagnostic.requirement, None);
    assert_eq!(diagnostic.source_location, None);
    assert_eq!(diagnostic.recovery, None);
    (
        diagnostic.severity,
        diagnostic.operation,
        diagnostic.notation,
        diagnostic.message_kind,
        diagnostic
            .source_edition
            .expect("generation source edition should be present")
            .to_owned(),
        diagnostic
            .target_edition
            .expect("generation target edition should be present")
            .to_owned(),
        diagnostic.code,
        diagnostic.field_path,
    )
}

fn invalid_opm() -> Opm {
    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn")).expect("fixture should parse");
    message.body.segment.metadata.object_name.clear();
    message
}

#[test]
fn xml_generation_context_is_identical_across_public_entry_points() {
    let message = invalid_opm();
    let expected = snapshot(message.to_xml().expect_err("invalid model must fail"));

    let mut output = Vec::new();
    let erased = MessageType::Opm(message.clone());
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let errors = [
        message
            .to_xml_with(&GenerateOptions::source())
            .expect_err("versioned generation must fail"),
        message
            .write_xml_to(&mut output, &GenerateOptions::source())
            .expect_err("streaming generation must fail"),
        erased
            .to_xml()
            .expect_err("type-erased generation must fail"),
        erased
            .to_xml_with(&GenerateOptions::source())
            .expect_err("type-erased versioned generation must fail"),
        erased
            .to_xml_file(directory.path().join("invalid.xml"))
            .expect_err("file generation must fail"),
    ];

    assert!(output.is_empty());
    for error in errors {
        assert_eq!(snapshot(error), expected);
    }
    assert_eq!(expected.0, DiagnosticSeverity::Error);
    assert_eq!(expected.1, DiagnosticOperation::Generate);
    assert_eq!(expected.2, DiagnosticNotation::Xml);
    assert_eq!(expected.3, MessageKind::Opm);
    assert_eq!(expected.4, "3.0");
    assert_eq!(expected.5, "3.0");
}

#[test]
fn kvn_generation_context_is_identical_across_public_entry_points() {
    let message = invalid_opm();
    let expected = snapshot(message.to_kvn().expect_err("invalid model must fail"));

    let mut output = Vec::new();
    let erased = MessageType::Opm(message.clone());
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let errors = [
        message
            .to_kvn_with(&GenerateOptions::source())
            .expect_err("versioned generation must fail"),
        message
            .write_kvn_to(&mut output, &GenerateOptions::source())
            .expect_err("streaming generation must fail"),
        erased
            .to_kvn()
            .expect_err("type-erased generation must fail"),
        erased
            .to_kvn_with(&GenerateOptions::source())
            .expect_err("type-erased versioned generation must fail"),
        erased
            .to_kvn_file(directory.path().join("invalid.kvn"))
            .expect_err("file generation must fail"),
    ];

    assert!(output.is_empty());
    for error in errors {
        assert_eq!(snapshot(error), expected);
    }
    assert_eq!(expected.2, DiagnosticNotation::Kvn);
    assert_eq!(expected.3, MessageKind::Opm);
}
