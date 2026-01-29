// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::{from_str, MessageType};
use std::fs;

mod common;

#[test]
fn test_parse_all_samples() {
    let data_dir = common::data_dir();

    if !data_dir.exists() {
        eprintln!(
            "Data directory not found at {:?}, skipping integration tests relying on data",
            data_dir
        );
        return;
    }

    let mut failures = Vec::new();

    let kvn_dir = data_dir.join("kvn");
    if kvn_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(kvn_dir).unwrap().map(|e| e.unwrap()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("kvn") {
                let fname = path.file_name().unwrap().to_str().unwrap().to_string();
                println!("Parsing KVN: {:?}", fname);
                let content = fs::read_to_string(&path).unwrap();
                match from_str(&content) {
                    Ok(msg) => {
                        let is_match = match &msg {
                            MessageType::Opm(_) => fname.starts_with("opm"),
                            MessageType::Omm(_) => fname.starts_with("omm"),
                            MessageType::Oem(_) => fname.starts_with("oem"),
                            MessageType::Ocm(_) => fname.starts_with("ocm"),
                            MessageType::Tdm(_) => fname.starts_with("tdm"),
                            MessageType::Rdm(_) => fname.starts_with("rdm"),
                            MessageType::Cdm(_) => fname.starts_with("cdm"),
                            MessageType::Apm(_) => fname.starts_with("apm"),
                            MessageType::Aem(_) => fname.starts_with("aem"),
                            MessageType::Acm(_) => fname.starts_with("acm"),
                            MessageType::Ndm(_) => fname.starts_with("ndm"),
                        };

                        if !is_match {
                            failures.push(format!(
                                "{} parsed but type mismatch (got {:?})",
                                fname, msg
                            ));
                        }

                        // Round-trip verification: Serialize -> Parse again
                        match msg.to_kvn() {
                            Ok(kvn_out) => {
                                // Try to parse the output again
                                if let Err(e) = from_str(&kvn_out) {
                                    failures.push(format!(
                                        "{} KVN round-trip failed to parse: {}",
                                        fname, e
                                    ));
                                }
                            }
                            Err(e) => failures
                                .push(format!("{} failed to serialize to KVN: {}", fname, e)),
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse {}: {}", fname, e);
                        failures.push(format!("{} failed: {}", fname, e));
                    }
                }
            }
        }
    }

    let xml_dir = data_dir.join("xml");
    if xml_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(xml_dir).unwrap().map(|e| e.unwrap()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("xml") {
                let fname = path.file_name().unwrap().to_str().unwrap().to_string();
                if fname.starts_with("ndm_") {
                    println!("Parsing combined NDM XML: {:?}", fname);
                } else {
                    println!("Parsing XML: {:?}", fname);
                }

                let content = fs::read_to_string(&path).unwrap();
                match from_str(&content) {
                    Ok(msg) => {
                        // Check type match
                        let is_match = match &msg {
                            MessageType::Opm(_) => fname.starts_with("opm"),
                            MessageType::Omm(_) => fname.starts_with("omm"),
                            MessageType::Oem(_) => fname.starts_with("oem"),
                            MessageType::Ocm(_) => fname.starts_with("ocm"),
                            MessageType::Tdm(_) => fname.starts_with("tdm"),
                            MessageType::Rdm(_) => fname.starts_with("rdm"),
                            MessageType::Cdm(_) => fname.starts_with("cdm"),
                            MessageType::Apm(_) => fname.starts_with("apm"),
                            MessageType::Aem(_) => fname.starts_with("aem"),
                            MessageType::Acm(_) => fname.starts_with("acm"),
                            MessageType::Ndm(_) => fname.starts_with("ndm"),
                        };

                        if !is_match {
                            failures.push(format!(
                                "{} parsed but type mismatch (got {:?})",
                                fname, msg
                            ));
                        }

                        // Round-trip verification: Serialize -> Parse again
                        match msg.to_xml() {
                            Ok(xml_out) => {
                                // Try to parse the output again
                                if let Err(e) = from_str(&xml_out) {
                                    failures.push(format!(
                                        "{} XML round-trip failed to parse: {}\nContent:\n{}",
                                        fname, e, xml_out
                                    ));
                                }
                            }
                            Err(e) => failures
                                .push(format!("{} failed to serialize to XML: {}", fname, e)),
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse {}: {}", fname, e);
                        failures.push(format!("{} failed: {}", fname, e));
                    }
                }
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "Encountered {} parsing failures:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
