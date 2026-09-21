use crate::common::mutated;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::Ndm;
fn sample_acm_kvn() -> String {
    r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0.5 0.5 0.5 0.5
ATT_STOP
"#
    .to_string()
}

#[test]
fn parse_acm_success() {
    let kvn = sample_acm_kvn();
    let acm = Acm::from_kvn(&kvn).expect("ACM parse failed");

    assert_eq!(acm.version, "2.0");
    assert_eq!(
        acm.body.segment.metadata.object_name,
        "MARS GLOBAL SURVEYOR"
    );
    assert_eq!(acm.body.segment.data.att.len(), 1);
    assert_eq!(acm.body.segment.data.att[0].att_lines.len(), 1);
}

#[test]
fn acm_multiple_att_blocks() {
    let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = INSTRUMENT
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
"#;
    let acm = Acm::from_kvn(kvn).unwrap();
    assert_eq!(acm.body.segment.data.att.len(), 2);
}

#[test]
fn acm_missing_mandatory_metadata() {
    const FIXTURE: &str = include_str!("../../data/kvn/acm_g6.kvn");
    Acm::from_kvn(FIXTURE).expect("baseline fixture must carry OBJECT_NAME");

    let without_object_name = mutated(FIXTURE, "OBJECT_NAME = EUROBIRD-4A\n", "");
    assert_ne!(
        without_object_name, FIXTURE,
        "OBJECT_NAME matched no line, so nothing was dropped"
    );
    let error =
        Acm::from_kvn(&without_object_name).expect_err("message without OBJECT_NAME accepted");
    assert!(
        error
            .to_string()
            .contains("Missing required field: OBJECT_NAME in block ACM Metadata"),
        "unexpected diagnostic: {error}"
    );
}

#[test]
fn acm_physical_block() {
    let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
WET_MASS = 1000 [kg]
DRY_MASS = 500 [kg]
PHYS_STOP
"#;
    let acm = Acm::from_kvn(kvn).unwrap();
    let phys = acm.body.segment.data.phys.as_ref().unwrap();
    assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1000.0);
}

#[test]
fn acm_sensor_xml_roundtrip() {
    let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
AD_START
ATTITUDE_STATES = QUATERNION
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY_1
SENSOR_START
SENSOR_NUMBER = 1
SENSOR_USED = AST
SENSOR_STOP
AD_STOP
"#;

    let acm = Acm::from_kvn(kvn).expect("failed to parse ACM KVN");
    let xml = acm.to_xml().expect("failed to serialize ACM XML");
    let parsed = Acm::from_xml(&xml).expect("failed to parse ACM XML");
    assert_eq!(parsed, acm);
    assert_eq!(
        parsed.body.segment.data.ad.as_ref().unwrap().sensors.len(),
        1
    );
    assert_eq!(
        parsed.body.segment.data.ad.as_ref().unwrap().sensors[0].sensor_number,
        Some(1)
    );
}
