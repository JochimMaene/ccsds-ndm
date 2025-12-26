// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::parse_epoch;
use ccsds_ndm::common as core_common;
use ccsds_ndm::messages::rdm as core_rdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{
    ControlledType, DayInterval, DayIntervalRequired, ObjectDescription, Percentage, PercentageRequired,
    PositionRequired, YesNo,
};
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

// ============================================================================
// RDM - Re-entry Data Message
// ============================================================================

/// Represents a CCSDS Re-entry Data Message (RDM).
///
/// The RDM specifies a standard message format to be used in the exchange of
/// spacecraft re-entry information between Space Situational Awareness (SSA)
/// or Space Surveillance and Tracking (SST) data providers, satellite
/// owners/operators, and other parties.
///
/// Parameters
/// ----------
/// header : RdmHeader
///     The message header.
///     (Mandatory)
/// segment : RdmSegment
///     The message segment containing metadata and data.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct Rdm {
    pub inner: core_rdm::Rdm,
}

#[pymethods]
impl Rdm {
    #[new]
    #[pyo3(signature = (*, header, segment))]
    fn new(header: RdmHeader, segment: RdmSegment) -> Self {
        Self {
            inner: core_rdm::Rdm {
                header: header.inner,
                body: core_rdm::RdmBody {
                    segment: segment.inner,
                },
                id: Some("CCSDS_RDM_VERS".to_string()),
                version: "1.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Rdm(object_name='{}')",
            self.inner.body.segment.metadata.object_name
        )
    }

    /// The message header.
    ///
    /// :type: RdmHeader
    #[getter]
    fn get_header(&self) -> RdmHeader {
        RdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    #[setter]
    fn set_header(&mut self, header: RdmHeader) {
        self.inner.header = header.inner;
    }

    /// The message segment.
    ///
    /// :type: RdmSegment
    #[getter]
    fn get_segment(&self) -> RdmSegment {
        RdmSegment {
            inner: self.inner.body.segment.clone(),
        }
    }

    #[setter]
    fn set_segment(&mut self, segment: RdmSegment) {
        self.inner.body.segment = segment.inner;
    }

    /// Create an RDM message from a string.
    ///
    /// Parameters
    /// ----------
    /// data : str
    ///     Input string/content.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///     (Optional)
    ///
    /// Returns
    /// -------
    /// Rdm
    ///     The parsed RDM object.
    #[staticmethod]
    #[pyo3(signature = (data, format=None))]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => core_rdm::Rdm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => core_rdm::Rdm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Rdm(rdm)) => rdm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not RDM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    /// Create an RDM message from a file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Path to the input file.
    /// format : str, optional
    ///     Format ('kvn' or 'xml'). Auto-detected if None.
    ///     (Optional)
    ///
    /// Returns
    /// -------
    /// Rdm
    ///     The parsed RDM object.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Serialize to KVN string.
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized KVN string.
    fn to_kvn(&self) -> PyResult<String> {
        self.inner
            .to_kvn()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Serialize to XML string.
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized XML string.
    fn to_xml(&self) -> PyResult<String> {
        self.inner
            .to_xml()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Serialize to string (generic).
    ///
    /// Parameters
    /// ----------
    /// format : str
    ///     Format ('kvn' or 'xml').
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized string.
    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self.to_kvn(),
            "xml" => self.to_xml(),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
        }
    }

    /// Write to a file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Output file path.
    /// format : str
    ///     Format ('kvn' or 'xml').
    fn to_file(&self, path: &str, format: &str) -> PyResult<()> {
        let data = self.to_str(format)?;
        fs::write(path, data).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to write file: {}", e))
        })
    }
}

// ============================================================================
// RDM Header
// ============================================================================

/// Represents the Header of a Re-entry Data Message.
///
/// Parameters
/// ----------
/// originator : str
///     Creating agency or entity.
///     (Mandatory)
/// creation_date : str
///     File creation date and time in UTC.
///     (Mandatory)
/// message_id : str
///     ID that uniquely identifies a message from a given originator.
///     (Mandatory)
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct RdmHeader {
    pub inner: core_rdm::RdmHeader,
}

#[pymethods]
impl RdmHeader {
    #[new]
    #[pyo3(signature = (*, originator, creation_date, message_id, comment=None))]
    fn new(
        originator: String,
        creation_date: String,
        message_id: String,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_rdm::RdmHeader {
                comment: comment.unwrap_or_default(),
                creation_date: parse_epoch(&creation_date)?,
                originator,
                message_id,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("RdmHeader(originator='{}')", self.inner.originator)
    }

    /// Creating agency or entity.
    ///
    /// Value should be an entry from the SANA Organizations Registry.
    ///
    /// :type: str
    #[getter]
    fn get_originator(&self) -> String {
        self.inner.originator.clone()
    }
    #[setter]
    fn set_originator(&mut self, v: String) {
        self.inner.originator = v;
    }

    /// File creation date and time in UTC.
    ///
    /// Format: ISO 8601.
    ///
    /// :type: str
    #[getter]
    fn get_creation_date(&self) -> String {
        self.inner.creation_date.as_str().to_string()
    }
    #[setter]
    fn set_creation_date(&mut self, v: String) -> PyResult<()> {
        self.inner.creation_date = parse_epoch(&v)?;
        Ok(())
    }

    /// ID that uniquely identifies a message from a given originator.
    ///
    /// :type: str
    #[getter]
    fn get_message_id(&self) -> String {
        self.inner.message_id.clone()
    }
    #[setter]
    fn set_message_id(&mut self, v: String) {
        self.inner.message_id = v;
    }

    /// Comments.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// RDM Segment
// ============================================================================

/// Represents a single segment of an RDM.
///
/// An RDM segment consists of a Metadata Section and a Data Section.
///
/// Parameters
/// ----------
/// metadata : RdmMetadata
///     Segment metadata.
///     (Mandatory)
/// data : RdmData
///     Segment data.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct RdmSegment {
    pub inner: core_rdm::RdmSegment,
}

#[pymethods]
impl RdmSegment {
    #[new]
    #[pyo3(signature = (*, metadata, data))]
    fn new(metadata: RdmMetadata, data: RdmData) -> Self {
        Self {
            inner: core_rdm::RdmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "RdmSegment(object_name='{}')",
            self.inner.metadata.object_name
        )
    }

    /// Segment metadata.
    ///
    /// :type: RdmMetadata
    #[getter]
    fn get_metadata(&self) -> RdmMetadata {
        RdmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }
    #[setter]
    fn set_metadata(&mut self, v: RdmMetadata) {
        self.inner.metadata = v.inner;
    }

    /// Segment data.
    ///
    /// :type: RdmData
    #[getter]
    fn get_data(&self) -> RdmData {
        RdmData {
            inner: self.inner.data.clone(),
        }
    }
    #[setter]
    fn set_data(&mut self, v: RdmData) {
        self.inner.data = v.inner;
    }
}

// ============================================================================
// RDM Metadata
// ============================================================================

/// Represents the Metadata Section of an RDM Segment.
///
/// Contains configuration details and object identification for the re-entry data.
///
/// Mandatory Parameters
/// --------------------
/// object_name : str
///     Object name for which the orbit state is provided.
/// international_designator : str
///     The full international designator (COSPAR ID) for the object.
/// controlled_reentry : str
///     Specification of whether the re-entry is controlled or not (YES, NO, UNKNOWN).
/// center_name : str
///     Celestial body orbited by the object.
/// time_system : str
///     Time system for all data/metadata (e.g., UTC, TAI).
/// epoch_tzero : str
///     Epoch from which the ORBIT_LIFETIME is calculated.
///
/// Optional Parameters
/// -------------------
/// See the CCSDS RDM Blue Book for details on optional parameters.
#[pyclass]
#[derive(Clone)]
pub struct RdmMetadata {
    pub inner: core_rdm::RdmMetadata,
}

#[pymethods]
impl RdmMetadata {
    #[new]
    #[pyo3(signature = (
        *,
        object_name,
        international_designator,
        controlled_reentry,
        center_name,
        time_system,
        epoch_tzero,
        catalog_name=None,
        object_designator=None,
        object_type=None,
        object_owner=None,
        object_operator=None,
        ref_frame=None,
        ref_frame_epoch=None,
        ephemeris_name=None,
        gravity_model=None,
        atmospheric_model=None,
        solar_flux_prediction=None,
        n_body_perturbations=None,
        solar_rad_pressure=None,
        earth_tides=None,
        intrack_thrust=None,
        drag_parameters_source=None,
        drag_parameters_altitude=None,
        reentry_uncertainty_method=None,
        reentry_disintegration=None,
        impact_uncertainty_method=None,
        previous_message_id=None,
        previous_message_epoch=None,
        next_message_epoch=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        object_name: String,
        international_designator: String,
        controlled_reentry: String,
        center_name: String,
        time_system: String,
        epoch_tzero: String,
        catalog_name: Option<String>,
        object_designator: Option<String>,
        object_type: Option<String>,
        object_owner: Option<String>,
        object_operator: Option<String>,
        ref_frame: Option<String>,
        ref_frame_epoch: Option<String>,
        ephemeris_name: Option<String>,
        gravity_model: Option<String>,
        atmospheric_model: Option<String>,
        solar_flux_prediction: Option<String>,
        n_body_perturbations: Option<String>,
        solar_rad_pressure: Option<String>,
        earth_tides: Option<String>,
        intrack_thrust: Option<String>,
        drag_parameters_source: Option<String>,
        drag_parameters_altitude: Option<f64>,
        reentry_uncertainty_method: Option<String>,
        reentry_disintegration: Option<String>,
        impact_uncertainty_method: Option<String>,
        previous_message_id: Option<String>,
        previous_message_epoch: Option<String>,
        next_message_epoch: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use std::str::FromStr;

        let controlled_reentry_enum = ControlledType::from_str(&controlled_reentry)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let object_type_enum = match object_type {
            Some(s) => Some(
                ObjectDescription::from_str(&s)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            ),
            None => None,
        };

        let intrack_thrust_enum = match intrack_thrust {
            Some(s) => Some(YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))?),
            None => None,
        };

        Ok(Self {
            inner: core_rdm::RdmMetadata {
                comment: comment.unwrap_or_default(),
                object_name,
                international_designator,
                catalog_name,
                object_designator,
                object_type: object_type_enum,
                object_owner,
                object_operator,
                controlled_reentry: controlled_reentry_enum,
                center_name,
                time_system,
                epoch_tzero: parse_epoch(&epoch_tzero)?,
                ref_frame,
                ref_frame_epoch: ref_frame_epoch.map(|s| parse_epoch(&s)).transpose()?,
                ephemeris_name,
                gravity_model,
                atmospheric_model,
                solar_flux_prediction,
                n_body_perturbations,
                solar_rad_pressure,
                earth_tides,
                intrack_thrust: intrack_thrust_enum,
                drag_parameters_source,
                drag_parameters_altitude: drag_parameters_altitude
                    .map(PositionRequired::new),
                reentry_uncertainty_method,
                reentry_disintegration,
                impact_uncertainty_method,
                previous_message_id,
                previous_message_epoch: previous_message_epoch
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
                next_message_epoch: next_message_epoch
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("RdmMetadata(object_name='{}')", self.inner.object_name)
    }

    #[getter]
    fn get_object_name(&self) -> String {
        self.inner.object_name.clone()
    }
    #[setter]
    fn set_object_name(&mut self, v: String) {
        self.inner.object_name = v;
    }

    #[getter]
    fn get_international_designator(&self) -> String {
        self.inner.international_designator.clone()
    }
    #[setter]
    fn set_international_designator(&mut self, v: String) {
        self.inner.international_designator = v;
    }

    #[getter]
    fn get_catalog_name(&self) -> Option<String> {
        self.inner.catalog_name.clone()
    }
    #[setter]
    fn set_catalog_name(&mut self, v: Option<String>) {
        self.inner.catalog_name = v;
    }

    #[getter]
    fn get_object_designator(&self) -> Option<String> {
        self.inner.object_designator.clone()
    }
    #[setter]
    fn set_object_designator(&mut self, v: Option<String>) {
        self.inner.object_designator = v;
    }

    #[getter]
    fn get_object_type(&self) -> Option<String> {
        self.inner.object_type.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_object_type(&mut self, v: Option<String>) -> PyResult<()> {
        use std::str::FromStr;
        self.inner.object_type = match v {
            Some(s) => Some(
                ObjectDescription::from_str(&s)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            ),
            None => None,
        };
        Ok(())
    }

    #[getter]
    fn get_object_owner(&self) -> Option<String> {
        self.inner.object_owner.clone()
    }
    #[setter]
    fn set_object_owner(&mut self, v: Option<String>) {
        self.inner.object_owner = v;
    }

    #[getter]
    fn get_object_operator(&self) -> Option<String> {
        self.inner.object_operator.clone()
    }
    #[setter]
    fn set_object_operator(&mut self, v: Option<String>) {
        self.inner.object_operator = v;
    }

    #[getter]
    fn get_controlled_reentry(&self) -> String {
        self.inner.controlled_reentry.to_string()
    }
    #[setter]
    fn set_controlled_reentry(&mut self, v: String) -> PyResult<()> {
        use std::str::FromStr;
        self.inner.controlled_reentry =
            ControlledType::from_str(&v).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_center_name(&self) -> String {
        self.inner.center_name.clone()
    }
    #[setter]
    fn set_center_name(&mut self, v: String) {
        self.inner.center_name = v;
    }

    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }
    #[setter]
    fn set_time_system(&mut self, v: String) {
        self.inner.time_system = v;
    }

    #[getter]
    fn get_epoch_tzero(&self) -> String {
        self.inner.epoch_tzero.as_str().to_string()
    }
    #[setter]
    fn set_epoch_tzero(&mut self, v: String) -> PyResult<()> {
        self.inner.epoch_tzero = parse_epoch(&v)?;
        Ok(())
    }

    #[getter]
    fn get_ref_frame(&self) -> Option<String> {
        self.inner.ref_frame.clone()
    }
    #[setter]
    fn set_ref_frame(&mut self, v: Option<String>) {
        self.inner.ref_frame = v;
    }

    #[getter]
    fn get_ref_frame_epoch(&self) -> Option<String> {
        self.inner
            .ref_frame_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_ref_frame_epoch(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.ref_frame_epoch = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_ephemeris_name(&self) -> Option<String> {
        self.inner.ephemeris_name.clone()
    }
    #[setter]
    fn set_ephemeris_name(&mut self, v: Option<String>) {
        self.inner.ephemeris_name = v;
    }

    #[getter]
    fn get_gravity_model(&self) -> Option<String> {
        self.inner.gravity_model.clone()
    }
    #[setter]
    fn set_gravity_model(&mut self, v: Option<String>) {
        self.inner.gravity_model = v;
    }

    #[getter]
    fn get_atmospheric_model(&self) -> Option<String> {
        self.inner.atmospheric_model.clone()
    }
    #[setter]
    fn set_atmospheric_model(&mut self, v: Option<String>) {
        self.inner.atmospheric_model = v;
    }

    #[getter]
    fn get_solar_flux_prediction(&self) -> Option<String> {
        self.inner.solar_flux_prediction.clone()
    }
    #[setter]
    fn set_solar_flux_prediction(&mut self, v: Option<String>) {
        self.inner.solar_flux_prediction = v;
    }

    #[getter]
    fn get_n_body_perturbations(&self) -> Option<String> {
        self.inner.n_body_perturbations.clone()
    }
    #[setter]
    fn set_n_body_perturbations(&mut self, v: Option<String>) {
        self.inner.n_body_perturbations = v;
    }

    #[getter]
    fn get_solar_rad_pressure(&self) -> Option<String> {
        self.inner.solar_rad_pressure.clone()
    }
    #[setter]
    fn set_solar_rad_pressure(&mut self, v: Option<String>) {
        self.inner.solar_rad_pressure = v;
    }

    #[getter]
    fn get_earth_tides(&self) -> Option<String> {
        self.inner.earth_tides.clone()
    }
    #[setter]
    fn set_earth_tides(&mut self, v: Option<String>) {
        self.inner.earth_tides = v;
    }

    #[getter]
    fn get_intrack_thrust(&self) -> Option<String> {
        self.inner.intrack_thrust.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_intrack_thrust(&mut self, v: Option<String>) -> PyResult<()> {
        use std::str::FromStr;
        self.inner.intrack_thrust = match v {
            Some(s) => Some(YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))?),
            None => None,
        };
        Ok(())
    }

    #[getter]
    fn get_drag_parameters_source(&self) -> Option<String> {
        self.inner.drag_parameters_source.clone()
    }
    #[setter]
    fn set_drag_parameters_source(&mut self, v: Option<String>) {
        self.inner.drag_parameters_source = v;
    }

    #[getter]
    fn get_drag_parameters_altitude(&self) -> Option<f64> {
        self.inner.drag_parameters_altitude.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_drag_parameters_altitude(&mut self, v: Option<f64>) {
        self.inner.drag_parameters_altitude = v.map(PositionRequired::new);
    }

    #[getter]
    fn get_reentry_uncertainty_method(&self) -> Option<String> {
        self.inner.reentry_uncertainty_method.clone()
    }
    #[setter]
    fn set_reentry_uncertainty_method(&mut self, v: Option<String>) {
        self.inner.reentry_uncertainty_method = v;
    }

    #[getter]
    fn get_reentry_disintegration(&self) -> Option<String> {
        self.inner.reentry_disintegration.clone()
    }
    #[setter]
    fn set_reentry_disintegration(&mut self, v: Option<String>) {
        self.inner.reentry_disintegration = v;
    }

    #[getter]
    fn get_impact_uncertainty_method(&self) -> Option<String> {
        self.inner.impact_uncertainty_method.clone()
    }
    #[setter]
    fn set_impact_uncertainty_method(&mut self, v: Option<String>) {
        self.inner.impact_uncertainty_method = v;
    }

    #[getter]
    fn get_previous_message_id(&self) -> Option<String> {
        self.inner.previous_message_id.clone()
    }
    #[setter]
    fn set_previous_message_id(&mut self, v: Option<String>) {
        self.inner.previous_message_id = v;
    }

    #[getter]
    fn get_previous_message_epoch(&self) -> Option<String> {
        self.inner
            .previous_message_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_previous_message_epoch(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.previous_message_epoch = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_next_message_epoch(&self) -> Option<String> {
        self.inner
            .next_message_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_next_message_epoch(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.next_message_epoch = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// RDM Data
// ============================================================================

/// Represents the Data Section of an RDM Segment.
///
/// Contains logical blocks for atmospheric re-entry, ground impact, state vector,
/// covariance, object parameters, OD parameters, and user-defined parameters.
///
/// Parameters
/// ----------
/// atmospheric_reentry_parameters : AtmosphericReentryParameters
///     Mandatory atmospheric re-entry data.
/// ground_impact_parameters : GroundImpactParameters, optional
///     Ground impact and burn-up data.
/// state_vector : RdmStateVector, optional
///     Spacecraft state vector.
/// covariance_matrix : RdmCovarianceMatrix, optional
///     Position/velocity covariance matrix.
/// spacecraft_parameters : RdmSpacecraftParameters, optional
///     Object physical parameters.
/// od_parameters : OdParameters, optional
///     Orbit determination parameters.
/// user_defined_parameters : list[tuple[str, str]], optional
///     User defined parameters as key-value pairs.
/// comment : list[str], optional
///     Comments.
#[pyclass]
#[derive(Clone)]
pub struct RdmData {
    pub inner: core_rdm::RdmData,
}

#[pymethods]
impl RdmData {
    #[new]
    #[pyo3(signature = (
        *,
        atmospheric_reentry_parameters,
        ground_impact_parameters=None,
        state_vector=None,
        covariance_matrix=None,
        spacecraft_parameters=None,
        od_parameters=None,
        user_defined_parameters=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        atmospheric_reentry_parameters: AtmosphericReentryParameters,
        ground_impact_parameters: Option<GroundImpactParameters>,
        state_vector: Option<RdmStateVector>,
        covariance_matrix: Option<RdmCovarianceMatrix>,
        spacecraft_parameters: Option<RdmSpacecraftParameters>,
        od_parameters: Option<OdParameters>,
        user_defined_parameters: Option<Vec<(String, String)>>,
        comment: Option<Vec<String>>,
    ) -> Self {
        Self {
            inner: core_rdm::RdmData {
                comment: comment.unwrap_or_default(),
                atmospheric_reentry_parameters: atmospheric_reentry_parameters.inner,
                ground_impact_parameters: ground_impact_parameters.map(|g| g.inner),
                state_vector: state_vector.map(|sv| sv.inner),
                covariance_matrix: covariance_matrix.map(|cm| cm.inner),
                spacecraft_parameters: spacecraft_parameters.map(|sp| sp.inner),
                od_parameters: od_parameters.map(|op| op.inner),
                user_defined_parameters: user_defined_parameters.unwrap_or_default(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "RdmData(orbit_lifetime={} days)",
            self.inner
                .atmospheric_reentry_parameters
                .orbit_lifetime
                .value
        )
    }

    #[getter]
    fn get_atmospheric_reentry_parameters(&self) -> AtmosphericReentryParameters {
        AtmosphericReentryParameters {
            inner: self.inner.atmospheric_reentry_parameters.clone(),
        }
    }
    #[setter]
    fn set_atmospheric_reentry_parameters(&mut self, v: AtmosphericReentryParameters) {
        self.inner.atmospheric_reentry_parameters = v.inner;
    }

    #[getter]
    fn get_ground_impact_parameters(&self) -> Option<GroundImpactParameters> {
        self.inner
            .ground_impact_parameters
            .as_ref()
            .map(|g| GroundImpactParameters { inner: g.clone() })
    }
    #[setter]
    fn set_ground_impact_parameters(&mut self, v: Option<GroundImpactParameters>) {
        self.inner.ground_impact_parameters = v.map(|g| g.inner);
    }

    #[getter]
    fn get_state_vector(&self) -> Option<RdmStateVector> {
        self.inner
            .state_vector
            .as_ref()
            .map(|sv| RdmStateVector { inner: sv.clone() })
    }
    #[setter]
    fn set_state_vector(&mut self, v: Option<RdmStateVector>) {
        self.inner.state_vector = v.map(|sv| sv.inner);
    }

    #[getter]
    fn get_covariance_matrix(&self) -> Option<RdmCovarianceMatrix> {
        self.inner
            .covariance_matrix
            .as_ref()
            .map(|cm| RdmCovarianceMatrix { inner: cm.clone() })
    }
    #[setter]
    fn set_covariance_matrix(&mut self, v: Option<RdmCovarianceMatrix>) {
        self.inner.covariance_matrix = v.map(|cm| cm.inner);
    }

    #[getter]
    fn get_spacecraft_parameters(&self) -> Option<RdmSpacecraftParameters> {
        self.inner
            .spacecraft_parameters
            .as_ref()
            .map(|sp| RdmSpacecraftParameters { inner: sp.clone() })
    }
    #[setter]
    fn set_spacecraft_parameters(&mut self, v: Option<RdmSpacecraftParameters>) {
        self.inner.spacecraft_parameters = v.map(|sp| sp.inner);
    }

    #[getter]
    fn get_od_parameters(&self) -> Option<OdParameters> {
        self.inner
            .od_parameters
            .as_ref()
            .map(|op| OdParameters { inner: op.clone() })
    }
    #[setter]
    fn set_od_parameters(&mut self, v: Option<OdParameters>) {
        self.inner.od_parameters = v.map(|op| op.inner);
    }

    #[getter]
    fn get_user_defined_parameters(&self) -> Vec<(String, String)> {
        self.inner.user_defined_parameters.clone()
    }
    #[setter]
    fn set_user_defined_parameters(&mut self, v: Vec<(String, String)>) {
        self.inner.user_defined_parameters = v;
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// Atmospheric Reentry Parameters
// ============================================================================

/// Atmospheric re-entry information.
///
/// Parameters
/// ----------
/// orbit_lifetime : float
///     Remaining time in orbit (days).
/// reentry_altitude : float
///     Defined re-entry altitude (km).
#[pyclass]
#[derive(Clone)]
pub struct AtmosphericReentryParameters {
    pub inner: core_common::AtmosphericReentryParameters,
}

#[pymethods]
impl AtmosphericReentryParameters {
    #[new]
    #[pyo3(signature = (
        *,
        orbit_lifetime,
        reentry_altitude,
        orbit_lifetime_window_start=None,
        orbit_lifetime_window_end=None,
        nominal_reentry_epoch=None,
        reentry_window_start=None,
        reentry_window_end=None,
        orbit_lifetime_confidence_level=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        orbit_lifetime: f64,
        reentry_altitude: f64,
        orbit_lifetime_window_start: Option<f64>,
        orbit_lifetime_window_end: Option<f64>,
        nominal_reentry_epoch: Option<String>,
        reentry_window_start: Option<String>,
        reentry_window_end: Option<String>,
        orbit_lifetime_confidence_level: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_common::AtmosphericReentryParameters {
                comment: comment.unwrap_or_default(),
                orbit_lifetime: DayIntervalRequired::new(orbit_lifetime)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                reentry_altitude: PositionRequired::new(reentry_altitude),
                orbit_lifetime_window_start: orbit_lifetime_window_start
                    .map(|v| DayIntervalRequired::new(v))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                orbit_lifetime_window_end: orbit_lifetime_window_end
                    .map(|v| DayIntervalRequired::new(v))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                nominal_reentry_epoch: nominal_reentry_epoch
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
                reentry_window_start: reentry_window_start
                    .map(|s| parse_epoch(&s))
                    .transpose()?,
                reentry_window_end: reentry_window_end.map(|s| parse_epoch(&s)).transpose()?,
                orbit_lifetime_confidence_level: orbit_lifetime_confidence_level
                    .map(|v| PercentageRequired::new(v))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            },
        })
    }

    #[getter]
    fn get_orbit_lifetime(&self) -> f64 {
        self.inner.orbit_lifetime.value
    }
    #[setter]
    fn set_orbit_lifetime(&mut self, v: f64) {
        self.inner.orbit_lifetime.value = v;
    }

    #[getter]
    fn get_reentry_altitude(&self) -> f64 {
        self.inner.reentry_altitude.value
    }
    #[setter]
    fn set_reentry_altitude(&mut self, v: f64) {
        self.inner.reentry_altitude.value = v;
    }

    #[getter]
    fn get_orbit_lifetime_window_start(&self) -> Option<f64> {
        self.inner
            .orbit_lifetime_window_start
            .as_ref()
            .map(|v| v.value)
    }
    #[setter]
    fn set_orbit_lifetime_window_start(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.orbit_lifetime_window_start = v
            .map(|x| DayIntervalRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_orbit_lifetime_window_end(&self) -> Option<f64> {
        self.inner
            .orbit_lifetime_window_end
            .as_ref()
            .map(|v| v.value)
    }
    #[setter]
    fn set_orbit_lifetime_window_end(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.orbit_lifetime_window_end = v
            .map(|x| DayIntervalRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_nominal_reentry_epoch(&self) -> Option<String> {
        self.inner
            .nominal_reentry_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_nominal_reentry_epoch(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.nominal_reentry_epoch = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_reentry_window_start(&self) -> Option<String> {
        self.inner
            .reentry_window_start
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_reentry_window_start(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.reentry_window_start = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_reentry_window_end(&self) -> Option<String> {
        self.inner
            .reentry_window_end
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_reentry_window_end(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.reentry_window_end = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_orbit_lifetime_confidence_level(&self) -> Option<f64> {
        self.inner
            .orbit_lifetime_confidence_level
            .as_ref()
            .map(|v| v.value)
    }
    #[setter]
    fn set_orbit_lifetime_confidence_level(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.orbit_lifetime_confidence_level = v
            .map(|x| PercentageRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// Ground Impact Parameters
// ============================================================================

/// Ground impact and burn-up data parameters.
#[pyclass]
#[derive(Clone)]
pub struct GroundImpactParameters {
    pub inner: core_common::GroundImpactParameters,
}

#[pymethods]
impl GroundImpactParameters {
    #[new]
    #[pyo3(signature = (*, probability_of_impact=None, probability_of_burn_up=None, comment=None))]
    fn new(
        probability_of_impact: Option<f64>,
        probability_of_burn_up: Option<f64>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::Probability;

        Ok(Self {
            inner: core_common::GroundImpactParameters {
                comment: comment.unwrap_or_default(),
                probability_of_impact: probability_of_impact
                    .map(|v| Probability::new(v))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                probability_of_burn_up: probability_of_burn_up
                    .map(|v| Probability::new(v))
                    .transpose()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
                probability_of_break_up: None,
                probability_of_land_impact: None,
                probability_of_casualty: None,
                nominal_impact_epoch: None,
                impact_window_start: None,
                impact_window_end: None,
                impact_ref_frame: None,
                nominal_impact_lon: None,
                nominal_impact_lat: None,
                nominal_impact_alt: None,
                impact_1_confidence: None,
                impact_1_start_lon: None,
                impact_1_start_lat: None,
                impact_1_stop_lon: None,
                impact_1_stop_lat: None,
                impact_1_cross_track: None,
                impact_2_confidence: None,
                impact_2_start_lon: None,
                impact_2_start_lat: None,
                impact_2_stop_lon: None,
                impact_2_stop_lat: None,
                impact_2_cross_track: None,
                impact_3_confidence: None,
                impact_3_start_lon: None,
                impact_3_start_lat: None,
                impact_3_stop_lon: None,
                impact_3_stop_lat: None,
                impact_3_cross_track: None,
            },
        })
    }

    #[getter]
    fn get_probability_of_impact(&self) -> Option<f64> {
        self.inner.probability_of_impact.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_probability_of_impact(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.probability_of_impact = v
            .map(|x| ccsds_ndm::types::Probability::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_probability_of_burn_up(&self) -> Option<f64> {
        self.inner.probability_of_burn_up.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_probability_of_burn_up(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.probability_of_burn_up = v
            .map(|x| ccsds_ndm::types::Probability::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_probability_of_break_up(&self) -> Option<f64> {
        self.inner.probability_of_break_up.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_probability_of_break_up(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.probability_of_break_up = v
            .map(|x| ccsds_ndm::types::Probability::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_probability_of_land_impact(&self) -> Option<f64> {
        self.inner
            .probability_of_land_impact
            .as_ref()
            .map(|p| p.value)
    }
    #[setter]
    fn set_probability_of_land_impact(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.probability_of_land_impact = v
            .map(|x| ccsds_ndm::types::Probability::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_probability_of_casualty(&self) -> Option<f64> {
        self.inner.probability_of_casualty.as_ref().map(|p| p.value)
    }
    #[setter]
    fn set_probability_of_casualty(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.probability_of_casualty = v
            .map(|x| ccsds_ndm::types::Probability::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_nominal_impact_epoch(&self) -> Option<String> {
        self.inner
            .nominal_impact_epoch
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_nominal_impact_epoch(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.nominal_impact_epoch = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_impact_window_start(&self) -> Option<String> {
        self.inner
            .impact_window_start
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_impact_window_start(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.impact_window_start = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_impact_window_end(&self) -> Option<String> {
        self.inner
            .impact_window_end
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_impact_window_end(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.impact_window_end = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_impact_ref_frame(&self) -> Option<String> {
        self.inner.impact_ref_frame.clone()
    }
    #[setter]
    fn set_impact_ref_frame(&mut self, v: Option<String>) {
        self.inner.impact_ref_frame = v;
    }

    #[getter]
    fn get_nominal_impact_lon(&self) -> Option<f64> {
        self.inner.nominal_impact_lon.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_nominal_impact_lon(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.nominal_impact_lon = v
            .map(|x| ccsds_ndm::types::LongitudeRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_nominal_impact_lat(&self) -> Option<f64> {
        self.inner.nominal_impact_lat.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_nominal_impact_lat(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.nominal_impact_lat = v
            .map(|x| ccsds_ndm::types::LatitudeRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_nominal_impact_alt(&self) -> Option<f64> {
        self.inner.nominal_impact_alt.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_nominal_impact_alt(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.nominal_impact_alt = v
            .map(|x| ccsds_ndm::types::AltitudeRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// RDM Spacecraft Parameters
// ============================================================================

/// Object physical parameters.
#[pyclass]
#[derive(Clone)]
pub struct RdmSpacecraftParameters {
    pub inner: core_common::RdmSpacecraftParameters,
}

#[pymethods]
impl RdmSpacecraftParameters {
    #[new]
    #[pyo3(signature = (*, wet_mass=None, dry_mass=None, comment=None))]
    fn new(wet_mass: Option<f64>, dry_mass: Option<f64>, comment: Option<Vec<String>>) -> Self {
        use ccsds_ndm::types::Mass;

        Self {
            inner: core_common::RdmSpacecraftParameters {
                comment: comment.unwrap_or_default(),
                wet_mass: wet_mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                dry_mass: dry_mass.map(|v| Mass {
                    value: v,
                    units: None,
                }),
                hazardous_substances: None,
                solar_rad_area: None,
                solar_rad_coeff: None,
                drag_area: None,
                drag_coeff: None,
                rcs: None,
                ballistic_coeff: None,
                thrust_acceleration: None,
            },
        }
    }

    #[getter]
    fn get_wet_mass(&self) -> Option<f64> {
        self.inner.wet_mass.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_wet_mass(&mut self, v: Option<f64>) {
        self.inner.wet_mass = v.map(|x| ccsds_ndm::types::Mass {
            value: x,
            units: None,
        });
    }

    #[getter]
    fn get_dry_mass(&self) -> Option<f64> {
        self.inner.dry_mass.as_ref().map(|m| m.value)
    }
    #[setter]
    fn set_dry_mass(&mut self, v: Option<f64>) {
        self.inner.dry_mass = v.map(|x| ccsds_ndm::types::Mass {
            value: x,
            units: None,
        });
    }

    #[getter]
    fn get_hazardous_substances(&self) -> Option<String> {
        self.inner.hazardous_substances.clone()
    }
    #[setter]
    fn set_hazardous_substances(&mut self, v: Option<String>) {
        self.inner.hazardous_substances = v;
    }

    #[getter]
    fn get_solar_rad_area(&self) -> Option<f64> {
        self.inner.solar_rad_area.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_solar_rad_area(&mut self, v: Option<f64>) {
        self.inner.solar_rad_area = v.map(|x| ccsds_ndm::types::Area { value: x, units: None });
    }

    #[getter]
    fn get_solar_rad_coeff(&self) -> Option<f64> {
        self.inner.solar_rad_coeff
    }
    #[setter]
    fn set_solar_rad_coeff(&mut self, v: Option<f64>) {
        self.inner.solar_rad_coeff = v;
    }

    #[getter]
    fn get_drag_area(&self) -> Option<f64> {
        self.inner.drag_area.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_drag_area(&mut self, v: Option<f64>) {
        self.inner.drag_area = v.map(|x| ccsds_ndm::types::Area { value: x, units: None });
    }

    #[getter]
    fn get_drag_coeff(&self) -> Option<f64> {
        self.inner.drag_coeff
    }
    #[setter]
    fn set_drag_coeff(&mut self, v: Option<f64>) {
        self.inner.drag_coeff = v;
    }

    #[getter]
    fn get_rcs(&self) -> Option<f64> {
        self.inner.rcs.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_rcs(&mut self, v: Option<f64>) {
        self.inner.rcs = v.map(|x| ccsds_ndm::types::Area { value: x, units: None });
    }

    #[getter]
    fn get_ballistic_coeff(&self) -> Option<f64> {
        self.inner.ballistic_coeff.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_ballistic_coeff(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.ballistic_coeff = v
            .map(|x| ccsds_ndm::types::BallisticCoeffRequired::new(x))
            .transpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn get_thrust_acceleration(&self) -> Option<f64> {
        self.inner.thrust_acceleration.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_thrust_acceleration(&mut self, v: Option<f64>) {
        self.inner.thrust_acceleration = v.map(ccsds_ndm::types::Ms2Required::new);
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// Orbit Determination Parameters
// ============================================================================

/// Orbit Determination Parameters.
#[pyclass]
#[derive(Clone)]
pub struct OdParameters {
    pub inner: core_common::OdParameters,
}

#[pymethods]
impl OdParameters {
    #[new]
    fn new(comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_common::OdParameters {
                comment: comment.unwrap_or_default(),
                time_lastob_start: None,
                time_lastob_end: None,
                recommended_od_span: None,
                actual_od_span: None,
                obs_available: None,
                obs_used: None,
                tracks_available: None,
                tracks_used: None,
                residuals_accepted: None,
                weighted_rms: None,
            },
        }
    }

    #[getter]
    fn get_time_lastob_start(&self) -> Option<String> {
        self.inner
            .time_lastob_start
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_time_lastob_start(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.time_lastob_start = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_time_lastob_end(&self) -> Option<String> {
        self.inner
            .time_lastob_end
            .as_ref()
            .map(|e| e.as_str().to_string())
    }
    #[setter]
    fn set_time_lastob_end(&mut self, v: Option<String>) -> PyResult<()> {
        self.inner.time_lastob_end = v.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    #[getter]
    fn get_recommended_od_span(&self) -> Option<f64> {
        self.inner.recommended_od_span.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_recommended_od_span(&mut self, v: Option<f64>) {
        self.inner.recommended_od_span = v.map(|x| DayInterval {
            value: x,
            units: None,
        });
    }

    #[getter]
    fn get_actual_od_span(&self) -> Option<f64> {
        self.inner.actual_od_span.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_actual_od_span(&mut self, v: Option<f64>) {
        self.inner.actual_od_span = v.map(|x| DayInterval {
            value: x,
            units: None,
        });
    }

    #[getter]
    fn get_obs_available(&self) -> Option<u32> {
        self.inner.obs_available
    }
    #[setter]
    fn set_obs_available(&mut self, v: Option<u32>) {
        self.inner.obs_available = v;
    }

    #[getter]
    fn get_obs_used(&self) -> Option<u32> {
        self.inner.obs_used
    }
    #[setter]
    fn set_obs_used(&mut self, v: Option<u32>) {
        self.inner.obs_used = v;
    }

    #[getter]
    fn get_tracks_available(&self) -> Option<u32> {
        self.inner.tracks_available
    }
    #[setter]
    fn set_tracks_available(&mut self, v: Option<u32>) {
        self.inner.tracks_available = v;
    }

    #[getter]
    fn get_tracks_used(&self) -> Option<u32> {
        self.inner.tracks_used
    }
    #[setter]
    fn set_tracks_used(&mut self, v: Option<u32>) {
        self.inner.tracks_used = v;
    }

    #[getter]
    fn get_residuals_accepted(&self) -> Option<f64> {
        self.inner.residuals_accepted.as_ref().map(|v| v.value)
    }
    #[setter]
    fn set_residuals_accepted(&mut self, v: Option<f64>) -> PyResult<()> {
        self.inner.residuals_accepted = v.map(|x| Percentage { value: x, units: None });
        Ok(())
    }

    #[getter]
    fn get_weighted_rms(&self) -> Option<f64> {
        self.inner.weighted_rms
    }
    #[setter]
    fn set_weighted_rms(&mut self, v: Option<f64>) {
        self.inner.weighted_rms = v;
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}

// ============================================================================
// State Vector
// ============================================================================

/// Spacecraft State Vector.
#[pyclass]
#[derive(Clone)]
pub struct RdmStateVector {
    pub inner: core_common::StateVector,
}

#[pymethods]
impl RdmStateVector {
    #[new]
    #[pyo3(signature = (*, epoch, x, y, z, x_dot, y_dot, z_dot))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        epoch: String,
        x: f64,
        y: f64,
        z: f64,
        x_dot: f64,
        y_dot: f64,
        z_dot: f64,
    ) -> PyResult<Self> {
        use ccsds_ndm::types::{Position, Velocity};
        Ok(Self {
            inner: core_common::StateVector {
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

    #[getter]
    fn get_epoch(&self) -> String {
        self.inner.epoch.as_str().to_string()
    }
    #[setter]
    fn set_epoch(&mut self, v: String) -> PyResult<()> {
        self.inner.epoch = parse_epoch(&v)?;
        Ok(())
    }

    #[getter]
    fn get_x(&self) -> f64 {
        self.inner.x.value
    }
    #[setter]
    fn set_x(&mut self, v: f64) {
        self.inner.x.value = v;
    }

    #[getter]
    fn get_y(&self) -> f64 {
        self.inner.y.value
    }
    #[setter]
    fn set_y(&mut self, v: f64) {
        self.inner.y.value = v;
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z.value
    }
    #[setter]
    fn set_z(&mut self, v: f64) {
        self.inner.z.value = v;
    }

    #[getter]
    fn get_x_dot(&self) -> f64 {
        self.inner.x_dot.value
    }
    #[setter]
    fn set_x_dot(&mut self, v: f64) {
        self.inner.x_dot.value = v;
    }

    #[getter]
    fn get_y_dot(&self) -> f64 {
        self.inner.y_dot.value
    }
    #[setter]
    fn set_y_dot(&mut self, v: f64) {
        self.inner.y_dot.value = v;
    }

    #[getter]
    fn get_z_dot(&self) -> f64 {
        self.inner.z_dot.value
    }
    #[setter]
    fn set_z_dot(&mut self, v: f64) {
        self.inner.z_dot.value = v;
    }
}

// ============================================================================
// Covariance Matrix
// ============================================================================

/// Position/velocity covariance matrix.
#[pyclass]
#[derive(Clone)]
pub struct RdmCovarianceMatrix {
    pub inner: core_common::OpmCovarianceMatrix,
}

#[pymethods]
impl RdmCovarianceMatrix {
    #[new]
    fn new(cov_ref_frame: Option<String>, comment: Option<Vec<String>>) -> Self {
        // Initializes with zeros for the matrix elements
        Self {
            inner: core_common::OpmCovarianceMatrix {
                comment: comment.unwrap_or_default(),
                cov_ref_frame,
                cx_x: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_x: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_y: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_x: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_y: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_z: ccsds_ndm::types::PositionCovariance {
                    value: 0.0,
                    units: None,
                },
                cx_dot_x: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cx_dot_y: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cx_dot_z: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cx_dot_x_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_dot_x: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_dot_y: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_dot_z: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_dot_x_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cy_dot_y_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_x: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_y: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_z: ccsds_ndm::types::PositionVelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_x_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_y_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
                cz_dot_z_dot: ccsds_ndm::types::VelocityCovariance {
                    value: 0.0,
                    units: None,
                },
            },
        }
    }

    #[getter]
    fn get_cov_ref_frame(&self) -> Option<String> {
        self.inner.cov_ref_frame.clone()
    }
    #[setter]
    fn set_cov_ref_frame(&mut self, v: Option<String>) {
        self.inner.cov_ref_frame = v;
    }

    #[getter]
    fn get_cx_x(&self) -> f64 {
        self.inner.cx_x.value
    }
    #[setter]
    fn set_cx_x(&mut self, v: f64) {
        self.inner.cx_x.value = v;
    }

    // ... Implement all other covariance elements ...
    // Since there are 21 elements, providing a few examples. The pattern is identical.

    #[getter]
    fn get_cy_x(&self) -> f64 {
        self.inner.cy_x.value
    }
    #[setter]
    fn set_cy_x(&mut self, v: f64) {
        self.inner.cy_x.value = v;
    }

    #[getter]
    fn get_cy_y(&self) -> f64 {
        self.inner.cy_y.value
    }
    #[setter]
    fn set_cy_y(&mut self, v: f64) {
        self.inner.cy_y.value = v;
    }

    #[getter]
    fn get_cz_x(&self) -> f64 {
        self.inner.cz_x.value
    }
    #[setter]
    fn set_cz_x(&mut self, v: f64) {
        self.inner.cz_x.value = v;
    }

    #[getter]
    fn get_cz_y(&self) -> f64 {
        self.inner.cz_y.value
    }
    #[setter]
    fn set_cz_y(&mut self, v: f64) {
        self.inner.cz_y.value = v;
    }

    #[getter]
    fn get_cz_z(&self) -> f64 {
        self.inner.cz_z.value
    }
    #[setter]
    fn set_cz_z(&mut self, v: f64) {
        self.inner.cz_z.value = v;
    }

    // Velocity-Position Cross
    #[getter]
    fn get_cx_dot_x(&self) -> f64 {
        self.inner.cx_dot_x.value
    }
    #[setter]
    fn set_cx_dot_x(&mut self, v: f64) {
        self.inner.cx_dot_x.value = v;
    }

    #[getter]
    fn get_cx_dot_y(&self) -> f64 {
        self.inner.cx_dot_y.value
    }
    #[setter]
    fn set_cx_dot_y(&mut self, v: f64) {
        self.inner.cx_dot_y.value = v;
    }

    #[getter]
    fn get_cx_dot_z(&self) -> f64 {
        self.inner.cx_dot_z.value
    }
    #[setter]
    fn set_cx_dot_z(&mut self, v: f64) {
        self.inner.cx_dot_z.value = v;
    }

    // Velocity Covariance
    #[getter]
    fn get_cx_dot_x_dot(&self) -> f64 {
        self.inner.cx_dot_x_dot.value
    }
    #[setter]
    fn set_cx_dot_x_dot(&mut self, v: f64) {
        self.inner.cx_dot_x_dot.value = v;
    }

    #[getter]
    fn get_cy_dot_x(&self) -> f64 {
        self.inner.cy_dot_x.value
    }
    #[setter]
    fn set_cy_dot_x(&mut self, v: f64) {
        self.inner.cy_dot_x.value = v;
    }

    #[getter]
    fn get_cy_dot_y(&self) -> f64 {
        self.inner.cy_dot_y.value
    }
    #[setter]
    fn set_cy_dot_y(&mut self, v: f64) {
        self.inner.cy_dot_y.value = v;
    }

    #[getter]
    fn get_cy_dot_z(&self) -> f64 {
        self.inner.cy_dot_z.value
    }
    #[setter]
    fn set_cy_dot_z(&mut self, v: f64) {
        self.inner.cy_dot_z.value = v;
    }

    #[getter]
    fn get_cy_dot_x_dot(&self) -> f64 {
        self.inner.cy_dot_x_dot.value
    }
    #[setter]
    fn set_cy_dot_x_dot(&mut self, v: f64) {
        self.inner.cy_dot_x_dot.value = v;
    }

    #[getter]
    fn get_cy_dot_y_dot(&self) -> f64 {
        self.inner.cy_dot_y_dot.value
    }
    #[setter]
    fn set_cy_dot_y_dot(&mut self, v: f64) {
        self.inner.cy_dot_y_dot.value = v;
    }

    #[getter]
    fn get_cz_dot_x(&self) -> f64 {
        self.inner.cz_dot_x.value
    }
    #[setter]
    fn set_cz_dot_x(&mut self, v: f64) {
        self.inner.cz_dot_x.value = v;
    }

    #[getter]
    fn get_cz_dot_y(&self) -> f64 {
        self.inner.cz_dot_y.value
    }
    #[setter]
    fn set_cz_dot_y(&mut self, v: f64) {
        self.inner.cz_dot_y.value = v;
    }

    #[getter]
    fn get_cz_dot_z(&self) -> f64 {
        self.inner.cz_dot_z.value
    }
    #[setter]
    fn set_cz_dot_z(&mut self, v: f64) {
        self.inner.cz_dot_z.value = v;
    }

    #[getter]
    fn get_cz_dot_x_dot(&self) -> f64 {
        self.inner.cz_dot_x_dot.value
    }
    #[setter]
    fn set_cz_dot_x_dot(&mut self, v: f64) {
        self.inner.cz_dot_x_dot.value = v;
    }

    #[getter]
    fn get_cz_dot_y_dot(&self) -> f64 {
        self.inner.cz_dot_y_dot.value
    }
    #[setter]
    fn set_cz_dot_y_dot(&mut self, v: f64) {
        self.inner.cz_dot_y_dot.value = v;
    }

    #[getter]
    fn get_cz_dot_z_dot(&self) -> f64 {
        self.inner.cz_dot_z_dot.value
    }
    #[setter]
    fn set_cz_dot_z_dot(&mut self, v: f64) {
        self.inner.cz_dot_z_dot.value = v;
    }

    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, v: Vec<String>) {
        self.inner.comment = v;
    }
}