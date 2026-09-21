use crate::common::mutated;
use ccsds_ndm::common::{AngVelState, InertiaState};
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::{Ndm, Validate};
fn sample_apm_kvn() -> String {
    r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2002-11-04T17:22:31
QUAT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
Q1 = 0.5
Q2 = 0.5
Q3 = 0.5
QC = 0.5
QUAT_STOP
"#
    .to_string()
}

#[test]
fn parse_apm_success() {
    let kvn = sample_apm_kvn();
    let apm = Apm::from_kvn(&kvn).expect("APM parse failed");

    assert_eq!(apm.version, "2.0");
    assert_eq!(
        apm.body.segment.metadata.object_name,
        "MARS GLOBAL SURVEYOR"
    );
    assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    assert_eq!(apm.body.segment.data.quaternion_state[0].quaternion.q1, 0.5);
}

/// The two rules that reject an otherwise well-formed APM, as mutations of a shipped fixture.
///
/// `apm_g2.kvn` is the baseline because its `OBJECT_NAME` is not preceded by comments:
/// dropping that line from `apm_g1.kvn` leaves a `COMMENT` at the head of the metadata and the
/// message is refused for comment placement instead, which is not the rule under test.
#[test]
fn apm_rejects_missing_object_name_or_all_data_blocks() {
    const FIXTURE: &str = include_str!("../../data/kvn/apm_g2.kvn");
    Apm::from_kvn(FIXTURE).expect("baseline fixture must satisfy both rules");

    let attitude_at = FIXTURE.find("EULER_START").unwrap();
    for (mutated, expected) in [
        (
            FIXTURE[..attitude_at].to_owned(),
            "Missing required field: At least one logical block in block APM Data",
        ),
        (
            mutated(FIXTURE, "OBJECT_NAME = GOES-P\n", ""),
            "Missing required field: OBJECT_NAME in block APM Metadata",
        ),
    ] {
        let error = Apm::from_kvn(&mutated).expect_err("mutation accepted");
        assert!(
            error.to_string().contains(expected),
            "diagnostic did not name {expected}: {error}"
        );
    }
}

#[test]
fn apm_multiple_blocks() {
    let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
EULER_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
EULER_ROT_SEQ = XYZ
ANGLE_1 = 10 [deg]
ANGLE_2 = 20 [deg]
ANGLE_3 = 30 [deg]
EULER_STOP
"#;
    let apm = Apm::from_kvn(kvn).unwrap();
    assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    assert_eq!(apm.body.segment.data.euler_angle_state.len(), 1);
}
#[test]
fn apm_validation_single_blocks() {
    // Test that having just one block is sufficient
    let mut apm = Apm::from_kvn(&sample_apm_kvn()).unwrap();

    // Clear all blocks
    apm.body.segment.data.quaternion_state.clear();
    let error = apm
        .validate()
        .expect_err("a data section with no block was accepted");
    assert!(
        error.to_string().contains("At least one logical block"),
        "unexpected diagnostic: {error}"
    );

    // Add just Inertia
    apm.body.segment.data.inertia.push(InertiaState {
        comment: vec![],
        inertia_ref_frame: "SC_BODY".to_string(),
        ixx: ccsds_ndm::types::Moment::new(1.0, None),
        iyy: ccsds_ndm::types::Moment::new(2.0, None),
        izz: ccsds_ndm::types::Moment::new(3.0, None),
        ixy: ccsds_ndm::types::Moment::new(0.0, None),
        ixz: ccsds_ndm::types::Moment::new(0.0, None),
        iyz: ccsds_ndm::types::Moment::new(0.0, None),
    });
    assert!(apm.validate().is_ok());

    // Clear and add just Angular Velocity
    apm.body.segment.data.inertia.clear();
    apm.body.segment.data.angular_velocity.push(AngVelState {
        comment: vec![],
        ref_frame_a: "GCRF".to_string(),
        ref_frame_b: "SC_BODY".to_string(),
        angvel_frame: ccsds_ndm::types::AngVelFrameType("SC_BODY".to_string()),
        angvel_x: ccsds_ndm::types::AngleRate::new(0.1, None),
        angvel_y: ccsds_ndm::types::AngleRate::new(0.1, None),
        angvel_z: ccsds_ndm::types::AngleRate::new(0.1, None),
    });
    assert!(apm.validate().is_ok());
}
