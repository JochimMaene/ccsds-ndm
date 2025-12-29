// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::parse_epoch;
use ccsds_ndm::common as core_common;
use ccsds_ndm::types::{Acc, Position, Velocity};
use pyo3::prelude::*;

/// Represents the header of a CCSDS Orbit Data Message (ODM).
///
/// The header contains metadata common to all ODM message types.
///
/// Parameters
/// ----------
/// creation_date : str
///     File creation date/time in UTC.
/// originator : str
///     Creating agency or operator.
/// classification : str, optional
///     User-defined free-text message classification/caveats.
/// message_id : str, optional
///     ID that uniquely identifies a message from a given originator.
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OdmHeader {
    pub inner: core_common::OdmHeader,
}

#[pymethods]
impl OdmHeader {
    #[new]
    fn new(
        creation_date: String,
        originator: String,
        classification: Option<String>,
        message_id: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::OdmHeader {
                creation_date: parse_epoch(&creation_date)?,
                originator,
                message_id,
                classification,
                comment: comment.unwrap_or_default(),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "OdmHeader(originator='{}', creation_date='{}')",
            self.inner.originator,
            self.inner.creation_date.as_str()
        )
    }

    /// File creation date/time in UTC.
    ///
    /// :type: str
    #[getter]
    fn get_creation_date(&self) -> String {
        self.inner.creation_date.as_str().to_string()
    }

    #[setter]
    fn set_creation_date(&mut self, value: String) -> PyResult<()> {
        self.inner.creation_date = parse_epoch(&value)?;
        Ok(())
    }

    /// Creating agency or operator.
    ///
    /// :type: str
    #[getter]
    fn get_originator(&self) -> String {
        self.inner.originator.clone()
    }

    #[setter]
    fn set_originator(&mut self, value: String) {
        self.inner.originator = value;
    }

    /// ID that uniquely identifies a message from a given originator.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_message_id(&self) -> Option<String> {
        self.inner.message_id.clone()
    }

    #[setter]
    fn set_message_id(&mut self, value: Option<String>) {
        self.inner.message_id = value;
    }

    /// User-defined free-text message classification/caveats.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_classification(&self) -> Option<String> {
        self.inner.classification.clone()
    }

    #[setter]
    fn set_classification(&mut self, value: Option<String>) {
        self.inner.classification = value;
    }

    // Note: Using "comments" (plural) to match existing stub
    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

/// Position and velocity (and optionally acceleration) at a specific epoch.
///
/// Used in OEM to represent object state.
/// Units: Position in km, Velocity in km/s, Acceleration in km/s² (by default)
///
/// Parameters
/// ----------
/// epoch : str
///     Epoch of the state vector.
/// x : float
///     Position vector X-component (km).
/// y : float
///     Position vector Y-component (km).
/// z : float
///     Position vector Z-component (km).
/// x_dot : float
///     Velocity vector X-component (km/s).
/// y_dot : float
///     Velocity vector Y-component (km/s).
/// z_dot : float
///     Velocity vector Z-component (km/s).
/// x_ddot : float, optional
///     Acceleration vector X-component (km/s²).
/// y_ddot : float, optional
///     Acceleration vector Y-component (km/s²).
/// z_ddot : float, optional
///     Acceleration vector Z-component (km/s²).
#[pyclass(name = "StateVectorAcc")]
#[derive(Clone)]
pub struct StateVectorAcc {
    pub inner: core_common::StateVectorAcc,
}

#[pymethods]
impl StateVectorAcc {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        epoch: String,
        x: f64,
        y: f64,
        z: f64,
        x_dot: f64,
        y_dot: f64,
        z_dot: f64,
        x_ddot: Option<f64>,
        y_ddot: Option<f64>,
        z_ddot: Option<f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::StateVectorAcc {
                epoch: parse_epoch(&epoch)?,
                x: Position {
                    value: x,
                    units: None,
                },
                y: Position {
                    value: y,
                    units: None,
                },
                z: Position {
                    value: z,
                    units: None,
                },
                x_dot: Velocity {
                    value: x_dot,
                    units: None,
                },
                y_dot: Velocity {
                    value: y_dot,
                    units: None,
                },
                z_dot: Velocity {
                    value: z_dot,
                    units: None,
                },
                x_ddot: x_ddot.map(|v| Acc {
                    value: v,
                    units: None,
                }),
                y_ddot: y_ddot.map(|v| Acc {
                    value: v,
                    units: None,
                }),
                z_ddot: z_ddot.map(|v| Acc {
                    value: v,
                    units: None,
                }),
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "StateVectorAcc(epoch='{}', pos=[{:.3}, {:.3}, {:.3}], vel=[{:.6}, {:.6}, {:.6}])",
            self.inner.epoch.as_str(),
            self.inner.x.value,
            self.inner.y.value,
            self.inner.z.value,
            self.inner.x_dot.value,
            self.inner.y_dot.value,
            self.inner.z_dot.value
        )
    }

    /// Epoch of the state vector.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch = parse_epoch(&value)?;
        Ok(())
    }

    /// Position vector X-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_x(&self) -> f64 {
        self.inner.x.value
    }

    #[setter]
    fn set_x(&mut self, value: f64) {
        self.inner.x.value = value;
    }

    /// Position vector Y-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_y(&self) -> f64 {
        self.inner.y.value
    }

    #[setter]
    fn set_y(&mut self, value: f64) {
        self.inner.y.value = value;
    }

    /// Position vector Z-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z.value
    }

    #[setter]
    fn set_z(&mut self, value: f64) {
        self.inner.z.value = value;
    }

    /// Velocity vector X-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_x_dot(&self) -> f64 {
        self.inner.x_dot.value
    }

    #[setter]
    fn set_x_dot(&mut self, value: f64) {
        self.inner.x_dot.value = value;
    }

    /// Velocity vector Y-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_y_dot(&self) -> f64 {
        self.inner.y_dot.value
    }

    #[setter]
    fn set_y_dot(&mut self, value: f64) {
        self.inner.y_dot.value = value;
    }

    /// Velocity vector Z-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_z_dot(&self) -> f64 {
        self.inner.z_dot.value
    }

    #[setter]
    fn set_z_dot(&mut self, value: f64) {
        self.inner.z_dot.value = value;
    }

    /// Acceleration vector X-component (km/s²).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_x_ddot(&self) -> Option<f64> {
        self.inner.x_ddot.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_x_ddot(&mut self, value: Option<f64>) {
        self.inner.x_ddot = value.map(|v| Acc {
            value: v,
            units: None,
        });
    }

    /// Acceleration vector Y-component (km/s²).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_y_ddot(&self) -> Option<f64> {
        self.inner.y_ddot.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_y_ddot(&mut self, value: Option<f64>) {
        self.inner.y_ddot = value.map(|v| Acc {
            value: v,
            units: None,
        });
    }

    /// Acceleration vector Z-component (km/s²).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_z_ddot(&self) -> Option<f64> {
        self.inner.z_ddot.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_z_ddot(&mut self, value: Option<f64>) {
        self.inner.z_ddot = value.map(|v| Acc {
            value: v,
            units: None,
        });
    }
}

/// Position and velocity at a specific epoch (without acceleration).
///
/// Simplified version of StateVectorAcc used in OPM and other messages.
///
/// Parameters
/// ----------
/// epoch : str
///     Epoch of the state vector.
/// x : float
///     Position vector X-component (km).
/// y : float
///     Position vector Y-component (km).
/// z : float
///     Position vector Z-component (km).
/// x_dot : float
///     Velocity vector X-component (km/s).
/// y_dot : float
///     Velocity vector Y-component (km/s).
/// z_dot : float
///     Velocity vector Z-component (km/s).
#[pyclass]
#[derive(Clone)]
pub struct StateVector {
    pub inner: core_common::StateVector,
}

#[pymethods]
impl StateVector {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        epoch: String,
        x: f64,
        y: f64,
        z: f64,
        x_dot: f64,
        y_dot: f64,
        z_dot: f64,
        comments: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::StateVector {
                comment: comments.unwrap_or_default(),
                epoch: parse_epoch(&epoch)?,
                x: Position {
                    value: x,
                    units: None,
                },
                y: Position {
                    value: y,
                    units: None,
                },
                z: Position {
                    value: z,
                    units: None,
                },
                x_dot: Velocity {
                    value: x_dot,
                    units: None,
                },
                y_dot: Velocity {
                    value: y_dot,
                    units: None,
                },
                z_dot: Velocity {
                    value: z_dot,
                    units: None,
                },
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "StateVector(epoch='{}', pos=[{:.3}, {:.3}, {:.3}], vel=[{:.6}, {:.6}, {:.6}])",
            self.inner.epoch.as_str(),
            self.inner.x.value,
            self.inner.y.value,
            self.inner.z.value,
            self.inner.x_dot.value,
            self.inner.y_dot.value,
            self.inner.z_dot.value
        )
    }

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Epoch of the state vector.
    ///
    /// :type: str
    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }

    #[setter]
    fn set_epoch(&mut self, value: String) -> PyResult<()> {
        self.inner.epoch = parse_epoch(&value)?;
        Ok(())
    }

    /// Position vector X-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_x(&self) -> f64 {
        self.inner.x.value
    }

    #[setter]
    fn set_x(&mut self, value: f64) {
        self.inner.x.value = value;
    }

    /// Position vector Y-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_y(&self) -> f64 {
        self.inner.y.value
    }

    #[setter]
    fn set_y(&mut self, value: f64) {
        self.inner.y.value = value;
    }

    /// Position vector Z-component (km).
    ///
    /// :type: float
    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z.value
    }

    #[setter]
    fn set_z(&mut self, value: f64) {
        self.inner.z.value = value;
    }

    /// Velocity vector X-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_x_dot(&self) -> f64 {
        self.inner.x_dot.value
    }

    #[setter]
    fn set_x_dot(&mut self, value: f64) {
        self.inner.x_dot.value = value;
    }

    /// Velocity vector Y-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_y_dot(&self) -> f64 {
        self.inner.y_dot.value
    }

    #[setter]
    fn set_y_dot(&mut self, value: f64) {
        self.inner.y_dot.value = value;
    }

    /// Velocity vector Z-component (km/s).
    ///
    /// :type: float
    #[getter]
    fn get_z_dot(&self) -> f64 {
        self.inner.z_dot.value
    }

    #[setter]
    fn set_z_dot(&mut self, value: f64) {
        self.inner.z_dot.value = value;
    }
}

/// Spacecraft parameters.
///
/// Used in OPM and OMM.
///
/// Parameters
/// ----------
/// mass : float, optional
///     Spacecraft mass (kg).
/// solar_rad_area : float, optional
///     Solar radiation pressure area (m²).
/// solar_rad_coeff : float, optional
///     Solar radiation pressure coefficient.
/// drag_area : float, optional
///     Drag area (m²).
/// drag_coeff : float, optional
///     Drag coefficient.
#[pyclass]
#[derive(Clone)]
pub struct SpacecraftParameters {
    pub inner: core_common::SpacecraftParameters,
}

#[pymethods]
impl SpacecraftParameters {
    #[new]
    fn new(
        mass: Option<f64>,
        solar_rad_area: Option<f64>,
        solar_rad_coeff: Option<f64>,
        drag_area: Option<f64>,
        drag_coeff: Option<f64>,
    ) -> Self {
        use ccsds_ndm::types::{Area, Mass};
        Self {
            inner: core_common::SpacecraftParameters {
                comment: vec![],
                mass: mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                solar_rad_area: solar_rad_area.map(|v| Area {
                    value: v,
                    units: None,
                }),
                solar_rad_coeff,
                drag_area: drag_area.map(|v| Area {
                    value: v,
                    units: None,
                }),
                drag_coeff,
            },
        }
    }

    fn __repr__(&self) -> String {
        "SpacecraftParameters(...)".to_string()
    }

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn get_comments(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comments(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Spacecraft mass.
    ///
    /// Units: kg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_mass(&self) -> Option<f64> {
        self.inner.mass.as_ref().map(|m| m.value)
    }

    #[setter]
    fn set_mass(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Mass;
        self.inner.mass = value.map(|v| Mass {
            value: v,
            units: None,
        });
    }

    /// Solar radiation pressure area.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_solar_rad_area(&self) -> Option<f64> {
        self.inner.solar_rad_area.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_solar_rad_area(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.solar_rad_area = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    /// Solar radiation pressure coefficient.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_solar_rad_coeff(&self) -> Option<f64> {
        self.inner.solar_rad_coeff
    }

    #[setter]
    fn set_solar_rad_coeff(&mut self, value: Option<f64>) {
        self.inner.solar_rad_coeff = value;
    }

    /// Drag area.
    ///
    /// Units: m²
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_area(&self) -> Option<f64> {
        self.inner.drag_area.as_ref().map(|a| a.value)
    }

    #[setter]
    fn set_drag_area(&mut self, value: Option<f64>) {
        use ccsds_ndm::types::Area;
        self.inner.drag_area = value.map(|v| Area {
            value: v,
            units: None,
        });
    }

    /// Drag coefficient.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_coeff(&self) -> Option<f64> {
        self.inner.drag_coeff
    }

    #[setter]
    fn set_drag_coeff(&mut self, value: Option<f64>) {
        self.inner.drag_coeff = value;
    }
}
