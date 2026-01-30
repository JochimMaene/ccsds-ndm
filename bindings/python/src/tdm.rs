// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::parse_epoch;
use ccsds_ndm::messages::tdm as core_tdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{self as core_types};
use ccsds_ndm::MessageType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::common::{parse_time_system, parse_yes_no};
use std::fs;
use std::str::FromStr;

// ============================================================================
// TDM - Tracking Data Message
// ============================================================================

/// Tracking Data Message (TDM).
///
/// The TDM specifies a standard message format for use in exchanging spacecraft tracking data
/// between space agencies. Such exchanges are used for distributing tracking data output from
/// routine interagency cross-supports.
///
/// Tracking data includes data types such as:
/// - Doppler
/// - Transmit/Received frequencies
/// - Range
/// - Angles
/// - Delta-DOR
/// - Media correction (ionosphere, troposphere)
/// - Meteorological data
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

    /// Tracking Data Message (TDM).
    ///
    /// The TDM specifies a standard message format for use in exchanging spacecraft tracking data
    /// between space agencies. Such exchanges are used for distributing tracking data output from
    /// routine interagency cross-supports.
    ///
    /// Tracking data includes data types such as:
    /// - Doppler
    /// - Transmit/Received frequencies
    /// - Range
    /// - Angles
    /// - Delta-DOR
    /// - Media correction (ionosphere, troposphere)
    /// - Meteorological data
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
    /// :type: list[TdmSegment]
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
        let inner =
            match format {
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
            Err(e) => Err(PyValueError::new_err(format!(
                "Failed to write file: {}",
                e
            ))),
        }
    }
}

// ============================================================================
// TDM Header
// ============================================================================

/// Represents the `tdmHeader` complex type.
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

    /// Creating agency. Value should be an entry from the ‘Abbreviation’ column in the SANA
    /// Organizations Registry, <https://sanaregistry.org/r/organizations/organizations.html>
    /// (reference `[11]`).
    ///
    /// Examples: CNES, ESA, GSFC, DLR, JPL, JAXA
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

    /// Data creation date/time in UTC. (For format specification, see 4.3.9.)
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23.4, 2006-001T00:00:00Z
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

    /// ID that uniquely identifies a message from a given originator. The format and content
    /// of the message identifier value are at the discretion of the originator.
    ///
    /// Examples: 201113719185
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

    /// Comments (allowed in the TDM Header only immediately after the TDM version number).
    /// (See 4.5 for formatting rules.)
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

// ============================================================================
// TDM Body
// ============================================================================

/// The TDM Body consists of one or more TDM Segments.
///
/// Parameters
/// ----------
/// segments : list[TdmSegment]
///     List of data segments.
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
    /// :type: list[TdmSegment]
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

    /// Metadata section for this TDM segment.
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

    /// Data section for this TDM segment.
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
        participant_1,
        time_system=None,
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
        participant_1: String,
        time_system: Option<Bound<'_, PyAny>>,
        track_id: Option<String>,

        data_types: Option<String>,
        start_time: Option<String>,
        stop_time: Option<String>,
        participant_2: Option<String>,
        participant_3: Option<String>,
        participant_4: Option<String>,
        participant_5: Option<String>,
        mode: Option<Bound<'_, PyAny>>,
        path: Option<Bound<'_, PyAny>>,

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
        corrections_applied: Option<Bound<'_, PyAny>>,
        ephemeris_name_1: Option<String>,
        ephemeris_name_2: Option<String>,
        ephemeris_name_3: Option<String>,
        ephemeris_name_4: Option<String>,
        ephemeris_name_5: Option<String>,
        comment: Option<Vec<String>>,
    ) -> PyResult<Self> {
        use std::str::FromStr;

        let time_system = match time_system {
            Some(ref ob) => parse_time_system(ob)?,
            None => "UTC".to_string(),
        };

        let mode = match mode {
            Some(ref ob) => Some(parse_tdm_mode(ob)?),
            None => None,
        };

        let path = match path {
            Some(ref ob) => Some(parse_tdm_path(ob)?),
            None => None,
        };

        let corrections_applied = match corrections_applied {
            Some(ref ob) => Some(parse_yes_no(ob)?),
            None => None,
        };

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
                path_1: path_1
                    .map(|s| core_types::TdmPath::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
                    .transpose()?,
                path_2: path_2
                    .map(|s| core_types::TdmPath::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
                    .transpose()?,
                transmit_band,
                receive_band,
                turnaround_numerator,
                turnaround_denominator,
                timetag_ref: timetag_ref
                    .map(|s| {
                        core_types::TdmTimetagRef::from_str(&s)
                            .map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                integration_interval,
                integration_ref: integration_ref
                    .map(|s| {
                        core_types::TdmIntegrationRef::from_str(&s)
                            .map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                freq_offset,
                range_mode: range_mode
                    .map(|s| {
                        core_types::TdmRangeMode::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                range_modulus,
                range_units: range_units
                    .map(|s| {
                        core_types::TdmRangeUnits::from_str(&s)
                            .map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                angle_type: angle_type
                    .map(|s| {
                        core_types::TdmAngleType::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                reference_frame: reference_frame
                    .map(|s| {
                        core_types::TdmReferenceFrame::from_str(&s)
                            .map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
                interpolation,
                interpolation_degree,
                doppler_count_bias,
                doppler_count_scale,
                doppler_count_rollover: doppler_count_rollover
                    .map(|s| core_types::YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
                    .transpose()?,
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
                data_quality: data_quality
                    .map(|s| {
                        core_types::TdmDataQuality::from_str(&s)
                            .map_err(|e| PyValueError::new_err(e.to_string()))
                    })
                    .transpose()?,
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

    /// The TRACK_ID keyword specifies a unique identifier for the tracking data in the
    /// associated data section. The value may be a freely selected string of characters and
    /// numbers, only required to be unique for each track of the corresponding sensor. For
    /// example, the value may be constructed from the measurement date and time and a counter
    /// to distinguish simultaneously tracked objects.
    ///
    /// Examples: 20190918_1200135-0001
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_track_id(&self) -> Option<String> {
        self.inner.track_id.clone()
    }
    #[setter]
    fn set_track_id(&mut self, value: Option<String>) {
        self.inner.track_id = value;
    }

    /// Comma-separated list of data types in the Data Section. The elements of the list shall
    /// be selected from the data types shown in table 3-5, with the exception of the
    /// DATA_START, DATA_STOP, and COMMENT keywords.
    ///
    /// Examples: RANGE, TRANSMIT_FREQ_n, RECEIVE_FREQ
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_data_types(&self) -> Option<String> {
        self.inner.data_types.clone()
    }
    #[setter]
    fn set_data_types(&mut self, value: Option<String>) {
        self.inner.data_types = value;
    }

    /// The TIME_SYSTEM keyword shall specify the time system used for timetags in the
    /// associated Data Section. This should be UTC for ground-based data. The value associated
    /// with this keyword must be selected from the full set of allowed values enumerated in
    /// the SANA Time Systems Registry <https://sanaregistry.org/r/time_systems> (reference `[12]`).
    /// (See annex B.)
    ///
    /// Examples: UTC, TAI, GPS, SCLK
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

    /// The START_TIME keyword shall specify the UTC start time of the total time span covered
    /// by the tracking data immediately following this Metadata Section. (For format
    /// specification, see 4.3.9.)
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
    ///
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

    /// The STOP_TIME keyword shall specify the UTC stop time of the total time span covered by
    /// the tracking data immediately following this Metadata Section. (For format
    /// specification, see 4.3.9.)
    ///
    /// Examples: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
    ///
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

    /// The PARTICIPANT_n keyword shall represent the participants (see 1.3.4.1) in a tracking
    /// data session. It is indexed to allow unambiguous reference to other data in the TDM
    /// (max index is 5). At least two participants must be specified for most sessions; for
    /// some special TDMs such as tropospheric media, only one participant need be listed.
    ///
    /// Examples: DSS-63-S400K, ROSETTA, `<Quasar catalog name>`, 1997-061A, UNKNOWN
    ///
    /// :type: str
    #[getter]
    fn get_participant_1(&self) -> String {
        self.inner.participant_1.clone()
    }
    #[setter]
    fn set_participant_1(&mut self, value: String) {
        self.inner.participant_1 = value;
    }

    /// The second participant in a tracking data session.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_participant_2(&self) -> Option<String> {
        self.inner.participant_2.clone()
    }
    #[setter]
    fn set_participant_2(&mut self, value: Option<String>) {
        self.inner.participant_2 = value;
    }

    /// The third participant in a tracking data session.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_participant_3(&self) -> Option<String> {
        self.inner.participant_3.clone()
    }
    #[setter]
    fn set_participant_3(&mut self, value: Option<String>) {
        self.inner.participant_3 = value;
    }

    /// The fourth participant in a tracking data session.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_participant_4(&self) -> Option<String> {
        self.inner.participant_4.clone()
    }
    #[setter]
    fn set_participant_4(&mut self, value: Option<String>) {
        self.inner.participant_4 = value;
    }

    /// The fifth participant in a tracking data session.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_participant_5(&self) -> Option<String> {
        self.inner.participant_5.clone()
    }
    #[setter]
    fn set_participant_5(&mut self, value: Option<String>) {
        self.inner.participant_5 = value;
    }

    /// The MODE keyword shall reflect the tracking mode associated with the Data Section of
    /// the segment. The value ‘SEQUENTIAL’ applies for most sequential signal paths; the name
    /// implies a sequential signal path between tracking participants. The value
    /// ‘SINGLE_DIFF’ applies only for differenced data.
    ///
    /// Examples: SEQUENTIAL, SINGLE_DIFF
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_mode(&self) -> Option<String> {
        self.inner.mode.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_mode(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmMode;
        use std::str::FromStr;
        self.inner.mode = value
            .map(|s| TdmMode::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }

    /// The PATH keywords shall reflect the signal path by listing the index of each participant
    /// in order, separated by commas, with no inserted white space. Correlated with the
    /// indices of the PARTICIPANT_n keywords. The first entry in the PATH shall be the
    /// transmit participant.
    ///
    /// Examples: PATH = 1,2,1, PATH_1 = 1,2,1, PATH_2 = 3,1
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_path(&self) -> Option<String> {
        self.inner.path.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_path(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmPath;
        use std::str::FromStr;
        self.inner.path = value
            .map(|s| TdmPath::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }

    /// The first signal path where the MODE is 'SINGLE_DIFF'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_path_1(&self) -> Option<String> {
        self.inner.path_1.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_path_1(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmPath;
        use std::str::FromStr;
        self.inner.path_1 = value
            .map(|s| TdmPath::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }

    /// The second signal path where the MODE is 'SINGLE_DIFF'.
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_path_2(&self) -> Option<String> {
        self.inner.path_2.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_path_2(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmPath;
        use std::str::FromStr;
        self.inner.path_2 = value
            .map(|s| TdmPath::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }

    /// Unique name of the external ephemeris file used for participant 1.
    ///
    /// Examples: SATELLITE_A_EPHEM27
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_1(&self) -> Option<String> {
        self.inner.ephemeris_name_1.clone()
    }
    #[setter]
    fn set_ephemeris_name_1(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_1 = value;
    }

    /// Unique name of the external ephemeris file used for participant 2.
    ///
    /// Examples: SATELLITE_A_EPHEM27
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_2(&self) -> Option<String> {
        self.inner.ephemeris_name_2.clone()
    }
    #[setter]
    fn set_ephemeris_name_2(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_2 = value;
    }

    /// Unique name of the external ephemeris file used for participant 3.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_3(&self) -> Option<String> {
        self.inner.ephemeris_name_3.clone()
    }
    #[setter]
    fn set_ephemeris_name_3(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_3 = value;
    }

    /// Unique name of the external ephemeris file used for participant 4.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_4(&self) -> Option<String> {
        self.inner.ephemeris_name_4.clone()
    }
    #[setter]
    fn set_ephemeris_name_4(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_4 = value;
    }

    /// Unique name of the external ephemeris file used for participant 5.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_ephemeris_name_5(&self) -> Option<String> {
        self.inner.ephemeris_name_5.clone()
    }
    #[setter]
    fn set_ephemeris_name_5(&mut self, value: Option<String>) {
        self.inner.ephemeris_name_5 = value;
    }

    /// The TRANSMIT_BAND keyword shall indicate the frequency band for transmitted
    /// frequencies. The frequency ranges associated with each band should be specified in the
    /// ICD.
    ///
    /// Examples: S, X, Ka, L, UHF, GREEN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_transmit_band(&self) -> Option<String> {
        self.inner.transmit_band.clone()
    }
    #[setter]
    fn set_transmit_band(&mut self, value: Option<String>) {
        self.inner.transmit_band = value;
    }

    /// The RECEIVE_BAND keyword shall indicate the frequency band for received frequencies.
    /// Although not required in general, the RECEIVE_BAND must be present if the MODE is
    /// SINGLE_DIFF and differenced frequencies or differenced range are provided in order to
    /// allow proper frequency dependent corrections to be applied.
    ///
    /// Examples: S, X, Ka, L, UHF, GREEN
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_receive_band(&self) -> Option<String> {
        self.inner.receive_band.clone()
    }
    #[setter]
    fn set_receive_band(&mut self, value: Option<String>) {
        self.inner.receive_band = value;
    }

    /// The TURNAROUND_NUMERATOR keyword shall indicate the numerator of the turnaround ratio
    /// that is necessary to calculate the coherent downlink from the uplink frequency.
    ///
    /// Examples: 240, 880
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_turnaround_numerator(&self) -> Option<i32> {
        self.inner.turnaround_numerator
    }
    #[setter]
    fn set_turnaround_numerator(&mut self, value: Option<i32>) {
        self.inner.turnaround_numerator = value;
    }

    /// The TURNAROUND_DENOMINATOR keyword shall indicate the denominator of the turnaround
    /// ratio that is necessary to calculate the coherent downlink from the uplink frequency.
    ///
    /// Examples: 221, 749
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_turnaround_denominator(&self) -> Option<i32> {
        self.inner.turnaround_denominator
    }
    #[setter]
    fn set_turnaround_denominator(&mut self, value: Option<i32>) {
        self.inner.turnaround_denominator = value;
    }

    /// The TIMETAG_REF keyword shall provide a reference for time tags in the tracking data.
    /// This keyword indicates whether the timetag associated with the data is the transmit
    /// time or the receive time.
    ///
    /// Examples: TRANSMIT, RECEIVE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_timetag_ref(&self) -> Option<String> {
        self.inner.timetag_ref.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_timetag_ref(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmTimetagRef;
        use std::str::FromStr;
        self.inner.timetag_ref = value
            .map(|s| TdmTimetagRef::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }

    /// The INTEGRATION_INTERVAL keyword shall provide the Doppler count time in seconds for
    /// Doppler data or for the creation of normal points.
    ///
    /// Examples: 60.0, 0.1, 1.0
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_integration_interval(&self) -> Option<f64> {
        self.inner.integration_interval
    }
    #[setter]
    fn set_integration_interval(&mut self, value: Option<f64>) {
        self.inner.integration_interval = value;
    }

    /// Indicates the relationship between the INTEGRATION_INTERVAL and the timetag on the
    /// data, i.e., whether the timetag represents the start, middle, or end of the integration
    /// period.
    ///
    /// Examples: START, MIDDLE, END
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_integration_ref(&self) -> Option<String> {
        self.inner.integration_ref.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_integration_ref(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmIntegrationRef;
        use std::str::FromStr;
        self.inner.integration_ref = value
            .map(|s| {
                TdmIntegrationRef::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The FREQ_OFFSET keyword represents a frequency in Hz that must be added to every
    /// RECEIVE_FREQ to reconstruct it. One use is if a Doppler shift frequency observable is
    /// transferred instead of the actual received frequency. The default shall be 0.0.
    ///
    /// Examples: 0.0, 8415000000.0
    ///
    /// Units: Hz
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_freq_offset(&self) -> Option<f64> {
        self.inner.freq_offset
    }
    #[setter]
    fn set_freq_offset(&mut self, value: Option<f64>) {
        self.inner.freq_offset = value;
    }

    /// The value of the RANGE_MODE keyword shall be ‘COHERENT’, in which case the range tones
    /// are coherent with the uplink carrier; ‘CONSTANT’, in which case the range tones have a
    /// constant frequency; or ‘ONE_WAY’ (used in Delta-DOR).
    ///
    /// Examples: COHERENT, CONSTANT, ONE_WAY
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_range_mode(&self) -> Option<String> {
        self.inner.range_mode.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_range_mode(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmRangeMode;
        use std::str::FromStr;
        self.inner.range_mode = value
            .map(|s| {
                TdmRangeMode::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The value associated with the RANGE_MODULUS keyword shall be the modulus of the range
    /// observable in the units as specified by the RANGE_UNITS keyword; that is, the actual
    /// (unambiguous) range is an integer k times the modulus, plus the observable value. The
    /// default value shall be 0.0.
    ///
    /// Examples: 32768.0, 2.0e+23, 0.0, 161.6484
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_range_modulus(&self) -> Option<f64> {
        self.inner.range_modulus
    }
    #[setter]
    fn set_range_modulus(&mut self, value: Option<f64>) {
        self.inner.range_modulus = value;
    }

    /// The RANGE_UNITS keyword specifies the units for the range observable. ‘km’ shall be
    /// used if the range is measured in kilometers. ‘s’ shall be used if the range is measured
    /// in seconds. ‘RU’, for ‘range units’, shall be used where the transmit frequency is
    /// changing. The default value shall be ‘km’.
    ///
    /// Examples: km, s, RU
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_range_units(&self) -> Option<String> {
        self.inner.range_units.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_range_units(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmRangeUnits;
        use std::str::FromStr;
        self.inner.range_units = value
            .map(|s| {
                TdmRangeUnits::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The ANGLE_TYPE keyword shall indicate the type of antenna geometry represented in the
    /// angle data (ANGLE_1 and ANGLE_2 keywords).
    ///
    /// Examples: AZEL, RADEC, XEYN, XSYE
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_angle_type(&self) -> Option<String> {
        self.inner.angle_type.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_angle_type(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmAngleType;
        use std::str::FromStr;
        self.inner.angle_type = value
            .map(|s| {
                TdmAngleType::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The REFERENCE_FRAME keyword shall be used in conjunction with the ‘ANGLE_TYPE=RADEC’
    /// keyword/value combination, indicating the inertial reference frame to which the antenna
    /// frame is referenced.
    ///
    /// Examples: EME2000, ICRF, ITRF1993, ITRF2000, TOD_EARTH
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_reference_frame(&self) -> Option<String> {
        self.inner.reference_frame.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_reference_frame(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmReferenceFrame;
        use std::str::FromStr;
        self.inner.reference_frame = value
            .map(|s| {
                TdmReferenceFrame::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The INTERPOLATION keyword shall specify the interpolation method to be used to calculate
    /// a transmit phase count at an arbitrary time in tracking data where the uplink frequency
    /// is not constant.
    ///
    /// Examples: HERMITE, LAGRANGE, LINEAR
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_interpolation(&self) -> Option<String> {
        self.inner.interpolation.clone()
    }
    #[setter]
    fn set_interpolation(&mut self, value: Option<String>) {
        self.inner.interpolation = value;
    }

    /// The INTERPOLATION_DEGREE keyword shall specify the recommended degree of the
    /// interpolating polynomial used to calculate a transmit phase count at an arbitrary time
    /// in tracking data where the uplink frequency is not constant.
    ///
    /// Examples: 3, 5, 7, 11
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_interpolation_degree(&self) -> Option<u32> {
        self.inner.interpolation_degree
    }
    #[setter]
    fn set_interpolation_degree(&mut self, value: Option<u32>) {
        self.inner.interpolation_degree = value;
    }

    /// Doppler counts are generally biased so as to accommodate negative Doppler within an
    /// accumulator. In order to reconstruct the measurement, the bias shall be subtracted from
    /// the DOPPLER_COUNT data value.
    ///
    /// Examples: 2.4e6, 240000000.0
    ///
    /// Units: Hz
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_doppler_count_bias(&self) -> Option<f64> {
        self.inner.doppler_count_bias
    }
    #[setter]
    fn set_doppler_count_bias(&mut self, value: Option<f64>) {
        self.inner.doppler_count_bias = value;
    }

    /// Doppler counts are generally scaled so as to capture partial cycles in an integer
    /// count. In order to reconstruct the measurement, the DOPPLER_COUNT data value shall be
    /// divided by the scale factor. The default shall be 1.
    ///
    /// Examples: 1000, 1
    ///
    /// :type: Optional[int]
    #[getter]
    fn get_doppler_count_scale(&self) -> Option<u64> {
        self.inner.doppler_count_scale
    }
    #[setter]
    fn set_doppler_count_scale(&mut self, value: Option<u64>) {
        self.inner.doppler_count_scale = value;
    }

    /// Doppler counts may overflow the accumulator and roll over in cases where the track is
    /// of long duration or very high Doppler shift. This flag indicates whether or not a
    /// counter rollover has occurred during the track.
    ///
    /// Examples: YES, NO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_doppler_count_rollover(&self) -> Option<String> {
        self.inner
            .doppler_count_rollover
            .as_ref()
            .map(|v| v.to_string())
    }
    #[setter]
    fn set_doppler_count_rollover(&mut self, value: Option<String>) -> PyResult<()> {
        use std::str::FromStr;
        self.inner.doppler_count_rollover = value
            .map(|s| core_types::YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }


    /// The TRANSMIT_DELAY_n keyword shall specify a fixed interval of time, in seconds,
    /// required for the signal to travel from the transmitting electronics to the transmit
    /// point. The default value shall be 0.0.
    ///
    /// Examples: 1.23, 0.0326, 0.00077
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_1(&self) -> Option<f64> {
        self.inner.transmit_delay_1
    }
    #[setter]
    fn set_transmit_delay_1(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_1 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 2.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_2(&self) -> Option<f64> {
        self.inner.transmit_delay_2
    }
    #[setter]
    fn set_transmit_delay_2(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_2 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 3.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_3(&self) -> Option<f64> {
        self.inner.transmit_delay_3
    }
    #[setter]
    fn set_transmit_delay_3(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_3 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 4.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_4(&self) -> Option<f64> {
        self.inner.transmit_delay_4
    }
    #[setter]
    fn set_transmit_delay_4(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_4 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 5.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_transmit_delay_5(&self) -> Option<f64> {
        self.inner.transmit_delay_5
    }
    #[setter]
    fn set_transmit_delay_5(&mut self, value: Option<f64>) {
        self.inner.transmit_delay_5 = value;
    }

    /// The RECEIVE_DELAY_n keyword shall specify a fixed interval of time, in seconds,
    /// required for the signal to travel from the tracking point to the receiving electronics.
    /// The default value shall be 0.0.
    ///
    /// Examples: 1.23, 0.0326, 0.00777
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_1(&self) -> Option<f64> {
        self.inner.receive_delay_1
    }
    #[setter]
    fn set_receive_delay_1(&mut self, value: Option<f64>) {
        self.inner.receive_delay_1 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 2.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_2(&self) -> Option<f64> {
        self.inner.receive_delay_2
    }
    #[setter]
    fn set_receive_delay_2(&mut self, value: Option<f64>) {
        self.inner.receive_delay_2 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 3.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_3(&self) -> Option<f64> {
        self.inner.receive_delay_3
    }
    #[setter]
    fn set_receive_delay_3(&mut self, value: Option<f64>) {
        self.inner.receive_delay_3 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 4.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_4(&self) -> Option<f64> {
        self.inner.receive_delay_4
    }
    #[setter]
    fn set_receive_delay_4(&mut self, value: Option<f64>) {
        self.inner.receive_delay_4 = value;
    }

    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 5.
    ///
    /// Units: s
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_receive_delay_5(&self) -> Option<f64> {
        self.inner.receive_delay_5
    }
    #[setter]
    fn set_receive_delay_5(&mut self, value: Option<f64>) {
        self.inner.receive_delay_5 = value;
    }

    /// Provides an estimate of the quality of the data, based on indicators from the producers
    /// of the data (e.g., bad time synchronization flags, marginal lock status indicators,
    /// etc.). The default value shall be ‘RAW’.
    ///
    /// Examples: RAW, VALIDATED, DEGRADED
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_data_quality(&self) -> Option<String> {
        self.inner.data_quality.as_ref().map(|v| v.to_string())
    }
    #[setter]
    fn set_data_quality(&mut self, value: Option<String>) -> PyResult<()> {
        use ccsds_ndm::types::TdmDataQuality;
        use std::str::FromStr;
        self.inner.data_quality = value
            .map(|s| {
                TdmDataQuality::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
            })
            .transpose()?;
        Ok(())
    }

    /// The set of CORRECTION_* keywords may be used to reflect the values of corrections that
    /// have been added to the data or should be added to the data (e.g., ranging station delay
    /// calibration, etc.).
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_angle_1(&self) -> Option<f64> {
        self.inner.correction_angle_1
    }
    #[setter]
    fn set_correction_angle_1(&mut self, value: Option<f64>) {
        self.inner.correction_angle_1 = value;
    }

    /// A correction value to be added to the ANGLE_2 data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_angle_2(&self) -> Option<f64> {
        self.inner.correction_angle_2
    }
    #[setter]
    fn set_correction_angle_2(&mut self, value: Option<f64>) {
        self.inner.correction_angle_2 = value;
    }

    /// A correction value to be added to the Doppler data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_doppler(&self) -> Option<f64> {
        self.inner.correction_doppler
    }
    #[setter]
    fn set_correction_doppler(&mut self, value: Option<f64>) {
        self.inner.correction_doppler = value;
    }

    /// A correction value to be added to the magnitude data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_mag(&self) -> Option<f64> {
        self.inner.correction_mag
    }
    #[setter]
    fn set_correction_mag(&mut self, value: Option<f64>) {
        self.inner.correction_mag = value;
    }

    /// A correction value to be added to the range data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_range(&self) -> Option<f64> {
        self.inner.correction_range
    }
    #[setter]
    fn set_correction_range(&mut self, value: Option<f64>) {
        self.inner.correction_range = value;
    }

    /// A correction value to be added to the RCS data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_rcs(&self) -> Option<f64> {
        self.inner.correction_rcs
    }
    #[setter]
    fn set_correction_rcs(&mut self, value: Option<f64>) {
        self.inner.correction_rcs = value;
    }

    /// A correction value to be added to the received frequency or phase count data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_receive(&self) -> Option<f64> {
        self.inner.correction_receive
    }
    #[setter]
    fn set_correction_receive(&mut self, value: Option<f64>) {
        self.inner.correction_receive = value;
    }

    /// A correction value to be added to the transmitted frequency or phase count data.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_transmit(&self) -> Option<f64> {
        self.inner.correction_transmit
    }
    #[setter]
    fn set_correction_transmit(&mut self, value: Option<f64>) {
        self.inner.correction_transmit = value;
    }

    /// A correction value for yearly aberration.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_aberration_yearly(&self) -> Option<f64> {
        self.inner.correction_aberration_yearly
    }
    #[setter]
    fn set_correction_aberration_yearly(&mut self, value: Option<f64>) {
        self.inner.correction_aberration_yearly = value;
    }

    /// A correction value for diurnal aberration.
    ///
    /// Examples: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// :type: Optional[float]
    #[getter]
    fn get_correction_aberration_diurnal(&self) -> Option<f64> {
        self.inner.correction_aberration_diurnal
    }
    #[setter]
    fn set_correction_aberration_diurnal(&mut self, value: Option<f64>) {
        self.inner.correction_aberration_diurnal = value;
    }

    /// This keyword is used to indicate whether or not the values associated with the
    /// CORRECTION_* keywords have been applied to the tracking data. Required if any of the
    /// CORRECTION_* keywords is used.
    ///
    /// Examples: YES, NO
    ///
    /// :type: Optional[str]
    #[getter]
    fn get_corrections_applied(&self) -> Option<String> {
        self.inner
            .corrections_applied
            .as_ref()
            .map(|v| v.to_string())
    }
    #[setter]
    fn set_corrections_applied(&mut self, value: Option<String>) -> PyResult<()> {
        use std::str::FromStr;
        self.inner.corrections_applied = value
            .map(|s| core_types::YesNo::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string())))
            .transpose()?;
        Ok(())
    }
}

// ============================================================================
// TDM Data
// ============================================================================

/// The Data Section of the TDM Segment consists of one or more Tracking Data Records.
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

    /// Comments.
    ///
    /// :type: list[TdmObservation]
    #[getter]
    fn get_comment(&self) -> Vec<String> {
        self.inner.comment.clone()
    }

    #[setter]
    fn set_comment(&mut self, value: Vec<String>) {
        self.inner.comment = value;
    }

    /// Tracking data records.
    ///
    /// :type: list[TdmObservation]
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



}

// ============================================================================
// TDM Observation
// ============================================================================

/// A single tracking data record consisting of a timetag and a measurement.
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
            "RECEIVE_PHASE_CT_1" => TdmObservationData::ReceivePhaseCt1(value),
            "RECEIVE_PHASE_CT_2" => TdmObservationData::ReceivePhaseCt2(value),
            "RECEIVE_PHASE_CT_3" => TdmObservationData::ReceivePhaseCt3(value),
            "RECEIVE_PHASE_CT_4" => TdmObservationData::ReceivePhaseCt4(value),
            "RECEIVE_PHASE_CT_5" => TdmObservationData::ReceivePhaseCt5(value),
            "TRANSMIT_PHASE_CT_1" => TdmObservationData::TransmitPhaseCt1(value),
            "TRANSMIT_PHASE_CT_2" => TdmObservationData::TransmitPhaseCt2(value),
            "TRANSMIT_PHASE_CT_3" => TdmObservationData::TransmitPhaseCt3(value),
            "TRANSMIT_PHASE_CT_4" => TdmObservationData::TransmitPhaseCt4(value),
            "TRANSMIT_PHASE_CT_5" => TdmObservationData::TransmitPhaseCt5(value),
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

    /// Time associated with the tracking observable.
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

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum TdmMode {
    Sequential,
    SingleDiff,
}

#[pymethods]
impl TdmMode {
    fn __str__(&self) -> &'static str {
        match self {
            TdmMode::Sequential => "SEQUENTIAL",
            TdmMode::SingleDiff => "SINGLE_DIFF",
        }
    }
    fn __repr__(&self) -> String {
        format!("TdmMode.{}", self.__str__())
    }
}

pub fn parse_tdm_mode(ob: &Bound<'_, PyAny>) -> PyResult<core_types::TdmMode> {
    if let Ok(val) = ob.extract::<TdmMode>() {
        Ok(match val {
            TdmMode::Sequential => core_types::TdmMode::Sequential,
            TdmMode::SingleDiff => core_types::TdmMode::SingleDiff,
        })
    } else if let Ok(s) = ob.extract::<String>() {
        core_types::TdmMode::from_str(&s).map_err(|e| PyValueError::new_err(e.to_string()))
    } else {
        Err(PyValueError::new_err("Expected TdmMode enum or string"))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Copy)]
pub enum TdmPath {
    Path1,
    Path2,
    Path3,
}

#[pymethods]
impl TdmPath {
    fn __str__(&self) -> &'static str {
        match self {
            TdmPath::Path1 => "1,2",
            TdmPath::Path2 => "2,1",
            TdmPath::Path3 => "1,2,1",
        }
    }
    fn __repr__(&self) -> String {
        format!("TdmPath.Path_{}", self.__str__())
    }
}

pub fn parse_tdm_path(ob: &Bound<'_, PyAny>) -> PyResult<core_types::TdmPath> {
    if let Ok(val) = ob.extract::<TdmPath>() {
        Ok(core_types::TdmPath(val.__str__().to_string()))
    } else if let Ok(s) = ob.extract::<String>() {
        Ok(core_types::TdmPath(s))
    } else {
        Err(PyValueError::new_err("Expected TdmPath enum or string"))
    }
}
