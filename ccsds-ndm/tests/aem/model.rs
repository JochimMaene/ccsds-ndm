use crate::common::assert_validation_field;
use ccsds_ndm::messages::aem::{AemBody, AemData};
use ccsds_ndm::types::{Angle, AttitudeTypeType};
use ccsds_ndm::Validate;
#[test]
fn aem_data_validation_mismatches() {
    use ccsds_ndm::common::*;

    let valid_q = AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
        epoch: "2023-01-01T00:00:00".parse().unwrap(),
        quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
    });

    let valid_euler = AemAttitudeState::EulerAngle(EulerAngle {
        epoch: "2023-01-01T00:00:00".parse().unwrap(),
        angle_1: Angle::new(10.0, None).unwrap(),
        angle_2: Angle::new(20.0, None).unwrap(),
        angle_3: Angle::new(30.0, None).unwrap(),
    });

    // Type mismatch: Expects QUATERNION, gets EULER_ANGLE
    let data = AemData {
        comment: vec![],
        attitude_states: vec![valid_euler.clone()],
    };
    assert_validation_field(
        &data.validate(&AttitudeTypeType::Quaternion).unwrap_err(),
        "expected QUATERNION data",
    );

    // Type mismatch: Expects EULER_ANGLE, gets QUATERNION
    let data_q = AemData {
        comment: vec![],
        attitude_states: vec![valid_q.clone()],
    };
    assert_validation_field(
        &data_q.validate(&AttitudeTypeType::EulerAngle).unwrap_err(),
        "expected EULER_ANGLE data",
    );

    // Check all other variants against a wrong type declaration
    let cases = vec![
        AttitudeTypeType::QuaternionDerivative,
        AttitudeTypeType::QuaternionAngVel,
        AttitudeTypeType::EulerAngleDerivative,
        AttitudeTypeType::EulerAngleAngVel,
        AttitudeTypeType::Spin,
        AttitudeTypeType::SpinNutation,
        AttitudeTypeType::SpinNutationMom,
    ];

    for attitude_type in cases {
        let d = AemData {
            comment: vec![],
            attitude_states: vec![valid_q.clone()],
        };
        assert_validation_field(
            &d.validate(&attitude_type).unwrap_err(),
            &format!("expected {attitude_type} data"),
        );
    }
}

#[test]
fn aem_data_requires_attitude_state() {
    let data = AemData {
        comment: vec![],
        attitude_states: vec![],
    };
    assert_validation_field(
        &ccsds_ndm::Validate::validate(&data).unwrap_err(),
        "attitudeState (at least one required)",
    );
}

#[test]
fn aem_body_requires_segment() {
    let body = AemBody { segment: vec![] };
    assert_validation_field(
        &body.validate().unwrap_err(),
        "segment (at least one required)",
    );
}
