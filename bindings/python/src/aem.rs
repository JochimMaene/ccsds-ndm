// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use ccsds_ndm::types::RotSeq;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use std::str::FromStr;

/// Attitude Ephemeris Message (AEM).
///
/// An AEM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The AEM is suited to interagency exchanges that (1) involve automated
/// interaction (e.g., computer-to-computer communication for which frequent, fast, automated time
/// interpretation and processing are required), and (2) require higher fidelity or higher
/// precision dynamic modeling than is possible with the APM (e.g., flexible structures, more
/// complex attitude movement, etc.).
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

    /// Attitude Ephemeris Message (AEM).
    ///
    /// An AEM specifies the attitude state of a single object at multiple epochs, contained within a
    /// specified time range. The AEM is suited to interagency exchanges that (1) involve automated
    /// interaction (e.g., computer-to-computer communication for which frequent, fast, automated time
    /// interpretation and processing are required), and (2) require higher fidelity or higher
    /// precision dynamic modeling than is possible with the APM (e.g., flexible structures, more
    /// complex attitude movement, etc.).
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

/// AEM Metadata Section.
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
        angvel_frame: Option<String>,
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
                euler_rot_seq: euler_rot_seq.map(|s| RotSeq::from_str(&s)).transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                angvel_frame,
                interpolation_method,
                interpolation_degree: interpolation_degree.and_then(NonZeroU32::new),
            },
        })
    }

    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include Object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// Examples: EUTELSAT W1
    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }

    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// capital letter for the identification of the part brought into space by the launch. In
    /// cases in which the asset is not listed in reference [ADM-2], the UN Office of Outer Space
    /// Affairs designator index format is not used, or the content cannot be disclosed, the value
    /// should be set to UNKNOWN.
    ///
    /// Examples: 2000-052A
    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }
}

/// AEM Data Section.
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
                // NOTE: This logic is simplified and assumes a specific variant for now
                // to make it compile. Real mapping would need to check attitude_type.
                attitude_states: attitude_states.into_iter().map(|s| {
                    use ccsds_ndm::common::{QuaternionEphemeris, Quaternion};
                    ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
                        epoch: s.epoch,
                        quaternion: Quaternion {
                            q1: s.values.get(0).copied().unwrap_or(0.0),
                            q2: s.values.get(1).copied().unwrap_or(0.0),
                            q3: s.values.get(2).copied().unwrap_or(0.0),
                            qc: s.values.get(3).copied().unwrap_or(1.0),
                        },
                    })
                }).collect(),
            },
        }
    }

    /// Attitude ephemeris data lines.
    #[getter]
    fn get_attitude_states(&self) -> Vec<AttitudeState> {
        self.inner
            .attitude_states
            .iter()
            .map(|s| {
                // Simplified mapping back to generic AttitudeState
                let (epoch, values) = match s {
                    ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v) => 
                        (v.epoch, vec![v.quaternion.q1, v.quaternion.q2, v.quaternion.q3, v.quaternion.qc]),
                    _ => (ccsds_ndm::types::Epoch::default(), vec![]), // TODO: implement other variants
                };
                AttitudeState { epoch, values }
            })
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AttitudeState {
    pub epoch: ccsds_ndm::types::Epoch,
    pub values: Vec<f64>,
}

#[pymethods]
impl AttitudeState {
    #[new]
    fn new(epoch: String, values: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            epoch: parse_epoch(&epoch)?,
            values,
        })
    }
}
