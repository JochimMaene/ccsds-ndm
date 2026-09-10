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
    assert!(wrap(invalid.clone()).to_kvn().is_err());
    assert!(wrap(invalid.clone()).to_xml().is_err());

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
        assert!(convert_file(&source, &destination, Notation::Xml).is_err());
        assert_eq!(std::fs::read(&destination).unwrap(), b"sentinel");
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            2,
            "conversion left temporary files"
        );
    }
}
