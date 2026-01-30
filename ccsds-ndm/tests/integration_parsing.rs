// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::{from_str, MessageType};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

#[test]
fn test_parse_all_samples() {
    if std::env::var("CCSDS_NDM_RUN_INTEGRATION").ok().as_deref() != Some("1") {
        eprintln!("Skipping integration parsing; set CCSDS_NDM_RUN_INTEGRATION=1 to enable.");
        return;
    }

    let data_dir = common::data_dir();

    if !data_dir.exists() {
        eprintln!(
            "Data directory not found at {:?}, skipping integration tests relying on data",
            data_dir
        );
        return;
    }

    let mut failures = Vec::new();
    let orekit_expectations = load_orekit_expectations(&data_dir);

    let mut files = Vec::new();
    for root in [
        data_dir.join("kvn"),
        data_dir.join("xml"),
        data_dir.join("ccsds"),
        data_dir.join("more_tests"),
    ] {
        if root.exists() {
            collect_candidate_files(&root, &mut files);
        }
    }

    files.sort();

    for path in files {
        let rel_path = path.strip_prefix(&data_dir).unwrap_or(&path);
        let rel_key = rel_path.to_string_lossy().replace('\\', "/");
        let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let fname_lower = fname.to_lowercase();
        let orekit_expectation = orekit_expectations
            .as_ref()
            .and_then(|exp| exp.expectation(&rel_key));
        let expected_fail = match orekit_expectation {
            Some(OrekitExpectation::Success) => false,
            Some(OrekitExpectation::Failure) => true,
            None => is_expected_failure(rel_path, &fname_lower),
        };
        let flag_success_against_orekit =
            matches!(orekit_expectation, Some(OrekitExpectation::Failure));
        let is_xml = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("xml"))
            .unwrap_or(false);

        if is_xml {
            if fname_lower.starts_with("ndm_") || fname_lower.starts_with("ndm") {
                println!("Parsing combined NDM XML: {:?}", rel_path);
            } else {
                println!("Parsing XML: {:?}", rel_path);
            }
        } else {
            println!("Parsing KVN: {:?}", rel_path);
        }

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                if expected_fail {
                    println!(
                        "Expected read failure for {}: {}",
                        rel_path.display(),
                        e
                    );
                    continue;
                } else {
                    failures.push(format!("{} failed to read: {}", rel_path.display(), e));
                    continue;
                }
            }
        };

        match from_str(&content) {
            Ok(msg) => {
                if flag_success_against_orekit {
                    failures.push(format!(
                        "{} parsed successfully but Orekit expects failure",
                        rel_key
                    ));
                    continue;
                }

                if expected_fail {
                    failures.push(format!(
                        "{} parsed successfully but was expected to fail",
                        rel_key
                    ));
                    continue;
                }

                if let Some(expected) = expected_type_from_name(&fname_lower) {
                    let actual = message_type_name(&msg);
                    if expected != actual {
                        failures.push(format!(
                            "{} parsed but type mismatch (got {:?})",
                            rel_key, msg
                        ));
                    }
                }

                if is_xml {
                    match msg.to_xml() {
                        Ok(xml_out) => {
                            if let Err(e) = from_str(&xml_out) {
                                failures.push(format!(
                                    "{} XML round-trip failed to parse: {}\nContent:\n{}",
                                    rel_key, e, xml_out
                                ));
                            }
                        }
                        Err(e) => failures.push(format!(
                            "{} failed to serialize to XML: {}",
                            rel_key, e
                        )),
                    }
                } else {
                    match msg.to_kvn() {
                        Ok(kvn_out) => {
                            if let Err(e) = from_str(&kvn_out) {
                                failures.push(format!(
                                    "{} KVN round-trip failed to parse: {}",
                                    rel_key, e
                                ));
                            }
                        }
                        Err(e) => failures.push(format!(
                            "{} failed to serialize to KVN: {}",
                            rel_key, e
                        )),
                    }
                }
            }
            Err(e) => {
                if expected_fail {
                    println!(
                        "Expected parse failure for {}: {}",
                        rel_key,
                        e
                    );
                } else {
                    println!("Failed to parse {}: {}", rel_key, e);
                    failures.push(format!("{} failed: {}", rel_key, e));
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

fn collect_candidate_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_candidate_files(&path, files);
            continue;
        }

        if should_parse_file(&path) {
            files.push(path);
        }
    }
}

fn should_parse_file(path: &Path) -> bool {
    let ext = match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return false,
    };

    matches!(
        ext.as_str(),
        "kvn"
            | "xml"
            | "txt"
            | "cdm"
            | "aem"
            | "oem"
            | "omm"
            | "opm"
            | "apm"
            | "acm"
            | "ocm"
            | "ndm"
            | "tdm"
            | "rdm"
    )
}

fn expected_type_from_name(lower_name: &str) -> Option<&'static str> {
    if lower_name.starts_with("ndmxml-1.0-oem") {
        Some("Oem")
    } else if lower_name.starts_with("ndmxml-1.0-omm") {
        Some("Omm")
    } else if lower_name.starts_with("opm") {
        Some("Opm")
    } else if lower_name.starts_with("omm") {
        Some("Omm")
    } else if lower_name.starts_with("oem") {
        Some("Oem")
    } else if lower_name.starts_with("ocm") {
        Some("Ocm")
    } else if lower_name.starts_with("tdm") {
        Some("Tdm")
    } else if lower_name.starts_with("rdm") {
        Some("Rdm")
    } else if lower_name.starts_with("cdm") {
        Some("Cdm")
    } else if lower_name.starts_with("apm") {
        Some("Apm")
    } else if lower_name.starts_with("aem") {
        Some("Aem")
    } else if lower_name.starts_with("acm") {
        Some("Acm")
    } else if lower_name.starts_with("ndm") {
        Some("Ndm")
    } else {
        None
    }
}

fn message_type_name(msg: &MessageType) -> &'static str {
    match msg {
        MessageType::Opm(_) => "Opm",
        MessageType::Omm(_) => "Omm",
        MessageType::Oem(_) => "Oem",
        MessageType::Ocm(_) => "Ocm",
        MessageType::Tdm(_) => "Tdm",
        MessageType::Rdm(_) => "Rdm",
        MessageType::Cdm(_) => "Cdm",
        MessageType::Apm(_) => "Apm",
        MessageType::Aem(_) => "Aem",
        MessageType::Acm(_) => "Acm",
        MessageType::Ndm(_) => "Ndm",
    }
}

fn is_expected_failure(rel_path: &Path, fname_lower: &str) -> bool {
    let rel = rel_path.to_string_lossy().replace('\\', "/").to_lowercase();
    if rel.contains("/ccsds/lexical/") {
        return true;
    }

    let keywords = [
        "missing",
        "wrong",
        "inconsistent",
        "spurious",
        "invalid",
        "unsupported",
        "unknown",
        "duplicate",
        "empty",
        "error",
        "no-",
        "too-",
        "not-implemented",
        "already-used",
        "repeated",
        "incompatible",
        "keyword-within",
        "number-format-error",
    ];

    keywords
        .iter()
        .any(|kw| fname_lower.contains(kw) || rel.contains(kw))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrekitExpectation {
    Success,
    Failure,
}

struct OrekitExpectations {
    success: HashSet<String>,
    failure: HashSet<String>,
}

impl OrekitExpectations {
    fn expectation(&self, rel_key: &str) -> Option<OrekitExpectation> {
        if self.success.contains(rel_key) {
            Some(OrekitExpectation::Success)
        } else if self.failure.contains(rel_key) {
            Some(OrekitExpectation::Failure)
        } else {
            None
        }
    }
}

fn load_orekit_expectations(data_dir: &Path) -> Option<OrekitExpectations> {
    let root = data_dir.join("ccsds_test").join("ndm");
    if !root.exists() {
        return None;
    }

    let mut java_files = Vec::new();
    collect_java_files(&root, &mut java_files);

    let mut success = HashSet::new();
    let mut failure = HashSet::new();

    for java_path in java_files {
        let content = match fs::read_to_string(&java_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        collect_orekit_expectations(&content, &mut success, &mut failure);
    }

    Some(OrekitExpectations { success, failure })
}

fn collect_java_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_java_files(&path, files);
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) == Some("java") {
            files.push(path);
        }
    }
}

fn collect_orekit_expectations(
    content: &str,
    success: &mut HashSet<String>,
    failure: &mut HashSet<String>,
) {
    let mut brace_depth = 0i32;
    let mut in_method = false;
    let mut method_start_depth = 0i32;
    let mut method_has_failure = false;
    let mut method_files: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        let is_method_start = looks_like_method_start(trimmed);

        if is_method_start && !in_method {
            in_method = true;
            method_start_depth = brace_depth;
            method_has_failure = false;
            method_files.clear();
        }

        if in_method {
            if line.contains("assertThrows")
                || line.contains("Assertions.assertThrows")
                || line.contains("catch (OrekitException")
                || line.contains("catch(OrekitException")
            {
                method_has_failure = true;
            }

            for path in extract_ccsds_paths(line) {
                if path.starts_with("ccsds/") {
                    method_files.push(path);
                }
            }
        }

        brace_depth += line.chars().filter(|c| *c == '{').count() as i32;
        brace_depth -= line.chars().filter(|c| *c == '}').count() as i32;

        if in_method && brace_depth <= method_start_depth {
            if method_has_failure {
                for path in &method_files {
                    failure.insert(path.clone());
                }
            } else {
                for path in &method_files {
                    if !failure.contains(path) {
                        success.insert(path.clone());
                    }
                }
            }

            in_method = false;
        }
    }
}

fn looks_like_method_start(trimmed: &str) -> bool {
    if trimmed.is_empty() {
        return false;
    }

    let starters = [
        "if ",
        "if(",
        "for ",
        "for(",
        "while ",
        "while(",
        "switch ",
        "switch(",
        "catch ",
        "catch(",
        "try",
        "else",
        "do",
        "class ",
        "interface ",
        "enum ",
        "@",
    ];

    if starters.iter().any(|s| trimmed.starts_with(s)) {
        return false;
    }

    trimmed.contains('(') && trimmed.contains(')') && trimmed.ends_with('{')
}

fn extract_ccsds_paths(line: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut idx = 0usize;

    while let Some(pos) = line[idx..].find("\"/ccsds/") {
        let start = idx + pos + 1;
        let rest = &line[start..];
        if let Some(end) = rest.find('"') {
            let path = &rest[..end];
            let normalized = path.trim_start_matches('/').to_string();
            paths.push(normalized);
            idx = start + end + 1;
        } else {
            break;
        }
    }

    paths
}
