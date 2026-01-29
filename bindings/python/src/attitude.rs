// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::common as core_common;
use ccsds_ndm::types::{Angle, AngleRate, Moment, Duration};
use pyo3::prelude::*;

/// Attitude quaternion.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct QuaternionState {
    pub inner: core_common::QuaternionState,
}

#[pymethods]
impl QuaternionState {
    #[new]
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        q1: f64,
        q2: f64,
        q3: f64,
        qc: f64,
        q1_dot: Option<f64>,
        q2_dot: Option<f64>,
        q3_dot: Option<f64>,
        qc_dot: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> Self {
        use ccsds_ndm::common::{Quaternion, QuaternionDot};
        use ccsds_ndm::types::QuaternionDotComponent;
        Self {
            inner: core_common::QuaternionState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                quaternion: Quaternion { q1, q2, q3, qc },
                quaternion_dot: q1_dot.and_then(|q1d| {
                    Some(QuaternionDot {
                        q1_dot: QuaternionDotComponent { value: q1d, units: None },
                        q2_dot: QuaternionDotComponent { value: q2_dot?, units: None },
                        q3_dot: QuaternionDotComponent { value: q3_dot?, units: None },
                        qc_dot: QuaternionDotComponent { value: qc_dot?, units: None },
                    })
                }),
            },
        }
    }

    /// Quaternion components Q1, Q2, Q3, QC.
    ///
    /// Units: dimensionless
    ///
    /// :type: float
    #[getter]
    fn get_q1(&self) -> f64 { self.inner.quaternion.q1 }

    #[setter]
    fn set_q1(&mut self, value: f64) { self.inner.quaternion.q1 = value; }

    /// Quaternion components Q1, Q2, Q3, QC.
    ///
    /// Units: dimensionless
    ///
    /// :type: float
    #[getter]
    fn get_q2(&self) -> f64 { self.inner.quaternion.q2 }

    #[setter]
    fn set_q2(&mut self, value: f64) { self.inner.quaternion.q2 = value; }

    /// Quaternion components Q1, Q2, Q3, QC.
    ///
    /// Units: dimensionless
    ///
    /// :type: float
    #[getter]
    fn get_q3(&self) -> f64 { self.inner.quaternion.q3 }

    #[setter]
    fn set_q3(&mut self, value: f64) { self.inner.quaternion.q3 = value; }

    /// Quaternion components Q1, Q2, Q3, QC.
    ///
    /// Units: dimensionless
    ///
    /// :type: float
    #[getter]
    fn get_qc(&self) -> f64 { self.inner.quaternion.qc }

    #[setter]
    fn set_qc(&mut self, value: f64) { self.inner.quaternion.qc = value; }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String { self.inner.ref_frame_a.clone() }

    #[setter]
    fn set_ref_frame_a(&mut self, value: String) { self.inner.ref_frame_a = value; }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String { self.inner.ref_frame_b.clone() }

    #[setter]
    fn set_ref_frame_b(&mut self, value: String) { self.inner.ref_frame_b = value; }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> { self.inner.comment.clone() }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) { self.inner.comment = value; }
}

/// Euler angle elements.
///
/// All mandatory elements of the logical block are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct EulerAngleState {
    pub inner: core_common::EulerAngleState,
}

#[pymethods]
impl EulerAngleState {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        euler_rot_seq: String,
        angle_1: f64,
        angle_2: f64,
        angle_3: f64,
        angle_1_dot: Option<f64>,
        angle_2_dot: Option<f64>,
        angle_3_dot: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use std::str::FromStr;
        Ok(Self {
            inner: core_common::EulerAngleState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                euler_rot_seq: ccsds_ndm::types::RotSeq::from_str(&euler_rot_seq)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                angle_1: Angle::new(angle_1, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                angle_2: Angle::new(angle_2, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                angle_3: Angle::new(angle_3, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                angle_1_dot: angle_1_dot.map(|v| AngleRate { value: v, units: None }),
                angle_2_dot: angle_2_dot.map(|v| AngleRate { value: v, units: None }),
                angle_3_dot: angle_3_dot.map(|v| AngleRate { value: v, units: None }),
            },
        })
    }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String { self.inner.ref_frame_a.clone() }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String { self.inner.ref_frame_b.clone() }

    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation.
    ///
    /// :type: str
    #[getter]
    fn get_euler_rot_seq(&self) -> String { self.inner.euler_rot_seq.to_string() }

    /// Angle of the first rotation.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_angle_1(&self) -> f64 { self.inner.angle_1.value }

    /// Angle of the second rotation.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_angle_2(&self) -> f64 { self.inner.angle_2.value }

    /// Angle of the third rotation.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_angle_3(&self) -> f64 { self.inner.angle_3.value }

    /// Time derivative of angle of the first rotation.
    ///
    /// Units: deg/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_angle_1_dot(&self) -> Option<f64> { self.inner.angle_1_dot.as_ref().map(|v| v.value) }

    /// Time derivative of angle of the second rotation.
    ///
    /// Units: deg/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_angle_2_dot(&self) -> Option<f64> { self.inner.angle_2_dot.as_ref().map(|v| v.value) }

    /// Time derivative of angle of the third rotation.
    ///
    /// Units: deg/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_angle_3_dot(&self) -> Option<f64> { self.inner.angle_3_dot.as_ref().map(|v| v.value) }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> { self.inner.comment.clone() }
}

/// Angular velocity vector.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct AngVelState {
    pub inner: core_common::AngVelState,
}

#[pymethods]
impl AngVelState {
    #[new]
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        angvel_frame: String,
        angvel_x: f64,
        angvel_y: f64,
        angvel_z: f64,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_common::AngVelState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                angvel_frame: ccsds_ndm::types::AngVelFrameType(angvel_frame),
                angvel_x: AngleRate { value: angvel_x, units: None },
                angvel_y: AngleRate { value: angvel_y, units: None },
                angvel_z: AngleRate { value: angvel_z, units: None },
            },
        }
    }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String { self.inner.ref_frame_a.clone() }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String { self.inner.ref_frame_b.clone() }

    /// Reference frame in which the components of the angular velocity vector are given. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_angvel_frame(&self) -> String { self.inner.angvel_frame.0.clone() }

    /// Component of the angular velocity vector on the X axis.
    ///
    /// Units: deg/s
    ///
    /// :type: float
    #[getter]
    fn get_angvel_x(&self) -> f64 { self.inner.angvel_x.value }

    /// Component of the angular velocity vector on the Y axis.
    ///
    /// Units: deg/s
    ///
    /// :type: float
    #[getter]
    fn get_angvel_y(&self) -> f64 { self.inner.angvel_y.value }

    /// Component of the angular velocity vector on the Z axis.
    ///
    /// Units: deg/s
    ///
    /// :type: float
    #[getter]
    fn get_angvel_z(&self) -> f64 { self.inner.angvel_z.value }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> { self.inner.comment.clone() }
}

/// Spin block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct SpinState {
    pub inner: core_common::SpinState,
}

#[pymethods]
impl SpinState {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ref_frame_a: String,
        ref_frame_b: String,
        spin_alpha: f64,
        spin_delta: f64,
        spin_angle: f64,
        spin_angle_vel: f64,
        nutation: Option<f64>,
        nutation_per: Option<f64>,
        nutation_phase: Option<f64>,
        momentum_alpha: Option<f64>,
        momentum_delta: Option<f64>,
        nutation_vel: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::SpinState {
                comment: comment.unwrap_or_default(),
                ref_frame_a,
                ref_frame_b,
                spin_alpha: Angle::new(spin_alpha, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                spin_delta: Angle::new(spin_delta, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                spin_angle: Angle::new(spin_angle, None).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                spin_angle_vel: AngleRate { value: spin_angle_vel, units: None },
                nutation: nutation.map(|v| Angle::new(v, None)).transpose().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                nutation_per: nutation_per.map(|v| Duration { value: v, units: None }),
                nutation_phase: nutation_phase.map(|v| Angle::new(v, None)).transpose().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                momentum_alpha: momentum_alpha.map(|v| Angle::new(v, None)).transpose().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                momentum_delta: momentum_delta.map(|v| Angle::new(v, None)).transpose().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
                nutation_vel: nutation_vel.map(|v| AngleRate { value: v, units: None }),
            },
        })
    }

    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_a(&self) -> String { self.inner.ref_frame_a.clone() }

    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_ref_frame_b(&self) -> String { self.inner.ref_frame_b.clone() }

    /// Right ascension of spin axis vector in frame A.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_spin_alpha(&self) -> f64 { self.inner.spin_alpha.value }

    /// Declination of the spin axis vector in frame A.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_spin_delta(&self) -> f64 { self.inner.spin_delta.value }

    /// Phase of the satellite about the spin axis.
    ///
    /// Units: deg
    ///
    /// :type: float
    #[getter]
    fn get_spin_angle(&self) -> f64 { self.inner.spin_angle.value }

    /// Angular velocity of satellite around spin axis.
    ///
    /// Units: deg/s
    ///
    /// :type: float
    #[getter]
    fn get_spin_angle_vel(&self) -> f64 { self.inner.spin_angle_vel.value }

    /// Nutation angle of spin axis.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nutation(&self) -> Option<f64> { self.inner.nutation.as_ref().map(|v| v.value) }

    /// Body nutation period of the spin axis.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nutation_per(&self) -> Option<f64> { self.inner.nutation_per.as_ref().map(|v| v.value) }

    /// Inertial nutation phase.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nutation_phase(&self) -> Option<f64> { self.inner.nutation_phase.as_ref().map(|v| v.value) }

    /// Right ascension of angular momentum vector in frame A.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_momentum_alpha(&self) -> Option<f64> { self.inner.momentum_alpha.as_ref().map(|v| v.value) }

    /// Declination of angular momentum vector in frame A.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_momentum_delta(&self) -> Option<f64> { self.inner.momentum_delta.as_ref().map(|v| v.value) }

    /// Angular velocity of spin vector around the angular momentum vector.
    ///
    /// Units: deg/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nutation_vel(&self) -> Option<f64> { self.inner.nutation_vel.as_ref().map(|v| v.value) }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> { self.inner.comment.clone() }
}

/// Inertia block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[pyclass]
#[derive(Clone)]
pub struct InertiaState {
    pub inner: core_common::InertiaState,
}

#[pymethods]
impl InertiaState {
    #[new]
    fn new(
        inertia_ref_frame: String,
        ixx: f64,
        iyy: f64,
        izz: f64,
        ixy: f64,
        ixz: f64,
        iyz: f64,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_common::InertiaState {
                comment: comment.unwrap_or_default(),
                inertia_ref_frame,
                ixx: Moment { value: ixx, units: None },
                iyy: Moment { value: iyy, units: None },
                izz: Moment { value: izz, units: None },
                ixy: Moment { value: ixy, units: None },
                ixz: Moment { value: ixz, units: None },
                iyz: Moment { value: iyz, units: None },
            },
        }
    }

    /// Coordinate system for the inertia tensor. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// :type: str
    #[getter]
    fn get_inertia_ref_frame(&self) -> String { self.inner.inertia_ref_frame.clone() }

    /// Moment of Inertia about the X-axis.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_ixx(&self) -> f64 { self.inner.ixx.value }

    /// Moment of Inertia about the Y-axis.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_iyy(&self) -> f64 { self.inner.iyy.value }

    /// Moment of Inertia about the Z-axis.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_izz(&self) -> f64 { self.inner.izz.value }

    /// Inertia Cross Product of the X and Y axes.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_ixy(&self) -> f64 { self.inner.ixy.value }

    /// Inertia Cross Product of the X and Z axes.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_ixz(&self) -> f64 { self.inner.ixz.value }

    /// Inertia Cross Product of the Y and Z axes.
    ///
    /// Units: kg*m²
    ///
    /// :type: float
    #[getter]
    fn get_iyz(&self) -> f64 { self.inner.iyz.value }

    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> { self.inner.comment.clone() }
}
