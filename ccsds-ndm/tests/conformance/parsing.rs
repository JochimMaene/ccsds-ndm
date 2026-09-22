// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::from_str;

use crate::common::fixtures;

#[test]
fn all_shipped_samples_parse_strictly() {
    let mut failures = Vec::new();

    for (name, source) in fixtures("", "kvn") {
        if let Err(error) = from_str(&source) {
            failures.push(format!("{name} failed to parse: {error}"));
        }
    }

    for (name, source) in fixtures("", "xml") {
        let parsed = from_str(&source);
        let known_nonconformant = name == "ndm_g22.xml";
        match (known_nonconformant, parsed) {
            (true, Ok(_)) => failures.push(format!(
                "{name} parsed strictly despite missing conditionally required OPM data"
            )),
            (false, Err(error)) => failures.push(format!("{name} failed to parse: {error}")),
            (true, Err(error)) => crate::common::assert_validation_field(&error, "MASS"),
            _ => {}
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
