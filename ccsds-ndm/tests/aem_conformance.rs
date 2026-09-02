use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::VersionedNdm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/aem_g4.kvn");
const SPIN_KVN: &str = include_str!("../data/kvn/aem_g5.kvn");
const XML: &str = include_str!("../data/xml/aem_g13.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

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
fn aem_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object = "OBJECT_NAME = MARS GLOBAL SURVEYOR";
    let object_id = "OBJECT_ID = 1996-062A";
    let first_state = "1996-11-28T21:29:07.2555 0.56748 0.03146 0.45689 0.68427";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            KVN.replace(object, &format!("{object}\n{object}")),
        ),
        (
            "reordered metadata",
            KVN.replace(
                &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
        (
            "unknown metadata",
            KVN.replace(object, &format!("{object}\nUNKNOWN = value")),
        ),
        (
            "comment after history begins",
            KVN.replace(first_state, &format!("{first_state}\nCOMMENT misplaced")),
        ),
        ("mismatched block", KVN.replace("META_STOP", "DATA_STOP")),
        ("unknown block", KVN.replace("DATA_START", "UNKNOWN_START")),
        (
            "XML-only metadata keyword in KVN",
            KVN.replace(
                "ATTITUDE_TYPE = QUATERNION",
                "ATTITUDE_TYPE = QUATERNION\nANGVEL_FRAME = A",
            ),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
    ] {
        assert!(Aem::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn aem_xml_rejects_unknown_choice_content_attributes_and_ordering_errors() {
    let object = "<OBJECT_NAME>TEST</OBJECT_NAME>";
    let object_id = "<OBJECT_ID>2000-999Z</OBJECT_ID>";
    let epoch = "<EPOCH>2000-100T00:00:00.000</EPOCH>";
    for (label, source) in [
        (
            "unknown attitude state choice",
            XML.replace("<attitudeState>", "<attitudeState><UNKNOWN/>"),
        ),
        (
            "two attitude state choices",
            XML.replace(
                "</quaternionEphemeris>",
                "</quaternionEphemeris><spin><EPOCH>2000-100T00:00:00.000</EPOCH><SPIN_ALPHA>1</SPIN_ALPHA><SPIN_DELTA>2</SPIN_DELTA><SPIN_ANGLE>3</SPIN_ANGLE><SPIN_ANGLE_VEL>4</SPIN_ANGLE_VEL></spin>",
            ),
        ),
        (
            "unknown container attribute",
            XML.replace("<attitudeState>", "<attitudeState unexpected=\"value\">"),
        ),
        (
            "illegal quaternion units",
            XML.replace("<Q1>-0.005068</Q1>", "<Q1 units=\"1\">-0.005068</Q1>"),
        ),
        (
            "duplicate epoch",
            XML.replace(epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered metadata",
            XML.replace(
                &format!("{object}\n{object_id}"),
                &format!("{object_id}\n{object}"),
            ),
        ),
    ] {
        assert!(Aem::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_aem_fixture_preserves_states_and_generates_valid_xml() {
    for name in ["aem_g4.kvn", "aem_g5.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Aem::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Aem::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Aem::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }
    for name in ["aem_g11.xml", "aem_g13.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
        let message = Aem::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Aem::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
        let kvn = message.to_kvn().unwrap();
        let normalized = Aem::from_kvn(&kvn).unwrap();
        assert_eq!(normalized.body.segment.len(), message.body.segment.len());
        assert_eq!(
            normalized
                .body
                .segment
                .iter()
                .map(|segment| segment.data.attitude_states.len())
                .collect::<Vec<_>>(),
            message
                .body
                .segment
                .iter()
                .map(|segment| segment.data.attitude_states.len())
                .collect::<Vec<_>>()
        );
    }
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
