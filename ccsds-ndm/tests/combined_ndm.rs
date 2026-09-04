// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{from_str, MessageType};

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

#[test]
fn combined_generation_preserves_child_checks() {
    let opm = Opm::from_kvn(OPM_KVN).unwrap();
    let combined = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![MessageType::Opm(opm.clone())],
    };
    let message = MessageType::Ndm(combined.clone());

    message.to_xml().unwrap();

    let mut invalid_xml = opm;
    invalid_xml.header.originator = "control \u{1}".into();
    let invalid_xml = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![MessageType::Opm(invalid_xml)],
    };
    let error = invalid_xml.to_xml().unwrap_err();
    assert_eq!(
        error.diagnostic().unwrap().message_kind,
        ccsds_ndm::validation::MessageKind::Opm
    );
}

#[test]
fn combined_xml_parser_enforces_the_normative_envelope() {
    assert!(CombinedNdm::from_xml("<ndm/>").is_ok());
    assert!(CombinedNdm::from_xml("<message></message>").is_err());
    assert!(CombinedNdm::from_xml("<ndm><UNKNOWN/></ndm>").is_err());
    assert!(CombinedNdm::from_xml("<ndm><message_id>wrong case</message_id></ndm>").is_err());
    assert!(CombinedNdm::from_xml("<ndm><comment>wrong case</comment></ndm>").is_err());
    assert!(CombinedNdm::from_xml(
        "<ndm><COMMENT>first</COMMENT><MESSAGE_ID>late</MESSAGE_ID></ndm>"
    )
    .is_err());
    assert!(CombinedNdm::from_xml("<ndm>").is_err());
    assert!(CombinedNdm::from_xml("<ndm></ndm><ndm></ndm>").is_err());
    assert!(CombinedNdm::from_xml("<ndm></ndm>").is_ok());

    let aem = Aem::from_xml(include_str!("../data/xml/aem_g11.xml")).unwrap();
    let input = CombinedNdm {
        id: None,
        comments: Vec::new(),
        messages: vec![MessageType::Aem(aem)],
    }
    .to_xml()
    .unwrap();
    let options = ccsds_ndm::ParseOptions::default().with_max_records(0);
    let error = CombinedNdm::from_xml_with_options(&input, &options).unwrap_err();
    assert_eq!(error.code(), Some("resource.record_limit_exceeded"));
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
fn test_combined_ndm_xml() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm>
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
        MessageType::Ndm(ndm) => {
            assert_eq!(ndm.id, Some("TEST_ID_123".to_string()));
            assert_eq!(ndm.comments, vec!["Global NDM comment".to_string()]);
            assert_eq!(ndm.messages.len(), 2);
            assert!(matches!(ndm.messages[0], MessageType::Opm(_)));
            assert!(matches!(ndm.messages[1], MessageType::Omm(_)));
        }
        _ => panic!("Expected MessageType::Ndm, got {:?}", msg),
    }
}

#[test]
fn test_combined_ndm_xml_attitude() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm>
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
        MessageType::Ndm(ndm) => {
            assert_eq!(
                ndm.comments,
                vec!["Example: 1 each APM, AEM, ACM in combined instantiation".to_string()]
            );
            assert_eq!(ndm.messages.len(), 3);
            assert!(matches!(ndm.messages[0], MessageType::Apm(_)));
            assert!(matches!(ndm.messages[1], MessageType::Aem(_)));
            assert!(matches!(ndm.messages[2], MessageType::Acm(_)));
        }
        _ => panic!("Expected MessageType::Ndm, got {:?}", msg),
    }
}
