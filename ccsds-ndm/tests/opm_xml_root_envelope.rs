// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
use std::path::{Path, PathBuf};
use std::process::Command;

const OPM: &str = include_str!("../data/kvn/opm_g1.kvn");
const ROOT: &str = concat!(
    "<opm xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" ",
    "id=\"CCSDS_OPM_VERS\" version=\"3.0\">"
);

fn opm() -> Opm {
    Opm::from_kvn(OPM).expect("the OPM fixture must remain valid")
}

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn assert_opm_envelope(xml: &str) {
    let body = xml
        .strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
        .expect("generated XML omitted the required declaration");
    assert!(
        body.starts_with(ROOT),
        "generated OPM root did not contain the required namespace followed by final id/version attributes: {xml}"
    );
}

fn assert_xsd_valid(xml: &str) {
    let generated = tempfile::NamedTempFile::new().expect("failed to create temporary XML file");
    std::fs::write(generated.path(), xml).expect("failed to write generated XML");
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema_path())
        .arg(generated.path())
        .output()
        .unwrap_or_else(|error| {
            panic!("xmllint is required for conformance tests; install libxml2-utils: {error}")
        });
    assert!(
        output.status.success(),
        "generated OPM failed the official XSD:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn typed_opm_string_generation_emits_the_required_root_namespace() {
    let opm = opm();
    let xml = opm.to_xml().expect("typed XML generation failed");
    assert_opm_envelope(&xml);
    assert_xsd_valid(&xml);

    let versioned = opm.to_xml().expect("versioned XML generation failed");
    assert_eq!(versioned, xml);
}

#[test]
fn streamed_opm_generation_emits_the_required_root_namespace() {
    let opm = opm();
    let mut output = Vec::new();
    opm.write_xml_to(&mut output)
        .expect("streamed XML generation failed");
    let xml = String::from_utf8(output).expect("generated XML was not UTF-8");

    assert_opm_envelope(&xml);
    assert_eq!(xml, opm.to_xml().unwrap());
}
