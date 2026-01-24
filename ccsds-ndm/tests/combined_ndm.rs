// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::{from_str, MessageType};

#[test]
fn test_combined_ndm_kvn() {
    let input = r#"
CCSDS_OPM_VERS = 2.0
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

CCSDS_OEM_VERS = 2.0
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

    let msg = from_str(input).unwrap();
    match msg {
        MessageType::Ndm(ndm) => {
            assert_eq!(ndm.messages.len(), 2);
            assert!(matches!(ndm.messages[0], MessageType::Opm(_)));
            assert!(matches!(ndm.messages[1], MessageType::Oem(_)));
        }
        _ => panic!("Expected MessageType::Ndm, got {:?}", msg),
    }
}

#[test]
fn test_combined_ndm_xml() {
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<ndm>
    <MESSAGE_ID>TEST_ID_123</MESSAGE_ID>
    <COMMENT>Global NDM comment</COMMENT>
    <opm id="1.0" version="2.0">
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
    <omm id="2.0" version="2.0">
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
