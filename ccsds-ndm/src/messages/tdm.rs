// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::{
    Epoch, Percentage, TdmAngleType, TdmDataQuality, TdmIntegrationRef, TdmMode, TdmPath,
    TdmRangeMode, TdmRangeUnits, TdmReferenceFrame, TdmTimetagRef, YesNo,
};
use fast_float;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

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
/// **CCSDS Reference**: 503.0-B-2.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "tdm")]
pub struct Tdm {
    pub header: TdmHeader,
    pub body: TdmBody,
    #[serde(rename = "@id")]
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
}

impl crate::traits::Validate for Tdm {
    fn validate(&self) -> Result<()> {
        Tdm::validate(self)
    }
}

impl Ndm for Tdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let tdm = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Tdm, &tdm)?;
        Ok(tdm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        if crate::validation::current_mode() == crate::validation::ValidationMode::Strict
            || crate::validation::current_mode() == crate::validation::ValidationMode::Lenient
        {
            if let Err(err) = validate_tdm_xml_metadata(xml) {
                crate::validation::handle_validation_error(
                    crate::validation::MessageKind::Tdm,
                    err,
                )?;
            }
        }
        let tdm: Self = crate::xml::from_str_with_context(xml, "TDM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Tdm, &tdm)?;
        Ok(tdm)
    }
}

impl Tdm {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.validate()
    }
}

impl ToKvn for Tdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_TDM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Header
//----------------------------------------------------------------------

/// Represents the `tdmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TdmHeader {
    /// Comments (allowed in the TDM Header only immediately after the TDM version number).
    /// (See 4.5 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Data creation date/time in UTC. (For format specification, see 4.3.9.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23.4, 2006-001T00:00:00Z
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.2.
    pub creation_date: Epoch,
    /// Creating agency. Value should be an entry from the ‘Abbreviation’ column in the SANA
    /// Organizations Registry, <https://sanaregistry.org/r/organizations/organizations.html>
    /// (reference `[11]`).
    ///
    /// **Examples**: CNES, ESA, GSFC, DLR, JPL, JAXA
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.2.
    #[builder(into)]
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator. The format and content
    /// of the message identifier value are at the discretion of the originator.
    ///
    /// **Examples**: 201113719185
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_id: Option<String>,
}

impl TdmHeader {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl ToKvn for TdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(v) = &self.message_id {
            writer.write_pair("MESSAGE_ID", v);
        }
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The TDM Body consists of one or more TDM Segments.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct TdmBody {
    #[serde(rename = "segment")]
    #[builder(default)]
    pub segments: Vec<TdmSegment>,
}

impl ToKvn for TdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

impl TdmBody {
    pub fn validate(&self) -> Result<()> {
        for segment in &self.segments {
            segment.validate()?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct TdmSegment {
    /// Metadata section for this TDM segment.
    pub metadata: TdmMetadata,
    /// Data section for this TDM segment.
    pub data: TdmData,
}

impl TdmSegment {
    pub fn validate(&self) -> Result<()> {
        self.metadata.validate()
    }
}

impl ToKvn for TdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TdmMetadata {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// The TRACK_ID keyword specifies a unique identifier for the tracking data in the
    /// associated data section. The value may be a freely selected string of characters and
    /// numbers, only required to be unique for each track of the corresponding sensor. For
    /// example, the value may be constructed from the measurement date and time and a counter
    /// to distinguish simultaneously tracked objects.
    ///
    /// **Examples**: 20190918_1200135-0001
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub track_id: Option<String>,
    /// Comma-separated list of data types in the Data Section. The elements of the list shall
    /// be selected from the data types shown in table 3-5, with the exception of the
    /// DATA_START, DATA_STOP, and COMMENT keywords.
    ///
    /// **Examples**: RANGE, TRANSMIT_FREQ_n, RECEIVE_FREQ
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub data_types: Option<String>,
    /// The TIME_SYSTEM keyword shall specify the time system used for timetags in the
    /// associated Data Section. This should be UTC for ground-based data. The value associated
    /// with this keyword must be selected from the full set of allowed values enumerated in
    /// the SANA Time Systems Registry <https://sanaregistry.org/r/time_systems> (reference `[12]`).
    /// (See annex B.)
    ///
    /// **Examples**: UTC, TAI, GPS, SCLK
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[builder(into)]
    pub time_system: String,
    /// The START_TIME keyword shall specify the UTC start time of the total time span covered
    /// by the tracking data immediately following this Metadata Section. (For format
    /// specification, see 4.3.9.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Epoch>,
    /// The STOP_TIME keyword shall specify the UTC stop time of the total time span covered by
    /// the tracking data immediately following this Metadata Section. (For format
    /// specification, see 4.3.9.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54, 2006-001T00:00:00Z
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Epoch>,
    /// The PARTICIPANT_n keyword shall represent the participants (see 1.3.4.1) in a tracking
    /// data session. It is indexed to allow unambiguous reference to other data in the TDM
    /// (max index is 5). At least two participants must be specified for most sessions; for
    /// some special TDMs such as tropospheric media, only one participant need be listed.
    ///
    /// **Examples**: DSS-63-S400K, ROSETTA, `<Quasar catalog name>`, 1997-061A, UNKNOWN
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[builder(into)]
    pub participant_1: String,
    /// The second participant in a tracking data session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub participant_2: Option<String>,
    /// The third participant in a tracking data session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub participant_3: Option<String>,
    /// The fourth participant in a tracking data session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub participant_4: Option<String>,
    /// The fifth participant in a tracking data session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub participant_5: Option<String>,
    /// The MODE keyword shall reflect the tracking mode associated with the Data Section of
    /// the segment. The value ‘SEQUENTIAL’ applies for most sequential signal paths; the name
    /// implies a sequential signal path between tracking participants. The value
    /// ‘SINGLE_DIFF’ applies only for differenced data.
    ///
    /// **Examples**: SEQUENTIAL, SINGLE_DIFF
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TdmMode>,
    /// The PATH keywords shall reflect the signal path by listing the index of each participant
    /// in order, separated by commas, with no inserted white space. Correlated with the
    /// indices of the PARTICIPANT_n keywords. The first entry in the PATH shall be the
    /// transmit participant.
    ///
    /// **Examples**: PATH = 1,2,1, PATH_1 = 1,2,1, PATH_2 = 3,1
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<TdmPath>,
    /// The first signal path where the MODE is 'SINGLE_DIFF'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_1: Option<TdmPath>,
    /// The second signal path where the MODE is 'SINGLE_DIFF'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_2: Option<TdmPath>,
    /// The TRANSMIT_BAND keyword shall indicate the frequency band for transmitted
    /// frequencies. The frequency ranges associated with each band should be specified in the
    /// ICD.
    ///
    /// **Examples**: S, X, Ka, L, UHF, GREEN
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub transmit_band: Option<String>,
    /// The RECEIVE_BAND keyword shall indicate the frequency band for received frequencies.
    /// Although not required in general, the RECEIVE_BAND must be present if the MODE is
    /// SINGLE_DIFF and differenced frequencies or differenced range are provided in order to
    /// allow proper frequency dependent corrections to be applied.
    ///
    /// **Examples**: S, X, Ka, L, UHF, GREEN
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub receive_band: Option<String>,
    /// The TURNAROUND_NUMERATOR keyword shall indicate the numerator of the turnaround ratio
    /// that is necessary to calculate the coherent downlink from the uplink frequency.
    ///
    /// **Examples**: 240, 880
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnaround_numerator: Option<i32>,
    /// The TURNAROUND_DENOMINATOR keyword shall indicate the denominator of the turnaround
    /// ratio that is necessary to calculate the coherent downlink from the uplink frequency.
    ///
    /// **Examples**: 221, 749
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnaround_denominator: Option<i32>,
    /// The TIMETAG_REF keyword shall provide a reference for time tags in the tracking data.
    /// This keyword indicates whether the timetag associated with the data is the transmit
    /// time or the receive time.
    ///
    /// **Examples**: TRANSMIT, RECEIVE
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timetag_ref: Option<TdmTimetagRef>,
    /// The INTEGRATION_INTERVAL keyword shall provide the Doppler count time in seconds for
    /// Doppler data or for the creation of normal points.
    ///
    /// **Examples**: 60.0, 0.1, 1.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_interval: Option<f64>,
    /// Indicates the relationship between the INTEGRATION_INTERVAL and the timetag on the
    /// data, i.e., whether the timetag represents the start, middle, or end of the integration
    /// period.
    ///
    /// **Examples**: START, MIDDLE, END
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_ref: Option<TdmIntegrationRef>,
    /// The FREQ_OFFSET keyword represents a frequency in Hz that must be added to every
    /// RECEIVE_FREQ to reconstruct it. One use is if a Doppler shift frequency observable is
    /// transferred instead of the actual received frequency. The default shall be 0.0.
    ///
    /// **Examples**: 0.0, 8415000000.0
    ///
    /// **Units**: Hz
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freq_offset: Option<f64>,
    /// The value of the RANGE_MODE keyword shall be ‘COHERENT’, in which case the range tones
    /// are coherent with the uplink carrier; ‘CONSTANT’, in which case the range tones have a
    /// constant frequency; or ‘ONE_WAY’ (used in Delta-DOR).
    ///
    /// **Examples**: COHERENT, CONSTANT, ONE_WAY
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_mode: Option<TdmRangeMode>,
    /// The value associated with the RANGE_MODULUS keyword shall be the modulus of the range
    /// observable in the units as specified by the RANGE_UNITS keyword; that is, the actual
    /// (unambiguous) range is an integer k times the modulus, plus the observable value. The
    /// default value shall be 0.0.
    ///
    /// **Examples**: 32768.0, 2.0e+23, 0.0, 161.6484
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_modulus: Option<f64>,
    /// The RANGE_UNITS keyword specifies the units for the range observable. ‘km’ shall be
    /// used if the range is measured in kilometers. ‘s’ shall be used if the range is measured
    /// in seconds. ‘RU’, for ‘range units’, shall be used where the transmit frequency is
    /// changing. The default value shall be ‘km’.
    ///
    /// **Examples**: km, s, RU
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_units: Option<TdmRangeUnits>,
    /// The ANGLE_TYPE keyword shall indicate the type of antenna geometry represented in the
    /// angle data (ANGLE_1 and ANGLE_2 keywords).
    ///
    /// **Examples**: AZEL, RADEC, XEYN, XSYE
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_type: Option<TdmAngleType>,
    /// The REFERENCE_FRAME keyword shall be used in conjunction with the ‘ANGLE_TYPE=RADEC’
    /// keyword/value combination, indicating the inertial reference frame to which the antenna
    /// frame is referenced.
    ///
    /// **Examples**: EME2000, ICRF, ITRF1993, ITRF2000, TOD_EARTH
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_frame: Option<TdmReferenceFrame>,
    /// The INTERPOLATION keyword shall specify the interpolation method to be used to calculate
    /// a transmit phase count at an arbitrary time in tracking data where the uplink frequency
    /// is not constant.
    ///
    /// **Examples**: HERMITE, LAGRANGE, LINEAR
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub interpolation: Option<String>,
    /// The INTERPOLATION_DEGREE keyword shall specify the recommended degree of the
    /// interpolating polynomial used to calculate a transmit phase count at an arbitrary time
    /// in tracking data where the uplink frequency is not constant.
    ///
    /// **Examples**: 3, 5, 7, 11
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<u32>,
    /// Doppler counts are generally biased so as to accommodate negative Doppler within an
    /// accumulator. In order to reconstruct the measurement, the bias shall be subtracted from
    /// the DOPPLER_COUNT data value.
    ///
    /// **Examples**: 2.4e6, 240000000.0
    ///
    /// **Units**: Hz
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_bias: Option<f64>,
    /// Doppler counts are generally scaled so as to capture partial cycles in an integer
    /// count. In order to reconstruct the measurement, the DOPPLER_COUNT data value shall be
    /// divided by the scale factor. The default shall be 1.
    ///
    /// **Examples**: 1000, 1
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_scale: Option<u64>,
    /// Doppler counts may overflow the accumulator and roll over in cases where the track is
    /// of long duration or very high Doppler shift. This flag indicates whether or not a
    /// counter rollover has occurred during the track.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_rollover: Option<YesNo>,
    /// The TRANSMIT_DELAY_n keyword shall specify a fixed interval of time, in seconds,
    /// required for the signal to travel from the transmitting electronics to the transmit
    /// point. The default value shall be 0.0.
    ///
    /// **Examples**: 1.23, 0.0326, 0.00077
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_1: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 2.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_2: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 3.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_3: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 4.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_4: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the
    /// transmitting electronics to the transmit point for participant 5.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_5: Option<f64>,
    /// The RECEIVE_DELAY_n keyword shall specify a fixed interval of time, in seconds,
    /// required for the signal to travel from the tracking point to the receiving electronics.
    /// The default value shall be 0.0.
    ///
    /// **Examples**: 1.23, 0.0326, 0.00777
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_1: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 2.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_2: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 3.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_3: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 4.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_4: Option<f64>,
    /// Fixed interval of time, in seconds, required for the signal to travel from the tracking
    /// point to the receiving electronics for participant 5.
    ///
    /// **Units**: s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_5: Option<f64>,
    /// Provides an estimate of the quality of the data, based on indicators from the producers
    /// of the data (e.g., bad time synchronization flags, marginal lock status indicators,
    /// etc.). The default value shall be ‘RAW’.
    ///
    /// **Examples**: RAW, VALIDATED, DEGRADED
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_quality: Option<TdmDataQuality>,
    /// The set of CORRECTION_* keywords may be used to reflect the values of corrections that
    /// have been added to the data or should be added to the data (e.g., ranging station delay
    /// calibration, etc.).
    ///
    /// **Examples**: -1.35, 0.23, -3.0e-1, 150000.0
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_angle_1: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_angle_2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_doppler: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_mag: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_range: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_rcs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_receive: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_transmit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_aberration_yearly: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_aberration_diurnal: Option<f64>,
    /// This keyword is used to indicate whether or not the values associated with the
    /// CORRECTION_* keywords have been applied to the tracking data. Required if any of the
    /// CORRECTION_* keywords is used.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 503.0-B-2, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrections_applied: Option<YesNo>,
    /// Unique name of the external ephemeris file used for participant 1.
    ///
    /// Examples: SATELLITE_A_EPHEM27
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ephemeris_name_1: Option<String>,
    /// Unique name of the external ephemeris file used for participant 2.
    ///
    /// Examples: SATELLITE_A_EPHEM27
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ephemeris_name_2: Option<String>,
    /// Unique name of the external ephemeris file used for participant 3.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ephemeris_name_3: Option<String>,
    /// Unique name of the external ephemeris file used for participant 4.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ephemeris_name_4: Option<String>,
    /// Unique name of the external ephemeris file used for participant 5.
    ///
    /// Examples: SATELLITE_A_EPHEMERIS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ephemeris_name_5: Option<String>,
}

impl TdmMetadata {
    pub fn validate(&self) -> Result<()> {
        // XSD Choice between PATH and (PATH_1, PATH_2)
        if self.path.is_some() && (self.path_1.is_some() || self.path_2.is_some()) {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed("TDM Metadata cannot have both PATH and PATH_1/PATH_2"),
                line: None,
            }
            .into());
        }
        if (self.path_1.is_some() && self.path_2.is_none())
            || (self.path_1.is_none() && self.path_2.is_some())
        {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "TDM Metadata must have both PATH_1 and PATH_2 if one is present",
                ),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

fn validate_tdm_xml_metadata(xml: &str) -> Result<()> {
    let allowed = tdm_metadata_allowed_tags();
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut in_metadata = false;
    let mut depth = 0usize;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let name_upper = name.to_ascii_uppercase();
                if name_upper == "METADATA" {
                    in_metadata = true;
                    depth = 1;
                    continue;
                }
                if in_metadata {
                    depth += 1;
                    if !allowed.contains(name_upper.as_str()) {
                        return Err(ValidationError::InvalidValue {
                            field: Cow::Borrowed("TDM Metadata keyword"),
                            value: name,
                            expected: Cow::Borrowed("allowed TDM metadata keyword"),
                            line: None,
                        }
                        .into());
                    }
                }
            }
            Ok(Event::End(e)) => {
                if in_metadata {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if name.eq_ignore_ascii_case("metadata") {
                        break;
                    }
                    depth = depth.saturating_sub(1);
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn tdm_metadata_allowed_tags() -> std::collections::HashSet<&'static str> {
    let tags = [
        "COMMENT",
        "TRACK_ID",
        "DATA_TYPES",
        "TIME_SYSTEM",
        "START_TIME",
        "STOP_TIME",
        "PARTICIPANT_1",
        "PARTICIPANT_2",
        "PARTICIPANT_3",
        "PARTICIPANT_4",
        "PARTICIPANT_5",
        "MODE",
        "PATH",
        "PATH_1",
        "PATH_2",
        "TRANSMIT_BAND",
        "RECEIVE_BAND",
        "TURNAROUND_NUMERATOR",
        "TURNAROUND_DENOMINATOR",
        "TIMETAG_REF",
        "INTEGRATION_INTERVAL",
        "INTEGRATION_REF",
        "FREQ_OFFSET",
        "RANGE_MODE",
        "RANGE_MODULUS",
        "RANGE_UNITS",
        "ANGLE_TYPE",
        "REFERENCE_FRAME",
        "INTERPOLATION",
        "INTERPOLATION_DEGREE",
        "DOPPLER_COUNT_BIAS",
        "DOPPLER_COUNT_SCALE",
        "DOPPLER_COUNT_ROLLOVER",
        "TRANSMIT_DELAY_1",
        "TRANSMIT_DELAY_2",
        "TRANSMIT_DELAY_3",
        "TRANSMIT_DELAY_4",
        "TRANSMIT_DELAY_5",
        "RECEIVE_DELAY_1",
        "RECEIVE_DELAY_2",
        "RECEIVE_DELAY_3",
        "RECEIVE_DELAY_4",
        "RECEIVE_DELAY_5",
        "DATA_QUALITY",
        "CORRECTION_ANGLE_1",
        "CORRECTION_ANGLE_2",
        "CORRECTION_DOPPLER",
        "CORRECTION_MAG",
        "CORRECTION_RANGE",
        "CORRECTION_RCS",
        "CORRECTION_RECEIVE",
        "CORRECTION_TRANSMIT",
        "CORRECTION_ABERRATION_YEARLY",
        "CORRECTION_ABERRATION_DIURNAL",
        "CORRECTIONS_APPLIED",
        "EPHEMERIS_NAME_1",
        "EPHEMERIS_NAME_2",
        "EPHEMERIS_NAME_3",
        "EPHEMERIS_NAME_4",
        "EPHEMERIS_NAME_5",
    ];
    tags.into_iter().collect()
}

impl ToKvn for TdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.track_id {
            writer.write_pair("TRACK_ID", v);
        }
        if let Some(v) = &self.data_types {
            writer.write_pair("DATA_TYPES", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        if let Some(v) = &self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = &self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        writer.write_pair("PARTICIPANT_1", &self.participant_1);
        if let Some(v) = &self.participant_2 {
            writer.write_pair("PARTICIPANT_2", v);
        }
        if let Some(v) = &self.participant_3 {
            writer.write_pair("PARTICIPANT_3", v);
        }
        if let Some(v) = &self.participant_4 {
            writer.write_pair("PARTICIPANT_4", v);
        }
        if let Some(v) = &self.participant_5 {
            writer.write_pair("PARTICIPANT_5", v);
        }
        if let Some(v) = &self.mode {
            writer.write_pair("MODE", v.to_string());
        }
        if let Some(v) = &self.path {
            writer.write_pair("PATH", v.0.as_str());
        }
        if let Some(v) = &self.path_1 {
            writer.write_pair("PATH_1", v.0.as_str());
        }
        if let Some(v) = &self.path_2 {
            writer.write_pair("PATH_2", v.0.as_str());
        }
        if let Some(v) = &self.ephemeris_name_1 {
            writer.write_pair("EPHEMERIS_NAME_1", v);
        }
        if let Some(v) = &self.ephemeris_name_2 {
            writer.write_pair("EPHEMERIS_NAME_2", v);
        }
        if let Some(v) = &self.ephemeris_name_3 {
            writer.write_pair("EPHEMERIS_NAME_3", v);
        }
        if let Some(v) = &self.ephemeris_name_4 {
            writer.write_pair("EPHEMERIS_NAME_4", v);
        }
        if let Some(v) = &self.ephemeris_name_5 {
            writer.write_pair("EPHEMERIS_NAME_5", v);
        }
        if let Some(v) = &self.transmit_band {
            writer.write_pair("TRANSMIT_BAND", v);
        }
        if let Some(v) = &self.receive_band {
            writer.write_pair("RECEIVE_BAND", v);
        }
        if let Some(v) = self.turnaround_numerator {
            writer.write_pair("TURNAROUND_NUMERATOR", v);
        }
        if let Some(v) = self.turnaround_denominator {
            writer.write_pair("TURNAROUND_DENOMINATOR", v);
        }
        if let Some(v) = &self.timetag_ref {
            writer.write_pair("TIMETAG_REF", v.to_string());
        }
        if let Some(v) = self.integration_interval {
            writer.write_pair("INTEGRATION_INTERVAL", v);
        }
        if let Some(v) = &self.integration_ref {
            writer.write_pair("INTEGRATION_REF", v.to_string());
        }
        if let Some(v) = self.freq_offset {
            writer.write_pair("FREQ_OFFSET", v);
        }
        if let Some(v) = &self.range_mode {
            writer.write_pair("RANGE_MODE", v.to_string());
        }
        if let Some(v) = self.range_modulus {
            writer.write_pair("RANGE_MODULUS", v);
        }
        if let Some(v) = &self.range_units {
            writer.write_pair("RANGE_UNITS", v.to_string());
        }
        if let Some(v) = &self.angle_type {
            writer.write_pair("ANGLE_TYPE", v.to_string());
        }
        if let Some(v) = &self.reference_frame {
            writer.write_pair("REFERENCE_FRAME", v.to_string());
        }
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
        if let Some(v) = self.doppler_count_bias {
            writer.write_pair("DOPPLER_COUNT_BIAS", v);
        }
        if let Some(v) = self.doppler_count_scale {
            writer.write_pair("DOPPLER_COUNT_SCALE", v);
        }
        if let Some(v) = &self.doppler_count_rollover {
            writer.write_pair("DOPPLER_COUNT_ROLLOVER", format!("{}", v));
        }
        if let Some(v) = self.transmit_delay_1 {
            writer.write_pair("TRANSMIT_DELAY_1", v);
        }
        if let Some(v) = self.transmit_delay_2 {
            writer.write_pair("TRANSMIT_DELAY_2", v);
        }
        if let Some(v) = self.transmit_delay_3 {
            writer.write_pair("TRANSMIT_DELAY_3", v);
        }
        if let Some(v) = self.transmit_delay_4 {
            writer.write_pair("TRANSMIT_DELAY_4", v);
        }
        if let Some(v) = self.transmit_delay_5 {
            writer.write_pair("TRANSMIT_DELAY_5", v);
        }
        if let Some(v) = self.receive_delay_1 {
            writer.write_pair("RECEIVE_DELAY_1", v);
        }
        if let Some(v) = self.receive_delay_2 {
            writer.write_pair("RECEIVE_DELAY_2", v);
        }
        if let Some(v) = self.receive_delay_3 {
            writer.write_pair("RECEIVE_DELAY_3", v);
        }
        if let Some(v) = self.receive_delay_4 {
            writer.write_pair("RECEIVE_DELAY_4", v);
        }
        if let Some(v) = self.receive_delay_5 {
            writer.write_pair("RECEIVE_DELAY_5", v);
        }
        if let Some(v) = &self.data_quality {
            writer.write_pair("DATA_QUALITY", v.to_string());
        }
        if let Some(v) = self.correction_angle_1 {
            writer.write_pair("CORRECTION_ANGLE_1", v);
        }
        if let Some(v) = self.correction_angle_2 {
            writer.write_pair("CORRECTION_ANGLE_2", v);
        }
        if let Some(v) = self.correction_doppler {
            writer.write_pair("CORRECTION_DOPPLER", v);
        }
        if let Some(v) = self.correction_mag {
            writer.write_pair("CORRECTION_MAG", v);
        }
        if let Some(v) = self.correction_range {
            writer.write_pair("CORRECTION_RANGE", v);
        }
        if let Some(v) = self.correction_rcs {
            writer.write_pair("CORRECTION_RCS", v);
        }
        if let Some(v) = self.correction_receive {
            writer.write_pair("CORRECTION_RECEIVE", v);
        }
        if let Some(v) = self.correction_transmit {
            writer.write_pair("CORRECTION_TRANSMIT", v);
        }
        if let Some(v) = self.correction_aberration_yearly {
            writer.write_pair("CORRECTION_ABERRATION_YEARLY", v);
        }
        if let Some(v) = self.correction_aberration_diurnal {
            writer.write_pair("CORRECTION_ABERRATION_DIURNAL", v);
        }
        if let Some(v) = &self.corrections_applied {
            writer.write_pair("CORRECTIONS_APPLIED", format!("{}", v));
        }
        writer.write_section("META_STOP");
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// The Data Section of the TDM Segment consists of one or more Tracking Data Records.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct TdmData {
    /// Comments.
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Tracking data records.
    #[serde(rename = "observation")]
    #[builder(default)]
    pub observations: Vec<TdmObservation>,
}

impl ToKvn for TdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("DATA_START");
        writer.write_comments(&self.comment);
        for obs in &self.observations {
            let key = obs.data.key();
            let val_str = obs.data.value_to_string();
            let line = format!("{} {}", obs.epoch, val_str);
            writer.write_pair(key, line);
        }
        writer.write_section("DATA_STOP");
    }
}

//----------------------------------------------------------------------
// Observation
//----------------------------------------------------------------------

/// A single tracking data record consisting of a timetag and a measurement.
#[derive(Serialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct TdmObservation {
    /// Time associated with the tracking observable.
    #[serde(rename = "EPOCH")]
    pub epoch: Epoch,
    /// The tracking observable (measurement or calculation).
    #[serde(rename = "$value")]
    pub data: TdmObservationData,
}

// Custom Deserialize to handle XML's flat structure correctly
impl<'de> Deserialize<'de> for TdmObservation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TdmObservationVisitor;

        impl<'de> Visitor<'de> for TdmObservationVisitor {
            type Value = TdmObservation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a TDM observation element")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut epoch: Option<Epoch> = None;
                let mut data: Option<TdmObservationData> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "EPOCH" => {
                            if epoch.is_some() {
                                return Err(serde::de::Error::duplicate_field("EPOCH"));
                            }
                            epoch = Some(map.next_value()?);
                        }
                        // Explicit matching of all data types
                        "ANGLE_1" => {
                            data = Some(TdmObservationData::Angle1(map.next_value()?));
                        }
                        "ANGLE_2" => {
                            data = Some(TdmObservationData::Angle2(map.next_value()?));
                        }
                        "CARRIER_POWER" => {
                            data = Some(TdmObservationData::CarrierPower(map.next_value()?));
                        }
                        "CLOCK_BIAS" => {
                            data = Some(TdmObservationData::ClockBias(map.next_value()?));
                        }
                        "CLOCK_DRIFT" => {
                            data = Some(TdmObservationData::ClockDrift(map.next_value()?));
                        }
                        "DOPPLER_COUNT" => {
                            data = Some(TdmObservationData::DopplerCount(map.next_value()?));
                        }
                        "DOPPLER_INSTANTANEOUS" => {
                            data =
                                Some(TdmObservationData::DopplerInstantaneous(map.next_value()?));
                        }
                        "DOPPLER_INTEGRATED" => {
                            data = Some(TdmObservationData::DopplerIntegrated(map.next_value()?));
                        }
                        "DOR" => {
                            data = Some(TdmObservationData::Dor(map.next_value()?));
                        }
                        "MAG" => {
                            data = Some(TdmObservationData::Mag(map.next_value()?));
                        }
                        "PC_N0" => {
                            data = Some(TdmObservationData::PcN0(map.next_value()?));
                        }
                        "PR_N0" => {
                            data = Some(TdmObservationData::PrN0(map.next_value()?));
                        }
                        "PRESSURE" => {
                            data = Some(TdmObservationData::Pressure(map.next_value()?));
                        }
                        "RANGE" => {
                            data = Some(TdmObservationData::Range(map.next_value()?));
                        }
                        "RCS" => {
                            data = Some(TdmObservationData::Rcs(map.next_value()?));
                        }
                        "RECEIVE_FREQ" => {
                            data = Some(TdmObservationData::ReceiveFreq(map.next_value()?));
                        }
                        "RECEIVE_FREQ_1" => {
                            data = Some(TdmObservationData::ReceiveFreq1(map.next_value()?));
                        }
                        "RECEIVE_FREQ_2" => {
                            data = Some(TdmObservationData::ReceiveFreq2(map.next_value()?));
                        }
                        "RECEIVE_FREQ_3" => {
                            data = Some(TdmObservationData::ReceiveFreq3(map.next_value()?));
                        }
                        "RECEIVE_FREQ_4" => {
                            data = Some(TdmObservationData::ReceiveFreq4(map.next_value()?));
                        }
                        "RECEIVE_FREQ_5" => {
                            data = Some(TdmObservationData::ReceiveFreq5(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_1" => {
                            data = Some(TdmObservationData::ReceivePhaseCt1(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_2" => {
                            data = Some(TdmObservationData::ReceivePhaseCt2(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_3" => {
                            data = Some(TdmObservationData::ReceivePhaseCt3(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_4" => {
                            data = Some(TdmObservationData::ReceivePhaseCt4(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_5" => {
                            data = Some(TdmObservationData::ReceivePhaseCt5(map.next_value()?));
                        }
                        "RHUMIDITY" => {
                            data = Some(TdmObservationData::Rhumidity(map.next_value()?));
                        }
                        "STEC" => {
                            data = Some(TdmObservationData::Stec(map.next_value()?));
                        }
                        "TEMPERATURE" => {
                            data = Some(TdmObservationData::Temperature(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_1" => {
                            data = Some(TdmObservationData::TransmitFreq1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_2" => {
                            data = Some(TdmObservationData::TransmitFreq2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_3" => {
                            data = Some(TdmObservationData::TransmitFreq3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_4" => {
                            data = Some(TdmObservationData::TransmitFreq4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_5" => {
                            data = Some(TdmObservationData::TransmitFreq5(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_1" => {
                            data = Some(TdmObservationData::TransmitFreqRate1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_2" => {
                            data = Some(TdmObservationData::TransmitFreqRate2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_3" => {
                            data = Some(TdmObservationData::TransmitFreqRate3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_4" => {
                            data = Some(TdmObservationData::TransmitFreqRate4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_5" => {
                            data = Some(TdmObservationData::TransmitFreqRate5(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_1" => {
                            data = Some(TdmObservationData::TransmitPhaseCt1(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_2" => {
                            data = Some(TdmObservationData::TransmitPhaseCt2(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_3" => {
                            data = Some(TdmObservationData::TransmitPhaseCt3(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_4" => {
                            data = Some(TdmObservationData::TransmitPhaseCt4(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_5" => {
                            data = Some(TdmObservationData::TransmitPhaseCt5(map.next_value()?));
                        }
                        "TROPO_DRY" => {
                            data = Some(TdmObservationData::TropoDry(map.next_value()?));
                        }
                        "TROPO_WET" => {
                            data = Some(TdmObservationData::TropoWet(map.next_value()?));
                        }
                        "VLBI_DELAY" => {
                            data = Some(TdmObservationData::VlbiDelay(map.next_value()?));
                        }
                        _ => {
                            // Consume unknown fields or attributes that might appear
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let epoch = epoch.ok_or_else(|| serde::de::Error::missing_field("EPOCH"))?;
                let data = data.ok_or_else(|| {
                    serde::de::Error::custom(
                        "Missing TDM observation data (must have one of: ANGLE_1, RANGE, etc.)",
                    )
                })?;

                Ok(TdmObservation { epoch, data })
            }
        }

        deserializer.deserialize_map(TdmObservationVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TdmObservationData {
    /// Azimuth, right ascension, or 'X' angle of the measurement.
    ///
    /// Units: deg
    #[serde(rename = "ANGLE_1")]
    Angle1(f64),
    /// Elevation, declination, or 'Y' angle of the measurement.
    ///
    /// Units: deg
    #[serde(rename = "ANGLE_2")]
    Angle2(f64),
    /// The strength of the radio signal transmitted by the spacecraft.
    ///
    /// Units: dBW
    CarrierPower(f64),
    /// Clock bias.
    ///
    /// Units: s
    ClockBias(f64),
    /// Clock drift.
    ///
    /// Units: s/s
    ClockDrift(f64),
    /// A count of the number of times the phase of a received signal slips one cycle.
    DopplerCount(f64),
    /// The instantaneous range rate of the spacecraft.
    ///
    /// Units: km/s
    DopplerInstantaneous(f64),
    /// The mean range rate of the spacecraft over the INTEGRATION_INTERVAL.
    ///
    /// Units: km/s
    DopplerIntegrated(f64),
    /// Delta-DOR observable.
    ///
    /// Units: s
    Dor(f64),
    /// The apparent visual magnitude of an object.
    Mag(f64),
    /// Carrier power to noise spectral density ratio.
    ///
    /// Units: dBHz
    #[serde(rename = "PC_N0")]
    PcN0(f64),
    /// Ranging power to noise spectral density ratio.
    ///
    /// Units: dBHz
    #[serde(rename = "PR_N0")]
    PrN0(f64),
    /// Atmospheric pressure observable.
    ///
    /// Units: hPa
    Pressure(f64),
    /// The range observable.
    ///
    /// Units: km, s, or RU
    Range(f64),
    /// Radar Cross Section.
    ///
    /// Units: m²
    Rcs(f64),
    /// Received frequency.
    ///
    /// Units: Hz
    ReceiveFreq(f64),
    /// Received frequency for channel 1.
    ///
    /// Units: Hz
    #[serde(rename = "RECEIVE_FREQ_1")]
    ReceiveFreq1(f64),
    /// Received frequency for channel 2.
    ///
    /// Units: Hz
    #[serde(rename = "RECEIVE_FREQ_2")]
    ReceiveFreq2(f64),
    /// Received frequency for channel 3.
    ///
    /// Units: Hz
    #[serde(rename = "RECEIVE_FREQ_3")]
    ReceiveFreq3(f64),
    /// Received frequency for channel 4.
    ///
    /// Units: Hz
    #[serde(rename = "RECEIVE_FREQ_4")]
    ReceiveFreq4(f64),
    /// Received frequency for channel 5.
    ///
    /// Units: Hz
    #[serde(rename = "RECEIVE_FREQ_5")]
    ReceiveFreq5(f64),
    /// Received phase count for channel 1.
    #[serde(rename = "RECEIVE_PHASE_CT_1")]
    ReceivePhaseCt1(f64),
    /// Received phase count for channel 2.
    #[serde(rename = "RECEIVE_PHASE_CT_2")]
    ReceivePhaseCt2(f64),
    /// Received phase count for channel 3.
    #[serde(rename = "RECEIVE_PHASE_CT_3")]
    ReceivePhaseCt3(f64),
    /// Received phase count for channel 4.
    #[serde(rename = "RECEIVE_PHASE_CT_4")]
    ReceivePhaseCt4(f64),
    /// Received phase count for channel 5.
    #[serde(rename = "RECEIVE_PHASE_CT_5")]
    ReceivePhaseCt5(f64),
    /// Relative humidity observable.
    ///
    /// Units: %
    Rhumidity(Percentage),
    /// Slant Total Electron Count (STEC).
    ///
    /// Units: TECU
    Stec(f64),
    /// Temperature observable.
    ///
    /// Units: K
    Temperature(f64),
    /// Transmitted frequency for channel 1.
    ///
    /// Units: Hz
    #[serde(rename = "TRANSMIT_FREQ_1")]
    TransmitFreq1(f64),
    /// Transmitted frequency for channel 2.
    ///
    /// Units: Hz
    #[serde(rename = "TRANSMIT_FREQ_2")]
    TransmitFreq2(f64),
    /// Transmitted frequency for channel 3.
    ///
    /// Units: Hz
    #[serde(rename = "TRANSMIT_FREQ_3")]
    TransmitFreq3(f64),
    /// Transmitted frequency for channel 4.
    ///
    /// Units: Hz
    #[serde(rename = "TRANSMIT_FREQ_4")]
    TransmitFreq4(f64),
    /// Transmitted frequency for channel 5.
    ///
    /// Units: Hz
    #[serde(rename = "TRANSMIT_FREQ_5")]
    TransmitFreq5(f64),
    /// Linear rate of change of the frequency for channel 1.
    ///
    /// Units: Hz/s
    #[serde(rename = "TRANSMIT_FREQ_RATE_1")]
    TransmitFreqRate1(f64),
    /// Linear rate of change of the frequency for channel 2.
    ///
    /// Units: Hz/s
    #[serde(rename = "TRANSMIT_FREQ_RATE_2")]
    TransmitFreqRate2(f64),
    /// Linear rate of change of the frequency for channel 3.
    ///
    /// Units: Hz/s
    #[serde(rename = "TRANSMIT_FREQ_RATE_3")]
    TransmitFreqRate3(f64),
    /// Linear rate of change of the frequency for channel 4.
    ///
    /// Units: Hz/s
    #[serde(rename = "TRANSMIT_FREQ_RATE_4")]
    TransmitFreqRate4(f64),
    /// Linear rate of change of the frequency for channel 5.
    ///
    /// Units: Hz/s
    #[serde(rename = "TRANSMIT_FREQ_RATE_5")]
    TransmitFreqRate5(f64),
    /// Transmitted phase count for channel 1.
    #[serde(rename = "TRANSMIT_PHASE_CT_1")]
    TransmitPhaseCt1(f64),
    /// Transmitted phase count for channel 2.
    #[serde(rename = "TRANSMIT_PHASE_CT_2")]
    TransmitPhaseCt2(f64),
    /// Transmitted phase count for channel 3.
    #[serde(rename = "TRANSMIT_PHASE_CT_3")]
    TransmitPhaseCt3(f64),
    /// Transmitted phase count for channel 4.
    #[serde(rename = "TRANSMIT_PHASE_CT_4")]
    TransmitPhaseCt4(f64),
    /// Transmitted phase count for channel 5.
    #[serde(rename = "TRANSMIT_PHASE_CT_5")]
    TransmitPhaseCt5(f64),
    /// Dry zenith delay through the troposphere.
    ///
    /// Units: m
    TropoDry(f64),
    /// Wet zenith delay through the troposphere.
    ///
    /// Units: m
    TropoWet(f64),
    /// VLBI delay.
    ///
    /// Units: s
    VlbiDelay(f64),
}

impl TdmObservationData {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Angle1(_) => "ANGLE_1",
            Self::Angle2(_) => "ANGLE_2",
            Self::CarrierPower(_) => "CARRIER_POWER",
            Self::ClockBias(_) => "CLOCK_BIAS",
            Self::ClockDrift(_) => "CLOCK_DRIFT",
            Self::DopplerCount(_) => "DOPPLER_COUNT",
            Self::DopplerInstantaneous(_) => "DOPPLER_INSTANTANEOUS",
            Self::DopplerIntegrated(_) => "DOPPLER_INTEGRATED",
            Self::Dor(_) => "DOR",
            Self::Mag(_) => "MAG",
            Self::PcN0(_) => "PC_N0",
            Self::PrN0(_) => "PR_N0",
            Self::Pressure(_) => "PRESSURE",
            Self::Range(_) => "RANGE",
            Self::Rcs(_) => "RCS",
            Self::ReceiveFreq(_) => "RECEIVE_FREQ",
            Self::ReceiveFreq1(_) => "RECEIVE_FREQ_1",
            Self::ReceiveFreq2(_) => "RECEIVE_FREQ_2",
            Self::ReceiveFreq3(_) => "RECEIVE_FREQ_3",
            Self::ReceiveFreq4(_) => "RECEIVE_FREQ_4",
            Self::ReceiveFreq5(_) => "RECEIVE_FREQ_5",
            Self::ReceivePhaseCt1(_) => "RECEIVE_PHASE_CT_1",
            Self::ReceivePhaseCt2(_) => "RECEIVE_PHASE_CT_2",
            Self::ReceivePhaseCt3(_) => "RECEIVE_PHASE_CT_3",
            Self::ReceivePhaseCt4(_) => "RECEIVE_PHASE_CT_4",
            Self::ReceivePhaseCt5(_) => "RECEIVE_PHASE_CT_5",
            Self::Rhumidity(_) => "RHUMIDITY",
            Self::Stec(_) => "STEC",
            Self::Temperature(_) => "TEMPERATURE",
            Self::TransmitFreq1(_) => "TRANSMIT_FREQ_1",
            Self::TransmitFreq2(_) => "TRANSMIT_FREQ_2",
            Self::TransmitFreq3(_) => "TRANSMIT_FREQ_3",
            Self::TransmitFreq4(_) => "TRANSMIT_FREQ_4",
            Self::TransmitFreq5(_) => "TRANSMIT_FREQ_5",
            Self::TransmitFreqRate1(_) => "TRANSMIT_FREQ_RATE_1",
            Self::TransmitFreqRate2(_) => "TRANSMIT_FREQ_RATE_2",
            Self::TransmitFreqRate3(_) => "TRANSMIT_FREQ_RATE_3",
            Self::TransmitFreqRate4(_) => "TRANSMIT_FREQ_RATE_4",
            Self::TransmitFreqRate5(_) => "TRANSMIT_FREQ_RATE_5",
            Self::TransmitPhaseCt1(_) => "TRANSMIT_PHASE_CT_1",
            Self::TransmitPhaseCt2(_) => "TRANSMIT_PHASE_CT_2",
            Self::TransmitPhaseCt3(_) => "TRANSMIT_PHASE_CT_3",
            Self::TransmitPhaseCt4(_) => "TRANSMIT_PHASE_CT_4",
            Self::TransmitPhaseCt5(_) => "TRANSMIT_PHASE_CT_5",
            Self::TropoDry(_) => "TROPO_DRY",
            Self::TropoWet(_) => "TROPO_WET",
            Self::VlbiDelay(_) => "VLBI_DELAY",
        }
    }

    pub fn value_to_string(&self) -> String {
        match self {
            Self::Rhumidity(v) => v.value.to_string(),
            Self::Angle1(v)
            | Self::Angle2(v)
            | Self::CarrierPower(v)
            | Self::ClockBias(v)
            | Self::ClockDrift(v)
            | Self::DopplerCount(v)
            | Self::DopplerInstantaneous(v)
            | Self::DopplerIntegrated(v)
            | Self::Dor(v)
            | Self::Mag(v)
            | Self::PcN0(v)
            | Self::PrN0(v)
            | Self::Pressure(v)
            | Self::Range(v)
            | Self::Rcs(v)
            | Self::ReceiveFreq(v)
            | Self::ReceiveFreq1(v)
            | Self::ReceiveFreq2(v)
            | Self::ReceiveFreq3(v)
            | Self::ReceiveFreq4(v)
            | Self::ReceiveFreq5(v)
            | Self::ReceivePhaseCt1(v)
            | Self::ReceivePhaseCt2(v)
            | Self::ReceivePhaseCt3(v)
            | Self::ReceivePhaseCt4(v)
            | Self::ReceivePhaseCt5(v)
            | Self::Stec(v)
            | Self::Temperature(v)
            | Self::TransmitFreq1(v)
            | Self::TransmitFreq2(v)
            | Self::TransmitFreq3(v)
            | Self::TransmitFreq4(v)
            | Self::TransmitFreq5(v)
            | Self::TransmitFreqRate1(v)
            | Self::TransmitFreqRate2(v)
            | Self::TransmitFreqRate3(v)
            | Self::TransmitFreqRate4(v)
            | Self::TransmitFreqRate5(v)
            | Self::TransmitPhaseCt1(v)
            | Self::TransmitPhaseCt2(v)
            | Self::TransmitPhaseCt3(v)
            | Self::TransmitPhaseCt4(v)
            | Self::TransmitPhaseCt5(v)
            | Self::TropoDry(v)
            | Self::TropoWet(v)
            | Self::VlbiDelay(v) => v.to_string(),
        }
    }

    pub fn from_key_val(key: &str, val: &str) -> Result<Self> {
        let pf = |s: &str| {
            fast_float::parse(s).map_err(|_| {
                CcsdsNdmError::Format(Box::new(crate::error::FormatError::InvalidFormat(format!(
                    "Invalid float: {}",
                    s
                ))))
            })
        };
        match key {
            "ANGLE_1" => Ok(Self::Angle1(pf(val)?)),
            "ANGLE_2" => Ok(Self::Angle2(pf(val)?)),
            "CARRIER_POWER" => Ok(Self::CarrierPower(pf(val)?)),
            "CLOCK_BIAS" => Ok(Self::ClockBias(pf(val)?)),
            "CLOCK_DRIFT" => Ok(Self::ClockDrift(pf(val)?)),
            "DOPPLER_COUNT" => Ok(Self::DopplerCount(pf(val)?)),
            "DOPPLER_INSTANTANEOUS" => Ok(Self::DopplerInstantaneous(pf(val)?)),
            "DOPPLER_INTEGRATED" => Ok(Self::DopplerIntegrated(pf(val)?)),
            "DOR" => Ok(Self::Dor(pf(val)?)),
            "MAG" => Ok(Self::Mag(pf(val)?)),
            "PC_N0" => Ok(Self::PcN0(pf(val)?)),
            "PR_N0" => Ok(Self::PrN0(pf(val)?)),
            "PRESSURE" => Ok(Self::Pressure(pf(val)?)),
            "RANGE" => Ok(Self::Range(pf(val)?)),
            "RCS" => Ok(Self::Rcs(pf(val)?)),
            "RECEIVE_FREQ" => Ok(Self::ReceiveFreq(pf(val)?)),
            "RECEIVE_FREQ_1" => Ok(Self::ReceiveFreq1(pf(val)?)),
            "RECEIVE_FREQ_2" => Ok(Self::ReceiveFreq2(pf(val)?)),
            "RECEIVE_FREQ_3" => Ok(Self::ReceiveFreq3(pf(val)?)),
            "RECEIVE_FREQ_4" => Ok(Self::ReceiveFreq4(pf(val)?)),
            "RECEIVE_FREQ_5" => Ok(Self::ReceiveFreq5(pf(val)?)),
            "RECEIVE_PHASE_CT_1" => Ok(Self::ReceivePhaseCt1(pf(val)?)),
            "RECEIVE_PHASE_CT_2" => Ok(Self::ReceivePhaseCt2(pf(val)?)),
            "RECEIVE_PHASE_CT_3" => Ok(Self::ReceivePhaseCt3(pf(val)?)),
            "RECEIVE_PHASE_CT_4" => Ok(Self::ReceivePhaseCt4(pf(val)?)),
            "RECEIVE_PHASE_CT_5" => Ok(Self::ReceivePhaseCt5(pf(val)?)),
            "RHUMIDITY" => Ok(Self::Rhumidity(Percentage::new(pf(val)?, None)?)),
            "STEC" => Ok(Self::Stec(pf(val)?)),
            "TEMPERATURE" => Ok(Self::Temperature(pf(val)?)),
            "TRANSMIT_FREQ_1" => Ok(Self::TransmitFreq1(pf(val)?)),
            "TRANSMIT_FREQ_2" => Ok(Self::TransmitFreq2(pf(val)?)),
            "TRANSMIT_FREQ_3" => Ok(Self::TransmitFreq3(pf(val)?)),
            "TRANSMIT_FREQ_4" => Ok(Self::TransmitFreq4(pf(val)?)),
            "TRANSMIT_FREQ_5" => Ok(Self::TransmitFreq5(pf(val)?)),
            "TRANSMIT_FREQ_RATE_1" => Ok(Self::TransmitFreqRate1(pf(val)?)),
            "TRANSMIT_FREQ_RATE_2" => Ok(Self::TransmitFreqRate2(pf(val)?)),
            "TRANSMIT_FREQ_RATE_3" => Ok(Self::TransmitFreqRate3(pf(val)?)),
            "TRANSMIT_FREQ_RATE_4" => Ok(Self::TransmitFreqRate4(pf(val)?)),
            "TRANSMIT_FREQ_RATE_5" => Ok(Self::TransmitFreqRate5(pf(val)?)),
            "TRANSMIT_PHASE_CT_1" => Ok(Self::TransmitPhaseCt1(pf(val)?)),
            "TRANSMIT_PHASE_CT_2" => Ok(Self::TransmitPhaseCt2(pf(val)?)),
            "TRANSMIT_PHASE_CT_3" => Ok(Self::TransmitPhaseCt3(pf(val)?)),
            "TRANSMIT_PHASE_CT_4" => Ok(Self::TransmitPhaseCt4(pf(val)?)),
            "TRANSMIT_PHASE_CT_5" => Ok(Self::TransmitPhaseCt5(pf(val)?)),
            "TROPO_DRY" => Ok(Self::TropoDry(pf(val)?)),
            "TROPO_WET" => Ok(Self::TropoWet(pf(val)?)),
            "VLBI_DELAY" => Ok(Self::VlbiDelay(pf(val)?)),
            _ => Err(crate::error::ValidationError::InvalidValue {
                field: key.to_string().into(),
                value: val.to_string(),
                expected: "valid TDM observation keyword".into(),
                line: None,
            }
            .into()),
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kitchen_sink_roundtrip() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = KITCHEN-SINK-001
META_START
TRACK_ID = TRACK_001
DATA_TYPES = RANGE,DOPPLER_INTEGRATED
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = QUASAR_1
PARTICIPANT_4 = RELAY_SAT
PARTICIPANT_5 = DSS-25
MODE = SEQUENTIAL
PATH = 1,2,1
EPHEMERIS_NAME_1 = DSS14_EPHEM
EPHEMERIS_NAME_2 = SC_EPHEM
EPHEMERIS_NAME_3 = QUASAR_EPHEM
EPHEMERIS_NAME_4 = RELAY_EPHEM
EPHEMERIS_NAME_5 = DSS25_EPHEM
TRANSMIT_BAND = X
RECEIVE_BAND = Ka
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
TIMETAG_REF = RECEIVE
INTEGRATION_INTERVAL = 60.0
INTEGRATION_REF = MIDDLE
FREQ_OFFSET = 0.0
RANGE_MODE = COHERENT
RANGE_MODULUS = 32768.0
RANGE_UNITS = km
ANGLE_TYPE = AZEL
REFERENCE_FRAME = EME2000
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
DOPPLER_COUNT_BIAS = 240000000.0
DOPPLER_COUNT_SCALE = 1000
DOPPLER_COUNT_ROLLOVER = NO
TRANSMIT_DELAY_1 = 0.000077
RECEIVE_DELAY_1 = 0.000088
DATA_QUALITY = VALIDATED
CORRECTION_RANGE = 0.001
CORRECTIONS_APPLIED = YES
META_STOP
DATA_START
COMMENT Range measurements
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
DOPPLER_INTEGRATED = 2023-01-01T00:02:00 -0.5
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).expect("parse kvn");
        let generated = tdm.to_kvn().expect("generate kvn");
        let tdm2 = Tdm::from_kvn(&generated).expect("parse generated kvn");

        assert_eq!(tdm.header, tdm2.header);
        assert_eq!(tdm.body.segments.len(), tdm2.body.segments.len());
        assert_eq!(
            tdm.body.segments[0].metadata.track_id,
            tdm2.body.segments[0].metadata.track_id
        );
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
    }

    #[test]
    fn test_xml_sample_parsing() {
        let xml = include_str!("../../../data/xml/tdm_e21.xml");
        let tdm = Tdm::from_xml(xml).expect("parse xml");
        assert!(!tdm.body.segments.is_empty());

        let generated_kvn = tdm.to_kvn().expect("convert to kvn");
        let tdm2 = Tdm::from_kvn(&generated_kvn).expect("parse generated kvn");

        assert_eq!(tdm.header.creation_date, tdm2.header.creation_date);
    }

    #[test]
    fn test_tdm_validation_path_exclusive() {
        // TDM cannot have both PATH and PATH_1/PATH_2
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = P1
PARTICIPANT_2 = P2
MODE = SEQUENTIAL
PATH = 1,2
PATH_1 = 1,2
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        assert!(Tdm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_tdm_validation_path_pairs() {
        // TDM must have both PATH_1 and PATH_2 if one is present
        let kvn_p1_only = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = P1
PARTICIPANT_2 = P2
MODE = SINGLE_DIFF
PATH_1 = 1,2
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        assert!(Tdm::from_kvn(kvn_p1_only).is_err());
    }

    #[test]
    fn test_tdm_validation_missing_mandatory() {
        // Missing TIME_SYSTEM
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = P1
PARTICIPANT_2 = P2
MODE = SEQUENTIAL
PATH = 1,2
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        assert!(Tdm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_exhaustive_observation_data() {
        // Exercise every variant of TdmObservationData
        use crate::types::Percentage;
        let cases = vec![
            ("ANGLE_1", TdmObservationData::Angle1(1.0)),
            ("ANGLE_2", TdmObservationData::Angle2(2.0)),
            ("CARRIER_POWER", TdmObservationData::CarrierPower(3.0)),
            ("CLOCK_BIAS", TdmObservationData::ClockBias(4.0)),
            ("CLOCK_DRIFT", TdmObservationData::ClockDrift(5.0)),
            ("DOPPLER_COUNT", TdmObservationData::DopplerCount(6.0)),
            (
                "DOPPLER_INSTANTANEOUS",
                TdmObservationData::DopplerInstantaneous(7.0),
            ),
            (
                "DOPPLER_INTEGRATED",
                TdmObservationData::DopplerIntegrated(8.0),
            ),
            ("DOR", TdmObservationData::Dor(9.0)),
            ("MAG", TdmObservationData::Mag(10.0)),
            ("PC_N0", TdmObservationData::PcN0(11.0)),
            ("PR_N0", TdmObservationData::PrN0(12.0)),
            ("PRESSURE", TdmObservationData::Pressure(13.0)),
            ("RANGE", TdmObservationData::Range(14.0)),
            ("RCS", TdmObservationData::Rcs(15.0)),
            ("RECEIVE_FREQ", TdmObservationData::ReceiveFreq(16.0)),
            ("RECEIVE_FREQ_1", TdmObservationData::ReceiveFreq1(17.0)),
            ("RECEIVE_FREQ_2", TdmObservationData::ReceiveFreq2(18.0)),
            ("RECEIVE_FREQ_3", TdmObservationData::ReceiveFreq3(19.0)),
            ("RECEIVE_FREQ_4", TdmObservationData::ReceiveFreq4(20.0)),
            ("RECEIVE_FREQ_5", TdmObservationData::ReceiveFreq5(21.0)),
            (
                "RECEIVE_PHASE_CT_1",
                TdmObservationData::ReceivePhaseCt1(22.0),
            ),
            (
                "RECEIVE_PHASE_CT_2",
                TdmObservationData::ReceivePhaseCt2(23.0),
            ),
            (
                "RECEIVE_PHASE_CT_3",
                TdmObservationData::ReceivePhaseCt3(24.0),
            ),
            (
                "RECEIVE_PHASE_CT_4",
                TdmObservationData::ReceivePhaseCt4(25.0),
            ),
            (
                "RECEIVE_PHASE_CT_5",
                TdmObservationData::ReceivePhaseCt5(26.0),
            ),
            (
                "RHUMIDITY",
                TdmObservationData::Rhumidity(Percentage::new(50.0, None).unwrap()),
            ),
            ("STEC", TdmObservationData::Stec(27.0)),
            ("TEMPERATURE", TdmObservationData::Temperature(28.0)),
            ("TRANSMIT_FREQ_1", TdmObservationData::TransmitFreq1(29.0)),
            ("TRANSMIT_FREQ_2", TdmObservationData::TransmitFreq2(30.0)),
            ("TRANSMIT_FREQ_3", TdmObservationData::TransmitFreq3(31.0)),
            ("TRANSMIT_FREQ_4", TdmObservationData::TransmitFreq4(32.0)),
            ("TRANSMIT_FREQ_5", TdmObservationData::TransmitFreq5(33.0)),
            (
                "TRANSMIT_FREQ_RATE_1",
                TdmObservationData::TransmitFreqRate1(34.0),
            ),
            (
                "TRANSMIT_FREQ_RATE_2",
                TdmObservationData::TransmitFreqRate2(35.0),
            ),
            (
                "TRANSMIT_FREQ_RATE_3",
                TdmObservationData::TransmitFreqRate3(36.0),
            ),
            (
                "TRANSMIT_FREQ_RATE_4",
                TdmObservationData::TransmitFreqRate4(37.0),
            ),
            (
                "TRANSMIT_FREQ_RATE_5",
                TdmObservationData::TransmitFreqRate5(38.0),
            ),
            (
                "TRANSMIT_PHASE_CT_1",
                TdmObservationData::TransmitPhaseCt1(39.0),
            ),
            (
                "TRANSMIT_PHASE_CT_2",
                TdmObservationData::TransmitPhaseCt2(40.0),
            ),
            (
                "TRANSMIT_PHASE_CT_3",
                TdmObservationData::TransmitPhaseCt3(41.0),
            ),
            (
                "TRANSMIT_PHASE_CT_4",
                TdmObservationData::TransmitPhaseCt4(42.0),
            ),
            (
                "TRANSMIT_PHASE_CT_5",
                TdmObservationData::TransmitPhaseCt5(43.0),
            ),
            ("TROPO_DRY", TdmObservationData::TropoDry(44.0)),
            ("TROPO_WET", TdmObservationData::TropoWet(45.0)),
            ("VLBI_DELAY", TdmObservationData::VlbiDelay(46.0)),
        ];

        for (expected_key, data) in cases {
            assert_eq!(data.key(), expected_key);
            let val_str = data.value_to_string();
            let parsed = TdmObservationData::from_key_val(expected_key, &val_str).unwrap();
            assert_eq!(data, parsed);
        }
    }

    #[test]
    fn test_tdm_metadata_indexed_fields() {
        let meta = TdmMetadata::builder()
            .time_system("UTC")
            .participant_1("P1")
            .participant_2("P2")
            .participant_3("P3")
            .participant_4("P4")
            .participant_5("P5")
            .ephemeris_name_1("E1")
            .ephemeris_name_2("E2")
            .ephemeris_name_3("E3")
            .ephemeris_name_4("E4")
            .ephemeris_name_5("E5")
            .transmit_delay_1(0.1)
            .transmit_delay_2(0.2)
            .transmit_delay_3(0.3)
            .transmit_delay_4(0.4)
            .transmit_delay_5(0.5)
            .receive_delay_1(1.1)
            .receive_delay_2(1.2)
            .receive_delay_3(1.3)
            .receive_delay_4(1.4)
            .receive_delay_5(1.5)
            .build();

        let mut writer = KvnWriter::new();
        meta.write_kvn(&mut writer);
        let out = writer.finish();
        assert!(out.contains("PARTICIPANT_5"));
        assert!(out.contains("EPHEMERIS_NAME_5"));
        assert!(out.contains("TRANSMIT_DELAY_5"));
        assert!(out.contains("RECEIVE_DELAY_5"));
    }

    #[test]
    fn test_tdm_correction_keywords() {
        let meta = TdmMetadata::builder()
            .time_system("UTC")
            .participant_1("P1")
            .correction_angle_1(0.1)
            .correction_angle_2(0.2)
            .correction_doppler(0.3)
            .correction_mag(0.4)
            .correction_range(0.5)
            .correction_rcs(0.6)
            .correction_receive(0.7)
            .correction_transmit(0.8)
            .correction_aberration_yearly(0.9)
            .correction_aberration_diurnal(1.0)
            .corrections_applied(YesNo::Yes)
            .build();

        let mut writer = KvnWriter::new();
        meta.write_kvn(&mut writer);
        let out = writer.finish();
        assert!(out.contains("CORRECTION_ANGLE_1"));
        assert!(out.contains("CORRECTION_ABERRATION_DIURNAL"));
        assert!(out.contains("CORRECTIONS_APPLIED"));
    }

    #[test]
    fn test_tdm_observation_data_errors() {
        // Invalid float
        assert!(TdmObservationData::from_key_val("RANGE", "abc").is_err());
        // Unknown key
        assert!(TdmObservationData::from_key_val("UNKNOWN", "1.0").is_err());
    }

    #[test]
    fn test_tdm_xml_exhaustive_observations() {
        // Exercise XML deserializer for a wide range of observation types
        let xml = r#"<tdm version="2.0">
  <header>
    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
    <ORIGINATOR>TEST</ORIGINATOR>
  </header>
  <body>
    <segment>
      <metadata>
        <TIME_SYSTEM>UTC</TIME_SYSTEM>
        <PARTICIPANT_1>P1</PARTICIPANT_1>
      </metadata>
      <data>
        <observation>
          <EPOCH>2023-01-01T00:00:00</EPOCH>
          <ANGLE_1>1.0</ANGLE_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:01:00</EPOCH>
          <ANGLE_2>2.0</ANGLE_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:02:00</EPOCH>
          <CARRIER_POWER>3.0</CARRIER_POWER>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:03:00</EPOCH>
          <CLOCK_BIAS>4.0</CLOCK_BIAS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:04:00</EPOCH>
          <CLOCK_DRIFT>5.0</CLOCK_DRIFT>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:05:00</EPOCH>
          <DOPPLER_COUNT>6.0</DOPPLER_COUNT>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:06:00</EPOCH>
          <DOPPLER_INSTANTANEOUS>7.0</DOPPLER_INSTANTANEOUS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:07:00</EPOCH>
          <DOPPLER_INTEGRATED>8.0</DOPPLER_INTEGRATED>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:08:00</EPOCH>
          <DOR>9.0</DOR>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:09:00</EPOCH>
          <MAG>10.0</MAG>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:10:00</EPOCH>
          <PC_N0>11.0</PC_N0>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:11:00</EPOCH>
          <PR_N0>12.0</PR_N0>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:12:00</EPOCH>
          <PRESSURE>13.0</PRESSURE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:13:00</EPOCH>
          <RANGE>14.0</RANGE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:14:00</EPOCH>
          <RCS>15.0</RCS>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:15:00</EPOCH>
          <RECEIVE_FREQ>16.0</RECEIVE_FREQ>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:16:00</EPOCH>
          <RECEIVE_FREQ_1>17.0</RECEIVE_FREQ_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:17:00</EPOCH>
          <RECEIVE_FREQ_2>18.0</RECEIVE_FREQ_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:18:00</EPOCH>
          <RECEIVE_FREQ_3>19.0</RECEIVE_FREQ_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:19:00</EPOCH>
          <RECEIVE_FREQ_4>20.0</RECEIVE_FREQ_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:20:00</EPOCH>
          <RECEIVE_FREQ_5>21.0</RECEIVE_FREQ_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:21:00</EPOCH>
          <RECEIVE_PHASE_CT_1>22.0</RECEIVE_PHASE_CT_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:22:00</EPOCH>
          <RECEIVE_PHASE_CT_2>23.0</RECEIVE_PHASE_CT_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:23:00</EPOCH>
          <RECEIVE_PHASE_CT_3>24.0</RECEIVE_PHASE_CT_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:24:00</EPOCH>
          <RECEIVE_PHASE_CT_4>25.0</RECEIVE_PHASE_CT_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:25:00</EPOCH>
          <RECEIVE_PHASE_CT_5>26.0</RECEIVE_PHASE_CT_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:26:00</EPOCH>
          <RHUMIDITY>27.0</RHUMIDITY>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:27:00</EPOCH>
          <STEC>28.0</STEC>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:28:00</EPOCH>
          <TEMPERATURE>29.0</TEMPERATURE>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:29:00</EPOCH>
          <TRANSMIT_FREQ_1>30.0</TRANSMIT_FREQ_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:30:00</EPOCH>
          <TRANSMIT_FREQ_2>31.0</TRANSMIT_FREQ_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:31:00</EPOCH>
          <TRANSMIT_FREQ_3>32.0</TRANSMIT_FREQ_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:32:00</EPOCH>
          <TRANSMIT_FREQ_4>33.0</TRANSMIT_FREQ_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:33:00</EPOCH>
          <TRANSMIT_FREQ_5>34.0</TRANSMIT_FREQ_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:34:00</EPOCH>
          <TRANSMIT_FREQ_RATE_1>35.0</TRANSMIT_FREQ_RATE_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:35:00</EPOCH>
          <TRANSMIT_FREQ_RATE_2>36.0</TRANSMIT_FREQ_RATE_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:36:00</EPOCH>
          <TRANSMIT_FREQ_RATE_3>37.0</TRANSMIT_FREQ_RATE_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:37:00</EPOCH>
          <TRANSMIT_FREQ_RATE_4>38.0</TRANSMIT_FREQ_RATE_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:38:00</EPOCH>
          <TRANSMIT_FREQ_RATE_5>39.0</TRANSMIT_FREQ_RATE_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:39:00</EPOCH>
          <TRANSMIT_PHASE_CT_1>40.0</TRANSMIT_PHASE_CT_1>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:40:00</EPOCH>
          <TRANSMIT_PHASE_CT_2>41.0</TRANSMIT_PHASE_CT_2>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:41:00</EPOCH>
          <TRANSMIT_PHASE_CT_3>42.0</TRANSMIT_PHASE_CT_3>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:42:00</EPOCH>
          <TRANSMIT_PHASE_CT_4>43.0</TRANSMIT_PHASE_CT_4>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:43:00</EPOCH>
          <TRANSMIT_PHASE_CT_5>44.0</TRANSMIT_PHASE_CT_5>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:44:00</EPOCH>
          <TROPO_DRY>45.0</TROPO_DRY>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:45:00</EPOCH>
          <TROPO_WET>46.0</TROPO_WET>
        </observation>
        <observation>
          <EPOCH>2023-01-01T00:46:00</EPOCH>
          <VLBI_DELAY>47.0</VLBI_DELAY>
        </observation>
      </data>
    </segment>
  </body>
</tdm>"#;
        let tdm = Tdm::from_xml(xml).expect("parse tdm xml");
        assert_eq!(tdm.body.segments[0].data.observations.len(), 47);
        let obs3 = &tdm.body.segments[0].data.observations[20];
        match obs3.data {
            TdmObservationData::ReceiveFreq5(v) => assert_eq!(v, 21.0),
            _ => panic!("Expected ReceiveFreq5"),
        }

        // Test duplicate EPOCH error handling
        let xml_dup = r#"<tdm version="2.0">
  <header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE><ORIGINATOR>T</ORIGINATOR></header>
  <body><segment><metadata><TIME_SYSTEM>UTC</TIME_SYSTEM><PARTICIPANT_1>P</PARTICIPANT_1></metadata>
  <data><observation><EPOCH>2023-01-01T00:00:00</EPOCH><EPOCH>2023-01-01T00:00:01</EPOCH><RANGE>1.0</RANGE></observation></data>
  </segment></body></tdm>"#;
        assert!(Tdm::from_xml(xml_dup).is_err());

        // Test unknown attribute/field skip
        let xml_unknown = r#"<tdm version="2.0" extra="val">
  <header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE><ORIGINATOR>T</ORIGINATOR></header>
  <body><segment><metadata><TIME_SYSTEM>UTC</TIME_SYSTEM><PARTICIPANT_1>P</PARTICIPANT_1></metadata>
  <data><observation extra="ignore"><EPOCH>2023-01-01T00:00:00</EPOCH><RANGE>1.0</RANGE></observation></data>
  </segment></body></tdm>"#;
        assert!(Tdm::from_xml(xml_unknown).is_ok());
    }
}
