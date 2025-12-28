// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

// bindings/python/src/types.rs
//
// Core CCSDS types exposed to Python
//
// This module provides Python bindings for fundamental types like Epoch.
// Units are NOT exposed - getters return raw f64 values.
// Default units are documented in the .pyi stub files.

use ccsds_ndm::types as core_types;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse an epoch string into the core Epoch type.
///
/// This is used throughout the bindings to convert Python strings to Epochs.
pub fn parse_epoch(s: &str) -> PyResult<core_types::Epoch> {
    s.parse()
        .map_err(|e: core_types::EpochError| PyValueError::new_err(e.to_string()))
}
