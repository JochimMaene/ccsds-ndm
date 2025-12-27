// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters};
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, FromKvnValue, Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
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
            _ => Err(CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown unit: {}",
                s
            ))),
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
            _ => Err(CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown unit: {}",
                s
            ))),
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
            _ => Err(CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown unit: {}",
                s
            ))),
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
            _ => Err(CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown unit: {}",
                s
            ))),
        }
    }
}
pub type MeanMotionDDot = UnitValue<f64, RevPerDay3Units>;

//----------------------------------------------------------------------
// Root OMM Structure
//----------------------------------------------------------------------

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
        let mut tokens = KvnTokenizer::new(kvn).peekable();

        // 1. Version Check
        let version = loop {
            match tokens.peek() {
                Some(Ok(KvnLine::Pair {
                    key: "CCSDS_OMM_VERS",
                    ..
                })) => {
                    if let Some(Ok(KvnLine::Pair { val, .. })) = tokens.next() {
                        break val.to_string();
                    }
                    unreachable!();
                }
                Some(Ok(KvnLine::Comment(_))) | Some(Ok(KvnLine::Empty)) => {
                    tokens.next(); // skip
                }
                Some(_) => {
                    return Err(CcsdsNdmError::MissingField(
                        "CCSDS_OMM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        // 2. Header
        let header = OdmHeader::from_kvn_tokens(&mut tokens)?;

        // 3. Body
        let body = OmmBody::from_kvn_tokens(&mut tokens)?;

        Ok(Omm {
            header,
            body,
            id: Some("CCSDS_OMM_VERS".to_string()),
            version,
        })
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

impl ToKvn for OmmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl OmmBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // OMM has exactly one segment
        let segment = OmmSegment::from_kvn_tokens(tokens)?;
        Ok(OmmBody { segment })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let metadata = OmmMetadata::from_kvn_tokens(tokens)?;
        let data = OmmData::from_kvn_tokens(tokens)?;
        Ok(OmmSegment { metadata, data })
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub object_id: String,
    pub center_name: String,
    pub ref_frame: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    pub time_system: String,
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

impl OmmMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut builder = OmmMetadataBuilder::default();

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::Comment(c) => {
                    builder.comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    // Stop when we hit a Data keyword (e.g. EPOCH)
                    // The standard usually puts EPOCH first in data.
                    if key == &"EPOCH" {
                        break;
                    }
                    builder.match_pair(key, val)?;
                    tokens.next();
                }
                _ => break,
            }
        }
        builder.build()
    }
}

#[derive(Default)]
struct OmmMetadataBuilder {
    comment: Vec<String>,
    object_name: Option<String>,
    object_id: Option<String>,
    center_name: Option<String>,
    ref_frame: Option<String>,
    ref_frame_epoch: Option<Epoch>,
    time_system: Option<String>,
    mean_element_theory: Option<String>,
}

impl OmmMetadataBuilder {
    fn match_pair(&mut self, key: &str, val: &str) -> Result<()> {
        match key {
            "OBJECT_NAME" => self.object_name = Some(val.to_string()),
            "OBJECT_ID" => self.object_id = Some(val.to_string()),
            "CENTER_NAME" => self.center_name = Some(val.to_string()),
            "REF_FRAME" => self.ref_frame = Some(val.to_string()),
            "REF_FRAME_EPOCH" => self.ref_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?),
            "TIME_SYSTEM" => self.time_system = Some(val.to_string()),
            "MEAN_ELEMENT_THEORY" => self.mean_element_theory = Some(val.to_string()),
            _ => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Unexpected OMM Metadata key: {}",
                    key
                )))
            }
        }
        Ok(())
    }

    fn build(self) -> Result<OmmMetadata> {
        Ok(OmmMetadata {
            comment: self.comment,
            object_name: self
                .object_name
                .ok_or(CcsdsNdmError::MissingField("OBJECT_NAME".into()))?,
            object_id: self
                .object_id
                .ok_or(CcsdsNdmError::MissingField("OBJECT_ID".into()))?,
            center_name: self
                .center_name
                .ok_or(CcsdsNdmError::MissingField("CENTER_NAME".into()))?,
            ref_frame: self
                .ref_frame
                .ok_or(CcsdsNdmError::MissingField("REF_FRAME".into()))?,
            ref_frame_epoch: self.ref_frame_epoch,
            time_system: self
                .time_system
                .ok_or(CcsdsNdmError::MissingField("TIME_SYSTEM".into()))?,
            mean_element_theory: self
                .mean_element_theory
                .ok_or(CcsdsNdmError::MissingField("MEAN_ELEMENT_THEORY".into()))?,
        })
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(rename = "meanElements")]
    pub mean_elements: MeanElements,
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<SpacecraftParameters>,
    #[serde(
        rename = "tleParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tle_parameters: Option<TleParameters>,
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
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
                writer.write_pair(&p.parameter, &p.value);
            }
        }
    }
}

impl OmmData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut me_builder = MeanElementsBuilder::default();
        let mut sp_builder = SpacecraftParametersBuilder::default();
        let mut tle_builder = TleParametersBuilder::default();
        let mut cov_builder = crate::messages::opm::OpmCovarianceMatrixBuilder::default();
        let mut ud_builder = UserDefinedBuilder::default();
        let mut pending_comments = Vec::new();

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::Comment(c) => {
                    if !me_builder.has_started() {
                        comment.push(c.to_string());
                    } else {
                        pending_comments.push(c.to_string());
                    }
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, unit } => {
                    let key = *key;

                    // Mean Elements
                    if me_builder.try_match(key, val, *unit)? {
                        me_builder.comment.extend(pending_comments.drain(..));
                        tokens.next();
                        continue;
                    }

                    // Spacecraft Params
                    if sp_builder.try_match(key, val, *unit)? {
                        sp_builder.comment.extend(pending_comments.drain(..));
                        tokens.next();
                        continue;
                    }

                    // TLE Params
                    if tle_builder.try_match(key, val, *unit)? {
                        tle_builder.comment.extend(pending_comments.drain(..));
                        tokens.next();
                        continue;
                    }

                    // Covariance
                    if cov_builder.try_match(key, val, *unit)? {
                        cov_builder.comment.extend(pending_comments.drain(..));
                        tokens.next();
                        continue;
                    }

                    // User Defined
                    if key.starts_with("USER_DEFINED_") {
                        ud_builder.comment.extend(pending_comments.drain(..));
                        ud_builder.params.push(UserDefinedParameter {
                            parameter: key.to_string(),
                            value: val.to_string(),
                        });
                        tokens.next();
                        continue;
                    }

                    return Err(CcsdsNdmError::KvnParse(format!(
                        "Unexpected OMM Data field: {}",
                        key
                    )));
                }
                _ => break,
            }
        }

        Ok(OmmData {
            comment,
            mean_elements: me_builder.build()?,
            spacecraft_parameters: sp_builder.build()?,
            tle_parameters: tle_builder.build()?,
            covariance_matrix: cov_builder.build()?,
            user_defined_parameters: ud_builder.build(),
        })
    }
}

//----------------------------------------------------------------------
// Mean Elements
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MeanElements {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub epoch: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semi_major_axis: Option<Distance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion: Option<MeanMotion>,
    pub eccentricity: f64,
    pub inclination: Inclination,
    pub ra_of_asc_node: Angle,
    pub arg_of_pericenter: Angle,
    pub mean_anomaly: Angle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gm: Option<Gm>,
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

#[derive(Default)]
struct MeanElementsBuilder {
    comment: Vec<String>,
    epoch: Option<Epoch>,
    semi_major_axis: Option<Distance>,
    mean_motion: Option<MeanMotion>,
    eccentricity: Option<f64>,
    inclination: Option<Inclination>,
    ra_of_asc_node: Option<Angle>,
    arg_of_pericenter: Option<Angle>,
    mean_anomaly: Option<Angle>,
    gm: Option<Gm>,
}

impl MeanElementsBuilder {
    fn has_started(&self) -> bool {
        self.epoch.is_some()
    }

    fn try_match(&mut self, key: &str, val: &str, unit: Option<&str>) -> Result<bool> {
        match key {
            "EPOCH" => self.epoch = Some(FromKvnValue::from_kvn_value(val)?),
            "SEMI_MAJOR_AXIS" => self.semi_major_axis = Some(Distance::from_kvn(val, unit)?),
            "MEAN_MOTION" => self.mean_motion = Some(MeanMotion::from_kvn(val, unit)?),
            "ECCENTRICITY" => self.eccentricity = Some(val.parse()?),
            "INCLINATION" => self.inclination = Some(Inclination::from_kvn(val, unit)?),
            "RA_OF_ASC_NODE" => self.ra_of_asc_node = Some(Angle::from_kvn(val, unit)?),
            "ARG_OF_PERICENTER" => self.arg_of_pericenter = Some(Angle::from_kvn(val, unit)?),
            "MEAN_ANOMALY" => self.mean_anomaly = Some(Angle::from_kvn(val, unit)?),
            "GM" => {
                let uv = UnitValue::<f64, GmUnits>::from_kvn(val, unit)?;
                self.gm = Some(Gm::new(uv.value, uv.units)?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn build(self) -> Result<MeanElements> {
        if self.semi_major_axis.is_some() && self.mean_motion.is_some() {
            return Err(CcsdsNdmError::KvnParse(
                "Cannot have both SEMI_MAJOR_AXIS and MEAN_MOTION".into(),
            ));
        }
        if self.semi_major_axis.is_none() && self.mean_motion.is_none() {
            return Err(CcsdsNdmError::MissingField(
                "Either SEMI_MAJOR_AXIS or MEAN_MOTION must be present".into(),
            ));
        }

        let eccentricity = self
            .eccentricity
            .ok_or(CcsdsNdmError::MissingField("ECCENTRICITY".into()))?;
        if eccentricity < 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "ECCENTRICITY must be >= 0.0, found {}",
                eccentricity
            )));
        }

        let inclination = self
            .inclination
            .ok_or(CcsdsNdmError::MissingField("INCLINATION".into()))?;

        Ok(MeanElements {
            comment: self.comment,
            epoch: self
                .epoch
                .ok_or(CcsdsNdmError::MissingField("EPOCH".into()))?,
            semi_major_axis: self.semi_major_axis,
            mean_motion: self.mean_motion,
            eccentricity,
            inclination,
            ra_of_asc_node: self
                .ra_of_asc_node
                .ok_or(CcsdsNdmError::MissingField("RA_OF_ASC_NODE".into()))?,
            arg_of_pericenter: self
                .arg_of_pericenter
                .ok_or(CcsdsNdmError::MissingField("ARG_OF_PERICENTER".into()))?,
            mean_anomaly: self
                .mean_anomaly
                .ok_or(CcsdsNdmError::MissingField("MEAN_ANOMALY".into()))?,
            gm: self.gm,
        })
    }
}

//----------------------------------------------------------------------
// Spacecraft Parameters
//----------------------------------------------------------------------

#[derive(Default)]
struct SpacecraftParametersBuilder {
    comment: Vec<String>,
    mass: Option<Mass>,
    solar_rad_area: Option<Area>,
    solar_rad_coeff: Option<f64>,
    drag_area: Option<Area>,
    drag_coeff: Option<f64>,
}

impl SpacecraftParametersBuilder {
    fn try_match(&mut self, key: &str, val: &str, unit: Option<&str>) -> Result<bool> {
        match key {
            "MASS" => self.mass = Some(Mass::from_kvn(val, unit)?),
            "SOLAR_RAD_AREA" => self.solar_rad_area = Some(Area::from_kvn(val, unit)?),
            "SOLAR_RAD_COEFF" => self.solar_rad_coeff = Some(val.parse()?),
            "DRAG_AREA" => self.drag_area = Some(Area::from_kvn(val, unit)?),
            "DRAG_COEFF" => self.drag_coeff = Some(val.parse()?),
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn build(self) -> Result<Option<SpacecraftParameters>> {
        if self.mass.is_none()
            && self.solar_rad_area.is_none()
            && self.solar_rad_coeff.is_none()
            && self.drag_area.is_none()
            && self.drag_coeff.is_none()
        {
            return Ok(None);
        }
        Ok(Some(SpacecraftParameters {
            comment: self.comment,
            mass: self.mass,
            solar_rad_area: self.solar_rad_area,
            solar_rad_coeff: self.solar_rad_coeff,
            drag_area: self.drag_area,
            drag_coeff: self.drag_coeff,
        }))
    }
}

//----------------------------------------------------------------------
// TLE Parameters
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TleParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norad_cat_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_set_no: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev_at_epoch: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bstar: Option<BStar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bterm: Option<M2kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion_dot: Option<MeanMotionDot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion_ddot: Option<MeanMotionDDot>,
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

#[derive(Default)]
struct TleParametersBuilder {
    comment: Vec<String>,
    ephemeris_type: Option<i32>,
    classification_type: Option<String>,
    norad_cat_id: Option<u32>,
    element_set_no: Option<u32>,
    rev_at_epoch: Option<u32>,
    bstar: Option<BStar>,
    bterm: Option<M2kg>,
    mean_motion_dot: Option<MeanMotionDot>,
    mean_motion_ddot: Option<MeanMotionDDot>,
    agom: Option<M2kg>,
}

impl TleParametersBuilder {
    fn try_match(&mut self, key: &str, val: &str, unit: Option<&str>) -> Result<bool> {
        match key {
            "EPHEMERIS_TYPE" => self.ephemeris_type = Some(val.parse()?),
            "CLASSIFICATION_TYPE" => self.classification_type = Some(val.to_string()),
            "NORAD_CAT_ID" => self.norad_cat_id = Some(val.parse()?),
            "ELEMENT_SET_NO" => self.element_set_no = Some(val.parse()?),
            "REV_AT_EPOCH" => self.rev_at_epoch = Some(val.parse()?),
            "BSTAR" => self.bstar = Some(BStar::from_kvn(val, unit)?),
            "BTERM" => self.bterm = Some(M2kg::from_kvn(val, unit)?),
            "MEAN_MOTION_DOT" => self.mean_motion_dot = Some(MeanMotionDot::from_kvn(val, unit)?),
            "MEAN_MOTION_DDOT" => {
                self.mean_motion_ddot = Some(MeanMotionDDot::from_kvn(val, unit)?)
            }
            "AGOM" => self.agom = Some(M2kg::from_kvn(val, unit)?),
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn build(self) -> Result<Option<TleParameters>> {
        // If block empty, return None
        if self.ephemeris_type.is_none()
            && self.classification_type.is_none()
            && self.norad_cat_id.is_none()
            && self.element_set_no.is_none()
            && self.rev_at_epoch.is_none()
            && self.bstar.is_none()
            && self.bterm.is_none()
            && self.mean_motion_dot.is_none()
            && self.mean_motion_ddot.is_none()
            && self.agom.is_none()
        {
            return Ok(None);
        }

        // Validate ELEMENT_SET_NO range [0, 9999] per XSD
        if let Some(esn) = self.element_set_no {
            if esn > 9999 {
                return Err(CcsdsNdmError::Validation(format!(
                    "ELEMENT_SET_NO must be <= 9999, found {}",
                    esn
                )));
            }
        }

        // Check Choice: BSTAR vs BTERM
        if self.bstar.is_some() && self.bterm.is_some() {
            return Err(CcsdsNdmError::KvnParse(
                "Cannot have both BSTAR and BTERM".into(),
            ));
        }
        if self.bstar.is_none() && self.bterm.is_none() {
            return Err(CcsdsNdmError::MissingField(
                "Either BSTAR or BTERM must be present in TLE Parameters".into(),
            ));
        }

        // MEAN_MOTION_DOT is mandatory in tleParametersType
        if self.mean_motion_dot.is_none() {
            return Err(CcsdsNdmError::MissingField(
                "MEAN_MOTION_DOT is required in TLE Parameters".into(),
            ));
        }

        // Check Choice: MEAN_MOTION_DDOT vs AGOM
        if self.mean_motion_ddot.is_some() && self.agom.is_some() {
            return Err(CcsdsNdmError::KvnParse(
                "Cannot have both MEAN_MOTION_DDOT and AGOM".into(),
            ));
        }
        if self.mean_motion_ddot.is_none() && self.agom.is_none() {
            return Err(CcsdsNdmError::MissingField(
                "Either MEAN_MOTION_DDOT or AGOM must be present in TLE Parameters".into(),
            ));
        }

        Ok(Some(TleParameters {
            comment: self.comment,
            ephemeris_type: self.ephemeris_type,
            classification_type: self.classification_type,
            norad_cat_id: self.norad_cat_id,
            element_set_no: self.element_set_no,
            rev_at_epoch: self.rev_at_epoch,
            bstar: self.bstar,
            bterm: self.bterm,
            mean_motion_dot: self.mean_motion_dot,
            mean_motion_ddot: self.mean_motion_ddot,
            agom: self.agom,
        }))
    }
}

//----------------------------------------------------------------------
// User Defined
//----------------------------------------------------------------------

#[derive(Default)]
struct UserDefinedBuilder {
    comment: Vec<String>,
    params: Vec<UserDefinedParameter>,
}

impl UserDefinedBuilder {
    fn build(self) -> Option<UserDefined> {
        if self.params.is_empty() && self.comment.is_empty() {
            None
        } else {
            Some(UserDefined {
                comment: self.comment,
                user_defined: self.params,
            })
        }
    }
}

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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
MEAN_ELEMENT_THEORY = SGP4
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
