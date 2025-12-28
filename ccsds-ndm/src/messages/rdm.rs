// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, OpmCovarianceMatrix,
    RdmSpacecraftParameters, StateVector,
};
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, FromKvnValue, Ndm, ToKvn};
use crate::types::FromKvn;
use crate::types::{
    ControlledType, DayIntervalRequired, Epoch, ObjectDescription, PercentageRequired,
    PositionRequired, YesNo,
};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

//----------------------------------------------------------------------
// Root RDM Structure
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "rdm")]
pub struct Rdm {
    pub header: RdmHeader,
    pub body: RdmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Rdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        writer.write_pair("CCSDS_RDM_VERS", &self.version);
        self.header.write_kvn(&mut writer);
        self.body.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let mut tokens = KvnTokenizer::new(kvn).peekable();

        // Version line
        let version = loop {
            match tokens.peek() {
                Some(Ok(KvnLine::Pair {
                    key: "CCSDS_RDM_VERS",
                    ..
                })) => {
                    if let Some(Ok(KvnLine::Pair { val, .. })) = tokens.next() {
                        break val.to_string();
                    }
                    unreachable!();
                }
                Some(Ok(KvnLine::Comment(_))) | Some(Ok(KvnLine::Empty)) => {
                    tokens.next();
                }
                Some(_) => {
                    return Err(CcsdsNdmError::MissingField(
                        "CCSDS_RDM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        let header = RdmHeader::from_kvn_tokens(&mut tokens)?;
        let body = RdmBody::from_kvn_tokens(&mut tokens)?;

        Ok(Rdm {
            header,
            body,
            id: Some("CCSDS_RDM_VERS".to_string()),
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
// Header
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub creation_date: Epoch,
    pub originator: String,
    pub message_id: String,
}

impl ToKvn for RdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

impl FromKvnTokens for RdmHeader {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut creation_date = None;
        let mut originator = None;
        let mut message_id = None;

        while tokens.peek().is_some() {
            if let Some(Err(_)) = tokens.peek() {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err());
            }
            match tokens
                .peek()
                .expect("Peeked value should exist")
                .as_ref()
                .expect("Peeked value should be Ok")
            {
                KvnLine::Comment(c) => {
                    comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    match *key {
                        "CREATION_DATE" => creation_date = Some(Epoch::from_kvn_value(val)?),
                        "ORIGINATOR" => originator = Some(val.to_string()),
                        "MESSAGE_ID" => message_id = Some(val.to_string()),
                        _ => break,
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        Ok(RdmHeader {
            comment,
            creation_date: creation_date
                .ok_or(CcsdsNdmError::MissingField("CREATION_DATE".into()))?,
            originator: originator.ok_or(CcsdsNdmError::MissingField("ORIGINATOR".into()))?,
            message_id: message_id.ok_or(CcsdsNdmError::MissingField("MESSAGE_ID".into()))?,
        })
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RdmBody {
    pub segment: Box<RdmSegment>,
}

impl ToKvn for RdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl RdmBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let segment = RdmSegment::from_kvn_tokens(tokens)?;
        Ok(RdmBody {
            segment: Box::new(segment),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RdmSegment {
    pub metadata: RdmMetadata,
    pub data: RdmData,
}

impl ToKvn for RdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl RdmSegment {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let metadata = RdmMetadata::from_kvn_tokens(tokens)?;
        let data = RdmData::from_kvn_tokens(tokens)?;
        Ok(Self { metadata, data })
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub international_designator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_designator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_operator: Option<String>,
    pub controlled_reentry: ControlledType,
    pub center_name: String,
    pub time_system: String,
    pub epoch_tzero: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_flux_prediction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_pressure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earth_tides: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrack_thrust: Option<YesNo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_parameters_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_parameters_altitude: Option<PositionRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_uncertainty_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_disintegration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_uncertainty_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_epoch: Option<Epoch>,
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
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v);
        }
        if let Some(v) = &self.object_owner {
            writer.write_pair("OBJECT_OWNER", v);
        }
        if let Some(v) = &self.object_operator {
            writer.write_pair("OBJECT_OPERATOR", v);
        }
        writer.write_pair("CONTROLLED_REENTRY", &self.controlled_reentry);
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", &self.epoch_tzero);
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
            writer.write_pair("INTRACK_THRUST", v);
        }
        if let Some(v) = &self.drag_parameters_source {
            writer.write_pair("DRAG_PARAMETERS_SOURCE", v);
        }
        if let Some(v) = &self.drag_parameters_altitude {
            writer.write_pair("DRAG_PARAMETERS_ALTITUDE", v);
        }
        if let Some(v) = &self.reentry_uncertainty_method {
            writer.write_pair("REENTRY_UNCERTAINTY_METHOD", v);
        }
        if let Some(v) = &self.reentry_disintegration {
            writer.write_pair("REENTRY_DISINTEGRATION", v);
        }
        if let Some(v) = &self.impact_uncertainty_method {
            writer.write_pair("IMPACT_UNCERTAINTY_METHOD", v);
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

impl RdmMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // No META_START in RDM (unlike TDM/CDM). Just flow from header.

        // Use Option for mandatory fields to track if they were actually provided
        let mut comment = Vec::new();
        let mut object_name: Option<String> = None;
        let mut international_designator: Option<String> = None;
        let mut catalog_name: Option<String> = None;
        let mut object_designator: Option<String> = None;
        let mut object_type: Option<ObjectDescription> = None;
        let mut object_owner: Option<String> = None;
        let mut object_operator: Option<String> = None;
        let mut controlled_reentry: Option<ControlledType> = None;
        let mut center_name: Option<String> = None;
        let mut time_system: Option<String> = None;
        let mut epoch_tzero: Option<Epoch> = None;
        let mut ref_frame: Option<String> = None;
        let mut ref_frame_epoch: Option<Epoch> = None;
        let mut ephemeris_name: Option<String> = None;
        let mut gravity_model: Option<String> = None;
        let mut atmospheric_model: Option<String> = None;
        let mut solar_flux_prediction: Option<String> = None;
        let mut n_body_perturbations: Option<String> = None;
        let mut solar_rad_pressure: Option<String> = None;
        let mut earth_tides: Option<String> = None;
        let mut intrack_thrust: Option<YesNo> = None;
        let mut drag_parameters_source: Option<String> = None;
        let mut drag_parameters_altitude: Option<PositionRequired> = None;
        let mut reentry_uncertainty_method: Option<String> = None;
        let mut reentry_disintegration: Option<String> = None;
        let mut impact_uncertainty_method: Option<String> = None;
        let mut previous_message_id: Option<String> = None;
        let mut previous_message_epoch: Option<Epoch> = None;
        let mut next_message_epoch: Option<Epoch> = None;

        while tokens.peek().is_some() {
            if let Some(Err(_)) = tokens.peek() {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err());
            }
            match tokens
                .peek()
                .expect("Peeked value should exist")
                .as_ref()
                .expect("Peeked value should be Ok")
            {
                // No META_STOP expectation
                KvnLine::Comment(c) => {
                    comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    // If we hit a Data keyword, stop.
                    if is_rdm_data_keyword(key) {
                        break;
                    }

                    let key = *key;
                    let val = *val;
                    match key {
                        "OBJECT_NAME" => object_name = Some(val.to_string()),
                        "INTERNATIONAL_DESIGNATOR" => {
                            international_designator = Some(val.to_string())
                        }
                        "CATALOG_NAME" => catalog_name = Some(val.to_string()),
                        "OBJECT_DESIGNATOR" => object_designator = Some(val.to_string()),
                        "OBJECT_TYPE" => object_type = Some(val.parse()?),
                        "OBJECT_OWNER" => object_owner = Some(val.to_string()),
                        "OBJECT_OPERATOR" => object_operator = Some(val.to_string()),
                        "CONTROLLED_REENTRY" => controlled_reentry = Some(val.parse()?),
                        "CENTER_NAME" => center_name = Some(val.to_string()),
                        "TIME_SYSTEM" => time_system = Some(val.to_string()),
                        "EPOCH_TZERO" => epoch_tzero = Some(Epoch::from_kvn_value(val)?),
                        "REF_FRAME" => ref_frame = Some(val.to_string()),
                        "REF_FRAME_EPOCH" => ref_frame_epoch = Some(Epoch::from_kvn_value(val)?),
                        "EPHEMERIS_NAME" => ephemeris_name = Some(val.to_string()),
                        "GRAVITY_MODEL" => gravity_model = Some(val.to_string()),
                        "ATMOSPHERIC_MODEL" => atmospheric_model = Some(val.to_string()),
                        "SOLAR_FLUX_PREDICTION" => solar_flux_prediction = Some(val.to_string()),
                        "N_BODY_PERTURBATIONS" => n_body_perturbations = Some(val.to_string()),
                        "SOLAR_RAD_PRESSURE" => solar_rad_pressure = Some(val.to_string()),
                        "EARTH_TIDES" => earth_tides = Some(val.to_string()),
                        "INTRACK_THRUST" => intrack_thrust = Some(val.parse()?),
                        "DRAG_PARAMETERS_SOURCE" => drag_parameters_source = Some(val.to_string()),
                        "DRAG_PARAMETERS_ALTITUDE" => {
                            drag_parameters_altitude = Some(PositionRequired::from_kvn(val, None)?)
                        }
                        "REENTRY_UNCERTAINTY_METHOD" => {
                            reentry_uncertainty_method = Some(val.to_string())
                        }
                        "REENTRY_DISINTEGRATION" => reentry_disintegration = Some(val.to_string()),
                        "IMPACT_UNCERTAINTY_METHOD" => {
                            impact_uncertainty_method = Some(val.to_string())
                        }
                        "PREVIOUS_MESSAGE_ID" => previous_message_id = Some(val.to_string()),
                        "PREVIOUS_MESSAGE_EPOCH" => {
                            previous_message_epoch = Some(Epoch::from_kvn_value(val)?)
                        }
                        "NEXT_MESSAGE_EPOCH" => {
                            next_message_epoch = Some(Epoch::from_kvn_value(val)?)
                        }
                        _ => {
                            return Err(CcsdsNdmError::KvnParse(format!(
                                "Unexpected RDM Metadata key: {}",
                                key
                            )))
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        // Validate all mandatory fields per XSD (no defaults allowed)
        Ok(RdmMetadata {
            comment,
            object_name: object_name
                .ok_or_else(|| CcsdsNdmError::MissingField("OBJECT_NAME".into()))?,
            international_designator: international_designator
                .ok_or_else(|| CcsdsNdmError::MissingField("INTERNATIONAL_DESIGNATOR".into()))?,
            catalog_name,
            object_designator,
            object_type,
            object_owner,
            object_operator,
            controlled_reentry: controlled_reentry
                .ok_or_else(|| CcsdsNdmError::MissingField("CONTROLLED_REENTRY".into()))?,
            center_name: center_name
                .ok_or_else(|| CcsdsNdmError::MissingField("CENTER_NAME".into()))?,
            time_system: time_system
                .ok_or_else(|| CcsdsNdmError::MissingField("TIME_SYSTEM".into()))?,
            epoch_tzero: epoch_tzero
                .ok_or_else(|| CcsdsNdmError::MissingField("EPOCH_TZERO".into()))?,
            ref_frame,
            ref_frame_epoch,
            ephemeris_name,
            gravity_model,
            atmospheric_model,
            solar_flux_prediction,
            n_body_perturbations,
            solar_rad_pressure,
            earth_tides,
            intrack_thrust,
            drag_parameters_source,
            drag_parameters_altitude,
            reentry_uncertainty_method,
            reentry_disintegration,
            impact_uncertainty_method,
            previous_message_id,
            previous_message_epoch,
            next_message_epoch,
        })
    }
}

// Helper to identify RDM Data Block keywords
fn is_rdm_data_keyword(key: &str) -> bool {
    matches!(
        key,
        "ORBIT_LIFETIME"
            | "REENTRY_ALTITUDE"
            | "ORBIT_LIFETIME_WINDOW_START"
            | "ORBIT_LIFETIME_WINDOW_END"
            | "NOMINAL_REENTRY_EPOCH"
            | "REENTRY_WINDOW_START"
            | "REENTRY_WINDOW_END"
            | "ORBIT_LIFETIME_CONFIDENCE_LEVEL"
            | "PROBABILITY_OF_IMPACT"
            | "PROBABILITY_OF_BURN_UP"
            | "PROBABILITY_OF_BREAK_UP"
            | "PROBABILITY_OF_LAND_IMPACT"
            | "PROBABILITY_OF_CASUALTY"
            | "NOMINAL_IMPACT_EPOCH"
            | "IMPACT_WINDOW_START"
            | "IMPACT_WINDOW_END"
            | "IMPACT_REF_FRAME"
            | "NOMINAL_IMPACT_LON"
            | "NOMINAL_IMPACT_LAT"
            | "NOMINAL_IMPACT_ALT"
            | "IMPACT_1_CONFIDENCE"
            | "IMPACT_1_START_LON"
            | "IMPACT_1_START_LAT"
            | "IMPACT_1_STOP_LON"
            | "IMPACT_1_STOP_LAT"
            | "IMPACT_1_CROSS_TRACK"
            | "IMPACT_2_CONFIDENCE"
            | "IMPACT_2_START_LON"
            | "IMPACT_2_START_LAT"
            | "IMPACT_2_STOP_LON"
            | "IMPACT_2_STOP_LAT"
            | "IMPACT_2_CROSS_TRACK"
            | "IMPACT_3_CONFIDENCE"
            | "IMPACT_3_START_LON"
            | "IMPACT_3_START_LAT"
            | "IMPACT_3_STOP_LON"
            | "IMPACT_3_STOP_LAT"
            | "IMPACT_3_CROSS_TRACK"
            | "EPOCH"
            | "X"
            | "Y"
            | "Z"
            | "X_DOT"
            | "Y_DOT"
            | "Z_DOT"
            | "COV_REF_FRAME"
            | "CX_X"
            | "CY_X"
            | "CY_Y"
            | "CZ_X"
            | "CZ_Y"
            | "CZ_Z"
            | "CX_DOT_X"
            | "CX_DOT_Y"
            | "CX_DOT_Z"
            | "CX_DOT_X_DOT"
            | "CY_DOT_X"
            | "CY_DOT_Y"
            | "CY_DOT_Z"
            | "CY_DOT_X_DOT"
            | "CY_DOT_Y_DOT"
            | "CZ_DOT_X"
            | "CZ_DOT_Y"
            | "CZ_DOT_Z"
            | "CZ_DOT_X_DOT"
            | "CZ_DOT_Y_DOT"
            | "CZ_DOT_Z_DOT"
            | "WET_MASS"
            | "DRY_MASS"
            | "HAZARDOUS_SUBSTANCES"
            | "SOLAR_RAD_AREA"
            | "SOLAR_RAD_COEFF"
            | "DRAG_AREA"
            | "DRAG_COEFF"
            | "RCS"
            | "BALLISTIC_COEFF"
            | "THRUST_ACCELERATION"
            | "TIME_LASTOB_START"
            | "TIME_LASTOB_END"
            | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN"
            | "OBS_AVAILABLE"
            | "OBS_USED"
            | "TRACKS_AVAILABLE"
            | "TRACKS_USED"
            | "RESIDUALS_ACCEPTED"
            | "WEIGHTED_RMS" // Also user defined
    ) || key.starts_with("USER_DEFINED_")
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(rename = "atmosphericReentryParameters")]
    pub atmospheric_reentry_parameters: AtmosphericReentryParameters,
    #[serde(
        rename = "groundImpactParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ground_impact_parameters: Option<GroundImpactParameters>,
    #[serde(
        rename = "stateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_vector: Option<StateVector>,
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<RdmSpacecraftParameters>,
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_defined_parameters: Vec<(String, String)>,
}

impl ToKvn for RdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // No DATA_START
        writer.write_comments(&self.comment);
        // Atmospheric (mandatory)
        let a = &self.atmospheric_reentry_parameters;
        writer.write_pair("ORBIT_LIFETIME", &a.orbit_lifetime);
        writer.write_pair("REENTRY_ALTITUDE", &a.reentry_altitude);
        if let Some(v) = &a.orbit_lifetime_window_start {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_START", v);
        }
        if let Some(v) = &a.orbit_lifetime_window_end {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_END", v);
        }
        if let Some(v) = &a.nominal_reentry_epoch {
            writer.write_pair("NOMINAL_REENTRY_EPOCH", v);
        }
        if let Some(v) = &a.reentry_window_start {
            writer.write_pair("REENTRY_WINDOW_START", v);
        }
        if let Some(v) = &a.reentry_window_end {
            writer.write_pair("REENTRY_WINDOW_END", v);
        }
        if let Some(v) = &a.orbit_lifetime_confidence_level {
            writer.write_pair("ORBIT_LIFETIME_CONFIDENCE_LEVEL", v);
        }

        // Ground impact (optional)
        if let Some(g) = &self.ground_impact_parameters {
            if let Some(v) = &g.probability_of_impact {
                writer.write_pair("PROBABILITY_OF_IMPACT", v);
            }
            if let Some(v) = &g.probability_of_burn_up {
                writer.write_pair("PROBABILITY_OF_BURN_UP", v);
            }
            if let Some(v) = &g.probability_of_break_up {
                writer.write_pair("PROBABILITY_OF_BREAK_UP", v);
            }
            if let Some(v) = &g.probability_of_land_impact {
                writer.write_pair("PROBABILITY_OF_LAND_IMPACT", v);
            }
            if let Some(v) = &g.probability_of_casualty {
                writer.write_pair("PROBABILITY_OF_CASUALTY", v);
            }
            if let Some(v) = &g.nominal_impact_epoch {
                writer.write_pair("NOMINAL_IMPACT_EPOCH", v);
            }
            if let Some(v) = &g.impact_window_start {
                writer.write_pair("IMPACT_WINDOW_START", v);
            }
            if let Some(v) = &g.impact_window_end {
                writer.write_pair("IMPACT_WINDOW_END", v);
            }
            if let Some(v) = &g.impact_ref_frame {
                writer.write_pair("IMPACT_REF_FRAME", v);
            }
            if let Some(v) = &g.nominal_impact_lon {
                writer.write_pair("NOMINAL_IMPACT_LON", v);
            }
            if let Some(v) = &g.nominal_impact_lat {
                writer.write_pair("NOMINAL_IMPACT_LAT", v);
            }
            if let Some(v) = &g.nominal_impact_alt {
                writer.write_pair("NOMINAL_IMPACT_ALT", v);
            }
            if let Some(v) = &g.impact_1_confidence {
                writer.write_pair("IMPACT_1_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_1_start_lon {
                writer.write_pair("IMPACT_1_START_LON", v);
            }
            if let Some(v) = &g.impact_1_start_lat {
                writer.write_pair("IMPACT_1_START_LAT", v);
            }
            if let Some(v) = &g.impact_1_stop_lon {
                writer.write_pair("IMPACT_1_STOP_LON", v);
            }
            if let Some(v) = &g.impact_1_stop_lat {
                writer.write_pair("IMPACT_1_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_1_cross_track {
                writer.write_pair("IMPACT_1_CROSS_TRACK", v);
            }
            if let Some(v) = &g.impact_2_confidence {
                writer.write_pair("IMPACT_2_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_2_start_lon {
                writer.write_pair("IMPACT_2_START_LON", v);
            }
            if let Some(v) = &g.impact_2_start_lat {
                writer.write_pair("IMPACT_2_START_LAT", v);
            }
            if let Some(v) = &g.impact_2_stop_lon {
                writer.write_pair("IMPACT_2_STOP_LON", v);
            }
            if let Some(v) = &g.impact_2_stop_lat {
                writer.write_pair("IMPACT_2_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_2_cross_track {
                writer.write_pair("IMPACT_2_CROSS_TRACK", v);
            }
            if let Some(v) = &g.impact_3_confidence {
                writer.write_pair("IMPACT_3_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_3_start_lon {
                writer.write_pair("IMPACT_3_START_LON", v);
            }
            if let Some(v) = &g.impact_3_start_lat {
                writer.write_pair("IMPACT_3_START_LAT", v);
            }
            if let Some(v) = &g.impact_3_stop_lon {
                writer.write_pair("IMPACT_3_STOP_LON", v);
            }
            if let Some(v) = &g.impact_3_stop_lat {
                writer.write_pair("IMPACT_3_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_3_cross_track {
                writer.write_pair("IMPACT_3_CROSS_TRACK", v);
            }
        }

        // Optional blocks: write when present
        if let Some(sv) = &self.state_vector {
            sv.write_kvn(writer);
        }
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
        if let Some(sp) = &self.spacecraft_parameters {
            // Write minimal known fields
            if let Some(v) = &sp.wet_mass {
                writer.write_pair("WET_MASS", v);
            }
            if let Some(v) = &sp.dry_mass {
                writer.write_pair("DRY_MASS", v);
            }
            if let Some(v) = &sp.hazardous_substances {
                writer.write_pair("HAZARDOUS_SUBSTANCES", v);
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_pair("SOLAR_RAD_AREA", v);
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_pair("SOLAR_RAD_COEFF", v);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_pair("DRAG_AREA", v);
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_pair("DRAG_COEFF", v);
            }
            if let Some(v) = &sp.rcs {
                writer.write_pair("RCS", v);
            }
            if let Some(v) = &sp.ballistic_coeff {
                writer.write_pair("BALLISTIC_COEFF", v);
            }
            if let Some(v) = &sp.thrust_acceleration {
                writer.write_pair("THRUST_ACCELERATION", v);
            }
        }
        if let Some(od) = &self.od_parameters {
            if let Some(v) = &od.time_lastob_start {
                writer.write_pair("TIME_LASTOB_START", v);
            }
            if let Some(v) = &od.time_lastob_end {
                writer.write_pair("TIME_LASTOB_END", v);
            }
            if let Some(v) = &od.recommended_od_span {
                writer.write_pair("RECOMMENDED_OD_SPAN", v);
            }
            if let Some(v) = &od.actual_od_span {
                writer.write_pair("ACTUAL_OD_SPAN", v);
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
                writer.write_pair("RESIDUALS_ACCEPTED", v);
            }
            if let Some(v) = &od.weighted_rms {
                writer.write_pair("WEIGHTED_RMS", v);
            }
        }

        for (k, v) in &self.user_defined_parameters {
            writer.write_pair(k, v);
        }
    }
}

impl RdmData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();

        // Accumulate fields for atmospheric and optional blocks
        let mut orbit_lifetime: Option<DayIntervalRequired> = None;
        let mut reentry_altitude: Option<PositionRequired> = None;
        let mut orbit_lifetime_window_start: Option<DayIntervalRequired> = None;
        let mut orbit_lifetime_window_end: Option<DayIntervalRequired> = None;
        let mut nominal_reentry_epoch: Option<Epoch> = None;
        let mut reentry_window_start: Option<Epoch> = None;
        let mut reentry_window_end: Option<Epoch> = None;
        let mut orbit_lifetime_confidence_level: Option<PercentageRequired> = None;

        let mut ground: GroundImpactParameters = GroundImpactParameters {
            comment: Vec::new(),
            probability_of_impact: None,
            probability_of_burn_up: None,
            probability_of_break_up: None,
            probability_of_land_impact: None,
            probability_of_casualty: None,
            nominal_impact_epoch: None,
            impact_window_start: None,
            impact_window_end: None,
            impact_ref_frame: None,
            nominal_impact_lon: None,
            nominal_impact_lat: None,
            nominal_impact_alt: None,
            impact_1_confidence: None,
            impact_1_start_lon: None,
            impact_1_start_lat: None,
            impact_1_stop_lon: None,
            impact_1_stop_lat: None,
            impact_1_cross_track: None,
            impact_2_confidence: None,
            impact_2_start_lon: None,
            impact_2_start_lat: None,
            impact_2_stop_lon: None,
            impact_2_stop_lat: None,
            impact_2_cross_track: None,
            impact_3_confidence: None,
            impact_3_start_lon: None,
            impact_3_start_lat: None,
            impact_3_stop_lon: None,
            impact_3_stop_lat: None,
            impact_3_cross_track: None,
        };
        let mut have_ground = false;
        let mut state_vector: Option<StateVector> = None;
        let mut covariance_matrix: Option<OpmCovarianceMatrix> = None;
        let mut spacecraft_parameters: Option<RdmSpacecraftParameters> = None;
        let mut od_parameters: Option<OdParameters> = None;
        let mut user_defined_parameters: Vec<(String, String)> = Vec::new();

        while tokens.peek().is_some() {
            if let Some(Err(_)) = tokens.peek() {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err());
            }
            match tokens
                .peek()
                .expect("Peeked value should exist")
                .as_ref()
                .expect("Peeked value should be Ok")
            {
                // No DATA_STOP expected
                KvnLine::Comment(c) => {
                    comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    let key = *key;
                    let val = *val;
                    match key {
                        // Atmospheric
                        "ORBIT_LIFETIME" => {
                            orbit_lifetime = Some(DayIntervalRequired::from_kvn(val, None)?)
                        }
                        "REENTRY_ALTITUDE" => {
                            reentry_altitude = Some(PositionRequired::from_kvn(val, None)?)
                        }
                        "ORBIT_LIFETIME_WINDOW_START" => {
                            orbit_lifetime_window_start =
                                Some(DayIntervalRequired::from_kvn(val, None)?)
                        }
                        "ORBIT_LIFETIME_WINDOW_END" => {
                            orbit_lifetime_window_end =
                                Some(DayIntervalRequired::from_kvn(val, None)?)
                        }
                        "NOMINAL_REENTRY_EPOCH" => {
                            nominal_reentry_epoch = Some(Epoch::from_kvn_value(val)?)
                        }
                        "REENTRY_WINDOW_START" => {
                            reentry_window_start = Some(Epoch::from_kvn_value(val)?)
                        }
                        "REENTRY_WINDOW_END" => {
                            reentry_window_end = Some(Epoch::from_kvn_value(val)?)
                        }
                        "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => {
                            orbit_lifetime_confidence_level =
                                Some(PercentageRequired::from_kvn(val, None)?)
                        }

                        // Ground impact
                        "PROBABILITY_OF_IMPACT" => {
                            ground.probability_of_impact =
                                Some(crate::types::Probability::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "PROBABILITY_OF_BURN_UP" => {
                            ground.probability_of_burn_up =
                                Some(crate::types::Probability::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "PROBABILITY_OF_BREAK_UP" => {
                            ground.probability_of_break_up =
                                Some(crate::types::Probability::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "PROBABILITY_OF_LAND_IMPACT" => {
                            ground.probability_of_land_impact =
                                Some(crate::types::Probability::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "PROBABILITY_OF_CASUALTY" => {
                            ground.probability_of_casualty =
                                Some(crate::types::Probability::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "NOMINAL_IMPACT_EPOCH" => {
                            ground.nominal_impact_epoch = Some(Epoch::from_kvn_value(val)?);
                            have_ground = true;
                        }
                        "IMPACT_WINDOW_START" => {
                            ground.impact_window_start = Some(Epoch::from_kvn_value(val)?);
                            have_ground = true;
                        }
                        "IMPACT_WINDOW_END" => {
                            ground.impact_window_end = Some(Epoch::from_kvn_value(val)?);
                            have_ground = true;
                        }
                        "IMPACT_REF_FRAME" => {
                            ground.impact_ref_frame = Some(val.to_string());
                            have_ground = true;
                        }
                        "NOMINAL_IMPACT_LON" => {
                            ground.nominal_impact_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "NOMINAL_IMPACT_LAT" => {
                            ground.nominal_impact_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "NOMINAL_IMPACT_ALT" => {
                            ground.nominal_impact_alt =
                                Some(crate::types::AltitudeRequired::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_1_CONFIDENCE" => {
                            ground.impact_1_confidence =
                                Some(PercentageRequired::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_1_START_LON" => {
                            ground.impact_1_start_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_1_START_LAT" => {
                            ground.impact_1_start_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_1_STOP_LON" => {
                            ground.impact_1_stop_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_1_STOP_LAT" => {
                            ground.impact_1_stop_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_1_CROSS_TRACK" => {
                            ground.impact_1_cross_track =
                                Some(crate::types::Distance::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_2_CONFIDENCE" => {
                            ground.impact_2_confidence =
                                Some(PercentageRequired::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_2_START_LON" => {
                            ground.impact_2_start_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_2_START_LAT" => {
                            ground.impact_2_start_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_2_STOP_LON" => {
                            ground.impact_2_stop_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_2_STOP_LAT" => {
                            ground.impact_2_stop_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_2_CROSS_TRACK" => {
                            ground.impact_2_cross_track =
                                Some(crate::types::Distance::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_3_CONFIDENCE" => {
                            ground.impact_3_confidence =
                                Some(PercentageRequired::from_kvn(val, None)?);
                            have_ground = true;
                        }
                        "IMPACT_3_START_LON" => {
                            ground.impact_3_start_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_3_START_LAT" => {
                            ground.impact_3_start_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_3_STOP_LON" => {
                            ground.impact_3_stop_lon =
                                Some(crate::types::LongitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_3_STOP_LAT" => {
                            ground.impact_3_stop_lat =
                                Some(crate::types::LatitudeRequired::new(val.parse::<f64>()?)?);
                            have_ground = true;
                        }
                        "IMPACT_3_CROSS_TRACK" => {
                            ground.impact_3_cross_track =
                                Some(crate::types::Distance::from_kvn(val, None)?);
                            have_ground = true;
                        }

                        // State vector fields (RDM uses flat structure, not a "STATE_VECTOR" keyword)
                        "EPOCH" => {
                            // EPOCH starts the state vector block
                            // Parse the state vector fields
                            state_vector = Some(StateVector::from_kvn_tokens(tokens)?);
                            continue; // from_kvn_tokens consumes the tokens including EPOCH
                        }
                        // Covariance matrix fields
                        "COV_REF_FRAME" | "CX_X" => {
                            covariance_matrix = Some(OpmCovarianceMatrix::from_kvn_tokens(tokens)?);
                            continue; // callee consumed
                        }
                        // Spacecraft params / OD params: treat as flat key-value
                        "WET_MASS"
                        | "DRY_MASS"
                        | "HAZARDOUS_SUBSTANCES"
                        | "SOLAR_RAD_AREA"
                        | "SOLAR_RAD_COEFF"
                        | "DRAG_AREA"
                        | "DRAG_COEFF"
                        | "RCS"
                        | "BALLISTIC_COEFF"
                        | "THRUST_ACCELERATION"
                        | "TIME_LASTOB_START"
                        | "TIME_LASTOB_END"
                        | "RECOMMENDED_OD_SPAN"
                        | "ACTUAL_OD_SPAN"
                        | "OBS_AVAILABLE"
                        | "OBS_USED"
                        | "TRACKS_AVAILABLE"
                        | "TRACKS_USED"
                        | "RESIDUALS_ACCEPTED"
                        | "WEIGHTED_RMS" => {
                            // We'll parse lazily into the appropriate structs below
                            // Fall through to capture generic user-defined and process later
                            user_defined_parameters.push((key.to_string(), val.to_string()));
                        }
                        _ => {
                            // Treat unknowns as user-defined
                            user_defined_parameters.push((key.to_string(), val.to_string()));
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        // Build atmospheric (required)
        let atmospheric_reentry_parameters = AtmosphericReentryParameters {
            comment: Vec::new(),
            orbit_lifetime: orbit_lifetime
                .ok_or(CcsdsNdmError::MissingField("ORBIT_LIFETIME".into()))?,
            reentry_altitude: reentry_altitude
                .ok_or(CcsdsNdmError::MissingField("REENTRY_ALTITUDE".into()))?,
            orbit_lifetime_window_start,
            orbit_lifetime_window_end,
            nominal_reentry_epoch,
            reentry_window_start,
            reentry_window_end,
            orbit_lifetime_confidence_level,
        };

        // Post-process collected key-values into structured optional blocks
        if !user_defined_parameters.is_empty() {
            // Attempt to map known keys into spacecraft/od parameter structs
            let mut sp = RdmSpacecraftParameters::default();
            let mut od = OdParameters::default();
            for (k, v) in &user_defined_parameters {
                match k.as_str() {
                    // Spacecraft
                    "WET_MASS" => sp.wet_mass = Some(crate::types::Mass::from_kvn(v, None)?),
                    "DRY_MASS" => sp.dry_mass = Some(crate::types::Mass::from_kvn(v, None)?),
                    "HAZARDOUS_SUBSTANCES" => sp.hazardous_substances = Some(v.clone()),
                    "SOLAR_RAD_AREA" => {
                        sp.solar_rad_area = Some(crate::types::Area::from_kvn(v, None)?)
                    }
                    "SOLAR_RAD_COEFF" => sp.solar_rad_coeff = Some(v.parse::<f64>()?),
                    "DRAG_AREA" => sp.drag_area = Some(crate::types::Area::from_kvn(v, None)?),
                    "DRAG_COEFF" => sp.drag_coeff = Some(v.parse::<f64>()?),
                    "RCS" => sp.rcs = Some(crate::types::Area::from_kvn(v, None)?),
                    "BALLISTIC_COEFF" => {
                        sp.ballistic_coeff = Some(crate::types::BallisticCoeffRequired::new(
                            v.parse::<f64>()?,
                        )?)
                    }
                    "THRUST_ACCELERATION" => {
                        sp.thrust_acceleration =
                            Some(crate::types::Ms2Required::new(v.parse::<f64>()?))
                    }
                    // OD
                    "TIME_LASTOB_START" => od.time_lastob_start = Some(Epoch::from_kvn_value(v)?),
                    "TIME_LASTOB_END" => od.time_lastob_end = Some(Epoch::from_kvn_value(v)?),
                    "RECOMMENDED_OD_SPAN" => {
                        od.recommended_od_span = Some(crate::types::DayInterval::from_kvn(v, None)?)
                    }
                    "ACTUAL_OD_SPAN" => {
                        od.actual_od_span = Some(crate::types::DayInterval::from_kvn(v, None)?)
                    }
                    "OBS_AVAILABLE" => od.obs_available = Some(v.parse::<u32>()?),
                    "OBS_USED" => od.obs_used = Some(v.parse::<u32>()?),
                    "TRACKS_AVAILABLE" => od.tracks_available = Some(v.parse::<u32>()?),
                    "TRACKS_USED" => od.tracks_used = Some(v.parse::<u32>()?),
                    "RESIDUALS_ACCEPTED" => {
                        od.residuals_accepted = Some(crate::types::Percentage::from_kvn(v, None)?)
                    }
                    "WEIGHTED_RMS" => od.weighted_rms = Some(v.parse::<f64>()?),
                    _ => {}
                }
            }
            if sp != RdmSpacecraftParameters::default() {
                spacecraft_parameters = Some(sp);
            }
            if od != OdParameters::default() {
                od_parameters = Some(od);
            }
        }

        Ok(RdmData {
            comment,
            atmospheric_reentry_parameters,
            ground_impact_parameters: if have_ground { Some(ground) } else { None },
            state_vector,
            covariance_matrix,
            spacecraft_parameters,
            od_parameters,
            user_defined_parameters,
        })
    }
}

// Helper to identify RDM Data Block keywords

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // XSD Compliance Tests for RDM
    // ==========================================

    // -----------------------------------------
    // rdmType (root element)
    // -----------------------------------------

    /// XSD: rdmType has id="CCSDS_RDM_VERS" fixed and version="1.0" fixed
    #[test]
    fn test_xsd_rdm_root_attributes() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
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
        assert_eq!(rdm.id, Some("CCSDS_RDM_VERS".to_string()));
        assert_eq!(rdm.version, "1.0");
    }

    // -----------------------------------------
    // rdmHeader
    // -----------------------------------------

    /// XSD: header requires CREATION_DATE (epochType), ORIGINATOR (string), MESSAGE_ID (string)
    #[test]
    fn test_xsd_rdm_header_mandatory_fields() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = ESA
MESSAGE_ID = ESA-20231113-001
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
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.header.message_id, "ESA-20231113-001");
    }

    /// XSD: COMMENT in header is optional (0..*)
    #[test]
    fn test_xsd_rdm_header_optional_comments() {
        // Without comments
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
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
        assert!(rdm.header.comment.is_empty());
    }

    // -----------------------------------------
    // rdmMetadata
    // -----------------------------------------

    /// XSD: metadata requires OBJECT_NAME, INTERNATIONAL_DESIGNATOR, CONTROLLED_REENTRY,
    /// CENTER_NAME, TIME_SYSTEM, EPOCH_TZERO
    #[test]
    fn test_xsd_rdm_metadata_mandatory_fields() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = SENTINEL-1A
INTERNATIONAL_DESIGNATOR = 2014-016A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let meta = &rdm.body.segment.metadata;
        assert_eq!(meta.object_name, "SENTINEL-1A");
        assert_eq!(meta.international_designator, "2014-016A");
        assert_eq!(meta.center_name, "EARTH");
        assert_eq!(meta.time_system, "TAI");
    }

    /// XSD: CONTROLLED_REENTRY is controlledType: YES|yes|NO|no|UNKNOWN|unknown
    #[test]
    fn test_xsd_rdm_controlled_type_values() {
        for (val, expected) in [
            ("YES", ControlledType::Yes),
            ("yes", ControlledType::Yes),
            ("NO", ControlledType::No),
            ("no", ControlledType::No),
            ("UNKNOWN", ControlledType::Unknown),
            ("unknown", ControlledType::Unknown),
        ] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = {}
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                val
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert_eq!(rdm.body.segment.metadata.controlled_reentry, expected);
        }
    }

    /// XSD: OBJECT_TYPE is objectDescriptionType with specific enum values
    #[test]
    fn test_xsd_rdm_object_type_enum() {
        for obj_type in ["PAYLOAD", "ROCKET BODY", "DEBRIS", "UNKNOWN", "OTHER"] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
OBJECT_TYPE = {}
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                obj_type
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.metadata.object_type.is_some());
        }
    }

    /// XSD: INTRACK_THRUST is yesNoType: YES|yes|NO|no
    #[test]
    fn test_xsd_rdm_intrack_thrust_yesno() {
        for val in ["YES", "NO"] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
INTRACK_THRUST = {}
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                val
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.metadata.intrack_thrust.is_some());
        }
    }

    /// XSD: Optional metadata fields
    #[test]
    fn test_xsd_rdm_metadata_optional_fields() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
OBJECT_OWNER = ESA
OBJECT_OPERATOR = EUMETSAT
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
EPHEMERIS_NAME = NONE
GRAVITY_MODEL = EGM-96: 36D 360
ATMOSPHERIC_MODEL = NRLMSISE-00
SOLAR_FLUX_PREDICTION = PREDICTED
N_BODY_PERTURBATIONS = MOON, SUN
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
DRAG_PARAMETERS_SOURCE = OD
DRAG_PARAMETERS_ALTITUDE = 200 [km]
PREVIOUS_MESSAGE_ID = PREV-001
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let meta = &rdm.body.segment.metadata;
        assert_eq!(meta.catalog_name, Some("SATCAT".to_string()));
        assert_eq!(meta.object_designator, Some("12345".to_string()));
        assert_eq!(meta.object_owner, Some("ESA".to_string()));
        assert_eq!(meta.ref_frame, Some("EME2000".to_string()));
        assert_eq!(meta.gravity_model, Some("EGM-96: 36D 360".to_string()));
        assert_eq!(meta.atmospheric_model, Some("NRLMSISE-00".to_string()));
    }

    // -----------------------------------------
    // rdmData and atmosphericReentryParametersType
    // -----------------------------------------

    /// XSD: atmosphericReentryParametersType requires ORBIT_LIFETIME (dayIntervalTypeUR)
    /// and REENTRY_ALTITUDE (positionTypeUR)
    #[test]
    fn test_xsd_rdm_atmospheric_mandatory_fields() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 23.5 [d]
REENTRY_ALTITUDE = 150.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!((atmos.orbit_lifetime.value - 23.5).abs() < 1e-9);
        assert!((atmos.reentry_altitude.value - 150.0).abs() < 1e-9);
    }

    /// XSD: dayIntervalTypeUR has positiveDouble base with required units
    #[test]
    fn test_xsd_rdm_day_interval_units_required() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_WINDOW_START = 4.0 [d]
ORBIT_LIFETIME_WINDOW_END = 7.0 [d]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!(atmos.orbit_lifetime_window_start.is_some());
        assert!(atmos.orbit_lifetime_window_end.is_some());
    }

    /// XSD: percentageTypeUR for ORBIT_LIFETIME_CONFIDENCE_LEVEL
    #[test]
    fn test_xsd_rdm_percentage_type() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_CONFIDENCE_LEVEL = 95.0 [%]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!(atmos.orbit_lifetime_confidence_level.is_some());
    }

    // -----------------------------------------
    // groundImpactParametersType
    // -----------------------------------------

    /// XSD: probabilityType is xsd:double with minInclusive=0 maxInclusive=1
    #[test]
    fn test_xsd_rdm_probability_type_range() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.5
PROBABILITY_OF_BURN_UP = 0.0
PROBABILITY_OF_CASUALTY = 1.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let ground = rdm
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_ref()
            .unwrap();
        assert!((ground.probability_of_impact.as_ref().unwrap().value - 0.5).abs() < 1e-9);
        assert!((ground.probability_of_burn_up.as_ref().unwrap().value - 0.0).abs() < 1e-9);
        assert!((ground.probability_of_casualty.as_ref().unwrap().value - 1.0).abs() < 1e-9);
    }

    /// XSD: latType has latRange restriction -90 to 90 with required units
    #[test]
    fn test_xsd_rdm_latitude_range() {
        for lat in ["-90.0", "0.0", "45.5", "90.0"] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_LAT = {}
"#,
                lat
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    /// XSD: lonType has lonRange restriction -180 to 180 with required units
    #[test]
    fn test_xsd_rdm_longitude_range() {
        for lon in ["-180.0", "-45.5", "0.0", "90.0", "180.0"] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_LON = {}
"#,
                lon
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    /// XSD: altType has altRange restriction -430.5 to 8848 (Earth surface)
    #[test]
    fn test_xsd_rdm_altitude_range() {
        for alt in ["-430.0", "0.0", "1000.0", "8000.0"] {
            let kvn = format!(
                r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_ALT = {}
"#,
                alt
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    /// XSD: Impact confidence intervals (1, 2, 3)
    #[test]
    fn test_xsd_rdm_impact_confidence_intervals() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_1_CONFIDENCE = 50.0 [%]
IMPACT_1_START_LON = -10.0
IMPACT_1_START_LAT = 40.0
IMPACT_1_STOP_LON = 10.0
IMPACT_1_STOP_LAT = 45.0
IMPACT_1_CROSS_TRACK = 100.0 [km]
IMPACT_2_CONFIDENCE = 90.0 [%]
IMPACT_2_START_LON = -15.0
IMPACT_2_START_LAT = 38.0
IMPACT_2_STOP_LON = 15.0
IMPACT_2_STOP_LAT = 47.0
IMPACT_2_CROSS_TRACK = 200.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let ground = rdm.body.segment.data.ground_impact_parameters.unwrap();
        assert!(ground.impact_1_confidence.is_some());
        assert!(ground.impact_2_confidence.is_some());
    }

    // -----------------------------------------
    // stateVectorType
    // -----------------------------------------

    /// XSD: stateVectorType with positionTypeUO and velocityTypeUO (units optional)
    #[test]
    fn test_xsd_rdm_state_vector_type() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sv = rdm.body.segment.data.state_vector.as_ref().unwrap();
        assert!((sv.x.value - 7000.0).abs() < 1e-9);
    }

    // -----------------------------------------
    // opmCovarianceMatrixType
    // -----------------------------------------

    /// XSD: Covariance matrix elements (21 elements for 6x6 lower triangular)
    #[test]
    fn test_xsd_rdm_covariance_matrix_type() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
COV_REF_FRAME = RTN
CX_X = 1.0e-4 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0e-4 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0e-4 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 1.0e-6 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 1.0e-6 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 1.0e-6 [km**2/s**2]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let cov = rdm.body.segment.data.covariance_matrix.as_ref().unwrap();
        assert_eq!(cov.cov_ref_frame, Some("RTN".to_string()));
        assert!((cov.cx_x.value - 1.0e-4).abs() < 1e-15);
    }

    // -----------------------------------------
    // rdmSpacecraftParametersType
    // -----------------------------------------

    /// XSD: rdmSpacecraftParametersType fields (all optional)
    #[test]
    fn test_xsd_rdm_spacecraft_parameters() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
WET_MASS = 3500 [kg]
DRY_MASS = 2000 [kg]
HAZARDOUS_SUBSTANCES = Hydrazine, Nuclear
SOLAR_RAD_AREA = 25.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 20.0 [m**2]
DRAG_COEFF = 2.2
RCS = 15.0 [m**2]
BALLISTIC_COEFF = 150.0
THRUST_ACCELERATION = 0.001
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sp = rdm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert!((sp.wet_mass.as_ref().unwrap().value - 3500.0).abs() < 1e-9);
        assert!((sp.dry_mass.as_ref().unwrap().value - 2000.0).abs() < 1e-9);
        assert_eq!(
            sp.hazardous_substances,
            Some("Hydrazine, Nuclear".to_string())
        );
        assert!(sp.ballistic_coeff.is_some());
        assert!(sp.thrust_acceleration.is_some());
    }

    /// XSD: SOLAR_RAD_COEFF and DRAG_COEFF are nonNegativeDouble
    #[test]
    fn test_xsd_rdm_coefficients_nonnegative() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
SOLAR_RAD_COEFF = 0.0
DRAG_COEFF = 0.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sp = rdm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert!((sp.solar_rad_coeff.unwrap() - 0.0).abs() < 1e-9);
        assert!((sp.drag_coeff.unwrap() - 0.0).abs() < 1e-9);
    }

    // -----------------------------------------
    // odParametersType
    // -----------------------------------------

    /// XSD: odParametersType fields (all optional)
    #[test]
    fn test_xsd_rdm_od_parameters() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
TIME_LASTOB_START = 2022-12-31T00:00:00
TIME_LASTOB_END = 2022-12-31T23:59:59
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 20
TRACKS_USED = 18
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.234
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let od = rdm.body.segment.data.od_parameters.as_ref().unwrap();
        assert!(od.time_lastob_start.is_some());
        assert!(od.time_lastob_end.is_some());
        assert_eq!(od.obs_available, Some(100));
        assert_eq!(od.obs_used, Some(95));
        assert_eq!(od.tracks_available, Some(20));
        assert_eq!(od.tracks_used, Some(18));
    }

    // -----------------------------------------
    // Sample files parsing
    // -----------------------------------------

    /// Parse official RDM KVN example C-1 (minimal)
    #[test]
    fn test_xsd_rdm_sample_c1_kvn() {
        let kvn = std::fs::read_to_string("../data/kvn/rdm_c1.kvn").unwrap();
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    }

    /// Parse official RDM KVN example C-2 (comprehensive)
    #[test]
    fn test_xsd_rdm_sample_c2_kvn() {
        // Note: C-2 has some typos in covariance values (O.10000 instead of 0.10000)
        // This test validates the structure, not the specific file parsing
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2018-04-22T09:31:34.00
ORIGINATOR = ESA
MESSAGE_ID = ESA/20180422-001
OBJECT_NAME = SPACEOBJECT
INTERNATIONAL_DESIGNATOR = 2018-099B
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 81594
OBJECT_TYPE = ROCKET BODY
OBJECT_OWNER = ESA
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2018-04-22T09:00:00.00
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96: 36D 36O
ATMOSPHERIC_MODEL = NRLMSISE-00
N_BODY_PERTURBATIONS = MOON
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
INTRACK_THRUST = NO
REENTRY_DISINTEGRATION = MASS-LOSS + BREAK-UP
PREVIOUS_MESSAGE_ID = ESA/20180421-007
NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2018-04-27T19:45:33
REENTRY_WINDOW_START = 2018-04-27T11:45:33
REENTRY_WINDOW_END = 2018-04-27T22:12:56
PROBABILITY_OF_IMPACT = 0.0
PROBABILITY_OF_BURN_UP = 1.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.catalog_name,
            Some("SATCAT".to_string())
        );
        assert!(rdm
            .body
            .segment
            .data
            .atmospheric_reentry_parameters
            .nominal_reentry_epoch
            .is_some());
    }

    /// Parse official RDM XML example C-3 (minimal)
    #[test]
    fn test_xsd_rdm_sample_c3_xml() {
        let xml = std::fs::read_to_string("../data/xml/rdm_c3.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    }

    /// Parse official RDM XML example C-4 (comprehensive)
    #[test]
    fn test_xsd_rdm_sample_c4_xml() {
        let xml = std::fs::read_to_string("../data/xml/rdm_c4.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.header.message_id, "ESA/20180422-001");
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        assert!(rdm.body.segment.data.state_vector.is_some());
        assert!(rdm.body.segment.data.covariance_matrix.is_some());
        assert!(rdm.body.segment.data.spacecraft_parameters.is_some());
        assert!(rdm.body.segment.data.od_parameters.is_some());
    }

    // -----------------------------------------
    // Original basic tests
    // -----------------------------------------

    #[test]
    fn test_rdm_basic_kvn_roundtrip() {
        let kvn = r#"
CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST-CENTER
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.message_id, "RDM-001");
        assert_eq!(rdm.body.segment.metadata.object_name, "TEST-SAT");
        let kvn2 = rdm.to_kvn().unwrap();
        assert!(kvn2.contains("CCSDS_RDM_VERS"));
        assert!(kvn2.contains("ORBIT_LIFETIME"));
    }

    // Annex A (ICS): Header shall include CREATION_DATE, ORIGINATOR, MESSAGE_ID
    #[test]
    fn test_rdm_header_requires_fields() {
        let kvn_missing_creation = r#"
    CCSDS_RDM_VERS = 1.0
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_creation).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "CREATION_DATE"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_originator = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    MESSAGE_ID = RDM-001
    META_START
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    META_STOP
    DATA_START
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    DATA_STOP
    "#;
        let err = Rdm::from_kvn(kvn_missing_originator).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "ORIGINATOR"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_msgid = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_msgid).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "MESSAGE_ID"),
            _ => panic!("Unexpected: {:?}", err),
        }
    }

    // Annex A: Metadata shall contain OBJECT_NAME, INTERNATIONAL_DESIGNATOR, CENTER_NAME, TIME_SYSTEM
    #[test]
    fn test_rdm_metadata_requires_mandatory_fields() {
        let kvn_missing_object_name = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_object_name).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "OBJECT_NAME"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_intl = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_intl).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "INTERNATIONAL_DESIGNATOR"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_center = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_center).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "CENTER_NAME"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_timesys = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_timesys).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "TIME_SYSTEM"),
            _ => panic!("Unexpected: {:?}", err),
        }

        // Test CONTROLLED_REENTRY is mandatory (XSD minOccurs=1)
        let kvn_missing_controlled = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_controlled).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "CONTROLLED_REENTRY"),
            _ => panic!("Unexpected: {:?}", err),
        }

        // Test EPOCH_TZERO is mandatory (XSD minOccurs=1)
        let kvn_missing_epoch = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    ORBIT_LIFETIME = 1 [d]
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_epoch).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "EPOCH_TZERO"),
            _ => panic!("Unexpected: {:?}", err),
        }
    }

    // Annex A: Data (Atmospheric) shall include ORBIT_LIFETIME and REENTRY_ALTITUDE
    #[test]
    fn test_rdm_data_requires_atmospheric_fields() {
        let kvn_missing_orbit_life = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    REENTRY_ALTITUDE = 80 [km]
    "#;
        let err = Rdm::from_kvn(kvn_missing_orbit_life).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "ORBIT_LIFETIME"),
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_reentry_alt = r#"
    CCSDS_RDM_VERS = 1.0
    CREATION_DATE = 2023-11-13T12:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = RDM-001
    OBJECT_NAME = TEST
    INTERNATIONAL_DESIGNATOR = 2023-001A
    CONTROLLED_REENTRY = NO
    CENTER_NAME = TEST
    TIME_SYSTEM = UTC
    EPOCH_TZERO = 2023-01-01T00:00:00
    ORBIT_LIFETIME = 1 [d]
    "#;
        let err = Rdm::from_kvn(kvn_missing_reentry_alt).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "REENTRY_ALTITUDE"),
            _ => panic!("Unexpected: {:?}", err),
        }
    }
}
