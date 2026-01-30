// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::ndm as core_ndm;
use ccsds_ndm::traits::Ndm as _;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

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

#[pymethods]
impl Ndm {
    #[new]
    #[pyo3(signature = (messages, id=None, comments=vec![]))]
    fn new(messages: Vec<Py<PyAny>>, id: Option<String>, comments: Vec<String>, py: Python) -> PyResult<Self> {
        let mut core_messages = Vec::new();
        for msg in messages {
            if let Ok(oem) = msg.extract::<Oem>(py) {
                core_messages.push(MessageType::Oem(oem.inner));
            } else if let Ok(cdm) = msg.extract::<Cdm>(py) {
                core_messages.push(MessageType::Cdm(cdm.inner));
            } else if let Ok(opm) = msg.extract::<Opm>(py) {
                core_messages.push(MessageType::Opm(opm.inner));
            } else if let Ok(omm) = msg.extract::<Omm>(py) {
                core_messages.push(MessageType::Omm(omm.inner));
            } else if let Ok(ocm) = msg.extract::<Ocm>(py) {
                core_messages.push(MessageType::Ocm(ocm.inner));
            } else if let Ok(rdm) = msg.extract::<Rdm>(py) {
                core_messages.push(MessageType::Rdm(rdm.inner));
            } else if let Ok(aem) = msg.extract::<crate::aem::Aem>(py) {
                core_messages.push(MessageType::Aem(aem.inner));
            } else if let Ok(apm) = msg.extract::<crate::apm::Apm>(py) {
                core_messages.push(MessageType::Apm(apm.inner));
            } else if let Ok(acm) = msg.extract::<crate::acm::Acm>(py) {
                core_messages.push(MessageType::Acm(acm.inner));
            } else if let Ok(ndm) = msg.extract::<Ndm>(py) {
                core_messages.push(MessageType::Ndm(ndm.inner));
            } else {
                return Err(PyValueError::new_err("Unsupported message type in NDM combined instantiation"));
            }
        }

        Ok(Self {
            inner: core_ndm::CombinedNdm {
                id,
                comments,
                messages: core_messages,
            },
        })
    }

    /// Parse an NDM combined instantiation from a string.
    #[staticmethod]
    #[pyo3(signature = (data, format=None))]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => core_ndm::CombinedNdm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => core_ndm::CombinedNdm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => {
                // For NDM, we need to be careful with auto-detection if it's not clearly XML <ndm>
                // The core's from_str might return MessageType::Ndm
                match ccsds_ndm::from_str(data) {
                    Ok(MessageType::Ndm(ndm)) => ndm,
                    Ok(other) => {
                        return Err(PyValueError::new_err(format!(
                            "Parsed message is not an NDM combined instantiation (got {:?})",
                            other
                        )))
                    }
                    Err(e) => return Err(PyValueError::new_err(e.to_string())),
                }
            }
        };
        Ok(Self { inner })
    }

    /// Parse an NDM combined instantiation from a file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Serialize to a string.
    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self
                .inner
                .to_kvn()
                .map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => self
                .inner
                .to_xml()
                .map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
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
    fn to_file(&self, path: &str, format: &str) -> PyResult<()> {
        let data = self.to_str(format)?;
        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(format!(
                "Failed to write file: {}",
                e
            ))),
        }
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
                MessageType::Aem(m) => Py::new(py, crate::aem::Aem { inner: m.clone() })?.into_any(),
                MessageType::Apm(m) => Py::new(py, crate::apm::Apm { inner: m.clone() })?.into_any(),
                MessageType::Acm(m) => Py::new(py, crate::acm::Acm { inner: m.clone() })?.into_any(),
            };
            py_messages.push(py_msg);
        }
        Ok(py_messages)
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

    fn __repr__(&self) -> String {
        format!(
            "Ndm(messages={}, id={:?})",
            self.inner.messages.len(),
            self.inner.id
        )
    }
}
