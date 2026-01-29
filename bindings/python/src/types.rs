// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
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

/// USER DEFINED PARAMETERS block (`userDefinedType`).
/// User-defined parameters.
///
/// Allow for the exchange of any desired orbital data not already provided in the message.
///
/// Parameters
/// ----------
///     parameters : dict[str, str], optional
///     A dictionary of user-defined parameters and their values.
/// comment : list[str], optional
///     Comments.
#[pyclass]
#[derive(Clone, Default)]
pub struct UserDefined {
    pub inner: core_types::UserDefined,
}

#[pymethods]
impl UserDefined {
    /// Create a new UserDefined object.
    #[new]
    #[pyo3(signature = (parameters=None, comment=None))]
    fn new(
        parameters: Option<std::collections::HashMap<String, String>>,
        comment: Option<Vec<String>>,
    ) -> Self {
        let user_defined = parameters
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| core_types::UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
        Self {
            inner: core_types::UserDefined {
                comment: comment.unwrap_or_default(),
                user_defined,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("UserDefined(params={})", self.inner.user_defined.len())
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// User-defined parameters.
    ///
    /// :type: dict[str, str]
    #[getter]
    fn get_user_defined(&self) -> std::collections::HashMap<String, String> {
        self.inner
            .user_defined
            .iter()
            .map(|p| (p.parameter.clone(), p.value.clone()))
            .collect()
    }

    #[setter]
    fn set_user_defined(&mut self, value: std::collections::HashMap<String, String>) {
        self.inner.user_defined = value
            .into_iter()
            .map(|(k, v)| core_types::UserDefinedParameter {
                parameter: k,
                value: v,
            })
            .collect();
    }
}
