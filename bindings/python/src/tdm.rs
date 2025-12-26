// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::parse_epoch;
use ccsds_ndm::messages::tdm as core_tdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;

// ============================================================================
// TDM - Tracking Data Message
// ============================================================================

/// Represents a CCSDS Tracking Data Message (TDM).
///
/// The TDM is a standard message format for use in exchanging spacecraft tracking data
/// between space agencies. Such exchanges are used for distributing tracking data output
/// from routine interagency cross-supports in which spacecraft missions managed by one
/// agency are tracked from a tracking station managed by a second agency.
///
/// Parameters
/// ----------
/// header : TdmHeader
///     The message header.
///     (Mandatory)
/// body : TdmBody
///     The message body containing segments.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct Tdm {
    pub inner: core_tdm::Tdm,
}

#[pymethods]
impl Tdm {
    #[new]
    #[pyo3(signature = (*, header, body))]
    fn new(header: TdmHeader, body: TdmBody) -> Self {
        Self {
            inner: core_tdm::Tdm {
                header: header.inner,
                body: body.inner,
                id: Some("CCSDS_TDM_VERS".to_string()),
                version: "2.0".to_string(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Tdm(version='{}', segments={})",
            self.inner.version,
            self.inner.body.segments.len()
        )
    }

    /// The message header.
    ///
    /// :type: TdmHeader
    #[getter]
    fn get_header(&self) -> TdmHeader {
        TdmHeader {
            inner: self.inner.header.clone(),
        }
    }

    #[setter]
    fn set_header(&mut self, header: TdmHeader) {
        self.inner.header = header.inner;
    }

    /// The message body.
    ///
    /// :type: TdmBody
    #[getter]
    fn get_body(&self) -> TdmBody {
        TdmBody {
            inner: self.inner.body.clone(),
        }
    }

    #[setter]
    fn set_body(&mut self, body: TdmBody) {
        self.inner.body = body.inner;
    }

    /// Shortcut to access segments directly from the body.
    ///
    /// :type: List[TdmSegment]
    #[getter]
    fn get_segments(&self) -> Vec<TdmSegment> {
        self.inner
            .body
            .segments
            .iter()
            .map(|s| TdmSegment { inner: s.clone() })
            .collect()
    }

    /// Create a TDM message from a string.
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
    /// Tdm
    ///     The parsed TDM object.
    #[staticmethod]
    #[pyo3(signature = (data, format=None))]
    fn from_str(data: &str, format: Option<&str>) -> PyResult<Self> {
        let inner = match format {
            Some("kvn") => core_tdm::Tdm::from_kvn(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some("xml") => core_tdm::Tdm::from_xml(data)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported format '{}'. Use 'kvn' or 'xml'",
                    other
                )))
            }
            None => match ccsds_ndm::from_str(data) {
                Ok(MessageType::Tdm(tdm)) => tdm,
                Ok(other) => {
                    return Err(PyValueError::new_err(format!(
                        "Parsed message is not TDM (got {:?})",
                        other
                    )))
                }
                Err(e) => return Err(PyValueError::new_err(e.to_string())),
            },
        };
        Ok(Self { inner })
    }

    /// Create a TDM message from a file.
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
    /// Tdm
    ///     The parsed TDM object.
    #[staticmethod]
    #[pyo3(signature = (path, format=None))]
    fn from_file(path: &str, format: Option<&str>) -> PyResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
        Self::from_str(&content, format)
    }

    /// Serialize to string.
    ///
    /// Parameters
    /// ----------
    /// format : str
    ///     Output format ('kvn' or 'xml').
    ///
    /// Returns
    /// -------
    /// str
    ///     The serialized string.
    fn to_str(&self, format: &str) -> PyResult<String> {
        match format {
            "kvn" => self
                .inner
                .to_kvn()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            "xml" => self
                .inner
                .to_xml()
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            other => Err(PyValueError::new_err(format!(
                "Unsupported format '{}'. Use 'kvn' or 'xml'",
                other
            ))),
        }
    }

    /// Write to file.
    ///
    /// Parameters
    /// ----------
    /// path : str
    ///     Output file path.
    /// format : str
    ///     Output format ('kvn' or 'xml').
    fn to_file(&self, path: &str, format: &str) -> PyResult<()> {
        let data = self.to_str(format)?;
        match fs::write(path, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyValueError::new_err(format!("Failed to write file: {}", e))),
        }
    }
}

// ============================================================================
// TDM Header
// ============================================================================

/// Represents the Header of a Tracking Data Message.
///
/// Parameters
/// ----------
/// originator : str
///     Creating agency. Value should be an entry from the SANA Organizations Registry.
///     (Mandatory)
/// creation_date : str
///     Data creation date/time in UTC.
///     (Mandatory)
/// message_id : str, optional
///     ID that uniquely identifies a message from a given originator.
///     (Optional)
/// comment : list[str], optional
///     Comments.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct TdmHeader {
    pub inner: core_tdm::TdmHeader,
}

#[pymethods]
impl TdmHeader {
    #[new]
    #[pyo3(signature = (*, originator, creation_date, message_id=None, comment=None))]
    fn new(
        originator: String,
        creation_date: String,
        message_id: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_tdm::TdmHeader {
                comment: comment.unwrap_or_default(),
                creation_date: parse_epoch(&creation_date)?,
                originator,
                message_id,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("TdmHeader(originator='{}')", self.inner.originator)
    }

    /// Creating agency.
    ///
    /// Value should be an entry from the 'Abbreviation' column in the SANA Organizations Registry.
    /// Examples: CNES, ESA, GSFC, DLR, JPL, JAXA, etc.
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

    /// Data creation date/time in UTC.
    ///
    /// Format: ISO 8601 (e.g., "2001-11-06T11:17:33" or "2006-001T00:00:00Z")
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

    /// ID that uniquely identifies a message from a given originator.
    ///
    /// The format and content of the message identifier value are at the discretion of the originator.
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

    /// Comments in the Header.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

// ============================================================================
// TDM Body
// ============================================================================

/// Represents the Body of a Tracking Data Message.
///
/// The TDM Body consists of one or more TDM Segments.
///
/// Parameters
/// ----------
/// segments : list[TdmSegment]
///     List of data segments.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct TdmBody {
    pub inner: core_tdm::TdmBody,
}

#[pymethods]
impl TdmBody {
    #[new]
    #[pyo3(signature = (*, segments))]
    fn new(segments: Vec<TdmSegment>) -> Self {
        Self {
            inner: core_tdm::TdmBody {
                segments: segments.into_iter().map(|s| s.inner).collect(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("TdmBody(segments={})", self.inner.segments.len())
    }

    /// List of TDM segments.
    ///
    /// Each segment consists of a Metadata Section and a Data Section.
    ///
    /// :type: List[TdmSegment]
    #[getter]
    fn get_segments(&self) -> Vec<TdmSegment> {
        self.inner
            .segments
            .iter()
            .map(|s| TdmSegment { inner: s.clone() })
            .collect()
    }

    #[setter]
    fn set_segments(&mut self, value: Vec<TdmSegment>) {
        self.inner.segments = value.into_iter().map(|s| s.inner).collect();
    }
}

// ============================================================================
// TDM Segment
// ============================================================================

/// Represents a single segment of a TDM.
///
/// A segment consists of a Metadata Section (configuration details) and a
/// Data Section (tracking data records).
///
/// Parameters
/// ----------
/// metadata : TdmMetadata
///     Segment metadata.
///     (Mandatory)
/// data : TdmData
///     Segment data.
///     (Mandatory)
#[pyclass]
#[derive(Clone)]
pub struct TdmSegment {
    pub inner: core_tdm::TdmSegment,
}

#[pymethods]
impl TdmSegment {
    #[new]
    #[pyo3(signature = (*, metadata, data))]
    fn new(metadata: TdmMetadata, data: TdmData) -> Self {
        Self {
            inner: core_tdm::TdmSegment {
                metadata: metadata.inner,
                data: data.inner,
            },
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "TdmSegment(participant_1='{}', observations={})",
            self.inner.metadata.participant_1,
            self.inner.data.observations.len()
        )
    }

    /// Segment metadata.
    ///
    /// Contains configuration details applicable to the Data Section.
    ///
    /// :type: TdmMetadata
    #[getter]
    fn get_metadata(&self) -> TdmMetadata {
        TdmMetadata {
            inner: self.inner.metadata.clone(),
        }
    }

    #[setter]
    fn set_metadata(&mut self, metadata: TdmMetadata) {
        self.inner.metadata = metadata.inner;
    }

    /// Segment data.
    ///
    /// Contains the tracking data records.
    ///
    /// :type: TdmData
    #[getter]
    fn get_data(&self) -> TdmData {
        TdmData {
            inner: self.inner.data.clone(),
        }
    }

    #[setter]
    fn set_data(&mut self, data: TdmData) {
        self.inner.data = data.inner;
    }
}

// ============================================================================
// TDM Metadata - Full implementation
// ============================================================================

/// Represents the Metadata Section of a TDM Segment.
///
/// Contains configuration details applicable to the Data Section in the same TDM Segment.
///
/// Mandatory Parameters
/// --------------------
/// time_system : str
///     Time system used for timetags (e.g., "UTC", "TAI").
/// participant_1 : str
///     First participant in the tracking session.
///
/// Optional Parameters
/// -------------------
/// Many optional parameters are available to describe the tracking configuration,
/// signal path, frequencies, and corrections. See CCSDS TDM Blue Book for full details.
#[pyclass]
#[derive(Clone)]
pub struct TdmMetadata {
    pub inner: core_tdm::TdmMetadata,
}

#[pymethods]
impl TdmMetadata {
    #[new]
    #[pyo3(signature = (
        *,
        time_system,
        participant_1,
        track_id=None,
        data_types=None,
        start_time=None,
        stop_time=None,
        participant_2=None,
        participant_3=None,
        participant_4=None,
        participant_5=None,
        mode=None,
        path=None,
        path_1=None,
        path_2=None,
        transmit_band=None,
        receive_band=None,
        turnaround_numerator=None,
        turnaround_denominator=None,
        timetag_ref=None,
        integration_interval=None,
        integration_ref=None,
        freq_offset=None,
        range_mode=None,
        range_modulus=None,
        range_units=None,
        angle_type=None,
        reference_frame=None,
        interpolation=None,
        interpolation_degree=None,
        doppler_count_bias=None,
        doppler_count_scale=None,
        doppler_count_rollover=None,
        transmit_delay_1=None,
        transmit_delay_2=None,
        transmit_delay_3=None,
        transmit_delay_4=None,
        transmit_delay_5=None,
        receive_delay_1=None,
        receive_delay_2=None,
        receive_delay_3=None,
        receive_delay_4=None,
        receive_delay_5=None,
        data_quality=None,
        correction_angle_1=None,
        correction_angle_2=None,
        correction_doppler=None,
        correction_mag=None,
        correction_range=None,
        correction_rcs=None,
        correction_receive=None,
        correction_transmit=None,
        correction_aberration_yearly=None,
        correction_aberration_diurnal=None,
        corrections_applied=None,
        ephemeris_name_1=None,
        ephemeris_name_2=None,
        ephemeris_name_3=None,
        ephemeris_name_4=None,
        ephemeris_name_5=None,
        comment=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        time_system: String,
        participant_1: String,
        track_id: Option<String>,
        data_types: Option<String>,
        start_time: Option<String>,
        stop_time: Option<String>,
        participant_2: Option<String>,
        participant_3: Option<String>,
        participant_4: Option<String>,
        participant_5: Option<String>,
        mode: Option<String>,
        path: Option<String>,
        path_1: Option<String>,
        path_2: Option<String>,
        transmit_band: Option<String>,
        receive_band: Option<String>,
        turnaround_numerator: Option<i32>,
        turnaround_denominator: Option<i32>,
        timetag_ref: Option<String>,
        integration_interval: Option<f64>,
        integration_ref: Option<String>,
        freq_offset: Option<f64>,
        range_mode: Option<String>,
        range_modulus: Option<f64>,
        range_units: Option<String>,
        angle_type: Option<String>,
        reference_frame: Option<String>,
        interpolation: Option<String>,
        interpolation_degree: Option<u32>,
        doppler_count_bias: Option<f64>,
        doppler_count_scale: Option<u64>,
        doppler_count_rollover: Option<String>,
        transmit_delay_1: Option<f64>,
        transmit_delay_2: Option<f64>,
        transmit_delay_3: Option<f64>,
        transmit_delay_4: Option<f64>,
        transmit_delay_5: Option<f64>,
        receive_delay_1: Option<f64>,
        receive_delay_2: Option<f64>,
        receive_delay_3: Option<f64>,
        receive_delay_4: Option<f64>,
        receive_delay_5: Option<f64>,
        data_quality: Option<String>,
        correction_angle_1: Option<f64>,
        correction_angle_2: Option<f64>,
        correction_doppler: Option<f64>,
        correction_mag: Option<f64>,
        correction_range: Option<f64>,
        correction_rcs: Option<f64>,
        correction_receive: Option<f64>,
        correction_transmit: Option<f64>,
        correction_aberration_yearly: Option<f64>,
        correction_aberration_diurnal: Option<f64>,
        corrections_applied: Option<String>,
        ephemeris_name_1: Option<String>,
        ephemeris_name_2: Option<String>,
        ephemeris_name_3: Option<String>,
        ephemeris_name_4: Option<String>,
        ephemeris_name_5: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: core_tdm::TdmMetadata {
                comment: comment.unwrap_or_default(),
                track_id,
                data_types,
                time_system,
                start_time: start_time.map(|s| parse_epoch(&s)).transpose()?,
                stop_time: stop_time.map(|s| parse_epoch(&s)).transpose()?,
                participant_1,
                participant_2,
                participant_3,
                participant_4,
                participant_5,
                mode,
                path,
                path_1,
                path_2,
                transmit_band,
                receive_band,
                turnaround_numerator,
                turnaround_denominator,
                timetag_ref,
                integration_interval,
                integration_ref,
                freq_offset,
                range_mode,
                range_modulus,
                range_units,
                angle_type,
                reference_frame,
                interpolation,
                interpolation_degree,
                doppler_count_bias,
                doppler_count_scale,
                doppler_count_rollover,
                transmit_delay_1,
                transmit_delay_2,
                transmit_delay_3,
                transmit_delay_4,
                transmit_delay_5,
                receive_delay_1,
                receive_delay_2,
                receive_delay_3,
                receive_delay_4,
                receive_delay_5,
                data_quality,
                correction_angle_1,
                correction_angle_2,
                correction_doppler,
                correction_mag,
                correction_range,
                correction_rcs,
                correction_receive,
                correction_transmit,
                correction_aberration_yearly,
                correction_aberration_diurnal,
                corrections_applied,
                ephemeris_name_1,
                ephemeris_name_2,
                ephemeris_name_3,
                ephemeris_name_4,
                ephemeris_name_5,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!("TdmMetadata(participant_1='{}')", self.inner.participant_1)
    }

    /// Comments.
    /// :type: List[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }
    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Unique identifier for the tracking data.
    /// :type: Optional[str]
    #[getter]
    fn get_track_id(&self) -> Option<String> {
        self.inner.track_id.clone()
    }
    #[setter]
    fn set_track_id(&mut self, value: Option<String>) {
        self.inner.track_id = value;
    }

    /// Comma-separated list of data types in the Data Section.
    /// :type: Optional[str]
    #[getter]
    fn get_data_types(&self) -> Option<String> {
        self.inner.data_types.clone()
    }
    #[setter]
    fn set_data_types(&mut self, value: Option<String>) {
        self.inner.data_types = value;
    }

    /// Time system used for timetags (e.g., UTC, TAI, GPS, SCLK).
    /// :type: str
    #[getter]
    fn get_time_system(&self) -> String {
        self.inner.time_system.clone()
    }
    #[setter]
    fn set_time_system(&mut self, value: String) {
        self.inner.time_system = value;
    }

    /// Start time of the tracking data.
    /// :type: Optional[str]
    #[getter]
    fn get_start_time(&self) -> Option<String> {
        self.inner
            .start_time
            .as_ref()
            .map(|t| t.as_str().to_string())
    }
    #[setter]
    fn set_start_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.start_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// Stop time of the tracking data.
    /// :type: Optional[str]
    #[getter]
    fn get_stop_time(&self) -> Option<String> {
        self.inner
            .stop_time
            .as_ref()
            .map(|t| t.as_str().to_string())
    }
    #[setter]
    fn set_stop_time(&mut self, value: Option<String>) -> PyResult<()> {
        self.inner.stop_time = value.map(|s| parse_epoch(&s)).transpose()?;
        Ok(())
    }

    /// First participant in the tracking session (Mandatory).
    /// :type: str
    #[getter]
    fn get_participant_1(&self) -> String {
        self.inner.participant_1.clone()
    }
    #[setter]
    fn set_participant_1(&mut self, value: String) {
        self.inner.participant_1 = value;
    }

    /// Second participant.
    /// :type: Optional[str]
    #[getter]
    fn get_participant_2(&self) -> Option<String> {
        self.inner.participant_2.clone()
    }
    #[setter]
    fn set_participant_2(&mut self, value: Option<String>) {
        self.inner.participant_2 = value;
    }

    /// Third participant.
    /// :type: Optional[str]
    #[getter]
    fn get_participant_3(&self) -> Option<String> {
        self.inner.participant_3.clone()
    }
    #[setter]
    fn set_participant_3(&mut self, value: Option<String>) {
        self.inner.participant_3 = value;
    }

    /// Fourth participant.
    /// :type: Optional[str]
    #[getter]
    fn get_participant_4(&self) -> Option<String> {
        self.inner.participant_4.clone()
    }
    #[setter]
    fn set_participant_4(&mut self, value: Option<String>) {
        self.inner.participant_4 = value;
    }

    /// Fifth participant.
    /// :type: Optional[str]
    #[getter]
    fn get_participant_5(&self) -> Option<String> {
        self.inner.participant_5.clone()
    }
    #[setter]
    fn set_participant_5(&mut self, value: Option<String>) {
        self.inner.participant_5 = value;
    }

    /// Tracking mode (e.g., SEQUENTIAL, SINGLE_DIFF).
    /// :type: Optional[str]
    #[getter]
    fn get_mode(&self) -> Option<String> {
        self.inner.mode.clone()
    }
    #[setter]
    fn set_mode(&mut self, value: Option<String>) {
        self.inner.mode = value;
    }

    /// Signal path (comma-separated participant indices). Used with SEQUENTIAL mode.
    /// :type: Optional[str]
    #[getter]
    fn get_path(&self) -> Option<String> {
        self.inner.path.clone()
    }
    #[setter]
    fn set_path(&mut self, value: Option<String>) {
        self.inner.path = value;
    }

    /// First signal path for SINGLE_DIFF mode.
    /// :type: Optional[str]
    #[getter]
    fn get_path_1(&self) -> Option<String> {
        self.inner.path_1.clone()
    }
    #[setter]
    fn set_path_1(&mut self, value: Option<String>) {
        self.inner.path_1 = value;
    }

    /// Second signal path for SINGLE_DIFF mode.
    /// :type: Optional[str]
    #[getter]
    fn get_path_2(&self) -> Option<String> {
        self.inner.path_2.clone()
    }
    #[setter]
    fn set_path_2(&mut self, value: Option<String>) {
        self.inner.path_2 = value;
    }

    /// Ephemeris name for Participant 1.
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_1(&self) -> Option<String> {
        self.inner.ephemeris_name_1.clone()
    }
    #[setter]
    fn set_ephemeris_name_1(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_1 = value;
    }

    /// Ephemeris name for Participant 2.
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_2(&self) -> Option<String> {
        self.inner.ephemeris_name_2.clone()
    }
    #[setter]
    fn set_ephemeris_name_2(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_2 = value;
    }

    /// Ephemeris name for Participant 3.
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_3(&self) -> Option<String> {
        self.inner.ephemeris_name_3.clone()
    }
    #[setter]
    fn set_ephemeris_name_3(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_3 = value;
    }

    /// Ephemeris name for Participant 4.
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_4(&self) -> Option<String> {
        self.inner.ephemeris_name_4.clone()
    }
    #[setter]
    fn set_ephemeris_name_4(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_4 = value;
    }

    /// Ephemeris name for Participant 5.
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_5(&self) -> Option<String> {
        self.inner.ephemeris_name_5.clone()
    }
    #[setter]
    fn set_ephemeris_name_5(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_5 = value;
    }

    /// Frequency band for transmitted frequencies (e.g., S, X, Ka).
    /// :type: Optional[str]
    #[getter]
    fn get_transmit_band(&self) -> Option<String> {
        self.inner.transmit_band.clone()
    }
    #[setter]
    fn set_transmit_band(&mut self, value: Option<String>) {
        self.inner.transmit_band = value;
    }

    /// Frequency band for received frequencies.
    /// :type: Optional[str]
    #[getter]
    fn get_receive_band(&self) -> Option<String> {
        self.inner.receive_band.clone()
    }
    #[setter]
    fn set_receive_band(&mut self, value: Option<String>) {
        self.inner.receive_band = value;
    }

    /// Numerator of the turnaround ratio.
    /// :type: Optional[int]
    #[getter]
    fn get_turnaround_numerator(&self) -> Option<i32> {
        self.inner.turnaround_numerator
    }
    #[setter]
    fn set_turnaround_numerator(&mut self, value: Option<i32>) {
        self.inner.turnaround_numerator = value;
    }

    /// Denominator of the turnaround ratio.
    /// :type: Optional[int]
    #[getter]
    fn get_turnaround_denominator(&self) -> Option<i32> {
        self.inner.turnaround_denominator
    }
    #[setter]
    fn set_turnaround_denominator(&mut self, value: Option<i32>) {
        self.inner.turnaround_denominator = value;
    }

    /// Reference for time tags (TRANSMIT or RECEIVE).
    /// :type: Optional[str]
    #[getter]
    fn get_timetag_ref(&self) -> Option<String> {
        self.inner.timetag_ref.clone()
    }
    #[setter]
    fn set_timetag_ref(&mut self, value: Option<String>) {
        self.inner.timetag_ref = value;
    }

    /// Doppler count time in seconds.
    /// :type: Optional[float]
    #[getter]
    fn get_integration_interval(&self) -> Option<f64> {
        self.inner.integration_interval
    }
    #[setter]
    fn set_integration_interval(&mut self, value: Option<f64>) {
        self.inner.integration_interval = value;
    }

    /// Relationship between INTEGRATION_INTERVAL and timetag (START, MIDDLE, END).
    /// :type: Optional[str]
    #[getter]
    fn get_integration_ref(&self) -> Option<String> {
        self.inner.integration_ref.clone()
    }
    #[setter]
    fn set_integration_ref(&mut self, value: Option<String>) {
        self.inner.integration_ref = value;
    }

    /// Frequency offset in Hz.
    /// :type: Optional[float]
    #[getter]
    fn get_freq_offset(&self) -> Option<f64> {
        self.inner.freq_offset
    }
    #[setter]
    fn set_freq_offset(&mut self, value: Option<f64>) {
        self.inner.freq_offset = value;
    }

    /// Range mode (COHERENT, CONSTANT, ONE_WAY).
    /// :type: Optional[str]
    #[getter]
    fn get_range_mode(&self) -> Option<String> {
        self.inner.range_mode.clone()
    }
    #[setter]
    fn set_range_mode(&mut self, value: Option<String>) {
        self.inner.range_mode = value;
    }

    /// Modulus of the range observable.
    /// :type: Optional[float]
    #[getter]
    fn get_range_modulus(&self) -> Option<f64> {
        self.inner.range_modulus
    }
    #[setter]
    fn set_range_modulus(&mut self, value: Option<f64>) {
        self.inner.range_modulus = value;
    }

    /// Units for the range observable (km, s, RU).
    /// :type: Optional[str]
    #[getter]
    fn get_range_units(&self) -> Option<String> {
        self.inner.range_units.clone()
    }
    #[setter]
    fn set_range_units(&mut self, value: Option<String>) {
        self.inner.range_units = value;
    }

    /// Type of antenna geometry (e.g., AZEL, RADEC).
    /// :type: Optional[str]
    #[getter]
    fn get_angle_type(&self) -> Option<String> {
        self.inner.angle_type.clone()
    }
    #[setter]
    fn set_angle_type(&mut self, value: Option<String>) {
        self.inner.angle_type = value;
    }

    /// Inertial reference frame (e.g., EME2000). Applies if ANGLE_TYPE is RADEC.
    /// :type: Optional[str]
    #[getter]
    fn get_reference_frame(&self) -> Option<String> {
        self.inner.reference_frame.clone()
    }
    #[setter]
    fn set_reference_frame(&mut self, value: Option<String>) {
        self.inner.reference_frame = value;
    }

    /// Interpolation method (e.g., HERMITE, LAGRANGE).
    /// :type: Optional[str]
    #[getter]
    fn get_interpolation(&self) -> Option<String> {
        self.inner.interpolation.clone()
    }
    #[setter]
    fn set_interpolation(&mut self, value: Option<String>) {
        self.inner.interpolation = value;
    }

    /// Degree of interpolation polynomial.
    /// :type: Optional[int]
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree
    }
    #[setter]
    fn set_interpolation_degree(&mut self, value: Option<u32>) {
        self.inner.interpolation_degree = value;
    }

    /// Bias for Doppler count (Hz).
    /// :type: Optional[float]
    #[getter]
    fn get_doppler_count_bias(&self) -> Option<f64> {
        self.inner.doppler_count_bias
    }
    #[setter]
    fn set_doppler_count_bias(&mut self, value: Option<f64>) {
        self.inner.doppler_count_bias = value;
    }

    /// Scale factor for Doppler count.
    /// :type: Optional[int]
    #[getter]
    fn get_doppler_count_scale(&self) -> Option<u64> {
        self.inner.doppler_count_scale
    }
    #[setter]
    fn set_doppler_count_scale(&mut self, value: Option<u64>) {
        self.inner.doppler_count_scale = value;
    }

    /// Indicator of Doppler count rollover (YES, NO).
    /// :type: Optional[str]
    #[getter]
    fn get_doppler_count_rollover(&self) -> Option<String> {
        self.inner.doppler_count_rollover.clone()
    }
    #[setter]
    fn set_doppler_count_rollover(&mut self, value: Option<String>) {
        self.inner.doppler_count_rollover = value;
    }

    /// Transmit delay for Participant 1 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_1(&self) -> Option<f64> {
        self.inner.transmit_delay_1
    }
    #[setter]
    fn set_transmit_delay_1(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_1 = value;
    }

    /// Transmit delay for Participant 2 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_2(&self) -> Option<f64> {
        self.inner.transmit_delay_2
    }
    #[setter]
    fn set_transmit_delay_2(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_2 = value;
    }

    /// Transmit delay for Participant 3 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_3(&self) -> Option<f64> {
        self.inner.transmit_delay_3
    }
    #[setter]
    fn set_transmit_delay_3(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_3 = value;
    }

    /// Transmit delay for Participant 4 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_4(&self) -> Option<f64> {
        self.inner.transmit_delay_4
    }
    #[setter]
    fn set_transmit_delay_4(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_4 = value;
    }

    /// Transmit delay for Participant 5 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_5(&self) -> Option<f64> {
        self.inner.transmit_delay_5
    }
    #[setter]
    fn set_transmit_delay_5(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_5 = value;
    }

    /// Receive delay for Participant 1 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_1(&self) -> Option<f64> {
        self.inner.receive_delay_1
    }
    #[setter]
    fn set_receive_delay_1(&mut self, value: Option<f64>) {
        self.inner.receive_delay_1 = value;
    }

    /// Receive delay for Participant 2 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_2(&self) -> Option<f64> {
        self.inner.receive_delay_2
    }
    #[setter]
    fn set_receive_delay_2(&mut self, value: Option<f64>) {
        self.inner.receive_delay_2 = value;
    }

    /// Receive delay for Participant 3 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_3(&self) -> Option<f64> {
        self.inner.receive_delay_3
    }
    #[setter]
    fn set_receive_delay_3(&mut self, value: Option<f64>) {
        self.inner.receive_delay_3 = value;
    }

    /// Receive delay for Participant 4 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_4(&self) -> Option<f64> {
        self.inner.receive_delay_4
    }
    #[setter]
    fn set_receive_delay_4(&mut self, value: Option<f64>) {
        self.inner.receive_delay_4 = value;
    }

    /// Receive delay for Participant 5 (seconds).
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_5(&self) -> Option<f64> {
        self.inner.receive_delay_5
    }
    #[setter]
    fn set_receive_delay_5(&mut self, value: Option<f64>) {
        self.inner.receive_delay_5 = value;
    }

    /// Data quality (RAW, VALIDATED, DEGRADED).
    /// :type: Optional[str]
    #[getter]
    fn get_data_quality(&self) -> Option<String> {
        self.inner.data_quality.clone()
    }
    #[setter]
    fn set_data_quality(&mut self, value: Option<String>) {
        self.inner.data_quality = value;
    }

    /// Angle 1 correction (degrees).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_angle_1(&self) -> Option<f64> {
        self.inner.correction_angle_1
    }
    #[setter]
    fn set_correction_angle_1(&mut self, value: Option<f64>) {
        self.inner.correction_angle_1 = value;
    }

    /// Angle 2 correction (degrees).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_angle_2(&self) -> Option<f64> {
        self.inner.correction_angle_2
    }
    #[setter]
    fn set_correction_angle_2(&mut self, value: Option<f64>) {
        self.inner.correction_angle_2 = value;
    }

    /// Doppler correction (km/s).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_doppler(&self) -> Option<f64> {
        self.inner.correction_doppler
    }
    #[setter]
    fn set_correction_doppler(&mut self, value: Option<f64>) {
        self.inner.correction_doppler = value;
    }

    /// Magnitude correction.
    /// :type: Optional[float]
    #[getter]
    fn get_correction_mag(&self) -> Option<f64> {
        self.inner.correction_mag
    }
    #[setter]
    fn set_correction_mag(&mut self, value: Option<f64>) {
        self.inner.correction_mag = value;
    }

    /// Range correction (Range Units, km, or s).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_range(&self) -> Option<f64> {
        self.inner.correction_range
    }
    #[setter]
    fn set_correction_range(&mut self, value: Option<f64>) {
        self.inner.correction_range = value;
    }

    /// RCS correction (m²).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_rcs(&self) -> Option<f64> {
        self.inner.correction_rcs
    }
    #[setter]
    fn set_correction_rcs(&mut self, value: Option<f64>) {
        self.inner.correction_rcs = value;
    }

    /// Receive correction (Hz).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_receive(&self) -> Option<f64> {
        self.inner.correction_receive
    }
    #[setter]
    fn set_correction_receive(&mut self, value: Option<f64>) {
        self.inner.correction_receive = value;
    }

    /// Transmit correction (Hz).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_transmit(&self) -> Option<f64> {
        self.inner.correction_transmit
    }
    #[setter]
    fn set_correction_transmit(&mut self, value: Option<f64>) {
        self.inner.correction_transmit = value;
    }

    /// Yearly aberration correction (deg).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_aberration_yearly(&self) -> Option<f64> {
        self.inner.correction_aberration_yearly
    }
    #[setter]
    fn set_correction_aberration_yearly(&mut self, value: Option<f64>) {
        self.inner.correction_aberration_yearly = value;
    }

    /// Diurnal aberration correction (deg).
    /// :type: Optional[float]
    #[getter]
    fn get_correction_aberration_diurnal(&self) -> Option<f64> {
        self.inner.correction_aberration_diurnal
    }
    #[setter]
    fn set_correction_aberration_diurnal(&mut self, value: Option<f64>) {
        self.inner.correction_aberration_diurnal = value;
    }

    /// Indication if corrections have been applied (YES, NO).
    /// :type: Optional[str]
    #[getter]
    fn get_corrections_applied(&self) -> Option<String> {
        self.inner.corrections_applied.clone()
    }
    #[setter]
    fn set_corrections_applied(&mut self, value: Option<String>) {
        self.inner.corrections_applied = value;
    }
}

// ============================================================================
// TDM Data
// ============================================================================

/// Represents the Data Section of a TDM Segment.
///
/// Contains one or more Tracking Data Records.
///
/// Parameters
/// ----------
/// observations : list[TdmObservation], optional
///     List of tracking data records.
///     (Optional)
/// comment : list[str], optional
///     Comments in the data section.
///     (Optional)
#[pyclass]
#[derive(Clone)]
pub struct TdmData {
    pub inner: core_tdm::TdmData,
}

#[pymethods]
impl TdmData {
    #[new]
    #[pyo3(signature = (*, observations=None, comment=None))]
    fn new(observations: Option<Vec<TdmObservation>>, comment: Option<Vec<String>>) -> Self {
        Self {
            inner: core_tdm::TdmData {
                comment: comment.unwrap_or_default(),
                observations: observations
                    .map(|obs| obs.into_iter().map(|o| o.inner).collect())
                    .unwrap_or_default(),
            },
        }
    }

    fn __repr__(&self) -> String {
        format!("TdmData(observations={})", self.inner.observations.len())
    }

    /// List of tracking observations.
    ///
    /// :type: List[TdmObservation]
    #[getter]
    fn get_observations(&self) -> Vec<TdmObservation> {
        self.inner
            .observations
            .iter()
            .map(|o| TdmObservation { inner: o.clone() })
            .collect()
    }

    #[setter]
    fn set_observations(&mut self, value: Vec<TdmObservation>) {
        self.inner.observations = value.into_iter().map(|o| o.inner).collect();
    }

    /// Count of observations.
    ///
    /// :type: int
    #[getter]
    fn get_observation_count(&self) -> usize {
        self.inner.observations.len()
    }

    /// Comments.
    ///
    /// :type: List[str]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }
}

// ============================================================================
// TDM Observation
// ============================================================================

/// Represents a single Tracking Data Record (TDR).
///
/// A TDR consists of a keyword (data type), a timetag (epoch), and a measurement.
///
/// Parameters
/// ----------
/// epoch : str
///     Time associated with the tracking observable.
/// keyword : str
///     Data type keyword (e.g., "RANGE", "RECEIVE_FREQ").
/// value : float
///     Tracking observable value. Note: For phase counts that require full precision strings,
///     use internal representation handling (this constructor takes float for simplicity,
///     but the object can hold string representations internally).
#[pyclass]
#[derive(Clone)]
pub struct TdmObservation {
    pub inner: core_tdm::TdmObservation,
}

#[pymethods]
impl TdmObservation {
    #[new]
    #[pyo3(signature = (*, epoch, keyword, value))]
    fn new(epoch: String, keyword: String, value: f64) -> PyResult<Self> {
        use core_tdm::TdmObservationData;

        // Parse the keyword to get the correct observation type
        let data = match keyword.as_str() {
            "RANGE" => TdmObservationData::Range(value),
            "DOPPLER_COUNT" => TdmObservationData::DopplerCount(value),
            "DOPPLER_INSTANTANEOUS" => TdmObservationData::DopplerInstantaneous(value),
            "DOPPLER_INTEGRATED" => TdmObservationData::DopplerIntegrated(value),
            "CARRIER_POWER" => TdmObservationData::CarrierPower(value),
            "PC_N0" => TdmObservationData::PcN0(value),
            "PR_N0" => TdmObservationData::PrN0(value),
            "RECEIVE_FREQ" => TdmObservationData::ReceiveFreq(value),
            "RECEIVE_FREQ_1" => TdmObservationData::ReceiveFreq1(value),
            "RECEIVE_FREQ_2" => TdmObservationData::ReceiveFreq2(value),
            "RECEIVE_FREQ_3" => TdmObservationData::ReceiveFreq3(value),
            "RECEIVE_FREQ_4" => TdmObservationData::ReceiveFreq4(value),
            "RECEIVE_FREQ_5" => TdmObservationData::ReceiveFreq5(value),
            "TRANSMIT_FREQ_1" => TdmObservationData::TransmitFreq1(value),
            "TRANSMIT_FREQ_2" => TdmObservationData::TransmitFreq2(value),
            "TRANSMIT_FREQ_3" => TdmObservationData::TransmitFreq3(value),
            "TRANSMIT_FREQ_4" => TdmObservationData::TransmitFreq4(value),
            "TRANSMIT_FREQ_5" => TdmObservationData::TransmitFreq5(value),
            "TRANSMIT_FREQ_RATE_1" => TdmObservationData::TransmitFreqRate1(value),
            "TRANSMIT_FREQ_RATE_2" => TdmObservationData::TransmitFreqRate2(value),
            "TRANSMIT_FREQ_RATE_3" => TdmObservationData::TransmitFreqRate3(value),
            "TRANSMIT_FREQ_RATE_4" => TdmObservationData::TransmitFreqRate4(value),
            "TRANSMIT_FREQ_RATE_5" => TdmObservationData::TransmitFreqRate5(value),
            "ANGLE_1" => TdmObservationData::Angle1(value),
            "ANGLE_2" => TdmObservationData::Angle2(value),
            "VLBI_DELAY" => TdmObservationData::VlbiDelay(value),
            "CLOCK_BIAS" => TdmObservationData::ClockBias(value),
            "CLOCK_DRIFT" => TdmObservationData::ClockDrift(value),
            "PRESSURE" => TdmObservationData::Pressure(value),
            "RHUMIDITY" => {
                TdmObservationData::Rhumidity(ccsds_ndm::types::Percentage { value, units: None })
            }
            "TEMPERATURE" => TdmObservationData::Temperature(value),
            "TROPO_DRY" => TdmObservationData::TropoDry(value),
            "TROPO_WET" => TdmObservationData::TropoWet(value),
            "STEC" => TdmObservationData::Stec(value),
            "MAG" => TdmObservationData::Mag(value),
            "RCS" => TdmObservationData::Rcs(value),
            "DOR" => TdmObservationData::Dor(value),
            "RECEIVE_PHASE_CT_1" => TdmObservationData::ReceivePhaseCt1(value.to_string()),
            "RECEIVE_PHASE_CT_2" => TdmObservationData::ReceivePhaseCt2(value.to_string()),
            "RECEIVE_PHASE_CT_3" => TdmObservationData::ReceivePhaseCt3(value.to_string()),
            "RECEIVE_PHASE_CT_4" => TdmObservationData::ReceivePhaseCt4(value.to_string()),
            "RECEIVE_PHASE_CT_5" => TdmObservationData::ReceivePhaseCt5(value.to_string()),
            "TRANSMIT_PHASE_CT_1" => TdmObservationData::TransmitPhaseCt1(value.to_string()),
            "TRANSMIT_PHASE_CT_2" => TdmObservationData::TransmitPhaseCt2(value.to_string()),
            "TRANSMIT_PHASE_CT_3" => TdmObservationData::TransmitPhaseCt3(value.to_string()),
            "TRANSMIT_PHASE_CT_4" => TdmObservationData::TransmitPhaseCt4(value.to_string()),
            "TRANSMIT_PHASE_CT_5" => TdmObservationData::TransmitPhaseCt5(value.to_string()),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown observation keyword: {}",
                    keyword
                )))
            }
        };

        Ok(Self {
            inner: core_tdm::TdmObservation {
                epoch: parse_epoch(&epoch)?,
                data,
            },
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "TdmObservation(epoch='{}', keyword='{}', value={})",
            self.inner.epoch,
            self.inner.data.key(),
            self.inner.data.value_to_string()
        )
    }

    /// Epoch of the observation.
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

    /// Keyword of the observation (e.g., "RANGE").
    ///
    /// :type: str
    #[getter]
    fn get_keyword(&self) -> String {
        self.inner.data.key().to_string()
    }

    /// Measurement value as float.
    ///
    /// Returns None if the value is not representable as a float (unlikely for TDM).
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_value(&self) -> Option<f64> {
        self.inner.data.value_to_string().parse::<f64>().ok()
    }

    /// Measurement value as string.
    ///
    /// Useful for phase counts which may require high precision.
    ///
    /// :type: str
    #[getter]
    fn get_value_str(&self) -> String {
        self.inner.data.value_to_string()
    }
}