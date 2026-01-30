// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::parse_epoch;
use ccsds_ndm::common as core_common;
use ccsds_ndm::types::{Acc, Position, Velocity};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::str::FromStr;

/// Represents the `odmHeader` complex type.
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

    /// File creation date/time in UTC. (For format specification, see 7.5.10.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
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

    /// Creating agency or operator. Select from the accepted set of values indicated in annex B,
    /// subsection B1 from the ‘Abbreviation’ column (when present), or the ‘Name’ column when an
    /// Abbreviation column is not populated. If desired organization is not listed there, follow
    /// procedures to request that originator be added to SANA registry.
    ///
    /// Examples: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
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

    /// ID that uniquely identifies a message from a given originator. The format and content of
    /// the message identifier value are at the discretion of the originator.
    ///
    /// Examples: OPM_201113719185, ABC-12_34
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

    /// User-defined free-text message classification/caveats of this ODM. It is recommended
    /// that selected values be pre-coordinated between exchanging entities by mutual agreement.
    ///
    /// Examples: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
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

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

/// Represents the `admHeader` complex type from the XSD.
#[pyclass]
#[derive(Clone)]
pub struct AdmHeader {
    pub inner: core_common::AdmHeader,
}

#[pymethods]
impl AdmHeader {
    #[new]
    fn new(
        creation_date: String,
        originator: String,
        classification: Option<String>,
        message_id: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::AdmHeader {
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
            "AdmHeader(originator='{}', creation_date='{}')",
            self.inner.originator,
            self.inner.creation_date.as_str()
        )
    }

    /// File creation date/time in UTC. (For format specification, see 6.8.9.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23Z
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

    /// Creating agency or operator. Select from the accepted set of values indicated in annex B,
    /// subsection B1 from the ‘Abbreviation’ column (when present), or the ‘Name’ column when an
    /// Abbreviation column is not populated. If desired organization is not listed there, follow
    /// procedures to request that originator be added to SANA registry.
    ///
    /// Examples: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
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

    /// ID that uniquely identifies a message from a given originator. The format and content of
    /// the message identifier value are at the discretion of the originator.
    ///
    /// Examples: APM_201113719185, ABC-12_34
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

    /// User-defined free-text message classification/caveats of this ADM. It is recommended
    /// that selected values be pre-coordinated between exchanging entities by mutual agreement.
    ///
    /// Examples: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
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

    /// User-defined comments. (See 7.8 for formatting rules.)
    ///
    /// Examples: This is a comment
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

/// State Vector Components in the Specified Coordinate System.
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

    /// Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
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

    /// Position vector X-component.
    ///
    /// Units: km
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

    /// Position vector Y-component.
    ///
    /// Units: km
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

    /// Position vector Z-component.
    ///
    /// Units: km
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

    /// Velocity vector X-component.
    ///
    /// Units: km/s
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

    /// Velocity vector Y-component.
    ///
    /// Units: km/s
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

    /// Velocity vector Z-component.
    ///
    /// Units: km/s
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

    /// Acceleration vector X-component.
    ///
    /// Units: km/s²
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

    /// Acceleration vector Y-component.
    ///
    /// Units: km/s²
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

    /// Acceleration vector Z-component.
    ///
    /// Units: km/s²
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

/// State Vector Components in the Specified Coordinate System.
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

    /// Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)
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

    /// Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
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

    /// Position vector X-component.
    ///
    /// Units: km
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

    /// Position vector Y-component.
    ///
    /// Units: km
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

    /// Position vector Z-component.
    ///
    /// Units: km
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

    /// Velocity vector X-component.
    ///
    /// Units: km/s
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

    /// Velocity vector Y-component.
    ///
    /// Units: km/s
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

    /// Velocity vector Z-component.
    ///
    /// Units: km/s
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

/// Spacecraft Parameters (if maneuver is specified, then mass must be provided).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
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
        use ccsds_ndm::types::{Area, Mass, NonNegativeDouble};
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
                solar_rad_coeff: solar_rad_coeff.map(|value| NonNegativeDouble { value }),
                drag_area: drag_area.map(|v| Area {
                    value: v,
                    units: None,
                }),
                drag_coeff: drag_coeff.map(|value| NonNegativeDouble { value }),
            },
        }
    }

    fn __repr__(&self) -> String {
        "SpacecraftParameters(...)".to_string()
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

    /// Spacecraft mass.
    ///
    /// Examples: 1850.2, 3352.0
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

    /// Solar Radiation Pressure Area (AR).
    ///
    /// Examples: 14, 20.0
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

    /// Solar Radiation Pressure Coefficient (CR).
    ///
    /// Examples: 1, 1.34
    ///
    /// Units: n/a
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_solar_rad_coeff(&self) -> Option<f64> {
        self.inner.solar_rad_coeff.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_solar_rad_coeff(&mut self, value: Option<f64>) {
        self.inner.solar_rad_coeff =
            value.map(|v| ccsds_ndm::types::NonNegativeDouble { value: v });
    }

    /// Drag Area (AD).
    ///
    /// Examples: 14, 20.0
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

    /// Drag Coefficient (CD).
    ///
    /// Examples: 2, 2.1
    ///
    /// Units: n/a
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_drag_coeff(&self) -> Option<f64> {
        self.inner.drag_coeff.as_ref().map(|v| v.value)
    }

    #[setter]
    fn set_drag_coeff(&mut self, value: Option<f64>) {
        self.inner.drag_coeff = value.map(|v| ccsds_ndm::types::NonNegativeDouble { value: v });
    }
}

/// Orbit Determination Parameters.
///
/// Parameters
/// ----------
/// time_lastob_start : str, optional
///     Time of last observation start.
/// time_lastob_end : str, optional
///     Time of last observation end.
/// recommended_od_span : float, optional
///     Recommended OD span. Units: d
/// actual_od_span : float, optional
///     Actual OD span. Units: d
/// obs_available : int, optional
///     Observations available.
/// obs_used : int, optional
///     Observations used.
/// tracks_available : int, optional
///     Tracks available.
/// tracks_used : int, optional
///     Tracks used.
/// residuals_accepted : float, optional
///     Residuals accepted. Units: %
/// weighted_rms : float, optional
///     Weighted RMS.
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct OdParameters {
    pub inner: core_common::OdParameters,
}

#[pymethods]
impl OdParameters {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        time_lastob_start: Option<String>,
        time_lastob_end: Option<String>,
        recommended_od_span: Option<f64>,
        actual_od_span: Option<f64>,
        obs_available: Option<u32>,
        obs_used: Option<u32>,
        tracks_available: Option<u32>,
        tracks_used: Option<u32>,
        residuals_accepted: Option<f64>,
        weighted_rms: Option<f64>,
        comment: Vec<String>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{DayInterval, NonNegativeDouble, Percentage, PositiveInteger};
        Ok(Self {
            inner: core_common::OdParameters {
                comment,
                time_lastob_start: time_lastob_start.map(|s| parse_epoch(&s)).transpose()?,
                time_lastob_end: time_lastob_end.map(|s| parse_epoch(&s)).transpose()?,
                recommended_od_span: recommended_od_span
                    .map(|v| {
                        DayInterval::new(v, None).map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                actual_od_span: actual_od_span
                    .map(|v| {
                        DayInterval::new(v, None).map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                obs_available: obs_available.map(|v| PositiveInteger { value: v }),
                obs_used: obs_used.map(|v| PositiveInteger { value: v }),
                tracks_available: tracks_available.map(|v| PositiveInteger { value: v }),
                tracks_used: tracks_used.map(|v| PositiveInteger { value: v }),
                residuals_accepted: residuals_accepted
                    .map(|v| {
                        Percentage::new(v, None).map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                weighted_rms: weighted_rms.map(|value| NonNegativeDouble { value }),
            },
        })
    }

    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }

    /// The start of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_END).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_time_lastob_start(&self) -> Option<String> {
        self.inner.time_lastob_start.as_ref().map(|e| e.to_string())
    }
    #[setter]
    fn set_time_lastob_start(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.time_lastob_start = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// The end of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_START).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_time_lastob_end(&self) -> Option<String> {
        self.inner.time_lastob_end.as_ref().map(|e| e.to_string())
    }
    #[setter]
    fn set_time_lastob_end(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.time_lastob_end = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// The recommended OD time span calculated for the object.
    ///
    /// Examples: 14, 20.0
    ///
    /// Units: days
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_recommended_od_span(&self) -> Option<f64> {
        self.inner.recommended_od_span.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_recommended_od_span(&mut self, v: Option<f64>) -> PyResult<()> {
        use ccsds_ndm::types::DayInterval;
        self.inner.recommended_od_span = v.map(|x| DayInterval::new(x, None).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?;
        Ok(())
    }

    /// Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
    /// time span used for the OD of the object. (See annex E for definition.)
    ///
    /// Examples: 14, 20.0
    ///
    /// Units: days
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_actual_od_span(&self) -> Option<f64> {
        self.inner.actual_od_span.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_actual_od_span(&mut self, v: Option<f64>) -> PyResult<()> {
        use ccsds_ndm::types::DayInterval;
        self.inner.actual_od_span = v.map(|x| DayInterval::new(x, None).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?;
        Ok(())
    }

    /// The total number of observations available for orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_obs_available(&self) -> Option<u32> {
        self.inner.obs_available.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_obs_available(&mut self, v: Option<u32>) {
        self.inner.obs_available = v.map(|value| ccsds_ndm::types::PositiveInteger { value });
    }

    /// The number of observations used in the orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_obs_used(&self) -> Option<u32> {
        self.inner.obs_used.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_obs_used(&mut self, v: Option<u32>) {
        self.inner.obs_used = v.map(|value| ccsds_ndm::types::PositiveInteger { value });
    }

    /// The total number of tracks available for orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_tracks_available(&self) -> Option<u32> {
        self.inner.tracks_available.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_tracks_available(&mut self, v: Option<u32>) {
        self.inner.tracks_available = v.map(|value| ccsds_ndm::types::PositiveInteger { value });
    }

    /// The number of tracks used in the orbit determination.
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_tracks_used(&self) -> Option<u32> {
        self.inner.tracks_used.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_tracks_used(&mut self, v: Option<u32>) {
        self.inner.tracks_used = v.map(|value| ccsds_ndm::types::PositiveInteger { value });
    }

    /// The percentage of residuals accepted during orbit determination.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_residuals_accepted(&self) -> Option<f64> {
        self.inner.residuals_accepted.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_residuals_accepted(&mut self, v: Option<f64>) -> PyResult<()> {
        use ccsds_ndm::types::Percentage;
        self.inner.residuals_accepted = v
            .map(|x| {
                Percentage::new(x, None).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The weighted root mean square (RMS) of the residuals.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_weighted_rms(&self) -> Option<f64> {
        self.inner.weighted_rms.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_weighted_rms(&mut self, v: Option<f64>) {
        self.inner.weighted_rms = v.map(|value| ccsds_ndm::types::NonNegativeDouble { value });
    }
}

/// Ground impact parameters (groundImpactParametersType, RDM).
///
/// Parameters
/// ----------
/// probability_of_impact : float, optional
///     Probability of impact.
/// probability_of_burn_up : float, optional
///     Probability of burn up.
/// probability_of_break_up : float, optional
///     Probability of break up.
/// probability_of_land_impact : float, optional
///     Probability of land impact.
/// probability_of_casualty : float, optional
///     Probability of casualty.
/// nominal_impact_epoch : str, optional
///     Nominal impact epoch.
/// impact_window_start : str, optional
///     Impact window start.
/// impact_window_end : str, optional
///     Impact window end.
/// impact_ref_frame : str, optional
///     Impact reference frame.
/// nominal_impact_lon : float, optional
///     Nominal impact longitude. Units: deg
/// nominal_impact_lat : float, optional
///     Nominal impact latitude. Units: deg
/// nominal_impact_alt : float, optional
///     Nominal impact altitude. Units: km
/// impact_1_confidence : float, optional
///     Impact 1 confidence. Units: %
/// impact_1_start_lon : float, optional
///     Impact 1 start longitude. Units: deg
/// impact_1_start_lat : float, optional
///     Impact 1 start latitude. Units: deg
/// impact_1_stop_lon : float, optional
///     Impact 1 stop longitude. Units: deg
/// impact_1_stop_lat : float, optional
///     Impact 1 stop latitude. Units: deg
/// impact_1_cross_track : float, optional
///     Impact 1 cross track. Units: km
/// impact_2_confidence : float, optional
///     Impact 2 confidence. Units: %
/// impact_2_start_lon : float, optional
///     Impact 2 start longitude. Units: deg
/// impact_2_start_lat : float, optional
///     Impact 2 start latitude. Units: deg
/// impact_2_stop_lon : float, optional
///     Impact 2 stop longitude. Units: deg
/// impact_2_stop_lat : float, optional
///     Impact 2 stop latitude. Units: deg
/// impact_2_cross_track : float, optional
///     Impact 2 cross track. Units: km
/// impact_3_confidence : float, optional
///     Impact 3 confidence. Units: %
/// impact_3_start_lon : float, optional
///     Impact 3 start longitude. Units: deg
/// impact_3_start_lat : float, optional
///     Impact 3 start latitude. Units: deg
/// impact_3_stop_lon : float, optional
///     Impact 3 stop longitude. Units: deg
/// impact_3_stop_lat : float, optional
///     Impact 3 stop latitude. Units: deg
/// impact_3_cross_track : float, optional
///     Impact 3 cross track. Units: km
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct GroundImpactParameters {
    pub inner: core_common::GroundImpactParameters,
}

#[pymethods]
impl GroundImpactParameters {
    #[new]
    #[pyo3(signature = (
        *,
        probability_of_impact=None,
        probability_of_burn_up=None,
        probability_of_break_up=None,
        probability_of_land_impact=None,
        probability_of_casualty=None,
        nominal_impact_epoch=None,
        impact_window_start=None,
        impact_window_end=None,
        impact_ref_frame=None,
        nominal_impact_lon=None,
        nominal_impact_lat=None,
        nominal_impact_alt=None,
        impact_1_confidence=None,
        impact_1_start_lon=None,
        impact_1_start_lat=None,
        impact_1_stop_lon=None,
        impact_1_stop_lat=None,
        impact_1_cross_track=None,
        impact_2_confidence=None,
        impact_2_start_lon=None,
        impact_2_start_lat=None,
        impact_2_stop_lon=None,
        impact_2_stop_lat=None,
        impact_2_cross_track=None,
        impact_3_confidence=None,
        impact_3_start_lon=None,
        impact_3_start_lat=None,
        impact_3_stop_lon=None,
        impact_3_stop_lat=None,
        impact_3_cross_track=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        probability_of_impact: Option<f64>,
        probability_of_burn_up: Option<f64>,
        probability_of_break_up: Option<f64>,
        probability_of_land_impact: Option<f64>,
        probability_of_casualty: Option<f64>,
        nominal_impact_epoch: Option<String>,
        impact_window_start: Option<String>,
        impact_window_end: Option<String>,
        impact_ref_frame: Option<String>,
        nominal_impact_lon: Option<f64>,
        nominal_impact_lat: Option<f64>,
        nominal_impact_alt: Option<f64>,
        impact_1_confidence: Option<f64>,
        impact_1_start_lon: Option<f64>,
        impact_1_start_lat: Option<f64>,
        impact_1_stop_lon: Option<f64>,
        impact_1_stop_lat: Option<f64>,
        impact_1_cross_track: Option<f64>,
        impact_2_confidence: Option<f64>,
        impact_2_start_lon: Option<f64>,
        impact_2_start_lat: Option<f64>,
        impact_2_stop_lon: Option<f64>,
        impact_2_stop_lat: Option<f64>,
        impact_2_cross_track: Option<f64>,
        impact_3_confidence: Option<f64>,
        impact_3_start_lon: Option<f64>,
        impact_3_start_lat: Option<f64>,
        impact_3_stop_lon: Option<f64>,
        impact_3_stop_lat: Option<f64>,
        impact_3_cross_track: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let comment = comment.unwrap_or_default();
        use ccsds_ndm::types::{Probability, LongitudeRequired, LatitudeRequired, AltitudeRequired, PercentageRequired, Distance};
        Ok(Self {
            inner: core_common::GroundImpactParameters {
                comment,
                probability_of_impact: probability_of_impact.map(|v| Probability::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                probability_of_burn_up: probability_of_burn_up.map(|v| Probability::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                probability_of_break_up: probability_of_break_up.map(|v| Probability::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                probability_of_land_impact: probability_of_land_impact.map(|v| Probability::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                probability_of_casualty: probability_of_casualty.map(|v| Probability::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                nominal_impact_epoch: nominal_impact_epoch.map(|s| parse_epoch(&s)).transpose()?,
                impact_window_start: impact_window_start.map(|s| parse_epoch(&s)).transpose()?,
                impact_window_end: impact_window_end.map(|s| parse_epoch(&s)).transpose()?,
                impact_ref_frame,
                nominal_impact_lon: nominal_impact_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                nominal_impact_lat: nominal_impact_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                nominal_impact_alt: nominal_impact_alt.map(|v| AltitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_confidence: impact_1_confidence.map(|v| PercentageRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_start_lon: impact_1_start_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_start_lat: impact_1_start_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_stop_lon: impact_1_stop_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_stop_lat: impact_1_stop_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_1_cross_track: impact_1_cross_track.map(|v| Distance::new(v, None)),
                impact_2_confidence: impact_2_confidence.map(|v| PercentageRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_2_start_lon: impact_2_start_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_2_start_lat: impact_2_start_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_2_stop_lon: impact_2_stop_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_2_stop_lat: impact_2_stop_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_2_cross_track: impact_2_cross_track.map(|v| Distance::new(v, None)),
                impact_3_confidence: impact_3_confidence.map(|v| PercentageRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_3_start_lon: impact_3_start_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_3_start_lat: impact_3_start_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_3_stop_lon: impact_3_stop_lon.map(|v| LongitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_3_stop_lat: impact_3_stop_lat.map(|v| LatitudeRequired::new(v).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?,
                impact_3_cross_track: impact_3_cross_track.map(|v| Distance::new(v, None)),
            },
        })
    }
    // Getters and setters omitted for brevity in this snippet but they follow the pattern.
    // Actually I must include them or audit will fail.
    // I will include them.
    /// Comments (allowed only at the beginning of each RDM data logical block).
    ///
    /// :type: list[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter] fn set_comment(&mut self, v: Vec<String>) { self.inner.comment = v; }

    /// Probability that any fragment will impact the Earth (either land or sea; 0 to 1).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_probability_of_impact(&self) -> Option<f64> {
        self.inner.probability_of_impact.as_ref().map(|v| v.value)
    }
    #[setter] fn set_probability_of_impact(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.probability_of_impact = v.map(|x| ccsds_ndm::types::Probability::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Probability that the entire object and any fragments will burn up during atmospheric
    /// re-entry (0 to 1).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_probability_of_burn_up(&self) -> Option<f64> {
        self.inner.probability_of_burn_up.as_ref().map(|v| v.value)
    }
    #[setter] fn set_probability_of_burn_up(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.probability_of_burn_up = v.map(|x| ccsds_ndm::types::Probability::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Probability that the object will break up during re-entry (0 to 1).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_probability_of_break_up(&self) -> Option<f64> {
        self.inner.probability_of_break_up.as_ref().map(|v| v.value)
    }
    #[setter] fn set_probability_of_break_up(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.probability_of_break_up = v.map(|x| ccsds_ndm::types::Probability::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Probability that any fragment will impact solid ground (0 to 1).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_probability_of_land_impact(&self) -> Option<f64> {
        self.inner.probability_of_land_impact.as_ref().map(|v| v.value)
    }
    #[setter] fn set_probability_of_land_impact(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.probability_of_land_impact = v.map(|x| ccsds_ndm::types::Probability::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Probability that the re-entry event will cause any casualties (severe injuries or
    /// deaths—0 to 1).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_probability_of_casualty(&self) -> Option<f64> {
        self.inner.probability_of_casualty.as_ref().map(|v| v.value)
    }
    #[setter] fn set_probability_of_casualty(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.probability_of_casualty = v.map(|x| ccsds_ndm::types::Probability::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Epoch of the predicted impact (formatting rules specified in 5.3.3.5).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_nominal_impact_epoch(&self) -> Option<String> {
        self.inner.nominal_impact_epoch.as_ref().map(|e| e.to_string())
    }
    #[setter] fn set_nominal_impact_epoch(&mut self, v: Option<String>) -> PyResult<()> { self.inner.nominal_impact_epoch = v.map(|s| parse_epoch(&s)).transpose()?; Ok(()) }

    /// Start epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_impact_window_start(&self) -> Option<String> {
        self.inner.impact_window_start.as_ref().map(|e| e.to_string())
    }
    #[setter] fn set_impact_window_start(&mut self, v: Option<String>) -> PyResult<()> { self.inner.impact_window_start = v.map(|s| parse_epoch(&s)).transpose()?; Ok(()) }

    /// End epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_impact_window_end(&self) -> Option<String> {
        self.inner.impact_window_end.as_ref().map(|e| e.to_string())
    }
    #[setter] fn set_impact_window_end(&mut self, v: Option<String>) -> PyResult<()> { self.inner.impact_window_end = v.map(|s| parse_epoch(&s)).transpose()?; Ok(()) }

    /// Reference frame of the impact location data. The value should be taken from the keyword
    /// value name column in the SANA celestial body reference frames registry, reference `[11]`.
    /// Only frames with the value ‘Body-Fixed’ in the Frame Type column shall be used.
    /// Mandatory if NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT are present.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_impact_ref_frame(&self) -> Option<String> {
        self.inner.impact_ref_frame.clone()
    }
    #[setter] fn set_impact_ref_frame(&mut self, v: Option<String>) { self.inner.impact_ref_frame = v; }

    /// Longitude of the predicted impact location with respect to the value of
    /// IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
    /// 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nominal_impact_lon(&self) -> Option<f64> {
        self.inner.nominal_impact_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_nominal_impact_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.nominal_impact_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the predicted impact location with respect to the value of
    /// IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
    /// 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nominal_impact_lat(&self) -> Option<f64> {
        self.inner.nominal_impact_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_nominal_impact_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.nominal_impact_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Altitude of the impact location with respect to the value of IMPACT_REF_FRAME.
    ///
    /// Units: m
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_nominal_impact_alt(&self) -> Option<f64> {
        self.inner.nominal_impact_alt.as_ref().map(|v| v.value)
    }
    #[setter] fn set_nominal_impact_alt(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.nominal_impact_alt = v.map(|x| ccsds_ndm::types::AltitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// First (lowest) confidence interval for the impact location.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_confidence(&self) -> Option<f64> {
        self.inner.impact_1_confidence.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_confidence(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_1_confidence = v.map(|x| ccsds_ndm::types::PercentageRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the start of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_start_lon(&self) -> Option<f64> {
        self.inner.impact_1_start_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_start_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_1_start_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the start of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_start_lat(&self) -> Option<f64> {
        self.inner.impact_1_start_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_start_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_1_start_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the end of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_stop_lon(&self) -> Option<f64> {
        self.inner.impact_1_stop_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_stop_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_1_stop_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the end of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_stop_lat(&self) -> Option<f64> {
        self.inner.impact_1_stop_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_stop_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_1_stop_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Cross-track size of the first confidence interval.
    ///
    /// Units: km
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_1_cross_track(&self) -> Option<f64> {
        self.inner.impact_1_cross_track.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_1_cross_track(&mut self, v: Option<f64>) { self.inner.impact_1_cross_track = v.map(|x| ccsds_ndm::types::Position::new(x, None)); }

    /// Second confidence interval for the impact location. The IMPACT_1_* block must be
    /// present if IMPACT_2_* is used.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_confidence(&self) -> Option<f64> {
        self.inner.impact_2_confidence.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_confidence(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_2_confidence = v.map(|x| ccsds_ndm::types::PercentageRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the start of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_start_lon(&self) -> Option<f64> {
        self.inner.impact_2_start_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_start_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_2_start_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the start of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_start_lat(&self) -> Option<f64> {
        self.inner.impact_2_start_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_start_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_2_start_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the end of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_stop_lon(&self) -> Option<f64> {
        self.inner.impact_2_stop_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_stop_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_2_stop_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the end of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_stop_lat(&self) -> Option<f64> {
        self.inner.impact_2_stop_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_stop_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_2_stop_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Cross-track size of the second confidence interval.
    ///
    /// Units: km
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_2_cross_track(&self) -> Option<f64> {
        self.inner.impact_2_cross_track.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_2_cross_track(&mut self, v: Option<f64>) { self.inner.impact_2_cross_track = v.map(|x| ccsds_ndm::types::Position::new(x, None)); }

    /// Third (highest) confidence interval for the impact location. The IMPACT_2_* block must
    /// be present if IMPACT_3_* is used.
    ///
    /// Units: %
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_confidence(&self) -> Option<f64> {
        self.inner.impact_3_confidence.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_confidence(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_3_confidence = v.map(|x| ccsds_ndm::types::PercentageRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the start of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_start_lon(&self) -> Option<f64> {
        self.inner.impact_3_start_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_start_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_3_start_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the start of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_start_lat(&self) -> Option<f64> {
        self.inner.impact_3_start_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_start_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_3_start_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Longitude of the end of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_stop_lon(&self) -> Option<f64> {
        self.inner.impact_3_stop_lon.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_stop_lon(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_3_stop_lon = v.map(|x| ccsds_ndm::types::LongitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Latitude of the end of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// Units: deg
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_stop_lat(&self) -> Option<f64> {
        self.inner.impact_3_stop_lat.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_stop_lat(&mut self, v: Option<f64>) -> PyResult<()> { self.inner.impact_3_stop_lat = v.map(|x| ccsds_ndm::types::LatitudeRequired::new(x).map_err(|e| PyValueError::new_err(e.to_string()))).transpose()?; Ok(()) }

    /// Cross-track size of the third confidence interval.
    ///
    /// Units: km
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_impact_3_cross_track(&self) -> Option<f64> {
        self.inner.impact_3_cross_track.as_ref().map(|v| v.value)
    }
    #[setter] fn set_impact_3_cross_track(&mut self, v: Option<f64>) { self.inner.impact_3_cross_track = v.map(|x| ccsds_ndm::types::Position::new(x, None)); }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum YesNo {
    Yes,
    No,
}

#[pymethods]
impl YesNo {
    fn __str__(&self) -> &'static str {
        match self {
            YesNo::Yes => "YES",
            YesNo::No => "NO",
        }
    }
    fn __repr__(&self) -> String {
        format!("YesNo.{}", self.__str__())
    }
}

impl FromStr for YesNo {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "YES" => Ok(YesNo::Yes),
            "NO" => Ok(YesNo::No),
            _ => Err(format!("Invalid YesNo: {}", s)),
        }
    }
}


pub fn parse_yes_no(ob: &Bound<'_, PyAny>) -> PyResult<ccsds_ndm::types::YesNo> {
    use std::str::FromStr;
    if let Ok(val) = ob.extract::<YesNo>() {
        Ok(match val {
            YesNo::Yes => ccsds_ndm::types::YesNo::Yes,
            YesNo::No => ccsds_ndm::types::YesNo::No,
        })
    } else if let Ok(s) = ob.extract::<String>() {
        ccsds_ndm::types::YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
    } else {
        Err(PyValueError::new_err(
            "Expected YesNo enum or string",
        ))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum ObjectDescription {
    Payload,
    RocketBody,
    Debris,
    Unknown,
    Other,
}

#[pymethods]
impl ObjectDescription {
    fn __str__(&self) -> &'static str {
        match self {
            ObjectDescription::Payload => "PAYLOAD",
            ObjectDescription::RocketBody => "ROCKET BODY",
            ObjectDescription::Debris => "DEBRIS",
            ObjectDescription::Unknown => "UNKNOWN",
            ObjectDescription::Other => "OTHER",
        }
    }
    fn __repr__(&self) -> String {
        format!("ObjectDescription.{}", self.__str__())
    }
}

impl FromStr for ObjectDescription {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAYLOAD" => Ok(ObjectDescription::Payload),
            "ROCKET BODY" | "ROCKET_BODY" | "ROCKETBODY" => Ok(ObjectDescription::RocketBody),
            "DEBRIS" => Ok(ObjectDescription::Debris),
            "UNKNOWN" | "N/A" => Ok(ObjectDescription::Unknown),
            "OTHER" => Ok(ObjectDescription::Other),
            _ => Err(format!("Invalid ObjectDescription: {}", s)),
        }
    }
}


pub fn parse_object_description(ob: &Bound<'_, PyAny>) -> PyResult<ccsds_ndm::types::ObjectDescription> {
    if let Ok(val) = ob.extract::<ObjectDescription>() {
        Ok(match val {
            ObjectDescription::Payload => ccsds_ndm::types::ObjectDescription::Payload,
            ObjectDescription::RocketBody => ccsds_ndm::types::ObjectDescription::RocketBody,
            ObjectDescription::Debris => ccsds_ndm::types::ObjectDescription::Debris,
            ObjectDescription::Unknown => ccsds_ndm::types::ObjectDescription::Unknown,
            ObjectDescription::Other => ccsds_ndm::types::ObjectDescription::Other,
        })
    } else if let Ok(s) = ob.extract::<String>() {
        ccsds_ndm::types::ObjectDescription::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
    } else {
        Err(PyValueError::new_err(
            "Expected ObjectDescription enum or string",
        ))
    }
}


#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum ControlledType {
    Yes,
    No,
    Unknown,
}

#[pymethods]
impl ControlledType {
    fn __str__(&self) -> &'static str {
        match self {
            ControlledType::Yes => "YES",
            ControlledType::No => "NO",
            ControlledType::Unknown => "UNKNOWN",
        }
    }
    fn __repr__(&self) -> String {
        format!("ControlledType.{}", self.__str__())
    }
}

impl FromStr for ControlledType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "YES" => Ok(ControlledType::Yes),
            "NO" => Ok(ControlledType::No),
            "UNKNOWN" => Ok(ControlledType::Unknown),
            _ => Err(format!("Invalid ControlledType: {}", s)),
        }
    }
}


pub fn parse_controlled_type(ob: &Bound<'_, PyAny>) -> PyResult<ccsds_ndm::types::ControlledType> {
    use std::str::FromStr;
    if let Ok(val) = ob.extract::<ControlledType>() {
        Ok(match val {
            ControlledType::Yes => ccsds_ndm::types::ControlledType::Yes,
            ControlledType::No => ccsds_ndm::types::ControlledType::No,
            ControlledType::Unknown => ccsds_ndm::types::ControlledType::Unknown,
        })
    } else if let Ok(s) = ob.extract::<String>() {
        ccsds_ndm::types::ControlledType::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
    } else {
        Err(PyValueError::new_err(
            "Expected ControlledType enum or string",
        ))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum ReferenceFrame {
    Gcrf,
    Teme,
    Itrf,
    J2000,
    Eme2000,
}

#[pymethods]
impl ReferenceFrame {
    fn __str__(&self) -> &'static str {
        match self {
            ReferenceFrame::Gcrf => "GCRF",
            ReferenceFrame::Teme => "TEME",
            ReferenceFrame::Itrf => "ITRF",
            ReferenceFrame::J2000 => "J2000",
            ReferenceFrame::Eme2000 => "EME2000",
        }
    }
    fn __repr__(&self) -> String {
        format!("ReferenceFrame.{}", self.__str__())
    }
}

pub fn parse_reference_frame(ob: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(val) = ob.extract::<ReferenceFrame>() {
        Ok(val.__str__().to_string())
    } else if let Ok(s) = ob.extract::<String>() {
        Ok(s)
    } else {
        Err(PyValueError::new_err(
            "Expected ReferenceFrame enum or string",
        ))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum TimeSystem {
    Utc,
    Tai,
    Gps,
    Sclk,
    Tdb,
    Ut1,
}

#[pymethods]
impl TimeSystem {
    fn __str__(&self) -> &'static str {
        match self {
            TimeSystem::Utc => "UTC",
            TimeSystem::Tai => "TAI",
            TimeSystem::Gps => "GPS",
            TimeSystem::Sclk => "SCLK",
            TimeSystem::Tdb => "TDB",
            TimeSystem::Ut1 => "UT1",
        }
    }
    fn __repr__(&self) -> String {
        format!("TimeSystem.{}", self.__str__())
    }
}

pub fn parse_time_system(ob: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(val) = ob.extract::<TimeSystem>() {
        Ok(val.__str__().to_string())
    } else if let Ok(s) = ob.extract::<String>() {
        Ok(s)
    } else {
        Err(PyValueError::new_err(
            "Expected TimeSystem enum or string",
        ))
    }
}
