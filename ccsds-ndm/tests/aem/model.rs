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

    // ADM 4.2.4.5: every record must match the segment's ATTITUDE_TYPE. `AemData` has no
    // metadata, so the segment is the single validation path for this rule.
    use ccsds_ndm::Ndm;
    let base = ccsds_ndm::messages::aem::Aem::from_kvn(crate::KVN)
        .unwrap()
        .body
        .segment[0]
        .clone();
    let check = |attitude_type: AttitudeTypeType, state: &AemAttitudeState| {
        let mut segment = base.clone();
        segment.metadata.attitude_type = attitude_type.clone();
        segment.data.attitude_states = vec![state.clone()];
        assert_validation_field(
            &segment.validate().unwrap_err(),
            &format!("expected {attitude_type} data"),
        );
    };
    check(AttitudeTypeType::Quaternion, &valid_euler);
    for attitude_type in [
        AttitudeTypeType::EulerAngle,
        AttitudeTypeType::QuaternionDerivative,
        AttitudeTypeType::QuaternionAngVel,
        AttitudeTypeType::EulerAngleDerivative,
        AttitudeTypeType::EulerAngleAngVel,
        AttitudeTypeType::Spin,
        AttitudeTypeType::SpinNutation,
        AttitudeTypeType::SpinNutationMom,
    ] {
        check(attitude_type, &valid_q);
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
