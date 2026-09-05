use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/cdm_363.kvn");
const XML: &str = include_str!("../data/xml/cdm_44.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn edited_cdm_numeric_values_are_validated_before_any_output() {
    for field in [
        "MASS",
        "AREA_PC",
        "AREA_DRG",
        "AREA_SRP",
        "CD_AREA_OVER_MASS",
        "CR_AREA_OVER_MASS",
        "SEDR",
        "THRUST_ACCELERATION",
        "COLLISION_PROBABILITY",
    ] {
        for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            if field == "THRUST_ACCELERATION" && value == -1.0 {
                continue; // The schema permits signed acceleration.
            }
            let mut message = Cdm::from_kvn(KVN).unwrap();
            let parameters = message.body.segments[0]
                .data
                .additional_parameters
                .as_mut()
                .unwrap();
            parameters.area_drg = parameters.area_pc.clone();
            parameters.area_srp = parameters.area_pc.clone();
            let target = match field {
                "MASS" => &mut parameters.mass.as_mut().unwrap().value,
                "AREA_PC" => &mut parameters.area_pc.as_mut().unwrap().value,
                "AREA_DRG" => &mut parameters.area_drg.as_mut().unwrap().value,
                "AREA_SRP" => &mut parameters.area_srp.as_mut().unwrap().value,
                "CD_AREA_OVER_MASS" => &mut parameters.cd_area_over_mass.as_mut().unwrap().value,
                "CR_AREA_OVER_MASS" => &mut parameters.cr_area_over_mass.as_mut().unwrap().value,
                "SEDR" => &mut parameters.sedr.as_mut().unwrap().value,
                "THRUST_ACCELERATION" => {
                    &mut parameters.thrust_acceleration.as_mut().unwrap().value
                }
                _ => {
                    &mut message
                        .body
                        .relative_metadata_data
                        .collision_probability
                        .as_mut()
                        .unwrap()
                        .value
                }
            };
            *target = value;
            let error = message
                .validate()
                .expect_err("invalid edited value accepted");
            assert!(error.to_string().contains(field), "{field}: {error}");
            assert!(message.to_xml().is_err(), "{field}={value}");
            assert!(message.to_kvn().is_err(), "{field}={value}");
            let mut output = Vec::new();
            assert!(message.write_xml_to(&mut output).is_err());
            assert!(output.is_empty());
            assert!(message.write_kvn_to(&mut output).is_err());
            assert!(output.is_empty());
        }
    }
    for probability in [0.0, 1.0] {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        message
            .body
            .relative_metadata_data
            .collision_probability
            .as_mut()
            .unwrap()
            .value = probability;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .mass
            .as_mut()
            .unwrap()
            .value = 0.0;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .thrust_acceleration
            .as_mut()
            .unwrap()
            .value = -1.0;
        validate_xml("numeric boundaries", &message.to_xml().unwrap());
        assert_eq!(Cdm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
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
    type CdmMutation = fn(&mut Cdm);
    let cases: [(&str, CdmMutation); 3] = [
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
            message.write_kvn_to(&mut output).is_err(),
            "streaming accepted {label}"
        );
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
}

#[test]
fn generation_rejects_partial_optional_covariance_rows() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    let covariance = message.body.segments[0]
        .data
        .covariance_matrix
        .as_mut()
        .unwrap();
    covariance.cdrg_t = None;

    let error = message
        .to_xml()
        .expect_err("a partial optional covariance row violates CCSDS 508.0-B-1 section 5.2.8");
    assert_eq!(error.code(), Some("validation.missing_required_field"));
    assert!(error.to_string().contains("CDRG_T"));
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
