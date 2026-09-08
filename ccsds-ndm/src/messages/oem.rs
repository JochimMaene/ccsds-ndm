// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Ndm;
use crate::types::{
    CalendarEpoch, Epoch, EpochKind, InterpolationDegree, PositionCovariance,
    PositionVelocityCovariance, VelocityCovariance,
};
use serde::{Deserialize, Serialize};

mod kvn;
mod xml;

#[cfg(test)]
use std::num::NonZeroU32;

fn absolute_epoch_error(epoch: &Epoch, field: &'static str) -> Option<ValidationError> {
    (epoch.kind() != EpochKind::Calendar || epoch.calendar_fields_are_valid() != Some(true)).then(
        || ValidationError::InvalidValue {
            field: field.into(),
            value: epoch.to_string(),
            expected: "a valid CCSDS calendar or ordinal absolute time tag".into(),
            line: None,
        },
    )
}

fn validate_within_path(
    result: Result<()>,
    parent_path: impl FnOnce() -> std::borrow::Cow<'static, str>,
) -> Result<()> {
    match result {
        Err(CcsdsNdmError::Validation(error)) => Err((*error).within_path(parent_path()).into()),
        result => result,
    }
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
        crate::versioning::validate_oem_edition(self)?;
        self.header.validate()?;
        validate_within_path(self.body.validate(), || "body".into())
    }
}

impl crate::traits::Validate for OemBody {
    fn validate(&self) -> Result<()> {
        self.validate_identity()?;
        for (index, segment) in self.segment.iter().enumerate() {
            validate_within_path(segment.validate(), || format!("segment[{index}]").into())?;
        }
        self.validate_useable_spans()
    }
}

impl OemBody {
    fn validate_identity(&self) -> Result<()> {
        if self.segment.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Body".into(),
                field: "segment (at least one required)".into(),
                line: None,
            }
            .at_path("segment")
            .into());
        }
        if let Some(first) = self.segment.first() {
            let ts = &first.metadata.time_system;
            let object_name = &first.metadata.object_name;
            let object_id = &first.metadata.object_id;
            for (index, segment) in self.segment.iter().enumerate().skip(1) {
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
                    .at_path(format!("segment[{index}].metadata.time_system"))
                    .into());
                }
                if segment.metadata.object_name != *object_name
                    || segment.metadata.object_id != *object_id
                {
                    return Err(crate::error::ValidationError::InvalidValue {
                        field: "OBJECT_NAME/OBJECT_ID".into(),
                        value: format!(
                            "{}/{}",
                            segment.metadata.object_name, segment.metadata.object_id
                        ),
                        expected: format!(
                            "one object throughout the OEM (expected {object_name}/{object_id})"
                        )
                        .into(),
                        line: None,
                    }
                    .at_path(format!("segment[{index}].metadata.object_id"))
                    .into());
                }
            }
        }
        Ok(())
    }

    fn validate_useable_spans(&self) -> Result<()> {
        use std::cmp::Ordering;

        for (index, segments) in self.segment.windows(2).enumerate() {
            let (Some(previous_stop), Some(next_start)) = (
                &segments[0].metadata.useable_stop_time,
                &segments[1].metadata.useable_start_time,
            ) else {
                continue;
            };
            if previous_stop.cmp_same_branch(next_start) == Some(Ordering::Greater) {
                return Err(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME".into(),
                    value: next_start.to_string(),
                    expected: format!(
                        "not earlier than the preceding segment USEABLE_STOP_TIME {previous_stop}"
                    )
                    .into(),
                    line: None,
                }
                .at_path(format!(
                    "segment[{}].metadata.useable_start_time",
                    index + 1
                ))
                .into());
            }
        }
        Ok(())
    }
}

impl crate::traits::Validate for OemSegment {
    fn validate(&self) -> Result<()> {
        validate_within_path(self.metadata.validate(), || "metadata".into())?;
        validate_within_path(self.data.validate(), || "data".into())?;
        match self.first_epoch_range_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }
}

impl OemSegment {
    /// Return the first epoch ordering or range violation in this segment, if any.
    fn first_epoch_range_error(&self) -> Option<ValidationError> {
        let mut first = None;
        let mut report = |error| {
            first.get_or_insert(error);
        };
        let mut range = OemEpochRangeCheck::new(&self.metadata);
        for (index, state) in self.data.state_vector.iter().enumerate() {
            range.state(index, state, &mut report);
        }
        for (index, covariance) in self.data.covariance_matrix.iter().enumerate() {
            range.covariance(index, covariance, &mut report);
        }
        first
    }
}

struct OemEpochRangeCheck<'a> {
    start: &'a Epoch,
    stop: &'a Epoch,
    start_key: Option<crate::types::EpochOrderKey<'a>>,
    stop_key: Option<crate::types::EpochOrderKey<'a>>,
    previous_state: Option<crate::types::EpochOrderKey<'a>>,
    previous_covariance: Option<crate::types::EpochOrderKey<'a>>,
}

impl<'a> OemEpochRangeCheck<'a> {
    fn new(metadata: &'a OemMetadata) -> Self {
        Self {
            start: &metadata.start_time,
            stop: &metadata.stop_time,
            start_key: metadata.start_time.order_key(),
            stop_key: metadata.stop_time.order_key(),
            previous_state: None,
            previous_covariance: None,
        }
    }

    fn state(
        &mut self,
        index: usize,
        state: &'a StateVectorAcc,
        report: &mut impl FnMut(ValidationError),
    ) {
        use std::cmp::Ordering;

        let path = || format!("data.state_vector[{index}].epoch");
        if let Some(error) = absolute_epoch_error(&state.epoch, "stateVector EPOCH") {
            report(error.at_path(path()));
        }
        let current = state.epoch.order_key();
        let in_total_span = match (self.start_key, current, self.stop_key) {
            (Some(start), Some(current), Some(stop)) => {
                start.compare(&current) != Some(Ordering::Greater)
                    && current.compare(&stop) != Some(Ordering::Greater)
            }
            _ => true,
        };
        if !in_total_span {
            report(
                ValidationError::OutOfRange {
                    name: "stateVector EPOCH".into(),
                    value: state.epoch.to_string(),
                    expected: format!(
                        "within START_TIME {} and STOP_TIME {}",
                        self.start, self.stop
                    )
                    .into(),
                    line: None,
                }
                .at_path(path()),
            );
        }
        if matches!(
            (self.previous_state, current),
            (Some(prior), Some(current)) if prior.compare(&current) == Some(Ordering::Greater)
        ) {
            report(
                ValidationError::InvalidValue {
                    field: "stateVector EPOCH".into(),
                    value: state.epoch.to_string(),
                    expected: "nondecreasing ephemeris time tags".into(),
                    line: None,
                }
                .at_path(path()),
            );
        }
        self.previous_state = current;
    }

    fn covariance(
        &mut self,
        index: usize,
        covariance: &'a OemCovarianceMatrix,
        report: &mut impl FnMut(ValidationError),
    ) {
        use std::cmp::Ordering;

        let current = covariance.epoch.order_key();
        if matches!(
            (self.previous_covariance, current),
            (Some(prior), Some(current)) if prior.compare(&current) != Some(Ordering::Less)
        ) {
            report(
                ValidationError::InvalidValue {
                    field: "covarianceMatrix EPOCH".into(),
                    value: covariance.epoch.to_string(),
                    expected: "strictly increasing covariance time tags".into(),
                    line: None,
                }
                .at_path(format!("data.covariance_matrix[{index}].epoch")),
            );
        }
        self.previous_covariance = current;
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
            if let Some(error) = absolute_epoch_error(epoch, field) {
                return Err(error.into());
            }
        }
        for (field, epoch) in [
            ("USEABLE_START_TIME", self.useable_start_time.as_ref()),
            ("USEABLE_STOP_TIME", self.useable_stop_time.as_ref()),
        ] {
            if let Some(epoch) = epoch {
                if let Some(error) = absolute_epoch_error(epoch, field) {
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
        match self.first_time_span_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }
}

impl OemMetadata {
    /// Return the first metadata time-span violation, if any.
    fn first_time_span_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        let out_of_order = |earlier: &Epoch, later: &Epoch| {
            earlier.cmp_same_branch(later) == Some(Ordering::Greater)
        };
        let within_total_span = |epoch: &Epoch| {
            !out_of_order(&self.start_time, epoch) && !out_of_order(epoch, &self.stop_time)
        };

        if out_of_order(&self.start_time, &self.stop_time) {
            return Some(ValidationError::InvalidValue {
                field: "START_TIME/STOP_TIME".into(),
                value: format!("{} > {}", self.start_time, self.stop_time),
                expected: "START_TIME no later than STOP_TIME".into(),
                line: None,
            });
        }
        for (name, epoch) in [
            ("USEABLE_START_TIME", self.useable_start_time.as_ref()),
            ("USEABLE_STOP_TIME", self.useable_stop_time.as_ref()),
        ] {
            if let Some(epoch) = epoch.filter(|epoch| !within_total_span(epoch)) {
                return Some(ValidationError::OutOfRange {
                    name: name.into(),
                    value: epoch.to_string(),
                    expected: "within the total START_TIME/STOP_TIME span".into(),
                    line: None,
                });
            }
        }
        if let (Some(start), Some(stop)) = (&self.useable_start_time, &self.useable_stop_time) {
            if out_of_order(start, stop) {
                return Some(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME/USEABLE_STOP_TIME".into(),
                    value: format!("{start} > {stop}"),
                    expected: "USEABLE_START_TIME no later than USEABLE_STOP_TIME".into(),
                    line: None,
                });
            }
        }
        None
    }
}

impl crate::traits::Validate for OemData {
    fn validate(&self) -> Result<()> {
        self.validate_presence()?;
        for (index, state_vector) in self.state_vector.iter().enumerate() {
            validate_within_path(state_vector.validate(), || {
                format!("state_vector[{index}]").into()
            })?;
        }
        for (index, covariance) in self.covariance_matrix.iter().enumerate() {
            validate_within_path(covariance.validate(), || {
                format!("covariance_matrix[{index}]").into()
            })?;
        }
        Ok(())
    }
}

impl OemData {
    fn validate_presence(&self) -> Result<()> {
        if self.state_vector.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "OEM Data".into(),
                field: "stateVector (at least one required)".into(),
                line: None,
            }
            .at_path("state_vector")
            .into());
        }
        Ok(())
    }
}

impl Ndm for Oem {
    fn to_kvn(&self) -> Result<String> {
        kvn::to_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        Self::from_kvn_with_options(kvn, &crate::options::ParseOptions::default())
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        Self::from_xml_with_options(xml, &crate::options::ParseOptions::default())
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

fn validate_input_size(input: &str, options: &crate::options::ParseOptions) -> Result<()> {
    if let Some(limit) = options.max_input_bytes {
        if input.len() > limit {
            return Err(crate::error::CcsdsNdmError::ResourceLimitExceeded {
                resource: "input_document",
                limit,
                actual: input.len(),
            });
        }
    }
    Ok(())
}

//----------------------------------------------------------------------
// Body & Segments
//----------------------------------------------------------------------

/// The body of the OEM, containing one or more segments.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OemBody {
    #[serde(rename = "segment")]
    #[builder(default)]
    pub segment: Vec<OemSegment>,
}

/// A single segment of the OEM.
///
/// Each segment contains metadata (context) and a list of ephemeris data points.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OemSegment {
    pub metadata: OemMetadata,
    pub data: OemData,
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// OEM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
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

//----------------------------------------------------------------------
// Data Section
//----------------------------------------------------------------------

/// OEM Data Section.
///
/// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
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

//----------------------------------------------------------------------
// Covariance Matrix
//----------------------------------------------------------------------

/// OEM Covariance Matrix.
///
/// Represents a 6x6 symmetric covariance matrix for position and velocity at a specific epoch.
/// The lower triangular portion is stored/transmitted.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
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

impl crate::traits::Validate for OemCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if let Some(error) = absolute_epoch_error(&self.epoch, "EPOCH") {
            return Err(error.into());
        }
        for (field, value) in self.values() {
            if let Some(error) = crate::common::covariance_value_error(field, value) {
                return Err(error.into());
            }
        }
        Ok(())
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
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Ndm, Validate};

    #[test]
    fn covariance_validation_distinguishes_variances_from_off_diagonal_terms() {
        let negative_variance =
            include_str!("../../data/kvn/oem_g13.kvn").replace("3.3313494e-04", "-3.3313494e-04");
        let error = Oem::from_kvn(&negative_variance).expect_err("negative variance must fail");
        let message = error.to_string();
        assert!(message.contains("CX_X"));
        assert!(message.contains("a non-negative variance on the covariance diagonal"));

        let valid_off_diagonal = include_str!("../../data/kvn/oem_g13.kvn");
        assert!(valid_off_diagonal.contains("-3.0700078e-04"));
        Oem::from_kvn(valid_off_diagonal).expect("negative off-diagonal values are valid");
    }

    #[test]
    fn serializes_multiple_covariance_matrices_in_one_block() {
        let oem = Oem::from_kvn(include_str!("../../data/kvn/oem_g13.kvn")).unwrap();
        assert_eq!(oem.body.segment[0].data.covariance_matrix.len(), 2);

        let output = oem.to_kvn().unwrap();
        assert_eq!(output.matches("COVARIANCE_START").count(), 1);
        assert_eq!(output.matches("COVARIANCE_STOP").count(), 1);
        assert_eq!(output.matches("EPOCH").count(), 2);
    }

    #[test]
    fn validates_required_model_invariants() {
        let mut metadata = OemMetadata::builder()
            .object_name("SAT")
            .object_id("1")
            .center_name("EARTH")
            .ref_frame("GCRF")
            .time_system("UTC")
            .start_time(Epoch::new("2023-01-01T12:00:00").unwrap())
            .stop_time(Epoch::new("2023-01-01T13:00:00").unwrap())
            .build();

        metadata.interpolation = Some("LAGRANGE".into());
        assert!(metadata.validate().is_err());

        metadata.interpolation_degree =
            Some(InterpolationDegree::from(NonZeroU32::new(5).unwrap()));
        assert!(metadata.validate().is_ok());
        assert!(OemData::builder().build().validate().is_err());
        assert!(OemBody { segment: vec![] }.validate().is_err());
    }
}
