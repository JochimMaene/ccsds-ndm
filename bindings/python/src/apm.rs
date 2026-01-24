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
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message is
/// suited to interagency exchanges that (1) involve automated interaction and/or human
/// interaction, and (2) do not require high-fidelity dynamic modeling.
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

    /// Attitude Parameter Message (APM).
    ///
    /// An APM specifies the attitude state of a single object at a specified epoch. This message is
    /// suited to interagency exchanges that (1) involve automated interaction and/or human
    /// interaction, and (2) do not require high-fidelity dynamic modeling.
    ///
    /// :type: AdmHeader
    #[getter]
    fn get_header(&self) -> AdmHeader {
        AdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    /// APM Segment.
    ///
    /// :type: ApmSegment
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

    /// APM Metadata Section.
    ///
    /// :type: ApmMetadata
    #[getter]
    fn get_metadata(&self) -> ApmMetadata {
        ApmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    /// APM Data Section.
    ///
    /// :type: ApmData
    #[getter]
    fn get_data(&self) -> ApmData {
        ApmData {
            inner: self.inner.data.clone(),
        }
    }
}

/// APM Metadata Section.
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

    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// Examples: EUTELSAT W1, MARS PATHFINDER, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }

    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// letter for the identification of the part brought into space by the launch. In cases in
    /// which the asset is not listed in reference [ADM-2], the UN Office of Outer Space Affairs
    /// designator index format is not used, or the content cannot be disclosed, the value should
    /// be set to UNKNOWN.
    ///
    /// Examples: 2000-052A
    ///
    /// :type: str
    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }
}

/// APM Data Section.
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
        epoch: String,
        quaternion_state: Option<Vec<QuaternionState>>,
        euler_angle_state: Option<Vec<EulerAngleState>>,
        angular_velocity: Option<Vec<AngVelState>>,
        spin: Option<Vec<SpinState>>,
        inertia: Option<Vec<InertiaState>>,
        maneuver_parameters: Option<Vec<ManeuverParameters>>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_apm::ApmData {
                comment: comment.unwrap_or_default(),
                epoch: parse_epoch(&epoch)?,
                quaternion_state: quaternion_state
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.inner)
                    .collect(),
                euler_angle_state: euler_angle_state
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.inner)
                    .collect(),
                angular_velocity: angular_velocity
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.inner)
                    .collect(),
                spin: spin
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.inner)
                    .collect(),
                inertia: inertia
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.inner)
                    .collect(),
                maneuver_parameters: maneuver_parameters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| m.inner)
                    .collect(),
            },
        })
    }

    /// Attitude quaternion. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// :type: list[QuaternionState]
    #[getter]
    fn get_quaternion_state(&self) -> Vec<QuaternionState> {
        self.inner
            .quaternion_state
            .iter()
            .map(|s| QuaternionState { inner: s.clone() })
            .collect()
    }

    /// Euler angle elements. All mandatory elements of the logical block are to be provided if the
    /// block is present. (See annex F for conventions and further detail.)
    ///
    /// :type: list[EulerAngleState]
    #[getter]
    fn get_euler_angle_state(&self) -> Vec<EulerAngleState> {
        self.inner
            .euler_angle_state
            .iter()
            .map(|s| EulerAngleState { inner: s.clone() })
            .collect()
    }
}

/// Maneuver Parameters (Repeat for each maneuver).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
#[pyclass]
#[derive(Clone)]
pub struct ManeuverParameters {
    pub inner: ccsds_ndm::common::AttManeuverState,
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
        use ccsds_ndm::types::{Torque, Duration, DeltaMassZ};
        Ok(Self {
            inner: ccsds_ndm::common::AttManeuverState {
                comment: comment.unwrap_or_default(),
                man_epoch_start: parse_epoch(&man_epoch_start)?,
                man_duration: Duration { value: man_duration, units: None },
                man_ref_frame,
                man_tor_x: Torque::new(man_tor_1, None),
                man_tor_y: Torque::new(man_tor_2, None),
                man_tor_z: Torque::new(man_tor_3, None),
                man_delta_mass: man_delta_mass.map(|v| DeltaMassZ { value: v, units: None }),
            },
        })
    }
}
