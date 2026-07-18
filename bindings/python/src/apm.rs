// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::attitude::{AngVelState, EulerAngleState, InertiaState, QuaternionState, SpinState};
use crate::common::parse_time_system;
use crate::common::AdmHeader;
use crate::types::parse_calendar_epoch;
use ccsds_ndm::messages::apm as core_apm;
use pyo3::prelude::*;

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
                id: Some("CCSDS_APM_VERS".to_string()),
                version: "2.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Apm(object_name='{}')",
            self.inner.body.segment.metadata.object_name
        )
    }

    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    #[setter]
    fn set_id(&mut self, value: Option<String>) {
        self.inner.id = value;
    }

    /// The message version.
    ///
    /// :type: str
    #[getter]
    fn get_version(&self) -> String {
        self.inner.version.clone()
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Apm, &value)?;
        self.inner.version = value;
        Ok(())
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

    #[setter]
    fn set_header(&mut self, header: AdmHeader) {
        self.inner.header = header.inner;
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

    #[setter]
    fn set_segment(&mut self, segment: ApmSegment) {
        self.inner.body.segment = segment.inner;
    }

    /// Validate the message against CCSDS rules.
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

    /// Serialize to KVN, preserving the source version by default.
    #[pyo3(signature = (version=None, max_output_bytes=None))]
    fn to_kvn(&self, version: Option<&str>, max_output_bytes: Option<usize>) -> PyResult<String> {
        crate::api::generate_string_with_limit(&self.inner, "kvn", version, max_output_bytes)
    }

    /// Serialize to XML, preserving the source version by default.
    #[pyo3(signature = (version=None, max_output_bytes=None))]
    fn to_xml(&self, version: Option<&str>, max_output_bytes: Option<usize>) -> PyResult<String> {
        crate::api::generate_string_with_limit(&self.inner, "xml", version, max_output_bytes)
    }

    /// Serialize to KVN or XML. ``validate`` must remain true.
    #[pyo3(signature = (format, validate=true, version=None, max_output_bytes=None))]
    fn to_str(
        &self,
        format: &str,
        validate: bool,
        version: Option<&str>,
        max_output_bytes: Option<usize>,
    ) -> PyResult<String> {
        crate::api::require_checked_generation(validate)?;
        crate::api::generate_string_with_limit(&self.inner, format, version, max_output_bytes)
    }

    /// Write directly to a KVN or XML file. ``validate`` must remain true.
    #[pyo3(signature = (path, format, validate=true, version=None, max_output_bytes=None))]
    fn to_file(
        &self,
        path: &str,
        format: &str,
        validate: bool,
        version: Option<&str>,
        max_output_bytes: Option<usize>,
    ) -> PyResult<()> {
        crate::api::require_checked_generation(validate)?;
        crate::api::generate_file_with_limit(&self.inner, path, format, version, max_output_bytes)
    }

    #[staticmethod]
    #[pyo3(signature = (data, format=None, max_input_bytes=None, max_xml_depth=None))]
    fn from_str(
        _py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_xml_depth, None);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    #[pyo3(signature = (path, format=None, max_input_bytes=None, max_xml_depth=None))]
    fn from_file(
        _py: Python<'_>,
        path: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_xml_depth: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_xml_depth, None);
        let inner = crate::api::parse_typed_file_with_options(path, format, &options)?;
        Ok(Self { inner })
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

    #[setter]
    fn set_metadata(&mut self, metadata: ApmMetadata) {
        self.inner.metadata = metadata.inner;
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

    #[setter]
    fn set_data(&mut self, data: ApmData) {
        self.inner.data = data.inner;
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

    #[setter]
    fn set_object_name(&mut self, value: String) {
        self.inner.object_name = value;
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

    #[setter]
    fn set_object_id(&mut self, value: String) {
        self.inner.object_id = value;
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

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
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

    #[setter]
    fn set_center_name(&mut self, value: Option<String>) {
        self.inner.center_name = value;
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

    #[setter]
    fn set_time_system(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner.time_system = parse_time_system(&value)?;
        Ok(())
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
    #[pyo3(signature = (
        epoch,
        quaternion_state=None,
        euler_angle_state=None,
        angular_velocity=None,
        spin=None,
        inertia=None,
        maneuver_parameters=None,
        comment=None
    ))]
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
                epoch: parse_calendar_epoch(&epoch)?,
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

    #[setter]
    fn set_quaternion_state(&mut self, value: Vec<QuaternionState>) {
        self.inner.quaternion_state = value.into_iter().map(|s| s.inner).collect();
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

    #[setter]
    fn set_euler_angle_state(&mut self, value: Vec<EulerAngleState>) {
        self.inner.euler_angle_state = value.into_iter().map(|s| s.inner).collect();
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

    #[setter]
    fn set_angular_velocity(&mut self, value: Vec<AngVelState>) {
        self.inner.angular_velocity = value.into_iter().map(|s| s.inner).collect();
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

    #[setter]
    fn set_spin(&mut self, value: Vec<SpinState>) {
        self.inner.spin = value.into_iter().map(|s| s.inner).collect();
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

    #[setter]
    fn set_inertia(&mut self, value: Vec<InertiaState>) {
        self.inner.inertia = value.into_iter().map(|s| s.inner).collect();
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

    #[setter]
    fn set_maneuver_parameters(&mut self, value: Vec<ManeuverParameters>) {
        self.inner.maneuver_parameters = value.into_iter().map(|m| m.inner).collect();
    }

    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch = parse_calendar_epoch(&value)?;
        Ok(())
    }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
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
    #[pyo3(signature = (
        man_epoch_start,
        man_duration,
        man_ref_frame,
        man_tor_1,
        man_tor_2,
        man_tor_3,
        man_delta_mass=None,
        comment=None
    ))]
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
        use ccsds_ndm::types::{DeltaMassZ, Duration, Torque};
        Ok(Self {
            inner: ccsds_ndm::common::AttManeuverState {
                comment: comment.unwrap_or_default(),
                man_epoch_start: parse_calendar_epoch(&man_epoch_start)?,
                man_duration: Duration {
                    value: man_duration,
                    units: None,
                },
                man_ref_frame,
                man_tor_x: Torque::new(man_tor_1, None),
                man_tor_y: Torque::new(man_tor_2, None),
                man_tor_z: Torque::new(man_tor_3, None),
                man_delta_mass: man_delta_mass.map(|v| DeltaMassZ {
                    value: v,
                    units: None,
                }),
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

    #[setter]
    fn set_man_epoch_start(&mut self, value: String) -> PyResult<()> {
        self.inner.man_epoch_start = parse_calendar_epoch(&value)?;
        Ok(())
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

    #[setter]
    fn set_man_duration(&mut self, value: f64) {
        self.inner.man_duration.value = value;
    }

    /// Reference frame in which the velocity increment vector data are given. The user must
    /// select from the accepted set of values indicated in 3.2.4.11.
    ///
    /// :type: str
    #[getter]
    fn get_man_ref_frame(&self) -> String {
        self.inner.man_ref_frame.clone()
    }

    #[setter]
    fn set_man_ref_frame(&mut self, value: String) {
        self.inner.man_ref_frame = value;
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

    #[setter]
    fn set_man_tor_x(&mut self, value: f64) {
        self.inner.man_tor_x.value = value;
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

    #[setter]
    fn set_man_tor_y(&mut self, value: f64) {
        self.inner.man_tor_y.value = value;
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

    #[setter]
    fn set_man_tor_z(&mut self, value: f64) {
        self.inner.man_tor_z.value = value;
    }

    /// Mass change during maneuver (value is < 0)
    ///
    /// Units: kg
    ///
    ///
    /// The applicable XML schema uses `deltamassTypeZ`, so zero is allowed.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_man_delta_mass(&self) -> Option<f64> {
        self.inner.man_delta_mass.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_man_delta_mass(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::DeltaMassZ;
        self.inner.man_delta_mass = value.map(|v| DeltaMassZ {
            value: v,
            units: None,
        });
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
}
