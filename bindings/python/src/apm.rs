// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
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
use crate::common::{parse_time_system};


/// Attitude Parameter Message (APM).
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message
/// is suited to interagency exchanges that involve automated interaction and/or human
/// interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.
///
/// The APM requires the use of a propagation technique to determine the attitude state at
/// times different from the specified epoch.
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

    /// Attitude Parameter Message (APM).
    ///
    /// An APM specifies the attitude state of a single object at a specified epoch. This message
    /// is suited to interagency exchanges that involve automated interaction and/or human
    /// interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.
    ///
    /// The APM requires the use of a propagation technique to determine the attitude state at
    /// times different from the specified epoch.
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
    #[pyo3(signature = (
        object_name,
        object_id,
        time_system=None,
        center_name=None,
        comment=None
    ))]
    fn new(
        object_name: String,
        object_id: String,
        time_system: Option<Bound<'_, PyAny>>,
        center_name: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        Ok(Self {
            inner: core_apm::ApmMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                object_id,
                center_name,
                time_system,
            },
        })
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

    /// Comments (allowed only at the beginning of the APM Metadata before OBJECT_NAME). Each
    /// comment line shall begin with this keyword.
    ///
    /// Examples: This is a comment.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// Examples: EARTH, BARYCENTER, MOON
    ///
    /// :type: str | None
    #[getter]
    fn get_center_name(&self) -> Option<String> {
        self.inner.center_name.clone()
    }

    /// Time system used for attitude and maneuver data. The set of allowed values is described in
    /// annex B, subsection B2.
    ///
    /// Examples: UTC, TAI
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
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

    /// Angular velocity vector.
    ///
    /// :type: list[AngVelState]
    #[getter]
    fn get_angular_velocity(&self) -> Vec<AngVelState> {
        self.inner
            .angular_velocity
            .iter()
            .map(|s| AngVelState { inner: s.clone() })
            .collect()
    }

    /// Spin. All mandatory elements are to be provided if the block is present. (See annex F for
    /// conventions and further detail.)
    ///
    /// :type: list[SpinState]
    #[getter]
    fn get_spin(&self) -> Vec<SpinState> {
        self.inner
            .spin
            .iter()
            .map(|s| SpinState { inner: s.clone() })
            .collect()
    }

    /// Inertia. All mandatory elements are to be provided if the block is present. (See annex F
    /// for conventions and further detail.)
    ///
    /// :type: list[InertiaState]
    #[getter]
    fn get_inertia(&self) -> Vec<InertiaState> {
        self.inner
            .inertia
            .iter()
            .map(|s| InertiaState { inner: s.clone() })
            .collect()
    }

    /// Maneuver Parameters.
    ///
    /// :type: list[ManeuverParameters]
    #[getter]
    fn get_maneuver_parameters(&self) -> Vec<ManeuverParameters> {
        self.inner
            .maneuver_parameters
            .iter()
            .map(|m| ManeuverParameters { inner: m.clone() })
            .collect()
    }

    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
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

    /// Epoch of ignition (see 7.5.10 for formatting rules)
    ///
    /// :type: str
    #[getter]
    fn get_man_epoch_start(&self) -> String {
        self.inner.man_epoch_start.as_str().to_string()
    }

    /// Maneuver duration (If = 0, impulsive maneuver)
    ///
    /// Units: s
    ///
    /// :type: float
    #[getter]
    fn get_man_duration(&self) -> f64 {
        self.inner.man_duration.value
    }

    /// Reference frame in which the velocity increment vector data are given. The user must
    /// select from the accepted set of values indicated in 3.2.4.11.
    ///
    /// :type: str
    #[getter]
    fn get_man_ref_frame(&self) -> String {
        self.inner.man_ref_frame.clone()
    }

    /// Torque X component.
    ///
    /// Units: N*m
    ///
    /// :type: float
    #[getter]
    fn get_man_tor_x(&self) -> f64 {
        self.inner.man_tor_x.value
    }

    /// Torque Y component.
    ///
    /// Units: N*m
    ///
    /// :type: float
    #[getter]
    fn get_man_tor_y(&self) -> f64 {
        self.inner.man_tor_y.value
    }

    /// Torque Z component.
    ///
    /// Units: N*m
    ///
    /// :type: float
    #[getter]
    fn get_man_tor_z(&self) -> f64 {
        self.inner.man_tor_z.value
    }

    /// Mass change during maneuver (value is < 0)
    ///
    /// Units: kg
    ///
    ///
    /// **Note**: The CCSDS standard requires this value to be strictly negative (`< 0`).
    /// However, this implementation allows non-negative values to support non-standard use cases.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_man_delta_mass(&self) -> Option<f64> {
        self.inner.man_delta_mass.as_ref().map(|v| v.value)
    }

    /// Comments (see 7.8 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
}
