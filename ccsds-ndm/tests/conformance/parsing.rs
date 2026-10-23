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

/// Rewrite every record of every shipped KVN fixture and require the same parsed model.
fn assert_kvn_mutation_is_insignificant(label: &str, mutate: impl Fn(&str) -> String) {
    let mut failures = Vec::new();
    for (name, source) in fixtures("", "kvn") {
        let mutated: String = source.lines().map(&mutate).collect();
        match (from_str(&source), from_str(&mutated)) {
            (Ok(expected), Ok(actual)) if actual == expected => {}
            (Ok(_), Ok(_)) => failures.push(format!("{name}: {label} changed the model")),
            (_, Err(error)) => failures.push(format!("{name}: {error}")),
            (Err(error), _) => failures.push(format!("{name} baseline: {error}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every book allows blank lines anywhere in KVN (e.g. ODM 7.3.5, ADM 6.3.5, TDM 3.2.6).
#[test]
fn a_blank_line_after_every_kvn_record_is_insignificant() {
    assert_kvn_mutation_is_insignificant("blank lines", |line| format!("{line}\n\n"));
}

/// Leading blanks are insignificant on every KVN record, including the version and header
/// (e.g. ODM 7.4.6), and a TAB reads as a blank (lossless producer deviation).
#[test]
fn leading_blanks_and_tabs_on_every_kvn_record_are_insignificant() {
    assert_kvn_mutation_is_insignificant("leading blanks", |line| format!("  {line}\n"));
    assert_kvn_mutation_is_insignificant("tabs", |line| {
        format!("\t{}\n", line.replacen(" = ", "\t=\t", 1))
    });
}
