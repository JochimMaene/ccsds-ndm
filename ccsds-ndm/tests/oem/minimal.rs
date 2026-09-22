use crate::common::mutated;
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 2020-001A
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T00:10:00
META_STOP
2023-01-01T00:00:00 1 2 3 4 5 6
"#;

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<oem id="CCSDS_OEM_VERS" version="3.0">
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
        <START_TIME>2023-01-01T00:00:00</START_TIME>
        <STOP_TIME>2023-01-01T00:10:00</STOP_TIME>
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
</oem>
"#;

#[test]
fn minimal_kvn_parses() {
    Oem::from_kvn(KVN).unwrap();
}

#[test]
fn minimal_xml_parses() {
    Oem::from_xml(XML).unwrap();
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(KVN, "1 2 3 4 5 6", "1 2 BAD 4 5 6");
    let error = Oem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "REF_FRAME", "BAD_KEY");
    let error = Oem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_inconsistent_time_system_kvn() {
    let mut input = KVN.to_owned();
    input.push_str(
        "META_START\nOBJECT_NAME = TEST\nOBJECT_ID = 2020-001A\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = TAI\nSTART_TIME = 2023-01-01T00:10:00\nSTOP_TIME = 2023-01-01T00:20:00\nMETA_STOP\n2023-01-01T00:10:00 1 2 3 4 5 6\n",
    );
    Oem::from_kvn(&mutated(&input, "TIME_SYSTEM = TAI", "TIME_SYSTEM = UTC"))
        .expect("valid control");
    let error = Oem::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"), "{error}");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment[1].metadata.time_system")
    );
    crate::common::assert_validation_field(&error, "TIME_SYSTEM");
}
