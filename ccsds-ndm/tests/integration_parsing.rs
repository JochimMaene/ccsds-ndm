// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
// SPDX-FileCopyrightText: 2025, 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::{from_str, MessageType};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_parse_all_samples() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // ccsds-ndm is in root/ccsds-ndm. data is in root/data.
    // So separate by one level up.
    let data_dir = manifest_dir.parent().unwrap().join("data");

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
                let fname = path.file_name().unwrap().to_str().unwrap();
                println!("Parsing KVN: {:?}", fname);
                let content = fs::read_to_string(&path).unwrap();
                match from_str(&content) {
                    Ok(msg) => {
                        if fname.starts_with("opm") {
                            if !matches!(msg, MessageType::Opm(_)) {
                                failures.push(format!("{} is not OPM", fname));
                            }
                        } else if fname.starts_with("omm") {
                            if !matches!(msg, MessageType::Omm(_)) {
                                failures.push(format!("{} is not OMM", fname));
                            }
                        } else if fname.starts_with("oem") {
                            if !matches!(msg, MessageType::Oem(_)) {
                                failures.push(format!("{} is not OEM", fname));
                            }
                        } else if fname.starts_with("ocm") {
                            if !matches!(msg, MessageType::Ocm(_)) {
                                failures.push(format!("{} is not OCM", fname));
                            }
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
                let fname = path.file_name().unwrap().to_str().unwrap();
                if fname.starts_with("ndm_") {
                    println!("Skipping NDM combined message: {}", fname);
                    continue;
                }
                println!("Parsing XML: {:?}", fname);
                let content = fs::read_to_string(&path).unwrap();
                match from_str(&content) {
                    Ok(msg) => {
                        if fname.starts_with("opm") {
                            if !matches!(msg, MessageType::Opm(_)) {
                                failures.push(format!("{} is not OPM", fname));
                            }
                        } else if fname.starts_with("omm") {
                            if !matches!(msg, MessageType::Omm(_)) {
                                failures.push(format!("{} is not OMM", fname));
                            }
                        } else if fname.starts_with("oem") {
                            if !matches!(msg, MessageType::Oem(_)) {
                                failures.push(format!("{} is not OEM", fname));
                            }
                        } else if fname.starts_with("ocm") {
                            if !matches!(msg, MessageType::Ocm(_)) {
                                failures.push(format!("{} is not OCM", fname));
                            }
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
