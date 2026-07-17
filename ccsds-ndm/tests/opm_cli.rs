use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use std::io::Write;
use std::process::{Command, Output, Stdio};

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");

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

    let unsupported = cli(
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
    assert_eq!(unsupported.status.code(), Some(3));
    assert!(unsupported.stdout.is_empty());
}
