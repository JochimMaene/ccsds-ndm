// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::ndm as core_ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

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
pub struct Ndm {
    id: Option<String>,
    comments: Vec<String>,
    messages: Py<PyList>,
}

impl Ndm {
    pub(crate) fn from_core(py: Python<'_>, value: core_ndm::CombinedNdm) -> PyResult<Self> {
        let messages = value
            .messages
            .into_iter()
            .map(|message| crate::message_to_py(py, message))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            id: value.id,
            comments: value.comments,
            messages: PyList::new(py, messages)?.unbind(),
        })
    }

    pub(crate) fn to_core(&self, py: Python<'_>) -> PyResult<core_ndm::CombinedNdm> {
        let messages = self
            .messages
            .bind(py)
            .iter()
            .map(|message| py_message_to_core(py, &message.unbind()))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(core_ndm::CombinedNdm {
            id: self.id.clone(),
            comments: self.comments.clone(),
            messages,
        })
    }
}

fn py_message_to_core(py: Python<'_>, msg: &Py<PyAny>) -> PyResult<MessageType> {
    if let Ok(oem) = msg.bind(py).extract::<PyRef<'_, Oem>>() {
        Ok(MessageType::Oem(oem.to_core(py)?))
    } else if let Ok(cdm) = msg.bind(py).extract::<PyRef<'_, Cdm>>() {
        Ok(MessageType::Cdm(cdm.to_core(py)?))
    } else if let Ok(opm) = msg.bind(py).extract::<PyRef<'_, Opm>>() {
        Ok(MessageType::Opm(opm.to_core(py)?))
    } else if let Ok(omm) = msg.bind(py).extract::<PyRef<'_, Omm>>() {
        Ok(MessageType::Omm(omm.to_core(py)?))
    } else if let Ok(ocm) = msg.bind(py).extract::<PyRef<'_, Ocm>>() {
        Ok(MessageType::Ocm(ocm.to_core(py)?))
    } else if let Ok(rdm) = msg.bind(py).extract::<PyRef<'_, Rdm>>() {
        Ok(MessageType::Rdm(rdm.to_core(py)?))
    } else if let Ok(tdm) = msg.bind(py).extract::<PyRef<'_, Tdm>>() {
        Ok(MessageType::Tdm(tdm.to_core(py)?))
    } else if let Ok(aem) = msg.bind(py).extract::<PyRef<'_, crate::aem::Aem>>() {
        Ok(MessageType::Aem(aem.to_core(py)?))
    } else if let Ok(apm) = msg.bind(py).extract::<PyRef<'_, crate::apm::Apm>>() {
        Ok(MessageType::Apm(apm.to_core(py)?))
    } else if let Ok(acm) = msg.bind(py).extract::<PyRef<'_, crate::acm::Acm>>() {
        Ok(MessageType::Acm(acm.to_core(py)?))
    } else if let Ok(ndm) = msg.bind(py).extract::<PyRef<'_, Ndm>>() {
        Ok(MessageType::Ndm(ndm.to_core(py)?))
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
        Ok(Self {
            id,
            comments,
            messages: PyList::new(py, messages)?.unbind(),
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
    fn validate(&self, py: Python<'_>, strict: bool) -> PyResult<Option<Vec<String>>> {
        crate::api::validate_message(&self.to_core(py)?, strict)
    }

    /// Parse an NDM combined instantiation from a string.
    #[staticmethod]
    #[pyo3(signature = (data, format=None, *, max_input_bytes=None, max_records=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_records);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Parse an NDM combined instantiation from a file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None, *, max_input_bytes=None, max_records=None))]
    fn from_file(
        py: Python<'_>,
        path: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_records);
        let inner = crate::api::parse_typed_file_with_options(path, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Serialize the contained messages to KVN using their source versions.
    #[pyo3(signature = (max_output_bytes=None))]
    fn to_kvn(&self, py: Python<'_>, max_output_bytes: Option<usize>) -> PyResult<String> {
        let mut options = ccsds_ndm::GenerateOptions::source();
        options.max_output_bytes = max_output_bytes;
        MessageType::Ndm(self.to_core(py)?)
            .to_kvn_with(&options)
            .map_err(crate::errors::ccsds_error_to_pyerr)
    }

    /// Serialize the contained messages to XML using their source versions.
    #[pyo3(signature = (max_output_bytes=None))]
    fn to_xml(&self, py: Python<'_>, max_output_bytes: Option<usize>) -> PyResult<String> {
        let mut options = ccsds_ndm::GenerateOptions::source();
        options.max_output_bytes = max_output_bytes;
        MessageType::Ndm(self.to_core(py)?)
            .to_xml_with(&options)
            .map_err(crate::errors::ccsds_error_to_pyerr)
    }

    /// Serialize to a string.
    #[pyo3(signature = (format, max_output_bytes=None))]
    fn to_str(
        &self,
        py: Python<'_>,
        format: &str,
        max_output_bytes: Option<usize>,
    ) -> PyResult<String> {
        match format {
            "kvn" => self.to_kvn(py, max_output_bytes),
            "xml" => self.to_xml(py, max_output_bytes),
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
    #[pyo3(signature = (path, format, max_output_bytes=None))]
    fn to_file(
        &self,
        py: Python<'_>,
        path: &str,
        format: &str,
        max_output_bytes: Option<usize>,
    ) -> PyResult<()> {
        let data = self.to_str(py, format, max_output_bytes)?;
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
    fn messages(&self, py: Python<'_>) -> Py<PyList> {
        self.messages.clone_ref(py)
    }

    #[setter]
    fn set_messages(&mut self, py: Python, messages: Vec<Py<PyAny>>) -> PyResult<()> {
        py_messages_to_core(py, &messages)?;
        self.messages = PyList::new(py, messages)?.unbind();
        Ok(())
    }

    /// Message Identifier (optional).
    ///
    /// :type: Optional[str]
    #[getter]
    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Comments (optional).
    ///
    /// :type: list[str]
    #[getter]
    fn comments(&self) -> Vec<String> {
        self.comments.clone()
    }

    #[setter]
    fn set_comments(&mut self, comments: Vec<String>) {
        self.comments = comments;
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "Ndm(messages={}, id={:?})",
            self.messages.bind(py).len(),
            self.id
        )
    }
}
