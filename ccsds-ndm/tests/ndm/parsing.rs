use ccsds_ndm::error::{DiagnosticNotation, DiagnosticOperation};
// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated_once;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::{from_str, Message, Ndm};

/// Prefix the XML declaration that NDM/XML 4.2 requires on the first line.
fn declared(body: &str) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{body}")
}

#[test]
fn combined_xml_parser_enforces_the_normative_envelope() {
    // ODM 8.12.6: the combined root carries the standard xmlns:xsi declaration.
    crate::common::assert_invalid_format(
        &CombinedNdm::from_xml(&declared("<ndm/>")).unwrap_err(),
        "the combined NDM root element must declare \
         xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"",
    );
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
fn sequential_kvn_messages_are_rejected_by_generic_detection() {
    let input = r#"
CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2021-01-01T12:00:00.000
ORIGINATOR    = NASA
OBJECT_NAME          = SATELLITE
OBJECT_ID            = 2020-001A
CENTER_NAME          = EARTH
REF_FRAME            = GCRF
TIME_SYSTEM          = UTC
EPOCH                = 2021-01-01T12:00:00.000
X                    = 6500.0 [km]
Y                    = 0.0 [km]
Z                    = 0.0 [km]
X_DOT                = 0.0 [km/s]
Y_DOT                = 7.5 [km/s]
Z_DOT                = 0.0 [km/s]

CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2021-01-01T12:00:00.000
ORIGINATOR    = NASA
META_START
OBJECT_NAME          = SATELLITE
OBJECT_ID            = 2020-001A
CENTER_NAME          = EARTH
REF_FRAME            = GCRF
TIME_SYSTEM          = UTC
START_TIME           = 2021-01-01T12:00:00.000
USEABLE_START_TIME   = 2021-01-01T12:00:00.000
USEABLE_STOP_TIME    = 2021-01-02T12:00:00.000
STOP_TIME            = 2021-01-02T12:00:00.000
INTERPOLATION        = HERMITE
INTERPOLATION_DEGREE = 1
META_STOP
2021-01-01T12:00:00.000 6500.0 0.0 0.0 0.0 7.5 0.0
"#;

    let error = from_str(input).unwrap_err();
    let diagnostic = error.diagnostic().unwrap();
    assert_eq!(diagnostic.code, Some("unsupported.notation"));
    assert_eq!(
        diagnostic.operation,
        ccsds_ndm::error::DiagnosticOperation::Parse
    );
    assert_eq!(
        diagnostic.notation,
        ccsds_ndm::error::DiagnosticNotation::Kvn
    );
    assert_eq!(
        diagnostic.message_kind,
        ccsds_ndm::validation::MessageKind::Ndm
    );
}

#[test]
fn combined_ndm_xml() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <MESSAGE_ID>TEST_ID_123</MESSAGE_ID>
    <COMMENT>Global NDM comment</COMMENT>
    <opm id="CCSDS_OPM_VERS" version="3.0">
        <header>
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
    <omm id="CCSDS_OMM_VERS" version="3.0">
        <header>
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
                    <MEAN_ELEMENT_THEORY>DSST</MEAN_ELEMENT_THEORY>
                </metadata>
                <data>
                   <meanElements>
                        <EPOCH>2019-01-01T00:00:00.000Z</EPOCH>
                        <SEMI_MAJOR_AXIS units="km">6700.0</SEMI_MAJOR_AXIS>
                        <ECCENTRICITY>0.001</ECCENTRICITY>
                        <INCLINATION units="deg">56.0</INCLINATION>
                        <RA_OF_ASC_NODE units="deg">10.0</RA_OF_ASC_NODE>
                        <ARG_OF_PERICENTER units="deg">5.0</ARG_OF_PERICENTER>
                        <MEAN_ANOMALY units="deg">20.0</MEAN_ANOMALY>
                        <GM units="km**3/s**2">398600.4418</GM>
                   </meanElements>
                </data>
            </segment>
        </body>
    </omm>
</ndm>
"#;

    let msg = from_str(input).unwrap();
    match msg {
        Message::Ndm(ndm) => {
            assert_eq!(ndm.id, Some("TEST_ID_123".to_string()));
            assert_eq!(ndm.comments, vec!["Global NDM comment".to_string()]);
            assert_eq!(ndm.messages.len(), 2);
            assert!(matches!(ndm.messages[0], Message::Opm(_)));
            assert!(matches!(ndm.messages[1], Message::Omm(_)));
        }
        _ => panic!("Expected Message::Ndm, got {:?}", msg),
    }
}

#[test]
fn combined_ndm_xml_attitude() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <COMMENT>Example: 1 each APM, AEM, ACM in combined instantiation</COMMENT>
    <apm id="CCSDS_APM_VERS" version="2.0">
        <header>
            <CREATION_DATE>2007-11-10T15:23:57</CREATION_DATE>
            <ORIGINATOR>CNES</ORIGINATOR>
        </header>
        <body>
            <segment>
                <metadata>
                    <OBJECT_NAME>TEST</OBJECT_NAME>
                    <OBJECT_ID>2007-011</OBJECT_ID>
                    <CENTER_NAME>EARTH</CENTER_NAME>
                    <TIME_SYSTEM>UTC</TIME_SYSTEM>
                </metadata>
                <data>
                    <EPOCH>2007-10-01T00:02:00.000</EPOCH>
                    <eulerAngleState>
                        <REF_FRAME_A>SC_BODY</REF_FRAME_A>
                        <REF_FRAME_B>J2000</REF_FRAME_B>
                        <EULER_ROT_SEQ>ZXZ</EULER_ROT_SEQ>
                        <ANGLE_1 units="deg">90.</ANGLE_1>
                        <ANGLE_2 units="deg">130.</ANGLE_2>
                        <ANGLE_3 units="deg">270.</ANGLE_3>
                    </eulerAngleState>
                </data>
            </segment>
        </body>
    </apm>
    <aem id="CCSDS_AEM_VERS" version="2.0">
        <header>
            <CREATION_DATE>2000-100T01:00:00</CREATION_DATE>
            <ORIGINATOR>NASA/JPL</ORIGINATOR>
        </header>
        <body>
            <segment>
                <metadata>
                    <OBJECT_NAME>TEST</OBJECT_NAME>
                    <OBJECT_ID>2000-999Z</OBJECT_ID>
                    <REF_FRAME_A>SC_BODY_1</REF_FRAME_A>
                    <REF_FRAME_B>J2000</REF_FRAME_B>
                    <TIME_SYSTEM>TDB</TIME_SYSTEM>
                    <START_TIME>2000-100T00:00:00.000</START_TIME>
                    <STOP_TIME>2000-100T00:00:00.000</STOP_TIME>
                    <ATTITUDE_TYPE>QUATERNION</ATTITUDE_TYPE>
                </metadata>
                <data>
                    <attitudeState>
                        <quaternionEphemeris>
                            <EPOCH>2000-100T00:00:00.000</EPOCH>
                            <quaternion>
                                <Q1>-0.005068</Q1>
                                <Q2>0.906506</Q2>
                                <Q3>0.002360</Q3>
                                <QC>0.422157</QC>
                            </quaternion>
                        </quaternionEphemeris>
                    </attitudeState>
                </data>
            </segment>
        </body>
    </aem>
    <acm id="CCSDS_ACM_VERS" version="2.0">
        <header>
            <CREATION_DATE>1998-11-06T09:23:57</CREATION_DATE>
            <ORIGINATOR>JAXA</ORIGINATOR>
        </header>
        <body>
            <segment>
                <metadata>
                    <OBJECT_NAME>EUROBIRD-4A</OBJECT_NAME>
                    <INTERNATIONAL_DESIGNATOR>2000-052A</INTERNATIONAL_DESIGNATOR>
                    <TIME_SYSTEM>UTC</TIME_SYSTEM>
                    <EPOCH_TZERO>1998-12-18T14:28:15.1172</EPOCH_TZERO>
                </metadata>
                <data>
                    <att>
                        <REF_FRAME_A>J2000</REF_FRAME_A>
                        <REF_FRAME_B>SC_BODY</REF_FRAME_B>
                        <NUMBER_STATES>4</NUMBER_STATES>
                        <ATT_TYPE>QUATERNION</ATT_TYPE>
                        <attLine>0.0 0.73566 -0.50547 0.41390 0.180707</attLine>
                    </att>
                </data>
            </segment>
        </body>
    </acm>
</ndm>
"#;

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
fn strict_parsing_remains_the_default() {
    crate::common::assert_validation_field(
        &ccsds_ndm::from_str(include_str!("../../data/xml/ndm_g22.xml")).unwrap_err(),
        "MASS",
    );
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
