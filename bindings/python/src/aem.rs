// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::types::parse_calendar_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::types::{
    Angle, AngleRate, AttitudeTypeType, Duration, InterpolationDegree, QuaternionDotComponent,
    RotSeq,
};
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

use std::str::FromStr;

fn expected_values_len(attitude_type: &AttitudeTypeType) -> usize {
    match attitude_type {
        AttitudeTypeType::Quaternion => 4,
        AttitudeTypeType::QuaternionDerivative => 8,
        AttitudeTypeType::QuaternionAngVel => 7,
        AttitudeTypeType::EulerAngle => 3,
        AttitudeTypeType::EulerAngleDerivative => 6,
        AttitudeTypeType::EulerAngleAngVel => 6,
        AttitudeTypeType::Spin => 4,
        AttitudeTypeType::SpinNutation => 7,
        AttitudeTypeType::SpinNutationMom => 7,
    }
}

fn infer_attitude_type_from_values_len(values_len: usize) -> PyResult<AttitudeTypeType> {
    match values_len {
        3 => Ok(AttitudeTypeType::EulerAngle),
        4 => Err(PyValueError::new_err(
            "Ambiguous 4-column AEM data; specify attitude_type explicitly (QUATERNION or SPIN)",
        )),
        6 => Err(PyValueError::new_err(
            "Ambiguous 6-column AEM data; specify attitude_type explicitly (EULER_ANGLE/DERIVATIVE or EULER_ANGLE/ANGVEL)",
        )),
        7 => Err(PyValueError::new_err(
            "Ambiguous 7-column AEM data; specify attitude_type explicitly (QUATERNION/ANGVEL, SPIN/NUTATION, or SPIN/NUTATION_MOM)",
        )),
        8 => Ok(AttitudeTypeType::QuaternionDerivative),
        _ => Err(PyValueError::new_err(format!(
            "Unsupported AEM data width {}. Allowed widths are 3, 4, 6, 7, 8",
            values_len
        ))),
    }
}

fn parse_attitude_type_or_infer(
    attitude_type: Option<&str>,
    values_len: usize,
) -> PyResult<AttitudeTypeType> {
    match attitude_type {
        Some(raw) => {
            AttitudeTypeType::from_str(raw).map_err(|e| PyValueError::new_err(e.to_string()))
        }
        None => infer_attitude_type_from_values_len(values_len),
    }
}

fn build_state_from_values(
    epoch: ccsds_ndm::types::CalendarEpoch,
    values: &[f64],
    attitude_type: &AttitudeTypeType,
) -> PyResult<core_aem::AemAttitudeStateWrapper> {
    let expected = expected_values_len(attitude_type);
    if values.len() != expected {
        return Err(PyValueError::new_err(format!(
            "ATTITUDE_TYPE {} requires {} values per row, got {}",
            attitude_type,
            expected,
            values.len()
        )));
    }

    use ccsds_ndm::common::{
        AemAttitudeState, AngVel, EulerAngle, EulerAngleAngVel, EulerAngleDerivative, Quaternion,
        QuaternionAngVel, QuaternionDerivative, QuaternionDot, QuaternionEphemeris, Spin,
        SpinNutation, SpinNutationMom,
    };

    let state = match attitude_type {
        AttitudeTypeType::Quaternion => {
            AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
                epoch,
                quaternion: Quaternion {
                    q1: values[0],
                    q2: values[1],
                    q3: values[2],
                    qc: values[3],
                },
            })
        }
        AttitudeTypeType::QuaternionDerivative => {
            AemAttitudeState::QuaternionDerivative(QuaternionDerivative {
                epoch,
                quaternion: Quaternion {
                    q1: values[0],
                    q2: values[1],
                    q3: values[2],
                    qc: values[3],
                },
                quaternion_dot: QuaternionDot {
                    q1_dot: QuaternionDotComponent {
                        value: values[4],
                        units: None,
                    },
                    q2_dot: QuaternionDotComponent {
                        value: values[5],
                        units: None,
                    },
                    q3_dot: QuaternionDotComponent {
                        value: values[6],
                        units: None,
                    },
                    qc_dot: QuaternionDotComponent {
                        value: values[7],
                        units: None,
                    },
                },
            })
        }
        AttitudeTypeType::QuaternionAngVel => {
            AemAttitudeState::QuaternionAngVel(QuaternionAngVel {
                epoch,
                quaternion: Quaternion {
                    q1: values[0],
                    q2: values[1],
                    q3: values[2],
                    qc: values[3],
                },
                ang_vel: AngVel {
                    angvel_x: AngleRate {
                        value: values[4],
                        units: None,
                    },
                    angvel_y: AngleRate {
                        value: values[5],
                        units: None,
                    },
                    angvel_z: AngleRate {
                        value: values[6],
                        units: None,
                    },
                },
            })
        }
        AttitudeTypeType::EulerAngle => AemAttitudeState::EulerAngle(EulerAngle {
            epoch,
            angle_1: Angle {
                value: values[0],
                units: None,
            },
            angle_2: Angle {
                value: values[1],
                units: None,
            },
            angle_3: Angle {
                value: values[2],
                units: None,
            },
        }),
        AttitudeTypeType::EulerAngleDerivative => {
            AemAttitudeState::EulerAngleDerivative(EulerAngleDerivative {
                epoch,
                angle_1: Angle {
                    value: values[0],
                    units: None,
                },
                angle_2: Angle {
                    value: values[1],
                    units: None,
                },
                angle_3: Angle {
                    value: values[2],
                    units: None,
                },
                angle_1_dot: AngleRate {
                    value: values[3],
                    units: None,
                },
                angle_2_dot: AngleRate {
                    value: values[4],
                    units: None,
                },
                angle_3_dot: AngleRate {
                    value: values[5],
                    units: None,
                },
            })
        }
        AttitudeTypeType::EulerAngleAngVel => {
            AemAttitudeState::EulerAngleAngVel(EulerAngleAngVel {
                epoch,
                angle_1: Angle {
                    value: values[0],
                    units: None,
                },
                angle_2: Angle {
                    value: values[1],
                    units: None,
                },
                angle_3: Angle {
                    value: values[2],
                    units: None,
                },
                angvel_x: AngleRate {
                    value: values[3],
                    units: None,
                },
                angvel_y: AngleRate {
                    value: values[4],
                    units: None,
                },
                angvel_z: AngleRate {
                    value: values[5],
                    units: None,
                },
            })
        }
        AttitudeTypeType::Spin => AemAttitudeState::Spin(Spin {
            epoch,
            spin_alpha: Angle {
                value: values[0],
                units: None,
            },
            spin_delta: Angle {
                value: values[1],
                units: None,
            },
            spin_angle: Angle {
                value: values[2],
                units: None,
            },
            spin_angle_vel: AngleRate {
                value: values[3],
                units: None,
            },
        }),
        AttitudeTypeType::SpinNutation => AemAttitudeState::SpinNutation(SpinNutation {
            epoch,
            spin_alpha: Angle {
                value: values[0],
                units: None,
            },
            spin_delta: Angle {
                value: values[1],
                units: None,
            },
            spin_angle: Angle {
                value: values[2],
                units: None,
            },
            spin_angle_vel: AngleRate {
                value: values[3],
                units: None,
            },
            nutation: Angle {
                value: values[4],
                units: None,
            },
            nutation_per: Duration {
                value: values[5],
                units: None,
            },
            nutation_phase: Angle {
                value: values[6],
                units: None,
            },
        }),
        AttitudeTypeType::SpinNutationMom => AemAttitudeState::SpinNutationMom(SpinNutationMom {
            epoch,
            spin_alpha: Angle {
                value: values[0],
                units: None,
            },
            spin_delta: Angle {
                value: values[1],
                units: None,
            },
            spin_angle: Angle {
                value: values[2],
                units: None,
            },
            spin_angle_vel: AngleRate {
                value: values[3],
                units: None,
            },
            momentum_alpha: Angle {
                value: values[4],
                units: None,
            },
            momentum_delta: Angle {
                value: values[5],
                units: None,
            },
            nutation_vel: AngleRate {
                value: values[6],
                units: None,
            },
        }),
    };

    Ok(state.into())
}

fn values_from_content(
    content: ccsds_ndm::common::AemAttitudeState,
) -> (ccsds_ndm::types::CalendarEpoch, Vec<f64>) {
    match content {
        ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v) => (
            v.epoch,
            vec![
                v.quaternion.q1,
                v.quaternion.q2,
                v.quaternion.q3,
                v.quaternion.qc,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(v) => (
            v.epoch,
            vec![
                v.quaternion.q1,
                v.quaternion.q2,
                v.quaternion.q3,
                v.quaternion.qc,
                v.quaternion_dot.q1_dot.value,
                v.quaternion_dot.q2_dot.value,
                v.quaternion_dot.q3_dot.value,
                v.quaternion_dot.qc_dot.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(v) => (
            v.epoch,
            vec![
                v.quaternion.q1,
                v.quaternion.q2,
                v.quaternion.q3,
                v.quaternion.qc,
                v.ang_vel.angvel_x.value,
                v.ang_vel.angvel_y.value,
                v.ang_vel.angvel_z.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::EulerAngle(v) => (
            v.epoch,
            vec![v.angle_1.value, v.angle_2.value, v.angle_3.value],
        ),
        ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(v) => (
            v.epoch,
            vec![
                v.angle_1.value,
                v.angle_2.value,
                v.angle_3.value,
                v.angle_1_dot.value,
                v.angle_2_dot.value,
                v.angle_3_dot.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(v) => (
            v.epoch,
            vec![
                v.angle_1.value,
                v.angle_2.value,
                v.angle_3.value,
                v.angvel_x.value,
                v.angvel_y.value,
                v.angvel_z.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::Spin(v) => (
            v.epoch,
            vec![
                v.spin_alpha.value,
                v.spin_delta.value,
                v.spin_angle.value,
                v.spin_angle_vel.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::SpinNutation(v) => (
            v.epoch,
            vec![
                v.spin_alpha.value,
                v.spin_delta.value,
                v.spin_angle.value,
                v.spin_angle_vel.value,
                v.nutation.value,
                v.nutation_per.value,
                v.nutation_phase.value,
            ],
        ),
        ccsds_ndm::common::AemAttitudeState::SpinNutationMom(v) => (
            v.epoch,
            vec![
                v.spin_alpha.value,
                v.spin_delta.value,
                v.spin_angle.value,
                v.spin_angle_vel.value,
                v.momentum_alpha.value,
                v.momentum_delta.value,
                v.nutation_vel.value,
            ],
        ),
    }
}

fn attitude_type_from_content(content: &ccsds_ndm::common::AemAttitudeState) -> AttitudeTypeType {
    match content {
        ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(_) => AttitudeTypeType::Quaternion,
        ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(_) => {
            AttitudeTypeType::QuaternionDerivative
        }
        ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(_) => {
            AttitudeTypeType::QuaternionAngVel
        }
        ccsds_ndm::common::AemAttitudeState::EulerAngle(_) => AttitudeTypeType::EulerAngle,
        ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(_) => {
            AttitudeTypeType::EulerAngleDerivative
        }
        ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(_) => {
            AttitudeTypeType::EulerAngleAngVel
        }
        ccsds_ndm::common::AemAttitudeState::Spin(_) => AttitudeTypeType::Spin,
        ccsds_ndm::common::AemAttitudeState::SpinNutation(_) => AttitudeTypeType::SpinNutation,
        ccsds_ndm::common::AemAttitudeState::SpinNutationMom(_) => {
            AttitudeTypeType::SpinNutationMom
        }
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
#[pyclass]
pub struct Aem {
    id: Option<String>,
    version: String,
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
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Aem, &value)?;
        self.version = value;
        Ok(())
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
    fn get_header(&self, py: Python<'_>) -> Py<AdmHeader> {
        self.header.clone_ref(py)
    }

    #[setter]
    fn set_header(&mut self, header: Py<AdmHeader>) {
        self.header = header;
    }

    /// AEM Segments.
    ///
    /// :type: list[AemSegment]
    #[getter]
    fn get_segments(&self, py: Python<'_>) -> Py<PyList> {
        self.segments.clone_ref(py)
    }

    #[setter]
    fn set_segments(&mut self, py: Python<'_>, segments: Vec<Py<AemSegment>>) -> PyResult<()> {
        self.segments = PyList::new(py, segments)?.unbind();
        Ok(())
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
    #[pyo3(signature = (data, format=None, *, max_input_bytes=None, max_records=None))]
    fn from_str(
        py: Python<'_>,
        data: &str,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_records);
        let inner = crate::api::parse_typed_with_options(data, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Parse an AEM from a KVN or XML file.
    #[staticmethod]
    #[pyo3(signature = (path, format=None, *, max_input_bytes=None, max_records=None))]
    fn from_file(
        py: Python<'_>,
        path: std::path::PathBuf,
        format: Option<&str>,
        max_input_bytes: Option<usize>,
        max_records: Option<usize>,
    ) -> PyResult<Self> {
        let options = crate::api::parse_options(max_input_bytes, max_records);
        let inner = crate::api::parse_typed_file_with_options(&path, format, &options)?;
        Self::from_core(py, inner)
    }

    /// Atomically write this AEM as KVN or XML.
    fn to_file(&self, py: Python<'_>, path: std::path::PathBuf, format: &str) -> PyResult<()> {
        crate::api::generate_file(&ccsds_ndm::Message::Aem(self.to_core(py)?), &path, format)
    }
}

#[pyclass]
pub struct AemSegment {
    metadata: Py<AemMetadata>,
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

#[pymethods]
impl AemSegment {
    #[new]
    fn new(metadata: Py<AemMetadata>, data: Py<AemData>) -> Self {
        Self { metadata, data }
    }

    /// AEM Metadata Section.
    ///
    /// :type: AemMetadata
    #[getter]
    fn get_metadata(&self, py: Python<'_>) -> Py<AemMetadata> {
        self.metadata.clone_ref(py)
    }

    #[setter]
    fn set_metadata(&mut self, metadata: Py<AemMetadata>) {
        self.metadata = metadata;
    }

    /// AEM Data Section.
    ///
    /// :type: AemData
    #[getter]
    fn get_data(&self, py: Python<'_>) -> Py<AemData> {
        self.data.clone_ref(py)
    }

    #[setter]
    fn set_data(&mut self, data: Py<AemData>) {
        self.data = data;
    }

    /// Validate the segment against CCSDS rules.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        self.to_core(py)?
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))
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
        ref_frame_a: Option<String>,
        ref_frame_b: Option<String>,
        start_time: Option<String>,
        stop_time: Option<String>,
        time_system: Option<String>,
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

        let time_system = time_system.unwrap_or_else(|| "UTC".to_string());
        let ref_frame_a = ref_frame_a.unwrap_or_else(|| "GCRF".to_string());
        let ref_frame_b = ref_frame_b.unwrap_or_else(|| "GCRF".to_string());
        let start_time =
            start_time.ok_or_else(|| PyValueError::new_err("start_time is required"))?;
        let stop_time = stop_time.ok_or_else(|| PyValueError::new_err("stop_time is required"))?;

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
                interpolation_degree: interpolation_degree
                    .and_then(NonZeroU32::new)
                    .map(InterpolationDegree),
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
    fn set_interpolation_degree(&mut self, value: Option<u32>) {
        self.inner.interpolation_degree = value
            .and_then(std::num::NonZeroU32::new)
            .map(InterpolationDegree);
    }
}

/// AEM Data Section.
#[pyclass]
pub struct AemData {
    comment: Vec<String>,
    attitude_states: Py<PyList>,
    attitude_type: Option<AttitudeTypeType>,
}

impl AemData {
    fn from_core(py: Python<'_>, value: core_aem::AemData) -> PyResult<Self> {
        let mut attitude_type = None;
        let mut states = Vec::with_capacity(value.attitude_states.len());
        for state in value.attitude_states {
            let content = state
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let this_type = attitude_type_from_content(&content);
            if attitude_type
                .as_ref()
                .is_some_and(|existing| existing != &this_type)
            {
                return Err(PyValueError::new_err(
                    "AEM data contains mixed attitude state types",
                ));
            }
            attitude_type = Some(this_type);
            let (epoch, values) = values_from_content(content);
            states.push(Py::new(py, AttitudeState { epoch, values })?);
        }
        Ok(Self {
            comment: value.comment,
            attitude_states: PyList::new(py, states)?.unbind(),
            attitude_type,
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

    fn resolved_type(
        &self,
        values: &[(ccsds_ndm::types::CalendarEpoch, Vec<f64>)],
    ) -> PyResult<Option<AttitudeTypeType>> {
        let Some((_, first)) = values.first() else {
            return Ok(self.attitude_type.clone());
        };
        if values.iter().any(|(_, values)| values.len() != first.len()) {
            return Err(PyValueError::new_err(
                "All attitude states must have the same number of values",
            ));
        }
        match self.attitude_type.as_ref() {
            Some(attitude_type) => {
                let expected = expected_values_len(attitude_type);
                if first.len() != expected {
                    return Err(PyValueError::new_err(format!(
                        "ATTITUDE_TYPE {attitude_type} requires {expected} values per state, got {}",
                        first.len()
                    )));
                }
                Ok(Some(attitude_type.clone()))
            }
            None => parse_attitude_type_or_infer(None, first.len()).map(Some),
        }
    }

    fn to_core(&self, py: Python<'_>) -> PyResult<core_aem::AemData> {
        let values = self.state_values(py)?;
        let attitude_type = self.resolved_type(&values)?;
        let attitude_states = match attitude_type {
            Some(attitude_type) => values
                .into_iter()
                .map(|(epoch, values)| build_state_from_values(epoch, &values, &attitude_type))
                .collect::<PyResult<Vec<_>>>()?,
            None => Vec::new(),
        };
        Ok(core_aem::AemData {
            comment: self.comment.clone(),
            attitude_states,
        })
    }
}

#[pymethods]
impl AemData {
    #[new]
    #[pyo3(signature = (attitude_states, attitude_type=None, comment=None))]
    fn new(
        py: Python<'_>,
        attitude_states: Vec<Py<AttitudeState>>,
        attitude_type: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let attitude_type = if attitude_states.is_empty() {
            attitude_type
                .as_deref()
                .map(AttitudeTypeType::from_str)
                .transpose()
                .map_err(|error| PyValueError::new_err(error.to_string()))?
        } else {
            let widths: std::collections::BTreeSet<usize> = attitude_states
                .iter()
                .map(|state| state.borrow(py).values.len())
                .collect();
            if widths.len() != 1 {
                return Err(PyValueError::new_err(
                    "All attitude states must have the same number of values",
                ));
            }
            let width = *widths.iter().next().unwrap();
            Some(parse_attitude_type_or_infer(
                attitude_type.as_deref(),
                width,
            )?)
        };

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
    #[pyo3(signature = (epochs, array, attitude_type=None, comment=None))]
    fn from_numpy(
        py: Python<'_>,
        epochs: Vec<String>,
        array: PyReadonlyArray2<f64>,
        attitude_type: Option<String>,
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

        let resolved_type = parse_attitude_type_or_infer(attitude_type.as_deref(), shape[1])?;
        let expected_cols = expected_values_len(&resolved_type);
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
            attitude_type: Some(resolved_type),
        })
    }

    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, comment: Vec<String>) {
        self.comment = comment;
    }

    /// Attitude ephemeris data lines.
    ///
    /// :type: list[AttitudeState]
    #[getter]
    fn get_attitude_states(&self, py: Python<'_>) -> Py<PyList> {
        self.attitude_states.clone_ref(py)
    }

    #[setter]
    fn set_attitude_states(
        &mut self,
        py: Python<'_>,
        attitude_states: Vec<Py<AttitudeState>>,
    ) -> PyResult<()> {
        if attitude_states.is_empty() {
            self.attitude_states = PyList::empty(py).unbind();
            return Ok(());
        }

        let widths: std::collections::BTreeSet<usize> = attitude_states
            .iter()
            .map(|state| state.borrow(py).values.len())
            .collect();
        if widths.len() != 1 {
            return Err(PyValueError::new_err(
                "All attitude states must have the same number of values",
            ));
        }

        let width = *widths.iter().next().unwrap();
        let resolved_type = if let Some(existing_type) = self.attitude_type.as_ref() {
            let existing_width = expected_values_len(existing_type);
            if existing_width != width {
                return Err(PyValueError::new_err(format!(
                    "Expected {} values per state based on existing data, got {}",
                    existing_width, width
                )));
            }
            existing_type.clone()
        } else {
            parse_attitude_type_or_infer(None, width)?
        };

        self.attitude_type = Some(resolved_type);
        self.attitude_states = PyList::new(py, attitude_states)?.unbind();
        Ok(())
    }

    /// Epochs for attitude states (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_attitude_states_epochs(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        Ok(self
            .state_values(py)?
            .into_iter()
            .map(|(epoch, _)| epoch.as_str().to_string())
            .collect())
    }

    #[setter]
    fn set_attitude_states_epochs(&mut self, py: Python<'_>, epochs: Vec<String>) -> PyResult<()> {
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

        for (index, epoch) in epochs.into_iter().enumerate() {
            let value = states.get_item(index)?;
            let mut state = value
                .extract::<PyRefMut<'_, AttitudeState>>()
                .map_err(|_| {
                    PyValueError::new_err(format!("attitude_states[{index}] must be AttitudeState"))
                })?;
            state.epoch = parse_calendar_epoch(&epoch)?;
        }
        Ok(())
    }

    /// Get attitude states as a 2D NumPy array.
    ///
    /// Use `attitude_states_epochs` for the corresponding epochs.
    ///
    /// Supports all AEM attitude state types, but all rows must be of the same type.
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_attitude_states_numpy<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        let states = self.state_values(py)?;
        if states.is_empty() {
            let array = PyArray::from_vec(py, Vec::<f64>::new())
                .reshape([0, 0])
                .unwrap();
            return Ok(array.into());
        }

        let resolved_type = self.resolved_type(&states)?.ok_or_else(|| {
            PyValueError::new_err("Attitude type is unavailable for non-empty data")
        })?;
        let first_values = states[0].1.clone();
        let expected_cols = first_values.len();

        let mut data = Vec::with_capacity(states.len() * expected_cols);
        data.extend(first_values);

        for (_, values) in states.into_iter().skip(1) {
            if values.len() != expected_cols {
                return Err(PyValueError::new_err(
                    "NumPy access requires all attitude states to have the same data width",
                ));
            }
            data.extend(values);
        }
        debug_assert_eq!(expected_values_len(&resolved_type), expected_cols);

        let array = PyArray::from_vec(py, data)
            .reshape([self.attitude_states.bind(py).len(), expected_cols])
            .unwrap();
        Ok(array.into())
    }

    #[setter]
    fn set_attitude_states_numpy(
        &mut self,
        py: Python<'_>,
        array: PyReadonlyArray2<f64>,
    ) -> PyResult<()> {
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

        let current = self.state_values(py)?;
        let resolved_type = self.resolved_type(&current)?.ok_or_else(|| {
            PyValueError::new_err("Attitude type is unavailable for non-empty data")
        })?;
        let expected_cols = expected_values_len(&resolved_type);
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
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AttitudeState {
    pub epoch: ccsds_ndm::types::CalendarEpoch,
    pub values: Vec<f64>,
}

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

    #[getter]
    fn get_values(&self) -> Vec<f64> {
        self.values.clone()
    }

    #[setter]
    fn set_values(&mut self, value: Vec<f64>) {
        self.values = value;
    }
}
