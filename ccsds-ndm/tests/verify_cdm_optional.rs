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
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
START_SCREEN_PERIOD = 
STOP_SCREEN_PERIOD =
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
OBJECT_NAME = OBJ1
CATALOG_NAME = CAT
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1000.0 [m]
Y = 2000.0 [m]
Z = 3000.0 [m]
X_DOT = 1.0 [m/s]
Y_DOT = 2.0 [m/s]
Z_DOT = 3.0 [m/s]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1500.0 [m]
Y = 2500.0 [m]
Z = 3500.0 [m]
X_DOT = 1.5 [m/s]
Y_DOT = 2.5 [m/s]
Z_DOT = 3.5 [m/s]
"#;

    let cdm = Cdm::from_kvn(kvn).expect("Failed to parse CDM KVN with empty optional fields");
    
    // START_SCREEN_PERIOD and STOP_SCREEN_PERIOD were empty in KVN, so they should be None
    assert!(cdm.body.relative_metadata_data.start_screen_period.is_none(), "START_SCREEN_PERIOD should be None");
    assert!(cdm.body.relative_metadata_data.stop_screen_period.is_none(), "STOP_SCREEN_PERIOD should be None");
}

#[test]
fn test_cdm_xml_optional_nil() {
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
            <START_SCREEN_PERIOD nil="true"/>
            <STOP_SCREEN_PERIOD></STOP_SCREEN_PERIOD>
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
            </data>
        </segment>
    </body>
</cdm>
"#;

    let cdm = Cdm::from_xml(xml).expect("Failed to parse CDM XML with nil optional fields");
    
    // START_SCREEN_PERIOD was nil="true", so it should be None
    assert!(cdm.body.relative_metadata_data.start_screen_period.is_none(), "START_SCREEN_PERIOD should be None");
    
    // STOP_SCREEN_PERIOD was empty tag, let's see what happens.
    // Ideally it should also be None if we want robust optional handling, 
    // or maybe it fails if not handled.
    // Based on `nullable_value` implementation:
    // It checks `nil="true"`.
    // It doesn't seem to explicitly check for empty content if nil is absent, 
    // so it might try to parse empty string as Epoch and fail, OR 
    // maybe serde treats empty tag as "default" -> None? 
    // Let's assert None for now and see.
    assert!(cdm.body.relative_metadata_data.stop_screen_period.is_none(), "STOP_SCREEN_PERIOD should be None");
}
