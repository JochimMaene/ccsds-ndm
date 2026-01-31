use ccsds_ndm::{from_str, MessageType};

#[test]
fn test_parse_nested_cdm_in_message() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<message xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <cdm id="CCSDS_CDM_VERS" version="1.0">
        <header>
            <CREATION_DATE>2025-01-01T00:00:00</CREATION_DATE>
            <ORIGINATOR>TEST</ORIGINATOR>
            <MESSAGE_ID>MSG-001</MESSAGE_ID>
        </header>
        <body>
            <relativeMetadataData>
                <TCA>2025-01-02T12:00:00</TCA>
                <MISS_DISTANCE units="m">100.0</MISS_DISTANCE>
                <RELATIVE_SPEED units="m/s">10.0</RELATIVE_SPEED>
                <relativeStateVector>
                    <RELATIVE_POSITION_R units="m">10.0</RELATIVE_POSITION_R>
                    <RELATIVE_POSITION_T units="m">20.0</RELATIVE_POSITION_T>
                    <RELATIVE_POSITION_N units="m">30.0</RELATIVE_POSITION_N>
                    <RELATIVE_VELOCITY_R units="m/s">0.1</RELATIVE_VELOCITY_R>
                    <RELATIVE_VELOCITY_T units="m/s">0.2</RELATIVE_VELOCITY_T>
                    <RELATIVE_VELOCITY_N units="m/s">0.3</RELATIVE_VELOCITY_N>
                </relativeStateVector>
            </relativeMetadataData>
            <segment>
                <metadata>
                    <OBJECT>OBJECT1</OBJECT>
                    <OBJECT_DESIGNATOR>12345</OBJECT_DESIGNATOR>
                    <CATALOG_NAME>SATCAT</CATALOG_NAME>
                    <OBJECT_NAME>SAT A</OBJECT_NAME>
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
                    <OBJECT_DESIGNATOR>67890</OBJECT_DESIGNATOR>
                    <CATALOG_NAME>SATCAT</CATALOG_NAME>
                    <OBJECT_NAME>SAT B</OBJECT_NAME>
                    <INTERNATIONAL_DESIGNATOR>2000-001A</INTERNATIONAL_DESIGNATOR>
                    <OBJECT_TYPE>PAYLOAD</OBJECT_TYPE>
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
</message>"#;

    let ndm = from_str(xml).expect("Should parse nested CDM");
    if let MessageType::Cdm(cdm) = ndm {
        assert_eq!(cdm.header.originator, "TEST");
    } else {
        panic!("Expected flattened CDM, got {:?}", ndm);
    }
}

#[test]
fn test_parse_wrapped_cdm_unknown_tag() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<somethingExtra>
    <cdm id="CCSDS_CDM_VERS" version="1.0">
        <header>
            <CREATION_DATE>2025-01-01T00:00:00</CREATION_DATE>
            <ORIGINATOR>TEST</ORIGINATOR>
            <MESSAGE_ID>MSG-001</MESSAGE_ID>
        </header>
        <body>
            <relativeMetadataData>
                <TCA>2025-01-02T12:00:00</TCA>
                <MISS_DISTANCE units="m">100.0</MISS_DISTANCE>
                <RELATIVE_SPEED units="m/s">10.0</RELATIVE_SPEED>
                <relativeStateVector>
                    <RELATIVE_POSITION_R units="m">10.0</RELATIVE_POSITION_R>
                    <RELATIVE_POSITION_T units="m">20.0</RELATIVE_POSITION_T>
                    <RELATIVE_POSITION_N units="m">30.0</RELATIVE_POSITION_N>
                    <RELATIVE_VELOCITY_R units="m/s">0.1</RELATIVE_VELOCITY_R>
                    <RELATIVE_VELOCITY_T units="m/s">0.2</RELATIVE_VELOCITY_T>
                    <RELATIVE_VELOCITY_N units="m/s">0.3</RELATIVE_VELOCITY_N>
                </relativeStateVector>
            </relativeMetadataData>
            <segment>
                <metadata>
                    <OBJECT>OBJECT1</OBJECT>
                    <OBJECT_DESIGNATOR>12345</OBJECT_DESIGNATOR>
                    <CATALOG_NAME>SATCAT</CATALOG_NAME>
                    <OBJECT_NAME>SAT A</OBJECT_NAME>
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
                    <OBJECT_DESIGNATOR>67890</OBJECT_DESIGNATOR>
                    <CATALOG_NAME>SATCAT</CATALOG_NAME>
                    <OBJECT_NAME>SAT B</OBJECT_NAME>
                    <INTERNATIONAL_DESIGNATOR>2000-001A</INTERNATIONAL_DESIGNATOR>
                    <OBJECT_TYPE>PAYLOAD</OBJECT_TYPE>
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
</somethingExtra>"#;

    // This should detect the CDM eventually.
    // BUT we need to make sure Cdm::from_xml can handle it if it's not at the start.
    let ndm = from_str(xml).expect("Should find nested CDM in random wrapper");
    assert!(matches!(ndm, MessageType::Cdm(_)));
}
