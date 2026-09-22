use crate::common::mutated;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;

const KVN: &str = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = STATION
PARTICIPANT_2 = SPACECRAFT
PATH_1 = 1,2
PATH_2 = 2,1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 100
DATA_STOP
"#;

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<tdm id="CCSDS_TDM_VERS" version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <PARTICIPANT_1>STATION</PARTICIPANT_1>
        <PARTICIPANT_2>SPACECRAFT</PARTICIPANT_2>
        <PATH_1>1,2</PATH_1>
        <PATH_2>2,1</PATH_2>
      </metadata>
      <data>
        <observation>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <RANGE>100</RANGE>
        </observation>
      </data>
    </segment>
  </body>
</tdm>
"#;

#[test]
fn minimal_kvn_parses() {
    Tdm::from_kvn(KVN).unwrap();
}

#[test]
fn minimal_xml_parses() {
    Tdm::from_xml(XML).unwrap();
}

#[test]
fn rejects_missing_time_system_xml() {
    let input = mutated(XML, "<TIME_SYSTEM>UTC</TIME_SYSTEM>", "");
    let error = Tdm::from_xml(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.xml.syntax"), "{error}");
}

#[test]
fn rejects_wrong_keyword_kvn() {
    let input = mutated(KVN, "TIME_SYSTEM", "BAD_KEY");
    let error = Tdm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_number_format_kvn() {
    let input = mutated(
        KVN,
        "RANGE = 2023-01-01T00:00:00 100",
        "RANGE = 2023-01-01T00:00:00 BAD",
    );
    let error = Tdm::from_kvn(&input).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{error}");
}

#[test]
fn rejects_missing_second_path_kvn() {
    let input = mutated(KVN, "PATH_2 = 2,1\n", "");
    let error = Tdm::from_kvn(&input).unwrap_err();
    assert!(
        matches!(error.as_validation_error(), Some(ccsds_ndm::error::ValidationError::Generic { message, .. }) if message.contains("PATH_1 and PATH_2")),
        "{error}"
    );
}
