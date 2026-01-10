// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters, UserDefined};
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
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
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "1/ER" => Ok(InvErUnits::InvEr),
            _ => Err(CcsdsNdmError::UnknownUnit(s.to_string())),
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
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rev/day" => Ok(RevPerDayUnits::RevPerDay),
            "REV/DAY" => Ok(RevPerDayUnits::RevPerDayUpper),
            _ => Err(CcsdsNdmError::UnknownUnit(s.to_string())),
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
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rev/day**2" => Ok(RevPerDay2Units::RevPerDay2),
            "REV/DAY**2" => Ok(RevPerDay2Units::RevPerDay2Upper),
            _ => Err(CcsdsNdmError::UnknownUnit(s.to_string())),
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
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rev/day**3" => Ok(RevPerDay3Units::RevPerDay3),
            "REV/DAY**3" => Ok(RevPerDay3Units::RevPerDay3Upper),
            _ => Err(CcsdsNdmError::UnknownUnit(s.to_string())),
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
/// expressed in mean Keplerian elements.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.1.1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "omm")]
pub struct Omm {
    pub header: OdmHeader,
    pub body: OmmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Omm {
    /// Validates the OMM message.
    pub fn validate(&self) -> Result<()> {
        let theory = &self.body.segment.metadata.mean_element_theory;
        self.body.validate(theory)?;
        Ok(())
    }
}

impl Ndm for Omm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        // 1. Header
        writer.write_pair("CCSDS_OMM_VERS", &self.version);
        self.header.write_kvn(&mut writer);

        // 2. Body
        self.body.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let omm: Self = crate::kvn::from_str(kvn)?;
        omm.validate()?;
        Ok(omm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::from_str(xml)
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OmmBody {
    #[serde(rename = "segment")]
    pub segment: OmmSegment,
}

impl OmmBody {
    pub fn validate(&self, theory: &str) -> Result<()> {
        self.segment.validate(theory)?;
        Ok(())
    }
}

impl ToKvn for OmmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OmmSegment {
    pub metadata: OmmMetadata,
    pub data: OmmData,
}

impl OmmSegment {
    pub fn validate(&self, theory: &str) -> Result<()> {
        self.data.validate(theory)?;
        Ok(())
    }
}

impl ToKvn for OmmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// Metadata for the OMM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmMetadata {
    /// Comments (allowed at the beginning of the OMM Metadata). (See 7.8 for formatting rules.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Spacecraft name for which mean element orbit state data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use names from the
    /// UN Office of Outer Space Affairs designator index (reference \[3\], which include Object name
    /// and international designator of the participant). If OBJECT_NAME is not listed in reference
    /// \[3\] or the content is either unknown or cannot be disclosed, the value should be set to UNKNOWN.
    ///
    /// **Examples**: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub object_name: String,
    /// Object identifier of the object for which mean element orbit state data is provided. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// the international spacecraft designator as published in the UN Office of Outer Space Affairs
    /// designator index (reference \[3\]). Recommended values have the format YYYY-NNNP{PP}, where:
    /// YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading
    /// zeros). P{PP} = At least one capital letter for the identification of the part brought into
    /// space by the launch. If the asset is not listed in reference \[3\], the UN Office of Outer
    /// Space Affairs designator index format is not used, or the content is either unknown or cannot
    /// be disclosed, the value should be set to UNKNOWN.
    ///
    /// **Examples**: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub object_id: String,
    /// Origin of the OMM reference frame, which shall be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. Natural bodies shall be selected from the accepted set of values
    /// indicated in annex B, subsection B2.
    ///
    /// **Examples**: EARTH, MARS, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub center_name: String,
    /// Reference frame in which the Keplerian element data are given. Use of values other than those
    /// in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and corresponding
    /// Simplified General Perturbations (SGP) orbit propagator ephemeris outputs are explicitly
    /// defined to be in the True Equator Mean Equinox of Date (TEME of Date) reference frame.
    /// Therefore, TEME of date shall be used for OMMs based on NORAD Two Line Element sets, rather
    /// than the almost imperceptibly different TEME of Epoch (see reference \[H2\] or \[H3\] for
    /// further details).
    ///
    /// **Examples**: ICRF, ITRF2000, EME2000, TEME
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub ref_frame: String,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
    /// 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    /// Time system used for Keplerian elements and covariance data. Use of values other than those
    /// in 3.2.3.2 should be documented in an ICD.
    ///
    /// **Examples**: UTC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub time_system: String,
    /// Description of the Mean Element Theory. Indicates the proper method to employ to propagate the
    /// state.
    ///
    /// **Examples**: SGP, SGP4, SGP4-XP, DSST, USM
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    pub mean_element_theory: String,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

impl OmmData {
    pub fn validate(&self, theory: &str) -> Result<()> {
        self.mean_elements.validate(theory)?;
        if let Some(tle) = &self.tle_parameters {
            tle.validate(theory)?;
        } else if theory == "SGP" || theory == "SGP4" || theory == "SGP4-XP" {
            return Err(CcsdsNdmError::Validation(format!(
                "TLE parameters required for theory {}",
                theory
            )));
        }
        Ok(())
    }
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
                writer.write_pair(&p.parameter, &p.value);
            }
        }
    }
}

//----------------------------------------------------------------------
// Mean Elements
//----------------------------------------------------------------------

/// Mean Keplerian Elements in the Specified Reference Frame.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MeanElements {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Epoch of Mean Keplerian elements. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub epoch: Epoch,
    /// Semi-major axis. Preferred over MEAN_MOTION.
    ///
    /// **Examples**: 28594.4
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semi_major_axis: Option<Distance>,
    /// Keplerian Mean motion.
    ///
    /// Required if MEAN_ELEMENT_THEORY = SGP/SGP4.
    ///
    /// **Examples**: 1.491325
    ///
    /// **Units**: rev/day
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion: Option<MeanMotion>,
    /// Eccentricity.
    ///
    /// **Examples**: 0.7303
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub eccentricity: f64,
    /// Inclination.
    ///
    /// **Examples**: 63.4
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub inclination: Inclination,
    /// Right ascension of ascending node.
    ///
    /// **Examples**: 345.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub ra_of_asc_node: Angle,
    /// Argument of pericenter.
    ///
    /// **Examples**: 270.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub arg_of_pericenter: Angle,
    /// Mean anomaly.
    ///
    /// **Examples**: 130.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub mean_anomaly: Angle,
    /// Gravitational Coefficient (Gravitational Constant × Central Mass).
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
    pub fn validate(&self, theory: &str) -> Result<()> {
        if self.semi_major_axis.is_some() && self.mean_motion.is_some() {
            return Err(CcsdsNdmError::Validation(
                "Both SEMI_MAJOR_AXIS and MEAN_MOTION are present".to_string(),
            ));
        }
        if self.semi_major_axis.is_none() && self.mean_motion.is_none() {
            return Err(CcsdsNdmError::Validation(
                "Neither SEMI_MAJOR_AXIS nor MEAN_MOTION are present".to_string(),
            ));
        }
        if (theory == "SGP" || theory == "SGP4") && self.mean_motion.is_none() {
            return Err(CcsdsNdmError::Validation(
                "MEAN_MOTION required for SGP/SGP4".into(),
            ));
        }
        if self.eccentricity < 0.0 {
            return Err(CcsdsNdmError::Validation(
                "ECCENTRICITY must be >= 0".to_string(),
            ));
        }
        Inclination::new(
            self.inclination.angle.value,
            self.inclination.angle.units.clone(),
        )?;
        Angle::new(self.ra_of_asc_node.value, self.ra_of_asc_node.units.clone())?;
        Angle::new(
            self.arg_of_pericenter.value,
            self.arg_of_pericenter.units.clone(),
        )?;
        Angle::new(self.mean_anomaly.value, self.mean_anomaly.units.clone())?;
        Ok(())
    }
}

impl ToKvn for MeanElements {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", &self.epoch);
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

/// TLE Related Parameters.
///
/// This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TleParameters {
    /// Comments (see 7.8 for formatting rules.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Ephemeris type. Indicates what type of propagator was used to transform the native state to
    /// the SGP/SGP4 ephemeris state. The default is 0. (See 4.2.4.7 for numeric definitions.)
    ///
    /// - 0 = SGP
    /// - 2 = SGP4
    /// - 3 = PPT3
    /// - 4 = SGP4-XP
    /// - 6 = Special Perturbations
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_type: Option<i32>,
    /// Classification Type, default value = U. Some sources suggest the following coding for
    /// the CLASSIFICATION_TYPE keyword: U=unclassified, S=secret
    ///
    /// **Examples**: U
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_type: Option<String>,
    /// NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword is
    /// only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
    ///
    /// **Examples**: 28893
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norad_cat_id: Option<u32>,
    /// Element set number for this satellite. Normally incremented sequentially but may be out of
    /// sync if it is generated from a backup source. Used to distinguish different TLEs, and
    /// therefore only meaningful if TLE-based data is being exchanged (i.e., MEAN_ELEMENT_THEORY =
    /// SGP/SGP4).
    ///
    /// **Examples**: 999
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_set_no: Option<u32>,
    /// Number of revolutions at epoch.
    ///
    /// **Examples**: 120
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev_at_epoch: Option<u32>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4 (BSTAR = drag parameter for SGP4).
    ///
    /// **Examples**: 0.0001
    ///
    /// **Units**: 1/ER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bstar: Option<BStar>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag coefficient,
    /// A = average cross-sectional area, m = mass. Example values for BTERM = 0.02 (rocket body),
    /// 0.0015 (payload); average value spanning 20,00 catalog objects = 0.0286.
    ///
    /// **Examples**: 0.02
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bterm: Option<M2kg>,
    /// First Time Derivative of the Mean Motion (i.e., a drag term, required when MEAN_ELEMENT_THEORY
    /// = SGP or PPT3). (See 4.2.4.7 for important details).
    ///
    /// **Examples**: 0.000001
    ///
    /// **Units**: rev/day²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion_dot: Option<MeanMotionDot>,
    /// MEAN_ELEMENT_THEORY= SGP or PPT3: Second Time Derivative of Mean Motion (i.e., a drag term).
    /// (See 4.2.4.7 for important details).
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: rev/day³
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion_ddot: Option<MeanMotionDDot>,
    /// MEAN_ELEMENT_THEORY= SGP4-XP: Solar radiation pressure coefficient AY/m, where y =
    /// reflectivity, A = average cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket
    /// body) and 0.001 (payload); average value spanning 20,00 catalog objects = 0.0143 m2/kg.
    ///
    /// **Examples**: 0.01
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agom: Option<M2kg>,
}

impl TleParameters {
    pub fn validate(&self, theory: &str) -> Result<()> {
        if (theory == "SGP" || theory == "SGP4") && self.mean_motion_dot.is_none() {
            return Err(CcsdsNdmError::Validation(
                "MEAN_MOTION_DOT required for SGP/SGP4".to_string(),
            ));
        }
        if let Some(et) = self.element_set_no {
            if et >= 10000 {
                return Err(CcsdsNdmError::Validation(
                    "ELEMENT_SET_NO must be < 10000".to_string(),
                ));
            }
        }
        if self.bstar.is_some() && self.bterm.is_some() {
            return Err(CcsdsNdmError::Validation(
                "Both BSTAR and BTERM are present".to_string(),
            ));
        }
        if self.mean_motion_ddot.is_some() && self.agom.is_some() {
            return Err(CcsdsNdmError::Validation(
                "Both MEAN_MOTION_DDOT and AGOM are present".to_string(),
            ));
        }
        Ok(())
    }
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
        if let Some(v) = &self.mean_motion_dot {
            writer.write_measure("MEAN_MOTION_DOT", v);
        }
        if let Some(v) = &self.mean_motion_ddot {
            writer.write_measure("MEAN_MOTION_DDOT", v);
        }
        if let Some(v) = &self.agom {
            writer.write_measure("AGOM", v);
        }
    }
}

//----------------------------------------------------------------------
// User Defined
//----------------------------------------------------------------------

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_omm_kvn() -> String {
        r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
MESSAGE_ID = OMM 201113719185
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 1000
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#
        .to_string()
    }

    #[test]
    fn parse_omm_success() {
        let kvn = sample_omm_kvn();
        let omm = Omm::from_kvn(&kvn).expect("OMM parse failed");

        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.header.originator, "JAXA");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES 9");
        assert_eq!(omm.body.segment.metadata.mean_element_theory, "SGP4");

        let me = &omm.body.segment.data.mean_elements;
        assert_eq!(me.mean_motion.as_ref().unwrap().value, 1.00273272);
        assert_eq!(me.eccentricity, 0.00050130);

        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(23581));
        assert_eq!(tle.bstar.as_ref().unwrap().value, 0.0001);
    }

    #[test]
    fn parse_omm_with_covariance() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
CX_X = 1.0 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0 [km**2]
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
        let omm = Omm::from_kvn(kvn).expect("OMM Covariance parse failed");
        assert!(omm.body.segment.data.covariance_matrix.is_some());
        assert_eq!(
            omm.body
                .segment
                .data
                .covariance_matrix
                .as_ref()
                .unwrap()
                .cx_x
                .value,
            1.0
        );
    }

    #[test]
    fn test_mean_elements_choice_semi_major_axis_only() {
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
        let omm = Omm::from_kvn(kvn).expect("Should parse with SEMI_MAJOR_AXIS");
        assert!(omm
            .body
            .segment
            .data
            .mean_elements
            .semi_major_axis
            .is_some());
        assert!(omm.body.segment.data.mean_elements.mean_motion.is_none());
        assert_eq!(
            omm.body
                .segment
                .data
                .mean_elements
                .semi_major_axis
                .as_ref()
                .unwrap()
                .value,
            7000.0
        );
    }

    #[test]
    fn test_mean_elements_choice_mean_motion_only() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION");
        assert!(omm.body.segment.data.mean_elements.mean_motion.is_some());
        assert!(omm
            .body
            .segment
            .data
            .mean_elements
            .semi_major_axis
            .is_none());
        assert_eq!(
            omm.body
                .segment
                .data
                .mean_elements
                .mean_motion
                .as_ref()
                .unwrap()
                .value,
            15.5
        );
    }

    #[test]
    fn test_mean_elements_choice_both_fails() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
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
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("SEMI_MAJOR_AXIS") && err.contains("MEAN_MOTION"));
    }

    #[test]
    fn test_mean_elements_choice_neither_fails() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("SEMI_MAJOR_AXIS") || err.contains("MEAN_MOTION"));
    }

    #[test]
    fn test_tle_choice_bstar_only() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BSTAR");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bstar.is_some());
        assert!(tle.bterm.is_none());
    }

    #[test]
    fn test_tle_choice_bterm_only() {
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
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BTERM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bterm.is_some());
        assert!(tle.bstar.is_none());
    }

    #[test]
    fn test_tle_choice_bstar_and_bterm_fails() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("BSTAR") && err.contains("BTERM"));
    }

    #[test]
    fn test_tle_choice_mean_motion_ddot_only() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION_DDOT");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.agom.is_none());
    }

    #[test]
    fn test_tle_choice_agom_only() {
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
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with AGOM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.agom.is_some());
        assert!(tle.mean_motion_ddot.is_none());
    }

    #[test]
    fn test_tle_choice_mean_motion_ddot_and_agom_fails() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
AGOM = 0.01 [m**2/kg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("MEAN_MOTION_DDOT") && err.contains("AGOM"));
    }

    #[test]
    fn test_eccentricity_non_negative() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = -0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("ECCENTRICITY") || err.contains("0"));
    }

    #[test]
    fn test_inclination_range_valid() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 180.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("INCLINATION = 180 should be valid");
        assert_eq!(
            omm.body.segment.data.mean_elements.inclination.angle.value,
            180.0
        );
    }

    #[test]
    fn test_inclination_out_of_range_negative() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = -10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Inclination") || err.contains("range"));
    }

    #[test]
    fn test_element_set_no_range_valid() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
ELEMENT_SET_NO = 9999
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("ELEMENT_SET_NO = 9999 should be valid");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.element_set_no, Some(9999));
    }

    #[test]
    fn test_element_set_no_out_of_range() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
ELEMENT_SET_NO = 10000
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("ELEMENT_SET_NO") || err.contains("9999"));
    }

    #[test]
    fn test_parse_sample_omm_g7() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2007-07-26T17:26:06
ORIGINATOR = NOAA
MESSAGE_ID = 2007-001A
COMMENT This is a comment
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP/SGP4
EPOCH = 2007-07-26T17:26:06
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 925
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Failed to parse omm_g7.kvn");

        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.header.originator, "NOAA");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES 9");
        assert_eq!(omm.body.segment.metadata.object_id, "1995-025A");
        assert_eq!(omm.body.segment.metadata.center_name, "EARTH");
        assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");
        assert_eq!(omm.body.segment.metadata.time_system, "UTC");
        assert_eq!(omm.body.segment.metadata.mean_element_theory, "SGP/SGP4");

        let me = &omm.body.segment.data.mean_elements;
        assert!(me.mean_motion.is_some());
        assert_eq!(me.mean_motion.as_ref().unwrap().value, 1.00273272);
        assert_eq!(me.eccentricity, 0.0005013);

        // Has TLE parameters
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(23581));
        assert_eq!(tle.element_set_no, Some(925));
    }

    #[test]
    fn test_missing_object_name() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("OBJECT_NAME"));
    }

    #[test]
    fn test_missing_epoch() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_mean_motion_dot_required() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("MEAN_MOTION_DOT"));
    }

    // =========================================================================
    // XML Parsing Tests
    // =========================================================================

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
    fn test_roundtrip_kvn_with_tle() {
        let kvn = sample_omm_kvn();
        let omm1 = Omm::from_kvn(&kvn).expect("First parse failed");
        let kvn2 = omm1.to_kvn().expect("Serialization failed");
        let omm2 = Omm::from_kvn(&kvn2).expect("Second parse failed");

        let tle1 = omm1.body.segment.data.tle_parameters.as_ref().unwrap();
        let tle2 = omm2.body.segment.data.tle_parameters.as_ref().unwrap();

        assert_eq!(tle1.norad_cat_id, tle2.norad_cat_id);
        assert_eq!(
            tle1.bstar.as_ref().unwrap().value,
            tle2.bstar.as_ref().unwrap().value
        );
    }

    // =========================================================================
    // Optional Section Tests
    // =========================================================================

    #[test]
    fn test_omm_without_tle_parameters() {
        // OMM with no TLE parameters (valid for non-SGP4 theories)
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
        let omm = Omm::from_kvn(kvn).expect("Should parse without TLE parameters");
        assert!(omm.body.segment.data.tle_parameters.is_none());
    }

    #[test]
    fn test_omm_with_spacecraft_parameters() {
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
MASS = 1500.0 [kg]
SOLAR_RAD_AREA = 20.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 15.0 [m**2]
DRAG_COEFF = 2.2
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with spacecraft parameters");
        let sp = omm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert_eq!(sp.mass.as_ref().unwrap().value, 1500.0);
        assert_eq!(sp.solar_rad_area.as_ref().unwrap().value, 20.0);
        assert_eq!(sp.solar_rad_coeff, Some(1.2));
        assert_eq!(sp.drag_area.as_ref().unwrap().value, 15.0);
        assert_eq!(sp.drag_coeff, Some(2.2));
    }

    #[test]
    fn test_omm_with_gm() {
        // GM is optional in meanElementsType
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
GM = 398600.4418 [km**3/s**2]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with GM");
        let me = &omm.body.segment.data.mean_elements;
        assert!(me.gm.is_some());
        assert_eq!(me.gm.as_ref().unwrap().value, 398600.4418);
    }

    #[test]
    fn test_omm_with_ref_frame_epoch() {
        // REF_FRAME_EPOCH is optional in metadata
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
REF_FRAME_EPOCH = 2000-01-01T12:00:00
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
        let omm = Omm::from_kvn(kvn).expect("Should parse with REF_FRAME_EPOCH");
        assert!(omm.body.segment.metadata.ref_frame_epoch.is_some());
    }

    // =========================================================================
    // Unit Acceptance Tests (XSD allows uppercase units)
    // =========================================================================

    #[test]
    fn test_units_without_brackets() {
        // KVN can have units without brackets
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5
ECCENTRICITY = 0.001
INCLINATION = 98.0
RA_OF_ASC_NODE = 10.0
ARG_OF_PERICENTER = 20.0
MEAN_ANOMALY = 30.0
"#;
        // This should parse - units are optional per KVN format
        let omm = Omm::from_kvn(kvn).expect("Should parse without explicit units");
        assert_eq!(
            omm.body
                .segment
                .data
                .mean_elements
                .mean_motion
                .as_ref()
                .unwrap()
                .value,
            15.5
        );
    }

    // =========================================================================
    // Version and Comment Tests
    // =========================================================================

    #[test]
    fn test_omm_version_30() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse OMM version 3.0");
        // XSD: version is fixed="3.0"
        assert_eq!(omm.version, "3.0");
    }

    #[test]
    fn test_omm_with_comments() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
COMMENT This is a header comment
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
COMMENT This is a metadata comment
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
COMMENT This is a data comment
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with comments");
        // Verify the message parsed correctly with comments
        assert_eq!(omm.body.segment.metadata.object_name, "SAT");
    }

    // =========================================================================
    // TLE Parameters MEAN_MOTION_DOT is required when tleParameters present
    // Per XSD: <xsd:element name="MEAN_MOTION_DOT" type="ndm:dRevType"/> (no minOccurs=0)
    // =========================================================================

    #[test]
    fn test_tle_mean_motion_dot_required() {
        // According to XSD tleParametersType, MEAN_MOTION_DOT is mandatory
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION_DOT");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.mean_motion_dot.is_some());
    }

    // =========================================================================
    // Angle Range Tests (XSD angleRange: -360 <= value < 360)
    // =========================================================================

    #[test]
    fn test_angle_range_boundary_negative_360() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = -360.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        // -360.0 is inclusive per XSD
        let omm = Omm::from_kvn(kvn).expect("RA_OF_ASC_NODE = -360 should be valid");
        assert_eq!(
            omm.body.segment.data.mean_elements.ra_of_asc_node.value,
            -360.0
        );
    }

    #[test]
    fn test_angle_range_boundary_positive_359() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 359.99 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("RA_OF_ASC_NODE = 359.99 should be valid");
        assert!(omm.body.segment.data.mean_elements.ra_of_asc_node.value < 360.0);
    }

    #[test]
    fn test_angle_range_out_of_bounds_positive() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 360.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        // 360.0 is exclusive per XSD (maxExclusive)
        let result = Omm::from_kvn(kvn);
        assert!(result.is_err());
    }
}
