use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

const ATT_KVN: &str = include_str!("../data/kvn/acm_g7.kvn");
const COV_KVN: &str = include_str!("../data/kvn/acm_g9.kvn");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn acm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object = "OBJECT_NAME = SDO";
    let designator = "INTERNATIONAL_DESIGNATOR = 2010-005A";
    let first_state = "0.000000 0.1153 -0.1424 0.8704 0.4571 2.271e-06 -4.405e-06 -3.785e-06";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            ATT_KVN.replace(object, &format!("{object}\n{object}")),
        ),
        (
            "reordered metadata",
            ATT_KVN.replace(
                &format!("{object}\n{designator}"),
                &format!("{designator}\n{object}"),
            ),
        ),
        (
            "unknown attitude keyword",
            ATT_KVN.replace(
                "ATT_TYPE = QUATERNION",
                "ATT_TYPE = QUATERNION\nUNKNOWN = 1",
            ),
        ),
        (
            "comment after history",
            ATT_KVN.replace(first_state, &format!("{first_state}\nCOMMENT misplaced")),
        ),
        (
            "assignment after history",
            ATT_KVN.replace(
                first_state,
                &format!("{first_state}\nRATE_TYPE = GYRO_BIAS"),
            ),
        ),
        (
            "sensor outside AD",
            ATT_KVN.replace("ATT_STOP", "ATT_STOP\nSENSOR_START\nSENSOR_STOP"),
        ),
        ("mismatched block", ATT_KVN.replace("ATT_STOP", "COV_STOP")),
        (
            "unknown block",
            ATT_KVN.replace("ATT_START", "UNKNOWN_START"),
        ),
        ("trailing assignment", format!("{ATT_KVN}UNKNOWN = value\n")),
    ] {
        assert!(Acm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn acm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let xml = Acm::from_kvn(COV_KVN).unwrap().to_xml().unwrap();
    let object = "<OBJECT_NAME>LRO</OBJECT_NAME>";
    let designator = "<INTERNATIONAL_DESIGNATOR>2009-031A</INTERNATIONAL_DESIGNATOR>";
    for (label, source) in [
        (
            "unknown covariance child",
            xml.replace("<cov>", "<cov><UNKNOWN/>"),
        ),
        (
            "illegal covariance-line attribute",
            xml.replace("<covLine>", "<covLine units=\"1\">"),
        ),
        (
            "unknown sensor attribute",
            xml.replace("<sensorData>", "<sensorData unexpected=\"value\">"),
        ),
        (
            "duplicate covariance type",
            xml.replace("</COV_TYPE>", "</COV_TYPE><COV_TYPE>ANGLE</COV_TYPE>"),
        ),
        (
            "reordered metadata",
            xml.replace(
                &format!("{object}{designator}"),
                &format!("{designator}{object}"),
            ),
        ),
    ] {
        assert!(Acm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_acm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["acm_g6.kvn", "acm_g7.kvn", "acm_g8.kvn", "acm_g9.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Acm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Acm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Acm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }
}

#[test]
fn acm_vectors_and_units_survive_both_notations() {
    let physical = Acm::from_kvn(include_str!("../data/kvn/acm_g8.kvn")).unwrap();
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
    let mut cases: Vec<(&str, Acm)> = Vec::new();
    let mut non_ascii = Acm::from_kvn(ATT_KVN).unwrap();
    non_ascii.body.segment.metadata.object_name = "ST5 €".to_owned();
    cases.push(("non-ASCII text", non_ascii));
    let mut overlong = Acm::from_kvn(ATT_KVN).unwrap();
    overlong.body.segment.metadata.object_name = "X".repeat(240);
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
fn kvn_generation_rounds_history_numbers_to_the_ccsds_digit_limit() {
    let mut message = Acm::from_kvn(ATT_KVN).unwrap();
    message.body.segment.data.att[0].att_lines[0].values[1] = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));

    let mut physical = Acm::from_kvn(include_str!("../data/kvn/acm_g8.kvn")).unwrap();
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

    assert!(message.to_xml().is_err());
    let mut output = Vec::new();
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty());
}

fn validate_xml(label: &str, xml: &str) {
    let document = NamedTempFile::new().unwrap();
    fs::write(document.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(repository_path("data/xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
