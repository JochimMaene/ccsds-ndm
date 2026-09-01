// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::traits::Ndm;

#[test]
fn test_cdm_kvn_optional_empty_value() {
    let kvn = r#"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
START_SCREEN_PERIOD =
STOP_SCREEN_PERIOD =
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.001 [km/s]
Y_DOT = 0.002 [km/s]
Z_DOT = 0.003 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1.5 [km]
Y = 2.5 [km]
Z = 3.5 [km]
X_DOT = 0.0015 [km/s]
Y_DOT = 0.0025 [km/s]
Z_DOT = 0.0035 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"#;

    let cdm = Cdm::from_kvn(kvn).expect("Failed to parse CDM KVN with empty optional fields");

    // START_SCREEN_PERIOD and STOP_SCREEN_PERIOD were empty in KVN, so they should be None
    assert!(
        cdm.body
            .relative_metadata_data
            .start_screen_period
            .is_none(),
        "START_SCREEN_PERIOD should be None"
    );
    assert!(
        cdm.body.relative_metadata_data.stop_screen_period.is_none(),
        "STOP_SCREEN_PERIOD should be None"
    );
}

#[test]
fn test_cdm_xml_optional_omission() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<cdm xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://sanaregistry.org/r/ndmxml/ndmxml-1.0-master.xsd" id="CCSDS_CDM_VERS" version="1.0">
    <header>
        <CREATION_DATE>2025-01-01T00:00:00</CREATION_DATE>
        <ORIGINATOR>TEST</ORIGINATOR>
        <MESSAGE_FOR>OPERATOR</MESSAGE_FOR>
        <MESSAGE_ID>MSG-001</MESSAGE_ID>
    </header>
    <body>
        <relativeMetadataData>
            <TCA>2025-01-02T12:00:00</TCA>
            <MISS_DISTANCE units="m">100.0</MISS_DISTANCE>
            <RELATIVE_SPEED units="m/s">7.5</RELATIVE_SPEED>
            <relativeStateVector>
                <RELATIVE_POSITION_R units="m">10.0</RELATIVE_POSITION_R>
                <RELATIVE_POSITION_T units="m">-20.0</RELATIVE_POSITION_T>
                <RELATIVE_POSITION_N units="m">5.0</RELATIVE_POSITION_N>
                <RELATIVE_VELOCITY_R units="m/s">0.1</RELATIVE_VELOCITY_R>
                <RELATIVE_VELOCITY_T units="m/s">-0.2</RELATIVE_VELOCITY_T>
                <RELATIVE_VELOCITY_N units="m/s">0.05</RELATIVE_VELOCITY_N>
            </relativeStateVector>
            <SCREEN_VOLUME_FRAME>RTN</SCREEN_VOLUME_FRAME>
            <SCREEN_VOLUME_SHAPE>BOX</SCREEN_VOLUME_SHAPE>
            <SCREEN_VOLUME_X units="m">1000.0</SCREEN_VOLUME_X>
            <SCREEN_VOLUME_Y units="m">2000.0</SCREEN_VOLUME_Y>
            <SCREEN_VOLUME_Z units="m">3000.0</SCREEN_VOLUME_Z>
            <COLLISION_PROBABILITY>0.001</COLLISION_PROBABILITY>
            <COLLISION_PROBABILITY_METHOD>FOSTER-1992</COLLISION_PROBABILITY_METHOD>
        </relativeMetadataData>
        <segment>
            <metadata>
                <OBJECT>OBJECT1</OBJECT>
                <OBJECT_DESIGNATOR>00001</OBJECT_DESIGNATOR>
                <CATALOG_NAME>CAT</CATALOG_NAME>
                <OBJECT_NAME>OBJ1</OBJECT_NAME>
                <INTERNATIONAL_DESIGNATOR>1998-067A</INTERNATIONAL_DESIGNATOR>
                <OBJECT_TYPE>PAYLOAD</OBJECT_TYPE>
                <EPHEMERIS_NAME>EPH1</EPHEMERIS_NAME>
                <COVARIANCE_METHOD>CALCULATED</COVARIANCE_METHOD>
                <MANEUVERABLE>YES</MANEUVERABLE>
                <REF_FRAME>GCRF</REF_FRAME>
            </metadata>
            <data>
                <stateVector>
                    <X units="km">1000.0</X>
                    <Y units="km">2000.0</Y>
                    <Z units="km">3000.0</Z>
                    <X_DOT units="km/s">1.0</X_DOT>
                    <Y_DOT units="km/s">2.0</Y_DOT>
                    <Z_DOT units="km/s">3.0</Z_DOT>
                </stateVector>
                <covarianceMatrix>
                    <CR_R units="m**2">1.0</CR_R>
                    <CT_R units="m**2">0.0</CT_R>
                    <CT_T units="m**2">1.0</CT_T>
                    <CN_R units="m**2">0.0</CN_R>
                    <CN_T units="m**2">0.0</CN_T>
                    <CN_N units="m**2">1.0</CN_N>
                    <CRDOT_R units="m**2/s">0.0</CRDOT_R>
                    <CRDOT_T units="m**2/s">0.0</CRDOT_T>
                    <CRDOT_N units="m**2/s">0.0</CRDOT_N>
                    <CRDOT_RDOT units="m**2/s**2">1.0</CRDOT_RDOT>
                    <CTDOT_R units="m**2/s">0.0</CTDOT_R>
                    <CTDOT_T units="m**2/s">0.0</CTDOT_T>
                    <CTDOT_N units="m**2/s">0.0</CTDOT_N>
                    <CTDOT_RDOT units="m**2/s**2">0.0</CTDOT_RDOT>
                    <CTDOT_TDOT units="m**2/s**2">1.0</CTDOT_TDOT>
                    <CNDOT_R units="m**2/s">0.0</CNDOT_R>
                    <CNDOT_T units="m**2/s">0.0</CNDOT_T>
                    <CNDOT_N units="m**2/s">0.0</CNDOT_N>
                    <CNDOT_RDOT units="m**2/s**2">0.0</CNDOT_RDOT>
                    <CNDOT_TDOT units="m**2/s**2">0.0</CNDOT_TDOT>
                    <CNDOT_NDOT units="m**2/s**2">1.0</CNDOT_NDOT>
                </covarianceMatrix>
            </data>
        </segment>
        <segment>
            <metadata>
                <OBJECT>OBJECT2</OBJECT>
                <OBJECT_DESIGNATOR>00002</OBJECT_DESIGNATOR>
                <CATALOG_NAME>CAT</CATALOG_NAME>
                <OBJECT_NAME>OBJ2</OBJECT_NAME>
                <INTERNATIONAL_DESIGNATOR>1998-067B</INTERNATIONAL_DESIGNATOR>
                <EPHEMERIS_NAME>EPH1</EPHEMERIS_NAME>
                <COVARIANCE_METHOD>CALCULATED</COVARIANCE_METHOD>
                <MANEUVERABLE>NO</MANEUVERABLE>
                <REF_FRAME>GCRF</REF_FRAME>
            </metadata>
            <data>
                <stateVector>
                    <X units="km">1500.0</X>
                    <Y units="km">2500.0</Y>
                    <Z units="km">3500.0</Z>
                    <X_DOT units="km/s">1.5</X_DOT>
                    <Y_DOT units="km/s">2.5</Y_DOT>
                    <Z_DOT units="km/s">3.5</Z_DOT>
                </stateVector>
                <covarianceMatrix>
                    <CR_R units="m**2">1.0</CR_R>
                    <CT_R units="m**2">0.0</CT_R>
                    <CT_T units="m**2">1.0</CT_T>
                    <CN_R units="m**2">0.0</CN_R>
                    <CN_T units="m**2">0.0</CN_T>
                    <CN_N units="m**2">1.0</CN_N>
                    <CRDOT_R units="m**2/s">0.0</CRDOT_R>
                    <CRDOT_T units="m**2/s">0.0</CRDOT_T>
                    <CRDOT_N units="m**2/s">0.0</CRDOT_N>
                    <CRDOT_RDOT units="m**2/s**2">1.0</CRDOT_RDOT>
                    <CTDOT_R units="m**2/s">0.0</CTDOT_R>
                    <CTDOT_T units="m**2/s">0.0</CTDOT_T>
                    <CTDOT_N units="m**2/s">0.0</CTDOT_N>
                    <CTDOT_RDOT units="m**2/s**2">0.0</CTDOT_RDOT>
                    <CTDOT_TDOT units="m**2/s**2">1.0</CTDOT_TDOT>
                    <CNDOT_R units="m**2/s">0.0</CNDOT_R>
                    <CNDOT_T units="m**2/s">0.0</CNDOT_T>
                    <CNDOT_N units="m**2/s">0.0</CNDOT_N>
                    <CNDOT_RDOT units="m**2/s**2">0.0</CNDOT_RDOT>
                    <CNDOT_TDOT units="m**2/s**2">0.0</CNDOT_TDOT>
                    <CNDOT_NDOT units="m**2/s**2">1.0</CNDOT_NDOT>
                </covarianceMatrix>
            </data>
        </segment>
    </body>
</cdm>
"#;

    let cdm = Cdm::from_xml(xml).expect("Failed to parse CDM XML with omitted optional fields");

    // Optional XML elements are represented by omission, as required by the CDM XSD.
    assert!(
        cdm.body
            .relative_metadata_data
            .start_screen_period
            .is_none(),
        "START_SCREEN_PERIOD should be None"
    );

    assert!(
        cdm.body.relative_metadata_data.stop_screen_period.is_none(),
        "STOP_SCREEN_PERIOD should be None"
    );
}
