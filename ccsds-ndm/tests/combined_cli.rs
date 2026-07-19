use std::io::Write;
use std::process::{Command, Output, Stdio};

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::traits::Ndm;

const XML: &str = include_str!("../../data/xml/ndm_g12.xml");

fn cli(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ccsds-ndm"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("CLI should start");
    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(stdin.as_bytes())
        .expect("stdin should be written");
    child.wait_with_output().expect("CLI should finish")
}

#[test]
fn combined_xml_uses_the_shared_cli_contract() {
    let valid = cli(&["validate", "--format", "xml", "-"], XML);
    assert_eq!(valid.status.code(), Some(0));
    assert!(valid.stdout.is_empty());
    assert!(valid.stderr.is_empty());

    let converted = cli(&["convert", "--to", "xml", "-"], XML);
    assert_eq!(converted.status.code(), Some(0));
    assert!(converted.stderr.is_empty());
    CombinedNdm::from_xml(&String::from_utf8(converted.stdout).unwrap())
        .expect("CLI output should remain a valid combined NDM");

    let limited = cli(
        &["convert", "--to", "xml", "--max-output-bytes", "1", "-"],
        XML,
    );
    assert_eq!(limited.status.code(), Some(4));
    assert!(limited.stdout.is_empty());

    let invalid = XML.replacen("<ndm ", "<ndm unexpected=\"value\" ", 1);
    let diagnostic = cli(&["validate", "--format", "xml", "--json", "-"], &invalid);
    assert_eq!(diagnostic.status.code(), Some(2));
    assert!(diagnostic.stdout.is_empty());
    let diagnostic: serde_json::Value = serde_json::from_slice(&diagnostic.stderr).unwrap();
    assert_eq!(diagnostic["operation"], "parse");
    assert_eq!(diagnostic["notation"], "xml");
    assert_eq!(diagnostic["message_kind"], "ndm");
}
