use ccsds_ndm::common::OdmHeader;
use ccsds_ndm::messages::ocm::{
    CovLine, ManLine, Ocm, OcmBody, OcmData, OcmMetadata, OcmSegment, OcmTrajState, TrajLine,
};
use ccsds_ndm::types::{Duration, TimeOffset, TimeUnits};
use ccsds_ndm::{Ndm, Validate};
#[test]
fn ocm_validation_traj_lines() {
    let mut ocm = Ocm::builder()
        .header(
            OdmHeader::builder()
                .originator("TEST")
                .creation_date("2000-01-01T00:00:00".parse().unwrap())
                .build(),
        )
        .body(
            OcmBody::builder()
                .segment(Box::new(
                    OcmSegment::builder()
                        .metadata(
                            OcmMetadata::builder()
                                .time_system("UTC")
                                .epoch_tzero("2000-01-01T00:00:00".parse().unwrap())
                                .build(),
                        )
                        .data(OcmData::default())
                        .build(),
                ))
                .build(),
        )
        .version("3.0")
        .build();

    let traj = OcmTrajState::builder()
        .center_name("EARTH")
        .traj_ref_frame("GCRF")
        .traj_type("CARTPV")
        .build();
    // Missing lines
    ocm.body.segment.data.traj.push(traj);
    crate::common::assert_rejects(&ocm, "trajLine");

    // Fix it
    ocm.body.segment.data.traj[0].traj_lines.push(TrajLine {
        epoch: "2000-01-01T00:00:00".parse().unwrap(),
        values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    });
    assert!(ocm.validate().is_ok());

    for value in ["2000-366T23:59:60Z", "12345.5"] {
        ocm.body.segment.data.traj[0].traj_lines[0].epoch = value.parse().unwrap();
        assert!(ocm.validate().is_ok(), "valid trajectory epoch {value}");
    }

    for value in ["2001-366T00:00:00", "2023-01-01T24:00:00", "+", "123."] {
        ocm.body.segment.data.traj[0].traj_lines[0].epoch = value.parse().unwrap();
        crate::common::assert_validation_field(&ocm.validate().unwrap_err(), "trajLine epoch");
    }
}

#[test]
fn parse_simple_ocm() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    assert_eq!(ocm.body.segment.data.traj.len(), 1);
    assert_eq!(ocm.body.segment.data.traj[0].traj_lines[0].values.len(), 6);
}

// =========================================================================
// XSD COMPLIANCE TESTS - Group 1: Mandatory Metadata Fields
// XSD: TIME_SYSTEM and EPOCH_TZERO are mandatory (no minOccurs="0")
// =========================================================================

#[test]
fn sample_ocm_g20_xml() {
    // Parse official CCSDS OCM XML example G-20
    let xml = include_str!("../../data/xml/ocm_g20.xml");
    let ocm = Ocm::from_xml(xml).unwrap();

    // Verify mandatory metadata
    assert!(!ocm.body.segment.metadata.time_system.is_empty());
}

#[test]
fn kvn_roundtrip() {
    // Full roundtrip: KVN -> Ocm -> KVN
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
"#;
    let ocm = Ocm::from_kvn(kvn).unwrap();
    let output = ocm.to_kvn().unwrap();

    // Parse output again
    let ocm2 = Ocm::from_kvn(&output).unwrap();
    assert_eq!(ocm2, ocm);
}

#[test]
fn covline_xml_serialization() {
    // Test serialization by wrapping in an XML struct
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestWrapper {
        cov_line: CovLine,
    }

    let cov_line = CovLine {
        epoch: "2023-01-01T00:00:00".parse().unwrap(),
        values: vec![1.0, 2.0, 3.0],
    };

    let wrapper = TestWrapper { cov_line };

    // Use quick-xml to serialize (which uses the custom Serialize impl)
    let xml = quick_xml::se::to_string(&wrapper).unwrap();

    // Deserialize and verify using quick-xml
    let deserialized: TestWrapper = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(deserialized.cov_line, wrapper.cov_line);
}

#[test]
fn manline_xml_serialization() {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestWrapper {
        man_line: ManLine,
    }

    let man_line = ManLine {
        epoch: "2023-01-01T00:00:00".parse().unwrap(),
        values: vec!["1.0".to_string(), "2.0".to_string(), "3.0".to_string()],
    };

    let wrapper = TestWrapper { man_line };

    // Use quick-xml to serialize
    let xml = quick_xml::se::to_string(&wrapper).unwrap();

    // Deserialize and verify
    let deserialized: TestWrapper = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(deserialized.man_line, wrapper.man_line);
}

#[test]
fn ocm_metadata_sclk_requirements_and_units() {
    let mut metadata = OcmMetadata::builder()
        .time_system("SCLK")
        .epoch_tzero("2023-01-01T00:00:00".parse().unwrap())
        .build();
    let error = metadata.validate().unwrap_err().to_string();
    assert!(error.contains("SCLK_OFFSET_AT_EPOCH"));

    metadata.sclk_offset_at_epoch = Some(TimeOffset {
        value: 100.0,
        units: Some(TimeUnits::Seconds),
    });
    let error = metadata.validate().unwrap_err().to_string();
    assert!(error.contains("SCLK_SEC_PER_SI_SEC"));

    metadata.sclk_sec_per_si_sec = Some(Duration {
        value: 1.0,
        units: Some(TimeUnits::Seconds),
    });
    assert!(metadata.validate().is_ok());

    metadata.next_leap_epoch = Some("2024-01-01T00:00:00".parse().unwrap());
    assert!(metadata.validate().is_ok());

    metadata.sclk_offset_at_epoch.as_mut().unwrap().units = Some(TimeUnits::Day);
    let error = metadata.validate().unwrap_err().to_string();
    assert!(error.contains("SCLK_OFFSET_AT_EPOCH"));

    metadata.sclk_offset_at_epoch.as_mut().unwrap().units = Some(TimeUnits::Seconds);
    metadata.sclk_sec_per_si_sec.as_mut().unwrap().units = Some(TimeUnits::Day);
    let error = metadata.validate().unwrap_err().to_string();
    assert!(error.contains("SCLK_SEC_PER_SI_SEC"));
}
