use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/apm_g1.kvn");
const XML: &str = include_str!("../data/xml/apm_g10.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn apm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = TRMM";
    let object_id = "OBJECT_ID = 1997-074A";
    let q1 = "Q1 = 0.00005";
    for (label, source) in [
        (
            "duplicate top-level keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            KVN.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "duplicate logical-block keyword",
            KVN.replace(q1, &format!("{q1}\n{q1}")),
        ),
        (
            "unknown logical-block keyword",
            KVN.replace(q1, &format!("{q1}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(q1, &format!("{q1}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("QUAT_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            KVN.replace("QUAT_STOP", "SPIN_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert!(Apm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn apm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2003-09-30T14:28:15.1172</EPOCH>";
    let q1 = "<Q1>0.00005</Q1>";
    for (label, source) in [
        (
            "unknown quaternion-state child",
            XML.replace("<quaternionState>", "<quaternionState><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown quaternion child",
            XML.replace("<quaternion>", "<quaternion><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            XML.replace(
                "<quaternionState>",
                "<quaternionState unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            XML.replace(q1, "<Q1 unexpected=\"value\">0.00005</Q1>"),
        ),
        (
            "duplicate epoch",
            XML.replace(epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered quaternion components",
            XML.replace(
                "<Q1>0.00005</Q1>\n<Q2>0.87543</Q2>",
                "<Q2>0.87543</Q2>\n<Q1>0.00005</Q1>",
            ),
        ),
    ] {
        assert!(Apm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_apm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["apm_g1.kvn", "apm_g2.kvn", "apm_g3.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Apm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Apm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Apm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Apm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Apm::from_xml(&xml).unwrap(), message);
    validate_xml("apm_g10.xml", &xml);
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
