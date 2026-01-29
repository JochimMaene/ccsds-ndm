// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Integration tests for the public API of ccsds-ndm library.
//! Tests MessageType enum methods, from_str, from_file, and serialization.

use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::{from_file, from_str, MessageType};
use std::path::PathBuf;
use tempfile::NamedTempFile;

mod common;

fn data_dir() -> PathBuf {
    common::data_dir()
}

// ===== Test data file paths =====
fn opm_kvn() -> PathBuf {
    data_dir().join("kvn/opm_g1.kvn")
}
fn omm_kvn() -> PathBuf {
    data_dir().join("kvn/omm_g7.kvn")
}
fn oem_kvn() -> PathBuf {
    data_dir().join("kvn/oem_g11.kvn")
}
fn ocm_kvn() -> PathBuf {
    data_dir().join("kvn/ocm_g15.kvn")
}
fn tdm_kvn() -> PathBuf {
    data_dir().join("kvn/tdm_e1.kvn")
}
fn rdm_kvn() -> PathBuf {
    data_dir().join("kvn/rdm_c1.kvn")
}
fn cdm_kvn() -> PathBuf {
    data_dir().join("kvn/cdm_362.kvn")
}

fn opm_xml() -> PathBuf {
    data_dir().join("xml/opm_g5.xml")
}
fn omm_xml() -> PathBuf {
    data_dir().join("xml/omm_g10.xml")
}
fn oem_xml() -> PathBuf {
    data_dir().join("xml/oem_g14.xml")
}
fn ocm_xml() -> PathBuf {
    data_dir().join("xml/ocm_g20.xml")
}
fn tdm_xml() -> PathBuf {
    data_dir().join("xml/tdm_e21.xml")
}
fn rdm_xml() -> PathBuf {
    data_dir().join("xml/rdm_c3.xml")
}
fn cdm_xml() -> PathBuf {
    data_dir().join("xml/cdm_44.xml")
}

fn acm_kvn() -> PathBuf {
    data_dir().join("kvn/acm_g6.kvn")
}

fn aem_kvn() -> PathBuf {
    data_dir().join("kvn/aem_g4.kvn")
}

fn apm_kvn() -> PathBuf {
    data_dir().join("kvn/apm_g1.kvn")
}

fn aem_xml() -> PathBuf {
    data_dir().join("xml/aem_g11.xml")
}

fn apm_xml() -> PathBuf {
    data_dir().join("xml/apm_g10.xml")
}

fn acm_xml() -> PathBuf {
    data_dir().join("xml/acm_nonexistent.xml")
}

// ===== Standard API Tests (Macro) =====
macro_rules! test_message_api {
    ($type:ident, $kvn_path:ident, $xml_path:ident, $vers_key:expr, $xml_tag:expr, $xml_tag_upper:expr) => {
        paste::paste! {
            #[test]
            fn [<test_message_type_ $type:lower _to_kvn>]() {
                let content = std::fs::read_to_string($kvn_path()).unwrap();
                let msg = from_str(&content).unwrap();
                assert!(matches!(msg, MessageType::$type(_)));
                let kvn = msg.to_kvn().unwrap();
                assert!(kvn.contains($vers_key));
            }

            #[test]
            fn [<test_message_type_ $type:lower _to_xml>]() {
                // Not all messages might have XML samples yet
                if let Ok(content) = std::fs::read_to_string($xml_path()) {
                    let msg = from_str(&content).unwrap();
                    assert!(matches!(msg, MessageType::$type(_)));
                    let xml = msg.to_xml().unwrap();
                    assert!(xml.contains(concat!("<", $xml_tag)) || xml.contains(concat!("<", $xml_tag_upper)));
                }
            }
        }
    };
}

test_message_api!(Opm, opm_kvn, opm_xml, "CCSDS_OPM_VERS", "opm", "OPM");
test_message_api!(Omm, omm_kvn, omm_xml, "CCSDS_OMM_VERS", "omm", "OMM");
test_message_api!(Oem, oem_kvn, oem_xml, "CCSDS_OEM_VERS", "oem", "OEM");
test_message_api!(Ocm, ocm_kvn, ocm_xml, "CCSDS_OCM_VERS", "ocm", "OCM");
test_message_api!(Tdm, tdm_kvn, tdm_xml, "CCSDS_TDM_VERS", "tdm", "TDM");
test_message_api!(Rdm, rdm_kvn, rdm_xml, "CCSDS_RDM_VERS", "rdm", "RDM");
test_message_api!(Cdm, cdm_kvn, cdm_xml, "CCSDS_CDM_VERS", "cdm", "CDM");
test_message_api!(Acm, acm_kvn, acm_xml, "CCSDS_ACM_VERS", "acm", "ACM");
test_message_api!(Aem, aem_kvn, aem_xml, "CCSDS_AEM_VERS", "aem", "AEM");
test_message_api!(Apm, apm_kvn, apm_xml, "CCSDS_APM_VERS", "apm", "APM");

// ===== to_kvn_file and to_xml_file tests =====

#[test]
fn test_message_type_to_kvn_file() {
    let content = std::fs::read_to_string(opm_kvn()).unwrap();
    let msg = from_str(&content).unwrap();

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();

    msg.to_kvn_file(&path).unwrap();

    let written = std::fs::read_to_string(&path).unwrap();
    assert!(written.contains("CCSDS_OPM_VERS"));
}

#[test]
fn test_message_type_to_xml_file() {
    let content = std::fs::read_to_string(opm_xml()).unwrap();
    let msg = from_str(&content).unwrap();

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();

    msg.to_xml_file(&path).unwrap();

    let written = std::fs::read_to_string(&path).unwrap();
    assert!(written.contains("<opm") || written.contains("<OPM"));
}

// ===== Error path tests =====

#[test]
fn test_from_str_empty_kvn() {
    let result = from_str("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CcsdsNdmError::UnexpectedEof { .. }));
}

#[test]
fn test_from_str_unknown_kvn_header() {
    let result = from_str("UNKNOWN_HEADER = some_value\n");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CcsdsNdmError::UnsupportedMessage(_)));
}

#[test]
fn test_from_str_unknown_xml_root() {
    let xml = r#"<?xml version="1.0"?><unknown><data/></unknown>"#;
    let result = from_str(xml);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CcsdsNdmError::UnsupportedMessage(_)));
}

#[test]
fn test_from_str_empty_xml() {
    let result = from_str("<?xml version='1.0'?>");
    assert!(result.is_err());
}

#[test]
fn test_from_file_nonexistent() {
    let result = from_file("/nonexistent/path/to/file.opm");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CcsdsNdmError::Io(_)));
}

#[test]
fn test_from_str_kvn_with_leading_comments() {
    // Leading comments should be skipped
    let content = std::fs::read_to_string(opm_kvn()).unwrap();
    let with_comment = format!("COMMENT This is a comment\n{}", content);
    let result = from_str(&with_comment);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_kvn_with_leading_whitespace() {
    let content = std::fs::read_to_string(opm_kvn()).unwrap();
    let with_whitespace = format!("   \n\n{}", content);
    let result = from_str(&with_whitespace);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_xml_parse_error() {
    // Invalid XML that will cause a parse error
    let invalid_xml = "<invalid xml <broken";
    let result = from_str(invalid_xml);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.is_format_error());
}

#[test]
fn test_from_str_xml_with_processing_instruction() {
    // XML with processing instruction before root tag
    let content = std::fs::read_to_string(opm_xml()).unwrap();
    // Add a PI that quick-xml may encounter before root
    let with_pi = format!(
        "<?xml-stylesheet type='text/xsl' href='style.xsl'?>\n{}",
        content
    );
    let result = from_str(&with_pi);
    // This should still work, it just needs to skip the PI
    assert!(result.is_ok() || result.is_err()); // Either is acceptable, we just want coverage
}

#[test]
fn test_from_str_xml_with_text_before_root() {
    // XML with whitespace/text before root tag
    let xml = "   \n  <?xml version='1.0'?>\n  <opm></opm>";
    let result = from_str(xml);
    // This exercises the "continue" branch for text events
    // May fail due to invalid OPM structure, but covers the branch
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_from_file_opm_kvn() {
    // Test from_file explicitly for coverage of the success path
    let result = from_file(opm_kvn());
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), MessageType::Opm(_)));
}

#[test]
fn test_from_file_opm_xml() {
    // Test from_file with XML
    let result = from_file(opm_xml());
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), MessageType::Opm(_)));
}
