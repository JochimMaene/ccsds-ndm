// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::common::{parse_reference_frame, parse_time_system};
use crate::types::parse_epoch;
use ccsds_ndm::messages::aem as core_aem;
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::types::{
    Angle, AngleRate, AttitudeTypeType, Duration, InterpolationDegree, QuaternionDotComponent,
    RotSeq,
};
use ccsds_ndm::MessageType;
use numpy::{PyArray, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

use std::str::FromStr;

fn expected_values_len(attitude_type: &AttitudeTypeType) -> usize {
    match attitude_type {
        AttitudeTypeType::Quaternion | AttitudeTypeType::QuaternionUpper => 4,
        AttitudeTypeType::QuaternionDerivative | AttitudeTypeType::QuaternionDerivativeUpper => 8,
        AttitudeTypeType::QuaternionAngVel | AttitudeTypeType::QuaternionAngVelUpper => 7,
        AttitudeTypeType::EulerAngle | AttitudeTypeType::EulerAngleUpper => 3,
        AttitudeTypeType::EulerAngleDerivative | AttitudeTypeType::EulerAngleDerivativeUpper => 6,
        AttitudeTypeType::EulerAngleAngVel | AttitudeTypeType::EulerAngleAngVelUpper => 6,
        AttitudeTypeType::Spin | AttitudeTypeType::SpinUpper => 4,
        AttitudeTypeType::SpinNutation | AttitudeTypeType::SpinNutationUpper => 7,
        AttitudeTypeType::SpinNutationMom | AttitudeTypeType::SpinNutationMomUpper => 7,
    }
}

fn infer_attitude_type_from_values_len(values_len: usize) -> PyResult<AttitudeTypeType> {
    match values_len {
        3 => Ok(AttitudeTypeType::EulerAngleUpper),
        4 => Err(PyValueError::new_err(
            "Ambiguous 4-column AEM data; specify attitude_type explicitly (QUATERNION or SPIN)",
        )),
        6 => Err(PyValueError::new_err(
            "Ambiguous 6-column AEM data; specify attitude_type explicitly (EULER_ANGLE/DERIVATIVE or EULER_ANGLE/ANGVEL)",
        )),
        7 => Err(PyValueError::new_err(
            "Ambiguous 7-column AEM data; specify attitude_type explicitly (QUATERNION/ANGVEL, SPIN/NUTATION, or SPIN/NUTATION_MOM)",
        )),
        8 => Ok(AttitudeTypeType::QuaternionDerivativeUpper),
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
    epoch: ccsds_ndm::types::Epoch,
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
        AttitudeTypeType::Quaternion | AttitudeTypeType::QuaternionUpper => {
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
        AttitudeTypeType::QuaternionDerivative | AttitudeTypeType::QuaternionDerivativeUpper => {
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
        AttitudeTypeType::QuaternionAngVel | AttitudeTypeType::QuaternionAngVelUpper => {
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
        AttitudeTypeType::EulerAngle | AttitudeTypeType::EulerAngleUpper => {
            AemAttitudeState::EulerAngle(EulerAngle {
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
            })
        }
        AttitudeTypeType::EulerAngleDerivative | AttitudeTypeType::EulerAngleDerivativeUpper => {
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
        AttitudeTypeType::EulerAngleAngVel | AttitudeTypeType::EulerAngleAngVelUpper => {
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
        AttitudeTypeType::Spin | AttitudeTypeType::SpinUpper => AemAttitudeState::Spin(Spin {
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
        AttitudeTypeType::SpinNutation | AttitudeTypeType::SpinNutationUpper => {
            AemAttitudeState::SpinNutation(SpinNutation {
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
            })
        }
        AttitudeTypeType::SpinNutationMom | AttitudeTypeType::SpinNutationMomUpper => {
            AemAttitudeState::SpinNutationMom(SpinNutationMom {
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
            })
        }
    };

    Ok(state.into())
}

fn values_from_content(
    content: ccsds_ndm::common::AemAttitudeState,
) -> (ccsds_ndm::types::Epoch, Vec<f64>) {
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
        ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(_) => {
            AttitudeTypeType::QuaternionUpper
        }
        ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(_) => {
            AttitudeTypeType::QuaternionDerivativeUpper
        }
        ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(_) => {
            AttitudeTypeType::QuaternionAngVelUpper
        }
        ccsds_ndm::common::AemAttitudeState::EulerAngle(_) => AttitudeTypeType::EulerAngleUpper,
        ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(_) => {
            AttitudeTypeType::EulerAngleDerivativeUpper
        }
        ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(_) => {
            AttitudeTypeType::EulerAngleAngVelUpper
        }
        ccsds_ndm::common::AemAttitudeState::Spin(_) => AttitudeTypeType::SpinUpper,
        ccsds_ndm::common::AemAttitudeState::SpinNutation(_) => AttitudeTypeType::SpinNutationUpper,
        ccsds_ndm::common::AemAttitudeState::SpinNutationMom(_) => {
            AttitudeTypeType::SpinNutationMomUpper
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
                id: Some("CCSDS_AEM_VERS".to_string()),
                version: "2.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Aem(object_name='{}', segments={})",
            self.inner
                .body
                .segment
                .first()
                .map(|s| s.metadata.object_name.clone())
                .unwrap_or_default(),
            self.inner.body.segment.len()
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
        crate::common::validate_version(ccsds_ndm::validation::MessageKind::Aem, &value)?;
        self.inner.version = value;
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
    fn get_header(&self) -> AdmHeader {
        AdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    #[setter]
    fn set_header(&mut self, header: AdmHeader) {
        self.inner.header = header.inner;
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

    #[setter]
    fn set_segments(&mut self, segments: Vec<AemSegment>) {
        self.inner.body.segment = segments.into_iter().map(|s| s.inner).collect();
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
        if strict {
            self.inner
                .validate()
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(None)
        } else {
            let mut issues = Vec::new();
            let _ = ccsds_ndm::validation::with_validation_mode(
                ccsds_ndm::validation::ValidationMode::Lenient,
                || match self.inner.validate() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        issues.push(e.to_string());
                        Ok(())
                    }
                },
            );

            let warnings = ccsds_ndm::validation::take_warnings();
            for w in warnings {
                issues.push(w.error.to_string());
            }

            if issues.is_empty() {
                Ok(None)
            } else {
                Ok(Some(issues))
            }
        }
    }

    /// Serialize to string.
    #[pyo3(signature = (format, validate=true))]
    fn to_str(&self, format: &str, validate: bool) -> PyResult<String> {
        if validate {
            self.validate(true)?;
        }
        match format {
            "kvn" => self
                .inner
                .to_kvn()
                .map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => ccsds_ndm::xml::to_string(&self.inner)
                .map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'",
                other
            ))),
        }
    }

    /// Write to file.
    #[pyo3(signature = (path, format, validate=true))]
    fn to_file(&self, path: &str, format: &str, validate: bool) -> PyResult<()> {
        let data = self.to_str(format, validate)?;
        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(format!(
                "Failed to write file: {}",
                e
            ))),
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

    #[setter]
    fn set_metadata(&mut self, metadata: AemMetadata) {
        self.inner.metadata = metadata.inner;
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

    #[setter]
    fn set_data(&mut self, data: AemData) {
        self.inner.data = data.inner;
    }

    /// Validate the segment against CCSDS rules.
    fn validate(&self) -> PyResult<()> {
        self.inner
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
                start_time: parse_epoch(&start_time)?,
                stop_time: parse_epoch(&stop_time)?,
                useable_start_time: useable_start_time.map(|s| parse_epoch(&s)).transpose()?,
                useable_stop_time: useable_stop_time.map(|s| parse_epoch(&s)).transpose()?,
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
    fn set_ref_frame_a(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner.ref_frame_a = parse_reference_frame(&value)?;
        Ok(())
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
    fn set_ref_frame_b(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner.ref_frame_b = parse_reference_frame(&value)?;
        Ok(())
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
    fn set_time_system(&mut self, value: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner.time_system = parse_time_system(&value)?;
        Ok(())
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
        self.inner.start_time = parse_epoch(&value)?;
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
        self.inner.stop_time = parse_epoch(&value)?;
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
        self.inner.useable_start_time = value.map(|s| parse_epoch(&s)).transpose()?;
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
        self.inner.useable_stop_time = value.map(|s| parse_epoch(&s)).transpose()?;
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
#[derive(Clone)]
pub struct AemData {
    pub inner: core_aem::AemData,
}

#[pymethods]
impl AemData {
    #[new]
    #[pyo3(signature = (attitude_states, attitude_type=None, comment=None))]
    fn new(
        attitude_states: Vec<AttitudeState>,
        attitude_type: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let states = if attitude_states.is_empty() {
            Vec::new()
        } else {
            let widths: std::collections::BTreeSet<usize> =
                attitude_states.iter().map(|s| s.values.len()).collect();
            if widths.len() != 1 {
                return Err(PyValueError::new_err(
                    "All attitude states must have the same number of values",
                ));
            }
            let width = *widths.iter().next().unwrap();
            let resolved_type = parse_attitude_type_or_infer(attitude_type.as_deref(), width)?;
            let mut mapped = Vec::with_capacity(attitude_states.len());
            for state in attitude_states {
                mapped.push(build_state_from_values(
                    state.epoch,
                    &state.values,
                    &resolved_type,
                )?);
            }
            mapped
        };

        Ok(Self {
            inner: core_aem::AemData {
                comment: comment.unwrap_or_default(),
                attitude_states: states,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("AemData(states={})", self.inner.attitude_states.len())
    }

    /// Validate the data section against CCSDS rules.
    fn validate(&self, attitude_type: String) -> PyResult<()> {
        let attitude_type = AttitudeTypeType::from_str(&attitude_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner
            .validate(&attitude_type)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    #[pyo3(signature = (epochs, array, attitude_type=None, comment=None))]
    fn from_numpy(
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
            attitude_states.push(build_state_from_values(
                parse_epoch(epoch_str)?,
                &row_values,
                &resolved_type,
            )?);
        }

        Ok(Self {
            inner: core_aem::AemData {
                comment: comment.unwrap_or_default(),
                attitude_states,
            },
        })
    }

    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, comment: Vec<String>) {
        self.inner.comment = comment;
    }

    /// Attitude ephemeris data lines.
    ///
    /// :type: list[AttitudeState]
    #[getter]
    fn get_attitude_states(&self) -> PyResult<Vec<AttitudeState>> {
        self.inner
            .attitude_states
            .iter()
            .map(|s| {
                let content = s
                    .content()
                    .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
                let (epoch, values) = values_from_content(content);
                Ok(AttitudeState { epoch, values })
            })
            .collect()
    }

    #[setter]
    fn set_attitude_states(&mut self, attitude_states: Vec<AttitudeState>) -> PyResult<()> {
        if attitude_states.is_empty() {
            self.inner.attitude_states.clear();
            return Ok(());
        }

        let widths: std::collections::BTreeSet<usize> =
            attitude_states.iter().map(|s| s.values.len()).collect();
        if widths.len() != 1 {
            return Err(PyValueError::new_err(
                "All attitude states must have the same number of values",
            ));
        }

        let width = *widths.iter().next().unwrap();
        let resolved_type = if let Some(existing) = self.inner.attitude_states.first() {
            let content = existing
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let existing_type = attitude_type_from_content(&content);
            let (_, existing_values) = values_from_content(content);
            let existing_width = existing_values.len();
            if existing_width != width {
                return Err(PyValueError::new_err(format!(
                    "Expected {} values per state based on existing data, got {}",
                    existing_width, width
                )));
            }
            existing_type
        } else {
            parse_attitude_type_or_infer(None, width)?
        };

        let mut mapped = Vec::with_capacity(attitude_states.len());
        for state in attitude_states {
            mapped.push(build_state_from_values(
                state.epoch,
                &state.values,
                &resolved_type,
            )?);
        }
        self.inner.attitude_states = mapped;
        Ok(())
    }

    /// Epochs for attitude states (ISO 8601).
    ///
    /// :type: list[str]
    #[getter]
    fn get_attitude_states_epochs(&self) -> PyResult<Vec<String>> {
        let mut epochs = Vec::with_capacity(self.inner.attitude_states.len());
        for s in &self.inner.attitude_states {
            let content = s
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let epoch = match content {
                ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::EulerAngle(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::Spin(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::SpinNutation(v) => v.epoch,
                ccsds_ndm::common::AemAttitudeState::SpinNutationMom(v) => v.epoch,
            };
            epochs.push(epoch.as_str().to_string());
        }
        Ok(epochs)
    }

    #[setter]
    fn set_attitude_states_epochs(&mut self, epochs: Vec<String>) -> PyResult<()> {
        if self.inner.attitude_states.is_empty() {
            return Err(PyValueError::new_err(
                "Cannot set epochs when no attitude states exist; create states first",
            ));
        }

        if epochs.len() != self.inner.attitude_states.len() {
            return Err(PyValueError::new_err(
                "Number of epochs must match number of attitude states",
            ));
        }

        let mut updated = Vec::with_capacity(epochs.len());
        for (state, epoch_str) in self.inner.attitude_states.iter().zip(epochs.iter()) {
            let content = state
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let epoch = parse_epoch(epoch_str)?;
            let updated_state = match content {
                ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::QuaternionEphemeris(v)
                }
                ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::QuaternionDerivative(v)
                }
                ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::QuaternionAngVel(v)
                }
                ccsds_ndm::common::AemAttitudeState::EulerAngle(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::EulerAngle(v)
                }
                ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::EulerAngleDerivative(v)
                }
                ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::EulerAngleAngVel(v)
                }
                ccsds_ndm::common::AemAttitudeState::Spin(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::Spin(v)
                }
                ccsds_ndm::common::AemAttitudeState::SpinNutation(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::SpinNutation(v)
                }
                ccsds_ndm::common::AemAttitudeState::SpinNutationMom(mut v) => {
                    v.epoch = epoch;
                    ccsds_ndm::common::AemAttitudeState::SpinNutationMom(v)
                }
            };
            updated.push(updated_state.into());
        }
        self.inner.attitude_states = updated;
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
        if self.inner.attitude_states.is_empty() {
            let array = PyArray::from_vec(py, Vec::<f64>::new())
                .reshape([0, 0])
                .unwrap();
            return Ok(array.into());
        }

        let first_content = self.inner.attitude_states[0]
            .content()
            .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
        let first_type = attitude_type_from_content(&first_content);
        let (_, first_values) = values_from_content(first_content);
        let expected_cols = first_values.len();

        let mut data = Vec::with_capacity(self.inner.attitude_states.len() * expected_cols);
        data.extend(first_values);

        for s in self.inner.attitude_states.iter().skip(1) {
            let content = s
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let this_type = attitude_type_from_content(&content);
            if this_type != first_type {
                return Err(PyValueError::new_err(
                    "NumPy access requires all attitude states to be of the same ATTITUDE_TYPE",
                ));
            }
            let (_, values) = values_from_content(content);
            if values.len() != expected_cols {
                return Err(PyValueError::new_err(
                    "NumPy access requires all attitude states to have the same data width",
                ));
            }
            data.extend(values);
        }

        let array = PyArray::from_vec(py, data)
            .reshape([self.inner.attitude_states.len(), expected_cols])
            .unwrap();
        Ok(array.into())
    }

    #[setter]
    fn set_attitude_states_numpy(&mut self, array: PyReadonlyArray2<f64>) -> PyResult<()> {
        let shape = array.shape();
        if shape.len() != 2 {
            return Err(PyValueError::new_err("NumPy array must be 2-dimensional"));
        }
        if self.inner.attitude_states.is_empty() {
            return Err(PyValueError::new_err(
                "Attitude epochs are missing; set attitude_states_epochs or use from_numpy",
            ));
        }
        if self.inner.attitude_states.len() != shape[0] {
            return Err(PyValueError::new_err(
                "Number of rows must match number of attitude states",
            ));
        }

        let first_existing = self.inner.attitude_states[0]
            .content()
            .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
        let resolved_type = attitude_type_from_content(&first_existing);
        let (_, first_values) = values_from_content(first_existing);
        let expected_cols = first_values.len();
        if shape[1] != expected_cols {
            return Err(PyValueError::new_err(format!(
                "NumPy array must have {} columns for this attitude state type",
                expected_cols
            )));
        }
        let array_view = array.as_array();
        let mut attitude_states = Vec::with_capacity(shape[0]);

        for (i, existing) in self.inner.attitude_states.iter().enumerate() {
            let content = existing
                .content()
                .ok_or_else(|| PyValueError::new_err("Attitude state is missing content"))?;
            let this_type = attitude_type_from_content(&content);
            if this_type != resolved_type {
                return Err(PyValueError::new_err(
                    "NumPy assignment requires all existing attitude states to have the same ATTITUDE_TYPE",
                ));
            }
            let (epoch, values) = values_from_content(content);
            if values.len() != expected_cols {
                return Err(PyValueError::new_err(
                    "NumPy access requires all attitude states to have the same data width",
                ));
            }

            let row = array_view.row(i);
            let row_values: Vec<f64> = row.iter().copied().collect();
            attitude_states.push(build_state_from_values(epoch, &row_values, &resolved_type)?);
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
