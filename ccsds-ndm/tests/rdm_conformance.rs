use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::VersionedNdm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/rdm_c2.kvn");
const XML: &str = include_str!("../data/xml/rdm_c4.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn rdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = SPACEOBJECT";
    let designator = "INTERNATIONAL_DESIGNATOR = 2018-099B";
    let lifetime = "ORBIT_LIFETIME = 5.5 [d]";
    for (label, source) in [
        (
            "duplicate keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            KVN.replace(
                &format!("{object_name}\n{designator}"),
                &format!("{designator}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            KVN.replace(lifetime, &format!("{lifetime}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(lifetime, &format!("{lifetime}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert!(Rdm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn rdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let lifetime = "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>";
    let altitude = "<REENTRY_ALTITUDE units=\"km\">80.0</REENTRY_ALTITUDE>";
    for (label, source) in [
        (
            "unknown atmospheric child",
            XML.replace(
                "<atmosphericReentryParameters>",
                "<atmosphericReentryParameters><UNKNOWN>1</UNKNOWN>",
            ),
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
            XML.replace(
                "<groundImpactParameters>",
                "<groundImpactParameters unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                lifetime,
                "<ORBIT_LIFETIME units=\"d\" unexpected=\"value\">5.5</ORBIT_LIFETIME>",
            ),
        ),
        (
            "duplicate element",
            XML.replace(lifetime, &format!("{lifetime}{lifetime}")),
        ),
        (
            "reordered elements",
            XML.replace(
                &format!("{lifetime}\n{altitude}"),
                &format!("{altitude}\n{lifetime}"),
            ),
        ),
    ] {
        assert!(Rdm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn rdm_kvn_preserves_each_logical_blocks_comments() {
    let mut message = Rdm::from_xml(XML).unwrap();
    // KVN COMMENT lexical whitespace is a separator, not message content. Normalize the single
    // leading space present in the Annex C XML sample before comparing semantic models.
    for comment in &mut message
        .body
        .segment
        .data
        .covariance_matrix
        .as_mut()
        .unwrap()
        .comment
    {
        *comment = comment.trim().to_string();
    }
    let kvn = message.to_kvn().unwrap();
    let reparsed = Rdm::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed, message);

    let data = &reparsed.body.segment.data;
    assert_eq!(
        data.ground_impact_parameters.as_ref().unwrap().comment,
        ["Short term re-entry prediction results"]
    );
    assert_eq!(
        data.state_vector.as_ref().unwrap().comment,
        ["State vector at the last OD epoch"]
    );
    assert_eq!(data.covariance_matrix.as_ref().unwrap().comment.len(), 1);
    assert_eq!(
        data.spacecraft_parameters.as_ref().unwrap().comment.len(),
        1
    );
    assert_eq!(data.od_parameters.as_ref().unwrap().comment.len(), 1);
}

#[test]
fn rdm_kvn_rejects_an_xml_only_outer_data_comment_before_writing() {
    let mut message = Rdm::from_xml(XML).unwrap();
    message.body.segment.data.comment.push("outer".into());
    assert!(message.to_kvn().is_err());
    assert!(message.to_kvn().is_err());
    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn every_shipped_rdm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["rdm_c1.kvn", "rdm_c2.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Rdm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Rdm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Rdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    for name in ["rdm_c3.xml", "rdm_c4.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
        let message = Rdm::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Rdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
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
