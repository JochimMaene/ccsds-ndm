use crate::sample_apm_kvn;
use ccsds_ndm::common::{AngVelState, InertiaState};
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::{Ndm, Validate};
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
