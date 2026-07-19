use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, MessageType, VersionedNdm};
use std::path::{Path, PathBuf};
use std::process::Command;

const KVN_FIXTURES: [(&str, &str); 3] = [
    ("oem_g11.kvn", include_str!("../../data/kvn/oem_g11.kvn")),
    ("oem_g12.kvn", include_str!("../../data/kvn/oem_g12.kvn")),
    ("oem_g13.kvn", include_str!("../../data/kvn/oem_g13.kvn")),
];
const XML: (&str, &str) = ("oem_g14.xml", include_str!("../../data/xml/oem_g14.xml"));

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn validate_xsd(label: &str, xml: &str) {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema_path())
        .arg(file.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance tests: {error}"));
    assert!(
        output.status.success(),
        "generated XML for {label} failed the official OEM 3.0 XSD:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn every_shipped_fixture_generates_deterministic_xsd_valid_xml_and_reparseable_kvn() {
    let mut messages = Vec::new();
    for (name, source) in KVN_FIXTURES {
        messages.push((name, Oem::from_kvn(source).unwrap()));
    }
    messages.push((XML.0, Oem::from_xml(XML.1).unwrap()));

    for (name, message) in messages {
        let xml = message.to_xml().unwrap();
        assert_eq!(message.to_xml().unwrap(), xml);
        validate_xsd(name, &xml);
        assert_eq!(Oem::from_xml(&xml).unwrap(), message);

        let kvn = message.to_kvn().unwrap();
        assert!(kvn.lines().all(|line| {
            line.len() <= 254 && line.bytes().all(|byte| (b' '..=b'~').contains(&byte))
        }));
        assert_eq!(Oem::from_kvn(&kvn).unwrap(), message);
    }
}

#[test]
fn public_generation_surfaces_are_identical_and_preflight_invalid_models() {
    let message = Oem::from_kvn(KVN_FIXTURES[2].1).unwrap();
    let expected_kvn = message.to_kvn().unwrap();
    let expected_xml = message.to_xml().unwrap();
    assert_eq!(
        message.to_kvn_with(&GenerateOptions::source()).unwrap(),
        expected_kvn
    );
    assert_eq!(
        message.to_xml_with(&GenerateOptions::source()).unwrap(),
        expected_xml
    );
    let mut streamed = Vec::new();
    message
        .write_kvn_to(&mut streamed, &GenerateOptions::source())
        .unwrap();
    assert_eq!(streamed, expected_kvn.as_bytes());
    streamed.clear();
    message
        .write_xml_to(&mut streamed, &GenerateOptions::source())
        .unwrap();
    assert_eq!(streamed, expected_xml.as_bytes());
    let erased = MessageType::Oem(message.clone());
    assert_eq!(erased.to_kvn().unwrap(), expected_kvn);
    assert_eq!(erased.to_xml().unwrap(), expected_xml);

    let mut invalid = message;
    invalid.body.segment[0].metadata.object_name.clear();
    for error in [
        invalid.to_kvn().unwrap_err(),
        invalid.to_xml().unwrap_err(),
        MessageType::Oem(invalid.clone()).to_kvn().unwrap_err(),
        MessageType::Oem(invalid).to_xml().unwrap_err(),
    ] {
        assert_eq!(error.code(), Some("validation.missing_required_field"));
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment[0].metadata.object_name")
        );
        let diagnostic = error
            .diagnostic()
            .expect("generation context should be present");
        assert_eq!(
            diagnostic.message_kind,
            ccsds_ndm::validation::MessageKind::Oem
        );
        assert_eq!(diagnostic.source_edition, Some("3.0"));
        assert_eq!(diagnostic.target_edition, Some("3.0"));
    }
}

#[test]
fn kvn_rounds_to_the_ccsds_digit_limit_and_rejects_partial_acceleration() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[2].1).unwrap();
    message.body.segment[0].data.state_vector[0].x.value = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));
    message.to_xml().expect("XML retains the exact f64 value");

    let mut message = Oem::from_xml(XML.1).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.y_ddot = None;
    let mut output = Vec::new();
    message
        .write_kvn_to(&mut output, &GenerateOptions::source())
        .expect_err("partial acceleration must fail preflight");
    assert!(output.is_empty());
    message
        .to_xml()
        .expect("partial acceleration remains valid XML");
}

#[test]
fn kvn_rejects_an_overlong_raw_record_before_writing() {
    let mut message = Oem::from_xml(XML.1).unwrap();
    let state = &mut message.body.segment[0].data.state_vector[0];
    state.epoch = "2019-12-18T12:00:00.1111111111111111111111111111111111111111"
        .parse()
        .unwrap();
    for value in [
        &mut state.x.value,
        &mut state.y.value,
        &mut state.z.value,
        &mut state.x_dot.value,
        &mut state.y_dot.value,
        &mut state.z_dot.value,
        &mut state.x_ddot.as_mut().unwrap().value,
        &mut state.y_ddot.as_mut().unwrap().value,
        &mut state.z_ddot.as_mut().unwrap().value,
    ] {
        *value = 1.797_693_134_862_315e308;
    }

    let mut output = Vec::new();
    let error = message
        .write_kvn_to(&mut output, &GenerateOptions::source())
        .expect_err("a record over 254 characters must fail preflight");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert!(output.is_empty());
}

#[test]
fn output_limits_are_exact_and_streaming_failures_emit_nothing() {
    let message = Oem::from_kvn(KVN_FIXTURES[2].1).unwrap();
    for (expected, generate, stream) in [
        (
            message.to_kvn().unwrap(),
            Oem::to_kvn_with as fn(&Oem, &GenerateOptions) -> _,
            Oem::write_kvn_to::<Vec<u8>> as fn(&Oem, &mut Vec<u8>, &GenerateOptions) -> _,
        ),
        (
            message.to_xml().unwrap(),
            Oem::to_xml_with,
            Oem::write_xml_to::<Vec<u8>>,
        ),
    ] {
        let exact = GenerateOptions::source().with_max_output_bytes(expected.len());
        let small = GenerateOptions::source().with_max_output_bytes(expected.len() - 1);
        assert_eq!(generate(&message, &exact).unwrap(), expected);
        assert_eq!(
            generate(&message, &small).unwrap_err().code(),
            Some("resource.output_limit_exceeded")
        );
        let mut output = Vec::new();
        assert!(stream(&message, &mut output, &small).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn unaudited_editions_are_rejected_instead_of_relabelled() {
    let message = Oem::from_kvn(KVN_FIXTURES[0].1).unwrap();
    let options = GenerateOptions::version("1.0");
    for error in [
        message.to_kvn_with(&options).unwrap_err(),
        message.to_xml_with(&options).unwrap_err(),
    ] {
        assert_eq!(
            error.code(),
            Some("generation.unsupported_version_conversion")
        );
    }

    let mut historical = message;
    historical.version = "1.0".into();
    assert!(historical.to_kvn().is_err());
    assert!(historical.to_xml().is_err());
}

#[test]
fn notation_specific_text_rules_fail_before_output() {
    let mut message = Oem::from_kvn(KVN_FIXTURES[2].1).unwrap();
    message.body.segment[0].metadata.object_name = "MARS €".into();
    message
        .to_xml()
        .expect("Unicode text is representable in XML 1.0");
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("validation.invalid_value")
    );

    message.body.segment[0].metadata.object_name = "MARS\u{1}".into();
    assert_eq!(
        message.to_xml().unwrap_err().code(),
        Some("validation.invalid_value")
    );

    message.body.segment[0].metadata.object_name = "x".repeat(240);
    assert_eq!(
        message.to_kvn().unwrap_err().code(),
        Some("validation.out_of_range")
    );
}
