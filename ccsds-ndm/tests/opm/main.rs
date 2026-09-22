// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::{Result, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
use common::validation_error_source;

#[path = "../common/mod.rs"]
mod common;
mod conversion;
mod generation;
mod minimal;
mod model;
mod parsing;
mod validation;

const KVN: &str = include_str!("../../data/kvn/opm_g1.kvn");
const OPM_3_KVN_FIXTURES: [(&str, &str); 4] = [
    ("opm_g1.kvn", include_str!("../../data/kvn/opm_g1.kvn")),
    ("opm_g2.kvn", include_str!("../../data/kvn/opm_g2.kvn")),
    ("opm_g3.kvn", include_str!("../../data/kvn/opm_g3.kvn")),
    ("opm_g4.kvn", include_str!("../../data/kvn/opm_g4.kvn")),
];

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

const MINIMAL: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
X = 1000 [km]
Y = 2000 [km]
Z = 3000 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
"#;
