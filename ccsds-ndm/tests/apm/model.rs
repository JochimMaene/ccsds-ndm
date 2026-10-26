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

#[test]
fn every_optional_attitude_block_revalidates_required_frames_and_numbers() {
    use ccsds_ndm::common::QuaternionDot;
    use ccsds_ndm::types::{DeltaMassZ, QuaternionDotComponent, TimeUnits};
    let base = Apm::from_kvn(include_str!("../../data/kvn/apm_g3.kvn")).unwrap();
    type Edit = fn(&mut Apm);
    let mutations: &[(&str, Edit)] = &[
        ("REF_FRAME_A", |m| {
            m.body.segment.data.quaternion_state[0].ref_frame_a.clear()
        }),
        ("REF_FRAME_B", |m| {
            m.body.segment.data.quaternion_state[0].ref_frame_b.clear()
        }),
        ("Q1_DOT", |m| {
            m.body.segment.data.quaternion_state[0].quaternion_dot = Some(QuaternionDot {
                q1_dot: QuaternionDotComponent::new(f64::NAN, None),
                q2_dot: QuaternionDotComponent::new(0.0, None),
                q3_dot: QuaternionDotComponent::new(0.0, None),
                qc_dot: QuaternionDotComponent::new(0.0, None),
            });
        }),
        ("INERTIA_REF_FRAME", |m| {
            m.body.segment.data.inertia[0].inertia_ref_frame.clear()
        }),
        ("MAN_DURATION", |m| {
            m.body.segment.data.maneuver_parameters[0]
                .man_duration
                .value = -1.0
        }),
        ("MAN_DURATION units", |m| {
            m.body.segment.data.maneuver_parameters[0]
                .man_duration
                .units = Some(TimeUnits::Day)
        }),
        ("MAN_REF_FRAME", |m| {
            m.body.segment.data.maneuver_parameters[0]
                .man_ref_frame
                .clear()
        }),
        ("MAN_TOR_X", |m| {
            m.body.segment.data.maneuver_parameters[0].man_tor_x.value = f64::INFINITY
        }),
        ("MAN_TOR_Y", |m| {
            m.body.segment.data.maneuver_parameters[0].man_tor_y.value = f64::NAN
        }),
        ("MAN_TOR_Z", |m| {
            m.body.segment.data.maneuver_parameters[0].man_tor_z.value = f64::NEG_INFINITY
        }),
        ("MAN_DELTA_MASS", |m| {
            m.body.segment.data.maneuver_parameters[0].man_delta_mass = Some(DeltaMassZ {
                value: 1.0,
                units: None,
            })
        }),
        ("MAN_DELTA_MASS", |m| {
            m.body.segment.data.maneuver_parameters[0].man_delta_mass = Some(DeltaMassZ {
                value: f64::NAN,
                units: None,
            })
        }),
    ];
    for (field, mutate) in mutations {
        let mut message = base.clone();
        mutate(&mut message);
        crate::common::assert_rejects(&message, field);
    }
    let mut euler = Apm::from_kvn(include_str!("../../data/kvn/apm_g2.kvn")).unwrap();
    euler.body.segment.data.euler_angle_state[0]
        .ref_frame_a
        .clear();
    crate::common::assert_rejects(&euler, "REF_FRAME_A");
    let mut angular = base;
    angular
        .body
        .segment
        .data
        .angular_velocity
        .push(AngVelState {
            comment: vec![],
            ref_frame_a: "ICRF".into(),
            ref_frame_b: "SC_BODY".into(),
            angvel_frame: ccsds_ndm::types::AngVelFrameType("".into()),
            angvel_x: ccsds_ndm::types::AngleRate::new(0.0, None),
            angvel_y: ccsds_ndm::types::AngleRate::new(0.0, None),
            angvel_z: ccsds_ndm::types::AngleRate::new(0.0, None),
        });
    crate::common::assert_rejects(&angular, "ANGVEL_FRAME");
}
