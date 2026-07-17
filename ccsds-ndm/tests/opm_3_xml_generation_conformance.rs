// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::{CcsdsNdmError, Result, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, MessageType, VersionedNdm};
use std::path::{Path, PathBuf};
use std::process::Command;

const OPM_3_KVN_FIXTURES: [(&str, &str); 4] = [
    ("opm_g1.kvn", include_str!("../../data/kvn/opm_g1.kvn")),
    ("opm_g2.kvn", include_str!("../../data/kvn/opm_g2.kvn")),
    ("opm_g3.kvn", include_str!("../../data/kvn/opm_g3.kvn")),
    ("opm_g4.kvn", include_str!("../../data/kvn/opm_g4.kvn")),
];
const OPM_3_XML_FIXTURES: [(&str, &str); 1] =
    [("opm_g5.xml", include_str!("../../data/xml/opm_g5.xml"))];

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn validate_with_official_xsd(label: &str, xml: &str) {
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
        "generated XML for {label} failed the official OPM 3.0 XSD:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_missing_object_name<T: std::fmt::Debug>(surface: &str, result: Result<T>) {
    match result.expect_err(surface) {
        CcsdsNdmError::Validation(error) => assert!(
            matches!(
                *error,
                ValidationError::MissingRequiredField { ref field, .. }
                    if field.as_ref() == "OBJECT_NAME"
            ),
            "{surface} returned the wrong validation error: {error}"
        ),
        error => panic!("{surface} returned a non-validation error: {error}"),
    }
}

#[test]
fn every_shipped_opm_3_fixture_generates_xsd_valid_xml() {
    for (name, kvn) in OPM_3_KVN_FIXTURES {
        let opm =
            Opm::from_kvn(kvn).unwrap_or_else(|error| panic!("failed to parse {name}: {error}"));
        assert_eq!(opm.version, "3.0", "{name} is not an OPM 3.0 fixture");
        let xml = opm
            .to_xml()
            .unwrap_or_else(|error| panic!("failed to generate XML for {name}: {error}"));
        validate_with_official_xsd(name, &xml);
    }

    for (name, xml) in OPM_3_XML_FIXTURES {
        let opm =
            Opm::from_xml(xml).unwrap_or_else(|error| panic!("failed to parse {name}: {error}"));
        assert_eq!(opm.version, "3.0", "{name} is not an OPM 3.0 fixture");
        let generated = opm
            .to_xml()
            .unwrap_or_else(|error| panic!("failed to regenerate XML for {name}: {error}"));
        validate_with_official_xsd(name, &generated);
    }
}

#[test]
fn opm_3_xml_generation_is_deterministic_across_rust_entry_points() {
    let opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    let expected = opm.to_xml().expect("typed generation failed");

    assert_eq!(opm.to_xml().expect("repeated generation failed"), expected);
    assert_eq!(
        opm.to_xml_with(&GenerateOptions::source())
            .expect("versioned generation failed"),
        expected
    );

    let mut streamed = Vec::new();
    opm.write_xml_to(&mut streamed, &GenerateOptions::source())
        .expect("streaming generation failed");
    assert_eq!(streamed, expected.as_bytes());

    let generic = MessageType::Opm(opm);
    assert_eq!(
        generic.to_xml().expect("generic generation failed"),
        expected
    );
    assert_eq!(
        generic
            .to_xml_with(&GenerateOptions::source())
            .expect("generic versioned generation failed"),
        expected
    );

    validate_with_official_xsd("deterministic Rust entry points", &expected);
}

#[test]
fn every_opm_3_xml_generation_entry_point_rejects_an_invalid_model() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.body.segment.metadata.object_name.clear();

    assert_missing_object_name("Ndm::to_xml", opm.to_xml());
    assert_missing_object_name(
        "VersionedNdm::to_xml_with",
        opm.to_xml_with(&GenerateOptions::source()),
    );

    let mut streamed = Vec::new();
    assert_missing_object_name(
        "VersionedNdm::write_xml_to",
        opm.write_xml_to(&mut streamed, &GenerateOptions::source()),
    );
    assert!(
        streamed.is_empty(),
        "invalid streaming output was partially written"
    );

    let generic = MessageType::Opm(opm);
    assert_missing_object_name("MessageType::to_xml", generic.to_xml());
    assert_missing_object_name(
        "MessageType::to_xml_with",
        generic.to_xml_with(&GenerateOptions::source()),
    );

    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let path = directory.path().join("invalid-opm.xml");
    assert_missing_object_name("MessageType::to_xml_file", generic.to_xml_file(&path));
    assert!(
        !path.exists(),
        "invalid file generation created an output file"
    );

    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.body.segment.data.state_vector.x.value = f64::NAN;
    assert_non_finite_rejected("Ndm::to_xml with non-finite state", opm.to_xml());
}

#[test]
fn opm_3_xml_generation_validates_nested_optional_blocks() {
    let mut spacecraft =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse spacecraft fixture");
    spacecraft
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_mut()
        .unwrap()
        .mass
        .as_mut()
        .unwrap()
        .value = f64::NAN;
    assert_non_finite_rejected("spacecraft parameters", spacecraft.to_xml());

    let mut spacecraft =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse spacecraft fixture");
    spacecraft
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_mut()
        .unwrap()
        .mass
        .as_mut()
        .unwrap()
        .value = -1.0;
    assert!(spacecraft.to_xml().is_err());

    let mut covariance =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[3].1).expect("failed to parse covariance fixture");
    covariance
        .body
        .segment
        .data
        .covariance_matrix
        .as_mut()
        .unwrap()
        .cx_x
        .value = f64::INFINITY;
    assert_non_finite_rejected("covariance matrix", covariance.to_xml());

    let mut maneuver =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    maneuver.body.segment.data.maneuver_parameters[0]
        .man_dv_1
        .value = f64::NAN;
    assert_non_finite_rejected("maneuver parameters", maneuver.to_xml());

    let mut maneuver =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    maneuver.body.segment.data.maneuver_parameters[0]
        .man_ref_frame
        .clear();
    assert!(maneuver.to_xml().is_err());

    let mut maneuver =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    maneuver.body.segment.data.maneuver_parameters[0]
        .man_duration
        .value = -1.0;
    assert!(maneuver.to_xml().is_err());

    let mut zero_delta_mass =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    zero_delta_mass.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 0.0;
    let xml = zero_delta_mass
        .to_xml()
        .expect("the OPM 3.0 XML schema permits zero MAN_DELTA_MASS");
    validate_with_official_xsd("zero MAN_DELTA_MASS", &xml);

    zero_delta_mass.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 1.0;
    assert!(zero_delta_mass.to_xml().is_err());
}

fn assert_non_finite_rejected<T: std::fmt::Debug>(surface: &str, result: Result<T>) {
    match result.expect_err(surface) {
        CcsdsNdmError::Validation(error) => assert!(
            matches!(
                *error,
                ValidationError::InvalidValue { ref expected, .. }
                    if expected.as_ref() == "a finite number"
            ),
            "{surface} returned the wrong validation error: {error}"
        ),
        error => panic!("{surface} returned a non-validation error: {error}"),
    }
}
