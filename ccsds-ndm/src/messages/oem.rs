// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::{KvnWriter, OdmFloat};
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::{
    AccUnits, CalendarEpoch, Epoch, EpochKind, InterpolationDegree, PositionCovariance,
    PositionCovarianceUnits, PositionUnits, PositionVelocityCovariance,
    PositionVelocityCovarianceUnits, VelocityCovariance, VelocityCovarianceUnits, VelocityUnits,
};
use serde::{Deserialize, Serialize};

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

/// Report a value with no CCSDS spelling that reads back as a finite number.
fn unrepresentable_number(field: &str, value: f64, path: String) -> CcsdsNdmError {
    ValidationError::InvalidValue {
        field: field.to_owned().into(),
        // Spell the offender in scientific form; `f64::to_string` expands large magnitudes to
        // hundreds of digits, which buries the diagnostic.
        value: format!("{value:e}"),
        expected: "a representable CCSDS number".into(),
        line: None,
    }
    .at_path(path)
    .into()
}

/// Reject a record wider than the normative KVN line limit.
fn line_length_error(
    name: &'static str,
    line_len: usize,
    path: impl FnOnce() -> String,
) -> Result<()> {
    if line_len <= 254 {
        return Ok(());
    }
    Err(ValidationError::OutOfRange {
        name: name.into(),
        value: line_len.to_string(),
        expected: "a KVN line no longer than 254 characters".into(),
        line: None,
    }
    .at_path(path())
    .into())
}

/// Split the 21 lower-triangular values into the normative 1/2/3/4/5/6-element rows.
fn covariance_rows<T>(values: &[T; 21]) -> [&[T]; 6] {
    [
        &values[0..1],
        &values[1..3],
        &values[3..6],
        &values[6..10],
        &values[10..15],
        &values[15..21],
    ]
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
        (|| {
            crate::generation::validate_output_version(
                crate::validation::MessageKind::Oem,
                &self.version,
                crate::generation::OutputFormat::Kvn,
            )?;
            // Estimate capacity: header + records, without a second output-sized allocation.
            let records = self.body.segment.iter().fold(0usize, |total, segment| {
                total
                    .saturating_add(segment.data.state_vector.len())
                    .saturating_add(segment.data.covariance_matrix.len().saturating_mul(7))
            });
            let estimated_capacity = records.saturating_mul(150).saturating_add(4096);
            let mut writer = KvnWriter::with_capacity(estimated_capacity);
            self.write_validated_kvn(&mut writer)?;
            writer.finish_checked()
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_generation_context(
                crate::validation::MessageKind::Oem,
                crate::error::DiagnosticNotation::Kvn,
                &self.version,
            )
        })
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
}

impl Oem {
    /// Run the generation pass against a discarding sink.
    ///
    /// `write_validated_kvn` is the single description of the OEM KVN layout, so validating
    /// through it keeps the checks and the emitted bytes from drifting apart. Streaming callers
    /// preflight here first so a rejected message never reaches the caller's sink half-written.
    pub(crate) fn validate_kvn_generation(&self) -> Result<()> {
        let mut sink = std::io::sink();
        let mut writer = KvnWriter::from_io(&mut sink);
        self.write_validated_kvn(&mut writer)?;
        writer.finish_io()
    }

    /// Write the complete OEM KVN document, validating each record as it is emitted.
    fn write_validated_kvn(&self, writer: &mut KvnWriter<'_>) -> Result<()> {
        fn text(field: &'static str, value: &str, path: String) -> Result<()> {
            if !value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                return Err(ValidationError::InvalidValue {
                    field: field.into(),
                    value: value.into(),
                    expected: "printable ASCII characters and blanks".into(),
                    line: None,
                }
                .at_path(path)
                .into());
            }
            // `KvnWriter::write_pair` left-pads the key to 20 columns and adds " = ".
            let line_len = field.len().max(20) + 3 + value.len();
            if line_len > 254 {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: line_len.to_string(),
                    expected: "a KVN line no longer than 254 characters".into(),
                    line: None,
                }
                .at_path(path)
                .into());
            }
            Ok(())
        }
        fn comments(values: &[String], path: String) -> Result<()> {
            for value in values {
                if let Some(error) = crate::validation::kvn_comment_error(value) {
                    return Err(error.at_path(path.clone()).into());
                }
            }
            Ok(())
        }
        crate::versioning::validate_root(
            crate::validation::MessageKind::Oem,
            &self.id,
            &self.version,
        )?;
        crate::versioning::validate_oem_edition(self)?;
        self.header.validate()?;
        validate_within_path(self.body.validate_identity(), || "body".into())?;
        validate_within_path(self.body.validate_useable_spans(), || "body".into())?;

        comments(&self.header.comment, "header.comment".into())?;
        if let Some(value) = &self.header.classification {
            text("CLASSIFICATION", value, "header.classification".into())?;
        }
        text(
            "ORIGINATOR",
            &self.header.originator,
            "header.originator".into(),
        )?;
        if let Some(value) = &self.header.message_id {
            text("MESSAGE_ID", value, "header.message_id".into())?;
        }
        writer.write_pair("CCSDS_OEM_VERS", &self.version);
        self.header.write_kvn(writer);
        for (segment_index, segment) in self.body.segment.iter().enumerate() {
            let base = format!("body.segment[{segment_index}]");
            let metadata = &segment.metadata;
            validate_within_path(metadata.validate(), || format!("{base}.metadata").into())?;
            validate_within_path(segment.data.validate_presence(), || {
                format!("{base}.data").into()
            })?;
            let mut epoch_range = OemEpochRangeCheck::new(metadata);
            comments(&metadata.comment, format!("{base}.metadata.comment"))?;
            for (field, value, member) in [
                ("OBJECT_NAME", metadata.object_name.as_str(), "object_name"),
                ("OBJECT_ID", metadata.object_id.as_str(), "object_id"),
                ("CENTER_NAME", metadata.center_name.as_str(), "center_name"),
                ("REF_FRAME", metadata.ref_frame.as_str(), "ref_frame"),
                ("TIME_SYSTEM", metadata.time_system.as_str(), "time_system"),
            ] {
                text(field, value, format!("{base}.metadata.{member}"))?;
            }
            if let Some(value) = &metadata.interpolation {
                text(
                    "INTERPOLATION",
                    value,
                    format!("{base}.metadata.interpolation"),
                )?;
            }
            comments(&segment.data.comment, format!("{base}.data.comment"))?;
            writer.write_section("META_START");
            metadata.write_kvn(writer);
            writer.write_section("META_STOP");
            writer.write_comments(&segment.data.comment);
            writer.write_empty();
            for (state_index, state) in segment.data.state_vector.iter().enumerate() {
                // The fused generation pass below applies the stronger OEM absolute/range/order
                // epoch checks and checks every numeric component through `OdmFloat`. Calling the
                // generic state validator here would scan the same epoch and values a second time.
                let mut epoch_error = None;
                epoch_range.state(state_index, state, &mut |error| {
                    epoch_error.get_or_insert(error);
                });
                if let Some(error) = epoch_error {
                    return Err(error.within_path(base.clone()).into());
                }
                let acceleration_count = [&state.x_ddot, &state.y_ddot, &state.z_ddot]
                    .into_iter()
                    .filter(|value| value.is_some())
                    .count();
                if acceleration_count != 0 && acceleration_count != 3 {
                    return Err(ValidationError::InvalidValue {
                        field: "X_DDOT/Y_DDOT/Z_DDOT".into(),
                        value: format!("{acceleration_count} acceleration components present"),
                        expected: "either no acceleration components or all three for OEM KVN"
                            .into(),
                        line: None,
                    }
                    .at_path(format!("{base}.data.state_vector[{state_index}]"))
                    .into());
                }
                let required_values = [
                    ("X", state.x.value, "x"),
                    ("Y", state.y.value, "y"),
                    ("Z", state.z.value, "z"),
                    ("X_DOT", state.x_dot.value, "x_dot"),
                    ("Y_DOT", state.y_dot.value, "y_dot"),
                    ("Z_DOT", state.z_dot.value, "z_dot"),
                ];
                let acceleration_values = [
                    ("X_DDOT", &state.x_ddot, "x_ddot"),
                    ("Y_DDOT", &state.y_ddot, "y_ddot"),
                    ("Z_DDOT", &state.z_ddot, "z_ddot"),
                ];
                writer.try_write_validated_line(|line, line_start| -> Result<()> {
                    line.push_str(state.epoch.as_str());
                    for (field, value, member) in required_values {
                        line.push(' ');
                        if !OdmFloat::write_if_valid(value, line) {
                            return Err(unrepresentable_number(
                                field,
                                value,
                                format!("{base}.data.state_vector[{state_index}].{member}"),
                            ));
                        }
                    }
                    for (field, value, member) in
                        acceleration_values
                            .into_iter()
                            .flat_map(|(field, value, member)| {
                                value.as_ref().map(|value| (field, value, member))
                            })
                    {
                        line.push(' ');
                        if !OdmFloat::write_if_valid(value.value, line) {
                            return Err(unrepresentable_number(
                                field,
                                value.value,
                                format!("{base}.data.state_vector[{state_index}].{member}"),
                            ));
                        }
                    }
                    line_length_error("stateVector", line.len() - line_start, || {
                        format!("{base}.data.state_vector[{state_index}]")
                    })
                })?;
            }
            if !segment.data.covariance_matrix.is_empty() {
                writer.write_empty();
                writer.write_section("COVARIANCE_START");
                for (covariance_index, covariance) in
                    segment.data.covariance_matrix.iter().enumerate()
                {
                    if covariance_index > 0 && !covariance.comment.is_empty() {
                        return Err(ValidationError::InvalidValue {
                            field: "COMMENT".into(),
                            value: covariance.comment.join(" | "),
                            expected:
                                "comments attached only to the first covariance matrix for OEM KVN"
                                    .into(),
                            line: None,
                        }
                        .at_path(format!(
                            "{base}.data.covariance_matrix[{covariance_index}].comment"
                        ))
                        .into());
                    }
                    comments(
                        &covariance.comment,
                        format!("{base}.data.covariance_matrix[{covariance_index}].comment"),
                    )?;
                    writer.write_comments(&covariance.comment);
                }
            }
            for (covariance_index, covariance) in segment.data.covariance_matrix.iter().enumerate()
            {
                // Numeric finiteness is subsumed by the representability checks used for every
                // emitted matrix value. Keep the non-numeric absolute-epoch rule explicitly.
                if let Some(error) = absolute_epoch_error(&covariance.epoch, "EPOCH") {
                    return Err(error
                        .at_path(format!(
                            "{base}.data.covariance_matrix[{covariance_index}].epoch"
                        ))
                        .into());
                }
                let mut epoch_error = None;
                epoch_range.covariance(covariance_index, covariance, &mut |error| {
                    epoch_error.get_or_insert(error);
                });
                if let Some(error) = epoch_error {
                    return Err(error.within_path(base.clone()).into());
                }
                if let Some(value) = &covariance.cov_ref_frame {
                    text(
                        "COV_REF_FRAME",
                        value,
                        format!("{base}.data.covariance_matrix[{covariance_index}].cov_ref_frame"),
                    )?;
                }
                writer.write_pair("EPOCH", covariance.epoch);
                if let Some(value) = &covariance.cov_ref_frame {
                    writer.write_pair("COV_REF_FRAME", value);
                }
                let values = covariance.values();
                for (row_index, row) in covariance_rows(&values).into_iter().enumerate() {
                    writer.try_write_validated_line(|line, line_start| -> Result<()> {
                        for (value_index, (field, value)) in row.iter().enumerate() {
                            if value_index > 0 {
                                line.push(' ');
                            }
                            let path = || {
                                format!(
                                    "{base}.data.covariance_matrix[{covariance_index}].{}",
                                    field.to_ascii_lowercase()
                                )
                            };
                            if let Some(error) =
                                crate::common::covariance_value_error(field, *value)
                            {
                                return Err(error.at_path(path()).into());
                            }
                            if !OdmFloat::write_if_valid(*value, line) {
                                return Err(unrepresentable_number(field, *value, path()));
                            }
                        }
                        line_length_error("covariance row", line.len() - line_start, || {
                            format!(
                                "{base}.data.covariance_matrix[{covariance_index}].row[{}]",
                                row_index + 1
                            )
                        })
                    })?;
                }
            }
            if !segment.data.covariance_matrix.is_empty() {
                writer.write_section("COVARIANCE_STOP");
            }
        }
        Ok(())
    }

    pub(crate) fn validate_xml_text(&self) -> Result<()> {
        fn check(field: &'static str, value: &str, path: String) -> Result<()> {
            match crate::validation::xml_text_error(field, value) {
                Some(error) => Err(error.at_path(path).into()),
                None => Ok(()),
            }
        }
        fn comments(values: &[String], path: String) -> Result<()> {
            for value in values {
                check("COMMENT", value, path.clone())?;
            }
            Ok(())
        }

        comments(&self.header.comment, "header.comment".into())?;
        if let Some(value) = &self.header.classification {
            check("CLASSIFICATION", value, "header.classification".into())?;
        }
        check(
            "ORIGINATOR",
            &self.header.originator,
            "header.originator".into(),
        )?;
        if let Some(value) = &self.header.message_id {
            check("MESSAGE_ID", value, "header.message_id".into())?;
        }
        for (index, segment) in self.body.segment.iter().enumerate() {
            let base = format!("body.segment[{index}]");
            let metadata = &segment.metadata;
            comments(&metadata.comment, format!("{base}.metadata.comment"))?;
            for (field, value, member) in [
                ("OBJECT_NAME", metadata.object_name.as_str(), "object_name"),
                ("OBJECT_ID", metadata.object_id.as_str(), "object_id"),
                ("CENTER_NAME", metadata.center_name.as_str(), "center_name"),
                ("REF_FRAME", metadata.ref_frame.as_str(), "ref_frame"),
                ("TIME_SYSTEM", metadata.time_system.as_str(), "time_system"),
            ] {
                check(field, value, format!("{base}.metadata.{member}"))?;
            }
            if let Some(value) = &metadata.interpolation {
                check(
                    "INTERPOLATION",
                    value,
                    format!("{base}.metadata.interpolation"),
                )?;
            }
            comments(&segment.data.comment, format!("{base}.data.comment"))?;
            for (covariance_index, covariance) in segment.data.covariance_matrix.iter().enumerate()
            {
                let cov_base = format!("{base}.data.covariance_matrix[{covariance_index}]");
                comments(&covariance.comment, format!("{cov_base}.comment"))?;
                if let Some(value) = &covariance.cov_ref_frame {
                    check("COV_REF_FRAME", value, format!("{cov_base}.cov_ref_frame"))?;
                }
            }
        }
        Ok(())
    }

    /// Strictly parse and validate an OEM KVN document with caller resource limits.
    pub fn from_kvn_with_options(
        kvn: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        let source_edition = kvn.split(['\r', '\n']).find_map(|line| {
            line.split_once('=')
                .filter(|(key, _)| key.trim() == "CCSDS_OEM_VERS")
                .map(|(_, value)| value.trim())
        });
        (|| {
            validate_input_size(kvn, options)?;
            let normalized = crate::kvn::normalize_line_endings(kvn);
            validate_oem_kvn_syntax(&normalized, options)?;
            let oem = Self::from_kvn_str(&normalized)?;
            crate::traits::Validate::validate(&oem)?;
            Ok(oem)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Oem,
                crate::error::DiagnosticNotation::Kvn,
                kvn,
                source_edition,
            )
        })
    }

    /// Strictly parse and validate an OEM XML document with caller resource limits.
    pub fn from_xml_with_options(
        xml: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        let mut source_edition = None;
        (|| {
            validate_input_size(xml, options)?;
            validate_oem_xml_envelope(xml, options, &mut source_edition)?;
            let mut oem: Self = crate::xml::from_str_with_context(xml, "OEM")?;
            oem.normalize_implicit_units();
            crate::traits::Validate::validate(&oem)?;
            Ok(oem)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Oem,
                crate::error::DiagnosticNotation::Xml,
                xml,
                source_edition.as_deref(),
            )
        })
    }

    fn normalize_implicit_units(&mut self) {
        for segment in &mut self.body.segment {
            for state in &mut segment.data.state_vector {
                state.x.units.get_or_insert(PositionUnits::Km);
                state.y.units.get_or_insert(PositionUnits::Km);
                state.z.units.get_or_insert(PositionUnits::Km);
                state.x_dot.units.get_or_insert(VelocityUnits::KmPerS);
                state.y_dot.units.get_or_insert(VelocityUnits::KmPerS);
                state.z_dot.units.get_or_insert(VelocityUnits::KmPerS);
                for acceleration in [&mut state.x_ddot, &mut state.y_ddot, &mut state.z_ddot]
                    .into_iter()
                    .flatten()
                {
                    acceleration.units.get_or_insert(AccUnits::KmPerS2);
                }
            }
            for covariance in &mut segment.data.covariance_matrix {
                for value in [
                    &mut covariance.cx_x,
                    &mut covariance.cy_x,
                    &mut covariance.cy_y,
                    &mut covariance.cz_x,
                    &mut covariance.cz_y,
                    &mut covariance.cz_z,
                ] {
                    value.units.get_or_insert(PositionCovarianceUnits::Km2);
                }
                for value in [
                    &mut covariance.cx_dot_x,
                    &mut covariance.cx_dot_y,
                    &mut covariance.cx_dot_z,
                    &mut covariance.cy_dot_x,
                    &mut covariance.cy_dot_y,
                    &mut covariance.cy_dot_z,
                    &mut covariance.cz_dot_x,
                    &mut covariance.cz_dot_y,
                    &mut covariance.cz_dot_z,
                ] {
                    value
                        .units
                        .get_or_insert(PositionVelocityCovarianceUnits::Km2PerS);
                }
                for value in [
                    &mut covariance.cx_dot_x_dot,
                    &mut covariance.cy_dot_x_dot,
                    &mut covariance.cy_dot_y_dot,
                    &mut covariance.cz_dot_x_dot,
                    &mut covariance.cz_dot_y_dot,
                    &mut covariance.cz_dot_z_dot,
                ] {
                    value.units.get_or_insert(VelocityCovarianceUnits::Km2PerS2);
                }
            }
        }
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

pub(crate) fn valid_odm_number(token: &str) -> bool {
    let bytes = token.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    if index == bytes.len() {
        return false;
    }

    let integer_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    let integer_digits = index - integer_start;
    if integer_digits == 0 {
        return false;
    }

    let mut fraction_digits = 0;
    let has_decimal = bytes.get(index) == Some(&b'.');
    if has_decimal {
        index += 1;
        let fraction_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        fraction_digits = index - fraction_start;
        if fraction_digits == 0 {
            return false;
        }
    }

    let significant_digits = integer_digits + fraction_digits;
    if significant_digits > 16 {
        return false;
    }
    if index == bytes.len() {
        return has_decimal || token.parse::<i32>().is_ok();
    }
    if !has_decimal || integer_digits != 1 || !matches!(bytes.get(index), Some(b'e' | b'E')) {
        return false;
    }

    index += 1;
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }
    let exponent_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    index == bytes.len() && index > exponent_start
}

/// Validate the OEM KVN record structure.
///
/// Expects `kvn` to have gone through [`crate::kvn::normalize_line_endings`], so every remaining
/// carriage return is part of a CRLF pair.
fn validate_oem_kvn_syntax(kvn: &str, options: &crate::options::ParseOptions) -> Result<()> {
    use crate::error::{CcsdsNdmError, FormatError};

    fn invalid(line: usize, offset: usize, message: impl AsRef<str>) -> CcsdsNdmError {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(
            crate::error::KvnParseError {
                line,
                column: 1,
                message: message.as_ref().to_owned(),
                contexts: vec!["strict OEM KVN"],
                offset,
            },
        ))))
    }

    fn header_rank(key: &str) -> Option<u8> {
        Some(match key {
            "CCSDS_OEM_VERS" => 0,
            "CLASSIFICATION" => 1,
            "CREATION_DATE" => 2,
            "ORIGINATOR" => 3,
            "MESSAGE_ID" => 4,
            _ => return None,
        })
    }

    fn metadata_rank(key: &str) -> Option<u8> {
        Some(match key {
            "OBJECT_NAME" => 1,
            "OBJECT_ID" => 2,
            "CENTER_NAME" => 3,
            "REF_FRAME" => 4,
            "REF_FRAME_EPOCH" => 5,
            "TIME_SYSTEM" => 6,
            "START_TIME" => 7,
            "USEABLE_START_TIME" => 8,
            "USEABLE_STOP_TIME" => 9,
            "STOP_TIME" => 10,
            "INTERPOLATION" => 11,
            "INTERPOLATION_DEGREE" => 12,
            _ => return None,
        })
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Phase {
        Header,
        Metadata,
        Ephemeris,
        Covariance,
    }

    let mut phase = Phase::Header;
    let mut header_rank_seen = None;
    let mut metadata_rank_seen = 0u8;
    let mut segments = 0usize;
    let mut state_records = 0usize;
    let mut covariance_records = 0usize;
    let mut total_records = 0usize;
    let mut covariance_row = 0usize;
    let mut covariance_epoch_seen = false;
    let mut covariance_frame_seen = false;
    let mut covariance_closed = false;
    let mut offset = 0usize;

    let enforce_record_limit = |actual: usize| -> Result<()> {
        if let Some(limit) = options.max_records {
            if actual > limit {
                return Err(CcsdsNdmError::ResourceLimitExceeded {
                    resource: "history_records",
                    limit,
                    actual,
                });
            }
        }
        Ok(())
    };

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.len() > 254 {
            return Err(invalid(
                line_number,
                offset,
                "line exceeds the normative 254-character limit",
            ));
        }
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return Err(invalid(
                line_number,
                offset,
                "non-printable or non-ASCII character",
            ));
        }
        let line = line.trim_start();
        if line.trim_end().is_empty() {
            offset += raw_line.len() + 1;
            continue;
        }
        if line == "COMMENT" || line.starts_with("COMMENT ") {
            let allowed = match phase {
                Phase::Header => header_rank_seen == Some(0),
                Phase::Metadata => metadata_rank_seen == 0,
                Phase::Ephemeris => state_records == 0,
                Phase::Covariance => !covariance_epoch_seen && covariance_row == 0,
            };
            if !allowed {
                return Err(invalid(
                    line_number,
                    offset,
                    "COMMENT is not at the beginning of an allowed OEM block",
                ));
            }
            offset += raw_line.len() + 1;
            continue;
        }
        let line = line.trim_end();

        match line {
            "META_START" => {
                let allowed = match phase {
                    Phase::Header => header_rank_seen.is_some(),
                    Phase::Ephemeris => state_records > 0,
                    _ => false,
                };
                if !allowed {
                    return Err(invalid(line_number, offset, "unexpected META_START"));
                }
                phase = Phase::Metadata;
                metadata_rank_seen = 0;
                state_records = 0;
                covariance_records = 0;
                covariance_closed = false;
                segments += 1;
            }
            "META_STOP" => {
                if phase != Phase::Metadata {
                    return Err(invalid(line_number, offset, "unexpected META_STOP"));
                }
                phase = Phase::Ephemeris;
            }
            "COVARIANCE_START" => {
                if phase != Phase::Ephemeris || state_records == 0 || covariance_closed {
                    return Err(invalid(
                        line_number,
                        offset,
                        "COVARIANCE_START must follow ephemeris records",
                    ));
                }
                phase = Phase::Covariance;
                covariance_row = 0;
                covariance_epoch_seen = false;
                covariance_frame_seen = false;
            }
            "COVARIANCE_STOP" => {
                if phase != Phase::Covariance || covariance_records == 0 || covariance_row != 6 {
                    return Err(invalid(
                        line_number,
                        offset,
                        "COVARIANCE_STOP must follow a complete covariance matrix",
                    ));
                }
                phase = Phase::Ephemeris;
                covariance_closed = true;
            }
            _ if line.contains('=') => {
                let (key, _) = line
                    .split_once('=')
                    .expect("the match guard found an equals");
                let key = key.trim();
                match phase {
                    Phase::Header => {
                        let rank = header_rank(key).ok_or_else(|| {
                            invalid(line_number, offset, "unknown OEM header keyword")
                        })?;
                        if header_rank_seen.is_some_and(|previous| rank <= previous) {
                            return Err(invalid(
                                line_number,
                                offset,
                                "duplicate or out-of-order OEM header keyword",
                            ));
                        }
                        if header_rank_seen.is_none() && rank != 0 {
                            return Err(invalid(
                                line_number,
                                offset,
                                "CCSDS_OEM_VERS must be the first record",
                            ));
                        }
                        header_rank_seen = Some(rank);
                    }
                    Phase::Metadata => {
                        let rank = metadata_rank(key).ok_or_else(|| {
                            invalid(line_number, offset, "unknown OEM metadata keyword")
                        })?;
                        if rank <= metadata_rank_seen {
                            return Err(invalid(
                                line_number,
                                offset,
                                "duplicate or out-of-order OEM metadata keyword",
                            ));
                        }
                        metadata_rank_seen = rank;
                    }
                    Phase::Covariance => match key {
                        "EPOCH" if covariance_row == 0 || covariance_row == 6 => {
                            covariance_row = 0;
                            covariance_epoch_seen = true;
                            covariance_frame_seen = false;
                            covariance_records += 1;
                            total_records += 1;
                            enforce_record_limit(total_records)?;
                        }
                        "COV_REF_FRAME"
                            if covariance_epoch_seen
                                && covariance_row == 0
                                && !covariance_frame_seen =>
                        {
                            covariance_frame_seen = true;
                        }
                        _ => {
                            return Err(invalid(
                                line_number,
                                offset,
                                "unexpected covariance keyword",
                            ));
                        }
                    },
                    Phase::Ephemeris => {
                        return Err(invalid(
                            line_number,
                            offset,
                            "assignments are not allowed in OEM ephemeris records",
                        ));
                    }
                }
            }
            _ => match phase {
                Phase::Ephemeris if !covariance_closed => {
                    state_records += 1;
                    total_records += 1;
                    enforce_record_limit(total_records)?;
                }
                Phase::Covariance if covariance_epoch_seen && covariance_row < 6 => {
                    covariance_row += 1;
                }
                _ => return Err(invalid(line_number, offset, "unexpected OEM record")),
            },
        }
        offset += raw_line.len() + 1;
    }

    if segments == 0 || phase != Phase::Ephemeris || state_records == 0 {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "incomplete OEM document",
        ));
    }
    Ok(())
}

fn validate_oem_xml_envelope(
    xml: &str,
    options: &crate::options::ParseOptions,
    source_edition: &mut Option<String>,
) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    const OEM: &[&[u8]] = &[b"header", b"body"];
    const HEADER: &[&[u8]] = &[
        b"COMMENT",
        b"CLASSIFICATION",
        b"CREATION_DATE",
        b"ORIGINATOR",
        b"MESSAGE_ID",
    ];
    const BODY: &[&[u8]] = &[b"segment"];
    const SEGMENT: &[&[u8]] = &[b"metadata", b"data"];
    const METADATA: &[&[u8]] = &[
        b"COMMENT",
        b"OBJECT_NAME",
        b"OBJECT_ID",
        b"CENTER_NAME",
        b"REF_FRAME",
        b"REF_FRAME_EPOCH",
        b"TIME_SYSTEM",
        b"START_TIME",
        b"USEABLE_START_TIME",
        b"USEABLE_STOP_TIME",
        b"STOP_TIME",
        b"INTERPOLATION",
        b"INTERPOLATION_DEGREE",
    ];
    const DATA: &[&[u8]] = &[b"COMMENT", b"stateVector", b"covarianceMatrix"];
    const STATE_VECTOR: &[&[u8]] = &[
        b"EPOCH", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT", b"X_DDOT", b"Y_DDOT", b"Z_DDOT",
    ];
    const COVARIANCE: &[&[u8]] = &[
        b"COMMENT",
        b"EPOCH",
        b"COV_REF_FRAME",
        b"CX_X",
        b"CY_X",
        b"CY_Y",
        b"CZ_X",
        b"CZ_Y",
        b"CZ_Z",
        b"CX_DOT_X",
        b"CX_DOT_Y",
        b"CX_DOT_Z",
        b"CX_DOT_X_DOT",
        b"CY_DOT_X",
        b"CY_DOT_Y",
        b"CY_DOT_Z",
        b"CY_DOT_X_DOT",
        b"CY_DOT_Y_DOT",
        b"CZ_DOT_X",
        b"CZ_DOT_Y",
        b"CZ_DOT_Z",
        b"CZ_DOT_X_DOT",
        b"CZ_DOT_Y_DOT",
        b"CZ_DOT_Z_DOT",
    ];

    fn rule(parent: &[u8], child: &[u8]) -> Option<XmlSequenceRule> {
        let sequence = match parent {
            b"oem" => OEM,
            b"header" => HEADER,
            b"body" => BODY,
            b"segment" => SEGMENT,
            b"metadata" => METADATA,
            b"data" => DATA,
            b"stateVector" => STATE_VECTOR,
            b"covarianceMatrix" => COVARIANCE,
            _ => return None,
        };
        sequence
            .iter()
            .position(|candidate| *candidate == child)
            .map(|rank| XmlSequenceRule {
                rank: rank as u16,
                repeatable: matches!(
                    child,
                    b"COMMENT" | b"segment" | b"stateVector" | b"covarianceMatrix"
                ),
            })
    }

    crate::xml::validate_standalone_document(
        xml,
        b"oem",
        "OEM",
        options,
        source_edition,
        crate::xml::MessageSchema {
            child_rule: rule,
            attribute_allowed: |element: &[u8], attribute: &[u8]| match attribute {
                b"units" => matches!(
                    element,
                    b"X" | b"Y"
                        | b"Z"
                        | b"X_DOT"
                        | b"Y_DOT"
                        | b"Z_DOT"
                        | b"X_DDOT"
                        | b"Y_DDOT"
                        | b"Z_DDOT"
                        | b"CX_X"
                        | b"CY_X"
                        | b"CY_Y"
                        | b"CZ_X"
                        | b"CZ_Y"
                        | b"CZ_Z"
                        | b"CX_DOT_X"
                        | b"CX_DOT_Y"
                        | b"CX_DOT_Z"
                        | b"CX_DOT_X_DOT"
                        | b"CY_DOT_X"
                        | b"CY_DOT_Y"
                        | b"CY_DOT_Z"
                        | b"CY_DOT_X_DOT"
                        | b"CY_DOT_Y_DOT"
                        | b"CZ_DOT_X"
                        | b"CZ_DOT_Y"
                        | b"CZ_DOT_Z"
                        | b"CZ_DOT_X_DOT"
                        | b"CZ_DOT_Y_DOT"
                        | b"CZ_DOT_Z_DOT"
                ),
                _ => false,
            },
            // Both repeatable OEM data records are bounded before serde materializes any of
            // them.
            is_record: |element: &[u8]| matches!(element, b"stateVector" | b"covarianceMatrix"),
        },
    )
}

impl ToKvn for Oem {
    fn validate_kvn(&self) -> Result<()> {
        self.validate_kvn_generation()
    }

    fn write_kvn(&self, writer: &mut KvnWriter) {
        // Callers reach this only after `validate_kvn` has run the same pass against a sink, so
        // the layout cannot fail here; the writer records any late I/O or lexical fault itself.
        let _ = self.write_validated_kvn(writer);
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
    use crate::traits::Ndm;

    #[test]
    fn oem_covariance_rejects_a_negative_variance() {
        // The first covariance row of the shipped fixture is CX_X, a variance.
        let kvn =
            include_str!("../../data/kvn/oem_g13.kvn").replace("3.3313494e-04", "-3.3313494e-04");
        let error = Oem::from_kvn(&kvn).expect_err("a negative variance is not representable");
        let message = error.to_string();
        assert!(
            message.contains("CX_X")
                && message.contains("a non-negative variance on the covariance diagonal"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn oem_covariance_accepts_a_negative_off_diagonal_term() {
        // CZ_X is an off-diagonal covariance, and the fixture already carries it negative.
        let kvn = include_str!("../../data/kvn/oem_g13.kvn");
        assert!(kvn.contains("-3.0700078e-04"));
        Oem::from_kvn(kvn).expect("shipped OEM fixture should parse");
    }

    #[test]
    fn odm_number_lexical_validation_matches_book_forms() {
        for value in [
            "0",
            "-2147483648",
            "2147483647",
            "0.0",
            "-12.5",
            "1.234567890123456",
            "1.0e0",
            "-1.234567890123456E+308",
        ] {
            assert!(valid_odm_number(value), "{value}");
        }
        for value in [
            "",
            "+",
            "2147483648",
            "-2147483649",
            ".5",
            "1.",
            "12e3",
            "1e3",
            "1.0e",
            "1.0e+",
            "1.2345678901234567",
        ] {
            assert!(!valid_odm_number(value), "{value}");
        }
    }

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
COMMENT Another data comment
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
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
        let xml = include_str!("../../data/xml/oem_g14.xml");
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
        let kvn = include_str!("../../data/kvn/oem_g11.kvn");
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
        let kvn = include_str!("../../data/kvn/oem_g13.kvn");
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
        let xml = include_str!("../../data/xml/oem_g14.xml");
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
USEABLE_STOP_TIME = 2025-01-01T23:50:00
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
