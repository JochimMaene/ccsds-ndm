// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
#[cfg(test)]
use crate::traits::Validate;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::io::Write;

//----------------------------------------------------------------------
// Root AEM Structure
//----------------------------------------------------------------------

/// Attitude Ephemeris Message (AEM).
///
/// An AEM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The AEM is suited to interagency exchanges that involve automated
/// interaction and require higher fidelity or higher precision dynamic modeling than is
/// possible with the APM.
///
/// The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric
/// torques, magnetics, etc.). It requires the use of an interpolation technique to interpret
/// the attitude state at times different from the tabular epochs.
///
/// **CCSDS Reference**: 504.0-B-2, Section 4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "aem")]
pub struct Aem {
    pub header: AdmHeader,
    pub body: AemBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_AEM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "2.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Aem {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Aem,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        crate::validation::collect_message_validation_errors(
            crate::validation::MessageKind::Aem,
            &self.id,
            &self.version,
            &self.header,
            &self.body,
        )
    }
}

impl Ndm for Aem {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Aem,
            &self.version,
            crate::generation::OutputFormat::Kvn,
            self,
        )?;
        self.validate_kvn_representability()?;
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        writer.finish_checked()
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        validate_kvn_syntax(kvn)?;
        let aem = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Aem,
            &self.version,
            crate::generation::OutputFormat::Xml,
            self,
        )?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"aem", "AEM")?;
        validate_xml_sequences(xml)?;
        let aem: Self = crate::xml::from_str_with_context(xml, "AEM")?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
    }
}

fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;
    crate::xml::validate_element_sequences(
        xml,
        "AEM",
        |parent, child| {
            let children = aem_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable =
                child == b"COMMENT" || child == b"segment" || child == b"attitudeState";
            Some(XmlSequenceRule { rank, repeatable })
        },
        |element, attribute| {
            attribute == b"units"
                && matches!(
                    element,
                    b"Q1_DOT"
                        | b"Q2_DOT"
                        | b"Q3_DOT"
                        | b"QC_DOT"
                        | b"ANGLE_1"
                        | b"ANGLE_2"
                        | b"ANGLE_3"
                        | b"ANGLE_1_DOT"
                        | b"ANGLE_2_DOT"
                        | b"ANGLE_3_DOT"
                        | b"ANGVEL_X"
                        | b"ANGVEL_Y"
                        | b"ANGVEL_Z"
                        | b"SPIN_ALPHA"
                        | b"SPIN_DELTA"
                        | b"SPIN_ANGLE"
                        | b"SPIN_ANGLE_VEL"
                        | b"NUTATION"
                        | b"NUTATION_PER"
                        | b"NUTATION_PHASE"
                        | b"MOMENTUM_ALPHA"
                        | b"MOMENTUM_DELTA"
                        | b"NUTATION_VEL"
                )
        },
    )
}

fn aem_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"aem" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CLASSIFICATION",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"segment"],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT_NAME",
            b"OBJECT_ID",
            b"CENTER_NAME",
            b"REF_FRAME_A",
            b"REF_FRAME_B",
            b"TIME_SYSTEM",
            b"START_TIME",
            b"USEABLE_START_TIME",
            b"USEABLE_STOP_TIME",
            b"STOP_TIME",
            b"ATTITUDE_TYPE",
            b"EULER_ROT_SEQ",
            b"ANGVEL_FRAME",
            b"INTERPOLATION_METHOD",
            b"INTERPOLATION_DEGREE",
        ],
        b"data" => &[b"COMMENT", b"attitudeState"],
        b"attitudeState" => &[
            b"quaternionEphemeris",
            b"quaternionDerivative",
            b"quaternionAngVel",
            b"eulerAngle",
            b"eulerAngleDerivative",
            b"eulerAngleAngVel",
            b"spin",
            b"spinNutation",
            b"spinNutationMom",
        ],
        b"quaternionEphemeris" => &[b"EPOCH", b"quaternion"],
        b"quaternionDerivative" => &[b"EPOCH", b"quaternion", b"quaternionDot"],
        b"quaternionAngVel" => &[b"EPOCH", b"quaternion", b"angVel"],
        b"quaternion" => &[b"Q1", b"Q2", b"Q3", b"QC"],
        b"quaternionDot" => &[b"Q1_DOT", b"Q2_DOT", b"Q3_DOT", b"QC_DOT"],
        b"angVel" => &[b"ANGVEL_X", b"ANGVEL_Y", b"ANGVEL_Z"],
        b"eulerAngle" => &[b"EPOCH", b"ANGLE_1", b"ANGLE_2", b"ANGLE_3"],
        b"eulerAngleDerivative" => &[
            b"EPOCH",
            b"ANGLE_1",
            b"ANGLE_2",
            b"ANGLE_3",
            b"ANGLE_1_DOT",
            b"ANGLE_2_DOT",
            b"ANGLE_3_DOT",
        ],
        b"eulerAngleAngVel" => &[
            b"EPOCH",
            b"ANGLE_1",
            b"ANGLE_2",
            b"ANGLE_3",
            b"ANGVEL_X",
            b"ANGVEL_Y",
            b"ANGVEL_Z",
        ],
        b"spin" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
        ],
        b"spinNutation" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
            b"NUTATION",
            b"NUTATION_PER",
            b"NUTATION_PHASE",
        ],
        b"spinNutationMom" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
            b"MOMENTUM_ALPHA",
            b"MOMENTUM_DELTA",
            b"NUTATION_VEL",
        ],
        _ => return None,
    })
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const HEADER: &[&str] = &[
        "CLASSIFICATION",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
    ];
    const META: &[&str] = &[
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "TIME_SYSTEM",
        "START_TIME",
        "USEABLE_START_TIME",
        "USEABLE_STOP_TIME",
        "STOP_TIME",
        "ATTITUDE_TYPE",
        "EULER_ROT_SEQ",
        "RATE_FRAME",
        "INTERPOLATION_METHOD",
        "INTERPOLATION_DEGREE",
    ];
    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating AEM KVN structure"],
            offset,
        }))))
    };
    let mut block = None;
    let mut last_block = None;
    let mut previous_key = None;
    let mut top_rank = None;
    let mut block_has_content = false;
    let mut offset = 0usize;
    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
        if line.as_bytes().contains(&b'\r') {
            return fail("lone carriage return");
        }
        if line.len() > 254 {
            return fail("line exceeds the normative 254-character limit");
        }
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return fail("non-printable or non-ASCII character");
        }
        let line = line.trim();
        if line.is_empty() {
            offset += raw_line.len() + 1;
            continue;
        }
        if line == "COMMENT" || line.starts_with("COMMENT ") {
            if block.is_none() && top_rank != Some(0) {
                return fail("AEM header COMMENT must immediately follow the version record");
            }
            if block_has_content && !(block.is_none() && top_rank == Some(0)) {
                return fail("COMMENT is not at the beginning of an AEM logical block");
            }
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if block.is_some() || !matches!(marker, "META" | "DATA") {
                return fail("unknown or nested AEM marked block");
            }
            if (marker == "META" && matches!(last_block, Some("META")))
                || (marker == "DATA" && last_block != Some("META"))
            {
                return fail("out-of-order AEM marked block");
            }
            block = Some(marker);
            previous_key = None;
            block_has_content = false;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if block != Some(marker) {
                return fail("mismatched AEM marked block end");
            }
            block = None;
            last_block = Some(marker);
            previous_key = None;
            block_has_content = false;
            offset += raw_line.len() + 1;
            continue;
        }
        match block {
            Some("DATA") => {
                if line.contains('=') {
                    return fail("assignment in AEM attitude-state history");
                }
                block_has_content = true;
            }
            Some("META") => {
                if !line.contains('=') {
                    return fail("expected one AEM metadata assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = META
                    .iter()
                    .position(|candidate| *candidate == key)
                    .ok_or_else(|| {
                        invalid(number, offset, "unknown AEM metadata keyword".into())
                    })?;
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order AEM metadata keyword");
                }
                previous_key = Some(rank);
                block_has_content = true;
            }
            None => {
                if last_block.is_some() {
                    return fail("content outside an AEM marked block");
                }
                if !line.contains('=') {
                    return fail("expected one AEM header assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = if key == "CCSDS_AEM_VERS" {
                    0
                } else {
                    HEADER
                        .iter()
                        .position(|candidate| *candidate == key)
                        .map(|rank| rank + 1)
                        .ok_or_else(|| {
                            invalid(number, offset, "unknown AEM header keyword".into())
                        })?
                };
                if top_rank.is_none() && rank != 0 {
                    return fail("CCSDS_AEM_VERS must be the first record");
                }
                if top_rank.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order AEM header keyword");
                }
                top_rank = Some(rank);
                block_has_content = true;
            }
            _ => unreachable!(),
        }
        offset += raw_line.len() + 1;
    }
    if block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unclosed AEM marked block".into(),
        ));
    }
    Ok(())
}

impl Aem {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        let check_text = |value: &str| -> Result<()> {
            if !value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                return Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "AEM KVN free text must contain printable ASCII without line breaks",
                    ),
                    line: None,
                }
                .into());
            }
            Ok(())
        };
        for comment in &self.header.comment {
            check_text(comment)?;
        }
        if let Some(value) = &self.header.classification {
            check_text(value)?;
        }
        check_text(&self.header.originator)?;
        if let Some(value) = &self.header.message_id {
            check_text(value)?;
        }
        for segment in &self.body.segment {
            for value in [
                Some(&segment.metadata.object_name),
                Some(&segment.metadata.object_id),
                segment.metadata.center_name.as_ref(),
                Some(&segment.metadata.ref_frame_a),
                Some(&segment.metadata.ref_frame_b),
                Some(&segment.metadata.time_system),
                segment.metadata.angvel_frame.as_ref(),
                segment.metadata.interpolation_method.as_ref(),
            ]
            .into_iter()
            .flatten()
            {
                check_text(value)?;
            }
            for comment in segment.metadata.comment.iter().chain(&segment.data.comment) {
                check_text(comment)?;
            }
            for state in &segment.data.attitude_states {
                validate_aem_state_numbers(state)?;
            }
        }
        let mut sink = AemKvnLexicalSink::default();
        let mut writer = KvnWriter::from_io(&mut sink);
        self.write_kvn(&mut writer);
        writer.finish_io()
    }
}

fn validate_aem_state_numbers(state: &AemAttitudeStateWrapper) -> Result<()> {
    let check = |values: &[f64]| -> Result<()> {
        if values
            .iter()
            .all(|value| crate::kvn::ser::OdmFloat::is_valid(*value))
        {
            Ok(())
        } else {
            Err(ValidationError::Generic {
                message: Cow::Borrowed("AEM KVN attitude-state numbers must be finite"),
                line: None,
            }
            .into())
        }
    };
    if let Some(v) = &state.quaternion_ephemeris {
        check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
        ])?;
    }
    if let Some(v) = &state.quaternion_derivative {
        check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
            v.quaternion_dot.q1_dot.value,
            v.quaternion_dot.q2_dot.value,
            v.quaternion_dot.q3_dot.value,
            v.quaternion_dot.qc_dot.value,
        ])?;
    }
    if let Some(v) = &state.quaternion_ang_vel {
        check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
            v.ang_vel.angvel_x.value,
            v.ang_vel.angvel_y.value,
            v.ang_vel.angvel_z.value,
        ])?;
    }
    if let Some(v) = &state.euler_angle {
        check(&[v.angle_1.value, v.angle_2.value, v.angle_3.value])?;
    }
    if let Some(v) = &state.euler_angle_derivative {
        check(&[
            v.angle_1.value,
            v.angle_2.value,
            v.angle_3.value,
            v.angle_1_dot.value,
            v.angle_2_dot.value,
            v.angle_3_dot.value,
        ])?;
    }
    if let Some(v) = &state.euler_angle_ang_vel {
        check(&[
            v.angle_1.value,
            v.angle_2.value,
            v.angle_3.value,
            v.angvel_x.value,
            v.angvel_y.value,
            v.angvel_z.value,
        ])?;
    }
    if let Some(v) = &state.spin {
        check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
        ])?;
    }
    if let Some(v) = &state.spin_nutation {
        check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
            v.nutation.value,
            v.nutation_per.value,
            v.nutation_phase.value,
        ])?;
    }
    if let Some(v) = &state.spin_nutation_mom {
        check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
            v.momentum_alpha.value,
            v.momentum_delta.value,
            v.nutation_vel.value,
        ])?;
    }
    Ok(())
}

#[derive(Default)]
struct AemKvnLexicalSink {
    line_len: usize,
}

impl Write for AemKvnLexicalSink {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        for byte in buffer {
            if *byte == b'\n' {
                if self.line_len > 254 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "generated AEM KVN record exceeds 254 characters",
                    ));
                }
                self.line_len = 0;
            } else {
                if !(b' '..=b'~').contains(byte) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "generated AEM KVN contains non-printable or non-ASCII content",
                    ));
                }
                self.line_len += 1;
            }
        }
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl crate::traits::Validate for AemBody {
    fn validate(&self) -> Result<()> {
        if self.segment.is_empty() {
            return Err(ValidationError::missing_required(
                "AEM Body",
                "segment (at least one required)",
            )
            .into());
        }
        for segment in &self.segment {
            segment.validate()?;
        }
        if let Some(error) = self.cross_segment_errors().into_iter().next() {
            return Err(error.into());
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();
        if self.segment.is_empty() {
            errors.push(ValidationError::missing_required(
                "AEM Body",
                "segment (at least one required)",
            ));
        }
        for (index, segment) in self.segment.iter().enumerate() {
            errors.extend(
                segment
                    .validation_errors()?
                    .into_iter()
                    .map(|error| error.at_path(format!("segment[{index}]"))),
            );
        }
        errors.extend(self.cross_segment_errors());
        Ok(errors)
    }
}

impl AemBody {
    fn cross_segment_errors(&self) -> Vec<ValidationError> {
        use std::cmp::Ordering;

        let mut errors = Vec::new();
        for (index, pair) in self.segment.windows(2).enumerate() {
            let previous_stop = pair[0].metadata.useable_stop_time;
            let current_start = pair[1].metadata.useable_start_time;
            if matches!(
                (previous_stop, current_start),
                (Some(previous), Some(current))
                    if previous
                        .into_epoch()
                        .cmp_same_branch(&current.into_epoch())
                        == Some(Ordering::Greater)
            ) {
                errors.push(
                    ValidationError::InvalidValue {
                        field: "USEABLE_START_TIME".into(),
                        value: current_start.unwrap().to_string(),
                        expected: "no earlier than the preceding segment's USEABLE_STOP_TIME"
                            .into(),
                        line: None,
                    }
                    .at_path(format!(
                        "segment[{}].metadata.useable_start_time",
                        index + 1
                    )),
                );
            }
        }
        errors
    }
}

impl crate::traits::Validate for AemSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        crate::traits::Validate::validate(&self.data)?;
        self.data.validate_with_type(&self.metadata.attitude_type)?;
        match self.timeline_errors().into_iter().next() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let mut errors = self.metadata.validation_errors()?;
        errors.extend(self.data.validation_errors()?);
        errors.extend(
            self.data
                .validation_errors_with_type(&self.metadata.attitude_type),
        );
        errors.extend(self.timeline_errors());
        Ok(errors)
    }
}

impl AemSegment {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }

    fn timeline_errors(&self) -> Vec<ValidationError> {
        use std::cmp::Ordering;

        let mut errors = Vec::new();
        let start = self.metadata.start_time.into_epoch();
        let stop = self.metadata.stop_time.into_epoch();
        let mut previous: Option<crate::types::Epoch> = None;
        for (index, state) in self.data.attitude_states.iter().enumerate() {
            let Some(epoch) = state.epoch() else {
                continue;
            };
            let current = epoch.into_epoch();
            if start.cmp_same_branch(&current) == Some(Ordering::Greater)
                || current.cmp_same_branch(&stop) == Some(Ordering::Greater)
            {
                errors.push(
                    ValidationError::OutOfRange {
                        name: "attitudeState EPOCH".into(),
                        value: epoch.to_string(),
                        expected: format!(
                            "within START_TIME {} and STOP_TIME {}",
                            self.metadata.start_time, self.metadata.stop_time
                        )
                        .into(),
                        line: None,
                    }
                    .at_path(format!("data.attitude_states[{index}].epoch")),
                );
            }
            if matches!(
                previous,
                Some(prior) if prior.cmp_same_branch(&current) != Some(Ordering::Less)
            ) {
                errors.push(
                    ValidationError::InvalidValue {
                        field: "attitudeState EPOCH".into(),
                        value: epoch.to_string(),
                        expected: "strictly increasing, non-repeated attitude time tags".into(),
                        line: None,
                    }
                    .at_path(format!("data.attitude_states[{index}].epoch")),
                );
            }
            previous = Some(current);
        }
        errors
    }
}

impl ToKvn for Aem {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_AEM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AemBody {
    #[serde(rename = "segment")]
    pub segment: Vec<AemSegment>,
}

impl ToKvn for AemBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for seg in &self.segment {
            seg.write_kvn(writer);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AemSegment {
    pub metadata: AemMetadata,
    pub data: AemData,
}

impl ToKvn for AemSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_line("META_START");
        self.metadata.write_kvn(writer);
        writer.write_line("META_STOP");
        writer.write_line("");
        writer.write_line("DATA_START");
        self.data.write_kvn(writer);
        writer.write_line("DATA_STOP");
        writer.write_line("");
    }
}

/// AEM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct AemMetadata {
    /// Comments allowed only at the beginning of the Metadata section. Each comment line shall
    /// begin with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include Object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// capital letter for the identification of the part brought into space by the launch. In
    /// cases in which the asset is not listed in reference [ADM-2], the UN Office of Outer Space
    /// Affairs designator index format is not used, or the content cannot be disclosed, the value
    /// should be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH, STS-106
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: ICRF, SC_BODY_1, INSTRUMENT_A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1, INSTRUMENT_A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Time system used for both attitude ephemeris data and metadata. The set of allowed values
    /// is described in annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub time_system: String,
    /// Start of TOTAL time span covered by attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    pub start_time: CalendarEpoch,
    /// Optional start of USEABLE time span covered by attitude ephemeris data immediately
    /// following this metadata block. To allow for proper interpolation near the beginning/end of
    /// the attitude ephemeris data block, it may be necessary to utilize this keyword with values
    /// within the time span covered by the attitude ephemeris data records as denoted by the
    /// START/STOP_TIME time tags. The USEABLE_START_TIME time tag of a new block of ephemeris data
    /// must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub useable_start_time: Option<CalendarEpoch>,
    /// Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
    /// this metadata block. (See also USEABLE_START_TIME.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub useable_stop_time: Option<CalendarEpoch>,
    /// End of TOTAL time span covered by the attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    pub stop_time: CalendarEpoch,
    /// The type of information contained in the data lines. This keyword must have a value from the
    /// set specified at the right. (See table 4-4 for details of the data contained in each line.)
    ///
    /// **Examples**: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
    /// EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub attitude_type: AttitudeTypeType,
    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation. This keyword is applicable only if ATTITUDE_TYPE
    /// specifies the use of Euler angles.
    ///
    /// **Examples**: ZXZ, XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub euler_rot_seq: Option<RotSeq>,
    /// The frame of reference in which angular velocity data are specified. The set of allowed
    /// values is described in annex B, subsection B3. This keyword is applicable only if
    /// ATTITUDE_TYPE specifies the use of angular velocities in conjunction with either
    /// quaternions or Euler angles.
    ///
    /// **Examples**: ICRF, SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub angvel_frame: Option<String>,
    /// Recommended interpolation method for attitude ephemeris data in the block immediately
    /// following this metadata block.
    ///
    /// **Examples**: LINEAR, HERMITE, LAGRANGE
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub interpolation_method: Option<String>,
    /// Recommended interpolation degree for attitude ephemeris data in the block immediately
    /// following this metadata block. It must be an integer value. This keyword must be used if
    /// the ‘INTERPOLATION_METHOD’ keyword is used.
    ///
    /// **Examples**: 1, 5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub interpolation_degree: Option<InterpolationDegree>,
}

impl AemMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "OBJECT_NAME").into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "OBJECT_ID").into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "TIME_SYSTEM").into());
        }
        if self.ref_frame_a.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "REF_FRAME_A").into());
        }
        if self.ref_frame_b.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "REF_FRAME_B").into());
        }
        // Validation Rule: INTERPOLATION_DEGREE is required if INTERPOLATION_METHOD is used
        if self.interpolation_method.is_some() && self.interpolation_degree.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "INTERPOLATION_DEGREE (required when INTERPOLATION_METHOD is present)",
            )
            .into());
        }

        let requires_euler_rot_seq = matches!(
            self.attitude_type,
            AttitudeTypeType::EulerAngle
                | AttitudeTypeType::EulerAngleUpper
                | AttitudeTypeType::EulerAngleDerivative
                | AttitudeTypeType::EulerAngleDerivativeUpper
                | AttitudeTypeType::EulerAngleAngVel
                | AttitudeTypeType::EulerAngleAngVelUpper
        );
        let requires_angvel_frame = matches!(
            self.attitude_type,
            AttitudeTypeType::QuaternionAngVel
                | AttitudeTypeType::QuaternionAngVelUpper
                | AttitudeTypeType::EulerAngleAngVel
                | AttitudeTypeType::EulerAngleAngVelUpper
        );

        // Validation Rule: EULER_ROT_SEQ is required if ATTITUDE_TYPE includes EULER_ANGLE
        if requires_euler_rot_seq && self.euler_rot_seq.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "EULER_ROT_SEQ (required for EULER_ANGLE types)",
            )
            .into());
        }

        // Validation Rule: ANGVEL_FRAME is required if ATTITUDE_TYPE includes ANGVEL
        if requires_angvel_frame && self.angvel_frame.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "ANGVEL_FRAME (required for ANGVEL types)",
            )
            .into());
        }

        match self.time_span_errors().into_iter().next() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn time_span_errors(&self) -> Vec<ValidationError> {
        use std::cmp::Ordering;

        let mut errors = Vec::new();
        let start = self.start_time.into_epoch();
        let stop = self.stop_time.into_epoch();
        if start.cmp_same_branch(&stop) == Some(Ordering::Greater) {
            errors.push(ValidationError::InvalidValue {
                field: "START_TIME/STOP_TIME".into(),
                value: format!("{} > {}", self.start_time, self.stop_time),
                expected: "START_TIME no later than STOP_TIME".into(),
                line: None,
            });
        }
        for (field, value) in [
            ("USEABLE_START_TIME", self.useable_start_time),
            ("USEABLE_STOP_TIME", self.useable_stop_time),
        ] {
            if let Some(value) = value {
                let value_epoch = value.into_epoch();
                if start.cmp_same_branch(&value_epoch) == Some(Ordering::Greater)
                    || value_epoch.cmp_same_branch(&stop) == Some(Ordering::Greater)
                {
                    errors.push(ValidationError::OutOfRange {
                        name: field.into(),
                        value: value.to_string(),
                        expected: "within the total START_TIME/STOP_TIME span".into(),
                        line: None,
                    });
                }
            }
        }
        if let (Some(useable_start), Some(useable_stop)) =
            (self.useable_start_time, self.useable_stop_time)
        {
            if useable_start
                .into_epoch()
                .cmp_same_branch(&useable_stop.into_epoch())
                == Some(Ordering::Greater)
            {
                errors.push(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME/USEABLE_STOP_TIME".into(),
                    value: format!("{useable_start} > {useable_stop}"),
                    expected: "USEABLE_START_TIME no later than USEABLE_STOP_TIME".into(),
                    line: None,
                });
            }
        }
        errors
    }
}

impl crate::traits::Validate for AemMetadata {
    fn validate(&self) -> Result<()> {
        self.validate()
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let requires_euler_rot_seq = matches!(
            self.attitude_type,
            AttitudeTypeType::EulerAngle
                | AttitudeTypeType::EulerAngleUpper
                | AttitudeTypeType::EulerAngleDerivative
                | AttitudeTypeType::EulerAngleDerivativeUpper
                | AttitudeTypeType::EulerAngleAngVel
                | AttitudeTypeType::EulerAngleAngVelUpper
        );
        let requires_angvel_frame = matches!(
            self.attitude_type,
            AttitudeTypeType::QuaternionAngVel
                | AttitudeTypeType::QuaternionAngVelUpper
                | AttitudeTypeType::EulerAngleAngVel
                | AttitudeTypeType::EulerAngleAngVelUpper
        );
        let mut errors = crate::validation::missing_required_fields(
            "AEM Metadata",
            [
                ("OBJECT_NAME", self.object_name.trim().is_empty()),
                ("OBJECT_ID", self.object_id.trim().is_empty()),
                ("TIME_SYSTEM", self.time_system.trim().is_empty()),
                ("REF_FRAME_A", self.ref_frame_a.trim().is_empty()),
                ("REF_FRAME_B", self.ref_frame_b.trim().is_empty()),
                (
                    "INTERPOLATION_DEGREE (required when INTERPOLATION_METHOD is present)",
                    self.interpolation_method.is_some() && self.interpolation_degree.is_none(),
                ),
                (
                    "EULER_ROT_SEQ (required for EULER_ANGLE types)",
                    requires_euler_rot_seq && self.euler_rot_seq.is_none(),
                ),
                (
                    "ANGVEL_FRAME (required for ANGVEL types)",
                    requires_angvel_frame && self.angvel_frame.is_none(),
                ),
            ],
        );
        errors.extend(self.time_span_errors());
        Ok(errors)
    }
}

impl ToKvn for AemMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("START_TIME", self.start_time);
        if let Some(v) = self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        writer.write_pair("STOP_TIME", self.stop_time);
        writer.write_pair("ATTITUDE_TYPE", &self.attitude_type);
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.angvel_frame {
            writer.write_pair("RATE_FRAME", v);
        }
        if let Some(v) = &self.interpolation_method {
            writer.write_pair("INTERPOLATION_METHOD", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

/// AEM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct AemData {
    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Attitude ephemeris data lines.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[serde(rename = "attitudeState")]
    #[builder(default)]
    pub attitude_states: Vec<AemAttitudeStateWrapper>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AemAttitudeStateWrapper {
    #[serde(
        rename = "quaternionEphemeris",
        skip_serializing_if = "Option::is_none"
    )]
    pub quaternion_ephemeris: Option<crate::common::QuaternionEphemeris>,
    #[serde(
        rename = "quaternionDerivative",
        skip_serializing_if = "Option::is_none"
    )]
    pub quaternion_derivative: Option<crate::common::QuaternionDerivative>,
    #[serde(
        rename = "quaternionAngVel",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub quaternion_ang_vel: Option<crate::common::QuaternionAngVel>,
    #[serde(
        rename = "eulerAngle",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub euler_angle: Option<crate::common::EulerAngle>,
    #[serde(
        rename = "eulerAngleDerivative",
        skip_serializing_if = "Option::is_none"
    )]
    pub euler_angle_derivative: Option<crate::common::EulerAngleDerivative>,
    #[serde(
        rename = "eulerAngleAngVel",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub euler_angle_ang_vel: Option<crate::common::EulerAngleAngVel>,
    #[serde(rename = "spin", skip_serializing_if = "Option::is_none", default)]
    pub spin: Option<crate::common::Spin>,
    #[serde(
        rename = "spinNutation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub spin_nutation: Option<crate::common::SpinNutation>,
    #[serde(
        rename = "spinNutationMom",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub spin_nutation_mom: Option<crate::common::SpinNutationMom>,
}

impl From<crate::common::AemAttitudeState> for AemAttitudeStateWrapper {
    fn from(state: crate::common::AemAttitudeState) -> Self {
        let mut wrapper = AemAttitudeStateWrapper {
            quaternion_ephemeris: None,
            quaternion_derivative: None,
            quaternion_ang_vel: None,
            euler_angle: None,
            euler_angle_derivative: None,
            euler_angle_ang_vel: None,
            spin: None,
            spin_nutation: None,
            spin_nutation_mom: None,
        };
        match state {
            crate::common::AemAttitudeState::QuaternionEphemeris(v) => {
                wrapper.quaternion_ephemeris = Some(v)
            }
            crate::common::AemAttitudeState::QuaternionDerivative(v) => {
                wrapper.quaternion_derivative = Some(v)
            }
            crate::common::AemAttitudeState::QuaternionAngVel(v) => {
                wrapper.quaternion_ang_vel = Some(v)
            }
            crate::common::AemAttitudeState::EulerAngle(v) => wrapper.euler_angle = Some(v),
            crate::common::AemAttitudeState::EulerAngleDerivative(v) => {
                wrapper.euler_angle_derivative = Some(v)
            }
            crate::common::AemAttitudeState::EulerAngleAngVel(v) => {
                wrapper.euler_angle_ang_vel = Some(v)
            }
            crate::common::AemAttitudeState::Spin(v) => wrapper.spin = Some(v),
            crate::common::AemAttitudeState::SpinNutation(v) => wrapper.spin_nutation = Some(v),
            crate::common::AemAttitudeState::SpinNutationMom(v) => {
                wrapper.spin_nutation_mom = Some(v)
            }
        }
        wrapper
    }
}

impl AemAttitudeStateWrapper {
    pub fn content(&self) -> Option<crate::common::AemAttitudeState> {
        if let Some(v) = &self.quaternion_ephemeris {
            return Some(crate::common::AemAttitudeState::QuaternionEphemeris(
                v.clone(),
            ));
        }
        if let Some(v) = &self.quaternion_derivative {
            return Some(crate::common::AemAttitudeState::QuaternionDerivative(
                v.clone(),
            ));
        }
        if let Some(v) = &self.quaternion_ang_vel {
            return Some(crate::common::AemAttitudeState::QuaternionAngVel(v.clone()));
        }
        if let Some(v) = &self.euler_angle {
            return Some(crate::common::AemAttitudeState::EulerAngle(v.clone()));
        }
        if let Some(v) = &self.euler_angle_derivative {
            return Some(crate::common::AemAttitudeState::EulerAngleDerivative(
                v.clone(),
            ));
        }
        if let Some(v) = &self.euler_angle_ang_vel {
            return Some(crate::common::AemAttitudeState::EulerAngleAngVel(v.clone()));
        }
        if let Some(v) = &self.spin {
            return Some(crate::common::AemAttitudeState::Spin(v.clone()));
        }
        if let Some(v) = &self.spin_nutation {
            return Some(crate::common::AemAttitudeState::SpinNutation(v.clone()));
        }
        if let Some(v) = &self.spin_nutation_mom {
            return Some(crate::common::AemAttitudeState::SpinNutationMom(v.clone()));
        }
        None
    }

    fn epoch(&self) -> Option<&CalendarEpoch> {
        self.quaternion_ephemeris
            .as_ref()
            .map(|state| &state.epoch)
            .or_else(|| {
                self.quaternion_derivative
                    .as_ref()
                    .map(|state| &state.epoch)
            })
            .or_else(|| self.quaternion_ang_vel.as_ref().map(|state| &state.epoch))
            .or_else(|| self.euler_angle.as_ref().map(|state| &state.epoch))
            .or_else(|| {
                self.euler_angle_derivative
                    .as_ref()
                    .map(|state| &state.epoch)
            })
            .or_else(|| self.euler_angle_ang_vel.as_ref().map(|state| &state.epoch))
            .or_else(|| self.spin.as_ref().map(|state| &state.epoch))
            .or_else(|| self.spin_nutation.as_ref().map(|state| &state.epoch))
            .or_else(|| self.spin_nutation_mom.as_ref().map(|state| &state.epoch))
    }

    fn populated_fields(&self) -> Vec<Cow<'static, str>> {
        let mut fields = Vec::new();
        for (field, populated) in [
            ("quaternionEphemeris", self.quaternion_ephemeris.is_some()),
            ("quaternionDerivative", self.quaternion_derivative.is_some()),
            ("quaternionAngVel", self.quaternion_ang_vel.is_some()),
            ("eulerAngle", self.euler_angle.is_some()),
            (
                "eulerAngleDerivative",
                self.euler_angle_derivative.is_some(),
            ),
            ("eulerAngleAngVel", self.euler_angle_ang_vel.is_some()),
            ("spin", self.spin.is_some()),
            ("spinNutation", self.spin_nutation.is_some()),
            ("spinNutationMom", self.spin_nutation_mom.is_some()),
        ] {
            if populated {
                fields.push(Cow::Borrowed(field));
            }
        }
        fields
    }

    fn matches_type(&self, attitude_type: &AttitudeTypeType) -> bool {
        match attitude_type {
            AttitudeTypeType::Quaternion | AttitudeTypeType::QuaternionUpper => {
                self.quaternion_ephemeris.is_some()
            }
            AttitudeTypeType::QuaternionDerivative
            | AttitudeTypeType::QuaternionDerivativeUpper => self.quaternion_derivative.is_some(),
            AttitudeTypeType::QuaternionAngVel | AttitudeTypeType::QuaternionAngVelUpper => {
                self.quaternion_ang_vel.is_some()
            }
            AttitudeTypeType::EulerAngle | AttitudeTypeType::EulerAngleUpper => {
                self.euler_angle.is_some()
            }
            AttitudeTypeType::EulerAngleDerivative
            | AttitudeTypeType::EulerAngleDerivativeUpper => self.euler_angle_derivative.is_some(),
            AttitudeTypeType::EulerAngleAngVel | AttitudeTypeType::EulerAngleAngVelUpper => {
                self.euler_angle_ang_vel.is_some()
            }
            AttitudeTypeType::Spin | AttitudeTypeType::SpinUpper => self.spin.is_some(),
            AttitudeTypeType::SpinNutation | AttitudeTypeType::SpinNutationUpper => {
                self.spin_nutation.is_some()
            }
            AttitudeTypeType::SpinNutationMom | AttitudeTypeType::SpinNutationMomUpper => {
                self.spin_nutation_mom.is_some()
            }
        }
    }
}

impl crate::traits::ToKvn for AemAttitudeStateWrapper {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        if let Some(value) = &self.quaternion_ephemeris {
            value.write_kvn(writer);
        } else if let Some(value) = &self.quaternion_derivative {
            value.write_kvn(writer);
        } else if let Some(value) = &self.quaternion_ang_vel {
            value.write_kvn(writer);
        } else if let Some(value) = &self.euler_angle {
            value.write_kvn(writer);
        } else if let Some(value) = &self.euler_angle_derivative {
            value.write_kvn(writer);
        } else if let Some(value) = &self.euler_angle_ang_vel {
            value.write_kvn(writer);
        } else if let Some(value) = &self.spin {
            value.write_kvn(writer);
        } else if let Some(value) = &self.spin_nutation {
            value.write_kvn(writer);
        } else if let Some(value) = &self.spin_nutation_mom {
            value.write_kvn(writer);
        }
    }
}

impl crate::traits::Validate for AemData {
    fn validate(&self) -> Result<()> {
        if self.attitude_states.is_empty() {
            return Err(ValidationError::missing_required(
                "AEM Data",
                "attitudeState (at least one required)",
            )
            .into());
        }
        for (idx, state) in self.attitude_states.iter().enumerate() {
            let count = [
                state.quaternion_ephemeris.is_some(),
                state.quaternion_derivative.is_some(),
                state.quaternion_ang_vel.is_some(),
                state.euler_angle.is_some(),
                state.euler_angle_derivative.is_some(),
                state.euler_angle_ang_vel.is_some(),
                state.spin.is_some(),
                state.spin_nutation.is_some(),
                state.spin_nutation_mom.is_some(),
            ]
            .into_iter()
            .filter(|populated| *populated)
            .count();
            match count {
                0 => {
                    return Err(ValidationError::missing_required(
                        "AEM Data",
                        format!("attitudeState[{}] (exactly one choice required)", idx + 1),
                    )
                    .into());
                }
                1 => {}
                _ => {
                    return Err(ValidationError::conflict(state.populated_fields()).into());
                }
            }
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let mut errors = crate::validation::missing_required_fields(
            "AEM Data",
            [(
                "attitudeState (at least one required)",
                self.attitude_states.is_empty(),
            )],
        );
        for (index, state) in self.attitude_states.iter().enumerate() {
            let fields = state.populated_fields();
            match fields.len() {
                0 => errors.push(ValidationError::missing_required(
                    "AEM Data",
                    format!("attitudeState[{}] (exactly one choice required)", index + 1),
                )),
                1 => {}
                _ => errors.push(ValidationError::conflict(fields)),
            }
        }
        Ok(errors)
    }
}

impl AemData {
    pub fn validate_with_type(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
        for (idx, state) in self.attitude_states.iter().enumerate() {
            if !state.matches_type(attitude_type) {
                return Err(ValidationError::generic(format!(
                    "Data line {} expected {} data",
                    idx + 1,
                    attitude_type
                ))
                .into());
            }
        }
        Ok(())
    }

    fn validation_errors_with_type(
        &self,
        attitude_type: &AttitudeTypeType,
    ) -> Vec<ValidationError> {
        let expected = attitude_type.to_string();
        self.attitude_states
            .iter()
            .enumerate()
            .filter(|(_, state)| !state.matches_type(attitude_type))
            .map(|(index, _)| {
                ValidationError::generic(format!(
                    "Data line {} expected {} data",
                    index + 1,
                    expected
                ))
            })
            .collect()
    }

    pub fn validate(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
        crate::traits::Validate::validate(self)?;
        self.validate_with_type(attitude_type)
    }
}

impl ToKvn for AemData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        for state in &self.attitude_states {
            state.write_kvn(writer);
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_aem_kvn() -> String {
        r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2002-12-18T12:00:00.000
STOP_TIME = 2002-12-18T12:01:00.000
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2002-12-18T12:00:00.000 0.5 0.5 0.5 0.5
2002-12-18T12:01:00.000 0.5 0.5 0.5 0.5
DATA_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_aem_success() {
        let kvn = sample_aem_kvn();
        let aem = Aem::from_kvn(&kvn).expect("AEM parse failed");

        assert_eq!(aem.version, "2.0");
        assert_eq!(aem.body.segment.len(), 1);
        let seg = &aem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert_eq!(seg.data.attitude_states.len(), 2);
    }

    #[test]
    fn test_aem_missing_mandatory_metadata() {
        let kvn = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        // Missing OBJECT_NAME
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_valid_epoch_ordering() {
        // The sample has strictly increasing epochs and should pass timeline validation.
        let kvn = sample_aem_kvn();
        assert!(Aem::from_kvn(&kvn).is_ok());
    }

    #[test]
    fn test_aem_validation_interpolation_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
INTERPOLATION_METHOD = HERMITE
# Missing INTERPOLATION_DEGREE
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_validation_euler_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = EULER_ANGLE
# Missing EULER_ROT_SEQ
META_STOP
DATA_START
2023-01-01T00:00:00 10 20 30
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_validation_angvel_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION/ANGVEL
# Missing ANGVEL_FRAME
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1 0.1 0.1 0.1
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }
    #[test]
    fn test_aem_data_validation_mismatches() {
        use crate::common::*;

        let valid_q = AemAttitudeStateWrapper::from(AemAttitudeState::QuaternionEphemeris(
            QuaternionEphemeris {
                epoch: "2023-01-01T00:00:00".parse().unwrap(),
                quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
            },
        ));

        let valid_euler = AemAttitudeStateWrapper::from(AemAttitudeState::EulerAngle(EulerAngle {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
        }));

        // Type mismatch: Expects QUATERNION, gets EULER_ANGLE
        let data = AemData {
            comment: vec![],
            attitude_states: vec![valid_euler.clone()],
        };
        assert!(data.validate(&AttitudeTypeType::QuaternionUpper).is_err());

        // Type mismatch: Expects EULER_ANGLE, gets QUATERNION
        let data_q = AemData {
            comment: vec![],
            attitude_states: vec![valid_q.clone()],
        };
        assert!(data_q.validate(&AttitudeTypeType::EulerAngleUpper).is_err());

        // Check all other variants against a wrong type declaration
        let cases = vec![
            AttitudeTypeType::QuaternionDerivativeUpper,
            AttitudeTypeType::QuaternionAngVelUpper,
            AttitudeTypeType::EulerAngleDerivativeUpper,
            AttitudeTypeType::EulerAngleAngVelUpper,
            AttitudeTypeType::SpinUpper,
            AttitudeTypeType::SpinNutationUpper,
            AttitudeTypeType::SpinNutationMomUpper,
        ];

        for attitude_type in cases {
            let d = AemData {
                comment: vec![],
                attitude_states: vec![valid_q.clone()],
            };
            assert!(
                d.validate(&attitude_type).is_err(),
                "Expected error for type {}",
                attitude_type
            );
        }
    }

    #[test]
    fn test_aem_data_requires_attitude_state() {
        let data = AemData {
            comment: vec![],
            attitude_states: vec![],
        };
        assert!(crate::traits::Validate::validate(&data).is_err());
    }

    #[test]
    fn test_aem_data_state_choice_conflict() {
        use crate::common::*;

        let mut wrapper = AemAttitudeStateWrapper::from(AemAttitudeState::QuaternionEphemeris(
            QuaternionEphemeris {
                epoch: "2023-01-01T00:00:00".parse().unwrap(),
                quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
            },
        ));

        wrapper.euler_angle = Some(EulerAngle {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
        });

        let data = AemData {
            comment: vec![],
            attitude_states: vec![wrapper],
        };
        assert!(crate::traits::Validate::validate(&data).is_err());
    }

    #[test]
    fn test_aem_body_requires_segment() {
        let body = AemBody { segment: vec![] };
        assert!(body.validate().is_err());
    }
}
