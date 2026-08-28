use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/omm_g9.kvn");
const XML: &str = include_str!("../data/xml/omm_g10.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn assert_kvn_rejected(label: &str, source: String) {
    assert!(Omm::from_kvn(&source).is_err(), "accepted {label}");
}

#[test]
fn omm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = GOES 9";
    let object_id = "OBJECT_ID = 1995-025A";
    for (label, source) in [
        (
            "duplicate keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered keywords",
            KVN.replace(
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            KVN.replace(object_name, &format!("{object_name}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(object_name, &format!("{object_name}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert_kvn_rejected(label, source);
    }
}

#[test]
fn omm_xml_rejects_unknown_nested_content_and_ordering_errors() {
    let epoch = "<EPOCH>2020-064T10:34:41.4264</EPOCH>";
    let mean_motion = "<MEAN_MOTION>1.00273272</MEAN_MOTION>";
    for (label, source) in [
        (
            "unknown mean-elements child",
            XML.replace("<meanElements>", "<meanElements><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown TLE child",
            XML.replace("<tleParameters>", "<tleParameters><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown mean-elements attribute",
            XML.replace("<meanElements>", "<meanElements unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                epoch,
                "<EPOCH unexpected=\"value\">2020-064T10:34:41.4264</EPOCH>",
            ),
        ),
        (
            "duplicate element",
            XML.replace(epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered elements",
            XML.replace(
                &format!("{epoch}\n{mean_motion}"),
                &format!("{mean_motion}\n{epoch}"),
            ),
        ),
    ] {
        assert!(Omm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_omm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["omm_g7.kvn", "omm_g8.kvn", "omm_g9.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Omm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Omm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Omm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Omm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Omm::from_xml(&xml).unwrap(), message);
    validate_xml("omm_g10.xml", &xml);
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
