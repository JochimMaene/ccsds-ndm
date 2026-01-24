// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
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
                        let is_match = if fname.starts_with("opm") { matches!(msg, MessageType::Opm(_)) }
                        else if fname.starts_with("omm") { matches!(msg, MessageType::Omm(_)) }
                        else if fname.starts_with("oem") { matches!(msg, MessageType::Oem(_)) }
                        else if fname.starts_with("ocm") { matches!(msg, MessageType::Ocm(_)) }
                        else if fname.starts_with("tdm") { matches!(msg, MessageType::Tdm(_)) }
                        else if fname.starts_with("rdm") { matches!(msg, MessageType::Rdm(_)) }
                        else if fname.starts_with("cdm") { matches!(msg, MessageType::Cdm(_)) }
                        else if fname.starts_with("apm") { matches!(msg, MessageType::Apm(_)) }
                        else if fname.starts_with("aem") { matches!(msg, MessageType::Aem(_)) }
                        else if fname.starts_with("acm") { matches!(msg, MessageType::Acm(_)) }
                        else {
                            // If we have a new type in data but not here, strictly speaking it's a "pass" if it parses,
                            // but we want to ensure we identified it correctly if we know the prefix.
                            true 
                        };
                        
                        if !is_match {
                             failures.push(format!("{} parsed but type mismatch (got {:?})", fname, msg));
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
                    println!("Skipping NDM combined message: {}", fname);
                    // TODO: Support NDM combined message parsing if library supports it
                    continue;
                }
                println!("Parsing XML: {:?}", fname);
                let content = fs::read_to_string(&path).unwrap();
                match from_str(&content) {
                    Ok(msg) => {
                        let is_match = if fname.starts_with("opm") { matches!(msg, MessageType::Opm(_)) }
                        else if fname.starts_with("omm") { matches!(msg, MessageType::Omm(_)) }
                        else if fname.starts_with("oem") { matches!(msg, MessageType::Oem(_)) }
                        else if fname.starts_with("ocm") { matches!(msg, MessageType::Ocm(_)) }
                        else if fname.starts_with("tdm") { matches!(msg, MessageType::Tdm(_)) }
                        else if fname.starts_with("rdm") { matches!(msg, MessageType::Rdm(_)) }
                        else if fname.starts_with("cdm") { matches!(msg, MessageType::Cdm(_)) }
                        else if fname.starts_with("apm") { matches!(msg, MessageType::Apm(_)) }
                        else if fname.starts_with("aem") { matches!(msg, MessageType::Aem(_)) }
                        else if fname.starts_with("acm") { matches!(msg, MessageType::Acm(_)) }
                        else { true };

                        if !is_match {
                            failures.push(format!("{} parsed but type mismatch (got {:?})", fname, msg));
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
