use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated_once;
use ccsds_ndm::messages::{ndm::CombinedNdm, ocm::Ocm};
use ccsds_ndm::{from_str, Message, Ndm};

#[test]
fn constituent_error_locations_refer_to_the_original_combined_document() {
    let original = include_str!("../../data/xml/ndm_g12.xml");
    assert_eq!(
        CombinedNdm::from_xml(&format!("\u{feff}{original}")).unwrap(),
        CombinedNdm::from_xml(original).unwrap(),
    );
    let source = mutated_once(original, "<Q1>-0.005068</Q1>", "<Q1>bad</Q1>");
    let expected_offset = source.find("bad</Q1>").unwrap() + "bad</Q1>".len();
    let expected_line = source[..expected_offset]
        .bytes()
        .filter(|b| *b == b'\n')
        .count()
        + 1;
    let expected_column = expected_offset - source[..expected_offset].rfind('\n').unwrap();
    for error in [
        CombinedNdm::from_xml(&source).unwrap_err(),
        from_str(&source).unwrap_err(),
        CombinedNdm::from_xml(&format!("\u{feff}{source}")).unwrap_err(),
        from_str(&format!("\u{feff}{source}")).unwrap_err(),
    ] {
        let diagnostic = error.diagnostic().unwrap();
        assert_eq!(diagnostic.byte_offset, Some(expected_offset));
        assert_eq!(
            diagnostic.source_location,
            Some((expected_line, expected_column))
        );
        assert!(error.to_string().contains(&format!("line {expected_line}")));
        assert!(diagnostic.original_token.unwrap().contains("bad</Q1>"));
    }
}

/// Prefix the XML declaration that NDM/XML 4.2 requires on the first line.
fn declared(body: &str) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{body}")
}

#[test]
fn envelope_text_decodes_entities_cdata_and_empty_elements() {
    let source = declared(
        "<ndm><?ignored instruction?><MESSAGE_ID>A&amp;B&#32;&lt;C&gt;</MESSAGE_ID>\
         <COMMENT>first <![CDATA[<&>]]> then &quot;quoted&quot;</COMMENT>\
         <COMMENT>before<?ignored instruction?> after</COMMENT><COMMENT/></ndm>",
    );
    let message = CombinedNdm::from_xml(&source).unwrap();
    assert_eq!(message.id.as_deref(), Some("A&B <C>"));
    assert_eq!(
        message.comments,
        ["first <&> then \"quoted\"", "before after", ""]
    );
    let xml = message.to_xml().unwrap();
    crate::common::validate_xml("combined text fields", &xml);
    assert_eq!(CombinedNdm::from_xml(&xml).unwrap(), message);
    assert_eq!(from_str(&source).unwrap(), Message::Ndm(message));

    let empty = CombinedNdm::from_xml(&declared("<ndm><MESSAGE_ID/><COMMENT/></ndm>")).unwrap();
    assert_eq!(empty.id.as_deref(), Some(""));
    assert_eq!(empty.comments, [""]);
    assert_eq!(
        CombinedNdm::from_xml(&empty.to_xml().unwrap()).unwrap(),
        empty
    );
    assert!(CombinedNdm::from_xml(&declared("<ndm/><?ignored instruction?>")).is_ok());
}

#[test]
fn envelope_text_rejects_attributes_and_nested_elements() {
    for element in ["MESSAGE_ID", "COMMENT"] {
        for content in [
            format!("<{element} bad=\"x\">a</{element}>"),
            format!("<{element} bad=\"x\"/>"),
            format!("<{element}><bad>x</bad></{element}>"),
            format!("<{element}><bad/></{element}>"),
        ] {
            let source = declared(&format!("<ndm>{content}</ndm>"));
            assert!(CombinedNdm::from_xml(&source).is_err(), "{content}");
            assert!(from_str(&source).is_err(), "{content}");
        }
    }
}

#[test]
fn envelope_text_accepts_only_namespace_resolved_schema_hints() {
    for element in ["MESSAGE_ID", "COMMENT"] {
        for hint in ["schemaLocation", "noNamespaceSchemaLocation"] {
            let location = if hint == "schemaLocation" {
                "urn:ccsds:schema:ndmxml schema.xsd"
            } else {
                "schema.xsd"
            };
            for (declarations, attributes) in [
                (
                    "xmlns:hint='http://www.w3.org/2001/XMLSchema-instance'",
                    format!("hint:{hint}='{location}'"),
                ),
                (
                    "",
                    format!("xmlns:local='http://www.w3.org/2001/XMLSchema-instance' local:{hint}='{location}'"),
                ),
            ] {
                for content in [
                    format!("<{element} {attributes}>value</{element}>"),
                    format!("<{element} {attributes}/>"),
                ] {
                    let source = declared(&format!("<ndm {declarations}>{content}</ndm>"));
                    crate::common::validate_xml("combined envelope schema hints", &source);
                    let message = CombinedNdm::from_xml(&source).unwrap();
                    assert_eq!(from_str(&source).unwrap(), Message::Ndm(message.clone()));
                    assert_eq!(CombinedNdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
                }
            }
        }
        for attributes in [
            "hint:schemaLocation='urn:ccsds:schema:ndmxml schema.xsd'",
            "xmlns:hint='urn:foreign' hint:schemaLocation='urn:ccsds:schema:ndmxml schema.xsd'",
            "xmlns:hint='http://www.w3.org/2001/XMLSchema-instance' hint:unknown='value'",
            "schemaLocation='urn:ccsds:schema:ndmxml schema.xsd'",
            "xmlns:a='http://www.w3.org/2001/XMLSchema-instance' xmlns:b='http://www.w3.org/2001/XMLSchema-instance' a:schemaLocation='urn:a a.xsd' b:schemaLocation='urn:b b.xsd'",
        ] {
            let source = declared(&format!("<ndm><{element} {attributes}/></ndm>"));
            assert!(CombinedNdm::from_xml(&source).is_err(), "{source}");
            assert!(from_str(&source).is_err(), "{source}");
        }
    }
    let source = declared(
        "<ndm><COMMENT xmlns:hint='http://www.w3.org/2001/XMLSchema-instance' hint:schemaLocation='urn:ccsds:schema:ndmxml schema.xsd'/><COMMENT hint:schemaLocation='urn:ccsds:schema:ndmxml schema.xsd'/></ndm>",
    );
    assert!(
        CombinedNdm::from_xml(&source).is_err(),
        "local declarations cannot leak to siblings"
    );
}

#[test]
fn combined_xml_rejects_forbidden_literal_characters_even_in_ignored_content() {
    for character in ['\0', '\u{1}', '\u{b}', '\u{fffe}', '\u{ffff}'] {
        for body in [
            format!("<ndm><MESSAGE_ID>{character}</MESSAGE_ID></ndm>"),
            format!("<ndm><COMMENT><![CDATA[{character}]]></COMMENT></ndm>"),
            format!("<ndm><!--{character}--></ndm>"),
            format!("<ndm><?ignored {character}?></ndm>"),
            format!("<ndm xmlns:unused='urn:{character}'/>"),
            format!("<ndm/><!--{character}-->"),
        ] {
            let source = declared(&body);
            let error = CombinedNdm::from_xml(&source).unwrap_err().to_string();
            assert!(error.contains("XML 1.0 characters"), "{error}");
            assert!(from_str(&source).is_err(), "{source}");
        }
    }
}

#[test]
fn combined_xml_parser_enforces_the_normative_envelope() {
    // An unused schema-instance namespace may be omitted, as on standalone roots.
    assert!(CombinedNdm::from_xml(&declared("<ndm/>")).is_ok());
    assert!(CombinedNdm::from_xml(&declared(
        "<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'/>"
    ))
    .is_ok());
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared("<message></message>")).unwrap_err(),
        "expected standalone combined NDM root element 'ndm'",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared(
            "<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'><UNKNOWN/></ndm>",
        ))
        .unwrap_err(),
        "unexpected content in combined NDM envelope",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared("<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'><message_id>wrong case</message_id></ndm>")).unwrap_err(),
        "unknown combined NDM child <message_id>",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared("<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'><comment>wrong case</comment></ndm>")).unwrap_err(),
        "unknown combined NDM child <comment>",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared("<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'><COMMENT>first</COMMENT><MESSAGE_ID>late</MESSAGE_ID></ndm>"))
            .unwrap_err(),
        "MESSAGE_ID must occur at most once before comments and messages",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared(
            "<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'>",
        ))
        .unwrap_err(),
        "incomplete combined NDM XML document",
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared(
            "<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'></ndm><ndm></ndm>",
        ))
        .unwrap_err(),
        "trailing content after combined NDM document",
    );
    assert!(CombinedNdm::from_xml(&declared(
        "<ndm xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'></ndm>"
    ))
    .is_ok());
}

#[test]
fn combined_ndm_xml_attitude() {
    let input = include_str!("../../data/xml/ndm_g12.xml");

    let msg = from_str(input).unwrap();
    match msg {
        Message::Ndm(ndm) => {
            assert_eq!(
                ndm.comments,
                vec!["Example: 1 each APM, AEM, ACM in combined instantiation".to_string()]
            );
            assert_eq!(ndm.messages.len(), 3);
            assert!(matches!(ndm.messages[0], Message::Apm(_)));
            assert!(matches!(ndm.messages[1], Message::Aem(_)));
            assert!(matches!(ndm.messages[2], Message::Acm(_)));
        }
        _ => panic!("Expected Message::Ndm, got {:?}", msg),
    }
}

#[test]
fn combined_xml_depth_has_a_fixed_safety_limit() {
    let document = format!("{}{}", "<ndm>".repeat(17), "</ndm>".repeat(17));
    let error = CombinedNdm::from_xml(&document).unwrap_err().to_string();
    assert!(error.contains("xml_depth"), "unexpected error: {error}");
}

#[test]
fn combined_xml_rejects_illegal_root_and_constituent_attributes() {
    let source = include_str!("../../data/xml/ndm_g12.xml");
    for (label, xml) in [
        (
            "attribute 'id' is not allowed on the combined NDM root",
            mutated_once(source, "<ndm ", "<ndm id=\"not-allowed\" "),
        ),
        (
            "unknown combined NDM root attribute 'unexpected'",
            mutated_once(source, "<ndm ", "<ndm unexpected=\"value\" "),
        ),
        (
            "attribute 'unexpected' is not allowed on a combined NDM constituent",
            mutated_once(source, "<apm id=", "<apm unexpected=\"value\" id="),
        ),
        (
            "combined NDM constituents require exactly one id and version attribute",
            mutated_once(source, " version=\"2.0\"", ""),
        ),
    ] {
        crate::common::assert_invalid_format(&CombinedNdm::from_xml(&xml).unwrap_err(), label);
    }
}

#[test]
fn combined_kvn_is_reported_as_an_unsupported_notation() {
    let error = CombinedNdm::from_kvn("CCSDS_OPM_VERS = 3.0\n").unwrap_err();
    let diagnostic = error.diagnostic().unwrap();
    assert_eq!(diagnostic.code, Some("unsupported.notation"));
    assert_eq!(diagnostic.operation, DiagnosticOperation::Parse);
    assert_eq!(diagnostic.notation, DiagnosticNotation::Kvn);
    assert_eq!(
        diagnostic.message_kind,
        ccsds_ndm::validation::MessageKind::Ndm
    );
}

/// Prefix every element of an unqualified fragment with `ndm:` (NDM/XML 4.3.5).
fn qualified(fragment: &str) -> String {
    fragment
        .replace("</", "\u{0}")
        .replace('<', "<ndm:")
        .replace('\u{0}', "</ndm:")
        .replace("<ndm:!--", "<!--")
}

#[test]
fn qualified_messages_reach_their_parsers_through_generic_dispatch() {
    let oem = include_str!("../../data/xml/oem_g14.xml");
    let expected = ccsds_ndm::messages::oem::Oem::from_xml(oem).unwrap();
    let body_start = oem.find("<header>").unwrap();
    let body_end = oem.rfind("</oem>").unwrap();
    let body = qualified(&oem[body_start..body_end]);

    let standalone = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <ndm:oem xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
         xmlns:ndm=\"urn:ccsds:schema:ndmxml\" id=\"CCSDS_OEM_VERS\" version=\"3.0\">\
         {body}</ndm:oem>"
    );
    assert_eq!(
        from_str(&standalone).unwrap(),
        Message::Oem(expected.clone())
    );

    // Constituents inherit the root namespace scope, including a single-quoted declaration
    // whose value contains a double quote.
    let combined = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <ndm:ndm xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
         xmlns:ndm=\"urn:ccsds:schema:ndmxml\" xmlns:q='urn:example:\"quoted\"'>\
         <ndm:oem id=\"CCSDS_OEM_VERS\" version=\"3.0\">{body}</ndm:oem></ndm:ndm>"
    );
    let Message::Ndm(message) = from_str(&combined).unwrap() else {
        panic!("a qualified combined instantiation should stay combined");
    };
    assert_eq!(message.messages, vec![Message::Oem(expected)]);
}

#[test]
fn constituents_carry_only_id_and_version_unlike_standalone_messages() {
    // ODM 8.12.7 restricts constituent attributes to id and version; a standalone root may
    // carry schema-location hints (NDM/XML 4.3.6).
    let oem = include_str!("../../data/xml/oem_g14.xml");
    ccsds_ndm::messages::oem::Oem::from_xml(oem).expect("standalone OEM with a schema hint");
    let body_start = oem.find("<header>").unwrap();
    let body_end = oem.rfind("</oem>").unwrap();
    let combined = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <ndm xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\
         <oem xsi:noNamespaceSchemaLocation=\"oem.xsd\" id=\"CCSDS_OEM_VERS\" version=\"3.0\">\
         {}</oem></ndm>",
        &oem[body_start..body_end]
    );
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&combined).unwrap_err(),
        "attribute 'xsi:noNamespaceSchemaLocation' is not allowed on a combined NDM constituent",
    );
}

#[test]
fn constituent_edition_check_compares_decoded_xml_attribute_values() {
    let source = include_str!("../../data/xml/ndm_g21.xml");
    let expected = CombinedNdm::from_xml(source).unwrap();
    let escaped = source.replacen("version=\"3.0\"", "version=\"&#51;.0\"", 1);
    assert!(escaped.contains("version=\"&#51;.0\""));
    assert_eq!(CombinedNdm::from_xml(&escaped).unwrap(), expected);
}

#[test]
fn envelope_accepts_decoded_whitespace_only_inside_the_root() {
    let expected = CombinedNdm::from_xml("<ndm><COMMENT>ok</COMMENT></ndm>").unwrap();
    for whitespace in ["&#32;", "&#x9;&#10;&#13;", "<![CDATA[ \t\r\n]]>"] {
        let xml = format!("<ndm>{whitespace}<COMMENT>ok</COMMENT>{whitespace}</ndm> \t\r\n");
        assert_eq!(CombinedNdm::from_xml(&xml).unwrap(), expected);
        assert_eq!(from_str(&xml).unwrap(), Message::Ndm(expected.clone()));
        for root in ["<ndm/>", "<ndm></ndm>"] {
            assert!(CombinedNdm::from_xml(&format!("{root}{whitespace}")).is_err());
        }
    }
    for text in ["&#65;", "&amp;", "<![CDATA[ x ]]>"] {
        assert!(CombinedNdm::from_xml(&format!("<ndm>{text}</ndm>")).is_err());
    }
}

/// Space-Track-shaped NDM/XML 1.0 envelope: no-namespace root, schema hint, ODM 2.0 child.
const SPACE_TRACK_NDM_1_0: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
     xsi:noNamespaceSchemaLocation="http://sanaregistry.org/r/ndmxml/ndmxml-1.0-master.xsd">
    <omm id="CCSDS_OMM_VERS" version="2.0">
        <header>
            <COMMENT>GENERATED VIA SPACE-TRACK.ORG API</COMMENT>
            <CREATION_DATE>2020-12-29T06:26:10</CREATION_DATE>
            <ORIGINATOR>18 SPCS</ORIGINATOR>
        </header>
        <body>
            <segment>
                <metadata>
                    <OBJECT_NAME>NUSAT-8 (MARIE)</OBJECT_NAME>
                    <OBJECT_ID>2020-003C</OBJECT_ID>
                    <CENTER_NAME>EARTH</CENTER_NAME>
                    <REF_FRAME>TEME</REF_FRAME>
                    <TIME_SYSTEM>UTC</TIME_SYSTEM>
                    <MEAN_ELEMENT_THEORY>SGP4</MEAN_ELEMENT_THEORY>
                </metadata>
                <data>
                    <meanElements>
                        <EPOCH>2020-12-29T03:57:59.406624</EPOCH>
                        <MEAN_MOTION>15.27989249</MEAN_MOTION>
                        <ECCENTRICITY>0.00133560</ECCENTRICITY>
                        <INCLINATION>97.2970</INCLINATION>
                        <RA_OF_ASC_NODE>66.4161</RA_OF_ASC_NODE>
                        <ARG_OF_PERICENTER>110.6345</ARG_OF_PERICENTER>
                        <MEAN_ANOMALY>334.7107</MEAN_ANOMALY>
                    </meanElements>
                    <tleParameters>
                        <EPHEMERIS_TYPE>0</EPHEMERIS_TYPE>
                        <CLASSIFICATION_TYPE>U</CLASSIFICATION_TYPE>
                        <NORAD_CAT_ID>45018</NORAD_CAT_ID>
                        <ELEMENT_SET_NO>999</ELEMENT_SET_NO>
                        <REV_AT_EPOCH>5327</REV_AT_EPOCH>
                        <BSTAR>0.00008455300000</BSTAR>
                        <MEAN_MOTION_DOT>0.00002241</MEAN_MOTION_DOT>
                        <MEAN_MOTION_DDOT>0.0000000000000</MEAN_MOTION_DDOT>
                    </tleParameters>
                </data>
            </segment>
        </body>
    </omm>
</ndm>
"#;

#[test]
fn ndm_xml_1_0_envelope_is_input_only_and_checked_against_its_own_schema() {
    let combined = CombinedNdm::from_xml(SPACE_TRACK_NDM_1_0).unwrap();
    let Message::Omm(omm) = &combined.messages[0] else {
        panic!("expected OMM")
    };
    assert_eq!(omm.version, "2.0");
    assert_eq!(
        from_str(SPACE_TRACK_NDM_1_0).unwrap(),
        Message::Ndm(combined.clone())
    );
    // Output stays scoped to the NDM/XML 4.0.0 envelope, which cannot carry ODM 2.0.
    ccsds_ndm::Validate::validate(&combined).unwrap();
    assert!(combined.to_xml().is_err());
    let mut sink = Vec::new();
    assert!(combined.write_xml_to(&mut sink).is_err());
    assert!(sink.is_empty());

    let body = &SPACE_TRACK_NDM_1_0
        [SPACE_TRACK_NDM_1_0.find("<omm").unwrap()..SPACE_TRACK_NDM_1_0.find("</ndm>").unwrap()];
    let ocm = Ocm::from_kvn(include_str!("../../data/kvn/ocm_g15.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let ocm = ocm.split_once('\n').unwrap().1;
    let apm_1_0 = r#"<apm id="CCSDS_APM_VERS" version="1.0"><header/></apm>"#;
    for (case, document) in [
        (
            "comments and constituents",
            format!("<ndm><COMMENT>c</COMMENT>{body}</ndm>"),
        ),
        (
            "child namespace declaration",
            format!(
                "<ndm>{}</ndm>",
                body.replacen("<omm ", "<omm xmlns:local=\"urn:example\" ", 1)
            ),
        ),
    ] {
        assert!(CombinedNdm::from_xml(&document).is_ok(), "{case}");
    }
    for (case, document) in [
        (
            "MESSAGE_ID is 4.0.0-only",
            format!("<ndm><MESSAGE_ID>x</MESSAGE_ID>{body}</ndm>"),
        ),
        (
            "COMMENT after a constituent",
            format!("<ndm>{body}<COMMENT>c</COMMENT></ndm>"),
        ),
        (
            "4.0.0-only constituent family",
            format!("<ndm>{body}{ocm}</ndm>"),
        ),
        (
            "qualified root",
            format!("<ndm xmlns=\"urn:ccsds:schema:ndmxml\">{body}</ndm>"),
        ),
        (
            "qualified constituent",
            format!(
                "<ndm>{}</ndm>",
                body.replacen("<omm ", "<omm xmlns=\"urn:ccsds:schema:ndmxml\" ", 1)
            ),
        ),
        (
            "1.0 types namespace on the root",
            format!(
                "<ndm xmlns=\"urn:ccsds:recommendation:navigation:schema:ndmxml\">{body}</ndm>"
            ),
        ),
        (
            "root version attribute",
            format!("<ndm version=\"1.0\">{body}</ndm>"),
        ),
        ("APM 1.0 constituent", format!("<ndm>{apm_1_0}</ndm>")),
        (
            "APM 1.0 beside ODM 2.0",
            format!("<ndm>{body}{apm_1_0}</ndm>"),
        ),
    ] {
        assert!(CombinedNdm::from_xml(&document).is_err(), "{case}");
    }
}
