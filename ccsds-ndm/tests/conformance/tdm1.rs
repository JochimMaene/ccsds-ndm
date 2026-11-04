// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! TDM 1.0 XML input against the NDM/XML 1.0 schema (505.0-B-1, `ndmxml-1.0-tdm-1.0.xsd`).

use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;
use std::path::Path;
use std::process::Command;

const B9: &str = include_str!("../../data/legacy/tdm_1_0_b9.xml");

/// Validates an unqualified TDM 1.0 instance with the bundled NDM/XML 1.0 schema.
fn tdm_1_0_schema_accepts(xml: &str) -> bool {
    let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd/ndmxml-1.0-tdm-1.0.xsd");
    let wrapper = tempfile::NamedTempFile::new().expect("temporary wrapper schema");
    std::fs::write(
        wrapper.path(),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:ndm="urn:ccsds:recommendation:navigation:schema:ndmxml">
  <xsd:import namespace="urn:ccsds:recommendation:navigation:schema:ndmxml" schemaLocation="{}"/>
  <xsd:element name="tdm" type="ndm:tdmType"/>
</xsd:schema>
"#,
            schema.display()
        ),
    )
    .expect("write wrapper schema");
    let instance = tempfile::NamedTempFile::new().expect("temporary TDM XML");
    std::fs::write(instance.path(), xml).expect("write TDM XML");
    Command::new("xmllint")
        .args(["--noout", "--schema"])
        .arg(wrapper.path())
        .arg(instance.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance tests: {error}"))
        .status
        .success()
}

#[test]
fn tdm_1_0_xml_input_follows_the_ndm_xml_1_0_schema() {
    // 505.0-B-1 figure B-9: schema-valid, read as an input-only 1.0 message.
    assert!(tdm_1_0_schema_accepts(B9));
    let message = Tdm::from_xml(B9).unwrap();
    assert_eq!(message.version, "1.0");
    assert_eq!(
        ccsds_ndm::from_str(B9).unwrap(),
        ccsds_ndm::Message::Tdm(message.clone())
    );
    for error in [message.to_xml().unwrap_err(), message.to_kvn().unwrap_err()] {
        assert_eq!(error.code(), Some("generation.unsupported_output_version"));
    }

    // The schema's closed TIME_SYSTEM list includes TOD and lowercase spellings.
    for accepted in ["TOD", "utc"] {
        let xml = B9.replace("<TIME_SYSTEM>UTC<", &format!("<TIME_SYSTEM>{accepted}<"));
        assert!(tdm_1_0_schema_accepts(&xml), "{accepted}");
        Tdm::from_xml(&xml).unwrap();
    }

    // Each rejection is also a schema rejection, so the reader matches its edition's schema.
    let rejected = [
        (
            "TCG time system",
            B9.replace("<TIME_SYSTEM>UTC<", "<TIME_SYSTEM>TCG<"),
            "TIME_SYSTEM",
        ),
        (
            "mixed-case time system",
            B9.replace("<TIME_SYSTEM>UTC<", "<TIME_SYSTEM>Utc<"),
            "TIME_SYSTEM",
        ),
        (
            "2.0-only MESSAGE_ID",
            B9.replacen(
                "<ORIGINATOR>NASA/JPL</ORIGINATOR>",
                "<ORIGINATOR>NASA/JPL</ORIGINATOR><MESSAGE_ID>M1</MESSAGE_ID>",
                1,
            ),
            "MESSAGE_ID",
        ),
        (
            "RHUMIDITY units",
            B9.replacen(
                "<RECEIVE_FREQ_1>-409.2735</RECEIVE_FREQ_1>",
                "<RHUMIDITY units=\"%\">50</RHUMIDITY>",
                1,
            ),
            "RHUMIDITY",
        ),
    ];
    for (label, xml, field) in rejected {
        assert!(
            !tdm_1_0_schema_accepts(&xml),
            "{label} should fail the 1.0 schema"
        );
        let error = Tdm::from_xml(&xml).unwrap_err();
        assert!(error.to_string().contains(field), "{label}: {error}");
    }

    // A unitless RHUMIDITY is valid 1.0 content.
    let humidity = B9.replacen(
        "<RECEIVE_FREQ_1>-409.2735</RECEIVE_FREQ_1>",
        "<RHUMIDITY>50</RHUMIDITY>",
        1,
    );
    assert!(tdm_1_0_schema_accepts(&humidity));
    Tdm::from_xml(&humidity).unwrap();

    // NDM/XML 1.0 had no qualified schema set: neither the root nor any descendant may be
    // namespaced.
    let qualified_children = B9
        .replacen("<tdm ", "<tdm xmlns:ndm=\"urn:ccsds:schema:ndmxml\" ", 1)
        .replace("<header>", "<ndm:header>")
        .replace("</header>", "</ndm:header>")
        .replace("<body>", "<ndm:body>")
        .replace("</body>", "</ndm:body>");
    assert!(!tdm_1_0_schema_accepts(&qualified_children));
    assert!(Tdm::from_xml(&qualified_children)
        .unwrap_err()
        .to_string()
        .contains("qualified"));
    let qualified = B9
        .replacen(
            "<tdm ",
            "<ndm:tdm xmlns:ndm=\"urn:ccsds:schema:ndmxml\" ",
            1,
        )
        .replace("</tdm>", "</ndm:tdm>");
    assert!(Tdm::from_xml(&qualified)
        .unwrap_err()
        .to_string()
        .contains("qualified"));
}
