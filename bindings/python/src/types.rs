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
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse an epoch string into the core Epoch type.
///
/// This is used throughout the bindings to convert Python strings to Epochs.
pub fn parse_epoch(s: &str) -> PyResult<core_types::Epoch> {
    s.parse()
        .map_err(|e: core_types::EpochError| crate::errors::NdmEpochError::new_err(e.to_string()))
}

/// Parse an absolute calendar/ordinal epoch string into the core CalendarEpoch type.
pub fn parse_calendar_epoch(s: &str) -> PyResult<core_types::CalendarEpoch> {
    s.parse()
        .map_err(|e: core_types::EpochError| crate::errors::NdmEpochError::new_err(e.to_string()))
}

/// Parse a finite ACM/ADM relative time string into the core RelativeTime type.
pub fn parse_relative_time(s: &str) -> PyResult<core_types::RelativeTime> {
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
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Default)]
pub struct UserDefined {
    pub inner: core_types::UserDefined,
}

#[gen_stub_pymethods]
#[pymethods]
impl UserDefined {
    /// Create a block in dictionary insertion order, with any comments first.
    #[new]
    #[pyo3(signature = (parameters=None, comment=None))]
    fn new(
        #[gen_stub(override_type(type_repr = "dict[str, str] | None"))] parameters: Option<
            Bound<'_, pyo3::types::PyDict>,
        >,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let mut result = Self::default();
        result.set_comment(comment.unwrap_or_default());
        if let Some(parameters) = parameters {
            result.set_user_defined(parameters)?;
        }
        Ok(result)
    }

    fn __repr__(&self) -> String {
        format!("UserDefined(params={})", self.inner.parameters().count())
    }

    /// Comment snapshot; assignment places all supplied comments before parameters.
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comments().cloned().collect()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner
            .entries
            .retain(|entry| matches!(entry, core_types::UserDefinedEntry::Parameter(_)));
        self.inner.entries.splice(
            0..0,
            value.into_iter().map(core_types::UserDefinedEntry::Comment),
        );
    }

    /// Parameter dictionary snapshot, in message order; assign to replace parameters.
    #[gen_stub(override_return_type(type_repr = "dict[str, str]"))]
    #[getter]
    fn get_user_defined(&self, py: Python<'_>) -> PyResult<Py<pyo3::types::PyDict>> {
        let result = pyo3::types::PyDict::new(py);
        for value in self.inner.parameters() {
            result.set_item(&value.parameter, &value.value)?;
        }
        Ok(result.unbind())
    }

    #[setter]
    fn set_user_defined(
        &mut self,
        #[gen_stub(override_type(type_repr = "dict[str, str]"))] value: Bound<
            '_,
            pyo3::types::PyDict,
        >,
    ) -> PyResult<()> {
        let mut parameters = value
            .iter()
            .map(|(key, value)| {
                Ok(core_types::UserDefinedEntry::Parameter(
                    core_types::UserDefinedParameter {
                        parameter: key.extract()?,
                        value: value.extract()?,
                    },
                ))
            })
            .collect::<PyResult<Vec<_>>>()?
            .into_iter();
        self.inner.entries.retain_mut(|entry| {
            if matches!(entry, core_types::UserDefinedEntry::Parameter(_)) {
                if let Some(value) = parameters.next() {
                    *entry = value;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
        self.inner.entries.extend(parameters);
        Ok(())
    }

    /// Ordered snapshot: strings are comments; (name, value) tuples are parameters.
    /// Assign the whole list after editing. XML keeps interleaving; KVN refuses it.
    ///
    /// :type: list[str | tuple[str, str]]
    #[gen_stub(override_return_type(type_repr = "list[str | tuple[str, str]]"))]
    #[getter]
    fn get_entries(&self, py: Python<'_>) -> PyResult<Py<pyo3::types::PyList>> {
        let result = pyo3::types::PyList::empty(py);
        for entry in &self.inner.entries {
            match entry {
                core_types::UserDefinedEntry::Comment(value) => result.append(value)?,
                core_types::UserDefinedEntry::Parameter(value) => {
                    result.append((&value.parameter, &value.value))?
                }
            }
        }
        Ok(result.unbind())
    }

    #[setter]
    fn set_entries(
        &mut self,
        #[gen_stub(override_type(type_repr = "list[str | tuple[str, str]]"))] value: Bound<
            '_,
            pyo3::types::PyList,
        >,
    ) -> PyResult<()> {
        let entries = value
            .iter()
            .map(|entry| {
                if let Ok(comment) = entry.extract::<String>() {
                    Ok(core_types::UserDefinedEntry::Comment(comment))
                } else {
                    let (parameter, value) = entry.extract::<(String, String)>()?;
                    Ok(core_types::UserDefinedEntry::Parameter(
                        core_types::UserDefinedParameter { parameter, value },
                    ))
                }
            })
            .collect::<PyResult<_>>()?;
        self.inner.entries = entries;
        Ok(())
    }
}
