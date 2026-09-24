// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

mod kvn;
mod xml;

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters};
use crate::error::{EnumParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::traits::{Ndm, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;
mod tle;

//----------------------------------------------------------------------
// OMM Specific Units
//----------------------------------------------------------------------

// 1/ER (Inverse Earth Radii) for BSTAR
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum InvErUnits {
    #[serde(rename = "1/ER")]
    #[default]
    InvEr,
}
impl std::fmt::Display for InvErUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1/ER")
    }
}
impl FromStr for InvErUnits {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1/ER" => Ok(InvErUnits::InvEr),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "1/ER",
            }),
        }
    }
}
pub type BStar = UnitValue<f64, InvErUnits>;

// rev/day for MEAN_MOTION
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDayUnits {
    #[serde(rename = "rev/day", alias = "REV/DAY")]
    #[default]
    RevPerDay,
}
impl std::fmt::Display for RevPerDayUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rev/day")
    }
}
impl FromStr for RevPerDayUnits {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day" | "REV/DAY" => Ok(RevPerDayUnits::RevPerDay),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day or REV/DAY",
            }),
        }
    }
}
pub type MeanMotion = UnitValue<f64, RevPerDayUnits>;

// rev/day**2 for MEAN_MOTION_DOT
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDay2Units {
    #[serde(rename = "rev/day**2", alias = "REV/DAY**2")]
    #[default]
    RevPerDay2,
}
impl std::fmt::Display for RevPerDay2Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rev/day**2")
    }
}
impl FromStr for RevPerDay2Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**2" | "REV/DAY**2" => Ok(RevPerDay2Units::RevPerDay2),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day**2 or REV/DAY**2",
            }),
        }
    }
}
pub type MeanMotionDot = UnitValue<f64, RevPerDay2Units>;

// rev/day**3 for MEAN_MOTION_DDOT
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDay3Units {
    #[serde(rename = "rev/day**3", alias = "REV/DAY**3")]
    #[default]
    RevPerDay3,
}
impl std::fmt::Display for RevPerDay3Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rev/day**3")
    }
}
impl FromStr for RevPerDay3Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**3" | "REV/DAY**3" => Ok(RevPerDay3Units::RevPerDay3),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day**3 or REV/DAY**3",
            }),
        }
    }
}
pub type MeanMotionDDot = UnitValue<f64, RevPerDay3Units>;

//----------------------------------------------------------------------
// Root OMM Structure
//----------------------------------------------------------------------

/// Orbit Mean-Elements Message (OMM).
///
/// The OMM contains the orbital characteristics of a single object at a specified epoch,
/// expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
/// ascension of ascending node, argument of perigee, and mean anomaly.
///
/// These elements are adequate for providing the initial mean state of analytical and
/// semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
/// be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
/// of heritage users.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "omm")]
pub struct Omm {
    pub header: OdmHeader,
    pub body: OmmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_OMM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "3.0".to_string(), into)]
    pub version: String,
}

/// Optional overrides when constructing a minimal OMM from TLE lines.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TleToOmmOptions {
    /// Spacecraft name to use in generated metadata. Defaults to `UNKNOWN`.
    pub object_name: Option<String>,
    /// Object identifier to use in generated metadata.
    /// Defaults to the international designator derived from TLE line 1.
    pub object_id: Option<String>,
    /// Originator value for the generated ODM header. Defaults to `UNKNOWN`.
    pub originator: Option<String>,
    /// Optional message ID for the generated ODM header.
    pub message_id: Option<String>,
    /// Optional creation date override for the generated ODM header.
    /// Defaults to the parsed TLE epoch.
    pub creation_date: Option<CalendarEpoch>,
}

impl crate::traits::Validate for Omm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Omm,
            &self.id,
            &self.version,
        )?;
        crate::versioning::validate_omm_edition(self)?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Omm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        kvn::validate_kvn_syntax(kvn)?;
        let omm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&omm)?;
        Ok(omm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        xml::validate_xml_sequences(xml)?;
        let omm: Self = crate::xml::from_str_with_context(xml, "OMM")?;
        crate::traits::Validate::validate(&omm)?;
        Ok(omm)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OmmBody {
    #[serde(rename = "segment")]
    pub segment: OmmSegment,
}

impl crate::traits::Validate for OmmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OmmSegment {
    pub metadata: OmmMetadata,
    pub data: OmmData,
}

impl crate::traits::Validate for OmmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate_with_metadata(&self.metadata)
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// Metadata for the OMM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct OmmMetadata {
    /// Comments (allowed at the beginning of the OMM Metadata). (See 7.8 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which mean element orbit state data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use names
    /// from the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
    /// Object name and international designator of the participant). If OBJECT_NAME is not
    /// listed in reference `[3]` or the content is either unknown or cannot be disclosed, the
    /// value should be set to UNKNOWN.
    ///
    /// **Examples**: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Object identifier of the object for which mean element orbit state data is provided.
    /// While there is no CCSDS-based restriction on the value for this keyword, it is
    /// recommended to use the international spacecraft designator as published in the UN Office
    /// of Outer Space Affairs designator index (reference `[3]`). Recommended values have the
    /// format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of
    /// launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the
    /// identification of the part brought into space by the launch. If the asset is not listed
    /// in reference `[3]`, the UN Office of Outer Space Affairs designator index format is not
    /// used, or the content is either unknown or cannot be disclosed, the value should be set
    /// to UNKNOWN.
    ///
    /// **Examples**: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Origin of the OMM reference frame, which shall be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. Natural bodies shall be selected from the accepted set of values
    /// indicated in annex B, subsection B2.
    ///
    /// **Examples**: EARTH, MARS, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub center_name: String,
    /// Reference frame in which the Keplerian element data are given. Use of values other than
    /// those in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and
    /// corresponding Simplified General Perturbations (SGP) orbit propagator ephemeris outputs
    /// are explicitly defined to be in the True Equator Mean Equinox of Date (TEME of Date)
    /// reference frame. Therefore, TEME of date shall be used for OMMs based on NORAD Two Line
    /// Element sets, rather than the almost imperceptibly different TEME of Epoch (see
    /// reference `[H2]` or `[H3]` for further details).
    ///
    /// **Examples**: ICRF, ITRF2000, EME2000, TEME
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub ref_frame: String,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame.
    /// (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<CalendarEpoch>,
    /// Time system used for Keplerian elements and covariance data. Use of values other than
    /// those in 3.2.3.2 should be documented in an ICD.
    ///
    /// **Examples**: UTC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub time_system: String,
    /// Description of the Mean Element Theory. Indicates the proper method to employ to
    /// propagate the state.
    ///
    /// **Examples**: SGP, SGP4, SGP4-XP, DSST, USM
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub mean_element_theory: String,
}

impl crate::traits::Validate for OmmMetadata {
    fn validate(&self) -> Result<()> {
        crate::validation::epoch_precision(&[("REF_FRAME_EPOCH", &self.ref_frame_epoch)])?;
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.center_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "CENTER_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "REF_FRAME".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        if self.mean_element_theory.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "MEAN_ELEMENT_THEORY".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// OMM Data section.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct OmmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Mean Keplerian Elements in the Specified Reference Frame.
    #[serde(rename = "meanElements")]
    pub mean_elements: MeanElements,
    /// Spacecraft Parameters.
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<SpacecraftParameters>,
    /// TLE Related Parameters (Only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
    #[serde(
        rename = "tleParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tle_parameters: Option<TleParameters>,
    /// Position/Velocity Covariance Matrix (6x6 Lower Triangular Form).
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
    /// User-Defined Parameters.
    #[serde(
        rename = "userDefinedParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_parameters: Option<UserDefined>,
}

impl crate::traits::Validate for OmmData {
    fn validate(&self) -> Result<()> {
        crate::validation::validate_at_field_path(
            self.mean_elements.validate(),
            "body.segment.data.mean_elements",
        )?;
        if let Some(parameters) = &self.spacecraft_parameters {
            crate::validation::validate_at_field_path(
                parameters.validate(),
                "body.segment.data.spacecraft_parameters",
            )?;
        }
        if let Some(tle) = &self.tle_parameters {
            crate::validation::validate_at_field_path(
                tle.validate_values(),
                "body.segment.data.tle_parameters",
            )?;
        }
        if let Some(covariance) = &self.covariance_matrix {
            crate::validation::validate_at_field_path(
                covariance.validate(),
                "body.segment.data.covariance_matrix",
            )?;
        }
        Ok(())
    }
}

impl OmmData {
    pub fn validate_with_metadata(&self, metadata: &OmmMetadata) -> Result<()> {
        let theory = metadata.mean_element_theory.as_str();

        self.validate()?;

        // 1. Validate TLE Parameters presence based on theory
        match theory {
            "SGP" | "SGP4" | "PPT3" | "SGP4-XP" => {
                let tle =
                    self.tle_parameters
                        .as_ref()
                        .ok_or(ValidationError::MissingRequiredField {
                            block: Cow::Borrowed("OMM Data"),
                            field: Cow::Borrowed("TLE_PARAMETERS"),
                            line: None,
                        })?;
                tle.validate(theory)?;
            }
            _ => {
                if let Some(tle) = &self.tle_parameters {
                    tle.validate(theory)?;
                }
            }
        }

        // 2. Validate Mean Motion vs Semi Major Axis
        // If SGP/SGP4, MEAN_MOTION is preferred/required.
        if matches!(theory, "SGP" | "SGP4") && self.mean_elements.mean_motion.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("Mean Elements"),
                field: Cow::Borrowed("MEAN_MOTION"),
                line: None,
            }
            .into());
        }

        Ok(())
    }
}

//----------------------------------------------------------------------
// Mean Elements
//----------------------------------------------------------------------

/// Mean Keplerian Elements in the Specified Reference Frame.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct MeanElements {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of Mean Keplerian elements (see 7.5.10 for formatting rules)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub epoch: CalendarEpoch,
    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// **Examples**: 28594.4
    ///
    /// **Units**: km or rev/day
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semi_major_axis: Option<Distance>,
    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// **Examples**: 1.491325
    ///
    /// **Units**: km or rev/day
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion: Option<MeanMotion>,
    /// Eccentricity
    ///
    /// **Examples**: 0.7303
    ///
    /// **Units**: n/a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub eccentricity: NonNegativeDouble,
    /// Inclination
    ///
    /// **Examples**: 63.4
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub inclination: Inclination,
    /// Right ascension of ascending node
    ///
    /// **Examples**: 345.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub ra_of_asc_node: Angle,
    /// Argument of pericenter
    ///
    /// **Examples**: 270.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub arg_of_pericenter: Angle,
    /// Mean anomaly
    ///
    /// **Examples**: 130.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub mean_anomaly: Angle,
    /// Gravitational Coefficient (Gravitational Constant × Central Mass)
    ///
    /// **Examples**: 398600.44
    ///
    /// **Units**: km³/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gm: Option<Gm>,
}

/// Reject a value the OMM schema types as a double but that is not a real number.
///
/// Range facets such as `nonNegativeDouble` are expressed as comparisons, and every comparison
/// against NaN is false, so a NaN passes them. Finiteness therefore has to be checked in its own
/// right rather than inferred from a range check.
fn finite(field: &'static str, value: f64) -> Result<()> {
    if value.is_finite() {
        return Ok(());
    }
    Err(ValidationError::InvalidValue {
        field: field.into(),
        value: value.to_string(),
        expected: "a finite number".into(),
        line: None,
    }
    .into())
}

impl crate::traits::Validate for MeanElements {
    fn validate(&self) -> Result<()> {
        crate::validation::epoch_precision(&[("EPOCH", &self.epoch)])?;
        match (self.semi_major_axis.is_some(), self.mean_motion.is_some()) {
            (true, false) | (false, true) => {}
            _ => {
                return Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "Mean Elements must have exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION",
                    ),
                    line: None,
                }
                .into())
            }
        }

        if let Some(value) = &self.semi_major_axis {
            finite("SEMI_MAJOR_AXIS", value.value)?;
        }
        if let Some(value) = &self.mean_motion {
            finite("MEAN_MOTION", value.value)?;
        }
        finite("ECCENTRICITY", self.eccentricity.value)?;
        if self.eccentricity.value < 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "ECCENTRICITY".into(),
                value: self.eccentricity.value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }

        let inclination = self.inclination.angle.value;
        finite("INCLINATION", inclination)?;
        if !(0.0..=180.0).contains(&inclination) {
            return Err(ValidationError::OutOfRange {
                name: "INCLINATION".into(),
                value: inclination.to_string(),
                expected: "[0, 180]".into(),
                line: None,
            }
            .into());
        }

        for (field, angle) in [
            ("RA_OF_ASC_NODE", &self.ra_of_asc_node),
            ("ARG_OF_PERICENTER", &self.arg_of_pericenter),
            ("MEAN_ANOMALY", &self.mean_anomaly),
        ] {
            finite(field, angle.value)?;
            if !(-360.0..360.0).contains(&angle.value) {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: angle.value.to_string(),
                    expected: "[-360, 360)".into(),
                    line: None,
                }
                .into());
            }
        }

        if let Some(gm) = &self.gm {
            Gm::validate_value(gm.value, "GM")?;
        }
        Ok(())
    }
}

impl TleParameters {
    /// Check the values the schema types as doubles, independently of the mean-element theory.
    ///
    /// The theory-dependent presence rules live in [`TleParameters::validate`], which needs
    /// metadata this pass deliberately does not.
    pub(crate) fn validate_values(&self) -> Result<()> {
        for (field, value) in [
            ("BSTAR", self.bstar.as_ref().map(|value| value.value)),
            ("BTERM", self.bterm.as_ref().map(|value| value.value)),
            ("MEAN_MOTION_DOT", Some(self.mean_motion_dot.value)),
            (
                "MEAN_MOTION_DDOT",
                self.mean_motion_ddot.as_ref().map(|value| value.value),
            ),
            ("AGOM", self.agom.as_ref().map(|value| value.value)),
        ] {
            let Some(value) = value else { continue };
            finite(field, value)?;
        }
        // The typed wrapper only enforces `elementSetNoType` in its constructor; the public field
        // is reachable directly, so the root has to restate the range.
        if let Some(value) = self.element_set_no {
            ElementSetNo::validate_value(value.value, "ELEMENT_SET_NO")?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Spacecraft Parameters
//----------------------------------------------------------------------

//----------------------------------------------------------------------
// TLE Parameters
//----------------------------------------------------------------------

/// TLE Related Parameters (This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct TleParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Ephemeris type. Default value = 0. (See 4.2.4.7.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_type: Option<i32>,
    /// Classification type. Default value = U. (See 4.2.4.7.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub classification_type: Option<String>,
    /// NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword
    /// is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norad_cat_id: Option<u32>,
    /// Element set number for this satellite. Normally incremented sequentially but may be out
    /// of sync if it is generated from a backup source. Used to distinguish different TLEs,
    /// and therefore only meaningful if TLE-based data is being exchanged (i.e.,
    /// MEAN_ELEMENT_THEORY = SGP/SGP4).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_set_no: Option<ElementSetNo>,
    /// Revolution Number
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev_at_epoch: Option<u32>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4 (BSTAR = drag parameter for SGP4).
    ///
    /// **Units**: 1/[Earth radii]
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bstar: Option<BStar>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag
    /// coefficient, A = average cross-sectional area, m = mass. Example values for BTERM =
    /// 0.02 (rocket body), 0.0015 (payload); average value spanning 20,000 catalog objects =
    /// 0.0286.
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bterm: Option<M2kg>,
    /// First Time Derivative of the Mean Motion (i.e., a drag term, required when
    /// MEAN_ELEMENT_THEORY = SGP or PPT3). (See 4.2.4.7 for important details).
    ///
    /// **Units**: rev/day²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub mean_motion_dot: MeanMotionDot,
    /// Second Time Derivative of Mean Motion (i.e., a drag term). (See 4.2.4.7 for important
    /// details). Required when MEAN_ELEMENT_THEORY= SGP or PPT3.
    ///
    /// **Units**: rev/day³
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion_ddot: Option<MeanMotionDDot>,
    /// Solar radiation pressure coefficient AY/m, where y = reflectivity, A = average
    /// cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket body) and 0.001
    /// (payload); average value spanning 20,000 catalog objects = 0.0143 m2/kg. Required
    /// when MEAN_ELEMENT_THEORY= SGP4-XP.
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agom: Option<M2kg>,
}

impl TleParameters {
    pub fn validate(&self, theory: &str) -> Result<()> {
        if self.bstar.is_some() && self.bterm.is_some() {
            return Err(ValidationError::Conflict {
                fields: vec![Cow::Borrowed("BSTAR"), Cow::Borrowed("BTERM")],
                line: None,
            }
            .into());
        }
        if self.mean_motion_ddot.is_some() && self.agom.is_some() {
            return Err(ValidationError::Conflict {
                fields: vec![Cow::Borrowed("MEAN_MOTION_DDOT"), Cow::Borrowed("AGOM")],
                line: None,
            }
            .into());
        }

        match theory {
            "SGP" | "PPT3" => {
                if self.mean_motion_ddot.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("MEAN_MOTION_DDOT"),
                        line: None,
                    }
                    .into());
                }
            }
            "SGP4" => {
                if self.bstar.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("BSTAR"),
                        line: None,
                    }
                    .into());
                }
            }
            "SGP4-XP" => {
                if self.bterm.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("BTERM"),
                        line: None,
                    }
                    .into());
                }
                if self.agom.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("AGOM"),
                        line: None,
                    }
                    .into());
                }
            }
            _ => {}
        }
        Ok(())
    }
}
