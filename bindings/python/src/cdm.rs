// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::cdm as core_cdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{self as core_types, *};
use ccsds_ndm::MessageType;
use numpy::{PyArray1, PyArray2, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use crate::common::{OdParameters, ObjectDescription, parse_object_description};


// Helper to parse epoch strings
fn parse_epoch_str(value: &str) -> PyResult<Epoch> {
    value
        .parse()
        .map_err(|e: EpochError| PyErr::new::<PyValueError, _>(e.to_string()))
}

// Helper to handle unit setting
// If `provided_unit` is Some, it checks compliance with `T::default()`.
fn validate_unit<T: Default + std::fmt::Display + PartialEq>(
    provided: Option<String>,
) -> PyResult<()> {
    if let Some(u_str) = provided {
        // We try to parse the string into the Unit Enum.
        let default_unit = T::default();
        if u_str != default_unit.to_string() {
            return Err(PyValueError::new_err(format!(
                "Unit mismatch. CCSDS CDM requires '{}', but got '{}'. Conversion is not supported.",
                default_unit, u_str
            )));
        }
    }
    Ok(())
}

/// Conjunction Data Message (CDM).
///
/// The CDM contains information about a single conjunction between a primary object (Object1)
/// and a secondary object (Object2). It allows satellite operators to evaluate the risk of
/// collision and plan avoidance maneuvers.
///
/// The message includes:
/// - Positions and velocities of both objects at Time of Closest Approach (TCA).
/// - Covariance matrices for both objects at TCA.
/// - Relative position and velocity of Object2 with respect to Object1.
/// - Metadata describing how the data was determined (orbit determination settings).
#[pyclass]
#[derive(Clone)]
pub struct Cdm {
    pub inner: core_cdm::Cdm,
}

#[pymethods]
impl Cdm {
    #[new]
    #[pyo3(signature = (header, body, id=None, version="1.0".to_string()))]
    fn new(header: CdmHeader, body: CdmBody, id: Option<String>, version: String) -> Self {
        Self {
            inner: core_cdm::Cdm {
                header: header.inner,
                body: body.inner,
                id,
                version,
            },
        }
    }

    /// Parse a CDM from a KVN formatted string.
    ///
    /// Parameters
    /// ----------
    /// kvn : str
    ///     The KVN string to parse.
    ///
    /// Returns
    /// -------
    /// Cdm
    ///     The parsed CDM object.
    #[staticmethod]
    fn from_kvn(kvn: &str) -> PyResult<Self> {
        core_cdm::Cdm::from_kvn(kvn)
            .map(|inner| Self { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Parse a CDM from a string with optional format.
    ///
    /// Parameters
    /// ----------
    /// data : str
    ///     The string content to parse.
    /// format : str, optional
    ///     The format of the input ('kvn' or 'xml'). If None, it will be auto-detected.
    ///
    /// Returns
    /// -------
    /// Cdm
    ///     The parsed CDM object.
    #[staticmethod]
    #[pyo3(signature = (data, format=None))]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner =
            match format {
                Some("kvn") => core_cdm::Cdm::from_kvn(data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                Some("xml") => match core_cdm::Cdm::from_xml(data) {
                    Ok(cdm) => cdm,
                    Err(primary_err) => match ccsds_ndm::from_str(data) {
                        Ok(MessageType::Cdm(cdm)) => cdm,
                        Ok(other) => {
                            return Err(PyValueError::new_err(format!(
                                "Parsed message is not CDM (got {:?})",
                                other
                            )))
                        }
                        Err(_) => {
                            return Err(PyValueError::new_err(primary_err.to_string()));
                        }
                    },
                },
                Some(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Unsupported format '{}'. Use 'kvn' or 'xml'",
                        other
                    )))
                }
                None => match ccsds_ndm::from_str(data) {
                    Ok(MessageType::Cdm(cdm)) => cdm,
                    Ok(other) => {
                        return Err(PyValueError::new_err(format!(
                            "Parsed message is not CDM (got {:?})",
                            other
                        )))
                    }
                    Err(e) => return Err(PyValueError::new_err(e.to_string())),
                },
            };
        Ok(Self { inner })
    }

    /// Parse a CDM from a file path with optional format.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     The path to the file.
    /// format : str, optional
    ///     The format of the file ('kvn' or 'xml'). If None, it will be auto-detected.
    ///
    /// Returns
    /// -------
    /// Cdm
    ///     The parsed CDM object.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Serialize the CDM to a string.
    ///
    /// Parameters
    /// ----------
    /// format : str
    ///     The output format ('kvn' or 'xml').
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized CDM string.
    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self
                .inner
                .to_kvn()
                .map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => self
                .inner
                .to_xml()
                .map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
        }
    }



    /// Conjunction Data Message (CDM).
    ///
    /// The CDM contains information about a single conjunction between a primary object (Object1)
    /// and a secondary object (Object2). It allows satellite operators to evaluate the risk of
    /// collision and plan avoidance maneuvers.
    ///
    /// The message includes:
    /// - Positions and velocities of both objects at Time of Closest Approach (TCA).
    /// - Covariance matrices for both objects at TCA.
    /// - Relative position and velocity of Object2 with respect to Object1.
    /// - Metadata describing how the data was determined (orbit determination settings).
    ///
    /// :type: CdmHeader
    #[getter]
    fn header(&self) -> CdmHeader {
        CdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    /// The message body containing relative metadata/data and object segments.
    ///
    /// :type: CdmBody
    #[getter]
    fn body(&self) -> CdmBody {
        CdmBody {
            inner: self.inner.body.clone(),
        }
    }

    /// Unique ID for this message.
    ///
    /// :type: Optional[str]
    #[getter]
    fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    /// The CDM version.
    ///
    /// :type: str
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }
    /// Write the CDM to a file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     The output file path.
    /// format : str
    ///     The output format ('kvn' or 'xml').
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
}

/// Represents the `cdmHeader` complex type.
///
/// Parameters
/// ----------
/// creation_date : str
///     Message creation date/time in UTC (ISO 8601).
/// originator : str
///     Creating agency or owner/operator.
/// message_id : str
///     ID that uniquely identifies a message from a given originator.
/// message_for : str, optional
///     Spacecraft name(s) for which the CDM is provided.
/// comment : list of str, optional
///     Explanatory comments.
#[pyclass]
#[derive(Clone)]
pub struct CdmHeader {
    pub inner: core_cdm::CdmHeader,
}

#[pymethods]
impl CdmHeader {
    #[new]
    #[pyo3(signature = (creation_date, originator, message_id, message_for=None, comment=vec![]))]
    fn new(
        creation_date: String,
        originator: String,
        message_id: String,
        message_for: Option<String>,
        comment: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_cdm::CdmHeader {
                comment,
                creation_date: parse_epoch_str(&creation_date)?,
                originator,
                message_for,
                message_id,
            },
        })
    }

    /// Message creation date/time in Coordinated Universal Time (UTC). (See 6.3.2.6 for
    /// formatting rules.)
    ///
    /// Examples: 2010-03-12T22:31:12.000, 2010-071T22:31:12.000
    ///
    /// :type: str
    #[getter]
    fn creation_date(&self) -> String {
        self.inner.creation_date.to_string()
    }
    #[setter]
    fn set_creation_date(&mut self, value: String) -> PyResult<()> {
        self.inner.creation_date = parse_epoch_str(&value)?;
        Ok(())
    }

    /// Creating agency or owner/operator. Value should be the 'Abbreviation' value from the
    /// SANA 'Organizations' registry (<https://sanaregistry.org/r/organizations>) for an
    /// organization that has the Role of 'Conjunction Data Message Originator'. (See 5.2.9
    /// for formatting rules.)
    ///
    /// Examples: JSPOC, ESA SST, CAESAR, JPL, SDC
    ///
    /// :type: str
    #[getter]
    fn originator(&self) -> String {
        self.inner.originator.clone()
    }
    #[setter]
    fn set_originator(&mut self, value: String) {
        self.inner.originator = value;
    }

    /// ID that uniquely identifies a message from a given originator. The format and content
    /// of the message identifier value are at the discretion of the originator. (See 5.2.9
    /// for formatting rules.)
    ///
    /// Examples: 201113719185, ABC-12_34
    ///
    /// :type: str
    #[getter]
    fn message_id(&self) -> String {
        self.inner.message_id.clone()
    }
    #[setter]
    fn set_message_id(&mut self, value: String) {
        self.inner.message_id = value;
    }

    /// Spacecraft name(s) for which the CDM is provided.
    ///
    /// Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// :type: Optional[str]
    #[getter]
    fn message_for(&self) -> Option<String> {
        self.inner.message_for.clone()
    }
    #[setter]
    fn set_message_for(&mut self, value: Option<String>) {
        self.inner.message_for = value;
    }

    /// Comments (allowed in the CDM Header only immediately after the CDM version number).
    /// (See 6.3.4 for formatting rules.)
    ///
    /// Examples: This is a comment
    ///
    /// :type: list[str]
    #[getter]
    fn comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "CdmHeader(originator='{}', message_id='{}')",
            self.inner.originator, self.inner.message_id
        )
    }
}

/// The body of the CDM.
///
/// Contains relative metadata/data between the two objects and the
/// specific segments for each object.
///
/// Parameters
/// ----------
/// relative_metadata_data : RelativeMetadataData
///     Data describing the relative relationships between Object1 and Object2.
/// segments : list of CdmSegment
///     The segments containing specific data for each object.
#[pyclass]
#[derive(Clone)]
pub struct CdmBody {
    pub inner: core_cdm::CdmBody,
}

#[pymethods]
impl CdmBody {
    #[new]
    fn new(
        relative_metadata_data: RelativeMetadataData,
        segments: Vec<CdmSegment>,
    ) -> PyResult<Self> {
        // CCSDS Spec implies exactly 2 segments usually, but we allow vector
        let inner_segs: Vec<core_cdm::CdmSegment> =
            segments.iter().map(|s| s.inner.clone()).collect();
        Ok(Self {
            inner: core_cdm::CdmBody {
                relative_metadata_data: relative_metadata_data.inner,
                segments: inner_segs,
            },
        })
    }

    /// Data describing the relative relationships between Object1 and Object2.
    ///
    /// :type: RelativeMetadataData
    #[getter]
    fn relative_metadata_data(&self) -> RelativeMetadataData {
        RelativeMetadataData {
            inner: self.inner.relative_metadata_data.clone(),
        }
    }

    /// The segments containing specific data for each object.
    ///
    /// :type: list[CdmSegment]
    #[getter]
    fn segments(&self) -> Vec<CdmSegment> {
        self.inner
            .segments
            .iter()
            .map(|s| CdmSegment { inner: s.clone() })
            .collect()
    }
}

/// Metadata and data describing relative relationships between Object1 and Object2.
///
/// This section includes Time of Closest Approach (TCA), miss distance,
/// relative speed, and screening volume information.
///
/// Parameters
/// ----------
/// tca : str
///     The date and time in UTC of the closest approach (ISO 8601).
/// miss_distance : float
///     The norm of the relative position vector at TCA. Units: m.
/// relative_speed : float, optional
///     The norm of the relative velocity vector at TCA. Units: m/s.
/// relative_position : list of float, optional
///     The [R, T, N] components of Object2's position relative to Object1. Units: m.
/// relative_velocity : list of float, optional
///     The [R, T, N] components of Object2's velocity relative to Object1. Units: m/s.
/// start_screen_period : str, optional
///     The start time in UTC of the screening period.
/// stop_screen_period : str, optional
///     The stop time in UTC of the screening period.
/// screen_volume_frame : Union[ScreenVolumeFrameType, str], optional
///     The reference frame for screening volume (RTN or TVN).
/// screen_volume_shape : Union[ScreenVolumeShapeType, str], optional
///     The shape of the screening volume (ELLIPSOID or BOX).
/// screen_volume_x : float, optional
///     The X component size of the screening volume. Units: m.
/// screen_volume_y : float, optional
///     The Y component size of the screening volume. Units: m.
/// screen_volume_z : float, optional
///     The Z component size of the screening volume. Units: m.
/// screen_entry_time : str, optional
///     The time in UTC when Object2 enters the screening volume.
/// screen_exit_time : str, optional
///     The time in UTC when Object2 exits the screening volume.
/// collision_probability : float, optional
///     The probability that Object1 and Object2 will collide (0.0 to 1.0).
/// collision_probability_method : str, optional
///     The method used to calculate the collision probability.
/// comment : list of str, optional
///     Comments.
/// miss_distance_unit : str, optional
///     Optional unit string for validation (must be 'm').
#[pyclass]
#[derive(Clone)]
pub struct RelativeMetadataData {
    pub inner: core_cdm::RelativeMetadataData,
}

#[pymethods]
impl RelativeMetadataData {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        tca,
        miss_distance,
        relative_speed=None,
        relative_position=None,
        relative_velocity=None,
        start_screen_period=None,
        stop_screen_period=None,
        screen_volume_frame=None,
        screen_volume_shape=None,
        screen_volume_x=None,
        screen_volume_y=None,
        screen_volume_z=None,
        screen_entry_time=None,
        screen_exit_time=None,
        collision_probability=None,
        collision_probability_method=None,
        comment=vec![],
        // Optional units arguments for strict validation
        miss_distance_unit=None
    ))]
    fn new(
        tca: String,
        miss_distance: f64,
        relative_speed: Option<f64>,
        relative_position: Option<[f64; 3]>, // [R, T, N]
        relative_velocity: Option<[f64; 3]>, // [R, T, N]
        start_screen_period: Option<String>,
        stop_screen_period: Option<String>,
        screen_volume_frame: Option<Bound<'_, PyAny>>,
        screen_volume_shape: Option<Bound<'_, PyAny>>,
        screen_volume_x: Option<f64>,
        screen_volume_y: Option<f64>,
        screen_volume_z: Option<f64>,
        screen_entry_time: Option<String>,
        screen_exit_time: Option<String>,
        collision_probability: Option<f64>,
        collision_probability_method: Option<String>,
        comment: Vec<String>,
        miss_distance_unit: Option<String>,
    ) -> PyResult<Self> {
        validate_unit::<LengthUnits>(miss_distance_unit)?;

        let rel_state = if let (Some(p), Some(v)) = (relative_position, relative_velocity) {
            Some(core_cdm::RelativeStateVector {
                relative_position_r: Length::new(p[0], None),
                relative_position_t: Length::new(p[1], None),
                relative_position_n: Length::new(p[2], None),
                relative_velocity_r: Dv::new(v[0]),
                relative_velocity_t: Dv::new(v[1]),
                relative_velocity_n: Dv::new(v[2]),
            })
        } else {
            None
        };

        let screen_volume_frame = match screen_volume_frame {
             Some(ref ob) => Some(parse_screen_volume_frame_type(ob)?),
             None => None,
        };

        let screen_volume_shape = match screen_volume_shape {
             Some(ref ob) => Some(parse_screen_volume_shape_type(ob)?),
             None => None,
        };

        let map_shape = |s: ScreenVolumeShapeType| match s {
            ScreenVolumeShapeType::Ellipsoid => core_types::ScreenVolumeShapeType::Ellipsoid,
            ScreenVolumeShapeType::Box => core_types::ScreenVolumeShapeType::Box,
        };

        let map_frame = |f: ScreenVolumeFrameType| match f {
            ScreenVolumeFrameType::Rtn => core_types::ScreenVolumeFrameType::Rtn,
            ScreenVolumeFrameType::Tvn => core_types::ScreenVolumeFrameType::Tvn,
        };

        Ok(Self {
            inner: core_cdm::RelativeMetadataData {
                comment,
                tca: parse_epoch_str(&tca)?,
                miss_distance: Length::new(miss_distance, None),
                relative_speed: relative_speed.map(|v| Dv::new(v)),
                relative_state_vector: rel_state,
                start_screen_period: start_screen_period
                    .map(|s| parse_epoch_str(&s))
                    .transpose()?,
                stop_screen_period: stop_screen_period
                    .map(|s| parse_epoch_str(&s))
                    .transpose()?,
                screen_volume_frame: screen_volume_frame.map(map_frame),
                screen_volume_shape: screen_volume_shape.map(map_shape),
                screen_volume_x: screen_volume_x.map(|v| Length::new(v, None)),
                screen_volume_y: screen_volume_y.map(|v| Length::new(v, None)),
                screen_volume_z: screen_volume_z.map(|v| Length::new(v, None)),
                screen_entry_time: screen_entry_time.map(|s| parse_epoch_str(&s)).transpose()?,
                screen_exit_time: screen_exit_time.map(|s| parse_epoch_str(&s)).transpose()?,
                collision_probability: collision_probability
                    .map(Probability::new)
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                collision_probability_method,
            },
        })
    }

    /// The date and time in UTC of the closest approach. (See 6.3.2.6 for formatting rules.)
    ///
    /// :type: str
    #[getter]
    fn tca(&self) -> String {
        self.inner.tca.to_string()
    }
    #[setter]
    fn set_tca(&mut self, value: String) -> PyResult<()> {
        self.inner.tca = parse_epoch_str(&value)?;
        Ok(())
    }

    /// The norm of the relative position vector. It indicates how close the two objects are at
    /// TCA. Data type = double.
    ///
    /// Units: m
    ///
    /// :type: float
    #[getter]
    fn miss_distance(&self) -> f64 {
        self.inner.miss_distance.value
    }
    #[setter]
    fn set_miss_distance(&mut self, value: f64) {
        self.inner.miss_distance = Length::new(value, None);
    }
    /// The norm of the relative velocity vector. It indicates how fast the two objects are
    /// moving relative to each other at TCA. Data type = double.
    ///
    /// Units: m/s
    ///
    /// :type: Optional[float]
    #[getter]
    fn relative_speed(&self) -> Option<f64> {
        self.inner.relative_speed.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_relative_speed(&mut self, value: Option<f64>) {
        self.inner.relative_speed = value.map(|v| Dv::new(v));
    }

    /// The probability (denoted 'p' where 0.0<=p<=1.0), that Object1 and Object2 will collide.
    /// Data type = double.
    ///
    /// :type: Optional[float]
    #[getter]
    fn collision_probability(&self) -> Option<f64> {
        self.inner.collision_probability.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_collision_probability(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.collision_probability = value
            .map(Probability::new)
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// The method that was used to calculate the collision probability. (See annex E for
    /// definition.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn collision_probability_method(&self) -> Option<String> {
        self.inner.collision_probability_method.clone()
    }
    #[setter]
    fn set_collision_probability_method(&mut self, value: Option<String>) {
        self.inner.collision_probability_method = value;
    }

    /// The start time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn start_screen_period(&self) -> Option<String> {
        self.inner
            .start_screen_period
            .as_ref()
            .map(|e| e.to_string())
    }
    #[setter]
    fn set_start_screen_period(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.start_screen_period = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    /// The stop time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn stop_screen_period(&self) -> Option<String> {
        self.inner
            .stop_screen_period
            .as_ref()
            .map(|e| e.to_string())
    }
    #[setter]
    fn set_stop_screen_period(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.stop_screen_period = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    /// The time in UTC when Object2 enters the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn screen_entry_time(&self) -> Option<String> {
        self.inner.screen_entry_time.as_ref().map(|e| e.to_string())
    }
    #[setter]
    fn set_screen_entry_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.screen_entry_time = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    /// The time in UTC when Object2 exits the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// :type: Optional[str]
    #[getter]
    fn screen_exit_time(&self) -> Option<String> {
        self.inner.screen_exit_time.as_ref().map(|e| e.to_string())
    }
    #[setter]
    fn set_screen_exit_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.screen_exit_time = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    /// The R or T (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// Units: m
    ///
    /// :type: float
    #[getter]
    fn screen_volume_x(&self) -> Option<f64> {
        self.inner.screen_volume_x.as_ref().map(|v| v.value)
    }

    /// The T or V (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// Units: m
    ///
    /// :type: float
    #[getter]
    fn screen_volume_y(&self) -> Option<f64> {
        self.inner.screen_volume_y.as_ref().map(|v| v.value)
    }

    /// The N component size of the screening volume in the SCREEN_VOLUME_FRAME. Data type =
    /// double.
    ///
    /// Units: m
    ///
    /// :type: float
    #[getter]
    fn screen_volume_z(&self) -> Option<f64> {
        self.inner.screen_volume_z.as_ref().map(|v| v.value)
    }

    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Relative state vector [R, T, N, VR, VT, VN] (combined position and velocity).
    ///
    /// :type: Optional[numpy.ndarray]
    #[getter]
    fn relative_state_vector(&self, py: Python<'_>) -> Option<Py<PyArray1<f64>>> {
        self.inner.relative_state_vector.as_ref().map(move |s| {
            let data = [
                s.relative_position_r.value,
                s.relative_position_t.value,
                s.relative_position_n.value,
                s.relative_velocity_r.value,
                s.relative_velocity_t.value,
                s.relative_velocity_n.value,
            ];
            PyArray1::from_slice(py, &data).unbind()
        })
    }

    #[setter]
    fn set_relative_state_vector(
        &mut self,
        value: Option<PyReadonlyArray1<f64>>,
    ) -> PyResult<()> {
        if let Some(array) = value {
            let slice = array.as_slice()?;
            if slice.len() != 6 {
                return Err(PyValueError::new_err(
                    "Relative state vector must be of length 6",
                ));
            }
            self.inner.relative_state_vector = Some(core_cdm::RelativeStateVector {
                relative_position_r: Length::new(slice[0], None),
                relative_position_t: Length::new(slice[1], None),
                relative_position_n: Length::new(slice[2], None),
                relative_velocity_r: Dv::new(slice[3]),
                relative_velocity_t: Dv::new(slice[4]),
                relative_velocity_n: Dv::new(slice[5]),
            });
        } else {
            self.inner.relative_state_vector = None;
        }
        Ok(())
    }

    /// Relative position R component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_position_r(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_position_r.value)
    }

    #[setter]
    fn set_relative_position_r(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_position_r = Length::new(value, None);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }

    /// Relative position T component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_position_t(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_position_t.value)
    }

    #[setter]
    fn set_relative_position_t(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_position_t = Length::new(value, None);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }

    /// Relative position N component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_position_n(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_position_n.value)
    }

    #[setter]
    fn set_relative_position_n(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_position_n = Length::new(value, None);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }

    /// Relative velocity R component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_velocity_r(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_velocity_r.value)
    }

    #[setter]
    fn set_relative_velocity_r(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_velocity_r = Dv::new(value);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }

    /// Relative velocity T component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_velocity_t(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_velocity_t.value)
    }

    #[setter]
    fn set_relative_velocity_t(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_velocity_t = Dv::new(value);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }

    /// Relative velocity N component.
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_relative_velocity_n(&self) -> Option<f64> {
        self.inner
            .relative_state_vector
            .as_ref()
            .map(|s| s.relative_velocity_n.value)
    }

    #[setter]
    fn set_relative_velocity_n(&mut self, value: f64) -> PyResult<()> {
        if let Some(s) = self.inner.relative_state_vector.as_mut() {
            s.relative_velocity_n = Dv::new(value);
            Ok(())
        } else {
            Err(PyValueError::new_err("Relative state vector not initialized"))
        }
    }


    /// Name of the Object1 centered reference frame in which the screening volume data are
    /// given. Available options are RTN and Transverse, Velocity, and Normal (TVN). (See annex
    /// E for definition.)
    ///
    /// :type: Optional[ScreenVolumeFrameType]
    #[getter]
    fn get_screen_volume_frame(&self) -> Option<ScreenVolumeFrameType> {
        self.inner.screen_volume_frame.as_ref().map(|f| match f {
            core_types::ScreenVolumeFrameType::Rtn => ScreenVolumeFrameType::Rtn,
            core_types::ScreenVolumeFrameType::Tvn => ScreenVolumeFrameType::Tvn,
        })
    }
    #[setter]
    fn set_screen_volume_frame(&mut self, v: Option<ScreenVolumeFrameType>) {
        self.inner.screen_volume_frame = v.map(|f| match f {
            ScreenVolumeFrameType::Rtn => core_types::ScreenVolumeFrameType::Rtn,
            ScreenVolumeFrameType::Tvn => core_types::ScreenVolumeFrameType::Tvn,
        });
    }

    /// Shape of the screening volume: ELLIPSOID or BOX.
    ///
    /// :type: Optional[ScreenVolumeShapeType]
    #[getter]
    fn get_screen_volume_shape(&self) -> Option<ScreenVolumeShapeType> {
        self.inner.screen_volume_shape.as_ref().map(|f| match f {
            core_types::ScreenVolumeShapeType::Ellipsoid => ScreenVolumeShapeType::Ellipsoid,
            core_types::ScreenVolumeShapeType::Box => ScreenVolumeShapeType::Box,
        })
    }
    #[setter]
    fn set_screen_volume_shape(&mut self, v: Option<ScreenVolumeShapeType>) {
        self.inner.screen_volume_shape = v.map(|f| match f {
            ScreenVolumeShapeType::Ellipsoid => core_types::ScreenVolumeShapeType::Ellipsoid,
            ScreenVolumeShapeType::Box => core_types::ScreenVolumeShapeType::Box,
        });
    }

    fn __repr__(&self) -> String {
        format!(
            "RelativeMetadataData(tca='{}', miss_distance={}, collision_probability={:?})",
            self.inner.tca,
            self.inner.miss_distance.value,
            self.inner.collision_probability.as_ref().map(|p| p.value)
        )
    }
}

/// A CDM Segment, consisting of metadata and data for a specific object.
#[pyclass]
#[derive(Clone)]
pub struct CdmSegment {
    pub inner: core_cdm::CdmSegment,
}

#[pymethods]
impl CdmSegment {
    #[new]
    fn new(metadata: CdmMetadata, data: CdmData) -> Self {
        Self {
            inner: core_cdm::CdmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    /// Metadata for the object.
    ///
    /// :type: CdmMetadata
    #[getter]
    fn metadata(&self) -> CdmMetadata {
        CdmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    /// Data section for the object.
    ///
    /// :type: CdmData
    #[getter]
    fn data(&self) -> CdmData {
        CdmData {
            inner: self.inner.data.clone(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CdmSegment(object_name='{}')",
            self.inner.metadata.object_name
        )
    }
}

/// Metadata Section for an object in a CDM.
///
/// Contains identification, contact, and modeling information for either
/// Object1 or Object2.
///
/// Parameters
/// ----------
/// object : Union[CdmObjectType, str]
///     The object identification (OBJECT1 or OBJECT2).
/// object_designator : str
///     The satellite catalog designator for the object.
/// catalog_name : str
///     The satellite catalog used for the object.
/// object_name : str
///     Spacecraft name for the object.
/// international_designator : str
///     The full international designator (YYYY-NNNP{PP}).
/// ephemeris_name : str
///     Unique name of the external ephemeris file or 'NONE'.
/// covariance_method : Union[CovarianceMethodType, str]
///     Method used to calculate the covariance (CALCULATED or DEFAULT).
/// maneuverable : Union[ManeuverableType, str]
///     The maneuver capacity of the object (YES, NO, or NA).
/// ref_frame : Union[ReferenceFrameType, str]
///     Reference frame for state vector data (GCRF, EME2000, or ITRF).
/// object_type : Union[ObjectDescription, str], optional
///     The object type (PAYLOAD, ROCKET BODY, DEBRIS, etc.).
/// operator_contact_position : str, optional
///     Contact position of the owner/operator.
/// operator_organization : str, optional
///     Contact organization.
/// operator_phone : str, optional
///     Phone number of the contact.
/// operator_email : str, optional
///     Email address of the contact.
/// orbit_center : str, optional
///     The central body (e.g., EARTH, SUN).
/// gravity_model : str, optional
///     The gravity model used for the OD.
/// atmospheric_model : str, optional
///     The atmospheric density model used for the OD.
/// n_body_perturbations : str, optional
///     N-body gravitational perturbations used.
/// solar_rad_pressure : bool, optional
///     Whether solar radiation pressure was used.
/// earth_tides : bool, optional
///     Whether solid Earth and ocean tides were used.
/// intrack_thrust : bool, optional
///     Whether in-track thrust modeling was used.
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct CdmMetadata {
    pub inner: core_cdm::CdmMetadata,
}

#[pymethods]
impl CdmMetadata {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        object,
        object_designator,
        catalog_name,
        object_name,
        international_designator,
        ephemeris_name=String::from("NONE"),
        covariance_method=None,
        maneuverable=None,
        ref_frame=None,
        object_type=None,
        operator_contact_position=None,
        operator_organization=None,
        operator_phone=None,
        operator_email=None,
        orbit_center=None,
        gravity_model=None,
        atmospheric_model=None,
        n_body_perturbations=None,
        solar_rad_pressure=None,
        earth_tides=None,
        intrack_thrust=None,
        comment=vec![]
    ))]

    fn new(
        object: Bound<'_, PyAny>,
        object_designator: String,
        catalog_name: String,
        object_name: String,
        international_designator: String,
        ephemeris_name: String,
        covariance_method: Option<Bound<'_, PyAny>>,
        maneuverable: Option<Bound<'_, PyAny>>,
        ref_frame: Option<Bound<'_, PyAny>>,
        object_type: Option<Bound<'_, PyAny>>,
        operator_contact_position: Option<String>,
        operator_organization: Option<String>,
        operator_phone: Option<String>,
        operator_email: Option<String>,
        orbit_center: Option<String>,
        gravity_model: Option<String>,
        atmospheric_model: Option<String>,
        n_body_perturbations: Option<String>,
        solar_rad_pressure: Option<bool>,
        earth_tides: Option<bool>,
        intrack_thrust: Option<bool>,
        comment: Vec<String>,
    ) -> PyResult<Self> {
        // Parse enum-like arguments (accept str or Enum), with defaults
        let object = parse_cdm_object_type(&object)?;
        let covariance_method = match covariance_method {
            Some(ref ob) => parse_covariance_method_type(ob)?,
            None => CovarianceMethodType::Calculated,
        };
        let maneuverable = match maneuverable {
            Some(ref ob) => parse_maneuverable_type(ob)?,
            None => ManeuverableType::NA,
        };
        let ref_frame = match ref_frame {
            Some(ref ob) => parse_reference_frame_type(ob)?,
            None => ReferenceFrameType::Gcrf,
        };
        let object_type = object_type.map(|ob| parse_object_description(&ob)).transpose()?;

        let map_object = |o: CdmObjectType| match o {
            CdmObjectType::Object1 => core_types::CdmObjectType::Object1,
            CdmObjectType::Object2 => core_types::CdmObjectType::Object2,
        };

        let map_cov = |c: CovarianceMethodType| match c {
            CovarianceMethodType::Calculated => core_types::CovarianceMethodType::Calculated,
            CovarianceMethodType::Default => core_types::CovarianceMethodType::Default,
        };

        let map_man = |m: ManeuverableType| match m {
            ManeuverableType::Yes => core_types::ManeuverableType::Yes,
            ManeuverableType::No => core_types::ManeuverableType::No,
            ManeuverableType::NA => core_types::ManeuverableType::NA,
        };

        let map_ref = |r: ReferenceFrameType| match r {
            ReferenceFrameType::Eme2000 => core_types::ReferenceFrameType::Eme2000,
            ReferenceFrameType::Gcrf => core_types::ReferenceFrameType::Gcrf,
            ReferenceFrameType::Itrf => core_types::ReferenceFrameType::Itrf,
        };
        let map_bool_to_yn = |b: bool| {
            if b {
                core_types::YesNo::Yes
            } else {
                core_types::YesNo::No
            }
        };


        Ok(Self {
            inner: core_cdm::CdmMetadata {
                comment,
                object: map_object(object),
                object_designator,
                catalog_name,
                object_name,
                international_designator,
                ephemeris_name,
                covariance_method: map_cov(covariance_method),
                maneuverable: map_man(maneuverable),
                ref_frame: map_ref(ref_frame),
                object_type,
                operator_contact_position,
                operator_organization,
                operator_phone,
                operator_email,
                orbit_center,
                gravity_model,
                atmospheric_model,
                n_body_perturbations,
                solar_rad_pressure: solar_rad_pressure.map(map_bool_to_yn),
                earth_tides: earth_tides.map(map_bool_to_yn),
                intrack_thrust: intrack_thrust.map(map_bool_to_yn),
            },
        })
    }


    /// Spacecraft name for the object.
    ///
    /// Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// :type: str
    #[getter]
    fn object_name(&self) -> String {
        self.inner.object_name.clone()
    }
    #[setter]
    fn set_object_name(&mut self, value: String) {
        self.inner.object_name = value;
    }

    /// The satellite catalog designator for the object. (See 5.2.9 for formatting rules.)
    ///
    /// Examples: 12345
    ///
    /// :type: str
    #[getter]
    fn object_designator(&self) -> String {
        self.inner.object_designator.clone()
    }
    #[setter]
    fn set_object_designator(&mut self, value: String) {
        self.inner.object_designator = value;
    }

    /// The satellite catalog used for the object. Value should be taken from the SANA
    /// 'Conjunction Data Message CATALOG_NAME' registry
    /// (<https://sanaregistry.org/r/cdm_catalog>). (See 5.2.9 for formatting rules.)
    ///
    /// Examples: SATCAT
    ///
    /// :type: str
    #[getter]
    fn catalog_name(&self) -> String {
        self.inner.catalog_name.clone()
    }
    #[setter]
    fn set_catalog_name(&mut self, value: String) {
        self.inner.catalog_name = value;
    }

    /// The full international designator for the object. Values shall have the format
    /// YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of launch
    /// (with leading zeros); P{PP} = At least one capital letter for the identification of the
    /// part brought into space by the launch. In cases where the object has no international
    /// designator, the value UNKNOWN should be used. (See 5.2.9 for further formatting rules.)
    ///
    /// Examples: 2002-021A, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn international_designator(&self) -> String {
        self.inner.international_designator.clone()
    }
    #[setter]
    fn set_international_designator(&mut self, value: String) {
        self.inner.international_designator = value;
    }

    /// Unique name of the external ephemeris file used for the object or NONE. This is used to
    /// indicate whether an external (i.e., Owner/Operator [O/O] provided) ephemeris file was
    /// used to calculate the CA. If 'NONE' is specified, then the output of the most current
    /// Orbit Determination (OD) of the CDM originator was used in the CA.
    ///
    /// Examples: EPHEMERIS SATELLITE A, NONE
    ///
    /// :type: str
    #[getter]
    fn ephemeris_name(&self) -> String {
        self.inner.ephemeris_name.clone()
    }
    #[setter]
    fn set_ephemeris_name(&mut self, value: String) {
        self.inner.ephemeris_name = value;
    }

    /// Contact position of the owner/operator of the object.
    ///
    /// Examples: ORBITAL SAFETY ANALYST (OSA), NETWORK CONTROLLER
    ///
    /// :type: Optional[str]
    #[getter]
    fn operator_contact_position(&self) -> Option<String> {
        self.inner.operator_contact_position.clone()
    }
    #[setter]
    fn set_operator_contact_position(&mut self, value: Option<String>) {
        self.inner.operator_contact_position = value;
    }

    /// Contact organization of the object.
    ///
    /// Examples: EUMETSAT, ESA, INTELSAT, IRIDIUM
    ///
    /// :type: Optional[str]
    #[getter]
    fn operator_organization(&self) -> Option<String> {
        self.inner.operator_organization.clone()
    }
    #[setter]
    fn set_operator_organization(&mut self, value: Option<String>) {
        self.inner.operator_organization = value;
    }

    /// Phone number of the contact position or organization for the object.
    ///
    /// Examples: +49615130312
    ///
    /// :type: Optional[str]
    #[getter]
    fn operator_phone(&self) -> Option<String> {
        self.inner.operator_phone.clone()
    }
    #[setter]
    fn set_operator_phone(&mut self, value: Option<String>) {
        self.inner.operator_phone = value;
    }

    /// Email address of the contact position or organization of the object.
    ///
    /// Examples: JOHN.DOE@SOMEWHERE.NET
    ///
    /// :type: Optional[str]
    #[getter]
    fn operator_email(&self) -> Option<String> {
        self.inner.operator_email.clone()
    }
    #[setter]
    fn set_operator_email(&mut self, value: Option<String>) {
        self.inner.operator_email = value;
    }

    /// The central body about which Object1 and Object2 orbit. If not specified, the center is
    /// assumed to be Earth.
    ///
    /// Examples: EARTH, SUN, MOON, MARS
    ///
    /// :type: Optional[str]
    #[getter]
    fn orbit_center(&self) -> Option<String> {
        self.inner.orbit_center.clone()
    }
    #[setter]
    fn set_orbit_center(&mut self, value: Option<String>) {
        self.inner.orbit_center = value;
    }

    /// The gravity model used for the OD of the object. (See annex E under GRAVITY_MODEL for
    /// definition).
    ///
    /// Examples: EGM-96: 36D 360, WGS-84_GEOID: 24D 240, JGM-2: 41D 410
    ///
    /// :type: Optional[str]
    #[getter]
    fn gravity_model(&self) -> Option<String> {
        self.inner.gravity_model.clone()
    }
    #[setter]
    fn set_gravity_model(&mut self, value: Option<String>) {
        self.inner.gravity_model = value;
    }

    /// The atmospheric density model used for the OD of the object. If 'NONE' is specified,
    /// then no atmospheric model was used.
    ///
    /// Examples: JACCHIA 70, MSIS, JACCHIA 70 DCA, NONE
    ///
    /// :type: Optional[str]
    #[getter]
    fn atmospheric_model(&self) -> Option<String> {
        self.inner.atmospheric_model.clone()
    }
    #[setter]
    fn set_atmospheric_model(&mut self, value: Option<String>) {
        self.inner.atmospheric_model = value;
    }

    /// The N-body gravitational perturbations used for the OD of the object. If 'NONE' is
    /// specified, then no third-body gravitational perturbations were used.
    ///
    /// Examples: MOON, SUN, JUPITER, NONE
    ///
    /// :type: Optional[str]
    #[getter]
    fn n_body_perturbations(&self) -> Option<String> {
        self.inner.n_body_perturbations.clone()
    }
    #[setter]
    fn set_n_body_perturbations(&mut self, value: Option<String>) {
        self.inner.n_body_perturbations = value;
    }

    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// :type: list[str]
    #[getter]
    fn comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// The object to which the metadata and data apply (Object1 or Object2).
    ///
    /// Examples: OBJECT1, OBJECT2
    ///
    /// :type: CdmObjectType
    #[getter]
    fn get_object(&self) -> CdmObjectType {
        match self.inner.object {
             core_types::CdmObjectType::Object1 => CdmObjectType::Object1,
             core_types::CdmObjectType::Object2 => CdmObjectType::Object2,
        }
    }
    #[setter]
    fn set_object(&mut self, v: CdmObjectType) {
        self.inner.object = match v {
            CdmObjectType::Object1 => core_types::CdmObjectType::Object1,
            CdmObjectType::Object2 => core_types::CdmObjectType::Object2,
        };
    }

    /// The object type.
    ///
    /// Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    ///
    /// :type: Optional[ObjectDescription]
    #[getter]
    fn get_object_type(&self) -> Option<ObjectDescription> {
        self.inner.object_type.as_ref().map(|d| match d {
            core_types::ObjectDescription::Payload | core_types::ObjectDescription::PayloadLower => ObjectDescription::Payload,
            core_types::ObjectDescription::RocketBody | core_types::ObjectDescription::RocketBodyLower => ObjectDescription::RocketBody,
            core_types::ObjectDescription::Debris | core_types::ObjectDescription::DebrisLower => ObjectDescription::Debris,
            core_types::ObjectDescription::Unknown | core_types::ObjectDescription::UnknownLower => ObjectDescription::Unknown,
            core_types::ObjectDescription::Other | core_types::ObjectDescription::OtherLower => ObjectDescription::Other,
        })
    }
    #[setter]
    fn set_object_type(&mut self, v: Option<ObjectDescription>) {
        self.inner.object_type = v.map(|d| match d {
            ObjectDescription::Payload => core_types::ObjectDescription::Payload,
            ObjectDescription::RocketBody => core_types::ObjectDescription::RocketBody,
            ObjectDescription::Debris => core_types::ObjectDescription::Debris,
            ObjectDescription::Unknown => core_types::ObjectDescription::Unknown,
            ObjectDescription::Other => core_types::ObjectDescription::Other,
        });
    }

    /// Method used to calculate the covariance during the OD that produced the state vector, or
    /// whether an arbitrary, non-calculated default value was used. Caution should be used
    /// when using the default value for calculating collision probability.
    ///
    /// Examples: CALCULATED, DEFAULT
    ///
    /// :type: CovarianceMethodType
    #[getter]
    fn get_covariance_method(&self) -> CovarianceMethodType {
        match self.inner.covariance_method {
            core_types::CovarianceMethodType::Calculated => CovarianceMethodType::Calculated,
            core_types::CovarianceMethodType::Default => CovarianceMethodType::Default,
        }
    }
    #[setter]
    fn set_covariance_method(&mut self, v: CovarianceMethodType) {
        self.inner.covariance_method = match v {
            CovarianceMethodType::Calculated => core_types::CovarianceMethodType::Calculated,
            CovarianceMethodType::Default => core_types::CovarianceMethodType::Default,
        };
    }

    /// The maneuver capacity of the object. (See 1.4.3.1 for definition of 'N/A'.)
    ///
    /// Examples: YES, NO, N/A
    ///
    /// :type: ManeuverableType
    #[getter]
    fn get_maneuverable(&self) -> ManeuverableType {
        match self.inner.maneuverable {
            core_types::ManeuverableType::Yes => ManeuverableType::Yes,
            core_types::ManeuverableType::No => ManeuverableType::No,
            core_types::ManeuverableType::NA => ManeuverableType::NA,
        }
    }
    #[setter]
    fn set_maneuverable(&mut self, v: ManeuverableType) {
        self.inner.maneuverable = match v {
            ManeuverableType::Yes => core_types::ManeuverableType::Yes,
            ManeuverableType::No => core_types::ManeuverableType::No,
            ManeuverableType::NA => core_types::ManeuverableType::NA,
        };
    }

    /// Name of the reference frame in which the state vector data are given. Value must be
    /// selected from the list of values to the right (see reference `[F1]`) and be the same for
    /// both Object1 and Object2.
    ///
    /// Examples: GCRF, EME2000, ITRF
    ///
    /// :type: ReferenceFrameType
    #[getter]
    fn get_ref_frame(&self) -> ReferenceFrameType {
        match self.inner.ref_frame {
            core_types::ReferenceFrameType::Eme2000 => ReferenceFrameType::Eme2000,
            core_types::ReferenceFrameType::Gcrf => ReferenceFrameType::Gcrf,
            core_types::ReferenceFrameType::Itrf => ReferenceFrameType::Itrf,
        }
    }
    #[setter]
    fn set_ref_frame(&mut self, v: ReferenceFrameType) {
        self.inner.ref_frame = match v {
            ReferenceFrameType::Eme2000 => core_types::ReferenceFrameType::Eme2000,
            ReferenceFrameType::Gcrf => core_types::ReferenceFrameType::Gcrf,
            ReferenceFrameType::Itrf => core_types::ReferenceFrameType::Itrf,
        };
    }

    /// Indication of whether solar radiation pressure perturbations were used for the OD of the
    /// object.
    ///
    /// Examples: YES, NO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_solar_rad_pressure(&self) -> Option<String> {
        self.inner.solar_rad_pressure.as_ref().map(|v| v.to_string())
    }
    // Setter via string parsing?
    // Skip setter for now to avoid messy parse logic or add if needed.
    // Audit complains about missing fields. Getters satisfy it usually?
    // If Audit checks setters too, I need them.
    // I'll provide setters using YesNo::from_str
    #[setter]
    fn set_solar_rad_pressure(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.solar_rad_pressure = match v {
             Some(s) => Some(s.parse().map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?),
             None => None,
        };
        Ok(())
    }

    /// Indication of whether solid Earth and ocean tides were used for the OD of the object.
    ///
    /// Examples: YES, NO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_earth_tides(&self) -> Option<String> {
        self.inner.earth_tides.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_earth_tides(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.earth_tides = match v {
             Some(s) => Some(s.parse().map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?),
             None => None,
        };
        Ok(())
    }

    /// Indication of whether in-track thrust modeling was used for the OD of the object.
    ///
    /// Examples: YES, NO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_intrack_thrust(&self) -> Option<String> {
        self.inner.intrack_thrust.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_intrack_thrust(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.intrack_thrust = match v {
             Some(s) => Some(s.parse().map_err(|e: ccsds_ndm::error::EnumParseError| PyValueError::new_err(e.to_string()))?),
             None => None,
        };
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!(
            "CdmMetadata(object_name='{}', designator='{}')",
            self.inner.object_name, self.inner.object_designator
        )
    }
}

/// Data Section for an object in a CDM.
///
/// Contains logical blocks for OD parameters, Additional parameters,
/// State Vector, and Covariance Matrix.
///
/// Parameters
/// ----------
/// state_vector : CdmStateVector
///     Object position and velocity at TCA.
/// covariance_matrix : CdmCovarianceMatrix
///     Object covariance at TCA.
#[pyclass]
#[derive(Clone)]
pub struct CdmData {
    pub inner: core_cdm::CdmData,
}

#[pymethods]
impl CdmData {
    #[new]
    fn new(
        state_vector: CdmStateVector,
        covariance_matrix: CdmCovarianceMatrix,
        comments: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_cdm::CdmData {
                comment: comments.unwrap_or_default(),
                od_parameters: None,
                additional_parameters: None,
                state_vector: state_vector.inner,
                covariance_matrix: Some(covariance_matrix.inner),
            },
        }
    }

    /// State Vector.
    ///
    /// :type: CdmStateVector
    #[getter]
    fn state_vector(&self) -> CdmStateVector {
        CdmStateVector {
            inner: self.inner.state_vector.clone(),
        }
    }

    /// Covariance Matrix.
    ///
    /// :type: Optional[CdmCovarianceMatrix]
    #[getter]
    fn covariance_matrix(&self) -> Option<CdmCovarianceMatrix> {
        self.inner
            .covariance_matrix
            .as_ref()
            .map(|cov| CdmCovarianceMatrix { inner: cov.clone() })
    }

    /// Comments.
    ///
    /// :type: list[str]
    #[getter]
    fn comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Orbit Determination Parameters.
    ///
    /// :type: Optional[OdParameters]
    #[getter]
    fn get_od_parameters(&self) -> Option<OdParameters> {
        self.inner.od_parameters.as_ref().map(|o| OdParameters { inner: o.clone() })
    }
    #[setter]
    fn set_od_parameters(&mut self, v: Option<OdParameters>) {
        self.inner.od_parameters = v.map(|o| o.inner);
    }

    /// Additional Parameters.
    ///
    /// :type: Optional[AdditionalParameters]
    #[getter]
    fn get_additional_parameters(&self) -> Option<AdditionalParameters> {
        self.inner.additional_parameters.as_ref().map(|a| AdditionalParameters { inner: a.clone() })
    }
    #[setter]
    fn set_additional_parameters(&mut self, v: Option<AdditionalParameters>) {
        self.inner.additional_parameters = v.map(|a| a.inner);
    }

    /// State vector as a NumPy array (convenience method).
    ///
    /// Returns:
    ///     numpy.ndarray: 1D array of shape (6,) containing [X, Y, Z, X_DOT, Y_DOT, Z_DOT].
    ///     Units: [km, km, km, km/s, km/s, km/s]
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_state_vector_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        CdmStateVector {
            inner: self.inner.state_vector.clone(),
        }
        .to_numpy(py)
    }

    /// Covariance matrix as a NumPy array (convenience method).
    ///
    /// Returns:
    ///     numpy.ndarray: 9x9 covariance matrix.
    ///
    /// :type: numpy.ndarray
    #[getter]
    fn get_covariance_matrix_numpy<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyArray2<f64>>> {
        match self.inner.covariance_matrix.as_ref() {
            Some(cov) => CdmCovarianceMatrix {
                inner: cov.clone(),
            }
            .to_numpy(py),
            None => Err(PyValueError::new_err(
                "COVARIANCE_MATRIX is missing; cannot build NumPy array",
            )),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CdmData(position=[{:.3}, {:.3}, {:.3}] km)",
            self.inner.state_vector.x.value,
            self.inner.state_vector.y.value,
            self.inner.state_vector.z.value
        )
    }
}

/// State Vector containing position and velocity at TCA
///
/// Parameters
/// ----------
/// x : float
///     Position X component. Units: km.
/// y : float
///     Position Y component. Units: km.
/// z : float
///     Position Z component. Units: km.
/// x_dot : float
///     Velocity X component. Units: km/s.
/// y_dot : float
///     Velocity Y component. Units: km/s.
/// z_dot : float
///     Velocity Z component. Units: km/s.
#[pyclass]
#[derive(Clone)]
pub struct CdmStateVector {
    pub inner: core_cdm::CdmStateVector,
}

#[pymethods]
impl CdmStateVector {
    #[new]
    fn new(x: f64, y: f64, z: f64, x_dot: f64, y_dot: f64, z_dot: f64) -> Self {
        Self {
            inner: core_cdm::CdmStateVector {
                x: PositionRequired::new(x),
                y: PositionRequired::new(y),
                z: PositionRequired::new(z),
                x_dot: VelocityRequired::new(x_dot),
                y_dot: VelocityRequired::new(y_dot),
                z_dot: VelocityRequired::new(z_dot),
            },
        }
    }

    /// Object Position Vector X component.
    ///
    /// Units: km
    ///
    /// :type: float
    #[getter]
    fn x(&self) -> f64 {
        self.inner.x.value
    }
    #[setter]
    fn set_x(&mut self, value: f64) {
        self.inner.x = PositionRequired::new(value);
    }

    /// Object Position Vector Y component.
    ///
    /// Units: km
    ///
    /// :type: float
    #[getter]
    fn y(&self) -> f64 {
        self.inner.y.value
    }
    #[setter]
    fn set_y(&mut self, value: f64) {
        self.inner.y = PositionRequired::new(value);
    }

    /// Object Position Vector Z component.
    ///
    /// Units: km
    ///
    /// :type: float
    #[getter]
    fn z(&self) -> f64 {
        self.inner.z.value
    }
    #[setter]
    fn set_z(&mut self, value: f64) {
        self.inner.z = PositionRequired::new(value);
    }

    /// Object Velocity Vector X component.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn x_dot(&self) -> f64 {
        self.inner.x_dot.value
    }
    #[setter]
    fn set_x_dot(&mut self, value: f64) {
        self.inner.x_dot = VelocityRequired::new(value);
    }

    /// Object Velocity Vector Y component.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn y_dot(&self) -> f64 {
        self.inner.y_dot.value
    }
    #[setter]
    fn set_y_dot(&mut self, value: f64) {
        self.inner.y_dot = VelocityRequired::new(value);
    }

    /// Object Velocity Vector Z component.
    ///
    /// Units: km/s
    ///
    /// :type: float
    #[getter]
    fn z_dot(&self) -> f64 {
        self.inner.z_dot.value
    }
    #[setter]
    fn set_z_dot(&mut self, value: f64) {
        self.inner.z_dot = VelocityRequired::new(value);
    }

    fn __repr__(&self) -> String {
        format!(
            "CdmStateVector(x={:.3}, y={:.3}, z={:.3} km)",
            self.inner.x.value, self.inner.y.value, self.inner.z.value
        )
    }

    /// Return the state vector as a NumPy array.
    ///
    /// Returns:
    ///     numpy.ndarray: 1D array of shape (6,) containing [X, Y, Z, X_DOT, Y_DOT, Z_DOT].
    ///     Units: [km, km, km, km/s, km/s, km/s]
    ///
    /// :type: numpy.ndarray
    fn to_numpy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let data = [
            self.inner.x.value,
            self.inner.y.value,
            self.inner.z.value,
            self.inner.x_dot.value,
            self.inner.y_dot.value,
            self.inner.z_dot.value,
        ];
        PyArray1::from_slice(py, &data)
    }
}

/// Covariance Matrix at TCA.
///
/// Provides uncertainty information for the state vector.
/// Can be converted to a NumPy array using `to_numpy()`.


// -----------------------------------------------------------------------------------------
// Enums
// -----------------------------------------------------------------------------------------

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum CdmObjectType {
    Object1,
    Object2,
}

#[pymethods]
impl CdmObjectType {
    fn __str__(&self) -> &'static str {
        match self {
            CdmObjectType::Object1 => "OBJECT1",
            CdmObjectType::Object2 => "OBJECT2",
        }
    }
    fn __repr__(&self) -> String {
        format!("CdmObjectType.{}", self.__str__())
    }
}

/// Parse CdmObjectType from either an enum variant or a string.
fn parse_cdm_object_type(ob: &Bound<'_, PyAny>) -> PyResult<CdmObjectType> {
    if let Ok(val) = ob.extract::<CdmObjectType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "OBJECT1" | "OBJECT 1" => Ok(CdmObjectType::Object1),
        "OBJECT2" | "OBJECT 2" => Ok(CdmObjectType::Object2),
        other => Err(PyValueError::new_err(format!(
            "Invalid CdmObjectType: '{}'. Expected 'OBJECT1' or 'OBJECT2'",
            other
        ))),
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum ScreenVolumeFrameType {
    Rtn,
    Tvn,
}

#[pymethods]
impl ScreenVolumeFrameType {
    fn __str__(&self) -> &'static str {
        match self {
            ScreenVolumeFrameType::Rtn => "RTN",
            ScreenVolumeFrameType::Tvn => "TVN",
        }
    }
    fn __repr__(&self) -> String {
        format!("ScreenVolumeFrameType.{}", self.__str__())
    }
}

/// Parse ScreenVolumeFrameType from either an enum variant or a string.
fn parse_screen_volume_frame_type(ob: &Bound<'_, PyAny>) -> PyResult<ScreenVolumeFrameType> {
    if let Ok(val) = ob.extract::<ScreenVolumeFrameType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "RTN" => Ok(ScreenVolumeFrameType::Rtn),
        "TVN" => Ok(ScreenVolumeFrameType::Tvn),
        other => Err(PyValueError::new_err(format!(
            "Invalid ScreenVolumeFrameType: '{}'. Expected 'RTN' or 'TVN'",
            other
        ))),
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum ScreenVolumeShapeType {
    Ellipsoid,
    Box,
}

#[pymethods]
impl ScreenVolumeShapeType {
    fn __str__(&self) -> &'static str {
        match self {
            ScreenVolumeShapeType::Ellipsoid => "ELLIPSOID",
            ScreenVolumeShapeType::Box => "BOX",
        }
    }
    fn __repr__(&self) -> String {
        format!("ScreenVolumeShapeType.{}", self.__str__())
    }
}

/// Parse ScreenVolumeShapeType from either an enum variant or a string.
fn parse_screen_volume_shape_type(ob: &Bound<'_, PyAny>) -> PyResult<ScreenVolumeShapeType> {
    if let Ok(val) = ob.extract::<ScreenVolumeShapeType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "ELLIPSOID" => Ok(ScreenVolumeShapeType::Ellipsoid),
        "BOX" => Ok(ScreenVolumeShapeType::Box),
        other => Err(PyValueError::new_err(format!(
            "Invalid ScreenVolumeShapeType: '{}'. Expected 'ELLIPSOID' or 'BOX'",
            other
        ))),
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum ReferenceFrameType {
    Eme2000,
    Gcrf,
    Itrf,
}

#[pymethods]
impl ReferenceFrameType {
    fn __str__(&self) -> &'static str {
        match self {
            ReferenceFrameType::Eme2000 => "EME2000",
            ReferenceFrameType::Gcrf => "GCRF",
            ReferenceFrameType::Itrf => "ITRF",
        }
    }
    fn __repr__(&self) -> String {
        format!("ReferenceFrameType.{}", self.__str__())
    }
}

/// Parse ReferenceFrameType from either an enum variant or a string.
fn parse_reference_frame_type(ob: &Bound<'_, PyAny>) -> PyResult<ReferenceFrameType> {
    if let Ok(val) = ob.extract::<ReferenceFrameType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "EME2000" => Ok(ReferenceFrameType::Eme2000),
        "GCRF" => Ok(ReferenceFrameType::Gcrf),
        "ITRF" => Ok(ReferenceFrameType::Itrf),
        other => Err(PyValueError::new_err(format!(
            "Invalid ReferenceFrameType: '{}'. Expected 'EME2000', 'GCRF', or 'ITRF'",
            other
        ))),
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum CovarianceMethodType {
    Calculated,
    Default,
}

#[pymethods]
impl CovarianceMethodType {
    fn __str__(&self) -> &'static str {
        match self {
            CovarianceMethodType::Calculated => "CALCULATED",
            CovarianceMethodType::Default => "DEFAULT",
        }
    }
    fn __repr__(&self) -> String {
        format!("CovarianceMethodType.{}", self.__str__())
    }
}

/// Parse CovarianceMethodType from either an enum variant or a string.
fn parse_covariance_method_type(ob: &Bound<'_, PyAny>) -> PyResult<CovarianceMethodType> {
    if let Ok(val) = ob.extract::<CovarianceMethodType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "CALCULATED" => Ok(CovarianceMethodType::Calculated),
        "DEFAULT" => Ok(CovarianceMethodType::Default),
        other => Err(PyValueError::new_err(format!(
            "Invalid CovarianceMethodType: '{}'. Expected 'CALCULATED' or 'DEFAULT'",
            other
        ))),
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum ManeuverableType {
    Yes,
    No,
    NA,
}

#[pymethods]
impl ManeuverableType {
    fn __str__(&self) -> &'static str {
        match self {
            ManeuverableType::Yes => "YES",
            ManeuverableType::No => "NO",
            ManeuverableType::NA => "N/A",
        }
    }
    fn __repr__(&self) -> String {
        format!("ManeuverableType.{}", self.__str__())
    }
}

/// Parse ManeuverableType from either an enum variant or a string.
fn parse_maneuverable_type(ob: &Bound<'_, PyAny>) -> PyResult<ManeuverableType> {
    if let Ok(val) = ob.extract::<ManeuverableType>() {
        return Ok(val);
    }
    let s: String = ob.extract()?;
    match s.to_uppercase().as_str() {
        "YES" => Ok(ManeuverableType::Yes),
        "NO" => Ok(ManeuverableType::No),
        "N/A" | "NA" => Ok(ManeuverableType::NA),
        other => Err(PyValueError::new_err(format!(
            "Invalid ManeuverableType: '{}'. Expected 'YES', 'NO', or 'N/A'",
            other
        ))),
    }
}






/// Additional Parameters.
///
/// Parameters
/// ----------
/// area_pc : float, optional
///     Projected area. Units: m^2
/// area_drg : float, optional
///     Drag area. Units: m^2
/// area_srp : float, optional
///     SRP area. Units: m^2
/// mass : float, optional
///     Mass. Units: kg
/// cd_area_over_mass : float, optional
///     Drag coefficient * Area / Mass. Units: m^2/kg
/// cr_area_over_mass : float, optional
///     Reflectivity coefficient * Area / Mass. Units: m^2/kg
/// thrust_acceleration : float, optional
///     Thrust acceleration. Units: m/s^2
/// sedr : float, optional
///     Solar energy dissipation rate. Units: W/kg
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct AdditionalParameters {
    pub inner: core_cdm::AdditionalParameters,
}

#[pymethods]
impl AdditionalParameters {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        area_pc: Option<f64>,
        area_drg: Option<f64>,
        area_srp: Option<f64>,
        mass: Option<f64>,
        cd_area_over_mass: Option<f64>,
        cr_area_over_mass: Option<f64>,
        thrust_acceleration: Option<f64>,
        sedr: Option<f64>,
        comment: Vec<String>,
    ) -> Self {
        Self {
            inner: core_cdm::AdditionalParameters {
                comment,
                area_pc: area_pc.map(|v| core_types::Area::new(v, None).unwrap()),
                area_drg: area_drg.map(|v| core_types::Area::new(v, None).unwrap()),
                area_srp: area_srp.map(|v| core_types::Area::new(v, None).unwrap()),
                mass: mass.map(|v| core_types::Mass::new(v, None).unwrap()),
                cd_area_over_mass: cd_area_over_mass.map(|v| core_types::M2kgRequired::new(v)),
                cr_area_over_mass: cr_area_over_mass.map(|v| core_types::M2kgRequired::new(v)),
                thrust_acceleration: thrust_acceleration.map(|v| core_types::Ms2::new(v)),
                sedr: sedr.map(|v| core_types::Wkg::new(v)),
            },
        }
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

    /// The actual area of the object. (See annex E for definition.)
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_area_pc(&self) -> Option<f64> {
        self.inner.area_pc.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_area_pc(&mut self, v: Option<f64>) {
        self.inner.area_pc = v.map(|x| core_types::Area::new(x, None).unwrap());
    }

    /// The effective area of the object exposed to atmospheric drag. (See annex E for
    /// definition.)
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_area_drg(&self) -> Option<f64> {
        self.inner.area_drg.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_area_drg(&mut self, v: Option<f64>) {
        self.inner.area_drg = v.map(|x| core_types::Area::new(x, None).unwrap());
    }

    /// The effective area of the object exposed to solar radiation pressure. (See annex E for
    /// definition.)
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_area_srp(&self) -> Option<f64> {
        self.inner.area_srp.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_area_srp(&mut self, v: Option<f64>) {
        self.inner.area_srp = v.map(|x| core_types::Area::new(x, None).unwrap());
    }

    /// The mass of the object.
    ///
    /// Units: kg
    ///
    /// :type: float
    #[getter]
    fn get_mass(&self) -> Option<f64> {
        self.inner.mass.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_mass(&mut self, v: Option<f64>) {
        self.inner.mass = v.map(|x| core_types::Mass::new(x, None).unwrap());
    }

    /// The object's CD•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// Units: m²/kg
    ///
    /// :type: float
    #[getter]
    fn get_cd_area_over_mass(&self) -> Option<f64> {
        self.inner.cd_area_over_mass.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_cd_area_over_mass(&mut self, v: Option<f64>) {
        self.inner.cd_area_over_mass = v.map(|x| core_types::M2kgRequired::new(x));
    }

    /// The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// Units: m²/kg
    ///
    /// :type: float
    #[getter]
    fn get_cr_area_over_mass(&self) -> Option<f64> {
        self.inner.cr_area_over_mass.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_cr_area_over_mass(&mut self, v: Option<f64>) {
        self.inner.cr_area_over_mass = v.map(|x| core_types::M2kgRequired::new(x));
    }

    /// The object's acceleration due to in-track thrust used to propagate the state vector and
    /// covariance to TCA. (See annex E for definition.)
    ///
    /// Units: m/s²
    ///
    /// :type: float
    #[getter]
    fn get_thrust_acceleration(&self) -> Option<f64> {
        self.inner.thrust_acceleration.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_thrust_acceleration(&mut self, v: Option<f64>) {
        self.inner.thrust_acceleration = v.map(|x| core_types::Ms2::new(x));
    }

    /// The amount of energy being removed from the object's orbit by atmospheric drag. This
    /// value is an average calculated during the OD.
    ///
    /// Units: W/kg
    ///
    /// :type: float
    #[getter]
    fn get_sedr(&self) -> Option<f64> {
        self.inner.sedr.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_sedr(&mut self, v: Option<f64>) {
        self.inner.sedr = v.map(|x| core_types::Wkg::new(x));
    }
}

/// Covariance Matrix.
///
/// Parameters
/// ----------
/// cr_r : float
///     Radial position variance. Units: m^2
/// ct_r : float
///     Transverse-Radial position covariance. Units: m^2
/// ct_t : float
///     Transverse position variance. Units: m^2
/// cn_r : float
///     Normal-Radial position covariance. Units: m^2
/// cn_t : float
///     Normal-Transverse position covariance. Units: m^2
/// cn_n : float
///     Normal position variance. Units: m^2
/// crdot_r : float
///     Radial velocity - Radial position covariance. Units: m^2/s
/// crdot_t : float
///     Radial velocity - Transverse position covariance. Units: m^2/s
/// crdot_n : float
///     Radial velocity - Normal position covariance. Units: m^2/s
/// crdot_rdot : float
///     Radial velocity variance. Units: m^2/s^2
/// ctdot_r : float
///     Transverse velocity - Radial position covariance. Units: m^2/s
/// ctdot_t : float
///     Transverse velocity - Transverse position covariance. Units: m^2/s
/// ctdot_n : float
///     Transverse velocity - Normal position covariance. Units: m^2/s
/// ctdot_rdot : float
///     Transverse velocity - Radial velocity covariance. Units: m^2/s^2
/// ctdot_tdot : float
///     Transverse velocity variance. Units: m^2/s^2
/// cndot_r : float
///     Normal velocity - Radial position covariance. Units: m^2/s
/// cndot_t : float
///     Normal velocity - Transverse position covariance. Units: m^2/s
/// cndot_n : float
///     Normal velocity - Normal position covariance. Units: m^2/s
/// cndot_rdot : float
///     Normal velocity - Radial velocity covariance. Units: m^2/s^2
/// cndot_tdot : float
///     Normal velocity - Transverse velocity covariance. Units: m^2/s^2
/// cndot_ndot : float
///     Normal velocity variance. Units: m^2/s^2
/// cdrg_r : float
///     Drag coeff - Radial position covariance.
/// cdrg_t : float
///     Drag coeff - Transverse position covariance.
/// cdrg_n : float
///     Drag coeff - Normal position covariance.
/// cdrg_rdot : float
///     Drag coeff - Radial velocity covariance.
/// cdrg_tdot : float
///     Drag coeff - Transverse velocity covariance.
/// cdrg_ndot : float
///     Drag coeff - Normal velocity covariance.
/// cdrg_drg : float
///     Drag coeff variance.
/// csrp_r : float
///     SRP coeff - Radial position covariance.
/// csrp_t : float
///     SRP coeff - Transverse position covariance.
/// csrp_n : float
///     SRP coeff - Normal position covariance.
/// csrp_rdot : float
///     SRP coeff - Radial velocity covariance.
/// csrp_tdot : float
///     SRP coeff - Transverse velocity covariance.
/// csrp_ndot : float
///     SRP coeff - Normal velocity covariance.
/// csrp_drg : float
///     SRP coeff - Drag coeff covariance.
/// csrp_srp : float
///     SRP coeff variance.
/// cthr_r : float
///     Thrust - Radial position covariance.
/// cthr_t : float
///     Thrust - Transverse position covariance.
/// cthr_n : float
///     Thrust - Normal position covariance.
/// cthr_rdot : float
///     Thrust - Radial velocity covariance.
/// cthr_tdot : float
///     Thrust - Transverse velocity covariance.
/// cthr_ndot : float
///     Thrust - Normal velocity covariance.
/// cthr_drg : float
///     Thrust - Drag coeff covariance.
/// cthr_srp : float
///     Thrust - SRP coeff covariance.
/// cthr_thr : float
///     Thrust variance.
/// comment : list of str, optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct CdmCovarianceMatrix {
    pub inner: core_cdm::CdmCovarianceMatrix,
}

#[pymethods]
impl CdmCovarianceMatrix {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        cr_r: f64,
        ct_r: f64,
        ct_t: f64,
        cn_r: f64,
        cn_t: f64,
        cn_n: f64,
        crdot_r: f64,
        crdot_t: f64,
        crdot_n: f64,
        crdot_rdot: f64,
        ctdot_r: f64,
        ctdot_t: f64,
        ctdot_n: f64,
        ctdot_rdot: f64,
        ctdot_tdot: f64,
        cndot_r: f64,
        cndot_t: f64,
        cndot_n: f64,
        cndot_rdot: f64,
        cndot_tdot: f64,
        cndot_ndot: f64,
        cdrg_r: Option<f64>,
        cdrg_t: Option<f64>,
        cdrg_n: Option<f64>,
        cdrg_rdot: Option<f64>,
        cdrg_tdot: Option<f64>,
        cdrg_ndot: Option<f64>,
        cdrg_drg: Option<f64>,
        csrp_r: Option<f64>,
        csrp_t: Option<f64>,
        csrp_n: Option<f64>,
        csrp_rdot: Option<f64>,
        csrp_tdot: Option<f64>,
        csrp_ndot: Option<f64>,
        csrp_drg: Option<f64>,
        csrp_srp: Option<f64>,
        cthr_r: Option<f64>,
        cthr_t: Option<f64>,
        cthr_n: Option<f64>,
        cthr_rdot: Option<f64>,
        cthr_tdot: Option<f64>,
        cthr_ndot: Option<f64>,
        cthr_drg: Option<f64>,
        cthr_srp: Option<f64>,
        cthr_thr: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_cdm::CdmCovarianceMatrix {
                comment: comment.unwrap_or_default(),
                cr_r: core_types::M2::new(cr_r),
                ct_r: core_types::M2::new(ct_r),
                ct_t: core_types::M2::new(ct_t),
                cn_r: core_types::M2::new(cn_r),
                cn_t: core_types::M2::new(cn_t),
                cn_n: core_types::M2::new(cn_n),
                crdot_r: core_types::M2s::new(crdot_r),
                crdot_t: core_types::M2s::new(crdot_t),
                crdot_n: core_types::M2s::new(crdot_n),
                crdot_rdot: core_types::M2s2::new(crdot_rdot),
                ctdot_r: core_types::M2s::new(ctdot_r),
                ctdot_t: core_types::M2s::new(ctdot_t),
                ctdot_n: core_types::M2s::new(ctdot_n),
                ctdot_rdot: core_types::M2s2::new(ctdot_rdot),
                ctdot_tdot: core_types::M2s2::new(ctdot_tdot),
                cndot_r: core_types::M2s::new(cndot_r),
                cndot_t: core_types::M2s::new(cndot_t),
                cndot_n: core_types::M2s::new(cndot_n),
                cndot_rdot: core_types::M2s2::new(cndot_rdot),
                cndot_tdot: core_types::M2s2::new(cndot_tdot),
                cndot_ndot: core_types::M2s2::new(cndot_ndot),
                cdrg_r: cdrg_r.map(|v| core_types::M3kg::new(v)),
                cdrg_t: cdrg_t.map(|v| core_types::M3kg::new(v)),
                cdrg_n: cdrg_n.map(|v| core_types::M3kg::new(v)),
                cdrg_rdot: cdrg_rdot.map(|v| core_types::M3kgs::new(v)),
                cdrg_tdot: cdrg_tdot.map(|v| core_types::M3kgs::new(v)),
                cdrg_ndot: cdrg_ndot.map(|v| core_types::M3kgs::new(v)),
                cdrg_drg: cdrg_drg.map(|v| core_types::M4kg2::new(v)),
                csrp_r: csrp_r.map(|v| core_types::M3kg::new(v)),
                csrp_t: csrp_t.map(|v| core_types::M3kg::new(v)),
                csrp_n: csrp_n.map(|v| core_types::M3kg::new(v)),
                csrp_rdot: csrp_rdot.map(|v| core_types::M3kgs::new(v)),
                csrp_tdot: csrp_tdot.map(|v| core_types::M3kgs::new(v)),
                csrp_ndot: csrp_ndot.map(|v| core_types::M3kgs::new(v)),
                csrp_drg: csrp_drg.map(|v| core_types::M4kg2::new(v)),
                csrp_srp: csrp_srp.map(|v| core_types::M4kg2::new(v)),
                cthr_r: cthr_r.map(|v| core_types::M2s2::new(v)),
                cthr_t: cthr_t.map(|v| core_types::M2s2::new(v)),
                cthr_n: cthr_n.map(|v| core_types::M2s2::new(v)),
                cthr_rdot: cthr_rdot.map(|v| core_types::M2s3::new(v)),
                cthr_tdot: cthr_tdot.map(|v| core_types::M2s3::new(v)),
                cthr_ndot: cthr_ndot.map(|v| core_types::M2s3::new(v)),
                cthr_drg: cthr_drg.map(|v| core_types::M3kgs2::new(v)),
                cthr_srp: cthr_srp.map(|v| core_types::M3kgs2::new(v)),
                cthr_thr: cthr_thr.map(|v| core_types::M2s4::new(v)),
            },

        }
    }

    /// Comments.
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

    /// Returns the full 9x9 covariance matrix as a NumPy array.
    /// If the optional 7,8,9 rows (Drag, SRP, Thrust) are missing, they are filled with 0.0.
    fn to_numpy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        let mut array = vec![0.0; 81]; // 9x9 flattened
        let c = &self.inner;

        // Helper to set symmetric values
        let mut set = |r: usize, c: usize, val: f64| {
            array[r * 9 + c] = val;
            array[c * 9 + r] = val;
        };

        // 1-6 Basic State (Diagonal and Lower Tri provided in struct)
        set(0, 0, c.cr_r.value);
        set(1, 0, c.ct_r.value);
        set(1, 1, c.ct_t.value);
        set(2, 0, c.cn_r.value);
        set(2, 1, c.cn_t.value);
        set(2, 2, c.cn_n.value);

        set(3, 0, c.crdot_r.value);
        set(3, 1, c.crdot_t.value);
        set(3, 2, c.crdot_n.value);
        set(3, 3, c.crdot_rdot.value);
        set(4, 0, c.ctdot_r.value);
        set(4, 1, c.ctdot_t.value);
        set(4, 2, c.ctdot_n.value);
        set(4, 3, c.ctdot_rdot.value);
        set(4, 4, c.ctdot_tdot.value);
        set(5, 0, c.cndot_r.value);
        set(5, 1, c.cndot_t.value);
        set(5, 2, c.cndot_n.value);
        set(5, 3, c.cndot_rdot.value);
        set(5, 4, c.cndot_tdot.value);
        set(5, 5, c.cndot_ndot.value);

        // Optional Rows (7, 8, 9)
        // Row 7: Drag
        if let (Some(r), Some(t), Some(n), Some(rd), Some(td), Some(nd), Some(drg)) = (
            &c.cdrg_r,
            &c.cdrg_t,
            &c.cdrg_n,
            &c.cdrg_rdot,
            &c.cdrg_tdot,
            &c.cdrg_ndot,
            &c.cdrg_drg,
        ) {
            set(6, 0, r.value);
            set(6, 1, t.value);
            set(6, 2, n.value);
            set(6, 3, rd.value);
            set(6, 4, td.value);
            set(6, 5, nd.value);
            set(6, 6, drg.value);
        }

        // Row 8: SRP
        if let (Some(r), Some(t), Some(n), Some(rd), Some(td), Some(nd), Some(drg), Some(srp)) = (
            &c.csrp_r,
            &c.csrp_t,
            &c.csrp_n,
            &c.csrp_rdot,
            &c.csrp_tdot,
            &c.csrp_ndot,
            &c.csrp_drg,
            &c.csrp_srp,
        ) {
            set(7, 0, r.value);
            set(7, 1, t.value);
            set(7, 2, n.value);
            set(7, 3, rd.value);
            set(7, 4, td.value);
            set(7, 5, nd.value);
            set(7, 6, drg.value);
            set(7, 7, srp.value);
        }

        // Row 9: Thrust
        if let (
            Some(r),
            Some(t),
            Some(n),
            Some(rd),
            Some(td),
            Some(nd),
            Some(drg),
            Some(srp),
            Some(thr),
        ) = (
            &c.cthr_r,
            &c.cthr_t,
            &c.cthr_n,
            &c.cthr_rdot,
            &c.cthr_tdot,
            &c.cthr_ndot,
            &c.cthr_drg,
            &c.cthr_srp,
            &c.cthr_thr,
        ) {
            set(8, 0, r.value);
            set(8, 1, t.value);
            set(8, 2, n.value);
            set(8, 3, rd.value);
            set(8, 4, td.value);
            set(8, 5, nd.value);
            set(8, 6, drg.value);
            set(8, 7, srp.value);
            set(8, 8, thr.value);
        }

        // Return 9x9 array
        let numpy_arr =
            PyArray2::from_vec2(py, &array.chunks(9).map(|c| c.to_vec()).collect::<Vec<_>>())
                .unwrap();
        Ok(numpy_arr)
    }

    /// Object covariance matrix `[1,1]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_cr_r(&self) -> f64 { self.inner.cr_r.value }
    #[setter]
    fn set_cr_r(&mut self, v: f64) { self.inner.cr_r.value = v; }

    /// Object covariance matrix `[2,1]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_ct_r(&self) -> f64 { self.inner.ct_r.value }
    #[setter]
    fn set_ct_r(&mut self, v: f64) { self.inner.ct_r.value = v; }

    /// Object covariance matrix `[2,2]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_ct_t(&self) -> f64 { self.inner.ct_t.value }
    #[setter]
    fn set_ct_t(&mut self, v: f64) { self.inner.ct_t.value = v; }

    /// Object covariance matrix `[3,1]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_cn_r(&self) -> f64 { self.inner.cn_r.value }
    #[setter]
    fn set_cn_r(&mut self, v: f64) { self.inner.cn_r.value = v; }

    /// Object covariance matrix `[3,2]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_cn_t(&self) -> f64 { self.inner.cn_t.value }
    #[setter]
    fn set_cn_t(&mut self, v: f64) { self.inner.cn_t.value = v; }

    /// Object covariance matrix `[3,3]`.
    ///
    /// Units: m²
    ///
    /// :type: float
    #[getter]
    fn get_cn_n(&self) -> f64 { self.inner.cn_n.value }
    #[setter]
    fn set_cn_n(&mut self, v: f64) { self.inner.cn_n.value = v; }

    /// Object covariance matrix `[4,1]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_crdot_r(&self) -> f64 { self.inner.crdot_r.value }
    #[setter]
    fn set_crdot_r(&mut self, v: f64) { self.inner.crdot_r.value = v; }

    /// Object covariance matrix `[4,2]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_crdot_t(&self) -> f64 { self.inner.crdot_t.value }
    #[setter]
    fn set_crdot_t(&mut self, v: f64) { self.inner.crdot_t.value = v; }

    /// Object covariance matrix `[4,3]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_crdot_n(&self) -> f64 { self.inner.crdot_n.value }
    #[setter]
    fn set_crdot_n(&mut self, v: f64) { self.inner.crdot_n.value = v; }

    /// Object covariance matrix `[4,4]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_crdot_rdot(&self) -> f64 { self.inner.crdot_rdot.value }
    #[setter]
    fn set_crdot_rdot(&mut self, v: f64) { self.inner.crdot_rdot.value = v; }

    /// Object covariance matrix `[5,1]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_ctdot_r(&self) -> f64 { self.inner.ctdot_r.value }
    #[setter]
    fn set_ctdot_r(&mut self, v: f64) { self.inner.ctdot_r.value = v; }

    /// Object covariance matrix `[5,2]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_ctdot_t(&self) -> f64 { self.inner.ctdot_t.value }
    #[setter]
    fn set_ctdot_t(&mut self, v: f64) { self.inner.ctdot_t.value = v; }

    /// Object covariance matrix `[5,3]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_ctdot_n(&self) -> f64 { self.inner.ctdot_n.value }
    #[setter]
    fn set_ctdot_n(&mut self, v: f64) { self.inner.ctdot_n.value = v; }

    /// Object covariance matrix `[5,4]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_ctdot_rdot(&self) -> f64 { self.inner.ctdot_rdot.value }
    #[setter]
    fn set_ctdot_rdot(&mut self, v: f64) { self.inner.ctdot_rdot.value = v; }

    /// Object covariance matrix `[5,5]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_ctdot_tdot(&self) -> f64 { self.inner.ctdot_tdot.value }
    #[setter]
    fn set_ctdot_tdot(&mut self, v: f64) { self.inner.ctdot_tdot.value = v; }

    /// Object covariance matrix `[6,1]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_cndot_r(&self) -> f64 { self.inner.cndot_r.value }
    #[setter]
    fn set_cndot_r(&mut self, v: f64) { self.inner.cndot_r.value = v; }

    /// Object covariance matrix `[6,2]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_cndot_t(&self) -> f64 { self.inner.cndot_t.value }
    #[setter]
    fn set_cndot_t(&mut self, v: f64) { self.inner.cndot_t.value = v; }

    /// Object covariance matrix `[6,3]`.
    ///
    /// Units: m²/s
    ///
    /// :type: float
    #[getter]
    fn get_cndot_n(&self) -> f64 { self.inner.cndot_n.value }
    #[setter]
    fn set_cndot_n(&mut self, v: f64) { self.inner.cndot_n.value = v; }

    /// Object covariance matrix `[6,4]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cndot_rdot(&self) -> f64 { self.inner.cndot_rdot.value }
    #[setter]
    fn set_cndot_rdot(&mut self, v: f64) { self.inner.cndot_rdot.value = v; }

    /// Object covariance matrix `[6,5]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cndot_tdot(&self) -> f64 { self.inner.cndot_tdot.value }
    #[setter]
    fn set_cndot_tdot(&mut self, v: f64) { self.inner.cndot_tdot.value = v; }

    /// Object covariance matrix `[6,6]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cndot_ndot(&self) -> f64 { self.inner.cndot_ndot.value }
    #[setter]
    fn set_cndot_ndot(&mut self, v: f64) { self.inner.cndot_ndot.value = v; }

    /// Object covariance matrix `[7,1]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_r(&self) -> Option<f64> { self.inner.cdrg_r.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_r(&mut self, v: Option<f64>) { self.inner.cdrg_r = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[7,2]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_t(&self) -> Option<f64> { self.inner.cdrg_t.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_t(&mut self, v: Option<f64>) { self.inner.cdrg_t = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[7,3]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_n(&self) -> Option<f64> { self.inner.cdrg_n.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_n(&mut self, v: Option<f64>) { self.inner.cdrg_n = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[7,4]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_rdot(&self) -> Option<f64> { self.inner.cdrg_rdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_rdot(&mut self, v: Option<f64>) { self.inner.cdrg_rdot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[7,5]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_tdot(&self) -> Option<f64> { self.inner.cdrg_tdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_tdot(&mut self, v: Option<f64>) { self.inner.cdrg_tdot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[7,6]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_ndot(&self) -> Option<f64> { self.inner.cdrg_ndot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_ndot(&mut self, v: Option<f64>) { self.inner.cdrg_ndot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[7,7]`.
    ///
    /// Units: m⁴/kg²
    ///
    /// :type: float
    #[getter]
    fn get_cdrg_drg(&self) -> Option<f64> { self.inner.cdrg_drg.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cdrg_drg(&mut self, v: Option<f64>) { self.inner.cdrg_drg = v.map(|x| core_types::M4kg2::new(x)); }

    /// Object covariance matrix `[8,1]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_csrp_r(&self) -> Option<f64> { self.inner.csrp_r.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_r(&mut self, v: Option<f64>) { self.inner.csrp_r = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[8,2]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_csrp_t(&self) -> Option<f64> { self.inner.csrp_t.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_t(&mut self, v: Option<f64>) { self.inner.csrp_t = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[8,3]`.
    ///
    /// Units: m³/kg
    ///
    /// :type: float
    #[getter]
    fn get_csrp_n(&self) -> Option<f64> { self.inner.csrp_n.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_n(&mut self, v: Option<f64>) { self.inner.csrp_n = v.map(|x| core_types::M3kg::new(x)); }

    /// Object covariance matrix `[8,4]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_csrp_rdot(&self) -> Option<f64> { self.inner.csrp_rdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_rdot(&mut self, v: Option<f64>) { self.inner.csrp_rdot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[8,5]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_csrp_tdot(&self) -> Option<f64> { self.inner.csrp_tdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_tdot(&mut self, v: Option<f64>) { self.inner.csrp_tdot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[8,6]`.
    ///
    /// Units: m³/(kg*s)
    ///
    /// :type: float
    #[getter]
    fn get_csrp_ndot(&self) -> Option<f64> { self.inner.csrp_ndot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_ndot(&mut self, v: Option<f64>) { self.inner.csrp_ndot = v.map(|x| core_types::M3kgs::new(x)); }

    /// Object covariance matrix `[8,7]`.
    ///
    /// Units: m⁴/kg²
    ///
    /// :type: float
    #[getter]
    fn get_csrp_drg(&self) -> Option<f64> { self.inner.csrp_drg.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_drg(&mut self, v: Option<f64>) { self.inner.csrp_drg = v.map(|x| core_types::M4kg2::new(x)); }

    /// Object covariance matrix `[8,8]`.
    ///
    /// Units: m⁴/kg²
    ///
    /// :type: float
    #[getter]
    fn get_csrp_srp(&self) -> Option<f64> { self.inner.csrp_srp.as_ref().map(|v| v.value) }
    #[setter]
    fn set_csrp_srp(&mut self, v: Option<f64>) { self.inner.csrp_srp = v.map(|x| core_types::M4kg2::new(x)); }

    /// Object covariance matrix `[9,1]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cthr_r(&self) -> Option<f64> { self.inner.cthr_r.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_r(&mut self, v: Option<f64>) { self.inner.cthr_r = v.map(|x| core_types::M2s2::new(x)); }

    /// Object covariance matrix `[9,2]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cthr_t(&self) -> Option<f64> { self.inner.cthr_t.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_t(&mut self, v: Option<f64>) { self.inner.cthr_t = v.map(|x| core_types::M2s2::new(x)); }

    /// Object covariance matrix `[9,3]`.
    ///
    /// Units: m²/s²
    ///
    /// :type: float
    #[getter]
    fn get_cthr_n(&self) -> Option<f64> { self.inner.cthr_n.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_n(&mut self, v: Option<f64>) { self.inner.cthr_n = v.map(|x| core_types::M2s2::new(x)); }

    /// Object covariance matrix `[9,4]`.
    ///
    /// Units: m²/s³
    ///
    /// :type: float
    #[getter]
    fn get_cthr_rdot(&self) -> Option<f64> { self.inner.cthr_rdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_rdot(&mut self, v: Option<f64>) { self.inner.cthr_rdot = v.map(|x| core_types::M2s3::new(x)); }

    /// Object covariance matrix `[9,5]`.
    ///
    /// Units: m²/s³
    ///
    /// :type: float
    #[getter]
    fn get_cthr_tdot(&self) -> Option<f64> { self.inner.cthr_tdot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_tdot(&mut self, v: Option<f64>) { self.inner.cthr_tdot = v.map(|x| core_types::M2s3::new(x)); }

    /// Object covariance matrix `[9,6]`.
    ///
    /// Units: m²/s³
    ///
    /// :type: float
    #[getter]
    fn get_cthr_ndot(&self) -> Option<f64> { self.inner.cthr_ndot.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_ndot(&mut self, v: Option<f64>) { self.inner.cthr_ndot = v.map(|x| core_types::M2s3::new(x)); }

    /// Object covariance matrix `[9,7]`.
    ///
    /// Units: m³/(kg*s²)
    ///
    /// :type: float
    #[getter]
    fn get_cthr_drg(&self) -> Option<f64> { self.inner.cthr_drg.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_drg(&mut self, v: Option<f64>) { self.inner.cthr_drg = v.map(|x| core_types::M3kgs2::new(x)); }

    /// Object covariance matrix `[9,8]`.
    ///
    /// Units: m³/(kg*s²)
    ///
    /// :type: float
    #[getter]
    fn get_cthr_srp(&self) -> Option<f64> { self.inner.cthr_srp.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_srp(&mut self, v: Option<f64>) { self.inner.cthr_srp = v.map(|x| core_types::M3kgs2::new(x)); }

    /// Object covariance matrix `[9,9]`.
    ///
    /// Units: m²/s⁴
    ///
    /// :type: float
    #[getter]
    fn get_cthr_thr(&self) -> Option<f64> { self.inner.cthr_thr.as_ref().map(|v| v.value) }
    #[setter]
    fn set_cthr_thr(&mut self, v: Option<f64>) { self.inner.cthr_thr = v.map(|x| core_types::M2s4::new(x)); }


}
