// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, OpmCovarianceMatrix,
    RdmSpacecraftParameters, StateVector,
};
use crate::error::Result;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::{
    CalendarEpoch, ControlledType, DisintegrationType, Distance, ImpactUncertaintyType,
    ObjectDescription, ReentryUncertaintyMethodType, UserDefined, YesNo,
};
use serde::{Deserialize, Serialize};

/// Re-entry Data Message (RDM).
///
/// The RDM specifies a standard message format to be used in the exchange of spacecraft
/// re-entry information between Space Situational Awareness (SSA) or Space Surveillance and
/// Tracking (SST) data providers, satellite owners/operators, and other parties.
///
/// It includes data such as:
/// - Remaining orbital lifetime
/// - Start and end of the re-entry and impact windows
/// - Impact location and probabilities
/// - Object physical properties
///
/// **CCSDS Reference**: 508.1-B-1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "rdm")]
pub struct Rdm {
    pub header: RdmHeader,
    pub body: RdmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_RDM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "1.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Rdm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Rdm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Rdm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        validate_kvn_syntax(kvn)?;
        let rdm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&rdm)?;
        Ok(rdm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"rdm", "RDM")?;
        validate_xml_sequences(xml)?;
        let rdm: Self = crate::xml::from_str_with_context(xml, "RDM")?;
        crate::traits::Validate::validate(&rdm)?;
        Ok(rdm)
    }
}

impl Rdm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        if !self.body.segment.data.comment.is_empty() {
            return Err(crate::error::ValidationError::Generic {
                message: "RDM XML data-level COMMENT cannot be represented distinctly from the first KVN logical-block COMMENT".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    const HEADER: &[&[u8]] = &[b"COMMENT", b"CREATION_DATE", b"ORIGINATOR", b"MESSAGE_ID"];
    const METADATA: &[&[u8]] = &[
        b"COMMENT",
        b"OBJECT_NAME",
        b"INTERNATIONAL_DESIGNATOR",
        b"CATALOG_NAME",
        b"OBJECT_DESIGNATOR",
        b"OBJECT_TYPE",
        b"OBJECT_OWNER",
        b"OBJECT_OPERATOR",
        b"CONTROLLED_REENTRY",
        b"CENTER_NAME",
        b"TIME_SYSTEM",
        b"EPOCH_TZERO",
        b"REF_FRAME",
        b"REF_FRAME_EPOCH",
        b"EPHEMERIS_NAME",
        b"GRAVITY_MODEL",
        b"ATMOSPHERIC_MODEL",
        b"SOLAR_FLUX_PREDICTION",
        b"N_BODY_PERTURBATIONS",
        b"SOLAR_RAD_PRESSURE",
        b"EARTH_TIDES",
        b"INTRACK_THRUST",
        b"DRAG_PARAMETERS_SOURCE",
        b"DRAG_PARAMETERS_ALTITUDE",
        b"REENTRY_UNCERTAINTY_METHOD",
        b"REENTRY_DISINTEGRATION",
        b"IMPACT_UNCERTAINTY_METHOD",
        b"PREVIOUS_MESSAGE_ID",
        b"PREVIOUS_MESSAGE_EPOCH",
        b"NEXT_MESSAGE_EPOCH",
    ];
    const DATA: &[&[u8]] = &[
        b"COMMENT",
        b"atmosphericReentryParameters",
        b"groundImpactParameters",
        b"stateVector",
        b"covarianceMatrix",
        b"spacecraftParameters",
        b"odParameters",
        b"userDefinedParameters",
    ];
    const ATMOSPHERIC: &[&[u8]] = &[
        b"COMMENT",
        b"ORBIT_LIFETIME",
        b"REENTRY_ALTITUDE",
        b"ORBIT_LIFETIME_WINDOW_START",
        b"ORBIT_LIFETIME_WINDOW_END",
        b"NOMINAL_REENTRY_EPOCH",
        b"REENTRY_WINDOW_START",
        b"REENTRY_WINDOW_END",
        b"ORBIT_LIFETIME_CONFIDENCE_LEVEL",
    ];
    const GROUND: &[&[u8]] = &[
        b"COMMENT",
        b"PROBABILITY_OF_IMPACT",
        b"PROBABILITY_OF_BURN_UP",
        b"PROBABILITY_OF_BREAK_UP",
        b"PROBABILITY_OF_LAND_IMPACT",
        b"PROBABILITY_OF_CASUALTY",
        b"NOMINAL_IMPACT_EPOCH",
        b"IMPACT_WINDOW_START",
        b"IMPACT_WINDOW_END",
        b"IMPACT_REF_FRAME",
        b"NOMINAL_IMPACT_LON",
        b"NOMINAL_IMPACT_LAT",
        b"NOMINAL_IMPACT_ALT",
        b"IMPACT_1_CONFIDENCE",
        b"IMPACT_1_START_LON",
        b"IMPACT_1_START_LAT",
        b"IMPACT_1_STOP_LON",
        b"IMPACT_1_STOP_LAT",
        b"IMPACT_1_CROSS_TRACK",
        b"IMPACT_2_CONFIDENCE",
        b"IMPACT_2_START_LON",
        b"IMPACT_2_START_LAT",
        b"IMPACT_2_STOP_LON",
        b"IMPACT_2_STOP_LAT",
        b"IMPACT_2_CROSS_TRACK",
        b"IMPACT_3_CONFIDENCE",
        b"IMPACT_3_START_LON",
        b"IMPACT_3_START_LAT",
        b"IMPACT_3_STOP_LON",
        b"IMPACT_3_STOP_LAT",
        b"IMPACT_3_CROSS_TRACK",
    ];
    const STATE: &[&[u8]] = &[
        b"COMMENT", b"EPOCH", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT",
    ];
    const COVARIANCE: &[&[u8]] = &[
        b"COMMENT",
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
    const SPACECRAFT: &[&[u8]] = &[
        b"COMMENT",
        b"WET_MASS",
        b"DRY_MASS",
        b"HAZARDOUS_SUBSTANCES",
        b"SOLAR_RAD_AREA",
        b"SOLAR_RAD_COEFF",
        b"DRAG_AREA",
        b"DRAG_COEFF",
        b"RCS",
        b"BALLISTIC_COEFF",
        b"THRUST_ACCELERATION",
    ];
    const OD: &[&[u8]] = &[
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
    ];
    const USER_DEFINED: &[&[u8]] = &[b"COMMENT", b"USER_DEFINED"];

    fn in_sequence(child: &[u8], sequence: &[&[u8]]) -> Option<XmlSequenceRule> {
        rank_of(child, sequence)
            .map(|rank| XmlSequenceRule::new(rank, matches!(child, b"COMMENT" | b"USER_DEFINED")))
    }

    /// `userDefinedType` wraps its children in a repeating sequence, so a COMMENT may open a new
    /// iteration after a USER_DEFINED.
    fn in_repeating_sequence(child: &[u8], sequence: &[&[u8]]) -> Option<XmlSequenceRule> {
        rank_of(child, sequence).map(|rank| {
            XmlSequenceRule::restarting(rank, matches!(child, b"COMMENT" | b"USER_DEFINED"))
        })
    }

    fn rank_of(child: &[u8], sequence: &[&[u8]]) -> Option<u16> {
        sequence
            .iter()
            .position(|candidate| *candidate == child)
            .map(|rank| rank as u16)
    }

    crate::xml::validate_element_sequences(
        xml,
        "RDM",
        |parent, child| match parent {
            b"rdm" => in_sequence(child, &[b"header", b"body"]),
            b"header" => in_sequence(child, HEADER),
            b"body" => in_sequence(child, &[b"segment"]),
            b"segment" => in_sequence(child, &[b"metadata", b"data"]),
            b"metadata" => in_sequence(child, METADATA),
            b"data" => in_sequence(child, DATA),
            b"atmosphericReentryParameters" => in_sequence(child, ATMOSPHERIC),
            b"groundImpactParameters" => in_sequence(child, GROUND),
            b"stateVector" => in_sequence(child, STATE),
            b"covarianceMatrix" => in_sequence(child, COVARIANCE),
            b"spacecraftParameters" => in_sequence(child, SPACECRAFT),
            b"odParameters" => in_sequence(child, OD),
            b"userDefinedParameters" => in_repeating_sequence(child, USER_DEFINED),
            _ => None,
        },
        |element, attribute| match attribute {
            b"parameter" => element == b"USER_DEFINED",
            b"units" => matches!(
                element,
                b"DRAG_PARAMETERS_ALTITUDE"
                    | b"ORBIT_LIFETIME"
                    | b"REENTRY_ALTITUDE"
                    | b"ORBIT_LIFETIME_WINDOW_START"
                    | b"ORBIT_LIFETIME_WINDOW_END"
                    | b"ORBIT_LIFETIME_CONFIDENCE_LEVEL"
                    | b"NOMINAL_IMPACT_LON"
                    | b"NOMINAL_IMPACT_LAT"
                    | b"NOMINAL_IMPACT_ALT"
                    | b"IMPACT_1_CONFIDENCE"
                    | b"IMPACT_1_START_LON"
                    | b"IMPACT_1_START_LAT"
                    | b"IMPACT_1_STOP_LON"
                    | b"IMPACT_1_STOP_LAT"
                    | b"IMPACT_1_CROSS_TRACK"
                    | b"IMPACT_2_CONFIDENCE"
                    | b"IMPACT_2_START_LON"
                    | b"IMPACT_2_START_LAT"
                    | b"IMPACT_2_STOP_LON"
                    | b"IMPACT_2_STOP_LAT"
                    | b"IMPACT_2_CROSS_TRACK"
                    | b"IMPACT_3_CONFIDENCE"
                    | b"IMPACT_3_START_LON"
                    | b"IMPACT_3_START_LAT"
                    | b"IMPACT_3_STOP_LON"
                    | b"IMPACT_3_STOP_LAT"
                    | b"IMPACT_3_CROSS_TRACK"
                    | b"X"
                    | b"Y"
                    | b"Z"
                    | b"X_DOT"
                    | b"Y_DOT"
                    | b"Z_DOT"
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
                    | b"WET_MASS"
                    | b"DRY_MASS"
                    | b"SOLAR_RAD_AREA"
                    | b"DRAG_AREA"
                    | b"RCS"
                    | b"BALLISTIC_COEFF"
                    | b"THRUST_ACCELERATION"
                    | b"RECOMMENDED_OD_SPAN"
                    | b"ACTUAL_OD_SPAN"
                    | b"RESIDUALS_ACCEPTED"
            ),
            _ => false,
        },
    )
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const KEYS: &[&str] = &[
        "CCSDS_RDM_VERS",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_DESIGNATOR",
        "OBJECT_TYPE",
        "OBJECT_OWNER",
        "OBJECT_OPERATOR",
        "CONTROLLED_REENTRY",
        "CENTER_NAME",
        "TIME_SYSTEM",
        "EPOCH_TZERO",
        "REF_FRAME",
        "REF_FRAME_EPOCH",
        "EPHEMERIS_NAME",
        "GRAVITY_MODEL",
        "ATMOSPHERIC_MODEL",
        "SOLAR_FLUX_PREDICTION",
        "N_BODY_PERTURBATIONS",
        "SOLAR_RAD_PRESSURE",
        "EARTH_TIDES",
        "INTRACK_THRUST",
        "DRAG_PARAMETERS_SOURCE",
        "DRAG_PARAMETERS_ALTITUDE",
        "REENTRY_UNCERTAINTY_METHOD",
        "REENTRY_DISINTEGRATION",
        "IMPACT_UNCERTAINTY_METHOD",
        "PREVIOUS_MESSAGE_ID",
        "PREVIOUS_MESSAGE_EPOCH",
        "NEXT_MESSAGE_EPOCH",
        "ORBIT_LIFETIME",
        "REENTRY_ALTITUDE",
        "ORBIT_LIFETIME_WINDOW_START",
        "ORBIT_LIFETIME_WINDOW_END",
        "NOMINAL_REENTRY_EPOCH",
        "REENTRY_WINDOW_START",
        "REENTRY_WINDOW_END",
        "ORBIT_LIFETIME_CONFIDENCE_LEVEL",
        "PROBABILITY_OF_IMPACT",
        "PROBABILITY_OF_BURN_UP",
        "PROBABILITY_OF_BREAK_UP",
        "PROBABILITY_OF_LAND_IMPACT",
        "PROBABILITY_OF_CASUALTY",
        "NOMINAL_IMPACT_EPOCH",
        "IMPACT_WINDOW_START",
        "IMPACT_WINDOW_END",
        "IMPACT_REF_FRAME",
        "NOMINAL_IMPACT_LON",
        "NOMINAL_IMPACT_LAT",
        "NOMINAL_IMPACT_ALT",
        "IMPACT_1_CONFIDENCE",
        "IMPACT_1_START_LON",
        "IMPACT_1_START_LAT",
        "IMPACT_1_STOP_LON",
        "IMPACT_1_STOP_LAT",
        "IMPACT_1_CROSS_TRACK",
        "IMPACT_2_CONFIDENCE",
        "IMPACT_2_START_LON",
        "IMPACT_2_START_LAT",
        "IMPACT_2_STOP_LON",
        "IMPACT_2_STOP_LAT",
        "IMPACT_2_CROSS_TRACK",
        "IMPACT_3_CONFIDENCE",
        "IMPACT_3_START_LON",
        "IMPACT_3_START_LAT",
        "IMPACT_3_STOP_LON",
        "IMPACT_3_STOP_LAT",
        "IMPACT_3_CROSS_TRACK",
        "EPOCH",
        "X",
        "Y",
        "Z",
        "X_DOT",
        "Y_DOT",
        "Z_DOT",
        "COV_REF_FRAME",
        "CX_X",
        "CY_X",
        "CY_Y",
        "CZ_X",
        "CZ_Y",
        "CZ_Z",
        "CX_DOT_X",
        "CX_DOT_Y",
        "CX_DOT_Z",
        "CX_DOT_X_DOT",
        "CY_DOT_X",
        "CY_DOT_Y",
        "CY_DOT_Z",
        "CY_DOT_X_DOT",
        "CY_DOT_Y_DOT",
        "CZ_DOT_X",
        "CZ_DOT_Y",
        "CZ_DOT_Z",
        "CZ_DOT_X_DOT",
        "CZ_DOT_Y_DOT",
        "CZ_DOT_Z_DOT",
        "WET_MASS",
        "DRY_MASS",
        "HAZARDOUS_SUBSTANCES",
        "SOLAR_RAD_AREA",
        "SOLAR_RAD_COEFF",
        "DRAG_AREA",
        "DRAG_COEFF",
        "RCS",
        "BALLISTIC_COEFF",
        "THRUST_ACCELERATION",
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

    fn rank(key: &str) -> Option<u16> {
        if key.starts_with("USER_DEFINED_") {
            return Some(KEYS.len() as u16);
        }
        KEYS.iter()
            .position(|candidate| *candidate == key)
            .map(|rank| rank as u16)
    }

    fn group(rank: u16) -> u8 {
        match rank {
            0 => 0,
            1..=3 => 1,
            4..=32 => 2,
            33..=40 => 3,
            41..=70 => 4,
            71..=77 => 5,
            78..=99 => 6,
            100..=109 => 7,
            110..=119 => 8,
            _ => 9,
        }
    }

    fn comments_start_block(previous: u16, key: &str) -> bool {
        let Some(current) = rank(key) else {
            return false;
        };
        match current {
            1 => previous == 0,
            4 => previous == 3,
            33 => group(previous) == 2,
            _ => group(current) >= 4 && group(previous) < group(current),
        }
    }

    crate::kvn::strict::validate_odm_assignments(
        kvn,
        &crate::kvn::strict::OdmAssignmentRules {
            context: "while validating RDM KVN structure",
            message_name: "RDM",
            rank,
            comment_starts_block: comments_start_block,
            allows_non_increasing: |previous, current| {
                current.rank == KEYS.len() as u16
                    && previous.rank == current.rank
                    && current.key.starts_with("USER_DEFINED_")
            },
        },
    )
}

impl ToKvn for Rdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_RDM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Header
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct RdmHeader {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// File creation date and time in UTC.
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23
    pub creation_date: CalendarEpoch,
    /// Creating agency or entity.
    ///
    /// Examples: DLR, ESA
    #[builder(into)]
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator.
    ///
    /// Examples: 201113719185, ESA20190101-3345
    #[builder(into)]
    pub message_id: String,
}

impl crate::traits::Validate for RdmHeader {
    fn validate(&self) -> Result<()> {
        if self.creation_date.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        if self.originator.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.message_id.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Header".into(),
                field: "MESSAGE_ID".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for RdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The RDM Body consists of a single segment.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct RdmBody {
    pub segment: Box<RdmSegment>,
}

impl crate::traits::Validate for RdmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }
}

impl ToKvn for RdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct RdmSegment {
    /// The metadata for this RDM segment.
    pub metadata: RdmMetadata,
    /// The data for this RDM segment.
    pub data: RdmData,
}

impl crate::traits::Validate for RdmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        if self.data.state_vector.is_some()
            && self
                .metadata
                .ref_frame
                .as_deref()
                .map(str::trim)
                .is_none_or(str::is_empty)
        {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "REF_FRAME (required when state vector is provided)".into(),
                line: None,
            }
            .into());
        }
        self.data.validate()
    }
}

impl ToKvn for RdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// The RDM Metadata provides information about the re-entry event.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct RdmMetadata {
    /// Comments (allowed only at the beginning of RDM metadata).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Object name for which the orbit state is provided. There is no CCSDS-based restriction
    /// on the value for this keyword, but it is recommended to use names from the UNOOSA
    /// registry—reference `[7]`, which includes object name and international designator of the
    /// participant (formatting rules specified in 5.2.3.3). For objects that are not in the
    /// UNOOSA registry, either a descriptive name (e.g., DEBRIS, if the object is identified as
    /// space debris) or UNKNOWN should be used.
    ///
    /// **Examples**: SENTINEL-1A, GOCE, ENVISAT, BRIZ R/B, DEBRIS, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[builder(into)]
    pub object_name: String,
    /// The full international designator (COSPAR ID) for the object. Values shall have the
    /// format YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of
    /// launch (with leading zeros); P{PP} = at least one capital letter for the identification
    /// of the part brought into space by the launch. In cases where the object has no
    /// international designator, the value UNKNOWN should be used (formatting rules specified
    /// in 5.2.3.3).
    ///
    /// **Examples**: 2010-012C, 2016-001A, 1985-067CD, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[builder(into)]
    pub international_designator: String,
    /// The satellite catalog used for the object (formatting rules specified in 5.2.3.3). The
    /// name should be taken from the appropriate SANA registry for catalog names, reference
    /// `[8]`.
    ///
    /// **Examples**: SATCAT, ESA SST
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub catalog_name: Option<String>,
    /// The CATALOG_NAME satellite catalog designator for the object (formatting rules
    /// specified in 5.2.3.3).
    ///
    /// **Examples**: 37451, 125387U
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub object_designator: Option<String>,
    /// The object type.
    ///
    /// **Examples**: PAYLOAD, ROCKET BODY, DEBRIS, OTHER, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub object_type: Option<ObjectDescription>,
    /// Owner of the object (e.g., company, agency, or country owning the satellite). The value
    /// should be taken from the abbreviation column in the SANA organizations registry,
    /// reference `[6]`.
    ///
    /// **Examples**: DLR, INTELSAT, ESA, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub object_owner: Option<String>,
    /// Operator of the object (e.g., company, agency, or country operating the satellite).
    /// The value should be taken from the abbreviation column in the SANA organizations
    /// registry, reference `[6]`.
    ///
    /// **Examples**: ESA, EUMETSAT
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub object_operator: Option<String>,
    /// Specification of whether the re-entry is controlled or not.
    ///
    /// **Examples**: YES, NO, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    pub controlled_reentry: ControlledType,
    /// Celestial body orbited by the object and origin of the reference frame, which may be a
    /// natural solar system body (planets, asteroids, comets, and natural satellites),
    /// including any planet barycenter or the solar system barycenter. The value should be
    /// taken from the orbit center column in the SANA orbit centers registry, reference `[9]`.
    ///
    /// **Examples**: EARTH, MOON, JUPITER
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[builder(into)]
    pub center_name: String,
    /// Time system for all data/metadata. The value should be taken from the name column in
    /// the SANA time systems registry, reference `[10]`.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[builder(into)]
    pub time_system: String,
    /// Epoch from which the ORBIT_LIFETIME is calculated (formatting rules specified in
    /// 5.3.3.5).
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    pub epoch_tzero: CalendarEpoch,
    /// Reference frame in which the (optional) orbit information will be provided. The value
    /// should be taken from the keyword value name column in the SANA celestial body reference
    /// frames registry, reference `[11]`. The reference frame must be the same for all orbit
    /// data elements, with the exception of the covariance matrix, for which a different
    /// reference frame may be specified, and the ground impact data. This keyword becomes
    /// mandatory if state vectors are provided in the data section.
    ///
    /// **Examples**: ITRF-97, EME2000, ICRF
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub ref_frame: Option<String>,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame
    /// (formatting rules specified in 5.3.3.5).
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_epoch: Option<CalendarEpoch>,
    /// Unique identifier of an external ephemeris file used or NONE.
    ///
    /// **Examples**: NONE, EPHEMERIS, INTELSAT2
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub ephemeris_name: Option<String>,
    /// The gravity model used in the simulation. The degree (D) and order (O) of the spherical
    /// harmonic coefficients applied should be given along with the name of the model.
    ///
    /// **Examples**: EGM-96: 36D 36O, JGM-2: 41D 41O
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub gravity_model: Option<String>,
    /// The atmosphere model(s) used in the simulation. If more than one model is used they
    /// should be listed on the same line and separated by a comma.
    ///
    /// **Examples**: MSIS, JACCHIA 70, MSISE-90, NRLMSISE-00
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub atmospheric_model: Option<String>,
    /// The method used to predict the solar flux and geomagnetic indices.
    ///
    /// **Examples**: STOCHASTIC, PREDICTED: MLLRT
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub solar_flux_prediction: Option<String>,
    /// Comma separated list of other bodies used in the simulation. The names of the bodies
    /// should be taken from the SANA registry for orbit centers, reference `[9]`. If no other
    /// bodies are used in the simulation, the value should be NONE.
    ///
    /// **Examples**: MOON, SUN, JUPITER, NONE
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub n_body_perturbations: Option<String>,
    /// Model used for the solar radiation pressure: either model name, or NO if solar
    /// radiation pressure was not modelled.
    ///
    /// **Examples**: GSPM04, NO
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub solar_rad_pressure: Option<String>,
    /// Model used for solid Earth and ocean tides: either model name, or NO if tides were not
    /// modelled.
    ///
    /// **Examples**: ESR, NO
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub earth_tides: Option<String>,
    /// Indicator on whether in-track thrust modeling was used in the simulation.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub intrack_thrust: Option<YesNo>,
    /// The method used to estimate the drag parameters of the object (DRAG_AREA, DRAG_COEFF,
    /// and/or BALLISTIC_COEFF).
    ///
    /// **Examples**: DESIGN, CFD: TOOL1, CFD DMSCFOAM, OD
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub drag_parameters_source: Option<String>,
    /// The altitude (in km) at which the object drag parameters (DRAG_AREA, DRAG_COEFF, and/or
    /// BALLISTIC_COEFF) are valid. The units shall be kilometers, and the conventions
    /// specified in 5.2.4.1 and 5.3.4 must be followed.
    ///
    /// **Examples**: 200 `[km]`, 175 `[km]`
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub drag_parameters_altitude: Option<Distance>,
    /// The method used to determine the orbit lifetime uncertainty or the re-entry windows.
    ///
    /// **Examples**: NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub reentry_uncertainty_method: Option<ReentryUncertaintyMethodType>,
    /// The aspects of disintegration during re-entry considered during simulations: none (the
    /// object was treated as a point mass), mass loss, break-ups (including explosion), or
    /// both. It is a coarse indication on whether the impact area in the data covers potential
    /// fragments as well.
    ///
    /// **Examples**: NONE, MASS-LOSS, BREAK-UP, MASS-LOSS + BREAK-UP
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub reentry_disintegration: Option<DisintegrationType>,
    /// The method used to determine the impact location confidence interval(s).
    ///
    /// **Examples**: NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub impact_uncertainty_method: Option<ImpactUncertaintyType>,
    /// ID of the previous RDM issued for this object.
    ///
    /// **Examples**: ESA/2015-563892348
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub previous_message_id: Option<String>,
    /// UTC Epoch of the previous RDM issued for this object (formatting rules specified in
    /// 5.3.3.5).
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub previous_message_epoch: Option<CalendarEpoch>,
    /// Scheduled UTC epoch of the next RDM for the same object (formatting rules specified in
    /// 5.3.3.5); N/A if no other message is scheduled.
    ///
    /// **Examples**: 2001-11-06T11:17:33, N/A
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub next_message_epoch: Option<CalendarEpoch>,
}

impl ToKvn for RdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(ref v) = self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        if let Some(v) = &self.object_owner {
            writer.write_pair("OBJECT_OWNER", v);
        }
        if let Some(v) = &self.object_operator {
            writer.write_pair("OBJECT_OPERATOR", v);
        }
        writer.write_pair("CONTROLLED_REENTRY", format!("{}", self.controlled_reentry));
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.ref_frame {
            writer.write_pair("REF_FRAME", v);
        }
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.ephemeris_name {
            writer.write_pair("EPHEMERIS_NAME", v);
        }
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.solar_flux_prediction {
            writer.write_pair("SOLAR_FLUX_PREDICTION", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", v);
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", v);
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", format!("{}", v));
        }
        if let Some(v) = &self.drag_parameters_source {
            writer.write_pair("DRAG_PARAMETERS_SOURCE", v);
        }
        if let Some(v) = &self.drag_parameters_altitude {
            writer.write_pair("DRAG_PARAMETERS_ALTITUDE", v);
        }
        if let Some(v) = &self.reentry_uncertainty_method {
            writer.write_pair("REENTRY_UNCERTAINTY_METHOD", v.to_string());
        }
        if let Some(v) = &self.reentry_disintegration {
            writer.write_pair("REENTRY_DISINTEGRATION", format!("{}", v));
        }
        if let Some(v) = &self.impact_uncertainty_method {
            writer.write_pair("IMPACT_UNCERTAINTY_METHOD", v.to_string());
        }
        if let Some(v) = &self.previous_message_id {
            writer.write_pair("PREVIOUS_MESSAGE_ID", v);
        }
        if let Some(v) = &self.previous_message_epoch {
            writer.write_pair("PREVIOUS_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.next_message_epoch {
            writer.write_pair("NEXT_MESSAGE_EPOCH", v);
        }
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// The RDM Data section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct RdmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Atmospheric re-entry parameters.
    #[serde(rename = "atmosphericReentryParameters")]
    pub atmospheric_reentry_parameters: AtmosphericReentryParameters,
    /// Ground impact parameters.
    #[serde(
        rename = "groundImpactParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ground_impact_parameters: Option<GroundImpactParameters>,
    /// State vector.
    #[serde(
        rename = "stateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_vector: Option<StateVector>,
    /// Position/velocity covariance matrix.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
    /// Spacecraft parameters.
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<RdmSpacecraftParameters>,
    /// Orbit determination parameters.
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    /// User defined parameters.
    #[serde(
        rename = "userDefinedParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_parameters: Option<UserDefined>,
}

impl crate::traits::Validate for RdmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.international_designator.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "INTERNATIONAL_DESIGNATOR".into(),
                line: None,
            }
            .into());
        }
        if self.center_name.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "CENTER_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        if self.epoch_tzero.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Metadata".into(),
                field: "EPOCH_TZERO".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl crate::traits::Validate for RdmData {
    fn validate(&self) -> Result<()> {
        if self.covariance_matrix.is_some() && self.state_vector.is_none() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "RDM Data".into(),
                field: "stateVector (required when covarianceMatrix is provided)".into(),
                line: None,
            }
            .into());
        }
        let arp = &self.atmospheric_reentry_parameters;
        if let (Some(start), Some(end)) = (
            &arp.orbit_lifetime_window_start,
            &arp.orbit_lifetime_window_end,
        ) {
            if start.value > end.value {
                return Err(crate::error::ValidationError::Generic {
                    message: "ORBIT_LIFETIME_WINDOW_START must be <= ORBIT_LIFETIME_WINDOW_END"
                        .into(),
                    line: None,
                }
                .into());
            }
        }
        if let Some(state_vector) = &self.state_vector {
            state_vector.validate()?;
        }
        Ok(())
    }
}

impl RdmData {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }

    fn write_kvn(&self, writer: &mut KvnWriter) {
        // No DATA_START
        writer.write_comments(&self.comment);
        // Atmospheric (mandatory)
        self.atmospheric_reentry_parameters.write_kvn(writer);

        // Ground impact (optional)
        if let Some(g) = &self.ground_impact_parameters {
            g.write_kvn(writer);
        }

        // Optional blocks: write when present
        if let Some(sv) = &self.state_vector {
            sv.write_kvn(writer);
        }
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.wet_mass {
                writer.write_measure("WET_MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.dry_mass {
                writer.write_measure("DRY_MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.hazardous_substances {
                writer.write_pair("HAZARDOUS_SUBSTANCES", v);
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_measure("SOLAR_RAD_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_pair("SOLAR_RAD_COEFF", v);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_measure("DRAG_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_pair("DRAG_COEFF", v);
            }
            if let Some(v) = &sp.rcs {
                writer.write_measure("RCS", &v.to_unit_value());
            }
            if let Some(v) = &sp.ballistic_coeff {
                writer.write_measure("BALLISTIC_COEFF", v);
            }
            if let Some(v) = &sp.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", &v.to_unit_value());
            }
        }
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

        if let Some(ud) = &self.user_defined_parameters {
            writer.write_comments(&ud.comment);
            for p in &ud.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kitchen_sink_roundtrip() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = COMPREHENSIVE_TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T09:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2023-01-06T19:45:33
REENTRY_WINDOW_START = 2023-01-06T11:45:33
REENTRY_WINDOW_END = 2023-01-06T22:12:56
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.75
EPOCH = 2023-01-01T09:30:12
X = 4000.000000 [km]
Y = 4000.000000 [km]
Z = 4000.000000 [km]
X_DOT = 7.000000 [km/s]
Y_DOT = 7.000000 [km/s]
Z_DOT = 7.000000 [km/s]
COV_REF_FRAME = RTN
CX_X = 0.10000 [km**2]
CY_X = 0.10000 [km**2]
CY_Y = 0.10000 [km**2]
CZ_X = 0.10000 [km**2]
CZ_Y = 0.10000 [km**2]
CZ_Z = 0.10000 [km**2]
CX_DOT_X = 0.02000 [km**2/s]
CX_DOT_Y = 0.02000 [km**2/s]
CX_DOT_Z = 0.02000 [km**2/s]
CX_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_X = 0.02000 [km**2/s]
CY_DOT_Y = 0.02000 [km**2/s]
CY_DOT_Z = 0.02000 [km**2/s]
CY_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
CZ_DOT_X = 0.02000 [km**2/s]
CZ_DOT_Y = 0.02000 [km**2/s]
CZ_DOT_Z = 0.02000 [km**2/s]
CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
WET_MASS = 3582 [kg]
DRAG_AREA = 23.3565 [m**2]
DRAG_COEFF = 2.2634
ACTUAL_OD_SPAN = 3.4554 [d]
TRACKS_AVAILABLE = 18
TRACKS_USED = 17
USER_DEFINED_TEST = VALUE
"#;
        let rdm = Rdm::from_kvn(kvn).expect("parse kvn");
        let generated = rdm.to_kvn().expect("generate kvn");
        let rdm2 = Rdm::from_kvn(&generated).expect("parse generated kvn");

        assert_eq!(rdm.header, rdm2.header);
        assert_eq!(
            rdm.body.segment.metadata.object_name,
            rdm2.body.segment.metadata.object_name
        );
        assert_eq!(
            rdm.body.segment.data.user_defined_parameters,
            rdm2.body.segment.data.user_defined_parameters
        );
    }

    /// Parse official RDM XML example C-3 (minimal)
    #[test]
    fn test_xsd_rdm_sample_c3_xml() {
        let xml = std::fs::read_to_string("data/xml/rdm_c3.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    }

    /// Parse official RDM XML example C-4 (comprehensive)
    #[test]
    fn test_xsd_rdm_sample_c4_xml() {
        let xml = std::fs::read_to_string("data/xml/rdm_c4.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.header.message_id, "ESA/20180422-001");
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        assert!(rdm.body.segment.data.state_vector.is_some());
        assert!(rdm.body.segment.data.covariance_matrix.is_some());
        assert!(rdm.body.segment.data.spacecraft_parameters.is_some());
        assert!(rdm.body.segment.data.od_parameters.is_some());
    }

    #[test]
    fn test_rdm_xml_roundtrip_minimal() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let xml = rdm.to_xml().unwrap();
        assert!(xml.contains("<rdm"));
        assert!(xml.contains("OBJECT_NAME"));
        let rdm2 = Rdm::from_xml(&xml).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.object_name,
            rdm2.body.segment.metadata.object_name
        );
    }

    #[test]
    fn test_rdm_validation_orbit_lifetime_window() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_WINDOW_START = 6.0 [d]
ORBIT_LIFETIME_WINDOW_END = 5.0 [d]
"#;
        assert!(Rdm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_rdm_validation_missing_ref_frame_when_state_vector_present() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 1.0 [km/s]
"#;
        assert!(Rdm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_rdm_validation_covariance_requires_state_vector() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
CX_X = 0.1 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 0.1 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 0.1 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 0.01 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 0.01 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 0.01 [km**2/s**2]
"#;
        assert!(Rdm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_rdm_validation_empty_object_name() {
        // Construct with empty OBJECT_NAME
        // Note: parser might not allow empty value for key, but if it does (e.g. "OBJECT_NAME = \n"), validation should catch it.
        // However, if parser treats empty value as error, then this test tests parser, which is fine.
        // But if we construct struct manually and call validate, that's what we really want to test if parser is loose.
        // For now, let's use KVN.
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME =
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        // If parser allows empty value, validate() catches it.
        // If parser disallows, it errors anyway.
        assert!(Rdm::from_kvn(kvn).is_err());
    }
}
