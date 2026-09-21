// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

mod common;
use crate::common::{mutated, mutated_once};
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::{from_str, Message, Ndm};

#[test]
fn standalone_xml_roots_allow_schema_hints_but_reject_unknown_attributes() {
    let acm = from_str(include_str!("../data/kvn/acm_g6.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let cases = [
        ("omm", include_str!("../data/xml/omm_g10.xml")),
        ("ocm", include_str!("../data/xml/ocm_g20.xml")),
        ("cdm", include_str!("../data/xml/cdm_44.xml")),
        ("tdm", include_str!("../data/xml/tdm_e21.xml")),
        ("rdm", include_str!("../data/xml/rdm_c3.xml")),
        ("aem", include_str!("../data/xml/aem_g11.xml")),
        ("apm", include_str!("../data/xml/apm_g10.xml")),
        ("acm", acm.as_str()),
    ];

    for (root, input) in cases {
        from_str(input).unwrap_or_else(|error| panic!("valid {root} fixture: {error}"));
        let invalid = mutated_once(
            input,
            &format!("<{root}"),
            &format!("<{root} unexpected=\"value\""),
        );
        let error = from_str(&invalid).expect_err("unknown root attribute must be rejected");
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{root}: {error}");
    }
}

#[test]
fn standalone_xml_roots_reject_trailing_documents() {
    let input = include_str!("../data/xml/omm_g10.xml");
    let error = from_str(&format!("{input}<omm/>")).unwrap_err();
    assert!(matches!(
        error.diagnostic().unwrap().message_kind,
        ccsds_ndm::validation::MessageKind::Omm
    ));
    assert_eq!(error.code(), Some("parse.xml.syntax"));
}

#[test]
fn standalone_xml_metadata_rejects_unknown_elements() {
    let acm = from_str(include_str!("../data/kvn/acm_g6.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let cases = [
        ("omm", include_str!("../data/xml/omm_g10.xml")),
        ("ocm", include_str!("../data/xml/ocm_g20.xml")),
        ("cdm", include_str!("../data/xml/cdm_44.xml")),
        ("tdm", include_str!("../data/xml/tdm_e21.xml")),
        ("rdm", include_str!("../data/xml/rdm_c3.xml")),
        ("aem", include_str!("../data/xml/aem_g11.xml")),
        ("apm", include_str!("../data/xml/apm_g10.xml")),
        ("acm", acm.as_str()),
    ];
    for (root, input) in cases {
        let invalid = mutated_once(input, "<metadata>", "<metadata><UNKNOWN/>");
        let error = from_str(&invalid).expect_err("unknown metadata element must be rejected");
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{root}: {error}");
    }
}

#[test]
fn generated_acm_is_still_detected_as_acm() {
    let message = from_str(include_str!("../data/kvn/acm_g6.kvn")).unwrap();
    assert!(matches!(message, Message::Acm(_)));
}

#[test]
fn strict_xml_rejects_forbidden_xml_1_characters() {
    let xml = mutated(
        include_str!("../data/xml/cdm_44.xml"),
        "JSPOC",
        "JS\u{1}POC",
    );
    let error = Cdm::from_xml(&xml).expect_err("U+0001 accepted in an XML document");
    assert!(
        error.to_string().contains("only XML 1.0 characters"),
        "unexpected diagnostic: {error}"
    );
}
