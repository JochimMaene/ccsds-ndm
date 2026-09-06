// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdParameters;
use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//----------------------------------------------------------------------
// Root CDM Structure
//----------------------------------------------------------------------

/// Conjunction Data Message (CDM).
///
/// The CDM contains information about a single conjunction between a primary object (Object1)
/// and a secondary object (Object2). It allows satellite operators to evaluate the risk of
/// collision and plan avoidance maneuvers.
///
/// The message includes:
/// - Positions and velocities of both objects at Time of Closest Approach (TCA).
/// - Covariance matrices for both objects at TCA.
/// - Relative position and velocity of Object2 with respect to Object1.
/// - Metadata describing how the data was determined (orbit determination settings).
///
/// **CCSDS Reference**: 508.0-B-1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "cdm")]
pub struct Cdm {
    pub header: CdmHeader,
    pub body: CdmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_CDM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "1.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Cdm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Cdm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Cdm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        validate_kvn_syntax(kvn)?;
        let cdm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&cdm)?;
        Ok(cdm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"cdm", "CDM")?;
        validate_xml_sequences(xml)?;
        let cdm: Self = crate::xml::from_str_with_context(xml, "CDM")?;
        crate::traits::Validate::validate(&cdm)?;
        Ok(cdm)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    crate::xml::validate_element_sequences(
        xml,
        "CDM",
        |parent, child| {
            let children = cdm_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable = child == b"COMMENT" || (parent == b"body" && child == b"segment");
            Some(XmlSequenceRule::new(rank, repeatable))
        },
        |element, attribute| {
            attribute == b"units"
                && matches!(
                    element,
                    b"MISS_DISTANCE"
                        | b"RELATIVE_SPEED"
                        | b"RELATIVE_POSITION_R"
                        | b"RELATIVE_POSITION_T"
                        | b"RELATIVE_POSITION_N"
                        | b"RELATIVE_VELOCITY_R"
                        | b"RELATIVE_VELOCITY_T"
                        | b"RELATIVE_VELOCITY_N"
                        | b"SCREEN_VOLUME_X"
                        | b"SCREEN_VOLUME_Y"
                        | b"SCREEN_VOLUME_Z"
                        | b"COLLISION_PROBABILITY"
                        | b"RECOMMENDED_OD_SPAN"
                        | b"ACTUAL_OD_SPAN"
                        | b"RESIDUALS_ACCEPTED"
                        | b"WEIGHTED_RMS"
                        | b"AREA_PC"
                        | b"AREA_DRG"
                        | b"AREA_SRP"
                        | b"MASS"
                        | b"CD_AREA_OVER_MASS"
                        | b"CR_AREA_OVER_MASS"
                        | b"THRUST_ACCELERATION"
                        | b"SEDR"
                        | b"X"
                        | b"Y"
                        | b"Z"
                        | b"X_DOT"
                        | b"Y_DOT"
                        | b"Z_DOT"
                        | b"CR_R"
                        | b"CT_R"
                        | b"CT_T"
                        | b"CN_R"
                        | b"CN_T"
                        | b"CN_N"
                        | b"CRDOT_R"
                        | b"CRDOT_T"
                        | b"CRDOT_N"
                        | b"CRDOT_RDOT"
                        | b"CTDOT_R"
                        | b"CTDOT_T"
                        | b"CTDOT_N"
                        | b"CTDOT_RDOT"
                        | b"CTDOT_TDOT"
                        | b"CNDOT_R"
                        | b"CNDOT_T"
                        | b"CNDOT_N"
                        | b"CNDOT_RDOT"
                        | b"CNDOT_TDOT"
                        | b"CNDOT_NDOT"
                        | b"CDRG_R"
                        | b"CDRG_T"
                        | b"CDRG_N"
                        | b"CDRG_RDOT"
                        | b"CDRG_TDOT"
                        | b"CDRG_NDOT"
                        | b"CDRG_DRG"
                        | b"CSRP_R"
                        | b"CSRP_T"
                        | b"CSRP_N"
                        | b"CSRP_RDOT"
                        | b"CSRP_TDOT"
                        | b"CSRP_NDOT"
                        | b"CSRP_DRG"
                        | b"CSRP_SRP"
                        | b"CTHR_R"
                        | b"CTHR_T"
                        | b"CTHR_N"
                        | b"CTHR_RDOT"
                        | b"CTHR_TDOT"
                        | b"CTHR_NDOT"
                        | b"CTHR_DRG"
                        | b"CTHR_SRP"
                        | b"CTHR_THR"
                )
        },
    )
}

fn cdm_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"cdm" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_FOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"relativeMetadataData", b"segment"],
        b"relativeMetadataData" => &[
            b"COMMENT",
            b"TCA",
            b"MISS_DISTANCE",
            b"RELATIVE_SPEED",
            b"relativeStateVector",
            b"START_SCREEN_PERIOD",
            b"STOP_SCREEN_PERIOD",
            b"SCREEN_VOLUME_FRAME",
            b"SCREEN_VOLUME_SHAPE",
            b"SCREEN_VOLUME_X",
            b"SCREEN_VOLUME_Y",
            b"SCREEN_VOLUME_Z",
            b"SCREEN_ENTRY_TIME",
            b"SCREEN_EXIT_TIME",
            b"COLLISION_PROBABILITY",
            b"COLLISION_PROBABILITY_METHOD",
        ],
        b"relativeStateVector" => &[
            b"RELATIVE_POSITION_R",
            b"RELATIVE_POSITION_T",
            b"RELATIVE_POSITION_N",
            b"RELATIVE_VELOCITY_R",
            b"RELATIVE_VELOCITY_T",
            b"RELATIVE_VELOCITY_N",
        ],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT",
            b"OBJECT_DESIGNATOR",
            b"CATALOG_NAME",
            b"OBJECT_NAME",
            b"INTERNATIONAL_DESIGNATOR",
            b"OBJECT_TYPE",
            b"OPERATOR_CONTACT_POSITION",
            b"OPERATOR_ORGANIZATION",
            b"OPERATOR_PHONE",
            b"OPERATOR_EMAIL",
            b"EPHEMERIS_NAME",
            b"COVARIANCE_METHOD",
            b"MANEUVERABLE",
            b"ORBIT_CENTER",
            b"REF_FRAME",
            b"GRAVITY_MODEL",
            b"ATMOSPHERIC_MODEL",
            b"N_BODY_PERTURBATIONS",
            b"SOLAR_RAD_PRESSURE",
            b"EARTH_TIDES",
            b"INTRACK_THRUST",
        ],
        b"data" => &[
            b"COMMENT",
            b"odParameters",
            b"additionalParameters",
            b"stateVector",
            b"covarianceMatrix",
        ],
        b"odParameters" => &[
            b"COMMENT",
            b"TIME_LASTOB_START",
            b"TIME_LASTOB_END",
            b"RECOMMENDED_OD_SPAN",
            b"ACTUAL_OD_SPAN",
            b"OBS_AVAILABLE",
            b"OBS_USED",
            b"TRACKS_AVAILABLE",
            b"TRACKS_USED",
            b"RESIDUALS_ACCEPTED",
            b"WEIGHTED_RMS",
        ],
        b"additionalParameters" => &[
            b"COMMENT",
            b"AREA_PC",
            b"AREA_DRG",
            b"AREA_SRP",
            b"MASS",
            b"CD_AREA_OVER_MASS",
            b"CR_AREA_OVER_MASS",
            b"THRUST_ACCELERATION",
            b"SEDR",
        ],
        b"stateVector" => &[b"COMMENT", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT"],
        b"covarianceMatrix" => &[
            b"COMMENT",
            b"CR_R",
            b"CT_R",
            b"CT_T",
            b"CN_R",
            b"CN_T",
            b"CN_N",
            b"CRDOT_R",
            b"CRDOT_T",
            b"CRDOT_N",
            b"CRDOT_RDOT",
            b"CTDOT_R",
            b"CTDOT_T",
            b"CTDOT_N",
            b"CTDOT_RDOT",
            b"CTDOT_TDOT",
            b"CNDOT_R",
            b"CNDOT_T",
            b"CNDOT_N",
            b"CNDOT_RDOT",
            b"CNDOT_TDOT",
            b"CNDOT_NDOT",
            b"CDRG_R",
            b"CDRG_T",
            b"CDRG_N",
            b"CDRG_RDOT",
            b"CDRG_TDOT",
            b"CDRG_NDOT",
            b"CDRG_DRG",
            b"CSRP_R",
            b"CSRP_T",
            b"CSRP_N",
            b"CSRP_RDOT",
            b"CSRP_TDOT",
            b"CSRP_NDOT",
            b"CSRP_DRG",
            b"CSRP_SRP",
            b"CTHR_R",
            b"CTHR_T",
            b"CTHR_N",
            b"CTHR_RDOT",
            b"CTHR_TDOT",
            b"CTHR_NDOT",
            b"CTHR_DRG",
            b"CTHR_SRP",
            b"CTHR_THR",
        ],
        _ => return None,
    })
}

fn cdm_kvn_key(key: &str) -> Option<(u8, u16)> {
    const HEADER: &[&str] = &["CREATION_DATE", "ORIGINATOR", "MESSAGE_FOR", "MESSAGE_ID"];
    const RELATIVE: &[&str] = &[
        "TCA",
        "MISS_DISTANCE",
        "RELATIVE_SPEED",
        "RELATIVE_POSITION_R",
        "RELATIVE_POSITION_T",
        "RELATIVE_POSITION_N",
        "RELATIVE_VELOCITY_R",
        "RELATIVE_VELOCITY_T",
        "RELATIVE_VELOCITY_N",
        "START_SCREEN_PERIOD",
        "STOP_SCREEN_PERIOD",
        "SCREEN_VOLUME_FRAME",
        "SCREEN_VOLUME_SHAPE",
        "SCREEN_VOLUME_X",
        "SCREEN_VOLUME_Y",
        "SCREEN_VOLUME_Z",
        "SCREEN_ENTRY_TIME",
        "SCREEN_EXIT_TIME",
        "COLLISION_PROBABILITY",
        "COLLISION_PROBABILITY_METHOD",
    ];
    const METADATA: &[&str] = &[
        "OBJECT",
        "OBJECT_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "OBJECT_TYPE",
        "OPERATOR_CONTACT_POSITION",
        "OPERATOR_ORGANIZATION",
        "OPERATOR_PHONE",
        "OPERATOR_EMAIL",
        "EPHEMERIS_NAME",
        "COVARIANCE_METHOD",
        "MANEUVERABLE",
        "ORBIT_CENTER",
        "REF_FRAME",
        "GRAVITY_MODEL",
        "ATMOSPHERIC_MODEL",
        "N_BODY_PERTURBATIONS",
        "SOLAR_RAD_PRESSURE",
        "EARTH_TIDES",
        "INTRACK_THRUST",
    ];
    const OD: &[&str] = &[
        "TIME_LASTOB_START",
        "TIME_LASTOB_END",
        "RECOMMENDED_OD_SPAN",
        "ACTUAL_OD_SPAN",
        "OBS_AVAILABLE",
        "OBS_USED",
        "TRACKS_AVAILABLE",
        "TRACKS_USED",
        "RESIDUALS_ACCEPTED",
        "WEIGHTED_RMS",
    ];
    const ADDITIONAL: &[&str] = &[
        "AREA_PC",
        "AREA_DRG",
        "AREA_SRP",
        "MASS",
        "CD_AREA_OVER_MASS",
        "CR_AREA_OVER_MASS",
        "THRUST_ACCELERATION",
        "SEDR",
    ];
    const STATE: &[&str] = &["X", "Y", "Z", "X_DOT", "Y_DOT", "Z_DOT"];
    const COVARIANCE: &[&str] = &[
        "CR_R",
        "CT_R",
        "CT_T",
        "CN_R",
        "CN_T",
        "CN_N",
        "CRDOT_R",
        "CRDOT_T",
        "CRDOT_N",
        "CRDOT_RDOT",
        "CTDOT_R",
        "CTDOT_T",
        "CTDOT_N",
        "CTDOT_RDOT",
        "CTDOT_TDOT",
        "CNDOT_R",
        "CNDOT_T",
        "CNDOT_N",
        "CNDOT_RDOT",
        "CNDOT_TDOT",
        "CNDOT_NDOT",
        "CDRG_R",
        "CDRG_T",
        "CDRG_N",
        "CDRG_RDOT",
        "CDRG_TDOT",
        "CDRG_NDOT",
        "CDRG_DRG",
        "CSRP_R",
        "CSRP_T",
        "CSRP_N",
        "CSRP_RDOT",
        "CSRP_TDOT",
        "CSRP_NDOT",
        "CSRP_DRG",
        "CSRP_SRP",
        "CTHR_R",
        "CTHR_T",
        "CTHR_N",
        "CTHR_RDOT",
        "CTHR_TDOT",
        "CTHR_NDOT",
        "CTHR_DRG",
        "CTHR_SRP",
        "CTHR_THR",
    ];

    for (block, keys) in [
        (0, HEADER),
        (1, RELATIVE),
        (2, METADATA),
        (3, OD),
        (4, ADDITIONAL),
        (5, STATE),
        (6, COVARIANCE),
    ] {
        if let Some(rank) = keys.iter().position(|candidate| *candidate == key) {
            return Some((block, rank as u16));
        }
    }
    None
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating CDM KVN structure"],
            offset,
        }))))
    };
    let mut saw_version = false;
    let mut current_block = None;
    let mut previous_key = None;
    let mut pending_comments = false;
    let mut segment_count = 0u8;
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
            pending_comments = true;
            offset += raw_line.len() + 1;
            continue;
        }
        if !line.contains('=') {
            return fail("expected exactly one CDM assignment");
        }
        let key = line.split_once('=').unwrap().0.trim();
        if key == "CCSDS_CDM_VERS" {
            if saw_version || current_block.is_some() || pending_comments {
                return fail("CCSDS_CDM_VERS must be the first record");
            }
            saw_version = true;
            offset += raw_line.len() + 1;
            continue;
        }
        if !saw_version {
            return fail("expected CCSDS_CDM_VERS as the first record");
        }
        let (block, rank) = cdm_kvn_key(key)
            .ok_or_else(|| invalid(number, offset, "unknown CDM keyword".into()))?;

        match current_block {
            None => {
                if block != 0 {
                    return fail("expected CDM header");
                }
            }
            Some(current) if block == current => {
                if pending_comments && previous_key.is_some() {
                    return fail("COMMENT is not at the beginning of a CDM logical block");
                }
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order CDM keyword");
                }
            }
            Some(5 | 6) if block == 2 => {
                segment_count += 1;
                if segment_count > 2 {
                    return fail("CDM contains more than two segments");
                }
            }
            Some(current) => {
                let allowed = matches!(
                    (current, block),
                    (0, 1) | (1, 2) | (2, 3..=5) | (3, 4..=5) | (4, 5) | (5, 6)
                );
                if !allowed {
                    return fail("out-of-order CDM logical block");
                }
            }
        }
        if block == 2 && key == "OBJECT" && current_block != Some(2) && segment_count == 0 {
            segment_count = 1;
        }
        current_block = Some(block);
        previous_key = Some(rank);
        pending_comments = false;
        offset += raw_line.len() + 1;
    }

    if pending_comments {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "trailing CDM COMMENT has no logical block".into(),
        ));
    }
    if !saw_version {
        return Err(invalid(1, 0, "missing CCSDS_CDM_VERS".into()));
    }
    Ok(())
}

impl Cdm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        for segment in &self.body.segments {
            let first_nested_comments = if let Some(od) = &segment.data.od_parameters {
                &od.comment
            } else if let Some(additional) = &segment.data.additional_parameters {
                &additional.comment
            } else {
                &segment.data.state_vector.comment
            };
            if !first_nested_comments.is_empty() {
                return Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "CDM KVN cannot distinguish the outer data COMMENT run from the first nested logical-block COMMENT run",
                    ),
                    line: None,
                }
                .into());
            }
        }

        // CDM has a fixed, bounded scalar shape. A private model fixed-point preflight therefore
        // catches every lossy KVN scalar spelling and lexical record without introducing an
        // unbounded history-sized allocation or emitting caller-visible bytes.
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        let output = writer.finish();
        validate_kvn_syntax(&output)?;
        let reparsed = Self::from_kvn_str(&output)?;
        crate::traits::Validate::validate(&reparsed)?;
        if reparsed != *self {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "CDM model cannot be represented in KVN without changing typed content",
                ),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Header
//----------------------------------------------------------------------

/// Represents the `cdmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmHeader {
    /// Comments (allowed in the CDM Header only immediately after the CDM version number).
    /// (See 6.3.4 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Message creation date/time in Coordinated Universal Time (UTC). (See 6.3.2.6 for
    /// formatting rules.)
    ///
    /// **Examples**: 2010-03-12T22:31:12.000, 2010-071T22:31:12.000
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    pub creation_date: CalendarEpoch,
    /// Creating agency or owner/operator. Value should be the 'Abbreviation' value from the
    /// SANA 'Organizations' registry (<https://sanaregistry.org/r/organizations>) for an
    /// organization that has the Role of 'Conjunction Data Message Originator'. (See 5.2.9
    /// for formatting rules.)
    ///
    /// **Examples**: JSPOC, ESA SST, CAESAR, JPL, SDC
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[builder(into)]
    pub originator: String,
    /// Spacecraft name(s) for which the CDM is provided.
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub message_for: Option<String>,
    /// ID that uniquely identifies a message from a given originator. The format and content
    /// of the message identifier value are at the discretion of the originator. (See 5.2.9
    /// for formatting rules.)
    ///
    /// **Examples**: 201113719185, ABC-12_34
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[builder(into)]
    pub message_id: String,
}

impl Cdm {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.validate()?;
        Ok(())
    }
}

impl ToKvn for Cdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_CDM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

impl crate::traits::Validate for CdmHeader {
    fn validate(&self) -> Result<()> {
        if self.creation_date.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        if self.originator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.message_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "MESSAGE_ID".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for CdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);

        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(v) = &self.message_for {
            writer.write_pair("MESSAGE_FOR", v);
        }
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

//----------------------------------------------------------------------
// Body
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct CdmBody {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: RelativeMetadataData,
    #[serde(rename = "segment")]
    pub segments: Vec<CdmSegment>,
}

impl crate::traits::Validate for CdmBody {
    fn validate(&self) -> Result<()> {
        if self.segments.len() != 2 {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed("CDM Body must have exactly 2 segments"),
                line: None,
            }
            .into());
        }
        let object1_count = self
            .segments
            .iter()
            .filter(|s| s.metadata.object == CdmObjectType::Object1)
            .count();
        let object2_count = self
            .segments
            .iter()
            .filter(|s| s.metadata.object == CdmObjectType::Object2)
            .count();
        if object1_count != 1 || object2_count != 1 {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "CDM Body segments must contain exactly one OBJECT1 and one OBJECT2",
                ),
                line: None,
            }
            .into());
        }
        self.relative_metadata_data.validate()?;
        for segment in &self.segments {
            segment.validate()?;
        }
        Ok(())
    }
}

impl ToKvn for CdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.relative_metadata_data.write_kvn(writer);
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

//----------------------------------------------------------------------
// Relative Metadata/Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeMetadataData {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// The date and time in UTC of the closest approach. (See 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    pub tca: CalendarEpoch,
    /// The norm of the relative position vector. It indicates how close the two objects are at
    /// TCA. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    pub miss_distance: Length,
    /// The norm of the relative velocity vector. It indicates how fast the two objects are
    /// moving relative to each other at TCA. Data type = double.
    ///
    /// **Units**: m/s
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub relative_speed: Option<Dv>,
    /// The relative position and velocity of Object2 with respect to Object1.
    #[serde(
        rename = "relativeStateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_state_vector: Option<RelativeStateVector>,
    /// The start time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub start_screen_period: Option<CalendarEpoch>,
    /// The stop time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub stop_screen_period: Option<CalendarEpoch>,
    /// Name of the Object1 centered reference frame in which the screening volume data are
    /// given. Available options are RTN and Transverse, Velocity, and Normal (TVN). (See annex
    /// E for definition.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_volume_frame: Option<ScreenVolumeFrameType>,
    /// Shape of the screening volume: ELLIPSOID or BOX.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_volume_shape: Option<ScreenVolumeShapeType>,

    /// The R or T (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_volume_x: Option<Length>,
    /// The T or V (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_volume_y: Option<Length>,
    /// The N component size of the screening volume in the SCREEN_VOLUME_FRAME. Data type =
    /// double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_volume_z: Option<Length>,
    /// The time in UTC when Object2 enters the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_entry_time: Option<CalendarEpoch>,
    /// The time in UTC when Object2 exits the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub screen_exit_time: Option<CalendarEpoch>,
    /// The probability (denoted 'p' where 0.0<=p<=1.0), that Object1 and Object2 will collide.
    /// Data type = double.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub collision_probability: Option<Probability>,
    /// The method that was used to calculate the collision probability. (See annex E for
    /// definition.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub collision_probability_method: Option<String>,
}

impl crate::traits::Validate for RelativeMetadataData {
    fn validate(&self) -> Result<()> {
        // These are plain doubles in the CDM schema, so only finiteness constrains them. The KVN
        // writer already refused non-finite numbers; `validate` and the XML writer did not, and
        // an infinity reached the output as the lexical `inf`, which `xsd:double` rejects.
        let mut numbers: Vec<(&'static str, f64)> =
            vec![("MISS_DISTANCE", self.miss_distance.value)];
        if let Some(value) = &self.relative_speed {
            numbers.push(("RELATIVE_SPEED", value.value));
        }
        if let Some(vector) = &self.relative_state_vector {
            numbers.extend([
                ("RELATIVE_POSITION_R", vector.relative_position_r.value),
                ("RELATIVE_POSITION_T", vector.relative_position_t.value),
                ("RELATIVE_POSITION_N", vector.relative_position_n.value),
                ("RELATIVE_VELOCITY_R", vector.relative_velocity_r.value),
                ("RELATIVE_VELOCITY_T", vector.relative_velocity_t.value),
                ("RELATIVE_VELOCITY_N", vector.relative_velocity_n.value),
            ]);
        }
        for (field, value) in [
            ("SCREEN_VOLUME_X", &self.screen_volume_x),
            ("SCREEN_VOLUME_Y", &self.screen_volume_y),
            ("SCREEN_VOLUME_Z", &self.screen_volume_z),
        ] {
            if let Some(value) = value {
                numbers.push((field, value.value));
            }
        }
        for (field, value) in numbers {
            require_finite(field, value)?;
        }
        if let Some(prob) = &self.collision_probability {
            if !(0.0..=1.0).contains(&prob.value) {
                return Err(crate::error::ValidationError::OutOfRange {
                    name: "COLLISION_PROBABILITY".into(),
                    value: prob.value.to_string(),
                    expected: "0.0 <= p <= 1.0".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

impl ToKvn for RelativeMetadataData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("TCA", self.tca);
        writer.write_measure("MISS_DISTANCE", &self.miss_distance);
        if let Some(v) = &self.relative_speed {
            writer.write_measure("RELATIVE_SPEED", &v.to_unit_value());
        }
        if let Some(v) = &self.relative_state_vector {
            v.write_kvn(writer);
        }
        if let Some(v) = &self.start_screen_period {
            writer.write_pair("START_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.stop_screen_period {
            writer.write_pair("STOP_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.screen_volume_frame {
            writer.write_pair("SCREEN_VOLUME_FRAME", v.to_string());
        }
        if let Some(v) = &self.screen_volume_shape {
            writer.write_pair("SCREEN_VOLUME_SHAPE", v.to_string());
        }

        if let Some(v) = &self.screen_volume_x {
            writer.write_measure("SCREEN_VOLUME_X", v);
        }
        if let Some(v) = &self.screen_volume_y {
            writer.write_measure("SCREEN_VOLUME_Y", v);
        }
        if let Some(v) = &self.screen_volume_z {
            writer.write_measure("SCREEN_VOLUME_Z", v);
        }
        if let Some(v) = &self.screen_entry_time {
            writer.write_pair("SCREEN_ENTRY_TIME", v);
        }
        if let Some(v) = &self.screen_exit_time {
            writer.write_pair("SCREEN_EXIT_TIME", v);
        }
        if let Some(v) = &self.collision_probability {
            writer.write_pair("COLLISION_PROBABILITY", v.value);
        }
        if let Some(v) = &self.collision_probability_method {
            writer.write_pair("COLLISION_PROBABILITY_METHOD", v);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeStateVector {
    pub relative_position_r: Length,
    pub relative_position_t: Length,
    pub relative_position_n: Length,
    pub relative_velocity_r: Dv,
    pub relative_velocity_t: Dv,
    pub relative_velocity_n: Dv,
}

impl ToKvn for RelativeStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_measure("RELATIVE_POSITION_R", &self.relative_position_r);
        writer.write_measure("RELATIVE_POSITION_T", &self.relative_position_t);
        writer.write_measure("RELATIVE_POSITION_N", &self.relative_position_n);
        writer.write_measure(
            "RELATIVE_VELOCITY_R",
            &self.relative_velocity_r.to_unit_value(),
        );
        writer.write_measure(
            "RELATIVE_VELOCITY_T",
            &self.relative_velocity_t.to_unit_value(),
        );
        writer.write_measure(
            "RELATIVE_VELOCITY_N",
            &self.relative_velocity_n.to_unit_value(),
        );
    }
}

//----------------------------------------------------------------------
// Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct CdmSegment {
    pub metadata: CdmMetadata,
    pub data: CdmData,
}

impl crate::traits::Validate for CdmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()?;
        Ok(())
    }
}

impl ToKvn for CdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmMetadata {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// The object to which the metadata and data apply (Object1 or Object2).
    ///
    /// **Examples**: OBJECT1, OBJECT2
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub object: CdmObjectType,
    /// The satellite catalog designator for the object. (See 5.2.9 for formatting rules.)
    ///
    /// **Examples**: 12345
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub object_designator: String,
    /// The satellite catalog used for the object. Value should be taken from the SANA
    /// 'Conjunction Data Message CATALOG_NAME' registry
    /// (<https://sanaregistry.org/r/cdm_catalog>). (See 5.2.9 for formatting rules.)
    ///
    /// **Examples**: SATCAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub catalog_name: String,
    /// Spacecraft name for the object.
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub object_name: String,
    /// The full international designator for the object. Values shall have the format
    /// YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of launch
    /// (with leading zeros); P{PP} = At least one capital letter for the identification of the
    /// part brought into space by the launch. In cases where the object has no international
    /// designator, the value UNKNOWN should be used. (See 5.2.9 for further formatting rules.)
    ///
    /// **Examples**: 2002-021A, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub international_designator: String,
    /// The object type.
    ///
    /// **Examples**: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub object_type: Option<ObjectDescription>,
    /// Contact position of the owner/operator of the object.
    ///
    /// **Examples**: ORBITAL SAFETY ANALYST (OSA), NETWORK CONTROLLER
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub operator_contact_position: Option<String>,
    /// Contact organization of the object.
    ///
    /// **Examples**: EUMETSAT, ESA, INTELSAT, IRIDIUM
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub operator_organization: Option<String>,
    /// Phone number of the contact position or organization for the object.
    ///
    /// **Examples**: +49615130312
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub operator_phone: Option<String>,
    /// Email address of the contact position or organization of the object.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.NET
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub operator_email: Option<String>,
    /// Unique name of the external ephemeris file used for the object or NONE. This is used to
    /// indicate whether an external (i.e., Owner/Operator [O/O] provided) ephemeris file was
    /// used to calculate the CA. If 'NONE' is specified, then the output of the most current
    /// Orbit Determination (OD) of the CDM originator was used in the CA.
    ///
    /// **Examples**: EPHEMERIS SATELLITE A, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub ephemeris_name: String,
    /// Method used to calculate the covariance during the OD that produced the state vector, or
    /// whether an arbitrary, non-calculated default value was used. Caution should be used
    /// when using the default value for calculating collision probability.
    ///
    /// **Examples**: CALCULATED, DEFAULT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub covariance_method: CovarianceMethodType,
    /// The maneuver capacity of the object. (See 1.4.3.1 for definition of 'N/A'.)
    ///
    /// **Examples**: YES, NO, N/A
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub maneuverable: ManeuverableType,
    /// The central body about which Object1 and Object2 orbit. If not specified, the center is
    /// assumed to be Earth.
    ///
    /// **Examples**: EARTH, SUN, MOON, MARS
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub orbit_center: Option<String>,
    /// Name of the reference frame in which the state vector data are given. Value must be
    /// selected from the list of values to the right (see reference `[F1]`) and be the same for
    /// both Object1 and Object2.
    ///
    /// **Examples**: GCRF, EME2000, ITRF
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub ref_frame: ReferenceFrameType,
    /// The gravity model used for the OD of the object. (See annex E under GRAVITY_MODEL for
    /// definition).
    ///
    /// **Examples**: EGM-96: 36D 360, WGS-84_GEOID: 24D 240, JGM-2: 41D 410
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub gravity_model: Option<String>,
    /// The atmospheric density model used for the OD of the object. If 'NONE' is specified,
    /// then no atmospheric model was used.
    ///
    /// **Examples**: JACCHIA 70, MSIS, JACCHIA 70 DCA, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub atmospheric_model: Option<String>,
    /// The N-body gravitational perturbations used for the OD of the object. If 'NONE' is
    /// specified, then no third-body gravitational perturbations were used.
    ///
    /// **Examples**: MOON, SUN, JUPITER, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub n_body_perturbations: Option<String>,
    /// Indication of whether solar radiation pressure perturbations were used for the OD of the
    /// object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub solar_rad_pressure: Option<YesNo>,
    /// Indication of whether solid Earth and ocean tides were used for the OD of the object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub earth_tides: Option<YesNo>,
    /// Indication of whether in-track thrust modeling was used for the OD of the object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub intrack_thrust: Option<YesNo>,
}

impl ToKvn for CdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair(
            "OBJECT",
            match self.object {
                CdmObjectType::Object1 => "OBJECT1",
                CdmObjectType::Object2 => "OBJECT2",
            },
        );
        writer.write_pair("OBJECT_DESIGNATOR", &self.object_designator);
        writer.write_pair("CATALOG_NAME", &self.catalog_name);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        if let Some(v) = &self.operator_contact_position {
            writer.write_pair("OPERATOR_CONTACT_POSITION", v);
        }
        if let Some(v) = &self.operator_organization {
            writer.write_pair("OPERATOR_ORGANIZATION", v);
        }
        if let Some(v) = &self.operator_phone {
            writer.write_pair("OPERATOR_PHONE", v);
        }
        if let Some(v) = &self.operator_email {
            writer.write_pair("OPERATOR_EMAIL", v);
        }
        writer.write_pair("EPHEMERIS_NAME", &self.ephemeris_name);
        writer.write_pair("COVARIANCE_METHOD", self.covariance_method.to_string());
        writer.write_pair("MANEUVERABLE", self.maneuverable.to_string());
        if let Some(v) = &self.orbit_center {
            writer.write_pair("ORBIT_CENTER", v);
        }
        writer.write_pair("REF_FRAME", self.ref_frame.to_string());
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", v.to_string());
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", v.to_string());
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", v.to_string());
        }
    }
}

impl crate::traits::Validate for CdmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_designator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "OBJECT_DESIGNATOR".into(),
                line: None,
            }
            .into());
        }
        if self.catalog_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "CATALOG_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.international_designator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "INTERNATIONAL_DESIGNATOR".into(),
                line: None,
            }
            .into());
        }
        if self.ephemeris_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "EPHEMERIS_NAME".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl CdmMetadata {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Orbit Determination Parameters.
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    /// Additional Parameters.
    #[serde(
        rename = "additionalParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parameters: Option<AdditionalParameters>,
    /// State Vector.
    #[serde(rename = "stateVector")]
    pub state_vector: CdmStateVector,
    /// Covariance Matrix.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<CdmCovarianceMatrix>,
}

impl crate::traits::Validate for CdmData {
    fn validate(&self) -> Result<()> {
        if self.covariance_matrix.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Data".into(),
                field: "covarianceMatrix".into(),
                line: None,
            }
            .into());
        }
        if let Some(parameters) = &self.od_parameters {
            parameters.validate()?;
        }
        if let Some(parameters) = &self.additional_parameters {
            parameters.validate()?;
        }
        self.state_vector.validate()?;
        if let Some(covariance_matrix) = &self.covariance_matrix {
            covariance_matrix.validate()?;
        }
        Ok(())
    }
}

impl ToKvn for CdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // OD Parameters
        if let Some(od) = &self.od_parameters {
            writer.write_comments(&od.comment);
            if let Some(v) = &od.time_lastob_start {
                writer.write_pair("TIME_LASTOB_START", v);
            }
            if let Some(v) = &od.time_lastob_end {
                writer.write_pair("TIME_LASTOB_END", v);
            }
            if let Some(v) = &od.recommended_od_span {
                writer.write_measure("RECOMMENDED_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.actual_od_span {
                writer.write_measure("ACTUAL_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.obs_available {
                writer.write_pair("OBS_AVAILABLE", v);
            }
            if let Some(v) = &od.obs_used {
                writer.write_pair("OBS_USED", v);
            }
            if let Some(v) = &od.tracks_available {
                writer.write_pair("TRACKS_AVAILABLE", v);
            }
            if let Some(v) = &od.tracks_used {
                writer.write_pair("TRACKS_USED", v);
            }
            if let Some(v) = &od.residuals_accepted {
                writer.write_measure("RESIDUALS_ACCEPTED", &v.to_unit_value());
            }
            if let Some(v) = &od.weighted_rms {
                writer.write_pair("WEIGHTED_RMS", v);
            }
        }
        // Additional Parameters
        if let Some(ap) = &self.additional_parameters {
            writer.write_comments(&ap.comment);
            if let Some(v) = &ap.area_pc {
                writer.write_measure("AREA_PC", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_drg {
                writer.write_measure("AREA_DRG", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_srp {
                writer.write_measure("AREA_SRP", &v.to_unit_value());
            }
            if let Some(v) = &ap.mass {
                writer.write_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.cd_area_over_mass {
                writer.write_measure("CD_AREA_OVER_MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.cr_area_over_mass {
                writer.write_measure("CR_AREA_OVER_MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", &v.to_unit_value());
            }
            if let Some(v) = &ap.sedr {
                writer.write_measure("SEDR", &v.to_unit_value());
            }
        }
        // State Vector
        self.state_vector.write_kvn(writer);
        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdditionalParameters {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,

    /// The actual area of the object. (See annex E for definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub area_pc: Option<Area>,
    /// The effective area of the object exposed to atmospheric drag. (See annex E for
    /// definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub area_drg: Option<Area>,

    /// The effective area of the object exposed to solar radiation pressure. (See annex E for
    /// definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub area_srp: Option<Area>,

    /// The mass of the object.
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub mass: Option<Mass>,

    /// The object's CD•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cd_area_over_mass: Option<M2kgRequired>,

    /// The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cr_area_over_mass: Option<M2kgRequired>,

    /// The object's acceleration due to in-track thrust used to propagate the state vector and
    /// covariance to TCA. (See annex E for definition.)
    ///
    /// **Units**: m/s²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub thrust_acceleration: Option<Ms2>,

    /// The amount of energy being removed from the object's orbit by atmospheric drag. This
    /// value is an average calculated during the OD.
    ///
    /// **Units**: W/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sedr: Option<Wkg>,
}

impl Validate for AdditionalParameters {
    fn validate(&self) -> Result<()> {
        // NDM/XML 4.0: areaType, massType, m2kgType and wkgType extend
        // nonNegativeDouble; only thrust acceleration is a signed double.
        for (field, value, nonnegative) in [
            ("AREA_PC", self.area_pc.as_ref().map(|v| v.value), true),
            ("AREA_DRG", self.area_drg.as_ref().map(|v| v.value), true),
            ("AREA_SRP", self.area_srp.as_ref().map(|v| v.value), true),
            ("MASS", self.mass.as_ref().map(|v| v.value), true),
            (
                "CD_AREA_OVER_MASS",
                self.cd_area_over_mass.as_ref().map(|v| v.value),
                true,
            ),
            (
                "CR_AREA_OVER_MASS",
                self.cr_area_over_mass.as_ref().map(|v| v.value),
                true,
            ),
            (
                "THRUST_ACCELERATION",
                self.thrust_acceleration.as_ref().map(|v| v.value),
                false,
            ),
            ("SEDR", self.sedr.as_ref().map(|v| v.value), true),
        ] {
            let Some(value) = value else { continue };
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
            if nonnegative && value < 0.0 {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: value.to_string(),
                    expected: ">= 0".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmStateVector {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Object Position Vector X component.
    ///
    /// Units: km
    pub x: PositionRequired,
    /// Object Position Vector Y component.
    ///
    /// Units: km
    pub y: PositionRequired,
    /// Object Position Vector Z component.
    ///
    /// Units: km
    pub z: PositionRequired,
    /// Object Velocity Vector X component.
    ///
    /// Units: km/s
    pub x_dot: VelocityRequired,
    /// Object Velocity Vector Y component.
    ///
    /// Units: km/s
    pub y_dot: VelocityRequired,
    /// Object Velocity Vector Z component.
    ///
    /// Units: km/s
    pub z_dot: VelocityRequired,
}

fn non_finite_error(field: &'static str, value: f64) -> Option<ValidationError> {
    (!value.is_finite()).then(|| ValidationError::InvalidValue {
        field: field.into(),
        value: value.to_string(),
        expected: "a finite number".into(),
        line: None,
    })
}

impl Validate for CdmStateVector {
    fn validate(&self) -> Result<()> {
        for (field, value) in self.values() {
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
        }
        Ok(())
    }
}

impl CdmStateVector {
    fn values(&self) -> impl Iterator<Item = (&'static str, f64)> {
        [
            ("X", self.x.value),
            ("Y", self.y.value),
            ("Z", self.z.value),
            ("X_DOT", self.x_dot.value),
            ("Y_DOT", self.y_dot.value),
            ("Z_DOT", self.z_dot.value),
        ]
        .into_iter()
    }
}

impl ToKvn for CdmStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_measure("X", &self.x.to_unit_value());
        writer.write_measure("Y", &self.y.to_unit_value());
        writer.write_measure("Z", &self.z.to_unit_value());
        writer.write_measure("X_DOT", &self.x_dot.to_unit_value());
        writer.write_measure("Y_DOT", &self.y_dot.to_unit_value());
        writer.write_measure("Z_DOT", &self.z_dot.to_unit_value());
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmCovarianceMatrix {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Object covariance matrix `[1,1]`.
    ///
    /// Units: m²
    pub cr_r: M2,
    /// Object covariance matrix `[2,1]`.
    ///
    /// Units: m²
    pub ct_r: M2,
    /// Object covariance matrix `[2,2]`.
    ///
    /// Units: m²
    pub ct_t: M2,
    /// Object covariance matrix `[3,1]`.
    ///
    /// Units: m²
    pub cn_r: M2,
    /// Object covariance matrix `[3,2]`.
    ///
    /// Units: m²
    pub cn_t: M2,
    /// Object covariance matrix `[3,3]`.
    ///
    /// Units: m²
    pub cn_n: M2,
    /// Object covariance matrix `[4,1]`.
    ///
    /// Units: m²/s
    pub crdot_r: M2s,
    /// Object covariance matrix `[4,2]`.
    ///
    /// Units: m²/s
    pub crdot_t: M2s,
    /// Object covariance matrix `[4,3]`.
    ///
    /// Units: m²/s
    pub crdot_n: M2s,
    /// Object covariance matrix `[4,4]`.
    ///
    /// Units: m²/s²
    pub crdot_rdot: M2s2,
    /// Object covariance matrix `[5,1]`.
    ///
    /// Units: m²/s
    pub ctdot_r: M2s,
    /// Object covariance matrix `[5,2]`.
    ///
    /// Units: m²/s
    pub ctdot_t: M2s,
    /// Object covariance matrix `[5,3]`.
    ///
    /// Units: m²/s
    pub ctdot_n: M2s,
    /// Object covariance matrix `[5,4]`.
    ///
    /// Units: m²/s²
    pub ctdot_rdot: M2s2,
    /// Object covariance matrix `[5,5]`.
    ///
    /// Units: m²/s²
    pub ctdot_tdot: M2s2,
    /// Object covariance matrix `[6,1]`.
    ///
    /// Units: m²/s
    pub cndot_r: M2s,
    /// Object covariance matrix `[6,2]`.
    ///
    /// Units: m²/s
    pub cndot_t: M2s,
    /// Object covariance matrix `[6,3]`.
    ///
    /// Units: m²/s
    pub cndot_n: M2s,
    /// Object covariance matrix `[6,4]`.
    ///
    /// Units: m²/s²
    pub cndot_rdot: M2s2,
    /// Object covariance matrix `[6,5]`.
    ///
    /// Units: m²/s²
    pub cndot_tdot: M2s2,
    /// Object covariance matrix `[6,6]`.
    ///
    /// Units: m²/s²
    pub cndot_ndot: M2s2,

    /// Object covariance matrix `[7,1]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_r: Option<M3kg>,
    /// Object covariance matrix `[7,2]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_t: Option<M3kg>,
    /// Object covariance matrix `[7,3]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_n: Option<M3kg>,
    /// Object covariance matrix `[7,4]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_rdot: Option<M3kgs>,
    /// Object covariance matrix `[7,5]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_tdot: Option<M3kgs>,
    /// Object covariance matrix `[7,6]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_ndot: Option<M3kgs>,
    /// Object covariance matrix `[7,7]`.
    ///
    /// Units: m⁴/kg²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cdrg_drg: Option<M4kg2>,

    /// Object covariance matrix `[8,1]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_r: Option<M3kg>,
    /// Object covariance matrix `[8,2]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_t: Option<M3kg>,
    /// Object covariance matrix `[8,3]`.
    ///
    /// Units: m³/kg
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_n: Option<M3kg>,
    /// Object covariance matrix `[8,4]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_rdot: Option<M3kgs>,
    /// Object covariance matrix `[8,5]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_tdot: Option<M3kgs>,
    /// Object covariance matrix `[8,6]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_ndot: Option<M3kgs>,
    /// Object covariance matrix `[8,7]`.
    ///
    /// Units: m⁴/kg²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_drg: Option<M4kg2>,
    /// Object covariance matrix `[8,8]`.
    ///
    /// Units: m⁴/kg²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub csrp_srp: Option<M4kg2>,

    /// Object covariance matrix `[9,1]`.
    ///
    /// Units: m²/s²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_r: Option<M2s2>,
    /// Object covariance matrix `[9,2]`.
    ///
    /// Units: m²/s²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_t: Option<M2s2>,
    /// Object covariance matrix `[9,3]`.
    ///
    /// Units: m²/s²
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_n: Option<M2s2>,
    /// Object covariance matrix `[9,4]`.
    ///
    /// Units: m²/s³
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_rdot: Option<M2s3>,
    /// Object covariance matrix `[9,5]`.
    ///
    /// Units: m²/s³
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_tdot: Option<M2s3>,
    /// Object covariance matrix `[9,6]`.
    ///
    /// Units: m²/s³
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_ndot: Option<M2s3>,
    /// Object covariance matrix `[9,7]`.
    ///
    /// Units: m³/(kg*s²)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_drg: Option<M3kgs2>,
    /// Object covariance matrix `[9,8]`.
    ///
    /// Units: m³/(kg*s²)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_srp: Option<M3kgs2>,
    /// Object covariance matrix `[9,9]`.
    ///
    /// Units: m²/s⁴
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cthr_thr: Option<M2s4>,
}

impl Validate for CdmCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if let Some(error) = self.optional_row_errors().into_iter().next() {
            return Err(error.into());
        }
        for (field, value) in self.values() {
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
        }
        Ok(())
    }
}

impl CdmCovarianceMatrix {
    fn optional_row_errors(&self) -> Vec<ValidationError> {
        let row_7 = [
            ("CDRG_R", self.cdrg_r.is_some()),
            ("CDRG_T", self.cdrg_t.is_some()),
            ("CDRG_N", self.cdrg_n.is_some()),
            ("CDRG_RDOT", self.cdrg_rdot.is_some()),
            ("CDRG_TDOT", self.cdrg_tdot.is_some()),
            ("CDRG_NDOT", self.cdrg_ndot.is_some()),
            ("CDRG_DRG", self.cdrg_drg.is_some()),
        ];
        let row_8 = [
            ("CSRP_R", self.csrp_r.is_some()),
            ("CSRP_T", self.csrp_t.is_some()),
            ("CSRP_N", self.csrp_n.is_some()),
            ("CSRP_RDOT", self.csrp_rdot.is_some()),
            ("CSRP_TDOT", self.csrp_tdot.is_some()),
            ("CSRP_NDOT", self.csrp_ndot.is_some()),
            ("CSRP_DRG", self.csrp_drg.is_some()),
            ("CSRP_SRP", self.csrp_srp.is_some()),
        ];
        let row_9 = [
            ("CTHR_R", self.cthr_r.is_some()),
            ("CTHR_T", self.cthr_t.is_some()),
            ("CTHR_N", self.cthr_n.is_some()),
            ("CTHR_RDOT", self.cthr_rdot.is_some()),
            ("CTHR_TDOT", self.cthr_tdot.is_some()),
            ("CTHR_NDOT", self.cthr_ndot.is_some()),
            ("CTHR_DRG", self.cthr_drg.is_some()),
            ("CTHR_SRP", self.cthr_srp.is_some()),
            ("CTHR_THR", self.cthr_thr.is_some()),
        ];
        let rows: [(&str, &[(&str, bool)]); 3] = [
            ("CDM Covariance Matrix row 7", &row_7),
            ("CDM Covariance Matrix row 8", &row_8),
            ("CDM Covariance Matrix row 9", &row_9),
        ];
        let Some(last_present_row) = rows
            .iter()
            .rposition(|(_, fields)| fields.iter().any(|(_, present)| *present))
        else {
            return Vec::new();
        };

        rows.into_iter()
            .take(last_present_row + 1)
            .flat_map(|(block, fields)| {
                fields
                    .iter()
                    .filter(|(_, present)| !present)
                    .map(move |(field, _)| ValidationError::MissingRequiredField {
                        block: block.into(),
                        field: (*field).into(),
                        line: None,
                    })
            })
            .collect()
    }

    fn values(&self) -> impl Iterator<Item = (&'static str, f64)> + '_ {
        let required = [
            ("CR_R", self.cr_r.value),
            ("CT_R", self.ct_r.value),
            ("CT_T", self.ct_t.value),
            ("CN_R", self.cn_r.value),
            ("CN_T", self.cn_t.value),
            ("CN_N", self.cn_n.value),
            ("CRDOT_R", self.crdot_r.value),
            ("CRDOT_T", self.crdot_t.value),
            ("CRDOT_N", self.crdot_n.value),
            ("CRDOT_RDOT", self.crdot_rdot.value),
            ("CTDOT_R", self.ctdot_r.value),
            ("CTDOT_T", self.ctdot_t.value),
            ("CTDOT_N", self.ctdot_n.value),
            ("CTDOT_RDOT", self.ctdot_rdot.value),
            ("CTDOT_TDOT", self.ctdot_tdot.value),
            ("CNDOT_R", self.cndot_r.value),
            ("CNDOT_T", self.cndot_t.value),
            ("CNDOT_N", self.cndot_n.value),
            ("CNDOT_RDOT", self.cndot_rdot.value),
            ("CNDOT_TDOT", self.cndot_tdot.value),
            ("CNDOT_NDOT", self.cndot_ndot.value),
        ];
        let optional = [
            ("CDRG_R", self.cdrg_r.as_ref().map(|value| value.value)),
            ("CDRG_T", self.cdrg_t.as_ref().map(|value| value.value)),
            ("CDRG_N", self.cdrg_n.as_ref().map(|value| value.value)),
            (
                "CDRG_RDOT",
                self.cdrg_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CDRG_TDOT",
                self.cdrg_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CDRG_NDOT",
                self.cdrg_ndot.as_ref().map(|value| value.value),
            ),
            ("CDRG_DRG", self.cdrg_drg.as_ref().map(|value| value.value)),
            ("CSRP_R", self.csrp_r.as_ref().map(|value| value.value)),
            ("CSRP_T", self.csrp_t.as_ref().map(|value| value.value)),
            ("CSRP_N", self.csrp_n.as_ref().map(|value| value.value)),
            (
                "CSRP_RDOT",
                self.csrp_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CSRP_TDOT",
                self.csrp_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CSRP_NDOT",
                self.csrp_ndot.as_ref().map(|value| value.value),
            ),
            ("CSRP_DRG", self.csrp_drg.as_ref().map(|value| value.value)),
            ("CSRP_SRP", self.csrp_srp.as_ref().map(|value| value.value)),
            ("CTHR_R", self.cthr_r.as_ref().map(|value| value.value)),
            ("CTHR_T", self.cthr_t.as_ref().map(|value| value.value)),
            ("CTHR_N", self.cthr_n.as_ref().map(|value| value.value)),
            (
                "CTHR_RDOT",
                self.cthr_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CTHR_TDOT",
                self.cthr_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CTHR_NDOT",
                self.cthr_ndot.as_ref().map(|value| value.value),
            ),
            ("CTHR_DRG", self.cthr_drg.as_ref().map(|value| value.value)),
            ("CTHR_SRP", self.cthr_srp.as_ref().map(|value| value.value)),
            ("CTHR_THR", self.cthr_thr.as_ref().map(|value| value.value)),
        ];

        required.into_iter().chain(
            optional
                .into_iter()
                .filter_map(|(field, value)| value.map(|value| (field, value))),
        )
    }
}

impl ToKvn for CdmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Required
        writer.write_measure("CR_R", &self.cr_r.to_unit_value());
        writer.write_measure("CT_R", &self.ct_r.to_unit_value());
        writer.write_measure("CT_T", &self.ct_t.to_unit_value());
        writer.write_measure("CN_R", &self.cn_r.to_unit_value());
        writer.write_measure("CN_T", &self.cn_t.to_unit_value());
        writer.write_measure("CN_N", &self.cn_n.to_unit_value());
        writer.write_measure("CRDOT_R", &self.crdot_r.to_unit_value());
        writer.write_measure("CRDOT_T", &self.crdot_t.to_unit_value());
        writer.write_measure("CRDOT_N", &self.crdot_n.to_unit_value());
        writer.write_measure("CRDOT_RDOT", &self.crdot_rdot.to_unit_value());
        writer.write_measure("CTDOT_R", &self.ctdot_r.to_unit_value());
        writer.write_measure("CTDOT_T", &self.ctdot_t.to_unit_value());
        writer.write_measure("CTDOT_N", &self.ctdot_n.to_unit_value());
        writer.write_measure("CTDOT_RDOT", &self.ctdot_rdot.to_unit_value());
        writer.write_measure("CTDOT_TDOT", &self.ctdot_tdot.to_unit_value());
        writer.write_measure("CNDOT_R", &self.cndot_r.to_unit_value());
        writer.write_measure("CNDOT_T", &self.cndot_t.to_unit_value());
        writer.write_measure("CNDOT_N", &self.cndot_n.to_unit_value());
        writer.write_measure("CNDOT_RDOT", &self.cndot_rdot.to_unit_value());
        writer.write_measure("CNDOT_TDOT", &self.cndot_tdot.to_unit_value());
        writer.write_measure("CNDOT_NDOT", &self.cndot_ndot.to_unit_value());

        // Optionals
        if let Some(v) = &self.cdrg_r {
            writer.write_measure("CDRG_R", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_t {
            writer.write_measure("CDRG_T", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_n {
            writer.write_measure("CDRG_N", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_rdot {
            writer.write_measure("CDRG_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_tdot {
            writer.write_measure("CDRG_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_ndot {
            writer.write_measure("CDRG_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_drg {
            writer.write_measure("CDRG_DRG", &v.to_unit_value());
        }

        if let Some(v) = &self.csrp_r {
            writer.write_measure("CSRP_R", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_t {
            writer.write_measure("CSRP_T", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_n {
            writer.write_measure("CSRP_N", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_rdot {
            writer.write_measure("CSRP_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_tdot {
            writer.write_measure("CSRP_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_ndot {
            writer.write_measure("CSRP_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_drg {
            writer.write_measure("CSRP_DRG", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_srp {
            writer.write_measure("CSRP_SRP", &v.to_unit_value());
        }

        if let Some(v) = &self.cthr_r {
            writer.write_measure("CTHR_R", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_t {
            writer.write_measure("CTHR_T", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_n {
            writer.write_measure("CTHR_N", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_rdot {
            writer.write_measure("CTHR_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_tdot {
            writer.write_measure("CTHR_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_ndot {
            writer.write_measure("CTHR_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_drg {
            writer.write_measure("CTHR_DRG", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_srp {
            writer.write_measure("CTHR_SRP", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_thr {
            writer.write_measure("CTHR_THR", &v.to_unit_value());
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    fn sample_cdm_kvn() -> String {
        let kvn = r#"
CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

"#;
        kvn.to_string()
    }

    #[test]
    fn kvn_roundtrip() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        let regenerated = cdm.to_kvn().expect("to_kvn");
        // Parse again to ensure structural equality
        let cdm2 = Cdm::from_kvn(&regenerated).expect("re-parse");
        assert_eq!(cdm.header.originator, cdm2.header.originator);
        assert_eq!(cdm.body.segments.len(), cdm2.body.segments.len());
    }

    #[test]
    fn xml_roundtrip() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        let xml = cdm.to_xml().expect("to_xml");
        let cdm2 = Cdm::from_xml(&xml).expect("from_xml");
        assert_eq!(cdm.header.originator, cdm2.header.originator);
        assert_eq!(cdm.body.segments.len(), cdm2.body.segments.len());
    }

    #[test]
    fn full_optional_fields_roundtrip() {
        let kvn = r#"
CCSDS_CDM_VERS = 1.0
COMMENT Header comment
CREATION_DATE = 2025-01-01T00:00:00

ORIGINATOR = TEST
MESSAGE_FOR = RECIPIENT
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
START_SCREEN_PERIOD = 2025-01-02T11:00:00
STOP_SCREEN_PERIOD = 2025-01-02T13:00:00
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
SCREEN_ENTRY_TIME = 2025-01-02T11:30:00
SCREEN_EXIT_TIME = 2025-01-02T12:30:00
COLLISION_PROBABILITY = 0.001
COLLISION_PROBABILITY_METHOD = FOSTER-1992

COMMENT Segment 1
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
OPERATOR_CONTACT_POSITION = POSITION
OPERATOR_ORGANIZATION = ORG
OPERATOR_PHONE = PHONE
OPERATOR_EMAIL = EMAIL
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
ORBIT_CENTER = EARTH
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96
ATMOSPHERIC_MODEL = JACCHIA 70 DCA
N_BODY_PERTURBATIONS = MOON,SUN
SOLAR_RAD_PRESSURE = YES
EARTH_TIDES = YES
INTRACK_THRUST = YES

TIME_LASTOB_START = 2025-01-01T00:00:00
TIME_LASTOB_END = 2025-01-02T00:00:00
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.0 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.23

AREA_PC = 10.0 [m**2]
AREA_DRG = 12.0 [m**2]
AREA_SRP = 15.0 [m**2]
MASS = 1000.0 [kg]
CD_AREA_OVER_MASS = 0.012 [m**2/kg]
CR_AREA_OVER_MASS = 0.015 [m**2/kg]
THRUST_ACCELERATION = 0.001 [m/s**2]
SEDR = 0.05 [W/kg]

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
CDRG_R = 0.001 [m**3/kg]
CDRG_T = 0.002 [m**3/kg]
CDRG_N = 0.003 [m**3/kg]
CDRG_RDOT = 0.0001 [m**3/(kg*s)]
CDRG_TDOT = 0.0002 [m**3/(kg*s)]
CDRG_NDOT = 0.0003 [m**3/(kg*s)]
CDRG_DRG = 0.00001 [m**4/kg**2]
CSRP_R = 0.001 [m**3/kg]
CSRP_T = 0.002 [m**3/kg]
CSRP_N = 0.003 [m**3/kg]
CSRP_RDOT = 0.0001 [m**3/(kg*s)]
CSRP_TDOT = 0.0002 [m**3/(kg*s)]
CSRP_NDOT = 0.0003 [m**3/(kg*s)]
CSRP_DRG = 0.00001 [m**4/kg**2]
CSRP_SRP = 0.00002 [m**4/kg**2]
CTHR_R = 0.001 [m**2/s**2]
CTHR_T = 0.002 [m**2/s**2]
CTHR_N = 0.003 [m**2/s**2]
CTHR_RDOT = 0.0001 [m**2/s**3]
CTHR_TDOT = 0.0002 [m**2/s**3]
CTHR_NDOT = 0.0003 [m**2/s**3]
CTHR_DRG = 0.00001 [m**3/(kg*s**2)]
CTHR_SRP = 0.00002 [m**3/(kg*s**2)]
CTHR_THR = 0.000001 [m**2/s**4]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"#;
        let cdm = Cdm::from_kvn(kvn).expect("parse full cdm");
        let regenerated = cdm.to_kvn().expect("generate full kvn");
        let cdm2 = Cdm::from_kvn(&regenerated).expect("parse regenerated full cdm");

        // Verify key fields
        assert_eq!(cdm.header.message_for, cdm2.header.message_for);
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_frame,
            cdm2.body.relative_metadata_data.screen_volume_frame
        );
        assert!(cdm.body.segments[0].data.od_parameters.is_some());
        assert!(cdm2.body.segments[0].data.od_parameters.is_some());
        assert!(cdm.body.segments[0]
            .data
            .covariance_matrix
            .as_ref()
            .unwrap()
            .cthr_thr
            .is_some());
        assert!(cdm2.body.segments[0]
            .data
            .covariance_matrix
            .as_ref()
            .unwrap()
            .cthr_thr
            .is_some());
    }

    #[test]
    fn test_cdm_validation_segment_count() {
        // Construct a CDM with only 1 segment manually (hard to do via KVN since keys are repetitive)
        // But we can try to parse a truncated KVN or use the builder
        let kvn_one_seg = r#"
CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
# Only one object segment
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000
X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"#;
        assert!(Cdm::from_kvn(kvn_one_seg).is_err());
    }

    #[test]
    fn test_cdm_validation_probability_range() {
        // Probability > 1.0
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace(
            "COLLISION_PROBABILITY = 0.001",
            "COLLISION_PROBABILITY = 1.5",
        );
        assert!(Cdm::from_kvn(&kvn).is_err());
    }

    #[test]
    fn test_cdm_validation_segment_count_mismatch() {
        let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();

        // Remove one segment => 1 segment
        cdm.body.segments.pop();
        assert!(cdm.validate().is_err());

        // Add valid segments to check 3 segments (also invalid)
        let seg = cdm.body.segments[0].clone();
        cdm.body.segments.push(seg.clone());
        cdm.body.segments.push(seg); // Now 3
        assert_eq!(cdm.body.segments.len(), 3);
        assert!(cdm.validate().is_err());
    }

    #[test]
    fn test_cdm_validation_requires_object1_and_object2() {
        let mut cdm = Cdm::from_kvn(&sample_cdm_kvn()).unwrap();
        cdm.body.segments[1].metadata.object = CdmObjectType::Object1;
        assert!(cdm.validate().is_err());
    }
}
