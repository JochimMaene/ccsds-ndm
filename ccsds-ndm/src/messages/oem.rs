// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::{
    CalendarEpoch, Epoch, InterpolationDegree, PositionCovariance, PositionVelocityCovariance,
    VelocityCovariance,
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[cfg(test)]
use crate::traits::Validate;
#[cfg(test)]
use std::num::NonZeroU32;

// Re-export CcsdsNdmError for use in tests
#[cfg(test)]
#[allow(unused_imports)]
use crate::error::CcsdsNdmError;

fn contextual_epoch_error(epoch: &Epoch, field: &'static str) -> Option<ValidationError> {
    (!epoch.is_contextually_valid()).then(|| ValidationError::InvalidValue {
        field: field.into(),
        value: epoch.to_string(),
        expected: "a valid calendar, ordinal, or non-degenerate numeric epoch".into(),
        line: None,
    })
}

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
    #[builder(required, default = Some("CCSDS_OEM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "3.0".to_string(), into)]
    pub version: String,
    pub header: OdmHeader,
    pub body: OemBody,
}

impl crate::traits::Validate for Oem {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Oem,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        crate::validation::collect_message_validation_errors(
            crate::validation::MessageKind::Oem,
            &self.id,
            &self.version,
            &self.header,
            &self.body,
        )
    }
}

impl crate::traits::Validate for OemBody {
    fn validate(&self) -> Result<()> {
        if self.segment.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Body".into(),
                field: "segment (at least one required)".into(),
                line: None,
            }
            .into());
        }
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

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        let mut errors = Vec::new();
        if self.segment.is_empty() {
            errors.push(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Body".into(),
                field: "segment (at least one required)".into(),
                line: None,
            });
        }
        if let Some(first) = self.segment.first() {
            let time_system = &first.metadata.time_system;
            for segment in &self.segment[1..] {
                if segment.metadata.time_system != *time_system {
                    errors.push(crate::error::ValidationError::InvalidValue {
                        field: "TIME_SYSTEM".into(),
                        value: segment.metadata.time_system.clone(),
                        expected: format!(
                            "consistent TIME_SYSTEM across OEM segments (expected {time_system})"
                        )
                        .into(),
                        line: None,
                    });
                }
            }
        }
        for segment in &self.segment {
            errors.extend(segment.validation_errors()?);
        }
        Ok(errors)
    }
}

impl crate::traits::Validate for OemSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()
    }

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        let mut errors = self.metadata.validation_errors()?;
        errors.extend(self.data.validation_errors()?);
        Ok(errors)
    }
}

impl crate::traits::Validate for OemMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_id.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.center_name.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "CENTER_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "REF_FRAME".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        for (field, epoch) in [
            ("START_TIME", &self.start_time),
            ("STOP_TIME", &self.stop_time),
        ] {
            if let Some(error) = contextual_epoch_error(epoch, field) {
                return Err(error.into());
            }
        }
        for (field, epoch) in [
            ("USEABLE_START_TIME", self.useable_start_time.as_ref()),
            ("USEABLE_STOP_TIME", self.useable_stop_time.as_ref()),
        ] {
            if let Some(epoch) = epoch {
                if let Some(error) = contextual_epoch_error(epoch, field) {
                    return Err(error.into());
                }
            }
        }
        if self.interpolation.is_some() && self.interpolation_degree.is_none() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Metadata".into(),
                field: "INTERPOLATION_DEGREE (required when INTERPOLATION is present)".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        let mut errors = crate::validation::missing_required_fields(
            "OEM Metadata",
            [
                ("OBJECT_NAME", self.object_name.trim().is_empty()),
                ("OBJECT_ID", self.object_id.trim().is_empty()),
                ("CENTER_NAME", self.center_name.trim().is_empty()),
                ("REF_FRAME", self.ref_frame.trim().is_empty()),
                ("TIME_SYSTEM", self.time_system.trim().is_empty()),
                (
                    "INTERPOLATION_DEGREE (required when INTERPOLATION is present)",
                    self.interpolation.is_some() && self.interpolation_degree.is_none(),
                ),
            ],
        );
        for (field, epoch) in [
            ("START_TIME", &self.start_time),
            ("STOP_TIME", &self.stop_time),
        ] {
            if let Some(error) = contextual_epoch_error(epoch, field) {
                errors.push(error);
            }
        }
        for (field, epoch) in [
            ("USEABLE_START_TIME", self.useable_start_time.as_ref()),
            ("USEABLE_STOP_TIME", self.useable_stop_time.as_ref()),
        ] {
            if let Some(epoch) = epoch {
                if let Some(error) = contextual_epoch_error(epoch, field) {
                    errors.push(error);
                }
            }
        }
        Ok(errors)
    }
}

impl crate::traits::Validate for OemData {
    fn validate(&self) -> Result<()> {
        if self.state_vector.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Data".into(),
                field: "stateVector (at least one required)".into(),
                line: None,
            }
            .into());
        }
        for state_vector in &self.state_vector {
            state_vector.validate()?;
        }
        for covariance in &self.covariance_matrix {
            covariance.validate()?;
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        let mut errors = crate::validation::missing_required_fields(
            "OEM Data",
            [(
                "stateVector (at least one required)",
                self.state_vector.is_empty(),
            )],
        );
        for state_vector in &self.state_vector {
            errors.extend(state_vector.validation_errors()?);
        }
        for covariance in &self.covariance_matrix {
            errors.extend(covariance.validation_errors()?);
        }
        Ok(errors)
    }
}

impl Ndm for Oem {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Oem,
            &self.version,
            crate::generation::OutputFormat::Kvn,
            self,
        )?;
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
        crate::traits::Validate::validate(&oem)?;
        Ok(oem)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Oem,
            &self.version,
            crate::generation::OutputFormat::Xml,
            self,
        )?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let oem: Self = crate::xml::from_str_with_context(xml, "OEM")?;
        crate::traits::Validate::validate(&oem)?;
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_epoch: Option<CalendarEpoch>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub interpolation: Option<String>,
    /// Recommended interpolation degree for ephemeris data in the immediately following set of
    /// ephemeris lines. Must be an integer value. This keyword must be used if the
    /// ‘INTERPOLATION’ keyword is used.
    ///
    /// **Examples**: 5, 8
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub interpolation_degree: Option<InterpolationDegree>,
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
        if !self.covariance_matrix.is_empty() {
            writer.write_empty();
            writer.write_section("COVARIANCE_START");

            // OEM comments are only permitted at the beginning of the covariance section.
            for cov in &self.covariance_matrix {
                writer.write_comments(&cov.comment);
            }

            for cov in &self.covariance_matrix {
                cov.write_kvn_matrix_lines(writer, false);
            }

            writer.write_section("COVARIANCE_STOP");
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
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
        self.write_kvn_matrix_lines(writer, true);
    }
}

impl crate::traits::Validate for OemCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if let Some(error) = contextual_epoch_error(&self.epoch, "EPOCH") {
            return Err(error.into());
        }
        for (field, value) in self.values() {
            if !value.is_finite() {
                return Err(crate::error::ValidationError::InvalidValue {
                    field: field.into(),
                    value: value.to_string(),
                    expected: "a finite number".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
        let mut errors = Vec::new();
        if let Some(error) = contextual_epoch_error(&self.epoch, "EPOCH") {
            errors.push(error);
        }
        errors.extend(self.values().into_iter().filter_map(|(field, value)| {
            (!value.is_finite()).then_some(crate::error::ValidationError::InvalidValue {
                field: field.into(),
                value: value.to_string(),
                expected: "a finite number".into(),
                line: None,
            })
        }));
        Ok(errors)
    }
}

impl OemCovarianceMatrix {
    fn values(&self) -> [(&'static str, f64); 21] {
        [
            ("CX_X", self.cx_x.value),
            ("CY_X", self.cy_x.value),
            ("CY_Y", self.cy_y.value),
            ("CZ_X", self.cz_x.value),
            ("CZ_Y", self.cz_y.value),
            ("CZ_Z", self.cz_z.value),
            ("CX_DOT_X", self.cx_dot_x.value),
            ("CX_DOT_Y", self.cx_dot_y.value),
            ("CX_DOT_Z", self.cx_dot_z.value),
            ("CX_DOT_X_DOT", self.cx_dot_x_dot.value),
            ("CY_DOT_X", self.cy_dot_x.value),
            ("CY_DOT_Y", self.cy_dot_y.value),
            ("CY_DOT_Z", self.cy_dot_z.value),
            ("CY_DOT_X_DOT", self.cy_dot_x_dot.value),
            ("CY_DOT_Y_DOT", self.cy_dot_y_dot.value),
            ("CZ_DOT_X", self.cz_dot_x.value),
            ("CZ_DOT_Y", self.cz_dot_y.value),
            ("CZ_DOT_Z", self.cz_dot_z.value),
            ("CZ_DOT_X_DOT", self.cz_dot_x_dot.value),
            ("CZ_DOT_Y_DOT", self.cz_dot_y_dot.value),
            ("CZ_DOT_Z_DOT", self.cz_dot_z_dot.value),
        ]
    }

    fn write_kvn_matrix_lines(&self, writer: &mut KvnWriter, write_comments: bool) {
        if write_comments {
            writer.write_comments(&self.comment);
        }
        writer.write_pair("EPOCH", self.epoch);
        if let Some(rf) = &self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", rf);
        }

        let mut b = zmij::Buffer::new();

        // Lower triangular formatting strict compliance (1, 2, 3, 4, 5, 6 items per line)
        writer.write_line(b.format(self.cx_x.value));

        let _ = writer.write_str(b.format(self.cy_x.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format(self.cy_y.value));

        let _ = writer.write_str(b.format(self.cz_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cz_y.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format(self.cz_z.value));

        let _ = writer.write_str(b.format(self.cx_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cx_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cx_dot_z.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format(self.cx_dot_x_dot.value));

        let _ = writer.write_str(b.format(self.cy_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cy_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cy_dot_z.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cy_dot_x_dot.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format(self.cy_dot_y_dot.value));

        let _ = writer.write_str(b.format(self.cz_dot_x.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cz_dot_y.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cz_dot_z.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cz_dot_x_dot.value));
        let _ = writer.write_str(" ");
        let _ = writer.write_str(b.format(self.cz_dot_y_dot.value));
        let _ = writer.write_str(" ");
        writer.write_line(b.format(self.cz_dot_z_dot.value));
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
    fn test_multiple_covariance_matrices_emit_single_covariance_block() {
        let kvn = include_str!("../../../data/kvn/oem_g13.kvn");
        let oem = Oem::from_kvn(kvn).expect("parse oem_g13");
        assert_eq!(oem.body.segment[0].data.covariance_matrix.len(), 2);

        let out = oem.to_kvn().expect("serialize oem_g13");
        assert_eq!(out.matches("COVARIANCE_START").count(), 1);
        assert_eq!(out.matches("COVARIANCE_STOP").count(), 1);
        assert_eq!(out.matches("EPOCH").count(), 2);
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

        meta.interpolation_degree = Some(InterpolationDegree::from(NonZeroU32::new(5).unwrap()));
        assert!(meta.validate().is_ok());
    }

    #[test]
    fn test_oem_data_empty_validation_internal() {
        let data = OemData::builder().build();
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_oem_body_requires_segment() {
        let body = OemBody { segment: vec![] };
        assert!(body.validate().is_err());
    }
}
