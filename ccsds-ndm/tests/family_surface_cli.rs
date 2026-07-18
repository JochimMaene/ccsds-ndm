use std::io::Write;
use std::process::{Command, Output, Stdio};

use ccsds_ndm::{from_str, MessageType};

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
fn every_remaining_standalone_family_uses_the_packaged_cli_contract() {
    let cases = [
        ("omm", include_str!("../../data/kvn/omm_g7.kvn")),
        ("ocm", include_str!("../../data/kvn/ocm_g15.kvn")),
        ("cdm", include_str!("../../data/kvn/cdm_362.kvn")),
        ("apm", include_str!("../../data/kvn/apm_g1.kvn")),
        ("aem", include_str!("../../data/kvn/aem_g4.kvn")),
        ("acm", include_str!("../../data/kvn/acm_g6.kvn")),
        ("rdm", include_str!("../../data/kvn/rdm_c1.kvn")),
        ("tdm", include_str!("../../data/kvn/tdm_e1.kvn")),
    ];

    for (kind, input) in cases {
        let valid = cli(&["validate", "--format", "kvn", "-"], input);
        assert_eq!(valid.status.code(), Some(0), "{kind} validate");
        assert!(valid.stdout.is_empty(), "{kind} validate stdout");
        assert!(valid.stderr.is_empty(), "{kind} validate stderr");

        let converted = cli(&["convert", "--from", "kvn", "--to", "xml", "-"], input);
        assert_eq!(converted.status.code(), Some(0), "{kind} convert");
        assert!(converted.stderr.is_empty(), "{kind} convert stderr");
        let xml = String::from_utf8(converted.stdout).unwrap();
        let parsed = from_str(&xml).expect("CLI output should parse through generic dispatch");
        assert!(
            matches!(
                (kind, parsed),
                ("omm", MessageType::Omm(_))
                    | ("ocm", MessageType::Ocm(_))
                    | ("cdm", MessageType::Cdm(_))
                    | ("apm", MessageType::Apm(_))
                    | ("aem", MessageType::Aem(_))
                    | ("acm", MessageType::Acm(_))
                    | ("rdm", MessageType::Rdm(_))
                    | ("tdm", MessageType::Tdm(_))
            ),
            "{kind} identity"
        );

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
            input,
        );
        assert_eq!(limited.status.code(), Some(4), "{kind} output limit");
        assert!(limited.stdout.is_empty(), "{kind} limited stdout");
    }
}
