// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Ndm;
use crate::types::{
    Epoch, EpochKind, InterpolationDegree, PositionCovariance, PositionVelocityCovariance,
    VelocityCovariance,
};
use serde::{Deserialize, Serialize};

mod kvn;
mod xml;

// ODM 7.5.9: underscores and runs of blanks denote the same text value; 7.5.3 admits the
// all-uppercase and all-lowercase spellings of one value.
fn text_value_eq(left: &str, right: &str) -> bool {
    let words = |value| str::split(value, [' ', '_']).filter(|word: &&str| !word.is_empty());
    let mut right_words = words(right);
    words(left).all(|word| {
        right_words
            .next()
            .is_some_and(|other| word.eq_ignore_ascii_case(other))
    }) && right_words.next().is_none()
}

/// The value with XML whitespace around it removed. XML keeps a padded xsd:string as written
/// (8.13.5), and KVN drops the padding (7.4.7); where the books leave its meaning open, a
/// padded value is read as the value itself while the model keeps the original text.
fn xml_trimmed(value: &str) -> &str {
    value.trim_matches([' ', '\t', '\r', '\n'])
}

/// Whether `TIME_SYSTEM` denotes elapsed time rather than an absolute time scale.
fn is_elapsed_time_system(time_system: &str) -> bool {
    ["MET", "MRT"]
        .iter()
        .any(|elapsed| xml_trimmed(time_system).eq_ignore_ascii_case(elapsed))
}

/// ODM 7.5.10 time tags: calendar or ordinal layout with at most 16 fractional digits, the
/// fixed-point maximum. MET and MRT durations use three-digit days (3.2.3.2).
fn epoch_error(epoch: &Epoch, field: &'static str, time_system: &str) -> Option<ValidationError> {
    let valid_layout = epoch.kind() == EpochKind::Calendar
        && (epoch.calendar_fields_are_valid() == Some(true)
            || is_elapsed_time_system(time_system) && epoch.is_elapsed_time());
    (!valid_layout || epoch.fraction_digits() > 16).then(|| ValidationError::InvalidValue {
        field: field.into(),
        value: epoch.to_string(),
        expected: "a CCSDS calendar or ordinal time tag with at most 16 fractional digits".into(),
        line: None,
    })
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
        let Some(first) = self.segment.first() else {
            return Err(ValidationError::MissingRequiredField {
                block: "OEM Body".into(),
                field: "segment (at least one required)".into(),
                line: None,
            }
            .at_path("segment")
            .into());
        };
        let metadata = &first.metadata;
        for (index, segment) in self.segment.iter().enumerate().skip(1) {
            // ODM 5.2.4.5: one time system throughout, in either permitted case (7.5.3) and
            // regardless of XML padding.
            if !xml_trimmed(&segment.metadata.time_system)
                .eq_ignore_ascii_case(xml_trimmed(&metadata.time_system))
            {
                return Err(ValidationError::InvalidValue {
                    field: "TIME_SYSTEM".into(),
                    value: segment.metadata.time_system.clone(),
                    expected: format!(
                        "consistent TIME_SYSTEM across OEM segments (expected {})",
                        metadata.time_system
                    )
                    .into(),
                    line: None,
                }
                .at_path(format!("segment[{index}].metadata.time_system"))
                .into());
            }
            // ODM 5.1.3: one object throughout.
            if !text_value_eq(&segment.metadata.object_name, &metadata.object_name)
                || !text_value_eq(&segment.metadata.object_id, &metadata.object_id)
            {
                return Err(ValidationError::InvalidValue {
                    field: "OBJECT_NAME/OBJECT_ID".into(),
                    value: format!(
                        "{}/{}",
                        segment.metadata.object_name, segment.metadata.object_id
                    ),
                    expected: format!(
                        "one object throughout the OEM (expected {}/{})",
                        metadata.object_name, metadata.object_id
                    )
                    .into(),
                    line: None,
                }
                .at_path(format!("segment[{index}].metadata.object_id"))
                .into());
            }
        }
        Ok(())
    }

    /// ODM 5.2.4.4: the useable spans of consecutive segments must not overlap, except at a
    /// shared endpoint. Segments need not be in time order. A span is known only when both of
    /// its USEABLE bounds are given, so a pair is checked only when both spans are complete;
    /// total spans may overlap.
    fn validate_useable_spans(&self) -> Result<()> {
        use std::cmp::Ordering;

        fn span(segment: &OemSegment) -> Option<(&Epoch, &Epoch)> {
            let metadata = &segment.metadata;
            metadata
                .useable_start_time
                .as_ref()
                .zip(metadata.useable_stop_time.as_ref())
        }
        let before =
            |left: &Epoch, right: &Epoch| left.cmp_same_branch(right) == Some(Ordering::Less);
        for (index, segments) in self.segment.windows(2).enumerate() {
            let (Some((previous_start, previous_stop)), Some((next_start, next_stop))) =
                (span(&segments[0]), span(&segments[1]))
            else {
                continue;
            };
            if before(next_start, previous_stop) && before(previous_start, next_stop) {
                return Err(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME".into(),
                    value: next_start.to_string(),
                    expected: format!(
                        "a useable span not overlapping the preceding segment's \
                         {previous_start} to {previous_stop}"
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

    /// 502.0-B-2 Cor. 1, table 5-3: a segment's USEABLE_START_TIME must be at or after the
    /// previous segment's USEABLE_STOP_TIME. The rule names only those two values, so it applies
    /// whenever both are given. 3.0 dropped it.
    pub(crate) fn validate_odm_2_useable_order(&self) -> Result<()> {
        for (index, segments) in self.segment.windows(2).enumerate() {
            let (Some(previous_stop), Some(next_start)) = (
                segments[0].metadata.useable_stop_time.as_ref(),
                segments[1].metadata.useable_start_time.as_ref(),
            ) else {
                continue;
            };
            if next_start.cmp_same_branch(previous_stop) == Some(std::cmp::Ordering::Less) {
                return Err(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME".into(),
                    value: next_start.to_string(),
                    expected: format!(
                        "a time at or after the preceding segment's USEABLE_STOP_TIME \
                         {previous_stop} (CCSDS 502.0-B-2)"
                    )
                    .into(),
                    line: None,
                }
                .at_path(format!(
                    "body.segment[{}].metadata.useable_start_time",
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
        match self.first_epoch_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }
}

impl OemSegment {
    /// Return the first invalid or out-of-span state epoch, or invalid or out-of-order covariance
    /// epoch.
    fn first_epoch_error(&self) -> Option<ValidationError> {
        let range = OemEpochRange::new(&self.metadata);
        let states = self
            .data
            .state_vector
            .iter()
            .enumerate()
            .find_map(|(index, state)| {
                range
                    .error(&state.epoch, "stateVector EPOCH")
                    .map(|error| error.at_path(format!("data.state_vector[{index}].epoch")))
            });
        states.or_else(|| {
            let mut previous = None;
            self.data
                .covariance_matrix
                .iter()
                .enumerate()
                .find_map(|(index, covariance)| {
                    range
                        .covariance_error(&covariance.epoch, &mut previous)
                        .map(|error| {
                            error.at_path(format!("data.covariance_matrix[{index}].epoch"))
                        })
                })
        })
    }
}

/// The total time span of one segment, against which its data epochs are checked.
struct OemEpochRange<'a> {
    metadata: &'a OemMetadata,
    start: Option<crate::types::EpochOrderKey<'a>>,
    stop: Option<crate::types::EpochOrderKey<'a>>,
}

impl<'a> OemEpochRange<'a> {
    fn new(metadata: &'a OemMetadata) -> Self {
        Self {
            metadata,
            start: metadata.start_time.order_key(),
            stop: metadata.stop_time.order_key(),
        }
    }

    /// Table 5-3: START_TIME and STOP_TIME bound the ephemeris data.
    fn error(&self, epoch: &Epoch, field: &'static str) -> Option<ValidationError> {
        use std::cmp::Ordering;

        if let Some(error) = epoch_error(epoch, field, &self.metadata.time_system) {
            return Some(error);
        }
        let outside = match (self.start, epoch.order_key(), self.stop) {
            (Some(start), Some(current), Some(stop)) => {
                start.compare(&current) == Some(Ordering::Greater)
                    || current.compare(&stop) == Some(Ordering::Greater)
            }
            _ => false,
        };
        outside.then(|| ValidationError::OutOfRange {
            name: field.into(),
            value: epoch.to_string(),
            expected: format!(
                "within START_TIME {} and STOP_TIME {}",
                self.metadata.start_time, self.metadata.stop_time
            )
            .into(),
            line: None,
        })
    }

    /// ODM 5.2.5.7: multiple covariance matrices are ordered by increasing time tag. Equal
    /// tags are not excluded.
    ///
    /// Table 5-3 says the total span also covers the covariance data, but the book's own
    /// example G-14 has a covariance epoch after STOP_TIME. With the book in conflict, the
    /// covariance epochs are not bounded by the span.
    fn covariance_error(
        &self,
        epoch: &'a Epoch,
        previous: &mut Option<crate::types::EpochOrderKey<'a>>,
    ) -> Option<ValidationError> {
        use std::cmp::Ordering;

        if let Some(error) =
            epoch_error(epoch, "covarianceMatrix EPOCH", &self.metadata.time_system)
        {
            return Some(error);
        }
        let current = epoch.order_key();
        let decreasing = matches!(
            (*previous, current),
            (Some(prior), Some(current)) if prior.compare(&current) == Some(Ordering::Greater)
        );
        *previous = current;
        decreasing.then(|| ValidationError::InvalidValue {
            field: "covarianceMatrix EPOCH".into(),
            value: epoch.to_string(),
            expected: "covariance time tags in increasing order".into(),
            line: None,
        })
    }
}

impl crate::traits::Validate for OemMetadata {
    fn validate(&self) -> Result<()> {
        for (field, member, value) in [
            ("OBJECT_NAME", "object_name", &self.object_name),
            ("OBJECT_ID", "object_id", &self.object_id),
            ("CENTER_NAME", "center_name", &self.center_name),
            ("REF_FRAME", "ref_frame", &self.ref_frame),
            ("TIME_SYSTEM", "time_system", &self.time_system),
        ] {
            if value.trim().is_empty() {
                return Err(ValidationError::missing_required("OEM Metadata", field)
                    .at_path(member)
                    .into());
            }
        }
        for (field, member, epoch) in [
            ("START_TIME", "start_time", Some(&self.start_time)),
            ("STOP_TIME", "stop_time", Some(&self.stop_time)),
            (
                "USEABLE_START_TIME",
                "useable_start_time",
                self.useable_start_time.as_ref(),
            ),
            (
                "USEABLE_STOP_TIME",
                "useable_stop_time",
                self.useable_stop_time.as_ref(),
            ),
            (
                "REF_FRAME_EPOCH",
                "ref_frame_epoch",
                self.ref_frame_epoch.as_ref(),
            ),
        ] {
            if let Some(error) =
                epoch.and_then(|epoch| epoch_error(epoch, field, &self.time_system))
            {
                return Err(error.at_path(member).into());
            }
        }
        if self.interpolation.is_some() && self.interpolation_degree.is_none() {
            return Err(ValidationError::missing_required(
                "OEM Metadata",
                "INTERPOLATION_DEGREE (required when INTERPOLATION is present)",
            )
            .at_path("interpolation_degree")
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
            return Some(
                ValidationError::InvalidValue {
                    field: "START_TIME/STOP_TIME".into(),
                    value: format!("{} > {}", self.start_time, self.stop_time),
                    expected: "START_TIME no later than STOP_TIME".into(),
                    line: None,
                }
                .at_path("start_time"),
            );
        }
        for (name, member, epoch) in [
            (
                "USEABLE_START_TIME",
                "useable_start_time",
                self.useable_start_time.as_ref(),
            ),
            (
                "USEABLE_STOP_TIME",
                "useable_stop_time",
                self.useable_stop_time.as_ref(),
            ),
        ] {
            if let Some(epoch) = epoch.filter(|epoch| !within_total_span(epoch)) {
                return Some(
                    ValidationError::OutOfRange {
                        name: name.into(),
                        value: epoch.to_string(),
                        expected: "within the total START_TIME/STOP_TIME span".into(),
                        line: None,
                    }
                    .at_path(member),
                );
            }
        }
        if let (Some(start), Some(stop)) = (&self.useable_start_time, &self.useable_stop_time) {
            if out_of_order(start, stop) {
                return Some(
                    ValidationError::InvalidValue {
                        field: "USEABLE_START_TIME/USEABLE_STOP_TIME".into(),
                        value: format!("{start} > {stop}"),
                        expected: "USEABLE_START_TIME no later than USEABLE_STOP_TIME".into(),
                        line: None,
                    }
                    .at_path("useable_start_time"),
                );
            }
        }
        None
    }
}

impl crate::traits::Validate for OemData {
    fn validate(&self) -> Result<()> {
        // Record epochs depend on the segment TIME_SYSTEM, XML values follow xsd:double
        // (ODM 8.13.4), and frame spelling is a KVN rule, so the segment and the KVN
        // boundary check the records.
        self.validate_presence()
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
        Self::from_kvn_strict(kvn)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        Self::from_xml_strict(xml)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
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
    /// (See 7.5.10 for formatting rules.) Like the other OEM epochs, it is interpreted in
    /// TIME_SYSTEM (7.5.11), so MET and MRT values are durations (3.2.3.2).
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_ref_frame: Option<String>,

    /// Covariance matrix `[1,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cx_x: PositionCovariance,
    /// Covariance matrix `[2,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_x: PositionCovariance,
    /// Covariance matrix `[2,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_y: PositionCovariance,
    /// Covariance matrix `[3,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_x: PositionCovariance,
    /// Covariance matrix `[3,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_y: PositionCovariance,
    /// Covariance matrix `[3,3]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_z: PositionCovariance,

    /// Covariance matrix `[4,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cx_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[4,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cx_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[4,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cx_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[4,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cx_dot_x_dot: VelocityCovariance,

    /// Covariance matrix `[5,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[5,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[5,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[5,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[5,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cy_dot_y_dot: VelocityCovariance,

    /// Covariance matrix `[6,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[6,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[6,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[6,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[6,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_y_dot: VelocityCovariance,
    /// Covariance matrix `[6,6]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.5.
    #[serde(serialize_with = "crate::types::serialize_without_units")]
    pub cz_dot_z_dot: VelocityCovariance,
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
