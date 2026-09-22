use crate::common::{assert_rejects, validate_xml};
use crate::SPIN_KVN;
use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::Ndm;
#[test]
fn apm_spin_revalidates_edited_numeric_values() {
    let base = Apm::from_kvn(SPIN_KVN).unwrap();
    let mut angle = base.clone();
    angle.body.segment.data.spin[0].spin_alpha.value = 360.0;
    assert_rejects(&angle, "SPIN_ALPHA");

    let mut rate = base.clone();
    rate.body.segment.data.spin[0].spin_angle_vel.value = f64::NAN;
    assert_rejects(&rate, "SPIN_ANGLE_VEL");

    let mut nutation = base.clone();
    nutation.body.segment.data.spin[0]
        .nutation
        .as_mut()
        .unwrap()
        .value = -360.1;
    assert_rejects(&nutation, "NUTATION");

    let mut period = base;
    period.body.segment.data.spin[0]
        .nutation_per
        .as_mut()
        .unwrap()
        .value = -1.0;
    assert_rejects(&period, "NUTATION_PER");

    let mut momentum = Apm::from_kvn(SPIN_KVN).unwrap();
    let spin = &mut momentum.body.segment.data.spin[0];
    spin.nutation = None;
    spin.nutation_per = None;
    spin.nutation_phase = None;
    spin.momentum_alpha = Some(spin.spin_alpha.clone());
    spin.momentum_delta = Some(spin.spin_delta.clone());
    spin.nutation_vel = Some(spin.spin_angle_vel.clone());
    spin.momentum_delta.as_mut().unwrap().value = 360.0;
    assert_rejects(&momentum, "MOMENTUM_DELTA");
}

/// `EulerAngleState`, `AngVelState`, and `InertiaState` had no validator reached from any root.
/// An Euler angle of 400 degrees passed `validate`, KVN, and XML, and the emitted document was
/// rejected by the reference schema on `[facet 'maxExclusive']`. Found by loosening the Python
/// setter that had been the only guard.
#[test]
fn apm_repeated_attitude_blocks_are_reached_from_the_root() {
    const EULER_KVN: &str = include_str!("../../data/kvn/apm_g2.kvn");
    const INERTIA_KVN: &str = include_str!("../../data/kvn/apm_g3.kvn");

    // One value is enough to prove the route is reached; `angleType`'s domain is exercised once
    // in `types.rs::test_angle_validation` rather than re-proved at every call site.
    let mut message = Apm::from_kvn(EULER_KVN).unwrap();
    message.body.segment.data.euler_angle_state[0].angle_1.value = 400.0;
    assert_rejects(&message, "EULER_ANGLE_1");

    let mut message = Apm::from_kvn(INERTIA_KVN).unwrap();
    message.body.segment.data.inertia[0].iyz.value = f64::INFINITY;
    assert_rejects(&message, "IYZ");

    // The accepted boundary still reaches the reference schema.
    let mut message = Apm::from_kvn(EULER_KVN).unwrap();
    message.body.segment.data.euler_angle_state[0].angle_1.value = -360.0;
    validate_xml("APM Euler boundary", &message.to_xml().unwrap());
}

#[test]
fn kvn_lexical_errors_have_the_same_category_for_string_and_streaming_output() {
    let mut message = Apm::from_kvn(include_str!("../../data/kvn/apm_g1.kvn")).unwrap();
    message.header.originator = "GSFC-é".into();
    let direct = message.to_kvn().unwrap_err();
    let streaming = message.write_kvn_to(&mut Vec::new()).unwrap_err();
    assert!(direct.as_validation_error().is_some());
    assert!(streaming.as_validation_error().is_some());
}
