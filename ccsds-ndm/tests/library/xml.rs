// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::messages::cdm::Cdm;

use crate::common::{mutated, mutated_once};
use ccsds_ndm::{from_str, Ndm};

#[test]
fn standalone_xml_roots_reject_unknown_attributes_and_metadata() {
    let acm = from_str(include_str!("../../data/kvn/acm_g6.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let cases = [
        ("omm", include_str!("../../data/xml/omm_g10.xml")),
        ("ocm", include_str!("../../data/xml/ocm_g20.xml")),
        ("cdm", include_str!("../../data/xml/cdm_44.xml")),
        ("tdm", include_str!("../../data/xml/tdm_e21.xml")),
        ("rdm", include_str!("../../data/xml/rdm_c3.xml")),
        ("aem", include_str!("../../data/xml/aem_g11.xml")),
        ("apm", include_str!("../../data/xml/apm_g10.xml")),
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
        let invalid = mutated_once(input, "<metadata>", "<metadata><UNKNOWN/>");
        let error = from_str(&invalid).unwrap_err();
        assert_eq!(error.code(), Some("parse.xml.syntax"), "{root}: {error}");
    }
}

#[test]
fn standalone_xml_roots_reject_trailing_documents() {
    let input = include_str!("../../data/xml/omm_g10.xml");
    let error = from_str(&format!("{input}<omm/>")).unwrap_err();
    assert!(matches!(
        error.diagnostic().unwrap().message_kind,
        ccsds_ndm::validation::MessageKind::Omm
    ));
    assert_eq!(error.code(), Some("parse.xml.syntax"));
}

#[test]
fn strict_xml_rejects_forbidden_xml_1_characters() {
    let xml = mutated(
        include_str!("../../data/xml/cdm_44.xml"),
        "JSPOC",
        "JS\u{1}POC",
    );
    let error = Cdm::from_xml(&xml).expect_err("U+0001 accepted in an XML document");
    assert!(
        error.to_string().contains("only XML 1.0 characters"),
        "unexpected diagnostic: {error}"
    );
}

#[test]
fn xml_rejects_mixed_content_and_invalid_references_without_losing_valid_text() {
    for input in [
        include_str!("../../data/xml/oem_g14.xml"),
        include_str!("../../data/xml/opm_g5.xml"),
        CDM,
    ] {
        for value in ["&unknown;", "&#1;", "<invalid"] {
            let root = input.rfind(" version=").unwrap();
            let mut invalid = input.to_owned();
            invalid.insert_str(root, &format!(" xsi:schemaLocation=\"{value}\""));
            assert_eq!(
                from_str(&invalid).unwrap_err().code(),
                Some("parse.xml.syntax")
            );
        }
        for text in ["unexpected", "<![CDATA[unexpected]]>", "&#65;", "&amp;"] {
            for marker in ["<header>", "</header>"] {
                let invalid = mutated_once(input, marker, &format!("{text}{marker}"));
                assert_eq!(
                    from_str(&invalid).unwrap_err().code(),
                    Some("parse.xml.syntax")
                );
            }
        }
        // XSD permits whitespace character information items regardless of their spelling.
        // libxml2 rejects whitespace CDATA here, contrary to XSD 1.0 3.4.4(2.3).
        let expected = from_str(input).unwrap();
        for whitespace in ["&#32;", "&#x9;&#10;&#13;", "<![CDATA[ \t\n]]>"] {
            for marker in ["<header>", "</header>", "</body>"] {
                let valid = mutated_once(input, marker, &format!("{whitespace}{marker}"));
                assert_eq!(from_str(&valid).unwrap(), expected);
            }
        }
        for reference in ["&#1;", "&#xFFFF;", "&unknown;"] {
            let invalid = mutated_once(input, "<ORIGINATOR>", &format!("<ORIGINATOR>{reference}"));
            assert_eq!(
                from_str(&invalid).unwrap_err().code(),
                Some("parse.xml.syntax")
            );
        }
        for suffix in ["&#32;", "&amp;", "\u{a0}", "<!-- invalid -- comment -->"] {
            assert!(
                from_str(&format!("{input}{suffix}")).is_err(),
                "accepted {suffix}"
            );
        }
        let valid = mutated_once(
            input,
            "<ORIGINATOR>",
            "<ORIGINATOR><![CDATA[A & B]]>&amp;&#65;",
        );
        let parsed = from_str(&valid).unwrap();
        assert!(parsed.to_xml().unwrap().contains("A &amp; B&amp;A"));
    }
}

const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

const CDM: &str = include_str!("../../data/xml/cdm_44.xml");

/// Wrap the fixture's root element in `wrapper`, keeping the XML declaration
/// first so the document stays well-formed and only the nesting is at fault.
fn wrapped_in(wrapper: &str) -> String {
    let cdm = CDM;
    let body = cdm.split_once("?>").map_or(cdm, |(_, rest)| rest).trim();
    format!("{XML_DECL}\n<{wrapper}>\n{body}\n</{wrapper}>\n")
}

/// Detection reports the bad root as `UnsupportedMessage`, which `from_str`
/// then wraps in parse context. Look through that wrapper so the assertion
/// pins the cause rather than the layer it arrives in.
fn unsupported_message_detail(error: &CcsdsNdmError) -> Option<&str> {
    match error {
        CcsdsNdmError::UnsupportedMessage(detail) => Some(detail),
        CcsdsNdmError::Parsing { source, .. } => unsupported_message_detail(source),
        _ => None,
    }
}

#[test]
fn wrapped_message_is_rejected_by_root_detection() {
    from_str(CDM).expect("unwrapped fixture must parse");
    for wrapper in ["message", "somethingExtra", "response"] {
        let error = from_str(&wrapped_in(wrapper))
            .expect_err(&format!("<{wrapper}> wrapper must be rejected"));

        let detail = unsupported_message_detail(&error).unwrap_or_else(|| {
            panic!("expected UnsupportedMessage for <{wrapper}>, got {error:?}")
        });
        assert!(
            detail.contains(wrapper),
            "diagnostic for <{wrapper}> should name the offending root, got: {detail}"
        );
    }
}
