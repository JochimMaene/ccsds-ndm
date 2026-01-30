// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::types::parse_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use ccsds_ndm::types::RotSeq;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{parse_reference_frame, parse_time_system};

use std::str::FromStr;

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
    ///
    /// :type: AdmHeader
    #[getter]
    fn get_header(&self) -> AdmHeader {
        AdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    /// AEM Segments.
    ///
    /// :type: list[AemSegment]
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

    /// AEM Metadata Section.
    ///
    /// :type: AemMetadata
    #[getter]
    fn get_metadata(&self) -> AemMetadata {
        AemMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    /// AEM Data Section.
    ///
    /// :type: AemData
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
    #[pyo3(signature = (
        object_name,
        object_id,
        ref_frame_a=None,
        ref_frame_b=None,
        start_time=None,
        stop_time=None,
        time_system=None,
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
        ref_frame_a: Option<Bound<'_, PyAny>>,
        ref_frame_b: Option<Bound<'_, PyAny>>,
        start_time: Option<String>,
        stop_time: Option<String>,
        time_system: Option<Bound<'_, PyAny>>,
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

        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };
        let ref_frame_a = match ref_frame_a {
            Some(ref ob) => parse_reference_frame(ob)?,
            None => "GCRF".to_string(),
        };
        let ref_frame_b = match ref_frame_b {
            Some(ref ob) => parse_reference_frame(ob)?,
            None => "GCRF".to_string(),
        };
        let start_time = start_time.ok_or_else(|| PyValueError::new_err("start_time is required"))?;
        let stop_time = stop_time.ok_or_else(|| PyValueError::new_err("stop_time is required"))?;

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
    ///
    /// :type: str
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
    ///
    /// :type: str
    #[getter]
    fn get_object_id(&self) -> String {
        self.inner.object_id.clone()
    }

    /// Comments allowed only at the beginning of the Metadata section. Each comment line shall
    /// begin with this keyword.
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
    /// Examples: EARTH, STS-106
    ///
    /// :type: str | None
    #[getter]
    fn get_center_name(&self) -> Option<String> {
        self.inner.center_name.clone()
    }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// Examples: ICRF, SC_BODY_1, INSTRUMENT_A
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String {
        self.inner.ref_frame_a.clone()
    }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// Examples: SC_BODY_1, INSTRUMENT_A
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String {
        self.inner.ref_frame_b.clone()
    }

    /// Time system used for both attitude ephemeris data and metadata. The set of allowed values
    /// is described in annex B, subsection B2.
    ///
    /// Examples: UTC, TAI
    ///
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }

    /// Start of TOTAL time span covered by attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// :type: str
    #[getter]
    fn get_start_time(&self) -> String {
        self.inner.start_time.as_str().to_string()
    }

    /// End of TOTAL time span covered by the attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// :type: str
    #[getter]
    fn get_stop_time(&self) -> String {
        self.inner.stop_time.as_str().to_string()
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
        self.inner.useable_start_time.as_ref().map(|e| e.as_str().to_string())
    }

    /// Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
    /// this metadata block. (See also USEABLE_START_TIME.)
    ///
    /// Examples: 1996-12-18T14:28:15.11
    ///
    /// :type: str | None
    #[getter]
    fn get_useable_stop_time(&self) -> Option<String> {
        self.inner.useable_stop_time.as_ref().map(|e| e.as_str().to_string())
    }

    /// The type of information contained in the data lines. This keyword must have a value from the
    /// set specified at the right. (See table 4-4 for details of the data contained in each line.)
    ///
    /// Examples: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
    /// EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
    ///
    /// :type: str
    #[getter]
    fn get_attitude_type(&self) -> String {
        self.inner.attitude_type.clone()
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

    /// Recommended interpolation degree for attitude ephemeris data in the block immediately
    /// following this metadata block. It must be an integer value. This keyword must be used if
    /// the ‘INTERPOLATION_METHOD’ keyword is used.
    ///
    /// Examples: 1, 5
    ///
    /// :type: int | None
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree.map(|d| d.get())
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
                    let state = ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
                        epoch: s.epoch,
                        quaternion: Quaternion {
                            q1: s.values.get(0).copied().unwrap_or(0.0),
                            q2: s.values.get(1).copied().unwrap_or(0.0),
                            q3: s.values.get(2).copied().unwrap_or(0.0),
                            qc: s.values.get(3).copied().unwrap_or(1.0),
                        },
                    });
                    state.into()
                }).collect(),
            },
        }
    }

    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    /// Attitude ephemeris data lines.
    ///
    /// :type: list[AttitudeState]
    #[getter]
    fn get_attitude_states(&self) -> Vec<AttitudeState> {
        self.inner
            .attitude_states
            .iter()
            .map(|s| {
                // Simplified mapping back to generic AttitudeState
                let (epoch, values) = match s.content() {
                    Some(ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v)) =>
                        (v.epoch, vec![v.quaternion.q1, v.quaternion.q2, v.quaternion.q3, v.quaternion.qc]),
                    _ => (ccsds_ndm::types::Epoch::new("1958-01-01T00:00:00").unwrap(), vec![]), // TODO: implement other variants
                };
                AttitudeState { epoch, values }
            })
            .collect()
    }

    /// Get attitude states as a tuple of epoch strings and a 2D NumPy array.
    ///
    /// :type: tuple[list[str], numpy.ndarray]
    #[getter]
    fn get_attitude_states_numpy<'py>(&self, py: Python<'py>) -> (Vec<String>, Py<PyAny>) {
        let mut epochs = Vec::with_capacity(self.inner.attitude_states.len());
        let mut max_cols = 0;

        // First pass to find max columns and collect epochs
        for s in &self.inner.attitude_states {
            if let Some(content) = s.content() {
                let values = match content {
                    ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v) =>
                        (v.epoch, vec![v.quaternion.q1, v.quaternion.q2, v.quaternion.q3, v.quaternion.qc]),
                    _ => (ccsds_ndm::types::Epoch::new("1958-01-01T00:00:00").unwrap(), vec![]),
                };
                epochs.push(values.0.as_str().to_string());
                max_cols = max_cols.max(values.1.len());
            }
        }

        let mut data = Vec::with_capacity(epochs.len() * max_cols);
        for s in &self.inner.attitude_states {
            if let Some(content) = s.content() {
                let values = match content {
                    ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v) =>
                        vec![v.quaternion.q1, v.quaternion.q2, v.quaternion.q3, v.quaternion.qc],
                    _ => vec![],
                };
                let mut row = values;
                row.resize(max_cols, f64::NAN);
                data.extend(row);
            }
        }

        let array = PyArray::from_vec(py, data)
            .reshape([epochs.len(), max_cols])
            .unwrap();
        (epochs, array.into())
    }

    #[setter]
    fn set_attitude_states_numpy(&mut self, value: (Vec<String>, PyReadonlyArray2<f64>)) -> PyResult<()> {
        let (epochs, array) = value;
        let shape = array.shape();
        if epochs.len() != shape[0] {
            return Err(PyValueError::new_err("Number of epochs must match number of rows in NumPy array"));
        }

        let array_view = array.as_array();
        let mut attitude_states = Vec::with_capacity(shape[0]);

        for (i, epoch_str) in epochs.iter().enumerate() {
            let row = array_view.row(i);
            // NOTE: Simplified to QuaternionEphemeris for now, matching other implementation
            use ccsds_ndm::common::{QuaternionEphemeris, Quaternion};
            let state = ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
                epoch: parse_epoch(epoch_str)?,
                quaternion: Quaternion {
                    q1: row.get(0).copied().unwrap_or(0.0),
                    q2: row.get(1).copied().unwrap_or(0.0),
                    q3: row.get(2).copied().unwrap_or(0.0),
                    qc: row.get(3).copied().unwrap_or(1.0),
                },
            });
            attitude_states.push(state.into());
        }
        self.inner.attitude_states = attitude_states;
        Ok(())
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

    #[getter]
    fn get_epoch(&self) -> String {
        self.epoch.as_str().to_string()
    }

    #[getter]
    fn get_values(&self) -> Vec<f64> {
        self.values.clone()
    }
}
