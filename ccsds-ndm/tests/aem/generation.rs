use crate::common::{assert_rejects, validate_xml};
use crate::{KVN, SPIN_KVN, XML};
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::Ndm;

#[test]
fn aem_xml_emits_canonical_uppercase_attitude_types() {
    let lowercase = XML
        .replace("QUATERNION/DERIVATIVE", "quaternion/derivative")
        .replace("QUATERNION/ANGVEL", "quaternion/angvel")
        .replace("QUATERNION", "quaternion");
    let generated = Aem::from_xml(&lowercase).unwrap().to_xml().unwrap();

    for value in ["QUATERNION", "QUATERNION/DERIVATIVE", "QUATERNION/ANGVEL"] {
        assert!(generated.contains(&format!("<ATTITUDE_TYPE>{value}</ATTITUDE_TYPE>")));
    }
    assert!(!generated.contains("<ATTITUDE_TYPE>quaternion"));
}

#[test]
fn every_kvn_generation_gate_rejects_invalid_state_before_output() {
    let mut cases: Vec<(&str, Aem)> = Vec::new();
    let mut non_ascii = Aem::from_kvn(SPIN_KVN).unwrap();
    non_ascii.body.segment[0].metadata.object_name = "ST5 €".to_owned();
    cases.push(("non-ASCII text", non_ascii));
    let mut overlong = Aem::from_kvn(SPIN_KVN).unwrap();
    overlong.body.segment[0].metadata.object_name = "X".repeat(240);
    cases.push(("overlong record", overlong));
    for (label, message) in cases {
        assert!(message.to_kvn().is_err(), "materialized accepted {label}");
        let mut output = Vec::new();
        assert!(
            message.write_kvn_to(&mut output).is_err(),
            "streaming accepted {label}"
        );
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
}

#[test]
fn every_aem_attitude_choice_revalidates_edited_numeric_values() {
    let mut cases = Vec::new();
    let mut repeated = Aem::from_kvn(KVN).unwrap();
    repeated.body.segment[0].data.attitude_states[1]
        .quaternion_ephemeris
        .as_mut()
        .unwrap()
        .quaternion
        .q1 = f64::NAN;
    cases.push(("Quaternion", repeated));

    let base = Aem::from_xml(XML).unwrap();
    let mut value = base.clone();
    value.body.segment[1].data.attitude_states[0]
        .quaternion_derivative
        .as_mut()
        .unwrap()
        .quaternion_dot
        .q1_dot
        .value = f64::NAN;
    cases.push(("Q1_DOT", value));
    let mut value = base.clone();
    value.body.segment[2].data.attitude_states[0]
        .quaternion_ang_vel
        .as_mut()
        .unwrap()
        .ang_vel
        .angvel_x
        .value = f64::INFINITY;
    cases.push(("ANGVEL_X", value));
    let mut value = base.clone();
    value.body.segment[3].data.attitude_states[0]
        .euler_angle
        .as_mut()
        .unwrap()
        .angle_1
        .value = 360.0;
    cases.push(("ANGLE_1", value));
    let mut value = base.clone();
    value.body.segment[4].data.attitude_states[0]
        .euler_angle_derivative
        .as_mut()
        .unwrap()
        .angle_1_dot
        .value = f64::NAN;
    cases.push(("ANGLE_1_DOT", value));
    let mut value = base.clone();
    value.body.segment[5].data.attitude_states[0]
        .euler_angle_ang_vel
        .as_mut()
        .unwrap()
        .angvel_x
        .value = f64::NEG_INFINITY;
    cases.push(("ANGVEL_X", value));
    let mut value = base.clone();
    value.body.segment[6].data.attitude_states[0]
        .spin
        .as_mut()
        .unwrap()
        .spin_alpha
        .value = 360.0;
    cases.push(("SPIN_ALPHA", value));
    let mut value = base.clone();
    value.body.segment[7].data.attitude_states[0]
        .spin_nutation
        .as_mut()
        .unwrap()
        .nutation_per
        .value = -1.0;
    cases.push(("NUTATION_PER", value));
    let mut value = base;
    value.body.segment[8].data.attitude_states[0]
        .spin_nutation_mom
        .as_mut()
        .unwrap()
        .nutation_vel
        .value = f64::NAN;
    cases.push(("NUTATION_VEL", value));

    for (field, message) in cases {
        assert_rejects(&message, field);
    }
}

#[test]
fn aem_angle_and_nutation_duration_boundaries_generate_valid_xml() {
    let mut message = Aem::from_xml(XML).unwrap();
    message.body.segment[3].data.attitude_states[0]
        .euler_angle
        .as_mut()
        .unwrap()
        .angle_1
        .value = -360.0;
    message.body.segment[7].data.attitude_states[0]
        .spin_nutation
        .as_mut()
        .unwrap()
        .nutation_per
        .value = 0.0;
    let xml = message.to_xml().unwrap();
    validate_xml("AEM numeric boundaries", &xml);
}

#[test]
fn kvn_generation_rounds_history_numbers_to_the_ccsds_digit_limit() {
    let mut message = Aem::from_kvn(SPIN_KVN).unwrap();
    message.body.segment[0].data.attitude_states[0]
        .spin
        .as_mut()
        .unwrap()
        .spin_alpha
        .value = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));
}
