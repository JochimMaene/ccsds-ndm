// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::from_str;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn all_shipped_samples_parse_strictly() {
    let data_root = data_dir();
    let mut failures = Vec::new();

    for file in sorted_files(&data_root.join("kvn"), "kvn") {
        if let Err(error) = from_str(&fs::read_to_string(&file).unwrap()) {
            failures.push(format!("{} failed to parse: {error}", file.display()));
        }
    }

    for file in sorted_files(&data_root.join("xml"), "xml") {
        let parsed = from_str(&fs::read_to_string(&file).unwrap());
        let known_nonconformant = file
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case("ndm_g22.xml"));
        match (known_nonconformant, parsed) {
            (true, Ok(_)) => failures.push(format!(
                "{} parsed strictly despite missing conditionally required OPM data",
                file.display()
            )),
            (false, Err(error)) => {
                failures.push(format!("{} failed to parse: {error}", file.display()))
            }
            _ => {}
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
}

fn sorted_files(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|value| value == extension))
        .collect();
    files.sort();
    files
}
