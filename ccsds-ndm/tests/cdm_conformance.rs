use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, VersionedNdm};
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../../data/kvn/cdm_363.kvn");
const XML: &str = include_str!("../../data/xml/cdm_44.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

#[test]
fn cdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let tca = "TCA = 2010-03-13T22:37:52.618";
    let miss = "MISS_DISTANCE = 715 [m]";
    let object = "OBJECT = OBJECT1";
    for (label, source) in [
        (
            "duplicate relative keyword",
            KVN.replace(miss, &format!("{miss}\n{miss}")),
        ),
        (
            "reordered relative keywords",
            KVN.replace(&format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "unknown metadata keyword",
            KVN.replace(object, &format!("{object}\nUNKNOWN = value")),
        ),
        (
            "comment inside relative block",
            KVN.replace(miss, &format!("{miss}\nCOMMENT misplaced")),
        ),
        ("unknown marked block", KVN.replace(object, "META_START")),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object, "OBJECT = OBJECT1 €"),
        ),
    ] {
        assert!(Cdm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn cdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let tca = "<TCA>2010-03-13T22:37:52.618</TCA>";
    let miss = "<MISS_DISTANCE units=\"m\">715</MISS_DISTANCE>";
    let object = "<OBJECT>OBJECT1</OBJECT>";
    for (label, source) in [
        (
            "unknown data child",
            XML.replace("<data>", "<data><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown covariance child",
            XML.replace(
                "<covarianceMatrix>",
                "<covarianceMatrix><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown container attribute",
            XML.replace("<segment>", "<segment unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                miss,
                "<MISS_DISTANCE units=\"m\" unexpected=\"value\">715</MISS_DISTANCE>",
            ),
        ),
        (
            "duplicate metadata child",
            XML.replace(object, &format!("{object}{object}")),
        ),
        (
            "reordered relative children",
            XML.replace(&format!("{tca}\n{miss}"), &format!("{miss}\n{tca}")),
        ),
        (
            "illegal nil attribute",
            XML.replace(tca, "<TCA nil=\"true\"/>"),
        ),
    ] {
        assert!(Cdm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_cdm_fixture_preserves_typed_content_and_generates_valid_xml() {
    for name in ["cdm_362.kvn", "cdm_363.kvn", "cdm_364.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Cdm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Cdm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Cdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Cdm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Cdm::from_xml(&xml).unwrap(), message);
    validate_xml("cdm_44.xml", &xml);
    assert!(
        message.to_kvn().is_err(),
        "ambiguous XML comment associations crossed into KVN"
    );
}

#[test]
fn every_kvn_generation_gate_rejects_loss_or_ambiguity_before_output() {
    let cases: [(&str, fn(&mut Cdm)); 3] = [
        ("first nested comment", |message| {
            message.body.segments[0]
                .data
                .od_parameters
                .as_mut()
                .unwrap()
                .comment
                .push("nested".to_owned());
        }),
        ("non-ASCII free text", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE €".to_owned();
        }),
        ("lossy multiline free text", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE\nA".to_owned();
        }),
    ];
    for (label, mutate) in cases {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        mutate(&mut message);
        assert!(message.to_kvn().is_err(), "materialized accepted {label}");
        let mut output = Vec::new();
        assert!(
            message
                .write_kvn_to(&mut output, &GenerateOptions::source())
                .is_err(),
            "streaming accepted {label}"
        );
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
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
