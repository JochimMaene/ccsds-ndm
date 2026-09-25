// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated_once;

use ccsds_ndm::messages::{oem::Oem, omm::Omm, opm::Opm};
use ccsds_ndm::{Ndm, Validate};
use std::path::Path;
use std::process::Command;

fn validate_with_odm_2_xsd(family: &str, xml: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd");
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

fn edition_2(input: &str) -> String {
    mutated_once(
        &input
            .lines()
            .filter(|line| !line.starts_with("MESSAGE_ID"))
            .map(|line| format!("{line}\n"))
            .collect::<String>(),
        "3.0",
        "2.0",
    )
}

#[test]
fn opm_2_generation_is_schema_valid_and_round_trips() {
    let opm = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g1.kvn"))).unwrap();
    let xml = opm.to_xml().unwrap();
    validate_with_odm_2_xsd("opm", &xml);
    let parsed = Opm::from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "2.0");
    assert_eq!(parsed, opm);
}

/// Every shipped OEM example as OEM 2.0. Annex J of 502.0-B-3 and the 2.0 schema differ from
/// 3.0 only in the header, so the 3.0 examples without MESSAGE_ID are valid 2.0 content.
fn oem_2_examples() -> Vec<Oem> {
    let mut messages: Vec<Oem> = [
        include_str!("../../data/kvn/oem_g11.kvn"),
        include_str!("../../data/kvn/oem_g12.kvn"),
        include_str!("../../data/kvn/oem_g13.kvn"),
    ]
    .into_iter()
    .map(|kvn| Oem::from_kvn(&edition_2(kvn)).unwrap())
    .collect();
    let mut xml = Oem::from_xml(include_str!("../../data/xml/oem_g14.xml")).unwrap();
    xml.version = "2.0".into();
    xml.header.message_id = None;
    messages.push(xml);
    messages
}

#[test]
fn oem_2_generation_is_schema_valid_and_round_trips() {
    for oem in oem_2_examples() {
        let xml = oem.to_xml().unwrap();
        validate_with_odm_2_xsd("oem", &xml);
        let parsed = Oem::from_xml(&xml).unwrap();
        assert_eq!(parsed.version, "2.0");
        assert_eq!(parsed, oem);
        let kvn = oem.to_kvn().unwrap();
        let first = kvn.lines().next().unwrap();
        assert!(
            first.starts_with("CCSDS_OEM_VERS") && first.ends_with("= 2.0"),
            "{first}"
        );
        assert_eq!(Oem::from_kvn(&kvn).unwrap(), oem);
    }
}

#[test]
fn oem_2_rejects_odm_3_header_fields_when_reading_and_writing() {
    // The 2.0 schema's ndmHeader has neither CLASSIFICATION nor MESSAGE_ID; annex J lists
    // MESSAGE_ID as new in 3.0.
    let base = oem_2_examples().remove(2);
    for (field, path) in [
        ("CLASSIFICATION", "header.classification"),
        ("MESSAGE_ID", "header.message_id"),
    ] {
        let mut message = base.clone();
        match field {
            "CLASSIFICATION" => message.header.classification = Some("SBU".into()),
            _ => message.header.message_id = Some("OEM-3-ONLY".into()),
        }
        for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
            assert_eq!(
                error.field_path().as_deref(),
                Some(path),
                "{field}: {error}"
            );
        }
        let mut as_three = message.clone();
        as_three.version = "3.0".into();
        let kvn = as_three.to_kvn().unwrap().replace("= 3.0", "= 2.0");
        let error = Oem::from_kvn(&kvn).unwrap_err();
        assert_eq!(
            error.field_path().as_deref(),
            Some(path),
            "KVN {field}: {error}"
        );
        let xml = as_three
            .to_xml()
            .unwrap()
            .replace("version=\"3.0\"", "version=\"2.0\"");
        let error = Oem::from_xml(&xml).unwrap_err();
        assert_eq!(
            error.field_path().as_deref(),
            Some(path),
            "XML {field}: {error}"
        );
    }
}

#[test]
fn oem_2_useable_start_may_not_precede_the_previous_useable_stop() {
    // 502.0-B-2 Cor. 1, table 5-3: "The USEABLE_START_TIME time tag at a new block of ephemeris
    // data must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous
    // block." 3.0 dropped the sentence, so the same content stays valid there.
    let epoch = |value: &str| value.parse::<ccsds_ndm::types::Epoch>().unwrap();
    let path = "body.segment[1].metadata.useable_start_time";
    let assert_rejected = |message: &Oem| {
        crate::common::assert_rejects(message, "USEABLE_START_TIME");
        assert_eq!(
            message.validate().unwrap_err().field_path().as_deref(),
            Some(path)
        );
        let mut as_three = message.clone();
        as_three.version = "3.0".into();
        let kvn = as_three.to_kvn().unwrap().replace("= 3.0", "= 2.0");
        let error = Oem::from_kvn(&kvn).unwrap_err();
        assert_eq!(error.field_path().as_deref(), Some(path), "KVN: {error}");
        let xml = as_three
            .to_xml()
            .unwrap()
            .replace("version=\"3.0\"", "version=\"2.0\"");
        let error = Oem::from_xml(&xml).unwrap_err();
        assert_eq!(error.field_path().as_deref(), Some(path), "XML: {error}");
    };
    let base = oem_2_examples().remove(0);
    assert_eq!(base.body.segment.len(), 2);

    // Disjoint useable spans in reverse time order.
    let mut reversed = base.clone();
    reversed.body.segment.swap(0, 1);
    assert_rejected(&reversed);

    // The two values the rule names suffice; the other bounds may be absent.
    let mut partial = base.clone();
    let previous = &mut partial.body.segment[0].metadata;
    previous.useable_start_time = None;
    previous.stop_time = epoch("2019-12-29T00:00:00");
    previous.useable_stop_time = Some(epoch("2019-12-29T00:00:00"));
    partial.body.segment[1].metadata.useable_stop_time = None;
    assert_rejected(&partial);

    // A shared endpoint satisfies "greater than or equal to".
    let mut shared = base.clone();
    shared.body.segment[0].metadata.stop_time = epoch("2019-12-28T22:08:02.5");
    shared.body.segment[0].metadata.useable_stop_time = Some(epoch("2019-12-28T22:08:02.5"));
    shared.validate().expect("shared endpoint");

    // Without the previous USEABLE_STOP_TIME or the new USEABLE_START_TIME the rule is silent.
    let mut unnamed = reversed.clone();
    unnamed.body.segment[0].metadata.useable_stop_time = None;
    unnamed.validate().expect("no previous USEABLE_STOP_TIME");
    let mut unnamed = reversed.clone();
    unnamed.body.segment[1].metadata.useable_start_time = None;
    unnamed.validate().expect("no new USEABLE_START_TIME");

    for mut message in [reversed, partial] {
        message.version = "3.0".into();
        message.validate().expect("3.0 has no ordering rule");
    }
}

#[test]
fn omm_2_generation_is_schema_valid_and_round_trips() {
    let omm = Omm::from_kvn(&edition_2(include_str!("../../data/kvn/omm_g7.kvn"))).unwrap();
    let xml = omm.to_xml().unwrap();
    validate_with_odm_2_xsd("omm", &xml);
    let parsed = Omm::from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "2.0");
    assert_eq!(parsed, omm);
}

#[test]
fn odm_2_generation_rejects_odm_3_only_fields() {
    let mut opm = Opm::from_kvn(&edition_2(include_str!("../../data/kvn/opm_g1.kvn"))).unwrap();
    opm.header.message_id = Some("OPM-3-ONLY".into());
    let error = opm.to_xml().unwrap_err();
    assert_eq!(error.field_path().as_deref(), Some("header.message_id"));
}
