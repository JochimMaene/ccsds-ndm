// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{parse_interpolation_degree, AdmHeader};
use crate::types::parse_calendar_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::types::{AttitudeTypeType, RotSeq};
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use std::str::FromStr;

fn parse_attitude_type(value: &str) -> PyResult<AttitudeTypeType> {
    AttitudeTypeType::from_str(value).map_err(|error| PyValueError::new_err(error.to_string()))
}

/// Attitude Ephemeris Message (AEM).
///
/// An AEM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The AEM is suited to interagency exchanges that involve automated
/// interaction and require higher fidelity or higher precision dynamic modeling than is
/// possible with the APM.
///
/// The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric
/// torques, magnetics, etc.). It requires the use of an interpolation technique to interpret
/// the attitude state at times different from the tabular epochs.
#[gen_stub_pyclass]
#[pyclass]
pub struct Aem {
    /// The message identifier.
    ///
    /// :type: Optional[str]
    #[pyo3(get)]
    id: Option<String>,

    /// The message version.
    ///
    /// :type: str
    #[pyo3(get)]
    version: String,

    /// The message header.
    ///
    /// :type: AdmHeader
    #[pyo3(get, set)]
    header: Py<AdmHeader>,

    segments: Py<PyList>,
}

impl Aem {
    pub(crate) fn from_core(py: Python<'_>, value: core_aem::Aem) -> PyResult<Self> {
        let segments = value
            .body
            .segment
            .into_iter()
            .map(|segment| Py::new(py, AemSegment::from_core(py, segment)?))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            id: value.id,
            version: value.version,
            header: Py::new(
                py,
                AdmHeader {
                    inner: value.header,
                },
            )?,
            segments: PyList::new(py, segments)?.unbind(),
        })
    }

    pub(crate) fn to_core(&self, py: Python<'_>) -> PyResult<core_aem::Aem> {
        let segment = self
            .segments
            .bind(py)
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value
                    .extract::<PyRef<'_, AemSegment>>()
                    .map_err(|_| {
                        PyValueError::new_err(format!("segments[{index}] must be AemSegment"))
                    })?
                    .to_core(py)
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(core_aem::Aem {
            id: self.id.clone(),
            version: self.version.clone(),
            header: self.header.borrow(py).inner.clone(),
            body: core_aem::AemBody { segment },
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Aem {
    #[new]
    fn new(py: Python<'_>, header: Py<AdmHeader>, segments: Vec<Py<AemSegment>>) -> PyResult<Self> {
        Ok(Self {
            header,
            segments: PyList::new(py, segments)?.unbind(),
            id: Some("CCSDS_AEM_VERS".to_string()),
            version: "2.0".to_string(),
        })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        let segments = self.segments.bind(py);
        let object_name = segments
            .get_item(0)
            .ok()
            .and_then(|value| value.extract::<PyRef<'_, AemSegment>>().ok())
            .map(|segment| segment.metadata.borrow(py).inner.object_name.clone())
            .unwrap_or_default();
        format!(
            "Aem(object_name='{}', segments={})",
            object_name,
            segments.len()
        )
    }

    #[setter]
    fn set_version(&mut self, value: String) -> PyResult<()> {
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Aem, &value)?;
        self.version = value;
        Ok(())
    }

    /// AEM Segments.
    ///
    /// :type: list[AemSegment]
    #[gen_stub(override_return_type(type_repr = "list[AemSegment]"))]
    #[getter]
    fn get_segments(&self, py: Python<'_>) -> Py<PyList> {
        self.segments.clone_ref(py)
    }

    #[setter]
    fn set_segments(&mut self, segments: Vec<Py<AemSegment>>) -> PyResult<()> {
        Python::attach(|py| {
            self.segments = PyList::new(py, segments)?.unbind();
            Ok(())
        })
    }

    /// Validate the message against CCSDS rules.
    ///
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        crate::api::validate_message(&self.to_core(py)?)
    }

    /// Serialize to validated KVN or XML.
    fn to_str(
        &self,
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="typing.Literal[\"kvn\", \"xml\"]", imports=("typing")))]
        format: &str,
    ) -> PyResult<String> {
        crate::api::generate_string(&self.to_core(py)?, format)
    }

    #[staticmethod]
    #[pyo3(signature = (data, format=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        #[gen_stub(override_type(type_repr="typing.Optional[typing.Literal[\"kvn\", \"xml\"]]", imports=("typing")))]
        format: Option<&str>,
    ) -> PyResult<Self> {
        let inner = crate::api::parse_typed(data, format)?;
        Self::from_core(py, inner)
    }

    /// Parse an AEM from a KVN or XML file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="builtins.str | os.PathLike[builtins.str]", imports=("builtins", "os")))]
        path: std::path::PathBuf,
        #[gen_stub(override_type(type_repr="typing.Optional[typing.Literal[\"kvn\", \"xml\"]]", imports=("typing")))]
        format: Option<&str>,
    ) -> PyResult<Self> {
        let inner = crate::api::parse_typed_file(&path, format)?;
        Self::from_core(py, inner)
    }

    /// Atomically write this AEM as KVN or XML.
    fn to_file(
        &self,
        py: Python<'_>,
        #[gen_stub(override_type(type_repr="builtins.str | os.PathLike[builtins.str]", imports=("builtins", "os")))]
        path: std::path::PathBuf,
        #[gen_stub(override_type(type_repr="typing.Literal[\"kvn\", \"xml\"]", imports=("typing")))]
        format: &str,
    ) -> PyResult<()> {
        crate::api::generate_file(&ccsds_ndm::Message::Aem(self.to_core(py)?), &path, format)
    }
}

#[gen_stub_pyclass]
#[pyclass]
pub struct AemSegment {
    /// AEM Metadata Section.
    ///
    /// :type: AemMetadata
    #[pyo3(get, set)]
    metadata: Py<AemMetadata>,

    /// AEM Data Section.
    ///
    /// :type: AemData
    #[pyo3(get, set)]
    data: Py<AemData>,
}

impl AemSegment {
    fn from_core(py: Python<'_>, value: core_aem::AemSegment) -> PyResult<Self> {
        Ok(Self {
            metadata: Py::new(
                py,
                AemMetadata {
                    inner: value.metadata,
                },
            )?,
            data: Py::new(py, AemData::from_core(py, value.data)?)?,
        })
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_aem::AemSegment> {
        Ok(core_aem::AemSegment {
            metadata: self.metadata.borrow(py).inner.clone(),
            data: self.data.borrow(py).to_core(py)?,
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl AemSegment {
    #[new]
    fn new(metadata: Py<AemMetadata>, data: Py<AemData>) -> Self {
        Self { metadata, data }
    }

    /// Validate the segment against CCSDS rules.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        self.to_core(py)?
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

/// AEM Metadata Section.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct AemMetadata {
    pub inner: core_aem::AemMetadata,
}

#[gen_stub_pymethods]
#[pymethods]
impl AemMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object_name,
        object_id,
        ref_frame_a,
        ref_frame_b,
        start_time,
        stop_time,
        time_system,
        attitude_type=String::from("QUATERNION"),
        center_name=None,
        useable_start_time=None,
        useable_stop_time=None,
        euler_rot_seq=None,
        angvel_frame=None,
        interpolation_method=None,
        interpolation_degree=None,
        comment=None
    ))]
    fn new(
        object_name: String,
        object_id: String,
        ref_frame_a: String,
        ref_frame_b: String,
        start_time: String,
        stop_time: String,
        time_system: String,
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
        let attitude_type = AttitudeTypeType::from_str(&attitude_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Self {
            inner: core_aem::AemMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                object_id,
                center_name,
                ref_frame_a,
                ref_frame_b,
                time_system,
                start_time: parse_calendar_epoch(&start_time)?,
                stop_time: parse_calendar_epoch(&stop_time)?,
                useable_start_time: useable_start_time
                    .map(|s| parse_calendar_epoch(&s))
                    .transpose()?,
                useable_stop_time: useable_stop_time
                    .map(|s| parse_calendar_epoch(&s))
                    .transpose()?,
                attitude_type,
                euler_rot_seq: euler_rot_seq
                    .map(|s| RotSeq::from_str(&s))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                angvel_frame,
                interpolation_method,
                interpolation_degree: parse_interpolation_degree(interpolation_degree)?,
            },
        })
    }

    /// Validate the metadata section against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include Object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// Examples: EUTELSAT W1
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
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
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// capital letter for the identification of the part brought into space by the launch. In
    /// cases in which the asset is not listed in reference [ADM-2], the UN Office of Outer Space
    /// Affairs designator index format is not used, or the content cannot be disclosed, the value
    /// should be set to UNKNOWN.
    ///
    /// Examples: 2000-052A
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
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

    /// Comments allowed only at the beginning of the Metadata section. Each comment line shall
    /// begin with this keyword.
    ///
    /// Examples: This is a comment.
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
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
    /// Examples: EARTH, STS-106
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

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// Examples: ICRF, SC_BODY_1, INSTRUMENT_A
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String {
        self.inner.ref_frame_a.clone()
    }

    #[setter]
    fn set_ref_frame_a(&mut self, value: String) {
        self.inner.ref_frame_a = value;
    }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// Examples: SC_BODY_1, INSTRUMENT_A
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String {
        self.inner.ref_frame_b.clone()
    }

    #[setter]
    fn set_ref_frame_b(&mut self, value: String) {
        self.inner.ref_frame_b = value;
    }

    /// Time system used for both attitude ephemeris data and metadata. The set of allowed values
    /// is described in annex B, subsection B2.
    ///
    /// Examples: UTC, TAI
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }

    #[setter]
    fn set_time_system(&mut self, value: String) {
        self.inner.time_system = value;
    }

    /// Start of TOTAL time span covered by attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_start_time(&self) -> String {
        self.inner.start_time.as_str().to_string()
    }

    #[setter]
    fn set_start_time(&mut self, value: String) -> PyResult<()> {
        self.inner.start_time = parse_calendar_epoch(&value)?;
        Ok(())
    }

    /// End of TOTAL time span covered by the attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_stop_time(&self) -> String {
        self.inner.stop_time.as_str().to_string()
    }

    #[setter]
    fn set_stop_time(&mut self, value: String) -> PyResult<()> {
        self.inner.stop_time = parse_calendar_epoch(&value)?;
        Ok(())
    }

    /// Optional start of USEABLE time span covered by attitude ephemeris data immediately
    /// following this metadata block. To allow for proper interpolation near the beginning/end of
    /// the attitude ephemeris data block, it may be necessary to utilize this keyword with values
    /// within the time span covered by the attitude ephemeris data records as denoted by the
    /// START/STOP_TIME time tags. The USEABLE_START_TIME time tag of a new block of ephemeris data
    /// must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous block.
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// :type: str | None
    #[getter]
    fn get_useable_start_time(&self) -> Option<String> {
        self.inner
            .useable_start_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_useable_start_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.useable_start_time = value.map(|s| parse_calendar_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
    /// this metadata block. (See also USEABLE_START_TIME.)
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// :type: str | None
    #[getter]
    fn get_useable_stop_time(&self) -> Option<String> {
        self.inner
            .useable_stop_time
            .as_ref()
            .map(|e| e.as_str().to_string())
    }

    #[setter]
    fn set_useable_stop_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.useable_stop_time = value.map(|s| parse_calendar_epoch(&s)).transpose()?;
        Ok(())
    }

    /// The type of information contained in the data lines. This keyword must have a value from the
    /// set specified at the right. (See table 4-4 for details of the data contained in each line.)
    ///
    /// Examples: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
    /// EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.3.
    ///
    /// :type: str
    #[getter]
    fn get_attitude_type(&self) -> String {
        self.inner.attitude_type.to_string()
    }

    #[setter]
    fn set_attitude_type(&mut self, value: String) -> PyResult<()> {
        self.inner.attitude_type =
            AttitudeTypeType::from_str(&value).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation. This keyword is applicable only if ATTITUDE_TYPE
    /// specifies the use of Euler angles.
    ///
    /// Examples: ZXZ, XYZ
    ///
    /// :type: str | None
    #[getter]
    fn get_euler_rot_seq(&self) -> Option<String> {
        self.inner.euler_rot_seq.as_ref().map(|s| s.to_string())
    }

    #[setter]
    fn set_euler_rot_seq(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.euler_rot_seq = value
            .map(|s| RotSeq::from_str(&s))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// The frame of reference in which angular velocity data are specified. The set of allowed
    /// values is described in annex B, subsection B3. This keyword is applicable only if
    /// ATTITUDE_TYPE specifies the use of angular velocities in conjunction with either
    /// quaternions or Euler angles.
    ///
    /// Examples: ICRF, SC_BODY_1
    ///
    /// :type: str | None
    #[getter]
    fn get_angvel_frame(&self) -> Option<String> {
        self.inner.angvel_frame.clone()
    }

    #[setter]
    fn set_angvel_frame(&mut self, value: Option<String>) {
        self.inner.angvel_frame = value;
    }

    /// Recommended interpolation method for attitude ephemeris data in the block immediately
    /// following this metadata block.
    ///
    /// Examples: LINEAR, HERMITE, LAGRANGE
    ///
    /// :type: str | None
    #[getter]
    fn get_interpolation_method(&self) -> Option<String> {
        self.inner.interpolation_method.clone()
    }

    #[setter]
    fn set_interpolation_method(&mut self, value: Option<String>) {
        self.inner.interpolation_method = value;
    }

    /// Recommended interpolation degree for attitude ephemeris data in the block immediately
    /// following this metadata block. It must be an integer value. This keyword must be used if
    /// the ‘INTERPOLATION_METHOD’ keyword is used.
    ///
    /// Examples: 1, 5
    ///
    /// :type: int | None
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree.map(|d| d.0.get())
    }

    #[setter]
    fn set_interpolation_degree(&mut self, value: Option<u32>) -> PyResult<()> {
        self.inner.interpolation_degree = parse_interpolation_degree(value)?;
        Ok(())
    }
}

/// AEM Data Section.
///
/// Parameters
/// ----------
///     attitude_states : list[AttitudeState]
///     Attitude states.
///     attitude_type : str
///     CCSDS attitude type shared by every state.
///     comment : list[str], optional
///     Comments.
#[gen_stub_pyclass]
#[pyclass]
pub struct AemData {
    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.4.
    ///
    /// :type: list[str]
    #[pyo3(get, set)]
    comment: Vec<String>,

    attitude_states: Py<PyList>,
    attitude_type: AttitudeTypeType,
}

impl AemData {
    fn from_core(py: Python<'_>, value: core_aem::AemData) -> PyResult<Self> {
        let mut attitude_type = None;
        let mut states = Vec::with_capacity(value.attitude_states.len());
        for state in value.attitude_states {
            let this_type = state.attitude_type();
            if attitude_type
                .as_ref()
                .is_some_and(|existing| existing != &this_type)
            {
                return Err(PyValueError::new_err(
                    "AEM data contains mixed attitude state types",
                ));
            }
            attitude_type = Some(this_type);
            let (epoch, values) = state.into_epoch_values();
            states.push(Py::new(py, AttitudeState { epoch, values })?);
        }
        Ok(Self {
            comment: value.comment,
            attitude_states: PyList::new(py, states)?.unbind(),
            attitude_type: attitude_type
                .ok_or_else(|| PyValueError::new_err("AEM data requires an attitude state"))?,
        })
    }

    fn state_values(
        &self,
        py: Python<'_>,
    ) -> PyResult<Vec<(ccsds_ndm::types::CalendarEpoch, Vec<f64>)>> {
        self.attitude_states
            .bind(py)
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value
                    .extract::<PyRef<'_, AttitudeState>>()
                    .map(|state| (state.epoch, state.values.clone()))
                    .map_err(|_| {
                        PyValueError::new_err(format!(
                            "attitude_states[{index}] must be AttitudeState"
                        ))
                    })
            })
            .collect()
    }

    fn validate_widths(
        &self,
        values: &[(ccsds_ndm::types::CalendarEpoch, Vec<f64>)],
    ) -> PyResult<()> {
        let expected = self.attitude_type.value_count();
        if let Some((_, values)) = values.iter().find(|(_, values)| values.len() != expected) {
            return Err(PyValueError::new_err(format!(
                "ATTITUDE_TYPE {} requires {expected} values per state, got {}",
                self.attitude_type,
                values.len()
            )));
        }
        Ok(())
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_aem::AemData> {
        let values = self.state_values(py)?;
        self.validate_widths(&values)?;
        let attitude_states = values
            .into_iter()
            .map(|(epoch, values)| {
                ccsds_ndm::common::AemAttitudeState::from_values(
                    epoch,
                    &values,
                    &self.attitude_type,
                )
                .ok_or_else(|| PyValueError::new_err("attitude state width changed"))
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(core_aem::AemData {
            comment: self.comment.clone(),
            attitude_states,
        })
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl AemData {
    #[new]
    #[pyo3(signature = (attitude_states, attitude_type, comment=None))]
    fn new(
        py: Python<'_>,
        attitude_states: Vec<Py<AttitudeState>>,
        attitude_type: String,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let attitude_type = parse_attitude_type(&attitude_type)?;
        let expected = attitude_type.value_count();
        if let Some(state) = attitude_states
            .iter()
            .find(|state| state.borrow(py).values.len() != expected)
        {
            return Err(PyValueError::new_err(format!(
                "ATTITUDE_TYPE {attitude_type} requires {expected} values per state, got {}",
                state.borrow(py).values.len()
            )));
        }

        Ok(Self {
            comment: comment.unwrap_or_default(),
            attitude_states: PyList::new(py, attitude_states)?.unbind(),
            attitude_type,
        })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("AemData(states={})", self.attitude_states.bind(py).len())
    }

    /// Validate the data section against CCSDS rules.
    fn validate(&self, py: Python<'_>, attitude_type: String) -> PyResult<()> {
        let attitude_type = AttitudeTypeType::from_str(&attitude_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.to_core(py)?
            .validate(&attitude_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    #[pyo3(signature = (epochs, array, attitude_type, comment=None))]
    fn from_numpy(
        py: Python<'_>,
        epochs: Vec<String>,
        array: PyReadonlyArray2<f64>,
        attitude_type: String,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let shape = array.shape();
        if shape.len() != 2 {
            return Err(PyValueError::new_err("NumPy array must be 2-dimensional"));
        }
        if epochs.len() != shape[0] {
            return Err(PyValueError::new_err(
                "Number of epochs must match number of rows in NumPy array",
            ));
        }

        let resolved_type = parse_attitude_type(&attitude_type)?;
        let expected_cols = resolved_type.value_count();
        if shape[1] != expected_cols {
            return Err(PyValueError::new_err(format!(
                "ATTITUDE_TYPE {} requires {} columns, got {}",
                resolved_type, expected_cols, shape[1]
            )));
        }

        let array_view = array.as_array();
        let mut attitude_states = Vec::with_capacity(shape[0]);

        for (i, epoch_str) in epochs.iter().enumerate() {
            let row = array_view.row(i);
            let row_values: Vec<f64> = row.iter().copied().collect();
            attitude_states.push(Py::new(
                py,
                AttitudeState {
                    epoch: parse_calendar_epoch(epoch_str)?,
                    values: row_values,
                },
            )?);
        }

        Ok(Self {
            comment: comment.unwrap_or_default(),
            attitude_states: PyList::new(py, attitude_states)?.unbind(),
            attitude_type: resolved_type,
        })
    }

    /// Attitude ephemeris data lines.
    ///
    /// CCSDS Reference: 504.0-B-2, Section 4.2.4.
    ///
    /// :type: list[AttitudeState]
    #[gen_stub(override_return_type(type_repr = "list[AttitudeState]"))]
    #[getter]
    fn get_attitude_states(&self, py: Python<'_>) -> Py<PyList> {
        self.attitude_states.clone_ref(py)
    }

    #[setter]
    fn set_attitude_states(&mut self, attitude_states: Vec<Py<AttitudeState>>) -> PyResult<()> {
        Python::attach(|py| {
            if attitude_states.is_empty() {
                self.attitude_states = PyList::empty(py).unbind();
                return Ok(());
            }

            let expected = self.attitude_type.value_count();
            if attitude_states
                .iter()
                .any(|state| state.borrow(py).values.len() != expected)
            {
                return Err(PyValueError::new_err(format!(
                    "ATTITUDE_TYPE {} requires {expected} values per state",
                    self.attitude_type
                )));
            }
            self.attitude_states = PyList::new(py, attitude_states)?.unbind();
            Ok(())
        })
    }

    /// Epochs for attitude states (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_attitude_states_epochs(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        self.attitude_states
            .bind(py)
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value
                    .extract::<PyRef<'_, AttitudeState>>()
                    .map(|state| state.epoch.as_str().to_string())
                    .map_err(|_| {
                        PyValueError::new_err(format!(
                            "attitude_states[{index}] must be AttitudeState"
                        ))
                    })
            })
            .collect()
    }

    #[setter]
    fn set_attitude_states_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        Python::attach(|py| {
            let states = self.attitude_states.bind(py);
            if states.is_empty() {
                return Err(PyValueError::new_err(
                    "Cannot set epochs when no attitude states exist; create states first",
                ));
            }

            if epochs.len() != states.len() {
                return Err(PyValueError::new_err(
                    "Number of epochs must match number of attitude states",
                ));
            }

            // Validate every epoch and every element type before mutating anything, so a failure
            // partway through the list cannot leave the earlier records already rewritten.
            let mut parsed = Vec::with_capacity(epochs.len());
            for (index, epoch) in epochs.iter().enumerate() {
                let value = states.get_item(index)?;
                value
                    .extract::<PyRefMut<'_, AttitudeState>>()
                    .map_err(|_| {
                        PyValueError::new_err(format!(
                            "attitude_states[{index}] must be AttitudeState"
                        ))
                    })?;
                parsed.push(parse_calendar_epoch(epoch)?);
            }

            for (index, epoch) in parsed.into_iter().enumerate() {
                let value = states.get_item(index)?;
                let mut state = value
                    .extract::<PyRefMut<'_, AttitudeState>>()
                    .map_err(|_| {
                        PyValueError::new_err(format!(
                            "attitude_states[{index}] must be AttitudeState"
                        ))
                    })?;
                state.epoch = epoch;
            }
            Ok(())
        })
    }

    /// Get attitude states as a 2D NumPy array.
    ///
    /// Use `attitude_states_epochs` for the corresponding epochs.
    ///
    /// Supports all AEM attitude state types, but all rows must be of the same type.
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_attitude_states_numpy<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Py<numpy::PyArray2<f64>>> {
        let states = self.state_values(py)?;
        if states.is_empty() {
            let array = PyArray::from_vec(py, Vec::<f64>::new())
                .reshape([0, 0])
                .unwrap();
            return Ok(array.into());
        }

        self.validate_widths(&states)?;
        let expected_cols = self.attitude_type.value_count();
        let data = states
            .into_iter()
            .flat_map(|(_, values)| values)
            .collect::<Vec<_>>();

        let array = PyArray::from_vec(py, data)
            .reshape([self.attitude_states.bind(py).len(), expected_cols])
            .unwrap();
        Ok(array.into())
    }

    #[setter]
    fn set_attitude_states_numpy(&mut self, array: PyReadonlyArray2<f64>) -> PyResult<()> {
        Python::attach(|py| {
            let shape = array.shape();
            if shape.len() != 2 {
                return Err(PyValueError::new_err("NumPy array must be 2-dimensional"));
            }
            let states = self.attitude_states.bind(py);
            if states.is_empty() {
                return Err(PyValueError::new_err(
                    "Attitude epochs are missing; set attitude_states_epochs or use from_numpy",
                ));
            }
            if states.len() != shape[0] {
                return Err(PyValueError::new_err(
                    "Number of rows must match number of attitude states",
                ));
            }

            let expected_cols = self.attitude_type.value_count();
            if shape[1] != expected_cols {
                return Err(PyValueError::new_err(format!(
                    "NumPy array must have {} columns for this attitude state type",
                    expected_cols
                )));
            }
            let array_view = array.as_array();
            for i in 0..shape[0] {
                let row = array_view.row(i);
                let row_values: Vec<f64> = row.iter().copied().collect();
                let value = states.get_item(i)?;
                let mut state = value
                    .extract::<PyRefMut<'_, AttitudeState>>()
                    .map_err(|_| {
                        PyValueError::new_err(format!("attitude_states[{i}] must be AttitudeState"))
                    })?;
                state.values = row_values;
            }
            Ok(())
        })
    }
}

#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct AttitudeState {
    pub epoch: ccsds_ndm::types::CalendarEpoch,
    #[pyo3(get, set)]
    pub values: Vec<f64>,
}

#[gen_stub_pymethods]
#[pymethods]
impl AttitudeState {
    #[new]
    fn new(epoch: String, values: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            epoch: parse_calendar_epoch(&epoch)?,
            values,
        })
    }

    #[getter]
    fn get_epoch(&self) -> String {
        self.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.epoch = parse_calendar_epoch(&value)?;
        Ok(())
    }
}
