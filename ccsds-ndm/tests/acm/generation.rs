use crate::common::{assert_rejects, mutated, validate_xml};
use crate::{ATT_KVN, COV_KVN};
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::types::{AngleRate, Vec4Double};
use ccsds_ndm::Ndm;
#[test]
fn acm_vectors_and_units_survive_both_notations() {
    let physical = Acm::from_kvn(include_str!("../../data/kvn/acm_g8.kvn")).unwrap();
    let normalized = Acm::from_xml(&physical.to_xml().unwrap()).unwrap();
    let cp = normalized.body.segment.data.phys.unwrap().cp.unwrap();
    assert_eq!(cp.elements, [0.04, -0.78, -0.023]);
    assert_eq!(cp.units.unwrap().to_string(), "m");

    let maneuver = Acm::from_kvn(ATT_KVN).unwrap();
    let normalized = Acm::from_xml(&maneuver.to_xml().unwrap()).unwrap();
    let momentum = normalized.body.segment.data.man[0]
        .target_momentum
        .as_ref()
        .unwrap();
    assert_eq!(momentum.elements, [1.3, -16.4, -11.35]);
}

#[test]
fn every_kvn_generation_gate_rejects_invalid_state_before_output() {
    let mut non_ascii = Acm::from_kvn(ATT_KVN).unwrap();
    non_ascii.body.segment.metadata.object_name = "ST5 €".to_owned();
    crate::common::assert_validation_field(&non_ascii.to_kvn().unwrap_err(), "printable ASCII");
    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &non_ascii.write_kvn_to(&mut output).unwrap_err(),
        "printable ASCII",
    );
    assert!(output.is_empty());
}

#[test]
fn acm_accepts_arbitrary_line_lengths() {
    let long_name = "X".repeat(300);
    let source = mutated(
        ATT_KVN,
        "OBJECT_NAME = SDO",
        &format!("OBJECT_NAME = {long_name}"),
    );
    let message = Acm::from_kvn(&source).unwrap();
    let generated = message.to_kvn().unwrap();
    assert!(generated.lines().any(|line| line.len() > 254));
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, generated.as_bytes());
}

#[test]
fn acm_physical_and_maneuver_values_are_revalidated_after_edits() {
    let physical = Acm::from_kvn(include_str!("../../data/kvn/acm_g8.kvn")).unwrap();
    let mut value = physical.clone();
    value.body.segment.data.phys.as_mut().unwrap().drag_coeff = Some(f64::NAN);
    assert_rejects(&value, "DRAG_COEFF");
    let mut value = physical.clone();
    value
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .wet_mass
        .as_mut()
        .unwrap()
        .value = -1.0;
    assert_rejects(&value, "WET_MASS");
    let mut value = physical.clone();
    let phys = value.body.segment.data.phys.as_mut().unwrap();
    phys.dry_mass = phys.wet_mass.clone();
    phys.dry_mass.as_mut().unwrap().value = f64::INFINITY;
    assert_rejects(&value, "DRY_MASS");
    let mut value = physical.clone();
    value
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .cp
        .as_mut()
        .unwrap()
        .elements[1] = f64::NAN;
    assert_rejects(&value, "CP");
    for field in ["IXX", "IYY", "IZZ", "IXY", "IXZ", "IYZ"] {
        let mut value = physical.clone();
        let phys = value.body.segment.data.phys.as_mut().unwrap();
        match field {
            "IXX" => phys.ixx.as_mut().unwrap().value = f64::NAN,
            "IYY" => phys.iyy.as_mut().unwrap().value = f64::NAN,
            "IZZ" => phys.izz.as_mut().unwrap().value = f64::NAN,
            "IXY" => phys.ixy.as_mut().unwrap().value = f64::NAN,
            "IXZ" => phys.ixz.as_mut().unwrap().value = f64::NAN,
            "IYZ" => phys.iyz.as_mut().unwrap().value = f64::NAN,
            _ => unreachable!(),
        }
        assert_rejects(&value, field);
    }

    let maneuver = Acm::from_kvn(ATT_KVN).unwrap();
    let mut value = maneuver.clone();
    value.body.segment.data.man[0]
        .man_duration
        .as_mut()
        .unwrap()
        .value = -1.0;
    assert_rejects(&value, "MAN_DURATION");
    let mut value = maneuver.clone();
    value.body.segment.data.man[0]
        .target_momentum
        .as_mut()
        .unwrap()
        .elements[1] = f64::NAN;
    assert_rejects(&value, "TARGET_MOMENTUM");
    let mut value = maneuver.clone();
    let man = &mut value.body.segment.data.man[0];
    man.target_momentum = None;
    man.target_mom_frame = None;
    man.target_attitude = Some(Vec4Double {
        values: vec![0.0, 0.0, 0.0, f64::NAN],
    });
    assert_rejects(&value, "TARGET_ATTITUDE");
    let mut value = maneuver;
    let man = &mut value.body.segment.data.man[0];
    man.target_momentum = None;
    man.target_mom_frame = None;
    man.target_spinrate = Some(AngleRate {
        value: f64::INFINITY,
        units: None,
    });
    assert_rejects(&value, "TARGET_SPINRATE");
}

#[test]
fn acm_mass_and_duration_zero_boundaries_generate_valid_xml() {
    let mut physical = Acm::from_kvn(include_str!("../../data/kvn/acm_g8.kvn")).unwrap();
    physical
        .body
        .segment
        .data
        .phys
        .as_mut()
        .unwrap()
        .wet_mass
        .as_mut()
        .unwrap()
        .value = 0.0;
    validate_xml("ACM zero mass", &physical.to_xml().unwrap());

    let mut maneuver = Acm::from_kvn(ATT_KVN).unwrap();
    maneuver.body.segment.data.man[0]
        .man_duration
        .as_mut()
        .unwrap()
        .value = 0.0;
    validate_xml("ACM zero duration", &maneuver.to_xml().unwrap());
}

#[test]
fn kvn_generation_rounds_history_numbers_to_the_ccsds_digit_limit() {
    let mut message = Acm::from_kvn(ATT_KVN).unwrap();
    message.body.segment.data.att[0].att_lines[0].values[1] = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));

    let mut physical = Acm::from_kvn(include_str!("../../data/kvn/acm_g8.kvn")).unwrap();
    physical.body.segment.data.phys.as_mut().unwrap().drag_coeff = Some(1.234_567_890_123_456_7);
    assert!(physical
        .to_kvn()
        .unwrap()
        .contains("DRAG_COEFF           = 1.234567890123457e0"));
}

#[test]
fn xml_generation_rejects_kvn_only_sensor_comments_before_output() {
    let mut message = Acm::from_kvn(COV_KVN).unwrap();
    message.body.segment.data.ad.as_mut().unwrap().sensors[0]
        .comment
        .push("KVN sensor comment".into());

    crate::common::assert_validation_field(&message.to_xml().unwrap_err(), "SENSOR COMMENT");
    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &message.write_xml_to(&mut output).unwrap_err(),
        "SENSOR COMMENT",
    );
    assert!(output.is_empty());
}

#[test]
fn acm_physical_description_survives_kvn_to_xml_conversion() {
    let message = Acm::from_kvn(include_str!("../../data/kvn/acm_g8.kvn")).unwrap();
    let xml = message.to_xml().unwrap();
    let acm = Acm::from_xml(&xml).unwrap();
    let physical = acm
        .body
        .segment
        .data
        .phys
        .expect("ACM physical description was dropped");
    assert_eq!(physical.wet_mass.unwrap().value, 1916.0);
    assert_eq!(physical.cp_ref_frame.as_deref(), Some("SC_BODY_1"));
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
