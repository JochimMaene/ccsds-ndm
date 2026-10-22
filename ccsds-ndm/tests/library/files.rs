// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::CcsdsNdmError;
use std::fs;

use ccsds_ndm::{from_file, from_file_with_notation, from_str, from_str_with_notation, Notation};

#[test]
fn from_file_preserves_kvn_and_xml_models() {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("opm.kvn", include_str!("../../data/kvn/opm_g1.kvn")),
        ("opm.xml", include_str!("../../data/xml/opm_g5.xml")),
    ] {
        let path = directory.path().join(name);
        fs::write(&path, source).unwrap();
        assert_eq!(
            from_file(&path).unwrap(),
            from_str(source).unwrap(),
            "{name}"
        );
        for notation in [None, Some(Notation::Kvn), Some(Notation::Xml)] {
            match from_str_with_notation(source, notation) {
                Ok(expected) => assert_eq!(
                    from_file_with_notation(&path, notation).unwrap(),
                    expected,
                    "{name} {notation:?}"
                ),
                Err(expected) => {
                    let error = from_file_with_notation(&path, notation).unwrap_err();
                    assert_eq!(error.code(), expected.code(), "{name} {notation:?}");
                    assert_eq!(
                        error.to_string(),
                        expected.to_string(),
                        "{name} {notation:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn from_file_reports_invalid_utf8_as_an_io_error() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("invalid.kvn");
    std::fs::write(&path, b"CCSDS_OPM_VERS = 3.0\n\xff").unwrap();
    let error = from_file(&path).unwrap_err();
    assert!(
        matches!(error, CcsdsNdmError::Io(ref source)
            if source.kind() == std::io::ErrorKind::InvalidData),
        "expected an encoding error, got {error}"
    );
}

#[test]
fn from_file_refuses_utf16_and_utf32_with_a_parse_error() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("wide.xml");
    let text = "<?xml version=\"1.0\"?><opm/>";
    let utf16_le: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let utf16_be: Vec<u8> = text.encode_utf16().flat_map(u16::to_be_bytes).collect();
    let utf32_be: Vec<u8> = text
        .chars()
        .flat_map(|c| u32::from(c).to_be_bytes())
        .collect();
    for (label, bytes) in [
        ("UTF-16LE", utf16_le.clone()),
        ("UTF-16LE BOM", [&[0xff, 0xfe][..], &utf16_le].concat()),
        ("UTF-16BE BOM", [&[0xfe, 0xff][..], &utf16_be].concat()),
        ("UTF-32BE", utf32_be),
    ] {
        std::fs::write(&path, bytes).unwrap();
        let error = from_file(&path).unwrap_err();
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{label}: {error}");
        assert!(
            error.to_string().contains("input must be UTF-8"),
            "{label}: {error}"
        );
    }
}

#[test]
fn from_file_reports_missing_files() {
    let directory = tempfile::tempdir().unwrap();
    let error = from_file(directory.path().join("missing.kvn")).unwrap_err();
    assert!(
        matches!(
            error,
            CcsdsNdmError::Io(ref source) if source.kind() == std::io::ErrorKind::NotFound
        ),
        "expected a missing-file error, got {error}"
    );
}
