use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::traits::Ndm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../../data/kvn/tdm_e1.kvn");
const XML: &str = include_str!("../../data/xml/tdm_e21.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

#[test]
fn tdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let time_system = "TIME_SYSTEM = UTC";
    let participant = "PARTICIPANT_1 = DSS-25";
    let observation = "TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733";
    for (label, source) in [
        (
            "duplicate header keyword",
            KVN.replace("ORIGINATOR = NASA", "ORIGINATOR = NASA\nORIGINATOR = NASA"),
        ),
        (
            "duplicate metadata keyword",
            KVN.replace(time_system, &format!("{time_system}\n{time_system}")),
        ),
        (
            "unknown metadata keyword",
            KVN.replace(time_system, &format!("{time_system}\nUNKNOWN = value")),
        ),
        (
            "comment after an observation",
            KVN.replace(observation, &format!("{observation}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("META_START", "UNKNOWN_START")),
        (
            "mismatched block end",
            KVN.replace("META_STOP", "DATA_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(participant, &format!("{participant} €")),
        ),
    ] {
        assert!(Tdm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn tdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let epoch = "<EPOCH>2007-069T15:22:22.000</EPOCH>";
    let observable = "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>";
    for (label, source) in [
        (
            "unknown metadata child",
            XML.replace("<metadata>", "<metadata><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown observation child",
            XML.replace("<observation>", "<observation><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown container attribute",
            XML.replace("<metadata>", "<metadata unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                observable,
                "<TRANSMIT_FREQ_1 unexpected=\"value\">7167941264.0</TRANSMIT_FREQ_1>",
            ),
        ),
        (
            "duplicate epoch",
            XML.replacen(epoch, &format!("{epoch}{epoch}"), 1),
        ),
        (
            "reordered observation members",
            XML.replacen(
                &format!("{epoch}\n{observable}"),
                &format!("{observable}\n{epoch}"),
                1,
            ),
        ),
    ] {
        assert!(Tdm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn tdm_xml_accepts_the_schema_defined_optional_observation_units() {
    let angle = XML.replacen(
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<ANGLE_1 units=\"deg\">1.0</ANGLE_1>",
        1,
    );
    Tdm::from_xml(&angle).expect("ANGLE_1 units=deg is allowed by the TDM schema");

    let humidity = XML.replacen(
        "<TRANSMIT_FREQ_1>7167941264.0</TRANSMIT_FREQ_1>",
        "<RHUMIDITY units=\"%\">50.0</RHUMIDITY>",
        1,
    );
    let mut message =
        Tdm::from_xml(&humidity).expect("RHUMIDITY units=% is allowed by the TDM schema");
    assert!(
        message.to_kvn().is_err(),
        "non-ASCII XML text is not representable in KVN"
    );
    message.body.segments[0].metadata.participant_1 = "DSS-25".into();
    let normalized = Tdm::from_kvn(&message.to_kvn().unwrap()).unwrap();
    match &normalized.body.segments[0].data.observations[0].data {
        ccsds_ndm::messages::tdm::TdmObservationData::Rhumidity(value) => {
            assert_eq!(value.value, 50.0);
            assert!(value.units.is_none());
        }
        other => panic!("expected normalized humidity, got {other:?}"),
    }
}

#[test]
fn every_shipped_tdm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    let kvn_dir = repository_path("data/kvn");
    let mut kvn_files = fs::read_dir(&kvn_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("tdm_") && name.ends_with(".kvn"))
        })
        .collect::<Vec<_>>();
    kvn_files.sort();
    assert_eq!(kvn_files.len(), 21, "unexpected TDM KVN fixture inventory");

    for path in kvn_files {
        let name = path.file_name().unwrap().to_string_lossy();
        let source = fs::read_to_string(&path).unwrap();
        let message = Tdm::from_kvn(&source).unwrap_or_else(|error| panic!("{name}: {error}"));
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Tdm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(&name, &xml);
    }

    for name in ["tdm_e21.xml", "tdm_e23.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
        let message = Tdm::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Tdm::from_xml(&xml).unwrap(), message, "{name} XML model");
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
