// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
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

    #[getter]
    fn get_q1(&self) -> f64 { self.inner.quaternion.q1 }
    #[getter]
    fn get_q2(&self) -> f64 { self.inner.quaternion.q2 }
    #[getter]
    fn get_q3(&self) -> f64 { self.inner.quaternion.q3 }
    #[getter]
    fn get_qc(&self) -> f64 { self.inner.quaternion.qc }

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
}
