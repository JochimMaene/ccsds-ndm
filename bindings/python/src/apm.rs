// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::attitude::{AngVelState, EulerAngleState, InertiaState, QuaternionState, SpinState};
use crate::common::parse_time_system;
use crate::common::AdmHeader;
use crate::types::parse_calendar_epoch;
use ccsds_ndm::messages::apm as core_apm;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Attitude Parameter Message (APM).
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message
/// is suited to interagency exchanges that involve automated interaction and/or human
/// interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.
///
/// The APM requires the use of a propagation technique to determine the attitude state at
/// times different from the specified epoch.
#[pyclass]
pub struct Apm {
    id: Option<String>,
    version: String,
    header: Py<AdmHeader>,
    segment: Py<ApmSegment>,
}

impl Apm {
    pub(crate) fn from_core(py: Python<'_>, value: core_apm::Apm) -> PyResult<Self> {
        Ok(Self {
            id: value.id,
            version: value.version,
            header: Py::new(
                py,
                AdmHeader {
                    inner: value.header,
                },
            )?,
            segment: Py::new(py, ApmSegment::from_core(py, value.body.segment)?)?,
        })
    }

    pub(crate) fn to_core(&self, py: Python<'_>) -> PyResult<core_apm::Apm> {
        Ok(core_apm::Apm {
            id: self.id.clone(),
            version: self.version.clone(),
            header: self.header.borrow(py).inner.clone(),
            body: core_apm::ApmBody {
                segment: self.segment.borrow(py).to_core(py)?,
            },
        })
    }
}

#[pymethods]
impl Apm {
    #[new]
    fn new(header: Py<AdmHeader>, segment: Py<ApmSegment>) -> Self {
        Self {
            header,
            segment,
            id: Some("CCSDS_APM_VERS".to_string()),
            version: "2.0".to_string(),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "Apm(object_name='{}')",
            self.segment
                .borrow(py)
                .metadata
                .borrow(py)
                .inner
                .object_name
        )
    }

    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    /// The message version.
    ///
    /// :type: str
    #[getter]
    fn get_version(&self) -> String {
        self.version.clone()
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Apm, &value)?;
        self.version = value;
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
    fn get_header(&self, py: Python<'_>) -> Py<AdmHeader> {
        self.header.clone_ref(py)
    }

    #[setter]
    fn set_header(&mut self, header: Py<AdmHeader>) {
        self.header = header;
    }

    /// APM Segment.
    ///
    /// :type: ApmSegment
    #[getter]
    fn get_segment(&self, py: Python<'_>) -> Py<ApmSegment> {
        self.segment.clone_ref(py)
    }

    #[setter]
    fn set_segment(&mut self, segment: Py<ApmSegment>) {
        self.segment = segment;
    }

    /// Validate the message against CCSDS rules.
    ///
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
    }

    /// Serialize to validated KVN or XML.
    fn to_str(&self, py: Python<'_>, format: &str) -> PyResult<String> {
        crate::api::generate_string(&self.to_core(py)?, format)
    }

    #[staticmethod]
    #[pyo3(signature = (data, format=None, *, max_input_bytes=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, None);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Parse an APM from a KVN or XML file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None, *, max_input_bytes=None))]
    fn from_file(
        py: Python<'_>,
        path: std::path::PathBuf,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, None);
        let inner = crate::api::parse_typed_file_with_options(&path, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Atomically write this APM as KVN or XML.
    fn to_file(&self, py: Python<'_>, path: std::path::PathBuf, format: &str) -> PyResult<()> {
        crate::api::generate_file(&ccsds_ndm::Message::Apm(self.to_core(py)?), &path, format)
    }
}

#[pyclass]
pub struct ApmSegment {
    metadata: Py<ApmMetadata>,
    data: Py<ApmData>,
}

impl ApmSegment {
    fn from_core(py: Python<'_>, value: core_apm::ApmSegment) -> PyResult<Self> {
        Ok(Self {
            metadata: Py::new(
                py,
                ApmMetadata {
                    inner: value.metadata,
                },
            )?,
            data: Py::new(py, ApmData::from_core(py, value.data)?)?,
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_apm::ApmSegment> {
        Ok(core_apm::ApmSegment {
            metadata: self.metadata.borrow(py).inner.clone(),
            data: self.data.borrow(py).to_core(py)?,
        })
    }
}

#[pymethods]
impl ApmSegment {
    #[new]
    fn new(metadata: Py<ApmMetadata>, data: Py<ApmData>) -> Self {
        Self { metadata, data }
    }

    /// APM Metadata Section.
    ///
    /// :type: ApmMetadata
    #[getter]
    fn get_metadata(&self, py: Python<'_>) -> Py<ApmMetadata> {
        self.metadata.clone_ref(py)
    }

    #[setter]
    fn set_metadata(&mut self, metadata: Py<ApmMetadata>) {
        self.metadata = metadata;
    }

    /// APM Data Section.
    ///
    /// :type: ApmData
    #[getter]
    fn get_data(&self, py: Python<'_>) -> Py<ApmData> {
        self.data.clone_ref(py)
    }

    #[setter]
    fn set_data(&mut self, data: Py<ApmData>) {
        self.data = data;
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
pub struct ApmData {
    comment: Vec<String>,
    epoch: ccsds_ndm::types::CalendarEpoch,
    quaternion_state: Py<PyList>,
    euler_angle_state: Py<PyList>,
    angular_velocity: Py<PyList>,
    spin: Py<PyList>,
    inertia: Py<PyList>,
    maneuver_parameters: Py<PyList>,
}

impl ApmData {
    fn from_core(py: Python<'_>, value: core_apm::ApmData) -> PyResult<Self> {
        macro_rules! py_list {
            ($values:expr, $wrapper:ident) => {{
                let objects = $values
                    .into_iter()
                    .map(|inner| Py::new(py, $wrapper { inner }))
                    .collect::<PyResult<Vec<_>>>()?;
                PyList::new(py, objects)?.unbind()
            }};
        }
        Ok(Self {
            comment: value.comment,
            epoch: value.epoch,
            quaternion_state: py_list!(value.quaternion_state, QuaternionState),
            euler_angle_state: py_list!(value.euler_angle_state, EulerAngleState),
            angular_velocity: py_list!(value.angular_velocity, AngVelState),
            spin: py_list!(value.spin, SpinState),
            inertia: py_list!(value.inertia, InertiaState),
            maneuver_parameters: py_list!(value.maneuver_parameters, ApmManeuverParameters),
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_apm::ApmData> {
        macro_rules! core_list {
            ($values:expr, $wrapper:ty, $name:literal) => {
                $values
                    .bind(py)
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        value
                            .extract::<PyRef<'_, $wrapper>>()
                            .map(|value| value.inner.clone())
                            .map_err(|_| {
                                pyo3::exceptions::PyValueError::new_err(format!(
                                    "{}[{index}] has the wrong type",
                                    $name
                                ))
                            })
                    })
                    .collect::<PyResult<Vec<_>>>()?
            };
        }
        Ok(core_apm::ApmData {
            comment: self.comment.clone(),
            epoch: self.epoch,
            quaternion_state: core_list!(
                self.quaternion_state,
                QuaternionState,
                "quaternion_state"
            ),
            euler_angle_state: core_list!(
                self.euler_angle_state,
                EulerAngleState,
                "euler_angle_state"
            ),
            angular_velocity: core_list!(self.angular_velocity, AngVelState, "angular_velocity"),
            spin: core_list!(self.spin, SpinState, "spin"),
            inertia: core_list!(self.inertia, InertiaState, "inertia"),
            maneuver_parameters: core_list!(
                self.maneuver_parameters,
                ApmManeuverParameters,
                "maneuver_parameters"
            ),
        })
    }
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
        py: Python<'_>,
        epoch: String,
        quaternion_state: Option<Vec<Py<QuaternionState>>>,
        euler_angle_state: Option<Vec<Py<EulerAngleState>>>,
        angular_velocity: Option<Vec<Py<AngVelState>>>,
        spin: Option<Vec<Py<SpinState>>>,
        inertia: Option<Vec<Py<InertiaState>>>,
        maneuver_parameters: Option<Vec<Py<ApmManeuverParameters>>>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            comment: comment.unwrap_or_default(),
            epoch: parse_calendar_epoch(&epoch)?,
            quaternion_state: PyList::new(py, quaternion_state.unwrap_or_default())?.unbind(),
            euler_angle_state: PyList::new(py, euler_angle_state.unwrap_or_default())?.unbind(),
            angular_velocity: PyList::new(py, angular_velocity.unwrap_or_default())?.unbind(),
            spin: PyList::new(py, spin.unwrap_or_default())?.unbind(),
            inertia: PyList::new(py, inertia.unwrap_or_default())?.unbind(),
            maneuver_parameters: PyList::new(py, maneuver_parameters.unwrap_or_default())?.unbind(),
        })
    }

    /// Attitude quaternion. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// :type: list[QuaternionState]
    #[getter]
    fn get_quaternion_state(&self, py: Python<'_>) -> Py<PyList> {
        self.quaternion_state.clone_ref(py)
    }

    #[setter]
    fn set_quaternion_state(
        &mut self,
        py: Python<'_>,
        value: Vec<Py<QuaternionState>>,
    ) -> PyResult<()> {
        self.quaternion_state = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Euler angle elements. All mandatory elements of the logical block are to be provided if the
    /// block is present. (See annex F for conventions and further detail.)
    ///
    /// :type: list[EulerAngleState]
    #[getter]
    fn get_euler_angle_state(&self, py: Python<'_>) -> Py<PyList> {
        self.euler_angle_state.clone_ref(py)
    }

    #[setter]
    fn set_euler_angle_state(
        &mut self,
        py: Python<'_>,
        value: Vec<Py<EulerAngleState>>,
    ) -> PyResult<()> {
        self.euler_angle_state = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Angular velocity vector.
    ///
    /// :type: list[AngVelState]
    #[getter]
    fn get_angular_velocity(&self, py: Python<'_>) -> Py<PyList> {
        self.angular_velocity.clone_ref(py)
    }

    #[setter]
    fn set_angular_velocity(
        &mut self,
        py: Python<'_>,
        value: Vec<Py<AngVelState>>,
    ) -> PyResult<()> {
        self.angular_velocity = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Spin. All mandatory elements are to be provided if the block is present. (See annex F for
    /// conventions and further detail.)
    ///
    /// :type: list[SpinState]
    #[getter]
    fn get_spin(&self, py: Python<'_>) -> Py<PyList> {
        self.spin.clone_ref(py)
    }

    #[setter]
    fn set_spin(&mut self, py: Python<'_>, value: Vec<Py<SpinState>>) -> PyResult<()> {
        self.spin = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Inertia. All mandatory elements are to be provided if the block is present. (See annex F
    /// for conventions and further detail.)
    ///
    /// :type: list[InertiaState]
    #[getter]
    fn get_inertia(&self, py: Python<'_>) -> Py<PyList> {
        self.inertia.clone_ref(py)
    }

    #[setter]
    fn set_inertia(&mut self, py: Python<'_>, value: Vec<Py<InertiaState>>) -> PyResult<()> {
        self.inertia = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Maneuver Parameters.
    ///
    /// :type: list[ApmManeuverParameters]
    #[getter]
    fn get_maneuver_parameters(&self, py: Python<'_>) -> Py<PyList> {
        self.maneuver_parameters.clone_ref(py)
    }

    #[setter]
    fn set_maneuver_parameters(
        &mut self,
        py: Python<'_>,
        value: Vec<Py<ApmManeuverParameters>>,
    ) -> PyResult<()> {
        self.maneuver_parameters = PyList::new(py, value)?.unbind();
        Ok(())
    }

    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.epoch = parse_calendar_epoch(&value)?;
        Ok(())
    }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.comment = value;
    }
}

/// Maneuver Parameters block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct ApmManeuverParameters {
    pub inner: ccsds_ndm::common::AttManeuverState,
}

#[pymethods]
impl ApmManeuverParameters {
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

    /// Epoch of start of maneuver. (For format specification, see 6.8.9.)
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

    /// Maneuver duration.
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

    /// Coordinate system for the torque vector. The set of allowed values is described in annex B,
    /// subsection B3.
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

    /// 1st component of the torque vector.
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

    /// 2nd component of the torque vector.
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

    /// 3rd component of the torque vector.
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
