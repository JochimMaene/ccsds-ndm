// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::ndm as core_ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::cdm::Cdm;
use crate::ocm::Ocm;
use crate::oem::Oem;
use crate::omm::Omm;
use crate::opm::Opm;
use crate::rdm::Rdm;
use crate::tdm::Tdm;

/// Combined Instantiation Navigation Data Message (NDM).
///
/// It is possible to create an XML instance that incorporates any number of NDM messages in a
/// logical suite called an ‘NDM combined instantiation’. Such combined instantiations may be
/// useful for some situations, for example: (1) a constellation of spacecraft in which
/// ephemeris data for all of the spacecraft is combined in a single XML message; (2) a
/// spacecraft attitude that depends upon a particular orbital state (an APM and its
/// associated OPM could be conveniently conveyed in a single NDM); (3) an ephemeris message
/// with the set of tracking data messages used in the orbit determination.
#[pyclass]
#[derive(Clone)]
pub struct Ndm {
    pub inner: core_ndm::CombinedNdm,
}

fn py_message_to_core(py: Python<'_>, msg: &Py<PyAny>) -> PyResult<MessageType> {
    if let Ok(oem) = msg.extract::<Oem>(py) {
        Ok(MessageType::Oem(oem.inner))
    } else if let Ok(cdm) = msg.extract::<Cdm>(py) {
        Ok(MessageType::Cdm(cdm.inner))
    } else if let Ok(opm) = msg.extract::<Opm>(py) {
        Ok(MessageType::Opm(opm.inner))
    } else if let Ok(omm) = msg.extract::<Omm>(py) {
        Ok(MessageType::Omm(omm.inner))
    } else if let Ok(ocm) = msg.extract::<Ocm>(py) {
        Ok(MessageType::Ocm(ocm.inner))
    } else if let Ok(rdm) = msg.extract::<Rdm>(py) {
        Ok(MessageType::Rdm(rdm.inner))
    } else if let Ok(tdm) = msg.extract::<Tdm>(py) {
        Ok(MessageType::Tdm(tdm.inner))
    } else if let Ok(aem) = msg.extract::<crate::aem::Aem>(py) {
        Ok(MessageType::Aem(aem.inner))
    } else if let Ok(apm) = msg.extract::<crate::apm::Apm>(py) {
        Ok(MessageType::Apm(apm.inner))
    } else if let Ok(acm) = msg.extract::<crate::acm::Acm>(py) {
        Ok(MessageType::Acm(acm.inner))
    } else if let Ok(ndm) = msg.extract::<Ndm>(py) {
        Ok(MessageType::Ndm(ndm.inner))
    } else {
        Err(PyValueError::new_err(
            "Unsupported message type in NDM combined instantiation",
        ))
    }
}

fn py_messages_to_core(py: Python<'_>, messages: &[Py<PyAny>]) -> PyResult<Vec<MessageType>> {
    messages
        .iter()
        .map(|msg| py_message_to_core(py, msg))
        .collect()
}

#[pymethods]
impl Ndm {
    #[new]
    #[pyo3(signature = (messages, id=None, comments=vec![]))]
    fn new(
        messages: Vec<Py<PyAny>>,
        id: Option<String>,
        comments: Vec<String>,
        py: Python,
    ) -> PyResult<Self> {
        let core_messages = py_messages_to_core(py, &messages)?;

        Ok(Self {
            inner: core_ndm::CombinedNdm {
                id,
                comments,
                messages: core_messages,
            },
        })
    }

    /// Validate the combined message against CCSDS rules.
    ///
    /// Parameters
    /// ----------
    /// strict : bool, optional
    ///     If True (default), raises ValueError on the first error found.
    ///     If False, returns a list of validation error messages (or None if valid).
    #[pyo3(signature = (strict=true))]
    fn validate(&self, strict: bool) -> PyResult<Option<Vec<String>>> {
        crate::api::validate_message(&self.inner, strict)
    }

    /// Parse an NDM combined instantiation from a string.
    #[staticmethod]
    #[pyo3(signature = (data, format=None, max_input_bytes=None, max_xml_depth=None, max_records=None))]
    fn from_str(
        _py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_xml_depth, max_records);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Ok(Self { inner })
    }

    /// Parse an NDM combined instantiation from a file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None, max_input_bytes=None, max_xml_depth=None, max_records=None))]
    fn from_file(
        _py: Python<'_>,
        path: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_xml_depth, max_records);
        let inner = crate::api::parse_typed_file_with_options(path, format, &options)?;
        Ok(Self { inner })
    }

    /// Serialize the contained messages to KVN using their source versions.
    #[pyo3(signature = (max_output_bytes=None))]
    fn to_kvn(&self, max_output_bytes: Option<usize>) -> PyResult<String> {
        let mut options = ccsds_ndm::GenerateOptions::source();
        options.max_output_bytes = max_output_bytes;
        MessageType::Ndm(self.inner.clone())
            .to_kvn_with(&options)
            .map_err(crate::errors::ccsds_error_to_pyerr)
    }

    /// Serialize the contained messages to XML using their source versions.
    #[pyo3(signature = (max_output_bytes=None))]
    fn to_xml(&self, max_output_bytes: Option<usize>) -> PyResult<String> {
        let mut options = ccsds_ndm::GenerateOptions::source();
        options.max_output_bytes = max_output_bytes;
        MessageType::Ndm(self.inner.clone())
            .to_xml_with(&options)
            .map_err(crate::errors::ccsds_error_to_pyerr)
    }

    /// Serialize to a string.
    #[pyo3(signature = (format, validate=true, max_output_bytes=None))]
    fn to_str(
        &self,
        format: &str,
        validate: bool,
        max_output_bytes: Option<usize>,
    ) -> PyResult<String> {
        crate::api::require_checked_generation(validate)?;
        match format {
            "kvn" => self.to_kvn(max_output_bytes),
            "xml" => self.to_xml(max_output_bytes),
            other => Err(crate::api::unsupported_format(other)),
        }
    }

    /// Write to file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Output file path.
    /// format : str
    ///     Output format ('kvn' or 'xml').
    /// validate : bool, optional
    ///     Whether to validate the message before writing (default: True).
    #[pyo3(signature = (path, format, validate=true, max_output_bytes=None))]
    fn to_file(
        &self,
        path: &str,
        format: &str,
        validate: bool,
        max_output_bytes: Option<usize>,
    ) -> PyResult<()> {
        let data = self.to_str(format, validate, max_output_bytes)?;
        crate::api::atomic_write(path, |output| {
            use std::io::Write;
            output
                .write_all(data.as_bytes())
                .map_err(|error| pyo3::exceptions::PyOSError::new_err(error.to_string()))
        })
    }

    /// List of contained navigation messages.
    ///
    /// :type: list[Union[Oem, Cdm, Opm, Omm, Ocm, Rdm, Tdm, Ndm]]
    #[getter]
    fn messages(&self, py: Python) -> PyResult<Vec<Py<PyAny>>> {
        let mut py_messages = Vec::new();
        for msg in &self.inner.messages {
            let py_msg = match msg {
                MessageType::Oem(m) => Py::new(py, Oem { inner: m.clone() })?.into_any(),
                MessageType::Cdm(m) => Py::new(py, Cdm { inner: m.clone() })?.into_any(),
                MessageType::Opm(m) => Py::new(py, Opm { inner: m.clone() })?.into_any(),
                MessageType::Omm(m) => Py::new(py, Omm { inner: m.clone() })?.into_any(),
                MessageType::Ocm(m) => Py::new(py, Ocm { inner: m.clone() })?.into_any(),
                MessageType::Rdm(m) => Py::new(py, Rdm { inner: m.clone() })?.into_any(),
                MessageType::Tdm(m) => Py::new(py, Tdm { inner: m.clone() })?.into_any(),
                MessageType::Ndm(m) => Py::new(py, Ndm { inner: m.clone() })?.into_any(),
                MessageType::Aem(m) => {
                    Py::new(py, crate::aem::Aem { inner: m.clone() })?.into_any()
                }
                MessageType::Apm(m) => {
                    Py::new(py, crate::apm::Apm { inner: m.clone() })?.into_any()
                }
                MessageType::Acm(m) => {
                    Py::new(py, crate::acm::Acm { inner: m.clone() })?.into_any()
                }
            };
            py_messages.push(py_msg);
        }
        Ok(py_messages)
    }

    #[setter]
    fn set_messages(&mut self, py: Python, messages: Vec<Py<PyAny>>) -> PyResult<()> {
        self.inner.messages = py_messages_to_core(py, &messages)?;
        Ok(())
    }

    /// Message Identifier (optional).
    ///
    /// :type: Optional[str]
    #[getter]
    fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    /// Comments (optional).
    ///
    /// :type: list[str]
    #[getter]
    fn comments(&self) -> Vec<String> {
        self.inner.comments.clone()
    }

    #[setter]
    fn set_comments(&mut self, comments: Vec<String>) {
        self.inner.comments = comments;
    }

    fn __repr__(&self) -> String {
        format!(
            "Ndm(messages={}, id={:?})",
            self.inner.messages.len(),
            self.inner.id
        )
    }
}
