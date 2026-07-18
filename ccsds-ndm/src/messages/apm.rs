// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AdmHeader, AngVelState, AttManeuverState, EulerAngleState, InertiaState, QuaternionState,
    SpinState,
};

use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
#[cfg(test)]
use crate::traits::Validate;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};

/// Attitude Parameter Message (APM).
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message
/// is suited to interagency exchanges that involve automated interaction and/or human
/// interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.
///
/// The APM requires the use of a propagation technique to determine the attitude state at
/// times different from the specified epoch.
///
/// **CCSDS Reference**: 504.0-B-2, Section 3.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "apm")]
pub struct Apm {
    pub header: AdmHeader,
    pub body: ApmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_APM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "2.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Apm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Apm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        crate::validation::collect_message_validation_errors(
            crate::validation::MessageKind::Apm,
            &self.id,
            &self.version,
            &self.header,
            &self.body,
        )
    }
}

impl Ndm for Apm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Apm,
            &self.version,
            crate::generation::OutputFormat::Kvn,
            self,
        )?;
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        validate_kvn_syntax(kvn)?;
        let apm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&apm)?;
        Ok(apm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::validate_for_generation(
            crate::validation::MessageKind::Apm,
            &self.version,
            crate::generation::OutputFormat::Xml,
            self,
        )?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"apm", "APM")?;
        validate_xml_sequences(xml)?;
        let apm: Self = crate::xml::from_str_with_context(xml, "APM")?;
        crate::traits::Validate::validate(&apm)?;
        Ok(apm)
    }
}

fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    let rule = |rank, repeatable| XmlSequenceRule { rank, repeatable };
    crate::xml::validate_element_sequences(
        xml,
        "APM",
        |parent, child| {
            Some(match (parent, child) {
                (b"apm", b"header") => rule(0, false),
                (b"apm", b"body") => rule(1, false),
                (b"header", b"COMMENT") => rule(0, true),
                (b"header", b"CLASSIFICATION") => rule(1, false),
                (b"header", b"CREATION_DATE") => rule(2, false),
                (b"header", b"ORIGINATOR") => rule(3, false),
                (b"header", b"MESSAGE_ID") => rule(4, false),
                (b"body", b"segment") => rule(0, false),
                (b"segment", b"metadata") => rule(0, false),
                (b"segment", b"data") => rule(1, false),
                (b"metadata", b"COMMENT") => rule(0, true),
                (b"metadata", b"OBJECT_NAME") => rule(1, false),
                (b"metadata", b"OBJECT_ID") => rule(2, false),
                (b"metadata", b"CENTER_NAME") => rule(3, false),
                (b"metadata", b"TIME_SYSTEM") => rule(4, false),
                (b"data", b"COMMENT") => rule(0, true),
                (b"data", b"EPOCH") => rule(1, false),
                (b"data", b"quaternionState") => rule(2, true),
                (b"data", b"eulerAngleState") => rule(3, true),
                (b"data", b"angularVelocity") => rule(4, true),
                (b"data", b"spin") => rule(5, true),
                (b"data", b"inertia") => rule(6, true),
                (b"data", b"maneuverParameters") => rule(7, true),
                (b"quaternionState", b"COMMENT") => rule(0, true),
                (b"quaternionState", b"REF_FRAME_A") => rule(1, false),
                (b"quaternionState", b"REF_FRAME_B") => rule(2, false),
                (b"quaternionState", b"quaternion") => rule(3, false),
                (b"quaternionState", b"quaternionDot") => rule(4, false),
                (b"quaternion", b"Q1") => rule(0, false),
                (b"quaternion", b"Q2") => rule(1, false),
                (b"quaternion", b"Q3") => rule(2, false),
                (b"quaternion", b"QC") => rule(3, false),
                (b"quaternionDot", b"Q1_DOT") => rule(0, false),
                (b"quaternionDot", b"Q2_DOT") => rule(1, false),
                (b"quaternionDot", b"Q3_DOT") => rule(2, false),
                (b"quaternionDot", b"QC_DOT") => rule(3, false),
                (b"eulerAngleState", b"COMMENT") => rule(0, true),
                (b"eulerAngleState", b"REF_FRAME_A") => rule(1, false),
                (b"eulerAngleState", b"REF_FRAME_B") => rule(2, false),
                (b"eulerAngleState", b"EULER_ROT_SEQ") => rule(3, false),
                (b"eulerAngleState", b"ANGLE_1") => rule(4, false),
                (b"eulerAngleState", b"ANGLE_2") => rule(5, false),
                (b"eulerAngleState", b"ANGLE_3") => rule(6, false),
                (b"eulerAngleState", b"ANGLE_1_DOT") => rule(7, false),
                (b"eulerAngleState", b"ANGLE_2_DOT") => rule(8, false),
                (b"eulerAngleState", b"ANGLE_3_DOT") => rule(9, false),
                (b"angularVelocity", b"COMMENT") => rule(0, true),
                (b"angularVelocity", b"REF_FRAME_A") => rule(1, false),
                (b"angularVelocity", b"REF_FRAME_B") => rule(2, false),
                (b"angularVelocity", b"ANGVEL_FRAME") => rule(3, false),
                (b"angularVelocity", b"ANGVEL_X") => rule(4, false),
                (b"angularVelocity", b"ANGVEL_Y") => rule(5, false),
                (b"angularVelocity", b"ANGVEL_Z") => rule(6, false),
                (b"spin", b"COMMENT") => rule(0, true),
                (b"spin", b"REF_FRAME_A") => rule(1, false),
                (b"spin", b"REF_FRAME_B") => rule(2, false),
                (b"spin", b"SPIN_ALPHA") => rule(3, false),
                (b"spin", b"SPIN_DELTA") => rule(4, false),
                (b"spin", b"SPIN_ANGLE") => rule(5, false),
                (b"spin", b"SPIN_ANGLE_VEL") => rule(6, false),
                (b"spin", b"NUTATION") => rule(7, false),
                (b"spin", b"NUTATION_PER") => rule(8, false),
                (b"spin", b"NUTATION_PHASE") => rule(9, false),
                (b"spin", b"MOMENTUM_ALPHA") => rule(10, false),
                (b"spin", b"MOMENTUM_DELTA") => rule(11, false),
                (b"spin", b"NUTATION_VEL") => rule(12, false),
                (b"inertia", b"COMMENT") => rule(0, true),
                (b"inertia", b"INERTIA_REF_FRAME") => rule(1, false),
                (b"inertia", b"IXX") => rule(2, false),
                (b"inertia", b"IYY") => rule(3, false),
                (b"inertia", b"IZZ") => rule(4, false),
                (b"inertia", b"IXY") => rule(5, false),
                (b"inertia", b"IXZ") => rule(6, false),
                (b"inertia", b"IYZ") => rule(7, false),
                (b"maneuverParameters", b"COMMENT") => rule(0, true),
                (b"maneuverParameters", b"MAN_EPOCH_START") => rule(1, false),
                (b"maneuverParameters", b"MAN_DURATION") => rule(2, false),
                (b"maneuverParameters", b"MAN_REF_FRAME") => rule(3, false),
                (b"maneuverParameters", b"MAN_TOR_X") => rule(4, false),
                (b"maneuverParameters", b"MAN_TOR_Y") => rule(5, false),
                (b"maneuverParameters", b"MAN_TOR_Z") => rule(6, false),
                (b"maneuverParameters", b"MAN_DELTA_MASS") => rule(7, false),
                _ => return None,
            })
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
                        | b"IXX"
                        | b"IYY"
                        | b"IZZ"
                        | b"IXY"
                        | b"IXZ"
                        | b"IYZ"
                        | b"MAN_DURATION"
                        | b"MAN_TOR_X"
                        | b"MAN_TOR_Y"
                        | b"MAN_TOR_Z"
                        | b"MAN_DELTA_MASS"
                )
        },
    )
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    fn top_rank(key: &str) -> Option<u16> {
        Some(match key {
            "CCSDS_APM_VERS" => 0,
            "CLASSIFICATION" => 1,
            "CREATION_DATE" => 2,
            "ORIGINATOR" => 3,
            "MESSAGE_ID" => 4,
            "OBJECT_NAME" => 10,
            "OBJECT_ID" => 11,
            "CENTER_NAME" => 12,
            "TIME_SYSTEM" => 13,
            "EPOCH" => 20,
            _ => return None,
        })
    }

    fn block_rank(block: &str, key: &str) -> Option<u16> {
        let keys: &[&str] = match block {
            "META" => &["OBJECT_NAME", "OBJECT_ID", "CENTER_NAME", "TIME_SYSTEM"],
            "QUAT" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "Q1",
                "Q2",
                "Q3",
                "QC",
                "Q1_DOT",
                "Q2_DOT",
                "Q3_DOT",
                "QC_DOT",
            ],
            "EULER" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "EULER_ROT_SEQ",
                "ANGLE_1",
                "ANGLE_2",
                "ANGLE_3",
                "ANGLE_1_DOT",
                "ANGLE_2_DOT",
                "ANGLE_3_DOT",
            ],
            "ANGVEL" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "ANGVEL_FRAME",
                "ANGVEL_X",
                "ANGVEL_Y",
                "ANGVEL_Z",
            ],
            "SPIN" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "SPIN_ALPHA",
                "SPIN_DELTA",
                "SPIN_ANGLE",
                "SPIN_ANGLE_VEL",
                "NUTATION",
                "NUTATION_PER",
                "NUTATION_PHASE",
                "MOMENTUM_ALPHA",
                "MOMENTUM_DELTA",
                "NUTATION_VEL",
            ],
            "INERTIA" => &[
                "INERTIA_REF_FRAME",
                "IXX",
                "IYY",
                "IZZ",
                "IXY",
                "IXZ",
                "IYZ",
            ],
            "MAN" => &[
                "MAN_EPOCH_START",
                "MAN_DURATION",
                "MAN_REF_FRAME",
                "MAN_TOR_X",
                "MAN_TOR_Y",
                "MAN_TOR_Z",
                "MAN_DELTA_MASS",
            ],
            _ => return None,
        };
        keys.iter()
            .position(|candidate| *candidate == key)
            .map(|rank| rank as u16)
    }

    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating APM KVN structure"],
            offset,
        }))))
    };
    let mut current_block: Option<&str> = None;
    let mut top_previous = None;
    let mut block_previous = None;
    let mut pending_comment = false;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(line_number, offset, message.into()));
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
            if current_block.is_some() && block_previous.is_some() {
                return fail("COMMENT is not at the beginning of a logical block");
            }
            if current_block.is_none() && !matches!(top_previous, Some(0 | 3 | 4 | 13)) {
                return fail("COMMENT is not at the beginning of a logical block");
            }
            pending_comment = true;
            offset += raw_line.len() + 1;
            continue;
        }

        if let Some(marker) = line.strip_suffix("_START") {
            if current_block.is_some()
                || !matches!(
                    marker,
                    "META" | "QUAT" | "EULER" | "ANGVEL" | "SPIN" | "INERTIA" | "MAN"
                )
            {
                return fail("unknown or nested APM logical-block start");
            }
            if pending_comment {
                return fail("COMMENT must follow, not precede, a logical-block start");
            }
            if marker == "META" {
                if !matches!(top_previous, Some(3 | 4)) {
                    return fail("META_START is out of order");
                }
            } else if top_previous != Some(20) {
                return fail("attitude logical block must follow EPOCH");
            }
            current_block = Some(marker);
            block_previous = None;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP") {
            if current_block != Some(marker) {
                return fail("mismatched APM logical-block end");
            }
            if pending_comment {
                return fail("trailing COMMENT has no logical block content");
            }
            if marker == "META" {
                top_previous = Some(13);
            }
            current_block = None;
            block_previous = None;
            offset += raw_line.len() + 1;
            continue;
        }

        if !line.contains('=') {
            return fail("expected an assignment or logical-block delimiter");
        }
        let key = line
            .split_once('=')
            .expect("assignment count checked")
            .0
            .trim();
        if let Some(block) = current_block {
            let rank = block_rank(block, key)
                .ok_or_else(|| invalid(line_number, offset, "unknown APM block keyword".into()))?;
            if block_previous.is_some_and(|previous| rank <= previous) {
                return fail("duplicate or out-of-order APM block keyword");
            }
            block_previous = Some(rank);
        } else {
            let rank = top_rank(key)
                .ok_or_else(|| invalid(line_number, offset, "unknown APM keyword".into()))?;
            if pending_comment {
                let starts_block = match key {
                    "CLASSIFICATION" | "CREATION_DATE" => top_previous == Some(0),
                    "OBJECT_NAME" => matches!(top_previous, Some(3 | 4)),
                    "EPOCH" => top_previous == Some(13),
                    _ => false,
                };
                if !starts_block {
                    return fail("COMMENT is not at the beginning of a logical block");
                }
            }
            if top_previous.is_some_and(|previous| rank <= previous) {
                return fail("duplicate or out-of-order APM keyword");
            }
            top_previous = Some(rank);
        }
        pending_comment = false;
        offset += raw_line.len() + 1;
    }

    if current_block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unterminated APM logical block".into(),
        ));
    }
    if pending_comment {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "trailing COMMENT has no logical block".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Apm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_APM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct ApmBody {
    // XSD says minOccurs=1 maxOccurs=1 for APM segment!
    #[serde(rename = "segment")]
    pub segment: ApmSegment,
}

impl crate::traits::Validate for ApmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        self.segment.validation_errors()
    }
}

impl ToKvn for ApmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct ApmSegment {
    pub metadata: ApmMetadata,
    pub data: ApmData,
}

impl crate::traits::Validate for ApmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let mut errors = self.metadata.validation_errors()?;
        errors.extend(self.data.validation_errors()?);
        Ok(errors)
    }
}

impl ApmSegment {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

impl ToKvn for ApmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_line("META_START");
        self.metadata.write_kvn(writer);
        writer.write_line("META_STOP");
        writer.write_line("");
        self.data.write_kvn(writer);
    }
}

/// APM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct ApmMetadata {
    /// Comments (allowed only at the beginning of the APM Metadata before OBJECT_NAME). Each
    /// comment line shall begin with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1, MARS PATHFINDER, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// letter for the identification of the part brought into space by the launch. In cases in
    /// which the asset is not listed in reference [ADM-2], the UN Office of Outer Space Affairs
    /// designator index format is not used, or the content cannot be disclosed, the value should
    /// be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH, BARYCENTER, MOON
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Time system used for attitude and maneuver data. The set of allowed values is described in
    /// annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub time_system: String,
}

impl crate::traits::Validate for ApmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        Ok(crate::validation::missing_required_fields(
            "APM Metadata",
            [
                ("OBJECT_NAME", self.object_name.trim().is_empty()),
                ("OBJECT_ID", self.object_id.trim().is_empty()),
                ("TIME_SYSTEM", self.time_system.trim().is_empty()),
            ],
        ))
    }
}

impl ToKvn for ApmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
    }
}

/// APM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct ApmData {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub epoch: CalendarEpoch,
    /// Attitude quaternion. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "quaternionState",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub quaternion_state: Vec<QuaternionState>,
    /// Euler angle elements. All mandatory elements of the logical block are to be provided if the
    /// block is present. (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "eulerAngleState",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub euler_angle_state: Vec<EulerAngleState>,
    /// Angular velocity vector. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "angularVelocity",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub angular_velocity: Vec<AngVelState>,
    /// Spin. All mandatory elements are to be provided if the block is present. (See annex F for
    /// conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "spin", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub spin: Vec<SpinState>,
    /// Inertia. All mandatory elements are to be provided if the block is present. (See annex F
    /// for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "inertia", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub inertia: Vec<InertiaState>,
    /// Maneuver Parameters. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "maneuverParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub maneuver_parameters: Vec<AttManeuverState>,
}

impl crate::traits::Validate for ApmData {
    fn validate(&self) -> Result<()> {
        if self.quaternion_state.is_empty()
            && self.euler_angle_state.is_empty()
            && self.angular_velocity.is_empty()
            && self.spin.is_empty()
            && self.inertia.is_empty()
            && self.maneuver_parameters.is_empty()
        {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Data".into(),
                field: "At least one logical block".into(),
                line: None,
            }
            .into());
        }
        for block in &self.quaternion_state {
            block.quaternion.validate()?;
        }
        for block in &self.spin {
            block.validate()?;
        }
        Ok(())
    }

    fn validation_errors(&self) -> Result<Vec<ValidationError>> {
        let mut errors = crate::validation::missing_required_fields(
            "APM Data",
            [(
                "At least one logical block",
                self.quaternion_state.is_empty()
                    && self.euler_angle_state.is_empty()
                    && self.angular_velocity.is_empty()
                    && self.spin.is_empty()
                    && self.inertia.is_empty()
                    && self.maneuver_parameters.is_empty(),
            )],
        );
        for block in &self.quaternion_state {
            errors.extend(block.quaternion.validation_errors()?);
        }
        for block in &self.spin {
            errors.extend(block.validation_errors()?);
        }
        Ok(errors)
    }
}

impl ApmData {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

impl ToKvn for ApmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        for block in &self.quaternion_state {
            writer.write_line("QUAT_START");
            block.write_kvn(writer);
            writer.write_line("QUAT_STOP");
            writer.write_line("");
        }
        for block in &self.euler_angle_state {
            writer.write_line("EULER_START");
            block.write_kvn(writer);
            writer.write_line("EULER_STOP");
            writer.write_line("");
        }
        for block in &self.angular_velocity {
            writer.write_line("ANGVEL_START");
            block.write_kvn(writer);
            writer.write_line("ANGVEL_STOP");
            writer.write_line("");
        }
        for block in &self.spin {
            writer.write_line("SPIN_START");
            block.write_kvn(writer);
            writer.write_line("SPIN_STOP");
            writer.write_line("");
        }
        for block in &self.inertia {
            writer.write_line("INERTIA_START");
            block.write_kvn(writer);
            writer.write_line("INERTIA_STOP");
            writer.write_line("");
        }
        for man in &self.maneuver_parameters {
            writer.write_line("MAN_START");
            man.write_kvn(writer);
            writer.write_line("MAN_STOP");
            writer.write_line("");
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_apm_kvn() -> String {
        r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2002-11-04T17:22:31
QUAT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
Q1 = 0.5
Q2 = 0.5
Q3 = 0.5
QC = 0.5
QUAT_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_apm_success() {
        let kvn = sample_apm_kvn();
        let apm = Apm::from_kvn(&kvn).expect("APM parse failed");

        assert_eq!(apm.version, "2.0");
        assert_eq!(
            apm.body.segment.metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
        assert_eq!(apm.body.segment.data.quaternion_state[0].quaternion.q1, 0.5);
    }

    #[test]
    fn test_apm_validation_empty_data() {
        let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
"#;
        // Should fail because there are no data blocks
        let res = Apm::from_kvn(kvn);
        assert!(res.is_err());
    }

    #[test]
    fn test_apm_missing_mandatory_metadata() {
        let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
"#;
        // Missing OBJECT_NAME
        assert!(Apm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_apm_multiple_blocks() {
        let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
EULER_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
EULER_ROT_SEQ = XYZ
ANGLE_1 = 10 [deg]
ANGLE_2 = 20 [deg]
ANGLE_3 = 30 [deg]
EULER_STOP
"#;
        let apm = Apm::from_kvn(kvn).unwrap();
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
        assert_eq!(apm.body.segment.data.euler_angle_state.len(), 1);
    }
    #[test]
    fn test_apm_validation_single_blocks() {
        // Test that having just one block is sufficient
        let mut apm = Apm::from_kvn(&sample_apm_kvn()).unwrap();

        // Clear all blocks
        apm.body.segment.data.quaternion_state.clear();
        assert!(apm.validate().is_err()); // Now empty

        // Add just Inertia
        apm.body.segment.data.inertia.push(InertiaState {
            comment: vec![],
            inertia_ref_frame: "SC_BODY".to_string(),
            ixx: crate::types::Moment::new(1.0, None),
            iyy: crate::types::Moment::new(2.0, None),
            izz: crate::types::Moment::new(3.0, None),
            ixy: crate::types::Moment::new(0.0, None),
            ixz: crate::types::Moment::new(0.0, None),
            iyz: crate::types::Moment::new(0.0, None),
        });
        assert!(apm.validate().is_ok());

        // Clear and add just Angular Velocity
        apm.body.segment.data.inertia.clear();
        apm.body.segment.data.angular_velocity.push(AngVelState {
            comment: vec![],
            ref_frame_a: "GCRF".to_string(),
            ref_frame_b: "SC_BODY".to_string(),
            angvel_frame: crate::types::AngVelFrameType("SC_BODY".to_string()),
            angvel_x: crate::types::AngleRate::new(0.1, None),
            angvel_y: crate::types::AngleRate::new(0.1, None),
            angvel_z: crate::types::AngleRate::new(0.1, None),
        });
        assert!(apm.validate().is_ok());
    }
}
