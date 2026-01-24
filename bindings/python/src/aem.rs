// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

/// Attitude Ephemeris Message (AEM).
#[pyclass]
#[derive(Clone)]
pub struct Aem {
    pub inner: core_aem::Aem,
}

#[pymethods]
impl Aem {
    #[new]
    fn new(header: AdmHeader, segments: Vec<AemSegment>) -> Self {
        Self {
            inner: core_aem::Aem {
                header: header.inner,
                body: core_aem::AemBody {
                    segment: segments.into_iter().map(|s| s.inner).collect(),
                },
                id: None,
                version: "2.0".to_string(),
            },
        }
    }

    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::aem::Aem::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::aem::Aem::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Aem(aem)) => aem,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not AEM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    #[staticmethod]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self.inner.to_kvn().map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => self.inner.to_xml().map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!("Unsupported format '{}'", other))),
        }
    }

    #[getter]
    fn get_header(&self) -> AdmHeader {
        AdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    #[getter]
    fn get_segments(&self) -> Vec<AemSegment> {
        self.inner
            .body
            .segment
            .iter()
            .map(|s| AemSegment { inner: s.clone() })
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AemSegment {
    pub inner: core_aem::AemSegment,
}

#[pymethods]
impl AemSegment {
    #[new]
    fn new(metadata: AemMetadata, data: AemData) -> Self {
        Self {
            inner: core_aem::AemSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    #[getter]
    fn get_metadata(&self) -> AemMetadata {
        AemMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[getter]
    fn get_data(&self) -> AemData {
        AemData {
            inner: self.inner.data.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AemMetadata {
    pub inner: core_aem::AemMetadata,
}

#[pymethods]
impl AemMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        object_name: String,
        object_id: String,
        ref_frame_a: String,
        ref_frame_b: String,
        time_system: String,
        start_time: String,
        stop_time: String,
        attitude_type: String,
        center_name: Option<String>,
        useable_start_time: Option<String>,
        useable_stop_time: Option<String>,
        euler_rot_seq: Option<String>,
        rate_frame: Option<String>,
        interpolation_method: Option<String>,
        interpolation_degree: Option<u32>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use std::num::NonZeroU32;
        Ok(Self {
            inner: core_aem::AemMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                object_id,
                center_name,
                ref_frame_a,
                ref_frame_b,
                time_system,
                start_time: parse_epoch(&start_time)?,
                stop_time: parse_epoch(&stop_time)?,
                useable_start_time: useable_start_time.map(|s| parse_epoch(&s)).transpose()?,
                useable_stop_time: useable_stop_time.map(|s| parse_epoch(&s)).transpose()?,
                attitude_type,
                euler_rot_seq,
                rate_frame,
                interpolation_method,
                interpolation_degree: interpolation_degree.and_then(NonZeroU32::new),
            },
        })
    }

    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }

    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AemData {
    pub inner: core_aem::AemData,
}

#[pymethods]
impl AemData {
    #[new]
    fn new(attitude_states: Vec<AttitudeState>, comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_aem::AemData {
                comment: comment.unwrap_or_default(),
                attitude_states: attitude_states.into_iter().map(|s| s.inner).collect(),
            },
        }
    }

    #[getter]
    fn get_attitude_states(&self) -> Vec<AttitudeState> {
        self.inner
            .attitude_states
            .iter()
            .map(|s| AttitudeState { inner: s.clone() })
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AttitudeState {
    pub inner: core_aem::AttitudeState,
}

#[pymethods]
impl AttitudeState {
    #[new]
    fn new(epoch: String, values: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            inner: core_aem::AttitudeState {
                epoch: parse_epoch(&epoch)?,
                values,
            },
        })
    }
}
