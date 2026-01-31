// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::Result;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::{Epoch, PositionCovariance, PositionVelocityCovariance, VelocityCovariance};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::num::NonZeroU32;

// Re-export CcsdsNdmError for use in tests
#[cfg(test)]
#[allow(unused_imports)]
use crate::error::CcsdsNdmError;

//----------------------------------------------------------------------
// Root OEM Structure
//----------------------------------------------------------------------

/// Orbit Ephemeris Message (OEM).
///
/// An OEM specifies the position and velocity of a single object at multiple epochs contained
/// within a specified time range. The message recipient must have a means of interpolating
/// across these state vectors to obtain the state at an arbitrary time contained within the
/// span of the ephemeris.
///
/// The OEM is suited to exchanges that:
/// 1. Involve automated interaction (e.g., computer-to-computer communication).
/// 2. Require higher fidelity or higher precision dynamic modeling than is possible with the OPM.
///
/// **CCSDS Reference**: 502.0-B-3, Section 5.1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "oem")]
pub struct Oem {
    #[serde(rename = "@id")]
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
    pub header: OdmHeader,
    pub body: OemBody,
}

impl Oem {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.validate()
    }
}

impl OemBody {
    pub fn validate(&self) -> Result<()> {
        if let Some(first) = self.segment.first() {
            let ts = &first.metadata.time_system;
            for segment in &self.segment[1..] {
                if segment.metadata.time_system != *ts {
                    return Err(crate::error::ValidationError::InvalidValue {
                        field: "TIME_SYSTEM".into(),
                        value: segment.metadata.time_system.clone(),
                        expected: format!(
                            "consistent TIME_SYSTEM across OEM segments (expected {})",
                            ts
                        )
                        .into(),
                        line: None,
                    }
                    .into());
                }
            }
        }
        for segment in &self.segment {
            segment.validate()?;
        }
        Ok(())
    }
}

impl OemSegment {
    pub fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()
    }
}

impl OemMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.interpolation.is_some() && self.interpolation_degree.is_none() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "INTERPOLATION_DEGREE (required when INTERPOLATION is present)".into(),
                line: None,
            }
            .into());
        }
        if self.start_time.as_str() > self.stop_time.as_str() {
            return Err(crate::error::ValidationError::Generic {
                message: "START_TIME must be <= STOP_TIME".into(),
                line: None,
            }
            .into());
        }
        if let (Some(start), Some(end)) = (&self.useable_start_time, &self.useable_stop_time) {
            if start.as_str() > end.as_str() {
                return Err(crate::error::ValidationError::Generic {
                    message: "USEABLE_START_TIME must be <= USEABLE_STOP_TIME".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

impl OemData {
    pub fn validate(&self) -> Result<()> {
        if self.state_vector.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Data".into(),
                field: "stateVector (at least one required)".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl crate::traits::Validate for Oem {
    fn validate(&self) -> Result<()> {
        Oem::validate(self)
    }
}

impl Ndm for Oem {
    fn to_kvn(&self) -> Result<String> {
        // Estimate capacity: header + (metadata + state vectors + covariance) for each segment
        let mut total_records = 0;
        for seg in &self.body.segment {
            total_records += seg.data.state_vector.len();
            total_records += seg.data.covariance_matrix.len() * 7; // Approx lines per cov
        }
        let estimated_capacity = total_records * 150 + 4096;
        let mut writer = KvnWriter::with_capacity(estimated_capacity);
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let oem = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Oem, &oem)?;
        Ok(oem)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let oem: Self = crate::xml::from_str_with_context(xml, "OEM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Oem, &oem)?;
        Ok(oem)
    }
}

impl ToKvn for Oem {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_OEM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segments
//----------------------------------------------------------------------

/// The body of the OEM, containing one or more segments.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OemBody {
    #[serde(rename = "segment")]
    #[builder(default)]
    pub segment: Vec<OemSegment>,
}

impl ToKvn for OemBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for seg in &self.segment {
            seg.write_kvn(writer);
        }
    }
}

/// A single segment of the OEM.
///
/// Each segment contains metadata (context) and a list of ephemeris data points.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OemSegment {
    pub metadata: OemMetadata,
    pub data: OemData,
}

impl ToKvn for OemSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        self.metadata.write_kvn(writer);
        writer.write_section("META_STOP");
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// OEM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OemMetadata {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which ephemeris data is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference `[3]`, which include Object name
    /// and international designator of the participant). If OBJECT_NAME is not listed in
    /// reference `[3]` or the content is either unknown or cannot be disclosed, the value should
    /// be set to UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Object identifier of the object for which ephemeris data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use the
    /// international spacecraft designator as published in the UN Office of Outer Space Affairs
    /// designator index. Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year
    /// of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
    /// P{PP} = At least one capital letter for the identification of the part brought into
    /// space by the launch. If the asset is not listed, the UN Office of Outer Space Affairs
    /// designator index format is not used, or the content is either unknown or cannot be
    /// disclosed, the value should be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Origin of the OEM reference frame, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the
    /// solar system barycenter, or another reference frame center (such as a spacecraft,
    /// formation flying reference ‘chief’ spacecraft, etc.). Natural bodies shall be selected
    /// from the accepted set of values indicated in annex B, subsection B2. For spacecraft, it
    /// is recommended to use either the OBJECT_ID or international designator of the
    /// participant as catalogued in the UN Office of Outer Space Affairs designator index
    /// (reference `[3]`).
    ///
    /// **Examples**: EARTH, EARTH BARYCENTER, MOON, SOLAR SYSTEM BARYCENTER, SUN,
    /// JUPITER BARYCENTER, STS 106, EROS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[builder(into)]
    pub center_name: String,
    /// Reference frame in which the ephemeris data are given. Use of values other than those in
    /// 3.2.3.3 should be documented in an ICD.
    ///
    /// **Examples**: ICRF, ITRF2000, EME2000, TEME
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[builder(into)]
    pub ref_frame: String,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame.
    /// (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    /// Time system used for ephemeris and covariance data. Use of values other than those in
    /// 3.2.3.2 should be documented in an ICD.
    ///
    /// **Examples**: UTC, TAI, TT, GPS, TDB, TCB
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[builder(into)]
    pub time_system: String,
    /// Start of TOTAL time span covered by ephemeris data and covariance data immediately
    /// following this metadata block. (For format specification, see 7.5.10.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    pub start_time: Epoch,
    /// Start time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
    /// message creator to introduce fictitious (but numerically smooth) data nodes prior to the
    /// actual data time history to support interpolation methods requiring more than two nodes
    /// (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword and
    /// introduction of fictitious node points are optional and may not be necessary.
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    /// Stop time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) This optional keyword allows the
    /// message creator to introduce fictitious (but numerically smooth) data nodes following
    /// the actual data time history to support interpolation methods requiring more than two
    /// nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this keyword
    /// and introduction of fictitious node points are optional and may not be necessary.
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    /// End of TOTAL time span covered by ephemeris data and covariance data immediately
    /// following this metadata block. (For format specification, see 7.5.10.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    pub stop_time: Epoch,
    /// This keyword may be used to specify the recommended interpolation method for ephemeris
    /// data in the immediately following set of ephemeris lines.
    ///
    /// **Examples**: HERMITE, LINEAR, LAGRANGE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub interpolation: Option<String>,
    /// Recommended interpolation degree for ephemeris data in the immediately following set of
    /// ephemeris lines. Must be an integer value. This keyword must be used if the
    /// ‘INTERPOLATION’ keyword is used.
    ///
    /// **Examples**: 5, 8
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<NonZeroU32>,
}

impl ToKvn for OemMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("REF_FRAME", &self.ref_frame);
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("START_TIME", self.start_time);
        if let Some(v) = &self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = &self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        writer.write_pair("STOP_TIME", self.stop_time);
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = &self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

//----------------------------------------------------------------------
// Data Section
//----------------------------------------------------------------------

/// OEM Data Section.
///
/// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OemData {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,

    /// List of state vectors. Each vector contains position, velocity, and optional
    /// acceleration.
    ///
    /// **Examples**: 2020-01-01T00:00:00.000 1234.567 2345.678 3456.789 1.234 2.345 3.456
    ///
    /// **Units**: km, km/s, km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
    #[serde(rename = "stateVector", default)]
    #[builder(default)]
    pub state_vector: Vec<StateVectorAcc>,

    /// List of covariance matrices (optional).
    ///
    /// **Units**: km², km²/s, km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub covariance_matrix: Vec<OemCovarianceMatrix>,
}

impl ToKvn for OemData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if !self.state_vector.is_empty() {
            writer.write_empty();
        }
        for sv in &self.state_vector {
            sv.write_kvn(writer);
        }
        for cov in &self.covariance_matrix {
            writer.write_empty();
            cov.write_kvn(writer);
        }
    }
}

//----------------------------------------------------------------------
// Covariance Matrix
//----------------------------------------------------------------------

/// OEM Covariance Matrix.
///
/// Represents a 6x6 symmetric covariance matrix for position and velocity at a specific epoch.
/// The lower triangular portion is stored/transmitted.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OemCovarianceMatrix {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of covariance matrix. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub epoch: Epoch,
    /// Reference frame in which the covariance data are given. Select from the accepted set of
    /// values indicated in 3.2.3.3 or 3.2.4.11.
    ///
    /// **Examples**: ICRF, EME2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_ref_frame: Option<String>,

    /// Covariance matrix `[1,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cx_x: PositionCovariance,
    /// Covariance matrix `[2,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_x: PositionCovariance,
    /// Covariance matrix `[2,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_y: PositionCovariance,
    /// Covariance matrix `[3,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_x: PositionCovariance,
    /// Covariance matrix `[3,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_y: PositionCovariance,
    /// Covariance matrix `[3,3]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_z: PositionCovariance,

    /// Covariance matrix `[4,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cx_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[4,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cx_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[4,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cx_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[4,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cx_dot_x_dot: VelocityCovariance,

    /// Covariance matrix `[5,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[5,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[5,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[5,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[5,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cy_dot_y_dot: VelocityCovariance,

    /// Covariance matrix `[6,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[6,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[6,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[6,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[6,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_y_dot: VelocityCovariance,
    /// Covariance matrix `[6,6]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    pub cz_dot_z_dot: VelocityCovariance,
}

impl ToKvn for OemCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COVARIANCE_START");
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        if let Some(rf) = &self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", rf);
        }

        let mut b = zmij::Buffer::new();

        // Lower triangular formatting strict compliance (1, 2, 3, 4, 5, 6 items per line)
        writer.write_line(b.format_finite(self.cx_x.value));

        let _ = writer.write_str(b.format_finite(self.cy_x.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format_finite(self.cy_y.value));

        let _ = writer.write_str(b.format_finite(self.cz_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cz_y.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format_finite(self.cz_z.value));

        let _ = writer.write_str(b.format_finite(self.cx_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cx_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cx_dot_z.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format_finite(self.cx_dot_x_dot.value));

        let _ = writer.write_str(b.format_finite(self.cy_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cy_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cy_dot_z.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cy_dot_x_dot.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format_finite(self.cy_dot_y_dot.value));

        let _ = writer.write_str(b.format_finite(self.cz_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cz_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cz_dot_z.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cz_dot_x_dot.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format_finite(self.cz_dot_y_dot.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format_finite(self.cz_dot_z_dot.value));

        writer.write_section("COVARIANCE_STOP");
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    #[test]
    fn test_header_optional_fields_roundtrip() {
        // A2.5.3 Items 3,4,7: COMMENT, CLASSIFICATION, MESSAGE_ID optional
        let kvn = r#"CCSDS_OEM_VERS = 3.0
COMMENT This is a header comment
CLASSIFICATION = SBU
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let out = oem.to_kvn().unwrap();
        assert!(out.contains("CLASSIFICATION"));
        assert!(out.contains("MESSAGE_ID"));
        let oem2 = Oem::from_kvn(&out).unwrap();
        assert_eq!(oem.header.classification, oem2.header.classification);
        assert_eq!(oem.header.message_id, oem2.header.message_id);
    }

    #[test]
    fn test_metadata_optional_fields() {
        // A2.5.3 Items 10, 15, 18, 19: Optional metadata fields
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT This is a metadata comment
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
REF_FRAME_EPOCH = 2000-01-01T00:00:00
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T01:00:00
USEABLE_STOP_TIME = 2023-01-01T23:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T01:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let meta = &oem.body.segment[0].metadata;
        assert_eq!(meta.comment, vec!["This is a metadata comment"]);
        assert!(meta.ref_frame_epoch.is_some());
        assert!(meta.useable_start_time.is_some());
        assert!(meta.useable_stop_time.is_some());

        let out = oem.to_kvn().unwrap();
        assert!(out.contains("COMMENT This is a metadata comment"));
        assert!(out.contains("REF_FRAME_EPOCH"));
        assert!(out.contains("USEABLE_START_TIME"));
        assert!(out.contains("USEABLE_STOP_TIME"));
    }

    #[test]
    fn test_data_comments() {
        // Test for comments within the data section
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
COMMENT This is a data section comment
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COMMENT Another data comment
2023-01-01T00:01:00 1060 2120 3180 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let data = &oem.body.segment[0].data;
        assert_eq!(
            data.comment,
            vec!["This is a data section comment", "Another data comment"]
        );
        assert_eq!(data.state_vector.len(), 2);

        let out = oem.to_kvn().unwrap();
        assert!(out.contains("COMMENT This is a data section comment"));
    }

    #[test]
    fn test_write_kvn() {
        // Parse then Write then Parse check
        let kvn_in = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-11-26T12:00:00
ORIGINATOR = RUST_TEST
META_START
OBJECT_NAME = TEST_SAT
OBJECT_ID = 12345
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
START_TIME = 2023-11-26T12:00:00
STOP_TIME = 2023-11-26T13:00:00
META_STOP
2023-11-26T12:00:00 6000.0 0.0 0.0 0.0 7.5 0.0
"#;
        let oem = Oem::from_kvn(kvn_in).unwrap();
        let kvn_out = oem.to_kvn().unwrap();

        let oem2 = Oem::from_kvn(&kvn_out).unwrap();
        assert_eq!(oem.header.originator, oem2.header.originator);
        assert_eq!(
            oem.body.segment[0].data.state_vector[0].epoch,
            oem2.body.segment[0].data.state_vector[0].epoch
        );
    }

    #[test]
    fn test_xsd_xml_roundtrip() {
        // Parse XML -> Write XML -> Parse XML should produce same result
        let xml = include_str!("../../../data/xml/oem_g14.xml");
        let oem1 = Oem::from_xml(xml).unwrap();
        let xml_out = oem1.to_xml().unwrap();
        let oem2 = Oem::from_xml(&xml_out).unwrap();

        assert_eq!(oem1.version, oem2.version);
        assert_eq!(oem1.header.originator, oem2.header.originator);
        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());

        let seg1 = &oem1.body.segment[0];
        let seg2 = &oem2.body.segment[0];
        assert_eq!(seg1.metadata.object_name, seg2.metadata.object_name);
        assert_eq!(seg1.data.state_vector.len(), seg2.data.state_vector.len());
        assert_eq!(
            seg1.data.covariance_matrix.len(),
            seg2.data.covariance_matrix.len()
        );
    }

    #[test]
    fn test_xsd_kvn_roundtrip() {
        // Parse KVN -> Write KVN -> Parse KVN should produce same result
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = HERMITE
INTERPOLATION_DEGREE = 5
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0 0.001 0.002 0.003
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
COV_REF_FRAME = RTN
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem1 = Oem::from_kvn(kvn).unwrap();
        let kvn_out = oem1.to_kvn().unwrap();
        let oem2 = Oem::from_kvn(&kvn_out).unwrap();

        assert_eq!(oem1.version, oem2.version);
        assert_eq!(oem1.header.originator, oem2.header.originator);
        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());

        let meta1 = &oem1.body.segment[0].metadata;
        let meta2 = &oem2.body.segment[0].metadata;
        assert_eq!(meta1.object_name, meta2.object_name);
        assert_eq!(meta1.interpolation, meta2.interpolation);
        assert_eq!(meta1.interpolation_degree, meta2.interpolation_degree);

        let data1 = &oem1.body.segment[0].data;
        let data2 = &oem2.body.segment[0].data;
        assert_eq!(data1.state_vector.len(), data2.state_vector.len());
        assert_eq!(data1.covariance_matrix.len(), data2.covariance_matrix.len());
    }

    #[test]
    fn test_xsd_kvn_sample_file_roundtrip() {
        // Parse sample KVN file and verify roundtrip
        let kvn = include_str!("../../../data/kvn/oem_g11.kvn");
        let oem1 = Oem::from_kvn(kvn).unwrap();
        let kvn_out = oem1.to_kvn().unwrap();
        let oem2 = Oem::from_kvn(&kvn_out).unwrap();

        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());
        for (seg1, seg2) in oem1.body.segment.iter().zip(oem2.body.segment.iter()) {
            assert_eq!(seg1.metadata.object_name, seg2.metadata.object_name);
            assert_eq!(seg1.data.state_vector.len(), seg2.data.state_vector.len());
        }
    }

    #[test]
    fn test_xsd_parse_xml_oem_g14() {
        // Parse official CCSDS sample file oem_g14.xml
        let xml = include_str!("../../../data/xml/oem_g14.xml");
        let oem = Oem::from_xml(xml).expect("Failed to parse oem_g14.xml");
        assert_eq!(oem.version, "3.0");
        assert_eq!(oem.header.originator, "NASA/JPL");
        assert!(oem.header.message_id.is_some());
        assert_eq!(oem.body.segment.len(), 1);
        // Verify state vectors with optional accelerations
        let seg = &oem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert_eq!(seg.data.state_vector.len(), 4);
        // XML sample has accelerations
        assert!(seg.data.state_vector[0].x_ddot.is_some());
        // XML sample has covariance
        assert_eq!(seg.data.covariance_matrix.len(), 1);
        assert!(seg.data.covariance_matrix[0].cov_ref_frame.is_some());
    }

    #[test]
    fn full_optional_fields_roundtrip() {
        let kvn = r#"
CCSDS_OEM_VERS = 3.0
COMMENT Header comment
CLASSIFICATION = UNCLASSIFIED
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

META_START
COMMENT Metadata comment
OBJECT_NAME = TEST_OBJ
OBJECT_ID = 12345
CENTER_NAME = EARTH
REF_FRAME = EME2000
REF_FRAME_EPOCH = 2000-01-01T00:00:00
TIME_SYSTEM = UTC
START_TIME = 2025-01-01T00:00:00
USEABLE_START_TIME = 2025-01-01T00:10:00
USEABLE_STOP_TIME = 2025-01-02T23:50:00
STOP_TIME = 2025-01-02T00:00:00
INTERPOLATION = HERMITE
INTERPOLATION_DEGREE = 7
META_STOP

COMMENT Data comment
2025-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0 3.0 0.01 0.02 0.03

COVARIANCE_START
EPOCH = 2025-01-01T00:00:00
COV_REF_FRAME = EME2000
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn).expect("parse full oem");
        let regenerated = oem.to_kvn().expect("generate full kvn");
        let oem2 = Oem::from_kvn(&regenerated).expect("parse regenerated full oem");

        assert_eq!(oem.header.message_id, oem2.header.message_id);
        assert_eq!(
            oem.body.segment[0].metadata.ref_frame_epoch,
            oem2.body.segment[0].metadata.ref_frame_epoch
        );
        assert_eq!(
            oem.body.segment[0].data.state_vector[0]
                .x_ddot
                .as_ref()
                .map(|v| v.value),
            Some(0.01)
        );
        assert_eq!(
            oem.body.segment[0].data.covariance_matrix[0]
                .cov_ref_frame
                .as_deref(),
            Some("EME2000")
        );
    }

    #[test]
    fn test_oem_validation_interpolation_reqs() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = HERMITE
# Missing INTERPOLATION_DEGREE
META_STOP
2023-01-01T00:00:00 1 2 3 4 5 6
"#;
        assert!(Oem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_oem_validation_time_range() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-02T00:00:00
STOP_TIME = 2023-01-01T00:00:00
META_STOP
2023-01-01T00:00:00 1 2 3 4 5 6
"#;
        assert!(Oem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_oem_validation_empty_state_vector() {
        // Construct KVN without data lines?
        // Parser logic for OEM data: it expects lines or comments until next block.
        // If no lines, `state_vector` will be empty.
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
COMMENT No data
"#;
        assert!(Oem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_oem_metadata_time_validation() {
        let mut meta = OemMetadata::builder()
            .object_name("SAT")
            .object_id("1")
            .center_name("EARTH")
            .ref_frame("GCRF")
            .time_system("UTC")
            .start_time(Epoch::new("2023-01-01T12:00:00").unwrap())
            .stop_time(Epoch::new("2023-01-01T11:00:00").unwrap()) // STOP < START
            .build();

        // START > STOP
        assert!(meta.validate().is_err());

        meta.stop_time = Epoch::new("2023-01-01T13:00:00").unwrap();
        assert!(meta.validate().is_ok());

        // USEABLE_START > USEABLE_STOP
        meta.useable_start_time = Some(Epoch::new("2023-01-01T12:30:00").unwrap());
        meta.useable_stop_time = Some(Epoch::new("2023-01-01T12:15:00").unwrap());
        assert!(meta.validate().is_err());
    }

    #[test]
    fn test_oem_metadata_interpolation_validation() {
        let mut meta = OemMetadata::builder()
            .object_name("SAT")
            .object_id("1")
            .center_name("EARTH")
            .ref_frame("GCRF")
            .time_system("UTC")
            .start_time(Epoch::new("2023-01-01T12:00:00").unwrap())
            .stop_time(Epoch::new("2023-01-01T13:00:00").unwrap())
            .build();

        meta.interpolation = Some("LAGRANGE".to_string());
        // Missing degree
        assert!(meta.validate().is_err());

        meta.interpolation_degree = Some(NonZeroU32::new(5).unwrap());
        assert!(meta.validate().is_ok());
    }

    #[test]
    fn test_oem_data_empty_validation_internal() {
        let data = OemData::builder().build();
        assert!(data.validate().is_err());
    }
}
