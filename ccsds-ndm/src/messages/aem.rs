// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{AdmHeader, AemAttitudeState};
use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
#[cfg(test)]
use crate::traits::Validate;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
}

impl Ndm for Aem {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let normalized = crate::kvn::normalize_line_endings(kvn);
        validate_kvn_syntax(&normalized)?;
        let aem = Self::from_kvn_str(&normalized)?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
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

fn validate_aem_xml_envelope(
    xml: &str,
    options: &crate::options::ParseOptions,
    source_edition: &mut Option<String>,
) -> Result<()> {
    use crate::xml::XmlSequenceRule;
    crate::xml::validate_standalone_document(
        xml,
        b"aem",
        "AEM",
        options,
        source_edition,
        crate::xml::MessageSchema {
            child_rule: |parent: &[u8], child: &[u8]| {
                let children = aem_xml_children(parent)?;
                let rank = children.iter().position(|candidate| *candidate == child)? as u16;
                let repeatable = matches!(child, b"COMMENT" | b"segment" | b"attitudeState");
                Some(XmlSequenceRule::new(rank, repeatable))
            },
            attribute_allowed: |element: &[u8], attribute: &[u8]| {
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
            is_record: |element: &[u8]| element == b"attitudeState",
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
    pub(crate) fn from_xml_with_options(
        xml: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        let mut source_edition = None;
        validate_aem_xml_envelope(xml, options, &mut source_edition)?;
        let aem: Self = crate::xml::from_str_with_context(xml, "AEM")?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
    }

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
        Ok(())
    }
}

fn validate_aem_state_numbers(state: &AemAttitudeState) -> Result<()> {
    let check = |values: &[f64]| -> Result<()> {
        if values
            .iter()
            .all(|value| crate::kvn::ser::OdmFloat::is_valid(*value))
        {
            Ok(())
        } else {
            Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "AEM KVN attitude-state numbers must be representable CCSDS numbers",
                ),
                line: None,
            }
            .into())
        }
    };
    match state {
        AemAttitudeState::QuaternionEphemeris(v) => check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
        ]),
        AemAttitudeState::QuaternionDerivative(v) => check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
            v.quaternion_dot.q1_dot.value,
            v.quaternion_dot.q2_dot.value,
            v.quaternion_dot.q3_dot.value,
            v.quaternion_dot.qc_dot.value,
        ]),
        AemAttitudeState::QuaternionAngVel(v) => check(&[
            v.quaternion.q1,
            v.quaternion.q2,
            v.quaternion.q3,
            v.quaternion.qc,
            v.ang_vel.angvel_x.value,
            v.ang_vel.angvel_y.value,
            v.ang_vel.angvel_z.value,
        ]),
        AemAttitudeState::EulerAngle(v) => {
            check(&[v.angle_1.value, v.angle_2.value, v.angle_3.value])
        }
        AemAttitudeState::EulerAngleDerivative(v) => check(&[
            v.angle_1.value,
            v.angle_2.value,
            v.angle_3.value,
            v.angle_1_dot.value,
            v.angle_2_dot.value,
            v.angle_3_dot.value,
        ]),
        AemAttitudeState::EulerAngleAngVel(v) => check(&[
            v.angle_1.value,
            v.angle_2.value,
            v.angle_3.value,
            v.angvel_x.value,
            v.angvel_y.value,
            v.angvel_z.value,
        ]),
        AemAttitudeState::Spin(v) => check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
        ]),
        AemAttitudeState::SpinNutation(v) => check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
            v.nutation.value,
            v.nutation_per.value,
            v.nutation_phase.value,
        ]),
        AemAttitudeState::SpinNutationMom(v) => check(&[
            v.spin_alpha.value,
            v.spin_delta.value,
            v.spin_angle.value,
            v.spin_angle_vel.value,
            v.momentum_alpha.value,
            v.momentum_delta.value,
            v.nutation_vel.value,
        ]),
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
        if let Some(error) = self.first_cross_segment_error() {
            return Err(error.into());
        }
        Ok(())
    }
}

impl AemBody {
    fn first_cross_segment_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        if let Some(error) = self.first_identity_error() {
            return Some(error);
        }

        for (index, pair) in self.segment.windows(2).enumerate() {
            let (Some(previous), Some(current)) = (
                pair[0].metadata.useable_stop_time,
                pair[1].metadata.useable_start_time,
            ) else {
                continue;
            };
            if previous.into_epoch().cmp_same_branch(&current.into_epoch())
                == Some(Ordering::Greater)
            {
                return Some(
                    ValidationError::InvalidValue {
                        field: "USEABLE_START_TIME".into(),
                        value: current.to_string(),
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
        None
    }

    /// Enforce the single-object rule from 504.0-B-2 section 2.3.1. Figure G-4 uses different
    /// casing for one object's name, while OBJECT_ID remains exact. TIME_SYSTEM is only fixed
    /// within each segment by section 4.2.4.8.2.
    fn first_identity_error(&self) -> Option<ValidationError> {
        let first = self.segment.first()?;
        let object_name = &first.metadata.object_name;
        let object_id = &first.metadata.object_id;
        for (index, segment) in self.segment.iter().enumerate().skip(1) {
            if !segment
                .metadata
                .object_name
                .eq_ignore_ascii_case(object_name)
                || segment.metadata.object_id != *object_id
            {
                return Some(
                    ValidationError::InvalidValue {
                        field: "OBJECT_NAME/OBJECT_ID".into(),
                        value: format!(
                            "{}/{}",
                            segment.metadata.object_name, segment.metadata.object_id
                        ),
                        expected: format!(
                            "one object throughout the AEM (expected {object_name}/{object_id})"
                        )
                        .into(),
                        line: None,
                    }
                    .at_path(format!("segment[{index}].metadata.object_id")),
                );
            }
        }
        None
    }
}

impl crate::traits::Validate for AemSegment {
    fn validate(&self) -> Result<()> {
        self.validate_inner()
    }
}

impl AemSegment {
    pub fn validate(&self) -> Result<()> {
        self.validate_inner()
    }

    fn validate_inner(&self) -> Result<()> {
        self.metadata.validate_inner()?;
        self.data.validate_structure()?;
        self.data
            .validate_attitude_type(&self.metadata.attitude_type)?;
        match self.first_timeline_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn first_timeline_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        let start = self.metadata.start_time.into_epoch();
        let stop = self.metadata.stop_time.into_epoch();
        let mut previous: Option<crate::types::Epoch> = None;
        for (index, state) in self.data.attitude_states.iter().enumerate() {
            let epoch = state.epoch();
            let current = epoch.into_epoch();
            if start.cmp_same_branch(&current) == Some(Ordering::Greater)
                || current.cmp_same_branch(&stop) == Some(Ordering::Greater)
            {
                return Some(
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
                return Some(
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
        None
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
        self.validate_inner()
    }

    fn validate_inner(&self) -> Result<()> {
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
                | AttitudeTypeType::EulerAngleDerivative
                | AttitudeTypeType::EulerAngleAngVel
        );
        let requires_angvel_frame = matches!(
            self.attitude_type,
            AttitudeTypeType::QuaternionAngVel | AttitudeTypeType::EulerAngleAngVel
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

        match self.first_time_span_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn first_time_span_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        let start = self.start_time.into_epoch();
        let stop = self.stop_time.into_epoch();
        if start.cmp_same_branch(&stop) == Some(Ordering::Greater) {
            return Some(ValidationError::InvalidValue {
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
                    return Some(ValidationError::OutOfRange {
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
                return Some(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME/USEABLE_STOP_TIME".into(),
                    value: format!("{useable_start} > {useable_stop}"),
                    expected: "USEABLE_START_TIME no later than USEABLE_STOP_TIME".into(),
                    line: None,
                });
            }
        }
        None
    }
}

impl crate::traits::Validate for AemMetadata {
    fn validate(&self) -> Result<()> {
        self.validate_inner()
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
#[derive(Debug, PartialEq, Clone, bon::Builder)]
pub struct AemData {
    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[builder(default)]
    pub comment: Vec<String>,
    /// Attitude ephemeris data lines.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[builder(default)]
    pub attitude_states: Vec<AemAttitudeState>,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
struct AemDataXml {
    #[serde(default)]
    comment: Vec<String>,
    #[serde(rename = "attitudeState")]
    attitude_states: Vec<AemAttitudeStateXml>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AemAttitudeStateXml {
    #[serde(rename = "quaternionEphemeris", default)]
    quaternion_ephemeris: Option<crate::common::QuaternionEphemeris>,
    #[serde(rename = "quaternionDerivative", default)]
    quaternion_derivative: Option<crate::common::QuaternionDerivative>,
    #[serde(rename = "quaternionAngVel", default)]
    quaternion_ang_vel: Option<crate::common::QuaternionAngVel>,
    #[serde(rename = "eulerAngle", default)]
    euler_angle: Option<crate::common::EulerAngle>,
    #[serde(rename = "eulerAngleDerivative", default)]
    euler_angle_derivative: Option<crate::common::EulerAngleDerivative>,
    #[serde(rename = "eulerAngleAngVel", default)]
    euler_angle_ang_vel: Option<crate::common::EulerAngleAngVel>,
    #[serde(rename = "spin", default)]
    spin: Option<crate::common::Spin>,
    #[serde(rename = "spinNutation", default)]
    spin_nutation: Option<crate::common::SpinNutation>,
    #[serde(rename = "spinNutationMom", default)]
    spin_nutation_mom: Option<crate::common::SpinNutationMom>,
}

impl AemAttitudeStateXml {
    fn into_state(self) -> std::result::Result<AemAttitudeState, &'static str> {
        let count = [
            self.quaternion_ephemeris.is_some(),
            self.quaternion_derivative.is_some(),
            self.quaternion_ang_vel.is_some(),
            self.euler_angle.is_some(),
            self.euler_angle_derivative.is_some(),
            self.euler_angle_ang_vel.is_some(),
            self.spin.is_some(),
            self.spin_nutation.is_some(),
            self.spin_nutation_mom.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count();
        if count != 1 {
            return Err("attitudeState requires exactly one attitude choice");
        }
        Ok(if let Some(value) = self.quaternion_ephemeris {
            AemAttitudeState::QuaternionEphemeris(value)
        } else if let Some(value) = self.quaternion_derivative {
            AemAttitudeState::QuaternionDerivative(value)
        } else if let Some(value) = self.quaternion_ang_vel {
            AemAttitudeState::QuaternionAngVel(value)
        } else if let Some(value) = self.euler_angle {
            AemAttitudeState::EulerAngle(value)
        } else if let Some(value) = self.euler_angle_derivative {
            AemAttitudeState::EulerAngleDerivative(value)
        } else if let Some(value) = self.euler_angle_ang_vel {
            AemAttitudeState::EulerAngleAngVel(value)
        } else if let Some(value) = self.spin {
            AemAttitudeState::Spin(value)
        } else if let Some(value) = self.spin_nutation {
            AemAttitudeState::SpinNutation(value)
        } else {
            AemAttitudeState::SpinNutationMom(self.spin_nutation_mom.unwrap())
        })
    }
}

impl<'de> Deserialize<'de> for AemData {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        use serde::de::Error;
        let value = AemDataXml::deserialize(deserializer)?;
        Ok(Self {
            comment: value.comment,
            attitude_states: value
                .attitude_states
                .into_iter()
                .map(|state| state.into_state().map_err(D::Error::custom))
                .collect::<std::result::Result<_, _>>()?,
        })
    }
}

impl Serialize for AemData {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AemData", 2)?;
        if !self.comment.is_empty() {
            state.serialize_field("COMMENT", &self.comment)?;
        }
        state.serialize_field(
            "attitudeState",
            &AemAttitudeStatesXml(&self.attitude_states),
        )?;
        state.end()
    }
}

struct AemAttitudeStatesXml<'a>(&'a [AemAttitudeState]);

impl Serialize for AemAttitudeStatesXml<'_> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.iter().map(AemAttitudeStateXmlRef))
    }
}

struct AemAttitudeStateXmlRef<'a>(&'a AemAttitudeState);

impl Serialize for AemAttitudeStateXmlRef<'_> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("attitudeState", 1)?;
        match self.0 {
            AemAttitudeState::QuaternionEphemeris(value) => {
                state.serialize_field("quaternionEphemeris", value)?
            }
            AemAttitudeState::QuaternionDerivative(value) => {
                state.serialize_field("quaternionDerivative", value)?
            }
            AemAttitudeState::QuaternionAngVel(value) => {
                state.serialize_field("quaternionAngVel", value)?
            }
            AemAttitudeState::EulerAngle(value) => state.serialize_field("eulerAngle", value)?,
            AemAttitudeState::EulerAngleDerivative(value) => {
                state.serialize_field("eulerAngleDerivative", value)?
            }
            AemAttitudeState::EulerAngleAngVel(value) => {
                state.serialize_field("eulerAngleAngVel", value)?
            }
            AemAttitudeState::Spin(value) => state.serialize_field("spin", value)?,
            AemAttitudeState::SpinNutation(value) => {
                state.serialize_field("spinNutation", value)?
            }
            AemAttitudeState::SpinNutationMom(value) => {
                state.serialize_field("spinNutationMom", value)?
            }
        }
        state.end()
    }
}

impl AemAttitudeState {
    fn epoch(&self) -> &CalendarEpoch {
        match self {
            Self::QuaternionEphemeris(v) => &v.epoch,
            Self::QuaternionDerivative(v) => &v.epoch,
            Self::QuaternionAngVel(v) => &v.epoch,
            Self::EulerAngle(v) => &v.epoch,
            Self::EulerAngleDerivative(v) => &v.epoch,
            Self::EulerAngleAngVel(v) => &v.epoch,
            Self::Spin(v) => &v.epoch,
            Self::SpinNutation(v) => &v.epoch,
            Self::SpinNutationMom(v) => &v.epoch,
        }
    }

    fn matches_type(&self, attitude_type: &AttitudeTypeType) -> bool {
        matches!(
            (self, attitude_type),
            (Self::QuaternionEphemeris(_), AttitudeTypeType::Quaternion)
                | (
                    Self::QuaternionDerivative(_),
                    AttitudeTypeType::QuaternionDerivative
                )
                | (
                    Self::QuaternionAngVel(_),
                    AttitudeTypeType::QuaternionAngVel
                )
                | (Self::EulerAngle(_), AttitudeTypeType::EulerAngle)
                | (
                    Self::EulerAngleDerivative(_),
                    AttitudeTypeType::EulerAngleDerivative
                )
                | (
                    Self::EulerAngleAngVel(_),
                    AttitudeTypeType::EulerAngleAngVel
                )
                | (Self::Spin(_), AttitudeTypeType::Spin)
                | (Self::SpinNutation(_), AttitudeTypeType::SpinNutation)
                | (Self::SpinNutationMom(_), AttitudeTypeType::SpinNutationMom)
        )
    }

    fn validate_state(&self) -> Result<()> {
        let angles = |values: &[(&'static str, f64)]| -> Result<()> {
            for (name, value) in values {
                Angle::validate_value(*value, name)?;
            }
            Ok(())
        };
        let finite = |values: &[(&'static str, f64)]| -> Result<()> {
            for (name, value) in values {
                if !value.is_finite() {
                    return Err(ValidationError::InvalidValue {
                        field: (*name).into(),
                        value: value.to_string(),
                        expected: "a finite number".into(),
                        line: None,
                    }
                    .into());
                }
            }
            Ok(())
        };

        match self {
            Self::QuaternionEphemeris(value) => {
                crate::traits::Validate::validate(&value.quaternion)
            }
            Self::QuaternionDerivative(value) => {
                crate::traits::Validate::validate(&value.quaternion)?;
                finite(&[
                    ("Q1_DOT", value.quaternion_dot.q1_dot.value),
                    ("Q2_DOT", value.quaternion_dot.q2_dot.value),
                    ("Q3_DOT", value.quaternion_dot.q3_dot.value),
                    ("QC_DOT", value.quaternion_dot.qc_dot.value),
                ])
            }
            Self::QuaternionAngVel(value) => {
                crate::traits::Validate::validate(&value.quaternion)?;
                finite(&[
                    ("ANGVEL_X", value.ang_vel.angvel_x.value),
                    ("ANGVEL_Y", value.ang_vel.angvel_y.value),
                    ("ANGVEL_Z", value.ang_vel.angvel_z.value),
                ])
            }
            Self::EulerAngle(value) => angles(&[
                ("ANGLE_1", value.angle_1.value),
                ("ANGLE_2", value.angle_2.value),
                ("ANGLE_3", value.angle_3.value),
            ]),
            Self::EulerAngleDerivative(value) => {
                angles(&[
                    ("ANGLE_1", value.angle_1.value),
                    ("ANGLE_2", value.angle_2.value),
                    ("ANGLE_3", value.angle_3.value),
                ])?;
                finite(&[
                    ("ANGLE_1_DOT", value.angle_1_dot.value),
                    ("ANGLE_2_DOT", value.angle_2_dot.value),
                    ("ANGLE_3_DOT", value.angle_3_dot.value),
                ])
            }
            Self::EulerAngleAngVel(value) => {
                angles(&[
                    ("ANGLE_1", value.angle_1.value),
                    ("ANGLE_2", value.angle_2.value),
                    ("ANGLE_3", value.angle_3.value),
                ])?;
                finite(&[
                    ("ANGVEL_X", value.angvel_x.value),
                    ("ANGVEL_Y", value.angvel_y.value),
                    ("ANGVEL_Z", value.angvel_z.value),
                ])
            }
            Self::Spin(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                ])?;
                finite(&[("SPIN_ANGLE_VEL", value.spin_angle_vel.value)])
            }
            Self::SpinNutation(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                    ("NUTATION", value.nutation.value),
                    ("NUTATION_PHASE", value.nutation_phase.value),
                ])?;
                finite(&[("SPIN_ANGLE_VEL", value.spin_angle_vel.value)])?;
                Duration::validate_value(value.nutation_per.value, "NUTATION_PER")
            }
            Self::SpinNutationMom(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                    ("MOMENTUM_ALPHA", value.momentum_alpha.value),
                    ("MOMENTUM_DELTA", value.momentum_delta.value),
                ])?;
                finite(&[
                    ("SPIN_ANGLE_VEL", value.spin_angle_vel.value),
                    ("NUTATION_VEL", value.nutation_vel.value),
                ])
            }
        }
    }
}

impl crate::traits::Validate for AemData {
    fn validate(&self) -> Result<()> {
        self.validate_structure()
    }
}

impl AemData {
    fn validate_structure(&self) -> Result<()> {
        if self.attitude_states.is_empty() {
            return Err(ValidationError::missing_required(
                "AEM Data",
                "attitudeState (at least one required)",
            )
            .into());
        }
        for state in &self.attitude_states {
            state.validate_state()?;
        }
        Ok(())
    }

    fn validate_attitude_type(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
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

    pub fn validate(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
        self.validate_structure()?;
        self.validate_attitude_type(attitude_type)
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

    #[test]
    fn test_aem_data_validation_mismatches() {
        use crate::common::*;

        let valid_q = AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        });

        let valid_euler = AemAttitudeState::EulerAngle(EulerAngle {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
        });

        // Type mismatch: Expects QUATERNION, gets EULER_ANGLE
        let data = AemData {
            comment: vec![],
            attitude_states: vec![valid_euler.clone()],
        };
        assert!(data.validate(&AttitudeTypeType::Quaternion).is_err());

        // Type mismatch: Expects EULER_ANGLE, gets QUATERNION
        let data_q = AemData {
            comment: vec![],
            attitude_states: vec![valid_q.clone()],
        };
        assert!(data_q.validate(&AttitudeTypeType::EulerAngle).is_err());

        // Check all other variants against a wrong type declaration
        let cases = vec![
            AttitudeTypeType::QuaternionDerivative,
            AttitudeTypeType::QuaternionAngVel,
            AttitudeTypeType::EulerAngleDerivative,
            AttitudeTypeType::EulerAngleAngVel,
            AttitudeTypeType::Spin,
            AttitudeTypeType::SpinNutation,
            AttitudeTypeType::SpinNutationMom,
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
    fn test_aem_body_requires_segment() {
        let body = AemBody { segment: vec![] };
        assert!(body.validate().is_err());
    }
}
