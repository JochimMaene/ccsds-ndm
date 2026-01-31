// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::{from_file, from_str, MessageType};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_from_file_kvn_and_xml_minimal() {
    let kvn = OPM_MINIMAL_KVN;
    let xml = OPM_MINIMAL_XML;

    let kvn_path = write_temp_file("opm_minimal.kvn", kvn);
    let xml_path = write_temp_file("opm_minimal.xml", xml);

    let kvn_msg = from_file(&kvn_path).expect("OPM KVN should parse");
    assert!(matches!(kvn_msg, MessageType::Opm(_)));

    let xml_msg = from_file(&xml_path).expect("OPM XML should parse");
    assert!(matches!(xml_msg, MessageType::Opm(_)));
}

#[test]
fn test_from_file_nonexistent() {
    let err = from_file("/tmp/ccsds-ndm-nonexistent.kvn").unwrap_err();
    match err {
        CcsdsNdmError::Io(_) => {}
        other => panic!("Expected IO error, got {other:?}"),
    }
}

#[test]
fn test_from_str_roundtrip_minimal() {
    let msg = from_str(OPM_MINIMAL_KVN).expect("OPM KVN should parse");
    let kvn = msg.to_kvn().expect("KVN serialize");
    let xml = msg.to_xml().expect("XML serialize");

    assert!(from_str(&kvn).is_ok());
    assert!(from_str(&xml).is_ok());
}

fn write_temp_file(name: &str, content: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let pid = std::process::id();
    let file_name = format!("ccsds_ndm_{pid}_{name}");
    path.push(file_name);
    fs::write(&path, content).expect("write temp file");
    path
}

const OPM_MINIMAL_KVN: &str = r#"CCSDS_OPM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1 [km]
Y = 2 [km]
Z = 3 [km]
X_DOT = 4 [km/s]
Y_DOT = 5 [km/s]
Z_DOT = 6 [km/s]
"#;

const OPM_MINIMAL_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<opm id="CCSDS_OPM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <OBJECT_NAME>TEST</OBJECT_NAME>
        <OBJECT_ID>2020-001A</OBJECT_ID>
        <CENTER_NAME>EARTH</CENTER_NAME>
        <REF_FRAME>GCRF</REF_FRAME>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
      </metadata>
      <data>
        <stateVector>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <X units="km">1</X>
          <Y units="km">2</Y>
          <Z units="km">3</Z>
          <X_DOT units="km/s">4</X_DOT>
          <Y_DOT units="km/s">5</Y_DOT>
          <Z_DOT units="km/s">6</Z_DOT>
        </stateVector>
      </data>
    </segment>
  </body>
</opm>
"#;
