// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::{oem::Oem, omm::Omm, opm::Opm};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, VersionedNdm};
use std::path::Path;
use std::process::Command;

fn validate_with_odm_2_xsd(family: &str, xml: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/xsd");
    let family_schema = root.join(format!("ndmxml-2.0.0-{family}-2.0.xsd"));
    let wrapper = tempfile::NamedTempFile::new().expect("temporary wrapper schema");
    std::fs::write(
        wrapper.path(),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:ndm="urn:ccsds:schema:ndmxml">
  <xsd:import namespace="urn:ccsds:schema:ndmxml" schemaLocation="{}"/>
  <xsd:element name="{family}" type="ndm:{family}Type"/>
</xsd:schema>
"#,
            family_schema.display()
        ),
    )
    .expect("write wrapper schema");
    let generated = tempfile::NamedTempFile::new().expect("temporary generated XML");
    std::fs::write(generated.path(), xml).expect("write generated XML");
    let output = Command::new("xmllint")
        .args(["--noout", "--schema"])
        .arg(wrapper.path())
        .arg(generated.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance tests: {error}"));
    assert!(
        output.status.success(),
        "generated {family} 2.0 XML failed the official schema:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn opm_2_generation_is_schema_valid_and_round_trips() {
    let opm = Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn")).unwrap();
    let xml = opm.to_xml_with(&GenerateOptions::version("2.0")).unwrap();
    validate_with_odm_2_xsd("opm", &xml);
    let parsed = Opm::from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "2.0");
}

#[test]
fn oem_2_generation_is_schema_valid_and_round_trips() {
    let oem = Oem::from_kvn(include_str!("../../data/kvn/oem_g11.kvn")).unwrap();
    let xml = oem.to_xml_with(&GenerateOptions::version("2.0")).unwrap();
    validate_with_odm_2_xsd("oem", &xml);
    let parsed = Oem::from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "2.0");
}

#[test]
fn omm_2_generation_is_schema_valid_and_round_trips() {
    let mut omm = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    omm.header.message_id = None;
    let xml = omm.to_xml_with(&GenerateOptions::version("2.0")).unwrap();
    validate_with_odm_2_xsd("omm", &xml);
    let parsed = Omm::from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "2.0");
}

#[test]
fn odm_2_generation_rejects_odm_3_only_fields() {
    let mut opm = Opm::from_kvn(include_str!("../../data/kvn/opm_g1.kvn")).unwrap();
    opm.header.message_id = Some("OPM-3-ONLY".into());
    let error = opm
        .to_xml_with(&GenerateOptions::version("2.0"))
        .unwrap_err();
    assert_eq!(error.field_path().as_deref(), Some("header.message_id"));
}

#[test]
fn unaudited_version_transitions_are_refused() {
    let legacy = include_str!("../../data/kvn/opm_g1.kvn").replacen("3.0", "1.0", 1);
    let opm = Opm::from_kvn(&legacy).unwrap();
    let error = opm.to_kvn_with(&GenerateOptions::latest()).unwrap_err();
    assert_eq!(
        error.code(),
        Some("generation.unsupported_version_conversion")
    );
}
