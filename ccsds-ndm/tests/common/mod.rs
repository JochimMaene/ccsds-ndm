// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("data")
}

/// Asserts that an invalid model is refused by every public output path.
///
/// One contract instead of a per-family copy. The count had already drifted — some families
/// asserted the streaming zero-byte guarantee and others did not — which is exactly the failure a
/// shared helper prevents.
///
/// `field` must appear in the diagnostic, so a test cannot pass because some *other* rule rejected
/// the model.
#[allow(dead_code)]
pub fn assert_rejects<M>(message: &M, field: &str)
where
    M: ccsds_ndm::Ndm + ccsds_ndm::Validate,
{
    let error = ccsds_ndm::Validate::validate(message)
        .expect_err("invalid edited value accepted by validate");
    assert!(
        error.to_string().contains(field),
        "diagnostic for {field} did not name it: {error}"
    );
    assert!(message.to_kvn().is_err(), "KVN accepted invalid {field}");
    assert!(message.to_xml().is_err(), "XML accepted invalid {field}");

    let mut output = Vec::new();
    assert!(
        message.write_kvn_to(&mut output).is_err(),
        "streaming KVN accepted invalid {field}"
    );
    assert!(output.is_empty(), "streaming KVN wrote bytes for {field}");
    assert!(
        message.write_xml_to(&mut output).is_err(),
        "streaming XML accepted invalid {field}"
    );
    assert!(output.is_empty(), "streaming XML wrote bytes for {field}");
}

/// Validates generated XML against the bundled reference schema.
///
/// libxml2 establishes structure, ordering and lexical form. It accepts NaN against bounding
/// facets, so this is not evidence of numeric validity; see the XSD oracle policy in the
/// validation contract.
#[allow(dead_code)]
pub fn validate_xml(label: &str, xml: &str) {
    let document = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(document.path(), xml).unwrap();
    let output = std::process::Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(data_dir().join("xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance evidence: {error}"));
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
