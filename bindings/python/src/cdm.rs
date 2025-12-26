// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::cdm as core_cdm;
use ccsds_ndm::traits::Ndm;
use std::fs;
use pyo3::exceptions::PyValueError;
use ccsds_ndm::MessageType;
use ccsds_ndm::types::{self as core_types, *};
use numpy::PyArray2;
use pyo3::prelude::*;

// Helper to parse epoch strings
fn parse_epoch_str(value: &str) -> PyResult<Epoch> {
    value.parse().map_err(|e: EpochError| PyErr::new::<PyValueError, _>(e.to_string()))
}

// Helper to handle unit setting
// If `provided_unit` is Some, it checks compliance with `T::default()`.
fn validate_unit<T: Default + std::fmt::Display + PartialEq>(provided: Option<String>) -> PyResult<()> {
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

/// Represents a CCSDS Conjunction Data Message (CDM).
///
/// The CDM specifies a standard message format for use in exchanging spacecraft
/// conjunction information between originators of Conjunction Assessments (CAs)
/// and satellite owner/operators and other authorized parties.
///
/// It contains information about a single conjunction between two objects,
/// including their positions/velocities, covariances at TCA, and relative
/// state data.
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
        let inner = match format {
            Some("kvn") => core_cdm::Cdm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => core_cdm::Cdm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => return Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Cdm(cdm)) => cdm,
                Ok(other) => return Err(PyValueError::new_err(format!(
                    "Parsed message is not CDM (got {:?})",
                    other
                ))),
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
            "kvn" => self.inner.to_kvn().map_err(|e| PyValueError::new_err(e.to_string())),
            "xml" => self.inner.to_xml().map_err(|e| PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
        }
    }

    

    #[getter]
    fn header(&self) -> CdmHeader {
        CdmHeader { inner: self.inner.header.clone() }
    }

    #[getter]
    fn body(&self) -> CdmBody {
        CdmBody { inner: self.inner.body.clone() }
    }

    #[getter]
    fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

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
            Err(e) => Err(PyValueError::new_err(format!("Failed to write file: {}", e))),
        }
    }

    
}

/// Header section of the CDM.
///
/// Contains metadata about the message itself, such as creation date,
/// originator, and unique identifiers.
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
        comment: Vec<String>
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

    /// Message creation date/time in UTC.
    ///
    /// :type: str
    #[getter]
    fn creation_date(&self) -> String { self.inner.creation_date.to_string() }
    #[setter]
    fn set_creation_date(&mut self, value: String) -> PyResult<()> {
        self.inner.creation_date = parse_epoch_str(&value)?;
        Ok(())
    }
    
    /// Creating agency or owner/operator.
    ///
    /// :type: str
    #[getter]
    fn originator(&self) -> String { self.inner.originator.clone() }
    #[setter]
    fn set_originator(&mut self, value: String) { self.inner.originator = value; }

    /// ID used to uniquely identify this message.
    ///
    /// :type: str
    #[getter]
    fn message_id(&self) -> String { self.inner.message_id.clone() }
    #[setter]
    fn set_message_id(&mut self, value: String) { self.inner.message_id = value; }

    /// Spacecraft name(s) for which the CDM is provided.
    ///
    /// :type: Optional[str]
    #[getter]
    fn message_for(&self) -> Option<String> { self.inner.message_for.clone() }
    #[setter]
    fn set_message_for(&mut self, value: Option<String>) { self.inner.message_for = value; }

    #[getter]
    fn comment(&self) -> Vec<String> { self.inner.comment.clone() }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) { self.inner.comment = value; }

    fn __repr__(&self) -> String {
        format!("CdmHeader(originator='{}', message_id='{}')",
            self.inner.originator, self.inner.message_id)
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
    fn new(relative_metadata_data: RelativeMetadataData, segments: Vec<CdmSegment>) -> PyResult<Self> {
        // CCSDS Spec implies exactly 2 segments usually, but we allow vector
        let inner_segs: Vec<core_cdm::CdmSegment> = segments.iter().map(|s| s.inner.clone()).collect();
        Ok(Self {
            inner: core_cdm::CdmBody {
                relative_metadata_data: relative_metadata_data.inner,
                segments: inner_segs
            }
        })
    }

    #[getter]
    fn relative_metadata_data(&self) -> RelativeMetadataData {
        RelativeMetadataData { inner: self.inner.relative_metadata_data.clone() }
    }

    #[getter]
    fn segments(&self) -> Vec<CdmSegment> {
        self.inner.segments.iter().map(|s| CdmSegment { inner: s.clone() }).collect()
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
/// screen_volume_frame : ScreenVolumeFrameType, optional
///     The reference frame for screening volume (RTN or TVN).
/// screen_volume_shape : ScreenVolumeShapeType, optional
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
        screen_volume_frame: Option<ScreenVolumeFrameType>,
        screen_volume_shape: Option<ScreenVolumeShapeType>,
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
                relative_velocity_r: Dv::new(v[0], None),
                relative_velocity_t: Dv::new(v[1], None),
                relative_velocity_n: Dv::new(v[2], None),
            })
        } else {
            None
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
                relative_speed: relative_speed.map(|v| Dv::new(v, None)),
                relative_state_vector: rel_state,
                start_screen_period: start_screen_period.map(|s| parse_epoch_str(&s)).transpose()?,
                stop_screen_period: stop_screen_period.map(|s| parse_epoch_str(&s)).transpose()?,
                screen_volume_frame: screen_volume_frame.map(map_frame),
                screen_volume_shape: screen_volume_shape.map(map_shape),
                screen_volume_x: screen_volume_x.map(|v| Length::new(v, None)),
                screen_volume_y: screen_volume_y.map(|v| Length::new(v, None)),
                screen_volume_z: screen_volume_z.map(|v| Length::new(v, None)),
                screen_entry_time: screen_entry_time.map(|s| parse_epoch_str(&s)).transpose()?,
                screen_exit_time: screen_exit_time.map(|s| parse_epoch_str(&s)).transpose()?,
                collision_probability: collision_probability.map(|v| Probability::new(v)).transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                collision_probability_method,
            }
        })
    }

    #[getter]
    fn tca(&self) -> String { self.inner.tca.to_string() }
    #[setter]
    fn set_tca(&mut self, value: String) -> PyResult<()> {
        self.inner.tca = parse_epoch_str(&value)?;
        Ok(())
    }

    #[getter]
    fn miss_distance(&self) -> f64 { self.inner.miss_distance.value }
    #[setter]
    fn set_miss_distance(&mut self, value: f64) {
        self.inner.miss_distance = Length::new(value, None);
    }
    #[getter]
    fn miss_distance_units(&self) -> String { "m".to_string() }
    
    #[getter]
    fn relative_speed(&self) -> Option<f64> { self.inner.relative_speed.as_ref().map(|v| v.value) }
    #[setter]
    fn set_relative_speed(&mut self, value: Option<f64>) {
        self.inner.relative_speed = value.map(|v| Dv::new(v, None));
    }
    #[getter]
    fn relative_speed_units(&self) -> String { "m/s".to_string() }

    #[getter]
    fn collision_probability(&self) -> Option<f64> { self.inner.collision_probability.as_ref().map(|v| v.value) }
    #[setter]
    fn set_collision_probability(&mut self, value: Option<f64>) -> PyResult<()> {
        self.inner.collision_probability = value.map(|v| Probability::new(v)).transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn collision_probability_method(&self) -> Option<String> { self.inner.collision_probability_method.clone() }
    #[setter]
    fn set_collision_probability_method(&mut self, value: Option<String>) {
        self.inner.collision_probability_method = value;
    }

    #[getter]
    fn start_screen_period(&self) -> Option<String> { 
        self.inner.start_screen_period.as_ref().map(|e| e.to_string()) 
    }
    #[setter]
    fn set_start_screen_period(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.start_screen_period = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn stop_screen_period(&self) -> Option<String> { 
        self.inner.stop_screen_period.as_ref().map(|e| e.to_string()) 
    }
    #[setter]
    fn set_stop_screen_period(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.stop_screen_period = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn screen_entry_time(&self) -> Option<String> { 
        self.inner.screen_entry_time.as_ref().map(|e| e.to_string()) 
    }
    #[setter]
    fn set_screen_entry_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.screen_entry_time = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn screen_exit_time(&self) -> Option<String> { 
        self.inner.screen_exit_time.as_ref().map(|e| e.to_string()) 
    }
    #[setter]
    fn set_screen_exit_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.screen_exit_time = value.map(|s| parse_epoch_str(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn screen_volume_x(&self) -> Option<f64> { self.inner.screen_volume_x.as_ref().map(|v| v.value) }
    #[getter]
    fn screen_volume_y(&self) -> Option<f64> { self.inner.screen_volume_y.as_ref().map(|v| v.value) }
    #[getter]
    fn screen_volume_z(&self) -> Option<f64> { self.inner.screen_volume_z.as_ref().map(|v| v.value) }

    #[getter]
    fn comment(&self) -> Vec<String> { self.inner.comment.clone() }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) { self.inner.comment = value; }

    fn __repr__(&self) -> String {
        format!("RelativeMetadataData(tca='{}', miss_distance={}, collision_probability={:?})",
            self.inner.tca, self.inner.miss_distance.value, 
            self.inner.collision_probability.as_ref().map(|p| p.value))
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
            }
        }
    }

    #[getter]
    fn metadata(&self) -> CdmMetadata { CdmMetadata { inner: self.inner.metadata.clone() } }

    #[getter]
    fn data(&self) -> CdmData { CdmData { inner: self.inner.data.clone() } }

    fn __repr__(&self) -> String {
        format!("CdmSegment(object_name='{}')", self.inner.metadata.object_name)
    }
}

/// Metadata Section for an object in a CDM.
///
/// Contains identification, contact, and modeling information for either
/// Object1 or Object2.
///
/// Parameters
/// ----------
/// object : CdmObjectType
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
/// covariance_method : CovarianceMethodType
///     Method used to calculate the covariance (CALCULATED or DEFAULT).
/// maneuverable : ManeuverableType
///     The maneuver capacity of the object (YES, NO, or NA).
/// ref_frame : ReferenceFrameType
///     Reference frame for state vector data (GCRF, EME2000, or ITRF).
/// object_type : ObjectDescription, optional
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
        ephemeris_name,
        covariance_method,
        maneuverable,
        ref_frame,
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
        object: CdmObjectType,
        object_designator: String,
        catalog_name: String,
        object_name: String,
        international_designator: String,
        ephemeris_name: String,
        covariance_method: CovarianceMethodType,
        maneuverable: ManeuverableType,
        ref_frame: ReferenceFrameType,
        object_type: Option<ObjectDescription>,
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
    ) -> Self {
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
        let map_bool_to_yn = |b: bool| if b { core_types::YesNo::Yes } else { core_types::YesNo::No };

        let map_desc = |d: ObjectDescription| match d {
            ObjectDescription::Payload => core_types::ObjectDescription::Payload,
            ObjectDescription::RocketBody => core_types::ObjectDescription::RocketBody,
            ObjectDescription::Debris => core_types::ObjectDescription::Debris,
            ObjectDescription::Unknown => core_types::ObjectDescription::Unknown,
            ObjectDescription::Other => core_types::ObjectDescription::Other,
        };

        Self {
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
                object_type: object_type.map(map_desc),
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
            }
        }
    }

    #[getter]
    fn object_name(&self) -> String { self.inner.object_name.clone() }
    #[setter]
    fn set_object_name(&mut self, value: String) { self.inner.object_name = value; }

    #[getter]
    fn object_designator(&self) -> String { self.inner.object_designator.clone() }
    #[setter]
    fn set_object_designator(&mut self, value: String) { self.inner.object_designator = value; }

    #[getter]
    fn catalog_name(&self) -> String { self.inner.catalog_name.clone() }
    #[setter]
    fn set_catalog_name(&mut self, value: String) { self.inner.catalog_name = value; }

    #[getter]
    fn international_designator(&self) -> String { self.inner.international_designator.clone() }
    #[setter]
    fn set_international_designator(&mut self, value: String) { self.inner.international_designator = value; }

    #[getter]
    fn ephemeris_name(&self) -> String { self.inner.ephemeris_name.clone() }
    #[setter]
    fn set_ephemeris_name(&mut self, value: String) { self.inner.ephemeris_name = value; }

    #[getter]
    fn operator_contact_position(&self) -> Option<String> { self.inner.operator_contact_position.clone() }
    #[setter]
    fn set_operator_contact_position(&mut self, value: Option<String>) { self.inner.operator_contact_position = value; }

    #[getter]
    fn operator_organization(&self) -> Option<String> { self.inner.operator_organization.clone() }
    #[setter]
    fn set_operator_organization(&mut self, value: Option<String>) { self.inner.operator_organization = value; }

    #[getter]
    fn operator_phone(&self) -> Option<String> { self.inner.operator_phone.clone() }
    #[setter]
    fn set_operator_phone(&mut self, value: Option<String>) { self.inner.operator_phone = value; }

    #[getter]
    fn operator_email(&self) -> Option<String> { self.inner.operator_email.clone() }
    #[setter]
    fn set_operator_email(&mut self, value: Option<String>) { self.inner.operator_email = value; }

    #[getter]
    fn orbit_center(&self) -> Option<String> { self.inner.orbit_center.clone() }
    #[setter]
    fn set_orbit_center(&mut self, value: Option<String>) { self.inner.orbit_center = value; }

    #[getter]
    fn gravity_model(&self) -> Option<String> { self.inner.gravity_model.clone() }
    #[setter]
    fn set_gravity_model(&mut self, value: Option<String>) { self.inner.gravity_model = value; }

    #[getter]
    fn atmospheric_model(&self) -> Option<String> { self.inner.atmospheric_model.clone() }
    #[setter]
    fn set_atmospheric_model(&mut self, value: Option<String>) { self.inner.atmospheric_model = value; }

    #[getter]
    fn n_body_perturbations(&self) -> Option<String> { self.inner.n_body_perturbations.clone() }
    #[setter]
    fn set_n_body_perturbations(&mut self, value: Option<String>) { self.inner.n_body_perturbations = value; }

    #[getter]
    fn comment(&self) -> Vec<String> { self.inner.comment.clone() }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) { self.inner.comment = value; }

    fn __repr__(&self) -> String {
        format!("CdmMetadata(object_name='{}', designator='{}')",
            self.inner.object_name, self.inner.object_designator)
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
    fn new(state_vector: CdmStateVector, covariance_matrix: CdmCovarianceMatrix) -> Self {
        Self {
            inner: core_cdm::CdmData {
                comment: vec![],
                od_parameters: None,
                additional_parameters: None,
                state_vector: state_vector.inner,
                covariance_matrix: covariance_matrix.inner,
            }
        }
    }

    #[getter]
    fn state_vector(&self) -> CdmStateVector { CdmStateVector { inner: self.inner.state_vector.clone() } }

    #[getter]
    fn covariance_matrix(&self) -> CdmCovarianceMatrix { CdmCovarianceMatrix { inner: self.inner.covariance_matrix.clone() } }

    #[getter]
    fn comment(&self) -> Vec<String> { self.inner.comment.clone() }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) { self.inner.comment = value; }

    fn __repr__(&self) -> String {
        format!("CdmData(position=[{:.3}, {:.3}, {:.3}] km)",
            self.inner.state_vector.x.value,
            self.inner.state_vector.y.value,
            self.inner.state_vector.z.value)
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
            }
        }
    }

    #[getter] fn x(&self) -> f64 { self.inner.x.value }
    #[setter] fn set_x(&mut self, value: f64) { self.inner.x = PositionRequired::new(value); }
    #[getter] fn x_units(&self) -> String { "km".to_string() }
    
    #[getter] fn y(&self) -> f64 { self.inner.y.value }
    #[setter] fn set_y(&mut self, value: f64) { self.inner.y = PositionRequired::new(value); }
    
    #[getter] fn z(&self) -> f64 { self.inner.z.value }
    #[setter] fn set_z(&mut self, value: f64) { self.inner.z = PositionRequired::new(value); }
    
    #[getter] fn x_dot(&self) -> f64 { self.inner.x_dot.value }
    #[setter] fn set_x_dot(&mut self, value: f64) { self.inner.x_dot = VelocityRequired::new(value); }
    #[getter] fn x_dot_units(&self) -> String { "km/s".to_string() }
    
    #[getter] fn y_dot(&self) -> f64 { self.inner.y_dot.value }
    #[setter] fn set_y_dot(&mut self, value: f64) { self.inner.y_dot = VelocityRequired::new(value); }
    
    #[getter] fn z_dot(&self) -> f64 { self.inner.z_dot.value }
    #[setter] fn set_z_dot(&mut self, value: f64) { self.inner.z_dot = VelocityRequired::new(value); }

    fn __repr__(&self) -> String {
        format!("CdmStateVector(x={:.3}, y={:.3}, z={:.3} km)",
            self.inner.x.value, self.inner.y.value, self.inner.z.value)
    }
}

/// Covariance Matrix at TCA.
/// 
/// Provides uncertainty information for the state vector.
/// Can be converted to a NumPy array using `to_numpy()`.
#[pyclass]
#[derive(Clone)]
pub struct CdmCovarianceMatrix {
    pub inner: core_cdm::CdmCovarianceMatrix,
}

#[pymethods]
impl CdmCovarianceMatrix {
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
        set(1, 0, c.ct_r.value); set(1, 1, c.ct_t.value);
        set(2, 0, c.cn_r.value); set(2, 1, c.cn_t.value); set(2, 2, c.cn_n.value);
        
        set(3, 0, c.crdot_r.value); set(3, 1, c.crdot_t.value); set(3, 2, c.crdot_n.value); set(3, 3, c.crdot_rdot.value);
        set(4, 0, c.ctdot_r.value); set(4, 1, c.ctdot_t.value); set(4, 2, c.ctdot_n.value); set(4, 3, c.ctdot_rdot.value); set(4, 4, c.ctdot_tdot.value);
        set(5, 0, c.cndot_r.value); set(5, 1, c.cndot_t.value); set(5, 2, c.cndot_n.value); set(5, 3, c.cndot_rdot.value); set(5, 4, c.cndot_tdot.value); set(5, 5, c.cndot_ndot.value);

        // Optional Rows (7, 8, 9)
        // Row 7: Drag
        if let (Some(r), Some(t), Some(n), Some(rd), Some(td), Some(nd), Some(drg)) = 
           (&c.cdrg_r, &c.cdrg_t, &c.cdrg_n, &c.cdrg_rdot, &c.cdrg_tdot, &c.cdrg_ndot, &c.cdrg_drg) {
            set(6, 0, r.value); set(6, 1, t.value); set(6, 2, n.value);
            set(6, 3, rd.value); set(6, 4, td.value); set(6, 5, nd.value);
            set(6, 6, drg.value);
        }

        // Row 8: SRP
        if let (Some(r), Some(t), Some(n), Some(rd), Some(td), Some(nd), Some(drg), Some(srp)) = 
           (&c.csrp_r, &c.csrp_t, &c.csrp_n, &c.csrp_rdot, &c.csrp_tdot, &c.csrp_ndot, &c.csrp_drg, &c.csrp_srp) {
            set(7, 0, r.value); set(7, 1, t.value); set(7, 2, n.value);
            set(7, 3, rd.value); set(7, 4, td.value); set(7, 5, nd.value);
            set(7, 6, drg.value); set(7, 7, srp.value);
        }

        // Row 9: Thrust
        if let (Some(r), Some(t), Some(n), Some(rd), Some(td), Some(nd), Some(drg), Some(srp), Some(thr)) = 
           (&c.cthr_r, &c.cthr_t, &c.cthr_n, &c.cthr_rdot, &c.cthr_tdot, &c.cthr_ndot, &c.cthr_drg, &c.cthr_srp, &c.cthr_thr) {
            set(8, 0, r.value); set(8, 1, t.value); set(8, 2, n.value);
            set(8, 3, rd.value); set(8, 4, td.value); set(8, 5, nd.value);
            set(8, 6, drg.value); set(8, 7, srp.value); set(8, 8, thr.value);
        }
        
        // Return 9x9 array
        let numpy_arr = PyArray2::from_vec2(py, &array.chunks(9).map(|c| c.to_vec()).collect::<Vec<_>>()).unwrap();
        Ok(numpy_arr)
    }
}

// -----------------------------------------------------------------------------------------
// Enums
// -----------------------------------------------------------------------------------------

#[pyclass]
#[derive(Clone)]
pub enum CdmObjectType {
    Object1,
    Object2,
}

#[pyclass]
#[derive(Clone)]
pub enum ScreenVolumeFrameType {
    Rtn,
    Tvn,
}

#[pyclass]
#[derive(Clone)]
pub enum ScreenVolumeShapeType {
    Ellipsoid,
    Box,
}

#[pyclass]
#[derive(Clone)]
pub enum ReferenceFrameType {
    Eme2000,
    Gcrf,
    Itrf,
}

#[pyclass]
#[derive(Clone)]
pub enum CovarianceMethodType {
    Calculated,
    Default,
}

#[pyclass]
#[derive(Clone)]
pub enum ManeuverableType {
    Yes,
    No,
    NA,
}

#[pyclass]
#[derive(Clone)]
pub enum ObjectDescription {
    Payload,
    RocketBody,
    Debris,
    Unknown,
    Other,
}