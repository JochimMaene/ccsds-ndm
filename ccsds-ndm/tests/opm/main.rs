// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

#[path = "../common/mod.rs"]
mod common;
mod conversion;
mod generation;
mod parsing;
mod validation;

use ccsds_ndm::error::{Result, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const OPM_3_KVN_FIXTURES: [(&str, &str); 4] = [
    ("opm_g1.kvn", include_str!("../../data/kvn/opm_g1.kvn")),
    ("opm_g2.kvn", include_str!("../../data/kvn/opm_g2.kvn")),
    ("opm_g3.kvn", include_str!("../../data/kvn/opm_g3.kvn")),
    ("opm_g4.kvn", include_str!("../../data/kvn/opm_g4.kvn")),
];
const OPM_3_XML_FIXTURES: [(&str, &str); 1] =
    [("opm_g5.xml", include_str!("../../data/xml/opm_g5.xml"))];

fn opm() -> Opm {
    Opm::from_kvn(KVN).unwrap()
}
fn opm_with_maneuvers() -> Opm {
    Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).unwrap()
}

fn assert_missing_required<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_field: &str,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("{surface} returned a non-validation error: {error}"));
    let validation = validation_error_source(validation);
    assert!(
        matches!(
            validation,
            ValidationError::MissingRequiredField { field, .. }
                if field.as_ref() == expected_field
        ),
        "{surface} returned the wrong validation error: {validation}"
    );
}

fn validation_error_source(error: &ValidationError) -> &ValidationError {
    match error {
        ValidationError::AtPath { source, .. } => validation_error_source(source),
        error => error,
    }
}

fn assert_invalid_value_diagnostic<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.invalid_value"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    assert!(error.as_validation_error().is_some());
}

fn assert_out_of_range_diagnostic<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.out_of_range"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    assert!(error.as_validation_error().is_some());
}
