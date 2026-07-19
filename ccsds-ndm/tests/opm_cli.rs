use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use std::io::Write;
use std::process::{Command, Output, Stdio};

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const OMM_KVN: &str = include_str!("../../data/kvn/omm_g7.kvn");

fn cli(args: &[&str], stdin: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ccsds-ndm"));
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("CLI should start");
    if let Some(input) = stdin {
        child
            .stdin
            .take()
            .expect("stdin should be piped")
            .write_all(input.as_bytes())
            .expect("stdin should be written");
    }
    child.wait_with_output().expect("CLI should finish")
}

#[test]
fn cli_dispatches_non_opm_messages_through_the_shared_contract() {
    let valid = cli(&["validate", "--format", "kvn", "-"], Some(OMM_KVN));
    assert_eq!(valid.status.code(), Some(0));
    assert!(valid.stdout.is_empty());
    assert!(valid.stderr.is_empty());

    let converted = cli(
        &["convert", "--from", "kvn", "--to", "xml", "-"],
        Some(OMM_KVN),
    );
    assert_eq!(converted.status.code(), Some(0));
    assert!(converted.stderr.is_empty());
    let xml = String::from_utf8(converted.stdout).unwrap();
    Omm::from_xml(&xml).expect("CLI output should be a valid OMM XML document");

    let limited = cli(
        &[
            "convert",
            "--from",
            "kvn",
            "--to",
            "xml",
            "--max-output-bytes",
            "1",
            "-",
        ],
        Some(OMM_KVN),
    );
    assert_eq!(limited.status.code(), Some(4));
    assert!(limited.stdout.is_empty());

    let invalid = OMM_KVN.replacen("OBJECT_NAME", "UNKNOWN_NAME", 1);
    let diagnostic = cli(
        &["validate", "--format", "kvn", "--json", "-"],
        Some(&invalid),
    );
    assert_eq!(diagnostic.status.code(), Some(2));
    let diagnostic: serde_json::Value = serde_json::from_slice(&diagnostic.stderr).unwrap();
    assert_eq!(diagnostic["operation"], "parse");
    assert_eq!(diagnostic["notation"], "kvn");
    assert_eq!(diagnostic["message_kind"], "omm");
}

#[test]
fn validate_has_stable_exit_and_json_diagnostic_contracts() {
    let valid = cli(&["validate", "--format", "kvn", "-"], Some(KVN));
    assert_eq!(valid.status.code(), Some(0));
    assert!(valid.stdout.is_empty());
    assert!(valid.stderr.is_empty());

    let invalid = KVN.replace(
        "OBJECT_NAME = OSPREY 5",
        "OBJECT_NAME = OSPREY 5\nUNKNOWN_KEY = value",
    );
    let output = cli(
        &["validate", "--format", "kvn", "--json", "-"],
        Some(&invalid),
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("stderr should contain one JSON diagnostic");
    assert_eq!(diagnostic["operation"], "parse");
    assert_eq!(diagnostic["notation"], "kvn");
    assert_eq!(diagnostic["code"], "parse.kvn.syntax");
    assert_eq!(diagnostic["line"], 6);

    let limited = cli(
        &["validate", "--format", "kvn", "--max-input-bytes", "1", "-"],
        Some(KVN),
    );
    assert_eq!(limited.status.code(), Some(4));
    assert!(limited.stdout.is_empty());

    let xml = Opm::from_kvn(KVN)
        .expect("KVN fixture should parse")
        .to_xml()
        .expect("fixture should generate as XML");
    let valid_xml = cli(&["validate", "--format", "xml", "-"], Some(&xml));
    assert_eq!(valid_xml.status.code(), Some(0));
    assert!(valid_xml.stdout.is_empty());
    assert!(valid_xml.stderr.is_empty());
}

#[test]
fn usage_errors_are_rejected_before_reading_input() {
    let output = cli(&["convert", "--from", "kvn", "-"], Some(KVN));
    assert_eq!(output.status.code(), Some(64));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("convert requires --to kvn|xml"));
}

#[test]
fn unidentified_inputs_still_have_parse_context() {
    let output = cli(
        &["validate", "--format", "kvn", "--json", "-"],
        Some("NOT_AN_NDM = 1\n"),
    );
    assert_eq!(output.status.code(), Some(2));
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["operation"], "parse");
    assert_eq!(diagnostic["notation"], "kvn");
    assert_eq!(diagnostic["message_kind"], "ndm");
}

#[test]
fn convert_keeps_document_bytes_separate_and_protects_destination_files() {
    let output = cli(&["convert", "--from", "kvn", "--to", "xml", "-"], Some(KVN));
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let xml = String::from_utf8(output.stdout).expect("stdout should be UTF-8 XML");
    let expected = Opm::from_kvn(KVN).expect("fixture should parse");
    assert_eq!(
        Opm::from_xml(&xml).expect("stdout should contain only the converted document"),
        expected
    );

    let reverse = cli(
        &["convert", "--from", "xml", "--to", "kvn", "-"],
        Some(&xml),
    );
    assert_eq!(reverse.status.code(), Some(0));
    assert!(reverse.stderr.is_empty());
    let kvn = String::from_utf8(reverse.stdout).expect("stdout should be UTF-8 KVN");
    assert_eq!(
        Opm::from_kvn(&kvn).expect("reverse-converted KVN should parse"),
        expected
    );

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let destination = directory.path().join("output.xml");
    std::fs::write(&destination, b"sentinel").expect("sentinel should be written");
    let invalid = cli(
        &[
            "convert",
            "--from",
            "kvn",
            "--to",
            "xml",
            "--output",
            destination.to_str().unwrap(),
            "-",
        ],
        Some("not an OPM"),
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert!(invalid.stdout.is_empty());
    assert_eq!(
        std::fs::read(destination).expect("destination should remain readable"),
        b"sentinel"
    );

    let odm_2 = cli(
        &[
            "convert",
            "--from",
            "kvn",
            "--to",
            "xml",
            "--target-version",
            "2.0",
            "-",
        ],
        Some(KVN),
    );
    assert_eq!(odm_2.status.code(), Some(0));
    assert!(odm_2.stderr.is_empty());
    let xml = String::from_utf8(odm_2.stdout).expect("ODM 2.0 XML should be UTF-8");
    let parsed = Opm::from_xml(&xml).expect("ODM 2.0 output should parse");
    assert_eq!(parsed.version, "2.0");
}
