use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::messages::{aem::Aem, oem::Oem, opm::Opm};
use ccsds_ndm::{convert, convert_file, Message, Ndm, Notation};

use crate::common::assert_rejects;

// Each typed adapter must preserve the same output and preflight guarantees.
fn assert_output_contract<M: Ndm + Clone>(
    valid: &M,
    invalid: &M,
    field: &str,
    wrap: fn(M) -> Message,
) {
    let kvn = valid.to_kvn().unwrap();
    let xml = valid.to_xml().unwrap();
    assert_eq!(valid.to_kvn().unwrap(), kvn);
    assert_eq!(valid.to_xml().unwrap(), xml);
    let mut output = Vec::new();
    valid.write_kvn_to(&mut output).unwrap();
    assert_eq!(output, kvn.as_bytes());
    output.clear();
    valid.write_xml_to(&mut output).unwrap();
    assert_eq!(output, xml.as_bytes());
    let erased = wrap(valid.clone());
    assert_eq!(erased.to_kvn().unwrap(), kvn);
    assert_eq!(erased.to_xml().unwrap(), xml);
    output.clear();
    erased.write_kvn_to(&mut output).unwrap();
    assert_eq!(output, kvn.as_bytes());
    output.clear();
    erased.write_xml_to(&mut output).unwrap();
    assert_eq!(output, xml.as_bytes());
    let expected = invalid.validate().unwrap_err();
    let invalid_erased = wrap(invalid.clone());
    output.clear();
    for error in [
        invalid_erased.to_kvn().unwrap_err(),
        invalid_erased.to_xml().unwrap_err(),
        invalid_erased.write_kvn_to(&mut output).unwrap_err(),
        invalid_erased.write_xml_to(&mut output).unwrap_err(),
    ] {
        assert_eq!(error.as_validation_error(), expected.as_validation_error());
    }
    assert!(output.is_empty(), "invalid generic output wrote bytes");

    for (notation, expected) in [(Notation::Kvn, &kvn), (Notation::Xml, &xml)] {
        assert_eq!(
            ccsds_ndm::from_str_with_notation(expected, Some(notation)).unwrap(),
            erased,
            "{:?} {notation:?} dispatch and model preservation",
            erased.kind()
        );
        let mut sink = InterruptedChunks::default();
        match notation {
            Notation::Kvn => valid.write_kvn_to(&mut sink),
            Notation::Xml => valid.write_xml_to(&mut sink),
        }
        .unwrap();
        assert_eq!(sink.accepted, expected.as_bytes());

        // A slice writer accepts a short prefix, then returns zero once full.
        let mut prefix = [0; 7];
        let error = match notation {
            Notation::Kvn => valid.write_kvn_to(&mut prefix.as_mut_slice()),
            Notation::Xml => valid.write_xml_to(&mut prefix.as_mut_slice()),
        }
        .unwrap_err();
        let ccsds_ndm::error::CcsdsNdmError::Generation { source, .. } = error else {
            panic!("missing generation error context");
        };
        let ccsds_ndm::error::CcsdsNdmError::Io(error) = *source else {
            panic!("expected the underlying I/O error");
        };
        assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
        assert_eq!(prefix, expected.as_bytes()[..7]);

        // Fail before output, inside the prefix, and at the end of the document.
        let mut limits = vec![0, 1, expected.len() - 1];
        match notation {
            Notation::Kvn => {
                limits.push("CCSDS_OPM_VERS".len() / 2);
                // OPM additionally exercises failures inside state and user-defined records.
                for (key, offset) in [
                    ("\nX                    =", "\nX                    =".len()),
                    ("\nUSER_DEFINED_", 2),
                ] {
                    if let Some(start) = expected.find(key) {
                        limits.push(start + offset);
                    }
                }
            }
            Notation::Xml => {
                let declaration_length = r#"<?xml version="1.0" encoding="UTF-8"?>"#.len();
                limits.extend([declaration_length / 2, declaration_length + 1]);
            }
        }
        for limit in limits {
            let mut sink = FailAfter::new(limit);
            let error = match notation {
                Notation::Kvn => valid.write_kvn_to(&mut sink),
                Notation::Xml => valid.write_xml_to(&mut sink),
            }
            .unwrap_err();
            assert_eq!(error.code(), Some("io.error"));
            assert_eq!(error.field_path(), None);
            let diagnostic = error.diagnostic().expect("generation context");
            assert_eq!(diagnostic.message_kind, erased.kind());
            assert_eq!(diagnostic.field_path, None);
            assert_eq!(
                diagnostic.notation,
                match notation {
                    Notation::Kvn => ccsds_ndm::error::DiagnosticNotation::Kvn,
                    Notation::Xml => ccsds_ndm::error::DiagnosticNotation::Xml,
                }
            );
            let ccsds_ndm::error::CcsdsNdmError::Generation { source, .. } = error else {
                panic!("missing generation error context");
            };
            let ccsds_ndm::error::CcsdsNdmError::Io(error) = *source else {
                panic!("expected the underlying I/O error");
            };
            assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
            assert_eq!(error.to_string(), "deliberate test sink failure");
            assert_eq!(sink.accepted, expected.as_bytes()[..limit]);
        }
    }
    assert_rejects(invalid, field);
}

// Exercise Write's retry and partial-write contracts throughout a successful document.
#[derive(Default)]
struct InterruptedChunks {
    accepted: Vec<u8>,
    interrupted: bool,
}

impl std::io::Write for InterruptedChunks {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.interrupted = !self.interrupted;
        if self.interrupted {
            return Err(std::io::ErrorKind::Interrupted.into());
        }
        let count = buffer.len().min(7);
        self.accepted.extend_from_slice(&buffer[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn aem_output_contract() {
    let valid = Aem::from_kvn(include_str!("../../data/kvn/aem_g4.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment[0].metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Aem);
}

#[test]
fn oem_output_contract() {
    let valid = Oem::from_kvn(include_str!("../../data/kvn/oem_g13.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment[0].metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Oem);
}

#[test]
fn opm_output_contract() {
    let valid = Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Opm);
}

#[test]
fn acm_output_contract() {
    let valid = Acm::from_kvn(include_str!("../../data/kvn/acm_g6.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Acm);
}

#[test]
fn apm_output_contract() {
    let valid = Apm::from_kvn(include_str!("../../data/kvn/apm_g1.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Apm);
}

#[test]
fn cdm_output_contract() {
    let valid = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segments[0].metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Cdm);
}

#[test]
fn ocm_output_contract() {
    let valid = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g15.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.header.originator.clear();
    assert_output_contract(&valid, &invalid, "ORIGINATOR", Message::Ocm);
}

#[test]
fn omm_output_contract() {
    let valid = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Omm);
}

#[test]
fn rdm_output_contract() {
    let valid = Rdm::from_kvn(include_str!("../../data/kvn/rdm_c1.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, "OBJECT_NAME", Message::Rdm);
}

#[test]
fn tdm_output_contract() {
    let valid = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e1.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.header.originator.clear();
    assert_output_contract(&valid, &invalid, "ORIGINATOR", Message::Tdm);
}

#[test]
fn file_conversion_replaces_destinations_and_preserves_permissions() {
    let input = include_str!("../../data/kvn/opm_g1.kvn");
    for notation in [Notation::Kvn, Notation::Xml] {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.kvn");
        let destination = directory.path().join("destination");
        std::fs::write(&source, input).unwrap();
        convert_file(&source, &destination, notation).unwrap();
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            convert(input, notation).unwrap()
        );

        std::fs::write(&destination, b"sentinel").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&destination)
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                std::fs::metadata(&source).unwrap().permissions().mode() & 0o777
            );
            std::fs::set_permissions(&destination, std::fs::Permissions::from_mode(0o640)).unwrap();
        }
        convert_file(&source, &destination, notation).unwrap();
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            convert(input, notation).unwrap()
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&destination)
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o640
            );
        }
        assert_eq!(std::fs::read_to_string(&source).unwrap(), input);
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 2);
    }
}

#[test]
fn failed_file_conversion_preserves_existing_and_absent_destinations() {
    let legacy = crate::common::mutated_once(
        include_str!("../../data/kvn/aem_g4.kvn"),
        "_VERS = 2.0",
        "_VERS = 1.0",
    );
    // A legacy message parses successfully but cannot be generated in either notation.
    ccsds_ndm::from_str(&legacy).unwrap();
    for input in ["invalid message", legacy.as_str()] {
        for notation in [Notation::Kvn, Notation::Xml] {
            let expected = convert(input, notation).unwrap_err();
            for existing in [false, true] {
                let directory = tempfile::tempdir().unwrap();
                let source = directory.path().join("source");
                let destination = directory.path().join("destination");
                std::fs::write(&source, input).unwrap();
                if existing {
                    std::fs::write(&destination, b"sentinel").unwrap();
                }
                let error = convert_file(&source, &destination, notation).unwrap_err();
                assert_eq!(error.code(), expected.code());
                assert_eq!(error.to_string(), expected.to_string());
                if existing {
                    assert_eq!(std::fs::read(&destination).unwrap(), b"sentinel");
                } else {
                    assert!(!destination.exists());
                }
                assert_eq!(std::fs::read_to_string(&source).unwrap(), input);
                assert_eq!(
                    std::fs::read_dir(directory.path()).unwrap().count(),
                    1 + usize::from(existing),
                    "conversion left temporary files"
                );
            }
        }
    }
}

#[test]
fn file_conversion_can_replace_its_own_source() {
    let input = include_str!("../../data/kvn/opm_g1.kvn");
    let expected = ccsds_ndm::from_str(input).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("message");
    std::fs::write(&path, input).unwrap();
    for notation in [Notation::Xml, Notation::Kvn] {
        let before = std::fs::read_to_string(&path).unwrap();
        convert_file(&path, &path, notation).unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            convert(&before, notation).unwrap()
        );
        assert_eq!(ccsds_ndm::from_file(&path).unwrap(), expected);
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
    }
}

#[test]
fn xml_generation_rejects_forbidden_xml_1_characters_before_streaming() {
    let mut cdm = Cdm::from_kvn(include_str!("../../data/kvn/cdm_362.kvn")).unwrap();
    cdm.header.originator = "JS\u{1}POC".into();
    let error = cdm.to_xml().expect_err("U+0001 reached XML generation");
    assert!(
        error.to_string().contains("only XML 1.0 characters"),
        "unexpected diagnostic: {error}"
    );

    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &cdm.write_xml_to(&mut output).unwrap_err(),
        "XML 1.0 characters",
    );
    assert!(output.is_empty());
}

#[test]
fn kvn_generation_rejects_non_ascii_and_control_text_before_streaming() {
    let mut apm = Apm::from_kvn(include_str!("../../data/kvn/apm_g1.kvn")).unwrap();
    apm.header.originator = "GSFC-é".into();
    let error = apm.to_kvn().expect_err("non-ASCII reached KVN generation");
    assert!(
        error.to_string().contains("only printable ASCII records"),
        "unexpected diagnostic: {error}"
    );

    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &apm.write_kvn_to(&mut output).unwrap_err(),
        "printable ASCII",
    );
    assert!(output.is_empty());

    // A tab is not printable ASCII either, and the streaming path must hold the same line: the
    // two notations had already drifted apart on whether they checked it before writing.
    let mut tdm = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e1.kvn")).unwrap();
    tdm.header.originator = "NA\tSA".into();
    let error = tdm.to_kvn().expect_err("a tab reached KVN generation");
    assert!(
        error.to_string().contains("expected printable ASCII"),
        "unexpected diagnostic: {error}"
    );

    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &tdm.write_kvn_to(&mut output).unwrap_err(),
        "ORIGINATOR",
    );
    assert!(output.is_empty(), "streaming KVN wrote bytes for a tab");
}

#[test]
fn type_erased_file_errors_keep_non_opm_generation_context() {
    let message = Message::Oem(Oem::from_kvn(include_str!("../../data/kvn/oem_g11.kvn")).unwrap());
    let directory = tempfile::tempdir().unwrap();
    let sentinel = directory.path().join("sentinel");
    std::fs::write(&sentinel, b"keep me").unwrap();
    let destination = directory.path().join("existing-directory");
    std::fs::create_dir(&destination).unwrap();

    for error in [
        message.to_kvn_file(&destination).unwrap_err(),
        message.to_xml_file(&destination).unwrap_err(),
    ] {
        assert_eq!(error.code(), Some("io.error"));
        let diagnostic = error.diagnostic().expect("file error should have context");
        assert_eq!(diagnostic.message_kind.as_str(), "OEM");
        assert_eq!(diagnostic.source_edition, Some("3.0"));
    }
    assert!(destination.is_dir());
    assert_eq!(std::fs::read(sentinel).unwrap(), b"keep me");
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 2);
}

#[test]
fn combined_output_contract() {
    let message = ccsds_ndm::from_str(include_str!("../../data/xml/ndm_g12.xml")).unwrap();
    let Message::Ndm(combined) = &message else {
        panic!("combined input lost its NDM identity");
    };
    let mut xml = Vec::new();
    combined.write_xml_to(&mut xml).unwrap();
    assert_eq!(xml, combined.to_xml().unwrap().as_bytes());
    let typed_xml = xml.clone();
    xml.clear();
    message.write_xml_to(&mut xml).unwrap();
    assert_eq!(xml, message.to_xml().unwrap().as_bytes());
    assert_eq!(xml, typed_xml);
    assert_eq!(
        ccsds_ndm::from_str(std::str::from_utf8(&xml).unwrap()).unwrap(),
        message
    );

    // Combined NDM has no KVN representation; both paths must refuse it.
    let mut kvn = Vec::new();
    for error in [
        combined.write_kvn_to(&mut kvn).unwrap_err(),
        combined.to_kvn().unwrap_err(),
        message.write_kvn_to(&mut kvn).unwrap_err(),
        message.to_kvn().unwrap_err(),
    ] {
        let diagnostic = error.diagnostic().unwrap();
        assert_eq!(diagnostic.code, Some("unsupported.notation"));
        assert_eq!(
            diagnostic.operation,
            ccsds_ndm::error::DiagnosticOperation::Generate
        );
        assert_eq!(
            diagnostic.notation,
            ccsds_ndm::error::DiagnosticNotation::Kvn
        );
        assert_eq!(diagnostic.source_edition, Some("combined"));
    }
    assert!(kvn.is_empty());
}

#[derive(Debug)]
struct FailAfter {
    accepted: Vec<u8>,
    limit: usize,
}

impl FailAfter {
    fn new(limit: usize) -> Self {
        Self {
            accepted: Vec::new(),
            limit,
        }
    }
}

impl std::io::Write for FailAfter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.accepted.len());
        if remaining == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "deliberate test sink failure",
            ));
        }

        let accepted = remaining.min(buffer.len());
        self.accepted.extend_from_slice(&buffer[..accepted]);
        Ok(accepted)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
