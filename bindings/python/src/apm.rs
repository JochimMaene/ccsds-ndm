// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::attitude::{QuaternionState, EulerAngleState, AngVelState, SpinState, InertiaState};
use crate::types::parse_epoch;
use ccsds_ndm::messages::apm as core_apm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

/// Attitude Parameter Message (APM).
#[pyclass]
#[derive(Clone)]
pub struct Apm {
    pub inner: core_apm::Apm,
}

#[pymethods]
impl Apm {
    #[new]
    fn new(header: AdmHeader, segment: ApmSegment) -> Self {
        Self {
            inner: core_apm::Apm {
                header: header.inner,
                body: core_apm::ApmBody {
                    segment: segment.inner,
                },
                id: None,
                version: "2.0".to_string(),
            },
        }
    }

    #[staticmethod]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => ccsds_ndm::messages::apm::Apm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => ccsds_ndm::messages::apm::Apm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Apm(apm)) => apm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not APM (got {:?})",
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
    fn get_segment(&self) -> ApmSegment {
        ApmSegment {
            inner: self.inner.body.segment.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ApmSegment {
    pub inner: core_apm::ApmSegment,
}

#[pymethods]
impl ApmSegment {
    #[new]
    fn new(metadata: ApmMetadata, data: ApmData) -> Self {
        Self {
            inner: core_apm::ApmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    #[getter]
    fn get_metadata(&self) -> ApmMetadata {
        ApmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[getter]
    fn get_data(&self) -> ApmData {
        ApmData {
            inner: self.inner.data.clone(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ApmMetadata {
    pub inner: core_apm::ApmMetadata,
}

#[pymethods]
impl ApmMetadata {
    #[new]
    fn new(
        object_name: String,
        object_id: String,
        time_system: String,
        center_name: Option<String>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_apm::ApmMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                object_id,
                center_name,
                time_system,
            },
        }
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
pub struct ApmData {
    pub inner: core_apm::ApmData,
}

#[pymethods]
impl ApmData {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        quaternion_state: Option<QuaternionState>,
        euler_angle_state: Option<EulerAngleState>,
        ang_vel_state: Option<AngVelState>,
        spin_state: Option<SpinState>,
        inertia_state: Option<InertiaState>,
        maneuver_parameters: Option<Vec<ManeuverParameters>>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_apm::ApmData {
                comment: comment.unwrap_or_default(),
                quaternion_state: quaternion_state.map(|s| s.inner),
                euler_angle_state: euler_angle_state.map(|s| s.inner),
                ang_vel_state: ang_vel_state.map(|s| s.inner),
                spin_state: spin_state.map(|s| s.inner),
                inertia_state: inertia_state.map(|s| s.inner),
                maneuver_parameters: maneuver_parameters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| m.inner)
                    .collect(),
            },
        }
    }

    #[getter]
    fn get_quaternion_state(&self) -> Option<QuaternionState> {
        self.inner.quaternion_state.as_ref().map(|s| QuaternionState { inner: s.clone() })
    }

    #[getter]
    fn get_euler_angle_state(&self) -> Option<EulerAngleState> {
        self.inner.euler_angle_state.as_ref().map(|s| EulerAngleState { inner: s.clone() })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ManeuverParameters {
    pub inner: core_apm::ManeuverParameters,
}

#[pymethods]
impl ManeuverParameters {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        man_epoch_start: String,
        man_duration: f64,
        man_ref_frame: String,
        man_tor_1: f64,
        man_tor_2: f64,
        man_tor_3: f64,
        man_delta_mass: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{Torque, Mass, Duration};
        Ok(Self {
            inner: core_apm::ManeuverParameters {
                comment: comment.unwrap_or_default(),
                man_epoch_start: parse_epoch(&man_epoch_start)?,
                man_duration: Duration { value: man_duration, units: None },
                man_ref_frame,
                man_tor_1: Torque::new(man_tor_1, None),
                man_tor_2: Torque::new(man_tor_2, None),
                man_tor_3: Torque::new(man_tor_3, None),
                man_delta_mass: man_delta_mass.map(|v| Mass { value: v, units: None }),
            },
        })
    }
}
