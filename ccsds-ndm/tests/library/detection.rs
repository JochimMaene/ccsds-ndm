// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated;

use ccsds_ndm::{detect::detect_notation, from_str, Message, Notation};

const OPM_KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const OPM_XML: &str = include_str!("../../data/xml/opm_g5.xml");

#[test]
fn notation_detection_and_auto_parse_are_bom_safe() {
    assert_eq!(detect_notation("\u{feff}  <opm/>").unwrap(), Notation::Xml);
    assert_eq!(
        detect_notation("\u{feff}\nCCSDS_OPM_VERS = 3.0").unwrap(),
        Notation::Kvn
    );
    assert!(matches!(
        detect_notation("\u{feff} \r\n").unwrap_err(),
        ccsds_ndm::error::CcsdsNdmError::UnexpectedEof { .. }
    ));

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
fn kvn_detection_does_not_bypass_strict_preamble_rules() {
    from_str(OPM_KVN).expect("baseline fixture must parse");
    let input = format!("\nCOMMENT leading comment\n{OPM_KVN}");
    let error = from_str(&input).expect_err("leading comments would be lost");
    assert_eq!(error.code(), Some("parse.kvn.syntax"));
}

#[test]
fn kvn_detection_does_not_bypass_printable_ascii_rules() {
    let input = mutated(OPM_KVN, "ORIGINATOR", "\tORIGINATOR");
    let error = from_str(&input).expect_err("tabs are not printable ASCII");
    assert_eq!(error.code(), Some("parse.kvn.syntax"));
}

#[test]
fn xml_detection_does_not_bypass_declaration_placement() {
    from_str(OPM_XML).expect("baseline fixture must parse");
    assert!(OPM_XML.starts_with("<?xml"));
    let error = from_str(&format!("\n{OPM_XML}")).expect_err("the XML declaration is not first");
    assert_eq!(error.code(), Some("parse.xml.syntax"));
}

#[test]
fn detect_failure_unknown_header() {
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
    let input = mutated(
        OPM_KVN,
        "COMMENT GEOCENTRIC, CARTESIAN, EARTH FIXED",
        "COMMENT text mentioning CCSDS_OEM_VERS is not an OEM header",
    );
    assert!(matches!(from_str(&input).unwrap(), Message::Opm(_)));
}

#[test]
fn xml_detection_accepts_an_empty_combined_instantiation() {
    let Message::Ndm(message) = from_str("<ndm/>").unwrap() else {
        panic!("an empty combined instantiation should preserve its NDM identity");
    };
    assert!(message.messages.is_empty());
}

#[test]
fn unrecognized_kvn_input_is_rejected() {
    let error = ccsds_ndm::from_str("NOT_A_CCSDS_MESSAGE").unwrap_err();
    assert!(error.to_string().contains("Could not identify KVN header"));
}
