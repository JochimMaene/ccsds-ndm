use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Duration, ManDc, Vec3Double};
use ccsds_ndm::VersionedNdm;
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/ocm_g18.kvn");
const XML: &str = include_str!("../data/xml/ocm_g20.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn ocm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let center = "CENTER_NAME = EARTH";
    let frame = "TRAJ_REF_FRAME = TOD_EARTH";
    for (label, source) in [
        (
            "duplicate metadata keyword",
            KVN.replace("TIME_SYSTEM = UTC", "TIME_SYSTEM = UTC\nTIME_SYSTEM = UTC"),
        ),
        (
            "reordered trajectory keywords",
            KVN.replace(&format!("{center}\n{frame}"), &format!("{frame}\n{center}")),
        ),
        (
            "unknown trajectory keyword",
            KVN.replace(center, &format!("{center}\nUNKNOWN = value")),
        ),
        (
            "comment after trajectory content",
            KVN.replace(center, &format!("{center}\nCOMMENT misplaced")),
        ),
        ("unknown block", KVN.replace("TRAJ_START", "UNKNOWN_START")),
        ("mismatched block end", KVN.replace("TRAJ_STOP", "COV_STOP")),
        (
            "out-of-order logical block",
            KVN.replace("PHYS_START", "PERT_START")
                .replace("PHYS_STOP", "PERT_STOP"),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII assignment",
            KVN.replace(center, &format!("{center} €")),
        ),
    ] {
        assert!(Ocm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn ocm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let center = "<CENTER_NAME>EARTH</CENTER_NAME>";
    let time_system = "<TIME_SYSTEM>UT1</TIME_SYSTEM>";
    for (label, source) in [
        (
            "unknown data child",
            XML.replace("<data>", "<data><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown trajectory child",
            XML.replace("<traj>", "<traj><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown leaf attribute",
            XML.replace(center, "<CENTER_NAME unexpected=\"value\">EARTH</CENTER_NAME>"),
        ),
        (
            "duplicate metadata child",
            XML.replace(time_system, &format!("{time_system}{time_system}")),
        ),
        (
            "reordered metadata children",
            XML.replace(
                "<OBJECT_NAME>OSPREY 5</OBJECT_NAME>\n<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>",
                "<INTERNATIONAL_DESIGNATOR>2022-999A</INTERNATIONAL_DESIGNATOR>\n<OBJECT_NAME>OSPREY 5</OBJECT_NAME>",
            ),
        ),
    ] {
        assert!(Ocm::from_xml(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn every_shipped_ocm_fixture_preserves_histories_and_generates_valid_xml() {
    for name in [
        "ocm_g15.kvn",
        "ocm_g16.kvn",
        "ocm_g17.kvn",
        "ocm_g18.kvn",
        "ocm_g19.kvn",
    ] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Ocm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Ocm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Ocm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    let message = Ocm::from_xml(XML).unwrap();
    let xml = message.to_xml().unwrap();
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);
    validate_xml("ocm_g20.xml", &xml);
}

#[test]
fn time_and_angle_vectors_use_the_schema_lexical_form_across_notations() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    let maneuver = &mut message.body.segment.data.man[0];
    maneuver.dc_type = ManDc::TimeAndAngle;
    maneuver.dc_win_open = Some("0".parse().unwrap());
    maneuver.dc_win_close = Some("10".parse().unwrap());
    maneuver.dc_exec_start = Some("1".parse().unwrap());
    maneuver.dc_exec_stop = Some("9".parse().unwrap());
    maneuver.dc_ref_time = Some("0".parse().unwrap());
    maneuver.dc_time_pulse_duration = Some(Duration::new(1.0, None).unwrap());
    maneuver.dc_time_pulse_period = Some(Duration::new(2.0, None).unwrap());
    maneuver.dc_ref_dir = Some(Vec3Double::new(1.0, 0.0, 0.0));
    maneuver.dc_body_frame = Some("SC_BODY".to_owned());
    maneuver.dc_body_trigger = Some(Vec3Double::new(0.0, 1.0, 0.0));
    maneuver.dc_pa_start_angle = Some("0".parse().unwrap());
    maneuver.dc_pa_stop_angle = Some("180".parse().unwrap());

    let xml = message.to_xml().unwrap();
    assert!(xml.contains("<DC_REF_DIR>1 0 0</DC_REF_DIR>"));
    assert!(xml.contains("<DC_BODY_TRIGGER>0 1 0</DC_BODY_TRIGGER>"));
    assert!(!xml.contains("<DC_REF_DIR><x>"));
    validate_xml("TIME_AND_ANGLE vectors", &xml);
    assert_eq!(Ocm::from_xml(&xml).unwrap(), message);

    let kvn = message.to_kvn().unwrap();
    assert_eq!(Ocm::from_kvn(&kvn).unwrap(), message);
}

#[test]
fn every_kvn_generation_gate_rejects_invalid_state_before_output() {
    type OcmMutation = fn(&mut Ocm);
    let cases: [(&str, OcmMutation); 2] = [
        ("non-ASCII free text", |message: &mut Ocm| {
            message.body.segment.metadata.object_name = Some("OSPREY €".to_owned());
        }),
        ("overlong keyword record", |message: &mut Ocm| {
            message.body.segment.metadata.object_name = Some("X".repeat(240));
        }),
    ];
    for (label, mutate) in cases {
        let mut message = Ocm::from_kvn(KVN).unwrap();
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
fn kvn_generation_rounds_trajectory_numbers_to_the_ccsds_digit_limit() {
    let mut message = Ocm::from_kvn(KVN).unwrap();
    message.body.segment.data.traj[0].traj_lines[0].values[0] = 1.234_567_890_123_456_7;
    assert!(message.to_kvn().unwrap().contains("1.234567890123457e0"));
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
