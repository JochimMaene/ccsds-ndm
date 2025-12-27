// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdParameters;
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

//----------------------------------------------------------------------
// Root CDM Structure
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "cdm")]
pub struct Cdm {
    pub header: CdmHeader,
    pub body: CdmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Cdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        // 1. Header
        writer.write_pair("CCSDS_CDM_VERS", &self.version);
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
                    key: "CCSDS_CDM_VERS",
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
                        "CCSDS_CDM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        // 2. Header
        let header = CdmHeader::from_kvn_tokens(&mut tokens)?;

        // 3. Body
        let body = CdmBody::from_kvn_tokens(&mut tokens)?;

        Ok(Cdm {
            header,
            body,
            id: Some("CCSDS_CDM_VERS".to_string()),
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
pub struct CdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub creation_date: Epoch,
    pub originator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_for: Option<String>,
    pub message_id: String,
}

impl ToKvn for CdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(msg_for) = &self.message_for {
            writer.write_pair("MESSAGE_FOR", msg_for);
        }
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

impl FromKvnTokens for CdmHeader {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut creation_date = None;
        let mut originator = None;
        let mut message_for = None;
        let mut message_id = None;

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }

            match peeked.as_ref().unwrap() {
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair {
                    key: "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_FOR" | "MESSAGE_ID",
                    ..
                } => {
                    if let Some(Ok(KvnLine::Pair { key, val, .. })) = tokens.next() {
                        match key {
                            "CREATION_DATE" => creation_date = Some(Epoch::new(val)?),
                            "ORIGINATOR" => originator = Some(val.to_string()),
                            "MESSAGE_FOR" => message_for = Some(val.to_string()),
                            "MESSAGE_ID" => message_id = Some(val.to_string()),
                            _ => unreachable!(),
                        }
                    }
                }
                KvnLine::Pair { .. } => break, // Start of Relative Metadata
                _ => break,
            }
        }

        Ok(CdmHeader {
            comment,
            creation_date: creation_date
                .ok_or_else(|| CcsdsNdmError::MissingField("CREATION_DATE is required".into()))?,
            originator: originator
                .ok_or_else(|| CcsdsNdmError::MissingField("ORIGINATOR is required".into()))?,
            message_for,
            message_id: message_id
                .ok_or_else(|| CcsdsNdmError::MissingField("MESSAGE_ID is required".into()))?,
        })
    }
}

//----------------------------------------------------------------------
// Body
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CdmBody {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: RelativeMetadataData,
    #[serde(rename = "segment")]
    pub segments: Vec<CdmSegment>,
}

impl ToKvn for CdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.relative_metadata_data.write_kvn(writer);
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

impl CdmBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // 1. Relative Metadata (Reads until segment metadata starts)
        let relative_metadata_data = RelativeMetadataData::from_kvn_tokens(tokens)?;

        // 2. Segments (Expect exactly 2)
        let mut segments = Vec::new();
        let mut pending_comments = Vec::new();

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::Pair { key: "OBJECT", .. } => {
                    let mut segment = CdmSegment::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        segment
                            .metadata
                            .comment
                            .splice(0..0, pending_comments.drain(..));
                    }
                    segments.push(segment);
                }
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        pending_comments.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                _ => break,
            }
        }

        if segments.len() != 2 {
            return Err(CcsdsNdmError::Validation(format!(
                "CDM body must contain exactly 2 segments, found {}",
                segments.len()
            )));
        }

        Ok(CdmBody {
            relative_metadata_data,
            segments,
        })
    }
}

//----------------------------------------------------------------------
// Relative Metadata/Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeMetadataData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub tca: Epoch,
    pub miss_distance: Length,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_speed: Option<Dv>,
    #[serde(
        rename = "relativeStateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_state_vector: Option<RelativeStateVector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_screen_period: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_screen_period: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_frame: Option<ScreenVolumeFrameType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_shape: Option<ScreenVolumeShapeType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_x: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_y: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_z: Option<Length>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_entry_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_exit_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_probability: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_probability_method: Option<String>,
}

impl ToKvn for RelativeMetadataData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("TCA", &self.tca);
        writer.write_measure("MISS_DISTANCE", &self.miss_distance);
        if let Some(v) = &self.relative_speed {
            writer.write_measure("RELATIVE_SPEED", v);
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
            writer.write_pair("SCREEN_VOLUME_FRAME", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.screen_volume_shape {
            writer.write_pair("SCREEN_VOLUME_SHAPE", format!("{:?}", v).to_uppercase());
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

impl RelativeMetadataData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut tca = None;
        let mut miss_distance = None;
        let mut relative_speed = None;
        let mut rel_pos_r = None;
        let mut rel_pos_t = None;
        let mut rel_pos_n = None;
        let mut rel_vel_r = None;
        let mut rel_vel_t = None;
        let mut rel_vel_n = None;
        let mut start_screen = None;
        let mut stop_screen = None;
        let mut screen_frame = None;
        let mut screen_shape = None;
        let mut screen_x = None;
        let mut screen_y = None;
        let mut screen_z = None;
        let mut entry_time = None;
        let mut exit_time = None;
        let mut coll_prob = None;
        let mut coll_method = None;

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }

            match peeked.as_ref().unwrap() {
                KvnLine::BlockStart("META") => break,
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, unit } => {
                    if *key == "OBJECT" {
                        // Start of segment metadata in CDM 1.0 style (no META block)
                        break;
                    }
                    match *key {
                        "TCA" => tca = Some(Epoch::new(val)?),
                        "MISS_DISTANCE" => miss_distance = Some(Length::from_kvn(val, *unit)?),
                        "RELATIVE_SPEED" => relative_speed = Some(Dv::from_kvn(val, *unit)?),
                        "RELATIVE_POSITION_R" => rel_pos_r = Some(Length::from_kvn(val, *unit)?),
                        "RELATIVE_POSITION_T" => rel_pos_t = Some(Length::from_kvn(val, *unit)?),
                        "RELATIVE_POSITION_N" => rel_pos_n = Some(Length::from_kvn(val, *unit)?),
                        "RELATIVE_VELOCITY_R" => rel_vel_r = Some(Dv::from_kvn(val, *unit)?),
                        "RELATIVE_VELOCITY_T" => rel_vel_t = Some(Dv::from_kvn(val, *unit)?),
                        "RELATIVE_VELOCITY_N" => rel_vel_n = Some(Dv::from_kvn(val, *unit)?),
                        "START_SCREEN_PERIOD" => start_screen = Some(Epoch::new(val)?),
                        "STOP_SCREEN_PERIOD" => stop_screen = Some(Epoch::new(val)?),
                        "SCREEN_VOLUME_FRAME" => {
                            screen_frame = Some(match val.to_uppercase().as_str() {
                                "RTN" => ScreenVolumeFrameType::Rtn,
                                "TVN" => ScreenVolumeFrameType::Tvn,
                                _ => {
                                    return Err(CcsdsNdmError::Validation(format!(
                                        "Invalid SCREEN_VOLUME_FRAME: {}",
                                        val
                                    )))
                                }
                            })
                        }
                        "SCREEN_VOLUME_SHAPE" => {
                            screen_shape = Some(match val.to_uppercase().as_str() {
                                "ELLIPSOID" => ScreenVolumeShapeType::Ellipsoid,
                                "BOX" => ScreenVolumeShapeType::Box,
                                _ => {
                                    return Err(CcsdsNdmError::Validation(format!(
                                        "Invalid SCREEN_VOLUME_SHAPE: {}",
                                        val
                                    )))
                                }
                            })
                        }
                        "SCREEN_VOLUME_X" => screen_x = Some(Length::from_kvn(val, *unit)?),
                        "SCREEN_VOLUME_Y" => screen_y = Some(Length::from_kvn(val, *unit)?),
                        "SCREEN_VOLUME_Z" => screen_z = Some(Length::from_kvn(val, *unit)?),
                        "SCREEN_ENTRY_TIME" => entry_time = Some(Epoch::new(val)?),
                        "SCREEN_EXIT_TIME" => exit_time = Some(Epoch::new(val)?),
                        "COLLISION_PROBABILITY" => {
                            coll_prob = Some(Probability::new(val.parse().map_err(
                                |e: std::num::ParseFloatError| {
                                    CcsdsNdmError::KvnParse(e.to_string())
                                },
                            )?)?);
                        }
                        "COLLISION_PROBABILITY_METHOD" => coll_method = Some(val.to_string()),
                        _ => {
                            return Err(CcsdsNdmError::KvnParse(format!(
                                "Unexpected field in Relative Metadata: {}",
                                key
                            )))
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        let relative_state_vector =
            if rel_pos_r.is_some() || rel_pos_t.is_some() || rel_pos_n.is_some() {
                Some(RelativeStateVector {
                    relative_position_r: rel_pos_r
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_POSITION_R".into()))?,
                    relative_position_t: rel_pos_t
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_POSITION_T".into()))?,
                    relative_position_n: rel_pos_n
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_POSITION_N".into()))?,
                    relative_velocity_r: rel_vel_r
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_VELOCITY_R".into()))?,
                    relative_velocity_t: rel_vel_t
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_VELOCITY_T".into()))?,
                    relative_velocity_n: rel_vel_n
                        .ok_or(CcsdsNdmError::MissingField("RELATIVE_VELOCITY_N".into()))?,
                })
            } else {
                None
            };

        Ok(RelativeMetadataData {
            comment,
            tca: tca.ok_or(CcsdsNdmError::MissingField("TCA".into()))?,
            miss_distance: miss_distance
                .ok_or(CcsdsNdmError::MissingField("MISS_DISTANCE".into()))?,
            relative_speed,
            relative_state_vector,
            start_screen_period: start_screen,
            stop_screen_period: stop_screen,
            screen_volume_frame: screen_frame,
            screen_volume_shape: screen_shape,
            screen_volume_x: screen_x,
            screen_volume_y: screen_y,
            screen_volume_z: screen_z,
            screen_entry_time: entry_time,
            screen_exit_time: exit_time,
            collision_probability: coll_prob,
            collision_probability_method: coll_method,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        writer.write_measure("RELATIVE_VELOCITY_R", &self.relative_velocity_r);
        writer.write_measure("RELATIVE_VELOCITY_T", &self.relative_velocity_t);
        writer.write_measure("RELATIVE_VELOCITY_N", &self.relative_velocity_n);
    }
}

//----------------------------------------------------------------------
// Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CdmSegment {
    pub metadata: CdmMetadata,
    pub data: CdmData,
}

impl ToKvn for CdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl CdmSegment {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // Support both CDM 2.0 with META blocks and CDM 1.0 without
        let metadata = match tokens.peek() {
            Some(Ok(KvnLine::Pair { key: "OBJECT", .. })) => CdmMetadata::from_kvn_tokens(tokens)?,
            Some(Ok(other)) => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Expected segment start, found {:?}",
                    other
                )))
            }
            Some(Err(_)) => {
                // Advance and return the owned error
                if let Some(Err(err)) = tokens.next() {
                    return Err(err);
                }
                unreachable!();
            }
            None => return Err(CcsdsNdmError::KvnParse("Unexpected end of input".into())),
        };
        let data = CdmData::from_kvn_tokens(tokens)?;

        Ok(CdmSegment { metadata, data })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object: CdmObjectType,
    pub object_designator: String,
    pub catalog_name: String,
    pub object_name: String,
    pub international_designator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_contact_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_email: Option<String>,
    pub ephemeris_name: String,
    pub covariance_method: CovarianceMethodType,
    pub maneuverable: ManeuverableType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_center: Option<String>,
    pub ref_frame: ReferenceFrameType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_pressure: Option<YesNo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earth_tides: Option<YesNo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
                _ => "OBJECT1",
            },
        );
        writer.write_pair("OBJECT_DESIGNATOR", &self.object_designator);
        writer.write_pair("CATALOG_NAME", &self.catalog_name);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", format!("{:?}", v).to_uppercase());
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
        writer.write_pair(
            "COVARIANCE_METHOD",
            format!("{:?}", self.covariance_method).to_uppercase(),
        );
        writer.write_pair(
            "MANEUVERABLE",
            format!("{:?}", self.maneuverable).to_uppercase(),
        );
        if let Some(v) = &self.orbit_center {
            writer.write_pair("ORBIT_CENTER", v);
        }
        writer.write_pair("REF_FRAME", format!("{:?}", self.ref_frame).to_uppercase());
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
            writer.write_pair("SOLAR_RAD_PRESSURE", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", format!("{:?}", v).to_uppercase());
        }
    }
}

impl CdmMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut builder = CdmMetadataBuilder::default();
        // Helper to detect start of DATA section
        fn is_data_key(key: &str) -> bool {
            matches!(
                key,
                "TIME_LASTOB_START"
                    | "TIME_LASTOB_END"
                    | "RECOMMENDED_OD_SPAN"
                    | "ACTUAL_OD_SPAN"
                    | "OBS_AVAILABLE"
                    | "OBS_USED"
                    | "TRACKS_AVAILABLE"
                    | "TRACKS_USED"
                    | "RESIDUALS_ACCEPTED"
                    | "WEIGHTED_RMS"
                    | "AREA_PC"
                    | "AREA_DRG"
                    | "AREA_SRP"
                    | "MASS"
                    | "CD_AREA_OVER_MASS"
                    | "CR_AREA_OVER_MASS"
                    | "THRUST_ACCELERATION"
                    | "SEDR"
                    | "X"
                    | "Y"
                    | "Z"
                    | "X_DOT"
                    | "Y_DOT"
                    | "Z_DOT"
                    | "CR_R"
                    | "CT_R"
                    | "CT_T"
                    | "CN_R"
                    | "CN_T"
                    | "CN_N"
                    | "CRDOT_R"
                    | "CRDOT_T"
                    | "CRDOT_N"
                    | "CRDOT_RDOT"
                    | "CTDOT_R"
                    | "CTDOT_T"
                    | "CTDOT_N"
                    | "CTDOT_RDOT"
                    | "CTDOT_TDOT"
                    | "CNDOT_R"
                    | "CNDOT_T"
                    | "CNDOT_N"
                    | "CNDOT_RDOT"
                    | "CNDOT_TDOT"
                    | "CNDOT_NDOT"
                    | "CDRG_R"
                    | "CDRG_T"
                    | "CDRG_N"
                    | "CDRG_RDOT"
                    | "CDRG_TDOT"
                    | "CDRG_NDOT"
                    | "CDRG_DRG"
                    | "CSRP_R"
                    | "CSRP_T"
                    | "CSRP_N"
                    | "CSRP_RDOT"
                    | "CSRP_TDOT"
                    | "CSRP_NDOT"
                    | "CSRP_DRG"
                    | "CSRP_SRP"
            )
        }

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::BlockEnd("META") => {
                    tokens.next();
                    break;
                }
                KvnLine::Comment(c) => {
                    builder.comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    if is_data_key(key) {
                        // Start of data section; do not consume, let data parser handle
                        break;
                    }
                    builder.match_pair(key, val)?;
                    tokens.next();
                }
                t => {
                    return Err(CcsdsNdmError::KvnParse(format!(
                        "Unexpected token in metadata: {:?}",
                        t
                    )))
                }
            }
        }
        builder.build()
    }
}

#[derive(Default)]
struct CdmMetadataBuilder {
    comment: Vec<String>,
    object: Option<CdmObjectType>,
    object_designator: Option<String>,
    catalog_name: Option<String>,
    object_name: Option<String>,
    international_designator: Option<String>,
    object_type: Option<ObjectDescription>,
    operator_contact_position: Option<String>,
    operator_organization: Option<String>,
    operator_phone: Option<String>,
    operator_email: Option<String>,
    ephemeris_name: Option<String>,
    covariance_method: Option<CovarianceMethodType>,
    maneuverable: Option<ManeuverableType>,
    orbit_center: Option<String>,
    ref_frame: Option<ReferenceFrameType>,
    gravity_model: Option<String>,
    atmospheric_model: Option<String>,
    n_body_perturbations: Option<String>,
    solar_rad_pressure: Option<YesNo>,
    earth_tides: Option<YesNo>,
    intrack_thrust: Option<YesNo>,
}

impl CdmMetadataBuilder {
    fn match_pair(&mut self, key: &str, val: &str) -> Result<()> {
        match key {
            "OBJECT" => {
                self.object = Some(match val.to_uppercase().as_str() {
                    "OBJECT1" => CdmObjectType::Object1,
                    "OBJECT2" => CdmObjectType::Object2,
                    _ => {
                        return Err(CcsdsNdmError::Validation(format!(
                            "Invalid OBJECT: {}",
                            val
                        )))
                    }
                })
            }
            "OBJECT_DESIGNATOR" => self.object_designator = Some(val.into()),
            "CATALOG_NAME" => self.catalog_name = Some(val.into()),
            "OBJECT_NAME" => self.object_name = Some(val.into()),
            "INTERNATIONAL_DESIGNATOR" => self.international_designator = Some(val.into()),
            "OBJECT_TYPE" => {
                self.object_type = Some(match val.to_uppercase().as_str() {
                    "PAYLOAD" => ObjectDescription::Payload,
                    "ROCKET BODY" => ObjectDescription::RocketBody,
                    "DEBRIS" => ObjectDescription::Debris,
                    "UNKNOWN" => ObjectDescription::Unknown,
                    "OTHER" => ObjectDescription::Other,
                    _ => ObjectDescription::Other,
                })
            }
            "OPERATOR_CONTACT_POSITION" => self.operator_contact_position = Some(val.into()),
            "OPERATOR_ORGANIZATION" => self.operator_organization = Some(val.into()),
            "OPERATOR_PHONE" => self.operator_phone = Some(val.into()),
            "OPERATOR_EMAIL" => self.operator_email = Some(val.into()),
            "EPHEMERIS_NAME" => self.ephemeris_name = Some(val.into()),
            "COVARIANCE_METHOD" => {
                self.covariance_method = Some(match val.to_uppercase().as_str() {
                    "CALCULATED" => CovarianceMethodType::Calculated,
                    "DEFAULT" => CovarianceMethodType::Default,
                    _ => {
                        return Err(CcsdsNdmError::Validation(format!(
                            "Invalid COV_METHOD: {}",
                            val
                        )))
                    }
                })
            }
            "MANEUVERABLE" => {
                self.maneuverable = Some(match val.to_uppercase().as_str() {
                    "YES" => ManeuverableType::Yes,
                    "NO" => ManeuverableType::No,
                    "N/A" => ManeuverableType::NA,
                    _ => {
                        return Err(CcsdsNdmError::Validation(format!(
                            "Invalid MANEUVERABLE: {}",
                            val
                        )))
                    }
                })
            }
            "ORBIT_CENTER" => self.orbit_center = Some(val.into()),
            "REF_FRAME" => {
                self.ref_frame = Some(match val.to_uppercase().as_str() {
                    "EME2000" => ReferenceFrameType::Eme2000,
                    "GCRF" => ReferenceFrameType::Gcrf,
                    "ITRF" => ReferenceFrameType::Itrf,
                    _ => {
                        return Err(CcsdsNdmError::Validation(format!(
                            "Invalid REF_FRAME: {}",
                            val
                        )))
                    }
                })
            }
            "GRAVITY_MODEL" => self.gravity_model = Some(val.into()),
            "ATMOSPHERIC_MODEL" => self.atmospheric_model = Some(val.into()),
            "N_BODY_PERTURBATIONS" => self.n_body_perturbations = Some(val.into()),
            "SOLAR_RAD_PRESSURE" => {
                self.solar_rad_pressure = Some(if val.eq_ignore_ascii_case("YES") {
                    YesNo::Yes
                } else {
                    YesNo::No
                })
            }
            "EARTH_TIDES" => {
                self.earth_tides = Some(if val.eq_ignore_ascii_case("YES") {
                    YesNo::Yes
                } else {
                    YesNo::No
                })
            }
            "INTRACK_THRUST" => {
                self.intrack_thrust = Some(if val.eq_ignore_ascii_case("YES") {
                    YesNo::Yes
                } else {
                    YesNo::No
                })
            }
            _ => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Unknown META key: {}",
                    key
                )))
            }
        }
        Ok(())
    }

    fn build(self) -> Result<CdmMetadata> {
        Ok(CdmMetadata {
            comment: self.comment,
            object: self
                .object
                .ok_or(CcsdsNdmError::MissingField("OBJECT".into()))?,
            object_designator: self
                .object_designator
                .ok_or(CcsdsNdmError::MissingField("OBJECT_DESIGNATOR".into()))?,
            catalog_name: self
                .catalog_name
                .ok_or(CcsdsNdmError::MissingField("CATALOG_NAME".into()))?,
            object_name: self
                .object_name
                .ok_or(CcsdsNdmError::MissingField("OBJECT_NAME".into()))?,
            international_designator: self.international_designator.ok_or(
                CcsdsNdmError::MissingField("INTERNATIONAL_DESIGNATOR".into()),
            )?,
            object_type: self.object_type,
            operator_contact_position: self.operator_contact_position,
            operator_organization: self.operator_organization,
            operator_phone: self.operator_phone,
            operator_email: self.operator_email,
            ephemeris_name: self
                .ephemeris_name
                .ok_or(CcsdsNdmError::MissingField("EPHEMERIS_NAME".into()))?,
            covariance_method: self
                .covariance_method
                .ok_or(CcsdsNdmError::MissingField("COVARIANCE_METHOD".into()))?,
            maneuverable: self
                .maneuverable
                .ok_or(CcsdsNdmError::MissingField("MANEUVERABLE".into()))?,
            orbit_center: self.orbit_center,
            ref_frame: self
                .ref_frame
                .ok_or(CcsdsNdmError::MissingField("REF_FRAME".into()))?,
            gravity_model: self.gravity_model,
            atmospheric_model: self.atmospheric_model,
            n_body_perturbations: self.n_body_perturbations,
            solar_rad_pressure: self.solar_rad_pressure,
            earth_tides: self.earth_tides,
            intrack_thrust: self.intrack_thrust,
        })
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    #[serde(
        rename = "additionalParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parameters: Option<AdditionalParameters>,
    #[serde(rename = "stateVector")]
    pub state_vector: CdmStateVector,
    #[serde(rename = "covarianceMatrix")]
    pub covariance_matrix: CdmCovarianceMatrix,
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
                writer.write_measure("CD_AREA_OVER_MASS", v);
            }
            if let Some(v) = &ap.cr_area_over_mass {
                writer.write_measure("CR_AREA_OVER_MASS", v);
            }
            if let Some(v) = &ap.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", v);
            }
            if let Some(v) = &ap.sedr {
                writer.write_measure("SEDR", v);
            }
        }
        // State Vector
        self.state_vector.write_kvn(writer);
        // Covariance
        self.covariance_matrix.write_kvn(writer);
    }
}

impl CdmData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut od_params = OdParameters::default();
        let mut add_params = AdditionalParameters::default();

        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut x_dot = None;
        let mut y_dot = None;
        let mut z_dot = None;

        // Using a Builder pattern locally for Covariance because strict requirements exist
        let mut cov = CdmCovarianceMatrixBuilder::default();

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::BlockStart("META") => break,
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, unit } => {
                    // In CDM 1.0 style, a new segment starts when encountering OBJECT.
                    // Data section must stop here and let the next segment parse metadata.
                    if *key == "OBJECT" {
                        break;
                    }
                    match *key {
                        // OD Parameters
                        "TIME_LASTOB_START" => od_params.time_lastob_start = Some(Epoch::new(val)?),
                        "TIME_LASTOB_END" => od_params.time_lastob_end = Some(Epoch::new(val)?),
                        "RECOMMENDED_OD_SPAN" => {
                            od_params.recommended_od_span = Some(DayInterval::from_kvn(val, *unit)?)
                        }
                        "ACTUAL_OD_SPAN" => {
                            od_params.actual_od_span = Some(DayInterval::from_kvn(val, *unit)?)
                        }
                        "OBS_AVAILABLE" => od_params.obs_available = Some(val.parse()?),
                        "OBS_USED" => od_params.obs_used = Some(val.parse()?),
                        "TRACKS_AVAILABLE" => od_params.tracks_available = Some(val.parse()?),
                        "TRACKS_USED" => od_params.tracks_used = Some(val.parse()?),
                        "RESIDUALS_ACCEPTED" => {
                            od_params.residuals_accepted = Some(Percentage::from_kvn(val, *unit)?)
                        }
                        "WEIGHTED_RMS" => od_params.weighted_rms = Some(val.parse()?),

                        // Additional Parameters
                        "AREA_PC" => add_params.area_pc = Some(Area::from_kvn(val, *unit)?),
                        "AREA_DRG" => add_params.area_drg = Some(Area::from_kvn(val, *unit)?),
                        "AREA_SRP" => add_params.area_srp = Some(Area::from_kvn(val, *unit)?),
                        "MASS" => add_params.mass = Some(Mass::from_kvn(val, *unit)?),
                        "CD_AREA_OVER_MASS" => {
                            add_params.cd_area_over_mass = Some(M2kg::from_kvn(val, *unit)?)
                        }
                        "CR_AREA_OVER_MASS" => {
                            add_params.cr_area_over_mass = Some(M2kg::from_kvn(val, *unit)?)
                        }
                        "THRUST_ACCELERATION" => {
                            add_params.thrust_acceleration = Some(Ms2::from_kvn(val, *unit)?)
                        }
                        "SEDR" => add_params.sedr = Some(Wkg::from_kvn(val, *unit)?),

                        // State Vector
                        "X" => {
                            x = Some(PositionRequired {
                                value: val.parse()?,
                                units: PositionUnits::Km,
                            })
                        }
                        "Y" => {
                            y = Some(PositionRequired {
                                value: val.parse()?,
                                units: PositionUnits::Km,
                            })
                        }
                        "Z" => {
                            z = Some(PositionRequired {
                                value: val.parse()?,
                                units: PositionUnits::Km,
                            })
                        }
                        "X_DOT" => {
                            x_dot = Some(VelocityRequired {
                                value: val.parse()?,
                                units: VelocityUnits::KmPerS,
                            })
                        }
                        "Y_DOT" => {
                            y_dot = Some(VelocityRequired {
                                value: val.parse()?,
                                units: VelocityUnits::KmPerS,
                            })
                        }
                        "Z_DOT" => {
                            z_dot = Some(VelocityRequired {
                                value: val.parse()?,
                                units: VelocityUnits::KmPerS,
                            })
                        }

                        // Covariance - delegate to builder
                        _ => {
                            if !cov.try_match_pair(key, val, *unit)? {
                                return Err(CcsdsNdmError::KvnParse(format!(
                                    "Unexpected field in Segment Data: {}",
                                    key
                                )));
                            }
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        let state_vector = CdmStateVector {
            x: x.ok_or(CcsdsNdmError::MissingField("X".into()))?,
            y: y.ok_or(CcsdsNdmError::MissingField("Y".into()))?,
            z: z.ok_or(CcsdsNdmError::MissingField("Z".into()))?,
            x_dot: x_dot.ok_or(CcsdsNdmError::MissingField("X_DOT".into()))?,
            y_dot: y_dot.ok_or(CcsdsNdmError::MissingField("Y_DOT".into()))?,
            z_dot: z_dot.ok_or(CcsdsNdmError::MissingField("Z_DOT".into()))?,
        };

        let od_parameters = if od_params.time_lastob_start.is_some() || od_params.obs_used.is_some()
        {
            Some(od_params)
        } else {
            None
        };
        let additional_parameters = if add_params.mass.is_some() {
            Some(add_params)
        } else {
            None
        };

        Ok(CdmData {
            comment,
            od_parameters,
            additional_parameters,
            state_vector,
            covariance_matrix: cov.build()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdditionalParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_pc: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_drg: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_srp: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd_area_over_mass: Option<M2kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cr_area_over_mass: Option<M2kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrust_acceleration: Option<Ms2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedr: Option<Wkg>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmStateVector {
    pub x: PositionRequired,
    pub y: PositionRequired,
    pub z: PositionRequired,
    pub x_dot: VelocityRequired,
    pub y_dot: VelocityRequired,
    pub z_dot: VelocityRequired,
}

impl ToKvn for CdmStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_measure("X", &self.x.to_unit_value());
        writer.write_measure("Y", &self.y.to_unit_value());
        writer.write_measure("Z", &self.z.to_unit_value());
        writer.write_measure("X_DOT", &self.x_dot.to_unit_value());
        writer.write_measure("Y_DOT", &self.y_dot.to_unit_value());
        writer.write_measure("Z_DOT", &self.z_dot.to_unit_value());
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmCovarianceMatrix {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub cr_r: M2,
    pub ct_r: M2,
    pub ct_t: M2,
    pub cn_r: M2,
    pub cn_t: M2,
    pub cn_n: M2,
    pub crdot_r: M2s,
    pub crdot_t: M2s,
    pub crdot_n: M2s,
    pub crdot_rdot: M2s2,
    pub ctdot_r: M2s,
    pub ctdot_t: M2s,
    pub ctdot_n: M2s,
    pub ctdot_rdot: M2s2,
    pub ctdot_tdot: M2s2,
    pub cndot_r: M2s,
    pub cndot_t: M2s,
    pub cndot_n: M2s,
    pub cndot_rdot: M2s2,
    pub cndot_tdot: M2s2,
    pub cndot_ndot: M2s2,

    // Optional terms
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_r: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_t: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_n: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_rdot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_tdot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_ndot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_drg: Option<M4kg2>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_r: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_t: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_n: Option<M3kg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_rdot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_tdot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_ndot: Option<M3kgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_drg: Option<M4kg2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_srp: Option<M4kg2>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_r: Option<M2s2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_t: Option<M2s2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_n: Option<M2s2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_rdot: Option<M2s3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_tdot: Option<M2s3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_ndot: Option<M2s3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_drg: Option<M3kgs2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_srp: Option<M3kgs2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_thr: Option<M2s4>,
}

#[derive(Default)]
struct CdmCovarianceMatrixBuilder {
    comment: Vec<String>,
    cr_r: Option<M2>,
    ct_r: Option<M2>,
    ct_t: Option<M2>,
    cn_r: Option<M2>,
    cn_t: Option<M2>,
    cn_n: Option<M2>,
    crdot_r: Option<M2s>,
    crdot_t: Option<M2s>,
    crdot_n: Option<M2s>,
    crdot_rdot: Option<M2s2>,
    ctdot_r: Option<M2s>,
    ctdot_t: Option<M2s>,
    ctdot_n: Option<M2s>,
    ctdot_rdot: Option<M2s2>,
    ctdot_tdot: Option<M2s2>,
    cndot_r: Option<M2s>,
    cndot_t: Option<M2s>,
    cndot_n: Option<M2s>,
    cndot_rdot: Option<M2s2>,
    cndot_tdot: Option<M2s2>,
    cndot_ndot: Option<M2s2>,

    cdrg_r: Option<M3kg>,
    cdrg_t: Option<M3kg>,
    cdrg_n: Option<M3kg>,
    cdrg_rdot: Option<M3kgs>,
    cdrg_tdot: Option<M3kgs>,
    cdrg_ndot: Option<M3kgs>,
    cdrg_drg: Option<M4kg2>,
    csrp_r: Option<M3kg>,
    csrp_t: Option<M3kg>,
    csrp_n: Option<M3kg>,
    csrp_rdot: Option<M3kgs>,
    csrp_tdot: Option<M3kgs>,
    csrp_ndot: Option<M3kgs>,
    csrp_drg: Option<M4kg2>,
    csrp_srp: Option<M4kg2>,
    cthr_r: Option<M2s2>,
    cthr_t: Option<M2s2>,
    cthr_n: Option<M2s2>,
    cthr_rdot: Option<M2s3>,
    cthr_tdot: Option<M2s3>,
    cthr_ndot: Option<M2s3>,
    cthr_drg: Option<M3kgs2>,
    cthr_srp: Option<M3kgs2>,
    cthr_thr: Option<M2s4>,
}

impl CdmCovarianceMatrixBuilder {
    fn try_match_pair(&mut self, key: &str, val: &str, unit: Option<&str>) -> Result<bool> {
        match key {
            "CR_R" => self.cr_r = Some(M2::from_kvn(val, unit)?),
            "CT_R" => self.ct_r = Some(M2::from_kvn(val, unit)?),
            "CT_T" => self.ct_t = Some(M2::from_kvn(val, unit)?),
            "CN_R" => self.cn_r = Some(M2::from_kvn(val, unit)?),
            "CN_T" => self.cn_t = Some(M2::from_kvn(val, unit)?),
            "CN_N" => self.cn_n = Some(M2::from_kvn(val, unit)?),
            "CRDOT_R" => self.crdot_r = Some(M2s::from_kvn(val, unit)?),
            "CRDOT_T" => self.crdot_t = Some(M2s::from_kvn(val, unit)?),
            "CRDOT_N" => self.crdot_n = Some(M2s::from_kvn(val, unit)?),
            "CRDOT_RDOT" => self.crdot_rdot = Some(M2s2::from_kvn(val, unit)?),
            "CTDOT_R" => self.ctdot_r = Some(M2s::from_kvn(val, unit)?),
            "CTDOT_T" => self.ctdot_t = Some(M2s::from_kvn(val, unit)?),
            "CTDOT_N" => self.ctdot_n = Some(M2s::from_kvn(val, unit)?),
            "CTDOT_RDOT" => self.ctdot_rdot = Some(M2s2::from_kvn(val, unit)?),
            "CTDOT_TDOT" => self.ctdot_tdot = Some(M2s2::from_kvn(val, unit)?),
            "CNDOT_R" => self.cndot_r = Some(M2s::from_kvn(val, unit)?),
            "CNDOT_T" => self.cndot_t = Some(M2s::from_kvn(val, unit)?),
            "CNDOT_N" => self.cndot_n = Some(M2s::from_kvn(val, unit)?),
            "CNDOT_RDOT" => self.cndot_rdot = Some(M2s2::from_kvn(val, unit)?),
            "CNDOT_TDOT" => self.cndot_tdot = Some(M2s2::from_kvn(val, unit)?),
            "CNDOT_NDOT" => self.cndot_ndot = Some(M2s2::from_kvn(val, unit)?),

            "CDRG_R" => self.cdrg_r = Some(M3kg::from_kvn(val, unit)?),
            "CDRG_T" => self.cdrg_t = Some(M3kg::from_kvn(val, unit)?),
            "CDRG_N" => self.cdrg_n = Some(M3kg::from_kvn(val, unit)?),
            "CDRG_RDOT" => self.cdrg_rdot = Some(M3kgs::from_kvn(val, unit)?),
            "CDRG_TDOT" => self.cdrg_tdot = Some(M3kgs::from_kvn(val, unit)?),
            "CDRG_NDOT" => self.cdrg_ndot = Some(M3kgs::from_kvn(val, unit)?),
            "CDRG_DRG" => self.cdrg_drg = Some(M4kg2::from_kvn(val, unit)?),

            "CSRP_R" => self.csrp_r = Some(M3kg::from_kvn(val, unit)?),
            "CSRP_T" => self.csrp_t = Some(M3kg::from_kvn(val, unit)?),
            "CSRP_N" => self.csrp_n = Some(M3kg::from_kvn(val, unit)?),
            "CSRP_RDOT" => self.csrp_rdot = Some(M3kgs::from_kvn(val, unit)?),
            "CSRP_TDOT" => self.csrp_tdot = Some(M3kgs::from_kvn(val, unit)?),
            "CSRP_NDOT" => self.csrp_ndot = Some(M3kgs::from_kvn(val, unit)?),
            "CSRP_DRG" => self.csrp_drg = Some(M4kg2::from_kvn(val, unit)?),
            "CSRP_SRP" => self.csrp_srp = Some(M4kg2::from_kvn(val, unit)?),

            "CTHR_R" => self.cthr_r = Some(M2s2::from_kvn(val, unit)?),
            "CTHR_T" => self.cthr_t = Some(M2s2::from_kvn(val, unit)?),
            "CTHR_N" => self.cthr_n = Some(M2s2::from_kvn(val, unit)?),
            "CTHR_RDOT" => self.cthr_rdot = Some(M2s3::from_kvn(val, unit)?),
            "CTHR_TDOT" => self.cthr_tdot = Some(M2s3::from_kvn(val, unit)?),
            "CTHR_NDOT" => self.cthr_ndot = Some(M2s3::from_kvn(val, unit)?),
            "CTHR_DRG" => self.cthr_drg = Some(M3kgs2::from_kvn(val, unit)?),
            "CTHR_SRP" => self.cthr_srp = Some(M3kgs2::from_kvn(val, unit)?),
            "CTHR_THR" => self.cthr_thr = Some(M2s4::from_kvn(val, unit)?),
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn build(self) -> Result<CdmCovarianceMatrix> {
        Ok(CdmCovarianceMatrix {
            comment: self.comment,
            cr_r: self
                .cr_r
                .ok_or(CcsdsNdmError::MissingField("CR_R".into()))?,
            ct_r: self
                .ct_r
                .ok_or(CcsdsNdmError::MissingField("CT_R".into()))?,
            ct_t: self
                .ct_t
                .ok_or(CcsdsNdmError::MissingField("CT_T".into()))?,
            cn_r: self
                .cn_r
                .ok_or(CcsdsNdmError::MissingField("CN_R".into()))?,
            cn_t: self
                .cn_t
                .ok_or(CcsdsNdmError::MissingField("CN_T".into()))?,
            cn_n: self
                .cn_n
                .ok_or(CcsdsNdmError::MissingField("CN_N".into()))?,
            crdot_r: self
                .crdot_r
                .ok_or(CcsdsNdmError::MissingField("CRDOT_R".into()))?,
            crdot_t: self
                .crdot_t
                .ok_or(CcsdsNdmError::MissingField("CRDOT_T".into()))?,
            crdot_n: self
                .crdot_n
                .ok_or(CcsdsNdmError::MissingField("CRDOT_N".into()))?,
            crdot_rdot: self
                .crdot_rdot
                .ok_or(CcsdsNdmError::MissingField("CRDOT_RDOT".into()))?,
            ctdot_r: self
                .ctdot_r
                .ok_or(CcsdsNdmError::MissingField("CTDOT_R".into()))?,
            ctdot_t: self
                .ctdot_t
                .ok_or(CcsdsNdmError::MissingField("CTDOT_T".into()))?,
            ctdot_n: self
                .ctdot_n
                .ok_or(CcsdsNdmError::MissingField("CTDOT_N".into()))?,
            ctdot_rdot: self
                .ctdot_rdot
                .ok_or(CcsdsNdmError::MissingField("CTDOT_RDOT".into()))?,
            ctdot_tdot: self
                .ctdot_tdot
                .ok_or(CcsdsNdmError::MissingField("CTDOT_TDOT".into()))?,
            cndot_r: self
                .cndot_r
                .ok_or(CcsdsNdmError::MissingField("CNDOT_R".into()))?,
            cndot_t: self
                .cndot_t
                .ok_or(CcsdsNdmError::MissingField("CNDOT_T".into()))?,
            cndot_n: self
                .cndot_n
                .ok_or(CcsdsNdmError::MissingField("CNDOT_N".into()))?,
            cndot_rdot: self
                .cndot_rdot
                .ok_or(CcsdsNdmError::MissingField("CNDOT_RDOT".into()))?,
            cndot_tdot: self
                .cndot_tdot
                .ok_or(CcsdsNdmError::MissingField("CNDOT_TDOT".into()))?,
            cndot_ndot: self
                .cndot_ndot
                .ok_or(CcsdsNdmError::MissingField("CNDOT_NDOT".into()))?,

            cdrg_r: self.cdrg_r,
            cdrg_t: self.cdrg_t,
            cdrg_n: self.cdrg_n,
            cdrg_rdot: self.cdrg_rdot,
            cdrg_tdot: self.cdrg_tdot,
            cdrg_ndot: self.cdrg_ndot,
            cdrg_drg: self.cdrg_drg,
            csrp_r: self.csrp_r,
            csrp_t: self.csrp_t,
            csrp_n: self.csrp_n,
            csrp_rdot: self.csrp_rdot,
            csrp_tdot: self.csrp_tdot,
            csrp_ndot: self.csrp_ndot,
            csrp_drg: self.csrp_drg,
            csrp_srp: self.csrp_srp,
            cthr_r: self.cthr_r,
            cthr_t: self.cthr_t,
            cthr_n: self.cthr_n,
            cthr_rdot: self.cthr_rdot,
            cthr_tdot: self.cthr_tdot,
            cthr_ndot: self.cthr_ndot,
            cthr_drg: self.cthr_drg,
            cthr_srp: self.cthr_srp,
            cthr_thr: self.cthr_thr,
        })
    }
}

impl ToKvn for CdmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Required
        writer.write_measure("CR_R", &self.cr_r);
        writer.write_measure("CT_R", &self.ct_r);
        writer.write_measure("CT_T", &self.ct_t);
        writer.write_measure("CN_R", &self.cn_r);
        writer.write_measure("CN_T", &self.cn_t);
        writer.write_measure("CN_N", &self.cn_n);
        writer.write_measure("CRDOT_R", &self.crdot_r);
        writer.write_measure("CRDOT_T", &self.crdot_t);
        writer.write_measure("CRDOT_N", &self.crdot_n);
        writer.write_measure("CRDOT_RDOT", &self.crdot_rdot);
        writer.write_measure("CTDOT_R", &self.ctdot_r);
        writer.write_measure("CTDOT_T", &self.ctdot_t);
        writer.write_measure("CTDOT_N", &self.ctdot_n);
        writer.write_measure("CTDOT_RDOT", &self.ctdot_rdot);
        writer.write_measure("CTDOT_TDOT", &self.ctdot_tdot);
        writer.write_measure("CNDOT_R", &self.cndot_r);
        writer.write_measure("CNDOT_T", &self.cndot_t);
        writer.write_measure("CNDOT_N", &self.cndot_n);
        writer.write_measure("CNDOT_RDOT", &self.cndot_rdot);
        writer.write_measure("CNDOT_TDOT", &self.cndot_tdot);
        writer.write_measure("CNDOT_NDOT", &self.cndot_ndot);

        // Optionals
        if let Some(v) = &self.cdrg_r {
            writer.write_measure("CDRG_R", v);
        }
        if let Some(v) = &self.cdrg_t {
            writer.write_measure("CDRG_T", v);
        }
        if let Some(v) = &self.cdrg_n {
            writer.write_measure("CDRG_N", v);
        }
        if let Some(v) = &self.cdrg_rdot {
            writer.write_measure("CDRG_RDOT", v);
        }
        if let Some(v) = &self.cdrg_tdot {
            writer.write_measure("CDRG_TDOT", v);
        }
        if let Some(v) = &self.cdrg_ndot {
            writer.write_measure("CDRG_NDOT", v);
        }
        if let Some(v) = &self.cdrg_drg {
            writer.write_measure("CDRG_DRG", v);
        }

        if let Some(v) = &self.csrp_r {
            writer.write_measure("CSRP_R", v);
        }
        if let Some(v) = &self.csrp_t {
            writer.write_measure("CSRP_T", v);
        }
        if let Some(v) = &self.csrp_n {
            writer.write_measure("CSRP_N", v);
        }
        if let Some(v) = &self.csrp_rdot {
            writer.write_measure("CSRP_RDOT", v);
        }
        if let Some(v) = &self.csrp_tdot {
            writer.write_measure("CSRP_TDOT", v);
        }
        if let Some(v) = &self.csrp_ndot {
            writer.write_measure("CSRP_NDOT", v);
        }
        if let Some(v) = &self.csrp_drg {
            writer.write_measure("CSRP_DRG", v);
        }
        if let Some(v) = &self.csrp_srp {
            writer.write_measure("CSRP_SRP", v);
        }

        if let Some(v) = &self.cthr_r {
            writer.write_measure("CTHR_R", v);
        }
        if let Some(v) = &self.cthr_t {
            writer.write_measure("CTHR_T", v);
        }
        if let Some(v) = &self.cthr_n {
            writer.write_measure("CTHR_N", v);
        }
        if let Some(v) = &self.cthr_rdot {
            writer.write_measure("CTHR_RDOT", v);
        }
        if let Some(v) = &self.cthr_tdot {
            writer.write_measure("CTHR_TDOT", v);
        }
        if let Some(v) = &self.cthr_ndot {
            writer.write_measure("CTHR_NDOT", v);
        }
        if let Some(v) = &self.cthr_drg {
            writer.write_measure("CTHR_DRG", v);
        }
        if let Some(v) = &self.cthr_srp {
            writer.write_measure("CTHR_SRP", v);
        }
        if let Some(v) = &self.cthr_thr {
            writer.write_measure("CTHR_THR", v);
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cdm_kvn() -> String {
        let kvn = r#"
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
    fn parse_cdm_kvn_success() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("CDM should parse");
        assert_eq!(cdm.version, "1.0");
        assert_eq!(cdm.header.originator, "TEST");
        assert_eq!(cdm.body.segments.len(), 2);
        assert!(cdm
            .body
            .relative_metadata_data
            .relative_state_vector
            .is_some());
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_frame,
            Some(ScreenVolumeFrameType::Rtn)
        );
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_shape,
            Some(ScreenVolumeShapeType::Box)
        );
    }

    #[test]
    fn header_missing_fields_error() {
        let kvn = r#"
CCSDS_CDM_VERS = 2.0
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
"#;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(msg) => assert!(msg.contains("CREATION_DATE")),
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn validate_exactly_two_segments() {
        // Build KVN with only one segment explicitly
        let kvn = r#"
    CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-ONE

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
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
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(msg) => assert!(msg.contains("exactly 2 segments")),
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn relative_state_vector_must_be_complete() {
        let kvn = r#"
    CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
// Missing RELATIVE_VELOCITY_N
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 1
CATALOG_NAME = CAT
OBJECT_NAME = O1
INTERNATIONAL_DESIGNATOR = 1998-067A
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
OBJECT = OBJECT2
OBJECT_DESIGNATOR = 2
CATALOG_NAME = CAT
OBJECT_NAME = O2
INTERNATIONAL_DESIGNATOR = 1998-067B
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
"#;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(msg) => assert!(msg.contains("RELATIVE_VELOCITY_N")),
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn covariance_missing_required_field() {
        // Remove CR_R from first segment to trigger error
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("CR_R = 1.0 [m**2]", "");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(msg) => assert!(msg.contains("CR_R")),
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn screen_frame_shape_validation() {
        // Invalid SCREEN_VOLUME_FRAME
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("SCREEN_VOLUME_FRAME = RTN", "SCREEN_VOLUME_FRAME = BAD");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(msg) => assert!(msg.contains("SCREEN_VOLUME_FRAME")),
            _ => panic!("unexpected error: {:?}", err),
        }

        // Invalid SCREEN_VOLUME_SHAPE
        let mut kvn2 = sample_cdm_kvn();
        kvn2 = kvn2.replace("SCREEN_VOLUME_SHAPE = BOX", "SCREEN_VOLUME_SHAPE = BALL");
        let err2 = Cdm::from_kvn(&kvn2).unwrap_err();
        match err2 {
            CcsdsNdmError::Validation(msg) => assert!(msg.contains("SCREEN_VOLUME_SHAPE")),
            _ => panic!("unexpected error: {:?}", err2),
        }
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
}
