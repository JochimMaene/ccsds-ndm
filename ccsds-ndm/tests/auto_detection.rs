// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{detect::detect_notation, from_str, Message, Notation};

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");
const OPM_XML: &str = include_str!("../data/xml/opm_g5.xml");

#[test]
fn notation_detection_and_auto_parse_are_bom_safe() {
    assert_eq!(detect_notation("\u{feff}  <opm/>").unwrap(), Notation::Xml);
    assert_eq!(
        detect_notation("\u{feff}\nCCSDS_OPM_VERS = 3.0").unwrap(),
        Notation::Kvn
    );
    assert!(detect_notation("\u{feff} \r\n").is_err());

    assert!(matches!(
        from_str(&format!("\u{feff}{OPM_KVN}")).unwrap(),
        Message::Opm(_)
    ));
    assert!(matches!(
        from_str(&format!("\u{feff}{OPM_XML}")).unwrap(),
        Message::Opm(_)
    ));
}

#[test]
fn test_kvn_detection_does_not_bypass_strict_preamble_rules() {
    let input = r#"

    COMMENT This file starts with blank lines
    COMMENT And multiple comments

    CCSDS_OPM_VERS = 3.0
    CREATION_DATE = 2021-01-01T12:00:00.000
    ORIGINATOR    = NASA

    OBJECT_NAME          = SATELLITE
    OBJECT_ID            = 2020-001A
    CENTER_NAME          = EARTH
    REF_FRAME            = GCRF
    TIME_SYSTEM          = UTC

    EPOCH = 2021-01-01T12:00:00.000
    X     = 6500.0 [km]
    Y     = 0.0 [km]
    Z     = 0.0 [km]
    X_DOT = 0.0 [km/s]
    Y_DOT = 7.5 [km/s]
    Z_DOT = 0.0 [km/s]
"#;
    let error = from_str(input).expect_err("leading comments would be lost");
    assert_eq!(error.code(), Some("parse.kvn.syntax"));
}

#[test]
fn test_kvn_detection_does_not_bypass_printable_ascii_rules() {
    let input = "\r\n\tCOMMENT Tab indented\r\n\t\t\r\nCCSDS_OPM_VERS = 3.0\r\nCREATION_DATE = 2024-01-01T00:00:00\r\nORIGINATOR=X\r\nOBJECT_NAME=Y\r\nOBJECT_ID=1\r\nCENTER_NAME=EARTH\r\nREF_FRAME=GCRF\r\nTIME_SYSTEM=UTC\r\nEPOCH=2024-01-01T00:00:00\r\nX=0\r\nY=0\r\nZ=0\r\nX_DOT=0\r\nY_DOT=0\r\nZ_DOT=0\r\n";
    let error = from_str(input).expect_err("tabs are not printable ASCII");
    assert_eq!(error.code(), Some("parse.kvn.syntax"));
}

#[test]
fn test_xml_detection_does_not_bypass_declaration_placement() {
    let input = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!-- A comment before the root element -->

      <opm id="CCSDS_OPM_VERS" version="3.0">
        <header>
          <COMMENT>This is a comment</COMMENT>
          <CREATION_DATE>2010-03-12T22:31:12.000</CREATION_DATE>
          <ORIGINATOR>NASA</ORIGINATOR>
        </header>
        <body>
          <segment>
            <metadata>
              <OBJECT_NAME>OSIRIS-REX</OBJECT_NAME>
              <OBJECT_ID>2016-055A</OBJECT_ID>
              <CENTER_NAME>SUN</CENTER_NAME>
              <REF_FRAME>EME2000</REF_FRAME>
              <TIME_SYSTEM>UTC</TIME_SYSTEM>
            </metadata>
            <data>
              <stateVector>
                <EPOCH>2019-01-01T00:00:00.000Z</EPOCH>
                <X units="km"> -1.439815777703372E+08 </X>
                <Y units="km">  4.026410714752945E+07 </Y>
                <Z units="km">  1.928732688463953E+07 </Z>
                <X_DOT units="km/s"> -9.100918933256038E+00 </X_DOT>
                <Y_DOT units="km/s"> -2.628965615712169E+01 </Y_DOT>
                <Z_DOT units="km/s"> -1.144865805537552E+01 </Z_DOT>
              </stateVector>
            </data>
          </segment>
        </body>
      </opm>
"#;
    let error = from_str(input).expect_err("the XML declaration is not first");
    assert_eq!(error.code(), Some("parse.xml.syntax"));
}

#[test]
fn test_detect_failure_unknown_header() {
    let input = r#"
    COMMENT This looks like NDM but has unknown header
    CCSDS_UNKNOWN_VERS = 1.0
    key = value
    "#;
    let err = from_str(input).unwrap_err();
    assert!(format!("{}", err).contains("Could not identify KVN header"));
}

#[test]
fn kvn_header_names_inside_values_do_not_create_a_combined_message() {
    let input = OPM_KVN.replace(
        "COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED",
        "COMMENT text mentioning CCSDS_OEM_VERS is not an OEM header",
    );
    assert!(matches!(from_str(&input).unwrap(), Message::Opm(_)));
}

#[test]
fn xml_detection_rejects_nonstandard_wrappers_and_preserves_combined_identity() {
    let opm_root = &OPM_XML[OPM_XML.find("<opm").unwrap()..];
    let wrapped = format!("<response>{opm_root}</response>");
    let error = from_str(&wrapped).expect_err("unknown wrappers are not strict NDM XML");
    assert!(error.to_string().contains("unsupported XML root tag"));

    let Message::Opm(opm) = from_str(OPM_XML).unwrap() else {
        panic!("fixture should be an OPM");
    };
    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![Message::Opm(opm)],
    }
    .to_xml()
    .unwrap();
    assert!(matches!(from_str(&combined).unwrap(), Message::Ndm(_)));
}

#[test]
fn xml_detection_accepts_an_empty_combined_instantiation() {
    let Message::Ndm(message) = from_str("<ndm/>").unwrap() else {
        panic!("an empty combined instantiation should preserve its NDM identity");
    };
    assert!(message.messages.is_empty());
}
