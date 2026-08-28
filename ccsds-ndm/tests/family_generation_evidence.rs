use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::{from_str, MessageType};
use tempfile::NamedTempFile;

const REMAINING_PREFIXES: [&str; 8] = [
    "omm_", "ocm_", "cdm_", "tdm_", "rdm_", "aem_", "apm_", "acm_",
];

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn fixture_paths(directory: &str, extension: &str) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(repository_path(directory))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|value| value == extension))
        .filter(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|name| {
                    REMAINING_PREFIXES
                        .iter()
                        .any(|prefix| name.starts_with(prefix))
                })
        })
        .collect();
    paths.sort();
    paths
}

fn validate_official_xsd(label: &str, xml: &str) {
    let document = NamedTempFile::new().unwrap();
    fs::write(document.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(repository_path("data/xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance evidence: {error}"));
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_standalone_generation(label: &str, message: &MessageType) {
    let kvn = message
        .to_kvn()
        .unwrap_or_else(|error| panic!("{label} KVN generation failed: {error}"));
    assert_eq!(message.to_kvn().unwrap(), kvn, "{label} KVN changed");
    let reparsed_kvn = from_str(&kvn).unwrap();
    assert_eq!(reparsed_kvn.kind(), message.kind());

    let xml = message
        .to_xml()
        .unwrap_or_else(|error| panic!("{label} XML generation failed: {error}"));
    assert_eq!(message.to_xml().unwrap(), xml, "{label} XML changed");
    validate_official_xsd(label, &xml);
    let reparsed_xml = from_str(&xml).unwrap();
    assert_eq!(reparsed_xml.kind(), message.kind());
}

#[test]
fn every_remaining_kvn_fixture_generates_deterministically_and_reparsably() {
    for path in fixture_paths("data/kvn", "kvn") {
        let label = path.file_name().unwrap().to_string_lossy();
        let input = fs::read_to_string(&path).unwrap();
        let message =
            from_str(&input).unwrap_or_else(|error| panic!("{label} strict parse failed: {error}"));
        assert_standalone_generation(&label, &message);
    }
}

#[test]
fn every_remaining_xml_fixture_generates_deterministically_and_reparsably() {
    for path in fixture_paths("data/xml", "xml") {
        let label = path.file_name().unwrap().to_string_lossy();
        let input = fs::read_to_string(&path).unwrap();
        let message =
            from_str(&input).unwrap_or_else(|error| panic!("{label} strict parse failed: {error}"));
        if label == "tdm_e21.xml" || label == "cdm_44.xml" {
            // XML permits Unicode strings; TDM KVN is restricted to printable ASCII. The
            // reference participant name contains typographic quotes, so conversion must fail
            // rather than emit KVN that the strict parser cannot accept.
            //
            // The CDM XML fixture independently populates both the outer data COMMENT and its
            // first nested OD COMMENT. CDM KVN has no delimiter for that boundary, so conversion
            // must likewise reject the ambiguous state instead of guessing a split.
            assert!(message.to_kvn().is_err());
            let xml = message.to_xml().unwrap();
            assert_eq!(message.to_xml().unwrap(), xml);
            validate_official_xsd(&label, &xml);
            assert_eq!(from_str(&xml).unwrap().kind(), message.kind());
            continue;
        }
        assert_standalone_generation(&label, &message);
    }
}

#[test]
fn acm_physical_description_survives_kvn_to_xml_conversion() {
    let input = fs::read_to_string(repository_path("data/kvn/acm_g8.kvn")).unwrap();
    let message = from_str(&input).unwrap();
    let xml = message.to_xml().unwrap();
    let reparsed = from_str(&xml).unwrap();
    let MessageType::Acm(acm) = reparsed else {
        panic!("generated ACM XML changed message type");
    };
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
fn aem_optional_xml_unit_annotations_are_normatively_normalized_through_kvn() {
    let input = fs::read_to_string(repository_path("data/xml/aem_g13.xml")).unwrap();
    assert!(input.contains("<NUTATION units=\"deg\">"));

    let message = from_str(&input).unwrap();
    let kvn = message.to_kvn().unwrap();
    // CCSDS 504.0-B-2 section 6.9.2 forbids units in AEM KVN data lines.
    assert!(!kvn.contains("[deg]"));
    assert!(!kvn.contains("[deg/s]"));

    let normalized_xml = from_str(&kvn).unwrap().to_xml().unwrap();
    // Section 7.6.10 makes these fixed XML unit annotations optional.
    assert!(normalized_xml.contains("<NUTATION>2</NUTATION>"));
    assert!(!normalized_xml.contains("<NUTATION units="));
    validate_official_xsd("AEM optional unit normalization", &normalized_xml);
}

#[test]
fn cdm_kvn_comments_keep_their_normative_block_association() {
    let input = fs::read_to_string(repository_path("data/kvn/cdm_363.kvn")).unwrap();
    let message = from_str(&input).unwrap();
    let MessageType::Cdm(cdm) = message else {
        panic!("CDM fixture changed message type");
    };

    assert_eq!(
        cdm.body.relative_metadata_data.comment,
        ["Relative Metadata/Data"]
    );

    let first = &cdm.body.segments[0];
    assert_eq!(first.metadata.comment, ["Object1 Metadata"]);
    // KVN has no delimiter between the outer data comments and the first nested block's
    // comments. Preserve the leading run and its position without guessing a split.
    assert_eq!(
        first.data.comment,
        ["Object1 Data", "Object1 OD Parameters"]
    );
    assert!(first
        .data
        .od_parameters
        .as_ref()
        .unwrap()
        .comment
        .is_empty());
    assert_eq!(
        first.data.additional_parameters.as_ref().unwrap().comment,
        [
            "Object1 Additional Parameters",
            "Apogee Altitude=779 km",
            "Perigee Altitude=765 km",
            "Inclination=86.4 deg",
        ]
    );
    assert_eq!(first.data.state_vector.comment, ["Object1 State Vector"]);
    assert_eq!(
        first.data.covariance_matrix.as_ref().unwrap().comment,
        ["Object1 Covariance in the RTN Coordinate Frame"]
    );

    let original = MessageType::Cdm(cdm.clone());
    let regenerated = original.to_kvn().unwrap();
    assert_eq!(from_str(&regenerated).unwrap(), original);
}
