// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::{from_str, MessageType};

#[test]
fn test_kvn_detect_messy_preamble() {
    let input = r#"

    COMMENT This file starts with blank lines
    COMMENT And multiple comments

    CCSDS_OPM_VERS = 2.0
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
    let msg = from_str(input).unwrap();
    assert!(matches!(msg, MessageType::Opm(_)));
}

#[test]
fn test_kvn_detect_crlf_and_tabs() {
    let input = "\r\n\tCOMMENT Tab indented\r\n\t\t\r\nCCSDS_OPM_VERS = 2.0\r\nCREATION_DATE = 2024-01-01T00:00:00\r\nORIGINATOR=X\r\nOBJECT_NAME=Y\r\nOBJECT_ID=1\r\nCENTER_NAME=EARTH\r\nREF_FRAME=GCRF\r\nTIME_SYSTEM=UTC\r\nEPOCH=2024-01-01T00:00:00\r\nX=0\r\nY=0\r\nZ=0\r\nX_DOT=0\r\nY_DOT=0\r\nZ_DOT=0\r\n";
    let msg = from_str(input).unwrap();
    assert!(matches!(msg, MessageType::Opm(_)));
}

#[test]
fn test_xml_detect_messy_preamble() {
    let input = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!-- A comment before the root element -->

      <opm id="1.0" version="2.0">
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
    let msg = from_str(input).unwrap();
    assert!(matches!(msg, MessageType::Opm(_)));
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
