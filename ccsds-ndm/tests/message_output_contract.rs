use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::messages::{aem::Aem, oem::Oem, opm::Opm};
use ccsds_ndm::{convert, convert_file, Message, Ndm, Notation};

mod common;
use common::{assert_rejects, FailAfter};

// Each typed adapter must preserve the same output and preflight guarantees.
fn assert_output_contract<M: Ndm + Clone>(valid: &M, invalid: &M, wrap: fn(M) -> Message) {
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
    let expected = invalid.validate().unwrap_err();
    for error in [
        wrap(invalid.clone()).to_kvn().unwrap_err(),
        wrap(invalid.clone()).to_xml().unwrap_err(),
    ] {
        assert_eq!(error.as_validation_error(), expected.as_validation_error());
    }

    for (notation, expected) in [(Notation::Kvn, &kvn), (Notation::Xml, &xml)] {
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
    assert_rejects(invalid, "OBJECT_NAME");
}

#[test]
fn aem_output_contract() {
    let valid = Aem::from_kvn(include_str!("../data/kvn/aem_g4.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment[0].metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, Message::Aem);
}

#[test]
fn oem_output_contract() {
    let valid = Oem::from_kvn(include_str!("../data/kvn/oem_g13.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment[0].metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, Message::Oem);
}

#[test]
fn opm_output_contract() {
    let valid = Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.body.segment.metadata.object_name.clear();
    assert_output_contract(&valid, &invalid, Message::Opm);
}

#[test]
fn file_conversion_preserves_destinations_on_failure_for_each_message() {
    for input in [
        include_str!("../data/kvn/aem_g4.kvn"),
        include_str!("../data/kvn/oem_g11.kvn"),
        include_str!("../data/kvn/opm_g1.kvn"),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.kvn");
        let destination = directory.path().join("destination.xml");
        std::fs::write(&source, input).unwrap();
        convert_file(&source, &destination, Notation::Xml).unwrap();
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            convert(input, Notation::Xml).unwrap()
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = |path: &std::path::Path| {
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777
            };
            assert_eq!(mode(&destination), mode(&source));
        }

        // An existing destination is replaced after a successful conversion.
        convert_file(&source, &destination, Notation::Xml).unwrap();
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            convert(input, Notation::Xml).unwrap()
        );

        std::fs::write(&source, "invalid message").unwrap();
        std::fs::write(&destination, b"sentinel").unwrap();
        let error = convert_file(&source, &destination, Notation::Xml).unwrap_err();
        assert!(
            error.to_string().contains("Could not identify KVN header"),
            "unexpected diagnostic: {error}"
        );
        assert_eq!(std::fs::read(&destination).unwrap(), b"sentinel");
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            2,
            "conversion left temporary files"
        );
    }
}

#[test]
fn xml_generation_rejects_forbidden_xml_1_characters_before_streaming() {
    let mut cdm = Cdm::from_kvn(include_str!("../data/kvn/cdm_362.kvn")).unwrap();
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
    let mut apm = Apm::from_kvn(include_str!("../data/kvn/apm_g1.kvn")).unwrap();
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
    let mut tdm = Tdm::from_kvn(include_str!("../data/kvn/tdm_e1.kvn")).unwrap();
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
    let message = Message::Oem(Oem::from_kvn(include_str!("../data/kvn/oem_g11.kvn")).unwrap());
    let directory = tempfile::tempdir().unwrap();

    for error in [
        message.to_kvn_file(directory.path()).unwrap_err(),
        message.to_xml_file(directory.path()).unwrap_err(),
    ] {
        let diagnostic = error.diagnostic().expect("file error should have context");
        assert_eq!(diagnostic.message_kind.as_str(), "OEM");
        assert_eq!(diagnostic.source_edition, Some("3.0"));
    }
}

/// Every family must reach the same bytes through the buffered and streaming entry points.
#[test]
fn streaming_generation_matches_buffered_output_for_every_family() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for fixture in [
        "kvn/acm_g6.kvn",
        "kvn/aem_g4.kvn",
        "kvn/apm_g1.kvn",
        "kvn/cdm_362.kvn",
        "kvn/ocm_g15.kvn",
        "kvn/oem_g11.kvn",
        "kvn/omm_g7.kvn",
        "kvn/opm_g1.kvn",
        "kvn/rdm_c1.kvn",
        "kvn/tdm_e1.kvn",
        "xml/ndm_g12.xml",
    ] {
        let message = ccsds_ndm::from_file(root.join("data").join(fixture)).unwrap();

        let mut xml = Vec::new();
        message.write_xml_to(&mut xml).unwrap();
        assert_eq!(String::from_utf8(xml).unwrap(), message.to_xml().unwrap());

        let mut kvn = Vec::new();
        let streamed = message.write_kvn_to(&mut kvn);
        match message {
            // Combined NDM has no KVN representation; both paths must refuse it.
            Message::Ndm(_) => {
                assert_eq!(streamed.unwrap_err().code(), Some("unsupported.notation"));
                assert_eq!(
                    message.to_kvn().unwrap_err().code(),
                    Some("unsupported.notation")
                );
                assert!(kvn.is_empty(), "{fixture}");
            }
            _ => {
                streamed.unwrap();
                assert_eq!(String::from_utf8(kvn).unwrap(), message.to_kvn().unwrap());
            }
        }
    }
}
