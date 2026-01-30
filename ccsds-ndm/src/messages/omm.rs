// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters};
use crate::error::{EnumParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

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
    #[serde(rename = "rev/day")]
    #[default]
    RevPerDay,
    #[serde(rename = "REV/DAY")]
    RevPerDayUpper,
}
impl std::fmt::Display for RevPerDayUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDayUnits::RevPerDay => write!(f, "rev/day"),
            RevPerDayUnits::RevPerDayUpper => write!(f, "REV/DAY"),
        }
    }
}
impl FromStr for RevPerDayUnits {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day" => Ok(RevPerDayUnits::RevPerDay),
            "REV/DAY" => Ok(RevPerDayUnits::RevPerDayUpper),
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
    #[serde(rename = "rev/day**2")]
    #[default]
    RevPerDay2,
    #[serde(rename = "REV/DAY**2")]
    RevPerDay2Upper,
}
impl std::fmt::Display for RevPerDay2Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDay2Units::RevPerDay2 => write!(f, "rev/day**2"),
            RevPerDay2Units::RevPerDay2Upper => write!(f, "REV/DAY**2"),
        }
    }
}
impl FromStr for RevPerDay2Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**2" => Ok(RevPerDay2Units::RevPerDay2),
            "REV/DAY**2" => Ok(RevPerDay2Units::RevPerDay2Upper),
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
    #[serde(rename = "rev/day**3")]
    #[default]
    RevPerDay3,
    #[serde(rename = "REV/DAY**3")]
    RevPerDay3Upper,
}
impl std::fmt::Display for RevPerDay3Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDay3Units::RevPerDay3 => write!(f, "rev/day**3"),
            RevPerDay3Units::RevPerDay3Upper => write!(f, "REV/DAY**3"),
        }
    }
}
impl FromStr for RevPerDay3Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**3" => Ok(RevPerDay3Units::RevPerDay3),
            "REV/DAY**3" => Ok(RevPerDay3Units::RevPerDay3Upper),
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
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
}

impl crate::traits::Validate for Omm {
    fn validate(&self) -> Result<()> {
        Omm::validate(self)
    }
}

impl Ndm for Omm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let omm = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Omm, &omm)?;
        Ok(omm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let omm: Self = crate::xml::from_str_with_context(xml, "OMM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Omm, &omm)?;
        Ok(omm)
    }
}

impl Omm {
    /// Validates the OMM against CCSDS constraints that cannot be checked during parsing.
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.segment.validate()
    }
}

impl ToKvn for Omm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_OMM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OmmBody {
    #[serde(rename = "segment")]
    pub segment: OmmSegment,
}

impl ToKvn for OmmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OmmSegment {
    pub metadata: OmmMetadata,
    pub data: OmmData,
}

impl ToKvn for OmmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl OmmSegment {
    pub fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate(&self.metadata)
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// Metadata for the OMM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    pub ref_frame_epoch: Option<Epoch>,
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

impl OmmMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("OMM Metadata"),
                field: Cow::Borrowed("OBJECT_ID"),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for OmmMetadata {
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
        writer.write_pair("MEAN_ELEMENT_THEORY", &self.mean_element_theory);
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// OMM Data section.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

impl ToKvn for OmmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Mean Elements
        self.mean_elements.write_kvn(writer);

        // Spacecraft Params
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.mass {
                writer.write_measure("MASS", &v.to_unit_value());
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
        }

        // TLE Params
        if let Some(tle) = &self.tle_parameters {
            tle.write_kvn(writer);
        }

        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }

        // User Defined
        if let Some(ud) = &self.user_defined_parameters {
            writer.write_comments(&ud.comment);
            for p in &ud.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
        }
    }
}

impl OmmData {
    pub fn validate(&self, metadata: &OmmMetadata) -> Result<()> {
        let theory = metadata.mean_element_theory.as_str();

        self.mean_elements.validate()?;

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
                // Not strictly required for other theories
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    pub epoch: Epoch,
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

impl MeanElements {
    pub fn validate(&self) -> Result<()> {
        match (self.semi_major_axis.is_some(), self.mean_motion.is_some()) {
            (true, false) | (false, true) => Ok(()),
            _ => Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "Mean Elements must have exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION",
                ),
                line: None,
            }
            .into()),
        }
    }
}

impl ToKvn for MeanElements {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        if let Some(v) = &self.semi_major_axis {
            writer.write_measure("SEMI_MAJOR_AXIS", v);
        }
        if let Some(v) = &self.mean_motion {
            writer.write_measure("MEAN_MOTION", v);
        }
        writer.write_pair("ECCENTRICITY", self.eccentricity);
        writer.write_measure("INCLINATION", &self.inclination.to_unit_value());
        writer.write_measure("RA_OF_ASC_NODE", &self.ra_of_asc_node.to_unit_value());
        writer.write_measure("ARG_OF_PERICENTER", &self.arg_of_pericenter.to_unit_value());
        writer.write_measure("MEAN_ANOMALY", &self.mean_anomaly.to_unit_value());
        if let Some(v) = &self.gm {
            writer.write_measure("GM", &UnitValue::new(v.value, v.units.clone()));
        }
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

impl ToKvn for TleParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(v) = self.ephemeris_type {
            writer.write_pair("EPHEMERIS_TYPE", v);
        }
        if let Some(v) = &self.classification_type {
            writer.write_pair("CLASSIFICATION_TYPE", v);
        }
        if let Some(v) = self.norad_cat_id {
            writer.write_pair("NORAD_CAT_ID", v);
        }
        if let Some(v) = self.element_set_no {
            writer.write_pair("ELEMENT_SET_NO", v);
        }
        if let Some(v) = self.rev_at_epoch {
            writer.write_pair("REV_AT_EPOCH", v);
        }
        if let Some(v) = &self.bstar {
            writer.write_measure("BSTAR", v);
        }
        if let Some(v) = &self.bterm {
            writer.write_measure("BTERM", v);
        }
        writer.write_measure("MEAN_MOTION_DOT", &self.mean_motion_dot);
        if let Some(v) = &self.mean_motion_ddot {
            writer.write_measure("MEAN_MOTION_DDOT", v);
        }
        if let Some(v) = &self.agom {
            writer.write_measure("AGOM", v);
        }
    }
}

impl TleParameters {
    pub fn validate(&self, theory: &str) -> Result<()> {
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

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // OMM Roundtrip Tests (Kitchen Sink)
    // =========================================================================

    #[test]
    fn full_optional_fields_roundtrip() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
COMMENT Header Comment 1
COMMENT Header Comment 2
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = SATELLITE
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
GM = 398600.4418 [km**3/s**2]
MASS = 1500.0 [kg]
SOLAR_RAD_AREA = 20.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 15.0 [m**2]
DRAG_COEFF = 2.2
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 99999
ELEMENT_SET_NO = 123
REV_AT_EPOCH = 500
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
COV_REF_FRAME = TEME
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.1
CZ_Y = 0.1
CZ_Z = 1.0
CX_DOT_X = 0.1
CX_DOT_Y = 0.1
CX_DOT_Z = 0.1
CX_DOT_X_DOT = 1.0
CY_DOT_X = 0.1
CY_DOT_Y = 0.1
CY_DOT_Z = 0.1
CY_DOT_X_DOT = 0.1
CY_DOT_Y_DOT = 1.0
CZ_DOT_X = 0.1
CZ_DOT_Y = 0.1
CZ_DOT_Z = 0.1
CZ_DOT_X_DOT = 0.1
CZ_DOT_Y_DOT = 0.1
CZ_DOT_Z_DOT = 1.0
USER_DEFINED_FOO = BAR
USER_DEFINED_BAZ = QUX
"#;
        let omm = Omm::from_kvn(kvn).expect("Failed to parse kitchen sink OMM");

        // Verify some fields
        assert_eq!(omm.header.message_id, Some("MSG-001".to_string()));
        assert_eq!(omm.header.comment.len(), 2);

        let me = &omm.body.segment.data.mean_elements;
        assert_eq!(me.gm.as_ref().unwrap().value, 398600.4418);

        let sp = omm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert_eq!(sp.mass.as_ref().unwrap().value, 1500.0);

        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(99999));

        let binding = omm
            .body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap();
        let ud = &binding.user_defined;
        assert_eq!(ud.len(), 2);
        assert_eq!(ud[0].parameter, "FOO");
        assert_eq!(ud[0].value, "BAR");

        // Roundtrip
        let kvn_out = omm.to_kvn().expect("Failed to serialize OMM");
        let omm2 = Omm::from_kvn(&kvn_out).expect("Failed to re-parse OMM");

        assert_eq!(omm, omm2);
    }

    #[test]
    fn test_parse_xml_omm_g10() {
        let xml = include_str!("../../../data/xml/omm_g10.xml");
        let omm = Omm::from_xml(xml).expect("Failed to parse omm_g10.xml");

        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES-9");
        assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");

        let me = &omm.body.segment.data.mean_elements;
        assert!(me.mean_motion.is_some());

        // Has covariance
        assert!(omm.body.segment.data.covariance_matrix.is_some());

        // Has TLE parameters
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(23581));
    }

    #[test]
    fn test_roundtrip_kvn_minimal() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm1 = Omm::from_kvn(kvn).expect("First parse failed");
        let kvn2 = omm1.to_kvn().expect("Serialization failed");
        let omm2 = Omm::from_kvn(&kvn2).expect("Second parse failed");

        assert_eq!(omm1.version, omm2.version);
        assert_eq!(omm1.header.originator, omm2.header.originator);
        assert_eq!(
            omm1.body.segment.metadata.object_name,
            omm2.body.segment.metadata.object_name
        );
        assert_eq!(
            omm1.body.segment.data.mean_elements.eccentricity,
            omm2.body.segment.data.mean_elements.eccentricity
        );
    }

    #[test]
    fn test_omm_validation_missing_mandatory_metadata() {
        // Missing OBJECT_NAME
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  EPHEMERIS_TYPE = 0
  CLASSIFICATION_TYPE = U
  NORAD_CAT_ID = 99999
  ELEMENT_SET_NO = 123
  REV_AT_EPOCH = 500
  BSTAR = 0.0001 [1/ER]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        // OBJECT_NAME is mandatory in the struct builder
        // The parser usually fails if a required field is missing for the builder
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_reqs() {
        // Case 1: SGP4 theory but missing TLE Parameters block
        let kvn_no_tle = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        let res = Omm::from_kvn(kvn_no_tle);
        assert!(res.is_err());
        // Check for specific error if possible, but strict error checking might be brittle
        // Expecting ValidationError::MissingRequiredField for TLE_PARAMETERS

        // Case 2: SGP4 theory but using SEMI_MAJOR_AXIS instead of MEAN_MOTION
        let kvn_sma = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  BSTAR = 0.0001 [1/ER]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
"#;
        // Validation logic should flag missing MEAN_MOTION for SGP4
        assert!(Omm::from_kvn(kvn_sma).is_err());

        // Case 3: SGP4 theory but missing BSTAR in TLE parameters
        let kvn_no_bstar = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
"#;
        assert!(Omm::from_kvn(kvn_no_bstar).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_xp_reqs() {
        // SGP4-XP requires AGOM and BTERM
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  BTERM = 0.01 [m**2/kg]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  # Missing AGOM
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_mean_elements_choice() {
        // Missing both SMA and Mean Motion
        let kvn_none = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn_none).is_err());

        // Both SMA and Mean Motion present
        let kvn_both = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn_both).is_err());
    }

    #[test]
    fn test_omm_validation_negative_values() {
        // Negative Eccentricity
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = -0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_units_parsing() {
        use std::str::FromStr;
        assert!(InvErUnits::from_str("1/ER").is_ok());
        assert!(InvErUnits::from_str("INVALID").is_err());

        assert!(RevPerDayUnits::from_str("rev/day").is_ok());
        assert!(RevPerDayUnits::from_str("REV/DAY").is_ok());
        assert!(RevPerDayUnits::from_str("INVALID").is_err());

        assert!(RevPerDay2Units::from_str("rev/day**2").is_ok());
        assert!(RevPerDay2Units::from_str("REV/DAY**2").is_ok());
        assert!(RevPerDay2Units::from_str("INVALID").is_err());

        assert!(RevPerDay3Units::from_str("rev/day**3").is_ok());
        assert!(RevPerDay3Units::from_str("REV/DAY**3").is_ok());
        assert!(RevPerDay3Units::from_str("INVALID").is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp_ppt3_reqs() {
        // SGP/PPT3 requires MEAN_MOTION_DDOT
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  # Missing MEAN_MOTION_DDOT
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_xp_additional_reqs() {
        // SGP4-XP requires BTERM and AGOM
        let data = OmmData::builder()
            .mean_elements(
                MeanElements::builder()
                    .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .mean_motion(MeanMotion::new(15.0, None))
                    .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                    .inclination(Inclination::new(10.0, None).unwrap())
                    .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                    .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                    .mean_anomaly(Angle::new(10.0, None).unwrap())
                    .build(),
            )
            .tle_parameters(
                TleParameters::builder()
                    .mean_motion_dot(MeanMotionDot::new(0.0, None))
                    .build(),
            )
            .build();

        let mut segment = OmmSegment::builder()
            .metadata(
                OmmMetadata::builder()
                    .object_name("SAT")
                    .object_id("1")
                    .center_name("EARTH")
                    .ref_frame("TEME")
                    .time_system("UTC")
                    .mean_element_theory("SGP4-XP")
                    .build(),
            )
            .data(data.clone())
            .build();

        // Missing BTERM
        assert!(segment.validate().is_err());

        segment.data.tle_parameters.as_mut().unwrap().bterm = Some(M2kg::new(0.01, None));
        // Missing AGOM
        assert!(segment.validate().is_err());

        segment.data.tle_parameters.as_mut().unwrap().agom = Some(M2kg::new(1.0, None));
        assert!(segment.validate().is_ok());
    }

    #[test]
    fn test_omm_serialization_gaps() {
        let mut tle = TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build();
        tle.bterm = Some(M2kg::new(0.01, None));
        tle.agom = Some(M2kg::new(1.0, None));

        let omm = Omm::builder()
            .version("3.0")
            .header(
                OdmHeader::builder()
                    .creation_date(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .originator("ME")
                    .build(),
            )
            .body(
                OmmBody::builder()
                    .segment(
                        OmmSegment::builder()
                            .metadata(
                                OmmMetadata::builder()
                                    .object_name("SAT")
                                    .object_id("1")
                                    .center_name("EARTH")
                                    .ref_frame("TEME")
                                    .time_system("UTC")
                                    .mean_element_theory("SGP4-XP")
                                    .build(),
                            )
                            .data(
                                OmmData::builder()
                                    .mean_elements(
                                        MeanElements::builder()
                                            .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                                            .mean_motion(MeanMotion::new(15.0, None))
                                            .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                                            .inclination(Inclination::new(10.0, None).unwrap())
                                            .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                                            .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                                            .mean_anomaly(Angle::new(10.0, None).unwrap())
                                            .build(),
                                    )
                                    .tle_parameters(tle)
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();

        let kvn = omm.to_kvn().unwrap();
        assert!(kvn.contains("BTERM"));
        assert!(kvn.contains("0.01"));
        assert!(kvn.contains("AGOM"));
        assert!(kvn.contains("1"));
    }

    #[test]
    fn test_omm_validation_theory_gaps() {
        // SGP missing MEAN_MOTION
        let meta = OmmMetadata::builder()
            .object_name("SAT")
            .object_id("1")
            .center_name("EARTH")
            .ref_frame("TEME")
            .time_system("UTC")
            .mean_element_theory("SGP")
            .build();
        let mut data = OmmData::builder()
            .mean_elements(
                MeanElements::builder()
                    .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .semi_major_axis(Distance::new(7000.0, None))
                    .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                    .inclination(Inclination::new(10.0, None).unwrap())
                    .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                    .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                    .mean_anomaly(Angle::new(10.0, None).unwrap())
                    .build(),
            )
            .tle_parameters(
                TleParameters::builder()
                    .mean_motion_dot(MeanMotionDot::new(0.0, None))
                    .build(),
            )
            .build();

        let segment = OmmSegment::builder()
            .metadata(meta)
            .data(data.clone())
            .build();
        assert!(segment.validate().is_err()); // Missing MEAN_MOTION for SGP

        // TleParameters SGP/PPT3 missing mean_motion_ddot
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP")
            .is_err());
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("PPT3")
            .is_err());

        // TleParameters SGP4 missing bstar
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP4")
            .is_err());

        // TleParameters SGP4-XP missing bterm/agom
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP4-XP")
            .is_err());
        let mut tle_xp = data.tle_parameters.clone().unwrap();
        tle_xp.bterm = Some(M2kg::new(0.01, None));
        assert!(tle_xp.validate("SGP4-XP").is_err()); // Missing AGOM

        // MeanElements both SMA and MeanMotion
        data.mean_elements.mean_motion = Some(MeanMotion::new(15.0, None));
        assert!(data.mean_elements.validate().is_err());

        // MeanElements neither SMA nor MeanMotion
        data.mean_elements.semi_major_axis = None;
        data.mean_elements.mean_motion = None;
        assert!(data.mean_elements.validate().is_err());

        // Unknown theory in TleParameters
        let tle = TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build();
        assert!(tle.validate("UNKNOWN").is_ok());
    }

    #[test]
    fn test_rev_per_day_units_display_all() {
        assert_eq!(format!("{}", RevPerDayUnits::RevPerDayUpper), "REV/DAY");
        assert_eq!(
            format!("{}", RevPerDay2Units::RevPerDay2Upper),
            "REV/DAY**2"
        );
        assert_eq!(
            format!("{}", RevPerDay3Units::RevPerDay3Upper),
            "REV/DAY**3"
        );
    }
}
