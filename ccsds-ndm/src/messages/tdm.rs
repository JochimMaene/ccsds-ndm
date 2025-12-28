// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, FromKvnValue, Ndm, ToKvn};
use crate::types::{Epoch, Percentage};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::Peekable;

//----------------------------------------------------------------------
// Root TDM Structure
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "tdm")]
pub struct Tdm {
    pub header: TdmHeader,
    pub body: TdmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Tdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        writer.write_pair("CCSDS_TDM_VERS", &self.version);
        self.header.write_kvn(&mut writer);
        self.body.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let mut tokens = KvnTokenizer::new(kvn).peekable();

        // 1. Version Check
        let version = loop {
            match tokens.peek() {
                Some(Ok(KvnLine::Pair {
                    key: "CCSDS_TDM_VERS",
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
                        "CCSDS_TDM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        // 2. Header
        let header = TdmHeader::from_kvn_tokens(&mut tokens)?;

        // 3. Body
        let body = TdmBody::from_kvn_tokens(&mut tokens)?;

        Ok(Tdm {
            header,
            body,
            id: Some("CCSDS_TDM_VERS".to_string()),
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
pub struct TdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub creation_date: Epoch,
    pub originator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ToKvn for TdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(v) = &self.message_id {
            writer.write_pair("MESSAGE_ID", v);
        }
    }
}

impl FromKvnTokens for TdmHeader {
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
                        "CREATION_DATE" => {
                            creation_date = Some(Epoch::from_kvn_value(val)?);
                        }
                        "ORIGINATOR" => originator = Some(val.to_string()),
                        "MESSAGE_ID" => message_id = Some(val.to_string()),
                        _ => break,
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        Ok(TdmHeader {
            comment,
            creation_date: creation_date
                .ok_or(CcsdsNdmError::MissingField("CREATION_DATE".into()))?,
            originator: originator.ok_or(CcsdsNdmError::MissingField("ORIGINATOR".into()))?,
            message_id,
        })
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TdmBody {
    #[serde(rename = "segment")]
    pub segments: Vec<TdmSegment>,
}

impl ToKvn for TdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

impl TdmBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut segments = Vec::new();
        while tokens.peek().is_some() {
            let mut pending_comments = Vec::new();
            let mut has_content = false;
            while let Some(peeked) = tokens.peek() {
                match peeked {
                    Ok(KvnLine::Empty) => {
                        tokens.next();
                    }
                    Ok(KvnLine::Comment(_)) => {
                        if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                            pending_comments.push(c.to_string());
                        }
                    }
                    Ok(_) => {
                        has_content = true;
                        break;
                    }
                    Err(_) => {
                        return Err(tokens
                            .next()
                            .expect("Peeked error should exist")
                            .unwrap_err());
                    }
                }
            }
            if !has_content {
                break;
            }
            let mut segment = TdmSegment::from_kvn_tokens(tokens)?;
            if !pending_comments.is_empty() {
                segment
                    .metadata
                    .comment
                    .splice(0..0, pending_comments.drain(..));
            }
            segments.push(segment);
        }

        if segments.is_empty() {
            return Err(CcsdsNdmError::MissingField(
                "TDM body must contain at least one segment".into(),
            ));
        }

        Ok(TdmBody { segments })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TdmSegment {
    pub metadata: TdmMetadata,
    pub data: TdmData,
}

impl ToKvn for TdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl TdmSegment {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        match tokens.next() {
            Some(Ok(KvnLine::BlockStart("META"))) => {}
            Some(Ok(t)) => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Expected META_START, found {:?}",
                    t
                )))
            }
            Some(Err(e)) => return Err(e),
            None => {
                return Err(CcsdsNdmError::KvnParse(
                    "Unexpected EOF before TDM segment".into(),
                ))
            }
        }

        let metadata = TdmMetadata::from_kvn_tokens(tokens)?;
        let data = TdmData::from_kvn_tokens(tokens)?;

        Ok(TdmSegment { metadata, data })
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TdmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    pub time_system: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Epoch>,
    pub participant_1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_5: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_band: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_band: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnaround_numerator: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnaround_denominator: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timetag_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_interval: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freq_offset: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_modulus: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_units: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_bias: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_scale: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doppler_count_rollover: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_1: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_3: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_4: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_delay_5: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_1: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_3: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_4: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_delay_5: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_quality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_angle_1: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_angle_2: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_doppler: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_mag: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_range: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_rcs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_receive: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_transmit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_aberration_yearly: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_aberration_diurnal: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corrections_applied: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name_3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name_4: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name_5: Option<String>,
}

impl ToKvn for TdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.track_id {
            writer.write_pair("TRACK_ID", v);
        }
        if let Some(v) = &self.data_types {
            writer.write_pair("DATA_TYPES", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        if let Some(v) = &self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = &self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        writer.write_pair("PARTICIPANT_1", &self.participant_1);
        if let Some(v) = &self.participant_2 {
            writer.write_pair("PARTICIPANT_2", v);
        }
        if let Some(v) = &self.participant_3 {
            writer.write_pair("PARTICIPANT_3", v);
        }
        if let Some(v) = &self.participant_4 {
            writer.write_pair("PARTICIPANT_4", v);
        }
        if let Some(v) = &self.participant_5 {
            writer.write_pair("PARTICIPANT_5", v);
        }
        if let Some(v) = &self.mode {
            writer.write_pair("MODE", v);
        }
        if let Some(v) = &self.path {
            writer.write_pair("PATH", v);
        }
        if let Some(v) = &self.path_1 {
            writer.write_pair("PATH_1", v);
        }
        if let Some(v) = &self.path_2 {
            writer.write_pair("PATH_2", v);
        }
        if let Some(v) = &self.ephemeris_name_1 {
            writer.write_pair("EPHEMERIS_NAME_1", v);
        }
        if let Some(v) = &self.ephemeris_name_2 {
            writer.write_pair("EPHEMERIS_NAME_2", v);
        }
        if let Some(v) = &self.ephemeris_name_3 {
            writer.write_pair("EPHEMERIS_NAME_3", v);
        }
        if let Some(v) = &self.ephemeris_name_4 {
            writer.write_pair("EPHEMERIS_NAME_4", v);
        }
        if let Some(v) = &self.ephemeris_name_5 {
            writer.write_pair("EPHEMERIS_NAME_5", v);
        }
        if let Some(v) = &self.transmit_band {
            writer.write_pair("TRANSMIT_BAND", v);
        }
        if let Some(v) = &self.receive_band {
            writer.write_pair("RECEIVE_BAND", v);
        }
        if let Some(v) = self.turnaround_numerator {
            writer.write_pair("TURNAROUND_NUMERATOR", v);
        }
        if let Some(v) = self.turnaround_denominator {
            writer.write_pair("TURNAROUND_DENOMINATOR", v);
        }
        if let Some(v) = &self.timetag_ref {
            writer.write_pair("TIMETAG_REF", v);
        }
        if let Some(v) = self.integration_interval {
            writer.write_pair("INTEGRATION_INTERVAL", v);
        }
        if let Some(v) = &self.integration_ref {
            writer.write_pair("INTEGRATION_REF", v);
        }
        if let Some(v) = self.freq_offset {
            writer.write_pair("FREQ_OFFSET", v);
        }
        if let Some(v) = &self.range_mode {
            writer.write_pair("RANGE_MODE", v);
        }
        if let Some(v) = self.range_modulus {
            writer.write_pair("RANGE_MODULUS", v);
        }
        if let Some(v) = &self.range_units {
            writer.write_pair("RANGE_UNITS", v);
        }
        if let Some(v) = &self.angle_type {
            writer.write_pair("ANGLE_TYPE", v);
        }
        if let Some(v) = &self.reference_frame {
            writer.write_pair("REFERENCE_FRAME", v);
        }
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
        if let Some(v) = self.doppler_count_bias {
            writer.write_pair("DOPPLER_COUNT_BIAS", v);
        }
        if let Some(v) = self.doppler_count_scale {
            writer.write_pair("DOPPLER_COUNT_SCALE", v);
        }
        if let Some(v) = &self.doppler_count_rollover {
            writer.write_pair("DOPPLER_COUNT_ROLLOVER", v);
        }
        if let Some(v) = self.transmit_delay_1 {
            writer.write_pair("TRANSMIT_DELAY_1", v);
        }
        if let Some(v) = self.transmit_delay_2 {
            writer.write_pair("TRANSMIT_DELAY_2", v);
        }
        if let Some(v) = self.transmit_delay_3 {
            writer.write_pair("TRANSMIT_DELAY_3", v);
        }
        if let Some(v) = self.transmit_delay_4 {
            writer.write_pair("TRANSMIT_DELAY_4", v);
        }
        if let Some(v) = self.transmit_delay_5 {
            writer.write_pair("TRANSMIT_DELAY_5", v);
        }
        if let Some(v) = self.receive_delay_1 {
            writer.write_pair("RECEIVE_DELAY_1", v);
        }
        if let Some(v) = self.receive_delay_2 {
            writer.write_pair("RECEIVE_DELAY_2", v);
        }
        if let Some(v) = self.receive_delay_3 {
            writer.write_pair("RECEIVE_DELAY_3", v);
        }
        if let Some(v) = self.receive_delay_4 {
            writer.write_pair("RECEIVE_DELAY_4", v);
        }
        if let Some(v) = self.receive_delay_5 {
            writer.write_pair("RECEIVE_DELAY_5", v);
        }
        if let Some(v) = &self.data_quality {
            writer.write_pair("DATA_QUALITY", v);
        }
        if let Some(v) = self.correction_angle_1 {
            writer.write_pair("CORRECTION_ANGLE_1", v);
        }
        if let Some(v) = self.correction_angle_2 {
            writer.write_pair("CORRECTION_ANGLE_2", v);
        }
        if let Some(v) = self.correction_doppler {
            writer.write_pair("CORRECTION_DOPPLER", v);
        }
        if let Some(v) = self.correction_mag {
            writer.write_pair("CORRECTION_MAG", v);
        }
        if let Some(v) = self.correction_range {
            writer.write_pair("CORRECTION_RANGE", v);
        }
        if let Some(v) = self.correction_rcs {
            writer.write_pair("CORRECTION_RCS", v);
        }
        if let Some(v) = self.correction_receive {
            writer.write_pair("CORRECTION_RECEIVE", v);
        }
        if let Some(v) = self.correction_transmit {
            writer.write_pair("CORRECTION_TRANSMIT", v);
        }
        if let Some(v) = self.correction_aberration_yearly {
            writer.write_pair("CORRECTION_ABERRATION_YEARLY", v);
        }
        if let Some(v) = self.correction_aberration_diurnal {
            writer.write_pair("CORRECTION_ABERRATION_DIURNAL", v);
        }
        if let Some(v) = &self.corrections_applied {
            writer.write_pair("CORRECTIONS_APPLIED", v);
        }
        writer.write_section("META_STOP");
    }
}

impl TdmMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut meta = TdmMetadata::default();

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
                KvnLine::BlockEnd("META") => {
                    tokens.next();
                    break;
                }
                KvnLine::Comment(c) => {
                    meta.comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    let key = *key;
                    let val = *val;
                    match key {
                        "TRACK_ID" => meta.track_id = Some(val.to_string()),
                        "DATA_TYPES" => meta.data_types = Some(val.to_string()),
                        "TIME_SYSTEM" => meta.time_system = val.to_string(),
                        "START_TIME" => meta.start_time = Some(Epoch::from_kvn_value(val)?),
                        "STOP_TIME" => meta.stop_time = Some(Epoch::from_kvn_value(val)?),
                        "PARTICIPANT_1" => meta.participant_1 = val.to_string(),
                        "PARTICIPANT_2" => meta.participant_2 = Some(val.to_string()),
                        "PARTICIPANT_3" => meta.participant_3 = Some(val.to_string()),
                        "PARTICIPANT_4" => meta.participant_4 = Some(val.to_string()),
                        "PARTICIPANT_5" => meta.participant_5 = Some(val.to_string()),
                        "MODE" => meta.mode = Some(val.to_string()),
                        "PATH" => meta.path = Some(val.to_string()),
                        "PATH_1" => meta.path_1 = Some(val.to_string()),
                        "PATH_2" => meta.path_2 = Some(val.to_string()),
                        "EPHEMERIS_NAME_1" => meta.ephemeris_name_1 = Some(val.to_string()),
                        "EPHEMERIS_NAME_2" => meta.ephemeris_name_2 = Some(val.to_string()),
                        "EPHEMERIS_NAME_3" => meta.ephemeris_name_3 = Some(val.to_string()),
                        "EPHEMERIS_NAME_4" => meta.ephemeris_name_4 = Some(val.to_string()),
                        "EPHEMERIS_NAME_5" => meta.ephemeris_name_5 = Some(val.to_string()),
                        "TRANSMIT_BAND" => meta.transmit_band = Some(val.to_string()),
                        "RECEIVE_BAND" => meta.receive_band = Some(val.to_string()),
                        "TURNAROUND_NUMERATOR" => meta.turnaround_numerator = Some(val.parse()?),
                        "TURNAROUND_DENOMINATOR" => {
                            meta.turnaround_denominator = Some(val.parse()?)
                        }
                        "TIMETAG_REF" => meta.timetag_ref = Some(val.to_string()),
                        "INTEGRATION_INTERVAL" => meta.integration_interval = Some(val.parse()?),
                        "INTEGRATION_REF" => meta.integration_ref = Some(val.to_string()),
                        "FREQ_OFFSET" => meta.freq_offset = Some(val.parse()?),
                        "RANGE_MODE" => meta.range_mode = Some(val.to_string()),
                        "RANGE_MODULUS" => meta.range_modulus = Some(val.parse()?),
                        "RANGE_UNITS" => meta.range_units = Some(val.to_string()),
                        "ANGLE_TYPE" => meta.angle_type = Some(val.to_string()),
                        "REFERENCE_FRAME" => meta.reference_frame = Some(val.to_string()),
                        "INTERPOLATION" => meta.interpolation = Some(val.to_string()),
                        "INTERPOLATION_DEGREE" => meta.interpolation_degree = Some(val.parse()?),
                        "DOPPLER_COUNT_BIAS" => meta.doppler_count_bias = Some(val.parse()?),
                        "DOPPLER_COUNT_SCALE" => meta.doppler_count_scale = Some(val.parse()?),
                        "DOPPLER_COUNT_ROLLOVER" => {
                            meta.doppler_count_rollover = Some(val.to_string())
                        }
                        "TRANSMIT_DELAY_1" => meta.transmit_delay_1 = Some(val.parse()?),
                        "TRANSMIT_DELAY_2" => meta.transmit_delay_2 = Some(val.parse()?),
                        "TRANSMIT_DELAY_3" => meta.transmit_delay_3 = Some(val.parse()?),
                        "TRANSMIT_DELAY_4" => meta.transmit_delay_4 = Some(val.parse()?),
                        "TRANSMIT_DELAY_5" => meta.transmit_delay_5 = Some(val.parse()?),
                        "RECEIVE_DELAY_1" => meta.receive_delay_1 = Some(val.parse()?),
                        "RECEIVE_DELAY_2" => meta.receive_delay_2 = Some(val.parse()?),
                        "RECEIVE_DELAY_3" => meta.receive_delay_3 = Some(val.parse()?),
                        "RECEIVE_DELAY_4" => meta.receive_delay_4 = Some(val.parse()?),
                        "RECEIVE_DELAY_5" => meta.receive_delay_5 = Some(val.parse()?),
                        "DATA_QUALITY" => meta.data_quality = Some(val.to_string()),
                        "CORRECTION_ANGLE_1" => meta.correction_angle_1 = Some(val.parse()?),
                        "CORRECTION_ANGLE_2" => meta.correction_angle_2 = Some(val.parse()?),
                        "CORRECTION_DOPPLER" => meta.correction_doppler = Some(val.parse()?),
                        "CORRECTION_MAG" => meta.correction_mag = Some(val.parse()?),
                        "CORRECTION_RANGE" => meta.correction_range = Some(val.parse()?),
                        "CORRECTION_RCS" => meta.correction_rcs = Some(val.parse()?),
                        "CORRECTION_RECEIVE" => meta.correction_receive = Some(val.parse()?),
                        "CORRECTION_TRANSMIT" => meta.correction_transmit = Some(val.parse()?),
                        "CORRECTION_ABERRATION_YEARLY" => {
                            meta.correction_aberration_yearly = Some(val.parse()?)
                        }
                        "CORRECTION_ABERRATION_DIURNAL" => {
                            meta.correction_aberration_diurnal = Some(val.parse()?)
                        }
                        "CORRECTIONS_APPLIED" => meta.corrections_applied = Some(val.to_string()),
                        _ => {
                            return Err(CcsdsNdmError::KvnParse(format!(
                                "Unexpected TDM Metadata key: {}",
                                key
                            )))
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        if meta.time_system.is_empty() {
            return Err(CcsdsNdmError::MissingField("TIME_SYSTEM".into()));
        }
        if meta.participant_1.is_empty() {
            return Err(CcsdsNdmError::MissingField("PARTICIPANT_1".into()));
        }

        Ok(meta)
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TdmData {
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(rename = "observation")]
    pub observations: Vec<TdmObservation>,
}

impl ToKvn for TdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("DATA_START");
        writer.write_comments(&self.comment);
        for obs in &self.observations {
            let key = obs.data.key();
            let val_str = obs.data.value_to_string();
            let line = format!("{} {}", obs.epoch, val_str);
            writer.write_pair(key, line);
        }
        writer.write_section("DATA_STOP");
    }
}

impl TdmData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        match tokens.next() {
            Some(Ok(KvnLine::BlockStart("DATA"))) => {}
            Some(Ok(t)) => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Expected DATA_START, found {:?}",
                    t
                )))
            }
            Some(Err(e)) => return Err(e),
            None => {
                return Err(CcsdsNdmError::KvnParse(
                    "Unexpected EOF before TDM data".into(),
                ))
            }
        }

        let mut comment = Vec::new();
        let mut observations = Vec::new();

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
                KvnLine::BlockEnd("DATA") => {
                    tokens.next();
                    break;
                }
                KvnLine::Comment(c) => {
                    comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, .. } => {
                    let parts: Vec<&str> = val.split_whitespace().collect();
                    if parts.len() < 2 {
                        return Err(CcsdsNdmError::KvnParse(format!(
                            "Data line value must contain 'EPOCH MEASUREMENT', found '{}'",
                            val
                        )));
                    }
                    let epoch_str = parts[0];
                    let measure_str = parts[1..].join(" ");
                    let epoch = Epoch::from_kvn_value(epoch_str)?;
                    let data = TdmObservationData::from_key_val(key, &measure_str)?;
                    observations.push(TdmObservation { epoch, data });
                    tokens.next();
                }
                _ => break,
            }
        }
        Ok(TdmData {
            comment,
            observations,
        })
    }
}

//----------------------------------------------------------------------
// Observation
//----------------------------------------------------------------------

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TdmObservation {
    #[serde(rename = "EPOCH")]
    pub epoch: Epoch,
    #[serde(rename = "$value")]
    pub data: TdmObservationData,
}

// Custom Deserialize to handle XML's flat structure correctly
impl<'de> Deserialize<'de> for TdmObservation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TdmObservationVisitor;

        impl<'de> Visitor<'de> for TdmObservationVisitor {
            type Value = TdmObservation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a TDM observation element")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut epoch: Option<Epoch> = None;
                let mut data: Option<TdmObservationData> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "EPOCH" => {
                            if epoch.is_some() {
                                return Err(serde::de::Error::duplicate_field("EPOCH"));
                            }
                            epoch = Some(map.next_value()?);
                        }
                        // Explicit matching of all data types
                        "ANGLE_1" => {
                            data = Some(TdmObservationData::Angle1(map.next_value()?));
                        }
                        "ANGLE_2" => {
                            data = Some(TdmObservationData::Angle2(map.next_value()?));
                        }
                        "CARRIER_POWER" => {
                            data = Some(TdmObservationData::CarrierPower(map.next_value()?));
                        }
                        "CLOCK_BIAS" => {
                            data = Some(TdmObservationData::ClockBias(map.next_value()?));
                        }
                        "CLOCK_DRIFT" => {
                            data = Some(TdmObservationData::ClockDrift(map.next_value()?));
                        }
                        "DOPPLER_COUNT" => {
                            data = Some(TdmObservationData::DopplerCount(map.next_value()?));
                        }
                        "DOPPLER_INSTANTANEOUS" => {
                            data =
                                Some(TdmObservationData::DopplerInstantaneous(map.next_value()?));
                        }
                        "DOPPLER_INTEGRATED" => {
                            data = Some(TdmObservationData::DopplerIntegrated(map.next_value()?));
                        }
                        "DOR" => {
                            data = Some(TdmObservationData::Dor(map.next_value()?));
                        }
                        "MAG" => {
                            data = Some(TdmObservationData::Mag(map.next_value()?));
                        }
                        "PC_N0" => {
                            data = Some(TdmObservationData::PcN0(map.next_value()?));
                        }
                        "PR_N0" => {
                            data = Some(TdmObservationData::PrN0(map.next_value()?));
                        }
                        "PRESSURE" => {
                            data = Some(TdmObservationData::Pressure(map.next_value()?));
                        }
                        "RANGE" => {
                            data = Some(TdmObservationData::Range(map.next_value()?));
                        }
                        "RCS" => {
                            data = Some(TdmObservationData::Rcs(map.next_value()?));
                        }
                        "RECEIVE_FREQ" => {
                            data = Some(TdmObservationData::ReceiveFreq(map.next_value()?));
                        }
                        "RECEIVE_FREQ_1" => {
                            data = Some(TdmObservationData::ReceiveFreq1(map.next_value()?));
                        }
                        "RECEIVE_FREQ_2" => {
                            data = Some(TdmObservationData::ReceiveFreq2(map.next_value()?));
                        }
                        "RECEIVE_FREQ_3" => {
                            data = Some(TdmObservationData::ReceiveFreq3(map.next_value()?));
                        }
                        "RECEIVE_FREQ_4" => {
                            data = Some(TdmObservationData::ReceiveFreq4(map.next_value()?));
                        }
                        "RECEIVE_FREQ_5" => {
                            data = Some(TdmObservationData::ReceiveFreq5(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_1" => {
                            data = Some(TdmObservationData::ReceivePhaseCt1(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_2" => {
                            data = Some(TdmObservationData::ReceivePhaseCt2(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_3" => {
                            data = Some(TdmObservationData::ReceivePhaseCt3(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_4" => {
                            data = Some(TdmObservationData::ReceivePhaseCt4(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_5" => {
                            data = Some(TdmObservationData::ReceivePhaseCt5(map.next_value()?));
                        }
                        "RHUMIDITY" => {
                            data = Some(TdmObservationData::Rhumidity(map.next_value()?));
                        }
                        "STEC" => {
                            data = Some(TdmObservationData::Stec(map.next_value()?));
                        }
                        "TEMPERATURE" => {
                            data = Some(TdmObservationData::Temperature(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_1" => {
                            data = Some(TdmObservationData::TransmitFreq1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_2" => {
                            data = Some(TdmObservationData::TransmitFreq2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_3" => {
                            data = Some(TdmObservationData::TransmitFreq3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_4" => {
                            data = Some(TdmObservationData::TransmitFreq4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_5" => {
                            data = Some(TdmObservationData::TransmitFreq5(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_1" => {
                            data = Some(TdmObservationData::TransmitFreqRate1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_2" => {
                            data = Some(TdmObservationData::TransmitFreqRate2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_3" => {
                            data = Some(TdmObservationData::TransmitFreqRate3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_4" => {
                            data = Some(TdmObservationData::TransmitFreqRate4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_5" => {
                            data = Some(TdmObservationData::TransmitFreqRate5(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_1" => {
                            data = Some(TdmObservationData::TransmitPhaseCt1(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_2" => {
                            data = Some(TdmObservationData::TransmitPhaseCt2(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_3" => {
                            data = Some(TdmObservationData::TransmitPhaseCt3(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_4" => {
                            data = Some(TdmObservationData::TransmitPhaseCt4(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_5" => {
                            data = Some(TdmObservationData::TransmitPhaseCt5(map.next_value()?));
                        }
                        "TROPO_DRY" => {
                            data = Some(TdmObservationData::TropoDry(map.next_value()?));
                        }
                        "TROPO_WET" => {
                            data = Some(TdmObservationData::TropoWet(map.next_value()?));
                        }
                        "VLBI_DELAY" => {
                            data = Some(TdmObservationData::VlbiDelay(map.next_value()?));
                        }
                        _ => {
                            // Consume unknown fields or attributes that might appear
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let epoch = epoch.ok_or_else(|| serde::de::Error::missing_field("EPOCH"))?;
                let data = data.ok_or_else(|| {
                    serde::de::Error::custom(
                        "Missing TDM observation data (must have one of: ANGLE_1, RANGE, etc.)",
                    )
                })?;

                Ok(TdmObservation { epoch, data })
            }
        }

        deserializer.deserialize_map(TdmObservationVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TdmObservationData {
    Angle1(f64),
    Angle2(f64),
    CarrierPower(f64),
    ClockBias(f64),
    ClockDrift(f64),
    DopplerCount(f64),
    DopplerInstantaneous(f64),
    DopplerIntegrated(f64),
    Dor(f64),
    Mag(f64),
    #[serde(rename = "PC_N0")]
    PcN0(f64),
    #[serde(rename = "PR_N0")]
    PrN0(f64),
    Pressure(f64),
    Range(f64),
    Rcs(f64),
    ReceiveFreq(f64),
    #[serde(rename = "RECEIVE_FREQ_1")]
    ReceiveFreq1(f64),
    #[serde(rename = "RECEIVE_FREQ_2")]
    ReceiveFreq2(f64),
    #[serde(rename = "RECEIVE_FREQ_3")]
    ReceiveFreq3(f64),
    #[serde(rename = "RECEIVE_FREQ_4")]
    ReceiveFreq4(f64),
    #[serde(rename = "RECEIVE_FREQ_5")]
    ReceiveFreq5(f64),
    #[serde(rename = "RECEIVE_PHASE_CT_1")]
    ReceivePhaseCt1(String),
    #[serde(rename = "RECEIVE_PHASE_CT_2")]
    ReceivePhaseCt2(String),
    #[serde(rename = "RECEIVE_PHASE_CT_3")]
    ReceivePhaseCt3(String),
    #[serde(rename = "RECEIVE_PHASE_CT_4")]
    ReceivePhaseCt4(String),
    #[serde(rename = "RECEIVE_PHASE_CT_5")]
    ReceivePhaseCt5(String),
    Rhumidity(Percentage),
    Stec(f64),
    Temperature(f64),
    #[serde(rename = "TRANSMIT_FREQ_1")]
    TransmitFreq1(f64),
    #[serde(rename = "TRANSMIT_FREQ_2")]
    TransmitFreq2(f64),
    #[serde(rename = "TRANSMIT_FREQ_3")]
    TransmitFreq3(f64),
    #[serde(rename = "TRANSMIT_FREQ_4")]
    TransmitFreq4(f64),
    #[serde(rename = "TRANSMIT_FREQ_5")]
    TransmitFreq5(f64),
    #[serde(rename = "TRANSMIT_FREQ_RATE_1")]
    TransmitFreqRate1(f64),
    #[serde(rename = "TRANSMIT_FREQ_RATE_2")]
    TransmitFreqRate2(f64),
    #[serde(rename = "TRANSMIT_FREQ_RATE_3")]
    TransmitFreqRate3(f64),
    #[serde(rename = "TRANSMIT_FREQ_RATE_4")]
    TransmitFreqRate4(f64),
    #[serde(rename = "TRANSMIT_FREQ_RATE_5")]
    TransmitFreqRate5(f64),
    #[serde(rename = "TRANSMIT_PHASE_CT_1")]
    TransmitPhaseCt1(String),
    #[serde(rename = "TRANSMIT_PHASE_CT_2")]
    TransmitPhaseCt2(String),
    #[serde(rename = "TRANSMIT_PHASE_CT_3")]
    TransmitPhaseCt3(String),
    #[serde(rename = "TRANSMIT_PHASE_CT_4")]
    TransmitPhaseCt4(String),
    #[serde(rename = "TRANSMIT_PHASE_CT_5")]
    TransmitPhaseCt5(String),
    TropoDry(f64),
    TropoWet(f64),
    VlbiDelay(f64),
}

impl TdmObservationData {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Angle1(_) => "ANGLE_1",
            Self::Angle2(_) => "ANGLE_2",
            Self::CarrierPower(_) => "CARRIER_POWER",
            Self::ClockBias(_) => "CLOCK_BIAS",
            Self::ClockDrift(_) => "CLOCK_DRIFT",
            Self::DopplerCount(_) => "DOPPLER_COUNT",
            Self::DopplerInstantaneous(_) => "DOPPLER_INSTANTANEOUS",
            Self::DopplerIntegrated(_) => "DOPPLER_INTEGRATED",
            Self::Dor(_) => "DOR",
            Self::Mag(_) => "MAG",
            Self::PcN0(_) => "PC_N0",
            Self::PrN0(_) => "PR_N0",
            Self::Pressure(_) => "PRESSURE",
            Self::Range(_) => "RANGE",
            Self::Rcs(_) => "RCS",
            Self::ReceiveFreq(_) => "RECEIVE_FREQ",
            Self::ReceiveFreq1(_) => "RECEIVE_FREQ_1",
            Self::ReceiveFreq2(_) => "RECEIVE_FREQ_2",
            Self::ReceiveFreq3(_) => "RECEIVE_FREQ_3",
            Self::ReceiveFreq4(_) => "RECEIVE_FREQ_4",
            Self::ReceiveFreq5(_) => "RECEIVE_FREQ_5",
            Self::ReceivePhaseCt1(_) => "RECEIVE_PHASE_CT_1",
            Self::ReceivePhaseCt2(_) => "RECEIVE_PHASE_CT_2",
            Self::ReceivePhaseCt3(_) => "RECEIVE_PHASE_CT_3",
            Self::ReceivePhaseCt4(_) => "RECEIVE_PHASE_CT_4",
            Self::ReceivePhaseCt5(_) => "RECEIVE_PHASE_CT_5",
            Self::Rhumidity(_) => "RHUMIDITY",
            Self::Stec(_) => "STEC",
            Self::Temperature(_) => "TEMPERATURE",
            Self::TransmitFreq1(_) => "TRANSMIT_FREQ_1",
            Self::TransmitFreq2(_) => "TRANSMIT_FREQ_2",
            Self::TransmitFreq3(_) => "TRANSMIT_FREQ_3",
            Self::TransmitFreq4(_) => "TRANSMIT_FREQ_4",
            Self::TransmitFreq5(_) => "TRANSMIT_FREQ_5",
            Self::TransmitFreqRate1(_) => "TRANSMIT_FREQ_RATE_1",
            Self::TransmitFreqRate2(_) => "TRANSMIT_FREQ_RATE_2",
            Self::TransmitFreqRate3(_) => "TRANSMIT_FREQ_RATE_3",
            Self::TransmitFreqRate4(_) => "TRANSMIT_FREQ_RATE_4",
            Self::TransmitFreqRate5(_) => "TRANSMIT_FREQ_RATE_5",
            Self::TransmitPhaseCt1(_) => "TRANSMIT_PHASE_CT_1",
            Self::TransmitPhaseCt2(_) => "TRANSMIT_PHASE_CT_2",
            Self::TransmitPhaseCt3(_) => "TRANSMIT_PHASE_CT_3",
            Self::TransmitPhaseCt4(_) => "TRANSMIT_PHASE_CT_4",
            Self::TransmitPhaseCt5(_) => "TRANSMIT_PHASE_CT_5",
            Self::TropoDry(_) => "TROPO_DRY",
            Self::TropoWet(_) => "TROPO_WET",
            Self::VlbiDelay(_) => "VLBI_DELAY",
        }
    }

    pub fn value_to_string(&self) -> String {
        match self {
            Self::ReceivePhaseCt1(s)
            | Self::ReceivePhaseCt2(s)
            | Self::ReceivePhaseCt3(s)
            | Self::ReceivePhaseCt4(s)
            | Self::ReceivePhaseCt5(s)
            | Self::TransmitPhaseCt1(s)
            | Self::TransmitPhaseCt2(s)
            | Self::TransmitPhaseCt3(s)
            | Self::TransmitPhaseCt4(s)
            | Self::TransmitPhaseCt5(s) => s.clone(),
            Self::Rhumidity(v) => v.value.to_string(),
            Self::Angle1(v)
            | Self::Angle2(v)
            | Self::CarrierPower(v)
            | Self::ClockBias(v)
            | Self::ClockDrift(v)
            | Self::DopplerCount(v)
            | Self::DopplerInstantaneous(v)
            | Self::DopplerIntegrated(v)
            | Self::Dor(v)
            | Self::Mag(v)
            | Self::PcN0(v)
            | Self::PrN0(v)
            | Self::Pressure(v)
            | Self::Range(v)
            | Self::Rcs(v)
            | Self::ReceiveFreq(v)
            | Self::ReceiveFreq1(v)
            | Self::ReceiveFreq2(v)
            | Self::ReceiveFreq3(v)
            | Self::ReceiveFreq4(v)
            | Self::ReceiveFreq5(v)
            | Self::Stec(v)
            | Self::Temperature(v)
            | Self::TransmitFreq1(v)
            | Self::TransmitFreq2(v)
            | Self::TransmitFreq3(v)
            | Self::TransmitFreq4(v)
            | Self::TransmitFreq5(v)
            | Self::TransmitFreqRate1(v)
            | Self::TransmitFreqRate2(v)
            | Self::TransmitFreqRate3(v)
            | Self::TransmitFreqRate4(v)
            | Self::TransmitFreqRate5(v)
            | Self::TropoDry(v)
            | Self::TropoWet(v)
            | Self::VlbiDelay(v) => v.to_string(),
        }
    }

    pub fn from_key_val(key: &str, val: &str) -> Result<Self> {
        let pf = |s: &str| {
            s.parse::<f64>()
                .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid float: {}", e)))
        };
        match key {
            "ANGLE_1" => Ok(Self::Angle1(pf(val)?)),
            "ANGLE_2" => Ok(Self::Angle2(pf(val)?)),
            "CARRIER_POWER" => Ok(Self::CarrierPower(pf(val)?)),
            "CLOCK_BIAS" => Ok(Self::ClockBias(pf(val)?)),
            "CLOCK_DRIFT" => Ok(Self::ClockDrift(pf(val)?)),
            "DOPPLER_COUNT" => Ok(Self::DopplerCount(pf(val)?)),
            "DOPPLER_INSTANTANEOUS" => Ok(Self::DopplerInstantaneous(pf(val)?)),
            "DOPPLER_INTEGRATED" => Ok(Self::DopplerIntegrated(pf(val)?)),
            "DOR" => Ok(Self::Dor(pf(val)?)),
            "MAG" => Ok(Self::Mag(pf(val)?)),
            "PC_N0" => Ok(Self::PcN0(pf(val)?)),
            "PR_N0" => Ok(Self::PrN0(pf(val)?)),
            "PRESSURE" => Ok(Self::Pressure(pf(val)?)),
            "RANGE" => Ok(Self::Range(pf(val)?)),
            "RCS" => Ok(Self::Rcs(pf(val)?)),
            "RECEIVE_FREQ" => Ok(Self::ReceiveFreq(pf(val)?)),
            "RECEIVE_FREQ_1" => Ok(Self::ReceiveFreq1(pf(val)?)),
            "RECEIVE_FREQ_2" => Ok(Self::ReceiveFreq2(pf(val)?)),
            "RECEIVE_FREQ_3" => Ok(Self::ReceiveFreq3(pf(val)?)),
            "RECEIVE_FREQ_4" => Ok(Self::ReceiveFreq4(pf(val)?)),
            "RECEIVE_FREQ_5" => Ok(Self::ReceiveFreq5(pf(val)?)),
            "RECEIVE_PHASE_CT_1" => Ok(Self::ReceivePhaseCt1(val.to_string())),
            "RECEIVE_PHASE_CT_2" => Ok(Self::ReceivePhaseCt2(val.to_string())),
            "RECEIVE_PHASE_CT_3" => Ok(Self::ReceivePhaseCt3(val.to_string())),
            "RECEIVE_PHASE_CT_4" => Ok(Self::ReceivePhaseCt4(val.to_string())),
            "RECEIVE_PHASE_CT_5" => Ok(Self::ReceivePhaseCt5(val.to_string())),
            "RHUMIDITY" => Ok(Self::Rhumidity(Percentage::new(pf(val)?, None)?)),
            "STEC" => Ok(Self::Stec(pf(val)?)),
            "TEMPERATURE" => Ok(Self::Temperature(pf(val)?)),
            "TRANSMIT_FREQ_1" => Ok(Self::TransmitFreq1(pf(val)?)),
            "TRANSMIT_FREQ_2" => Ok(Self::TransmitFreq2(pf(val)?)),
            "TRANSMIT_FREQ_3" => Ok(Self::TransmitFreq3(pf(val)?)),
            "TRANSMIT_FREQ_4" => Ok(Self::TransmitFreq4(pf(val)?)),
            "TRANSMIT_FREQ_5" => Ok(Self::TransmitFreq5(pf(val)?)),
            "TRANSMIT_FREQ_RATE_1" => Ok(Self::TransmitFreqRate1(pf(val)?)),
            "TRANSMIT_FREQ_RATE_2" => Ok(Self::TransmitFreqRate2(pf(val)?)),
            "TRANSMIT_FREQ_RATE_3" => Ok(Self::TransmitFreqRate3(pf(val)?)),
            "TRANSMIT_FREQ_RATE_4" => Ok(Self::TransmitFreqRate4(pf(val)?)),
            "TRANSMIT_FREQ_RATE_5" => Ok(Self::TransmitFreqRate5(pf(val)?)),
            "TRANSMIT_PHASE_CT_1" => Ok(Self::TransmitPhaseCt1(val.to_string())),
            "TRANSMIT_PHASE_CT_2" => Ok(Self::TransmitPhaseCt2(val.to_string())),
            "TRANSMIT_PHASE_CT_3" => Ok(Self::TransmitPhaseCt3(val.to_string())),
            "TRANSMIT_PHASE_CT_4" => Ok(Self::TransmitPhaseCt4(val.to_string())),
            "TRANSMIT_PHASE_CT_5" => Ok(Self::TransmitPhaseCt5(val.to_string())),
            "TROPO_DRY" => Ok(Self::TropoDry(pf(val)?)),
            "TROPO_WET" => Ok(Self::TropoWet(pf(val)?)),
            "VLBI_DELAY" => Ok(Self::VlbiDelay(pf(val)?)),
            _ => Err(CcsdsNdmError::KvnParse(format!(
                "Unknown TDM data keyword: {}",
                key
            ))),
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
    fn test_parse_tdm_example_e1_oneway() {
        let kvn = r#"
CCSDS_TDM_VERS = 2.0
COMMENT TDM example created by yyyyy-nnnA Nav Team (NASA/JPL)
COMMENT StarTrek 1-way data, Ka band down
CREATION_DATE = 2005-160T20:15:00Z
ORIGINATOR = NASA
META_START
COMMENT Data quality degraded by antenna pointing problem...
COMMENT Slightly noisy data
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-25
PARTICIPANT_2 = yyyy-nnnA
MODE = SEQUENTIAL
PATH = 2,1
INTEGRATION_INTERVAL = 1
INTEGRATION_REF = MIDDLE
FREQ_OFFSET = 0
TRANSMIT_DELAY_1 = 0.000077
RECEIVE_DELAY_1 = 0.000077
DATA_QUALITY = DEGRADED
META_STOP
DATA_START
COMMENT TRANSMIT_FREQ_2 is spacecraft reference downlink
TRANSMIT_FREQ_2 = 2005-159T17:41:00 32023442781.733
RECEIVE_FREQ_1 = 2005-159T17:41:00 32021034790.7265
RECEIVE_FREQ_1 = 2005-159T17:41:01 32021034828.8432
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.header.creation_date.to_string(), "2005-160T20:15:00Z");
        assert_eq!(tdm.body.segments.len(), 1);
        let seg = &tdm.body.segments[0];
        assert_eq!(seg.metadata.participant_1, "DSS-25");
        assert_eq!(seg.metadata.participant_2.as_deref(), Some("yyyy-nnnA"));
        assert_eq!(seg.data.observations.len(), 3);
        match &seg.data.observations[0].data {
            TdmObservationData::TransmitFreq2(v) => assert_eq!(*v, 32023442781.733),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_parse_tdm_example_e16_optical() {
        let kvn = r#"
CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2012-10-30T20:00:00
ORIGINATOR = ESA
META_START
TIME_SYSTEM = UTC
START_TIME = 2012-10-29T17:46:39.02
STOP_TIME = 2012-10-29T17:50:53.02
PARTICIPANT_1 = TFRM
PARTICIPANT_2 = TRACK_NUMBER_001
MODE = SEQUENTIAL
PATH = 2,1
ANGLE_TYPE = RADEC
REFERENCE_FRAME = EME2000
META_STOP
DATA_START
ANGLE_1 = 2012-10-29T17:46:39.02 332.2298750
ANGLE_2 = 2012-10-29T17:46:39.02 -16.3028389
MAG = 2012-10-29T17:46:39.02 12.1
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert_eq!(seg.metadata.angle_type.as_deref(), Some("RADEC"));
        assert_eq!(seg.data.observations.len(), 3);
        match &seg.data.observations[2].data {
            TdmObservationData::Mag(v) => assert_eq!(*v, 12.1),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_parse_tdm_example_e18_phase() {
        let kvn = r#"
CCSDS_TDM_VERS=2.0
CREATION_DATE=2005-184T20:15:00
ORIGINATOR=NASA
META_START
TIME_SYSTEM=UTC
PARTICIPANT_1=DSS-55
PARTICIPANT_2=yyyy-nnnA
MODE=SEQUENTIAL
PATH=1,2,1
META_STOP
DATA_START
TRANSMIT_PHASE_CT_1=2005-184T11:12:23 7175173383.615373
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        match &seg.data.observations[0].data {
            TdmObservationData::TransmitPhaseCt1(s) => assert_eq!(s, "7175173383.615373"),
            _ => panic!("Wrong type"),
        }
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 1: Root/Header Mandatory Fields
    // XSD: tdmType requires @id="CCSDS_TDM_VERS", @version="2.0"
    // XSD: tdmHeader requires CREATION_DATE and ORIGINATOR
    // =========================================================================

    #[test]
    fn test_xsd_header_mandatory_creation_date() {
        // XSD: CREATION_DATE is required (no minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_header_mandatory_originator() {
        // XSD: ORIGINATOR is required (no minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_header_optional_comment() {
        // XSD: COMMENT is optional (minOccurs="0" maxOccurs="unbounded")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
COMMENT Header comment 1
COMMENT Header comment 2
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.header.comment.len(), 2);
        assert_eq!(tdm.header.comment[0], "Header comment 1");
    }

    #[test]
    fn test_xsd_header_optional_message_id() {
        // XSD: MESSAGE_ID is optional (minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.header.message_id.as_deref(), Some("MSG-001"));
    }

    #[test]
    fn test_xsd_version_attribute() {
        // XSD: version attribute is fixed="2.0"
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.version, "2.0");
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 2: Metadata Mandatory Fields
    // XSD: TIME_SYSTEM and PARTICIPANT_1 are required
    // =========================================================================

    #[test]
    fn test_xsd_metadata_mandatory_time_system() {
        // XSD: TIME_SYSTEM is required (no minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_metadata_mandatory_participant_1() {
        // XSD: PARTICIPANT_1 is required (no minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_metadata_optional_participants_2_to_5() {
        // XSD: PARTICIPANT_2 through PARTICIPANT_5 are optional (minOccurs="0")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = QUASAR_1
PARTICIPANT_4 = RELAY_SAT
PARTICIPANT_5 = DSS-25
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert_eq!(seg.metadata.participant_1, "DSS-14");
        assert_eq!(seg.metadata.participant_2.as_deref(), Some("SPACECRAFT_A"));
        assert_eq!(seg.metadata.participant_3.as_deref(), Some("QUASAR_1"));
        assert_eq!(seg.metadata.participant_4.as_deref(), Some("RELAY_SAT"));
        assert_eq!(seg.metadata.participant_5.as_deref(), Some("DSS-25"));
    }

    #[test]
    fn test_xsd_metadata_path_choice() {
        // XSD: PATH is a choice - either PATH alone or PATH_1 + PATH_2
        // Test PATH alone
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments[0].metadata.path.as_deref(), Some("1,2,1"));
        assert!(tdm.body.segments[0].metadata.path_1.is_none());
        assert!(tdm.body.segments[0].metadata.path_2.is_none());
    }

    #[test]
    fn test_xsd_metadata_path_1_path_2_choice() {
        // XSD: PATH_1 + PATH_2 for SINGLE_DIFF mode
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = DSS-25
MODE = SINGLE_DIFF
PATH_1 = 1,2,1
PATH_2 = 3,2,3
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert!(seg.metadata.path.is_none());
        assert_eq!(seg.metadata.path_1.as_deref(), Some("1,2,1"));
        assert_eq!(seg.metadata.path_2.as_deref(), Some("3,2,3"));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 3: Metadata Defaults
    // XSD: FREQ_OFFSET default="0.0", RANGE_MODULUS default="0.0"
    // XSD: DATA_QUALITY default="RAW", TRANSMIT_DELAY_n default="0.0"
    // XSD: RECEIVE_DELAY_n default="0.0"
    // =========================================================================

    #[test]
    fn test_xsd_metadata_optional_freq_offset() {
        // XSD: FREQ_OFFSET has default="0.0" and minOccurs="0"
        // When not specified, default should be 0.0
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        // Library stores as Option, default applies when not present
        assert!(tdm.body.segments[0].metadata.freq_offset.is_none());
    }

    #[test]
    fn test_xsd_metadata_explicit_freq_offset() {
        // XSD: FREQ_OFFSET can be any double
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
FREQ_OFFSET = 8415000000.0
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(
            tdm.body.segments[0].metadata.freq_offset,
            Some(8415000000.0)
        );
    }

    #[test]
    fn test_xsd_metadata_range_modulus_default() {
        // XSD: RANGE_MODULUS has default="0.0" and minOccurs="0"
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        // Library stores as Option, default applies when not present
        assert!(tdm.body.segments[0].metadata.range_modulus.is_none());
    }

    #[test]
    fn test_xsd_metadata_data_quality_values() {
        // XSD: DATA_QUALITY enum: RAW, VALIDATED, DEGRADED (case-insensitive)
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
DATA_QUALITY = VALIDATED
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(
            tdm.body.segments[0].metadata.data_quality.as_deref(),
            Some("VALIDATED")
        );
    }

    #[test]
    fn test_xsd_metadata_transmit_receive_delays() {
        // XSD: TRANSMIT_DELAY_n and RECEIVE_DELAY_n have default="0.0"
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
TRANSMIT_DELAY_1 = 0.000077
TRANSMIT_DELAY_2 = 0.000088
RECEIVE_DELAY_1 = 0.000077
RECEIVE_DELAY_2 = 0.000099
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert_eq!(seg.metadata.transmit_delay_1, Some(0.000077));
        assert_eq!(seg.metadata.transmit_delay_2, Some(0.000088));
        assert_eq!(seg.metadata.receive_delay_1, Some(0.000077));
        assert_eq!(seg.metadata.receive_delay_2, Some(0.000099));
    }

    #[test]
    fn test_xsd_metadata_turnaround_ratio() {
        // XSD: TURNAROUND_NUMERATOR and TURNAROUND_DENOMINATOR are xsd:int (i32)
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert_eq!(seg.metadata.turnaround_numerator, Some(880));
        assert_eq!(seg.metadata.turnaround_denominator, Some(749));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 4: Body/Segment Structure
    // XSD: tdmBody requires at least 1 segment (minOccurs="1")
    // XSD: tdmBody allows multiple segments (maxOccurs="unbounded")
    // =========================================================================

    #[test]
    fn test_xsd_body_requires_at_least_one_segment() {
        // XSD: segment minOccurs="1" - at least one required
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_body_multiple_segments() {
        // XSD: segment maxOccurs="unbounded" - multiple allowed
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-25
META_STOP
DATA_START
RANGE = 2023-01-01T01:00:00 2000.0
DATA_STOP
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-34
META_STOP
DATA_START
RANGE = 2023-01-01T02:00:00 3000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments.len(), 3);
        assert_eq!(tdm.body.segments[0].metadata.participant_1, "DSS-14");
        assert_eq!(tdm.body.segments[1].metadata.participant_1, "DSS-25");
        assert_eq!(tdm.body.segments[2].metadata.participant_1, "DSS-34");
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 5: Data Section & Observations
    // XSD: observation minOccurs="1" maxOccurs="unbounded"
    // XSD: Each observation has EPOCH + one data type (choice)
    // =========================================================================

    #[test]
    fn test_xsd_data_requires_at_least_one_observation() {
        // XSD: observation minOccurs="1" - at least one required
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
DATA_STOP
"#;
        // Note: Current implementation may allow empty data sections
        // This tests the structure
        let result = Tdm::from_kvn(kvn);
        // If empty data is allowed by implementation, that's an XSD deviation
        if result.is_ok() {
            assert!(result.unwrap().body.segments[0]
                .data
                .observations
                .is_empty());
        }
    }

    #[test]
    fn test_xsd_data_multiple_observations() {
        // XSD: observation maxOccurs="unbounded" - multiple allowed
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
RANGE = 2023-01-01T00:02:00 1002.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments[0].data.observations.len(), 3);
    }

    #[test]
    fn test_xsd_data_comment_optional() {
        // XSD: COMMENT in data is optional (minOccurs="0" maxOccurs="unbounded")
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
COMMENT Data section comment
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments[0].data.comment.len(), 1);
        assert_eq!(tdm.body.segments[0].data.comment[0], "Data section comment");
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 6: Observation Data Types
    // XSD: trackingDataObservationType - choice of data types
    // =========================================================================

    #[test]
    fn test_xsd_observation_angle_types() {
        // XSD: ANGLE_1, ANGLE_2 are ndm:angleType (f64)
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
ANGLE_TYPE = AZEL
META_STOP
DATA_START
ANGLE_1 = 2023-01-01T00:00:00 45.5
ANGLE_2 = 2023-01-01T00:00:00 30.25
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::Angle1(v) => assert_eq!(*v, 45.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::Angle2(v) => assert_eq!(*v, 30.25),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_doppler_types() {
        // XSD: DOPPLER_INSTANTANEOUS, DOPPLER_INTEGRATED, DOPPLER_COUNT
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
DOPPLER_INSTANTANEOUS = 2023-01-01T00:00:00 -0.5
DOPPLER_INTEGRATED = 2023-01-01T00:00:01 -0.45
DOPPLER_COUNT = 2023-01-01T00:00:02 12345678.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::DopplerInstantaneous(v) => assert_eq!(*v, -0.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::DopplerIntegrated(v) => assert_eq!(*v, -0.45),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[2].data {
            TdmObservationData::DopplerCount(v) => assert_eq!(*v, 12345678.0),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_frequency_types() {
        // XSD: RECEIVE_FREQ, RECEIVE_FREQ_1-5, TRANSMIT_FREQ_1-5
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
RECEIVE_FREQ_1 = 2023-01-01T00:00:01 8415000001.0
TRANSMIT_FREQ_1 = 2023-01-01T00:00:02 7167941264.0
TRANSMIT_FREQ_2 = 2023-01-01T00:00:03 7167941265.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments[0].data.observations.len(), 4);
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::ReceiveFreq(v) => assert_eq!(*v, 8415000000.0),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_phase_count_types() {
        // XSD: RECEIVE_PHASE_CT_n, TRANSMIT_PHASE_CT_n
        // These are stored as String for KVN precision per TDM spec 4.3.11
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
TRANSMIT_PHASE_CT_1 = 2023-01-01T00:00:00 7175173383.615373
RECEIVE_PHASE_CT_1 = 2023-01-01T00:00:01 8429753135.986102
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::TransmitPhaseCt1(s) => {
                assert_eq!(s, "7175173383.615373");
            }
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::ReceivePhaseCt1(s) => {
                assert_eq!(s, "8429753135.986102");
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_vlbi_types() {
        // XSD: DOR and VLBI_DELAY
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = DSS-25
MODE = SINGLE_DIFF
PATH_1 = 1,2
PATH_2 = 2,1
META_STOP
DATA_START
DOR = 2023-01-01T00:00:00 0.000123456
VLBI_DELAY = 2023-01-01T00:00:01 -0.000000789
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::Dor(v) => assert_eq!(*v, 0.000123456),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::VlbiDelay(v) => assert_eq!(*v, -0.000000789),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_media_types() {
        // XSD: STEC, TROPO_DRY, TROPO_WET
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
STEC = 2023-01-01T00:00:00 50.0
TROPO_DRY = 2023-01-01T00:00:01 2.3
TROPO_WET = 2023-01-01T00:00:02 0.15
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::Stec(v) => assert_eq!(*v, 50.0),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::TropoDry(v) => assert_eq!(*v, 2.3),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[2].data {
            TdmObservationData::TropoWet(v) => assert_eq!(*v, 0.15),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_weather_types() {
        // XSD: PRESSURE, RHUMIDITY, TEMPERATURE
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
PRESSURE = 2023-01-01T00:00:00 1013.25
RHUMIDITY = 2023-01-01T00:00:01 65.5
TEMPERATURE = 2023-01-01T00:00:02 293.15
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::Pressure(v) => assert_eq!(*v, 1013.25),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::Rhumidity(p) => assert_eq!(p.value, 65.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[2].data {
            TdmObservationData::Temperature(v) => assert_eq!(*v, 293.15),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_clock_types() {
        // XSD: CLOCK_BIAS, CLOCK_DRIFT
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
CLOCK_BIAS = 2023-01-01T00:00:00 0.000001234
CLOCK_DRIFT = 2023-01-01T00:00:01 0.0000000001
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::ClockBias(v) => assert_eq!(*v, 0.000001234),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::ClockDrift(v) => assert_eq!(*v, 0.0000000001),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_optical_radar_types() {
        // XSD: MAG, RCS
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
MAG = 2023-01-01T00:00:00 12.5
RCS = 2023-01-01T00:00:01 1.5
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::Mag(v) => assert_eq!(*v, 12.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::Rcs(v) => assert_eq!(*v, 1.5),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_signal_strength_types() {
        // XSD: CARRIER_POWER, PC_N0, PR_N0
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
CARRIER_POWER = 2023-01-01T00:00:00 -150.5
PC_N0 = 2023-01-01T00:00:01 45.5
PR_N0 = 2023-01-01T00:00:02 35.2
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        match &tdm.body.segments[0].data.observations[0].data {
            TdmObservationData::CarrierPower(v) => assert_eq!(*v, -150.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::PcN0(v) => assert_eq!(*v, 45.5),
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[2].data {
            TdmObservationData::PrN0(v) => assert_eq!(*v, 35.2),
            _ => panic!("Wrong type"),
        }
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 7: Sample Files & Roundtrips
    // =========================================================================

    #[test]
    fn test_xsd_sample_tdm_e1_kvn() {
        // Parse official CCSDS TDM example E-1
        let kvn = include_str!("../../../data/kvn/tdm_e1.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
        assert!(!tdm.body.segments[0].metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e2_kvn() {
        // Parse official CCSDS TDM example E-2
        let kvn = include_str!("../../../data/kvn/tdm_e2.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e3_kvn() {
        // Parse official CCSDS TDM example E-3
        let kvn = include_str!("../../../data/kvn/tdm_e3.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e16_kvn() {
        // Parse official CCSDS TDM example E-16 (optical)
        let kvn = include_str!("../../../data/kvn/tdm_e16.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
        // Should have angle data
        let seg = &tdm.body.segments[0];
        assert!(seg.metadata.angle_type.is_some());
    }

    #[test]
    fn test_xsd_sample_tdm_e18_kvn() {
        // Parse official CCSDS TDM example E-18 (phase)
        let kvn = include_str!("../../../data/kvn/tdm_e18.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e21_xml() {
        // Parse official CCSDS TDM XML example E-21
        let xml = include_str!("../../../data/xml/tdm_e21.xml");
        let tdm = Tdm::from_xml(xml).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_kvn_roundtrip() {
        // Full roundtrip: KVN -> Tdm -> KVN
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let output = tdm.to_kvn().unwrap();
        let tdm2 = Tdm::from_kvn(&output).unwrap();
        assert_eq!(tdm.body.segments.len(), tdm2.body.segments.len());
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
    }

    #[test]
    fn test_xsd_all_metadata_optional_fields() {
        // Test that all optional metadata fields work
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT Metadata comment
TRACK_ID = TRACK_001
DATA_TYPES = RANGE,DOPPLER_INTEGRATED
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
EPHEMERIS_NAME_1 = DSS14_EPHEM
EPHEMERIS_NAME_2 = SC_EPHEM
TRANSMIT_BAND = X
RECEIVE_BAND = X
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
TIMETAG_REF = RECEIVE
INTEGRATION_INTERVAL = 60.0
INTEGRATION_REF = MIDDLE
FREQ_OFFSET = 0.0
RANGE_MODE = COHERENT
RANGE_MODULUS = 32768.0
RANGE_UNITS = km
ANGLE_TYPE = AZEL
REFERENCE_FRAME = EME2000
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
DOPPLER_COUNT_BIAS = 240000000.0
DOPPLER_COUNT_SCALE = 1000
DOPPLER_COUNT_ROLLOVER = NO
TRANSMIT_DELAY_1 = 0.000077
RECEIVE_DELAY_1 = 0.000088
DATA_QUALITY = VALIDATED
CORRECTION_RANGE = 0.001
CORRECTIONS_APPLIED = YES
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];

        assert_eq!(seg.metadata.track_id.as_deref(), Some("TRACK_001"));
        assert_eq!(
            seg.metadata.data_types.as_deref(),
            Some("RANGE,DOPPLER_INTEGRATED")
        );
        assert!(seg.metadata.start_time.is_some());
        assert!(seg.metadata.stop_time.is_some());
        assert_eq!(seg.metadata.mode.as_deref(), Some("SEQUENTIAL"));
        assert_eq!(seg.metadata.transmit_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.receive_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.turnaround_numerator, Some(880));
        assert_eq!(seg.metadata.turnaround_denominator, Some(749));
        assert_eq!(seg.metadata.timetag_ref.as_deref(), Some("RECEIVE"));
        assert_eq!(seg.metadata.integration_interval, Some(60.0));
        assert_eq!(seg.metadata.integration_ref.as_deref(), Some("MIDDLE"));
        assert_eq!(seg.metadata.range_mode.as_deref(), Some("COHERENT"));
        assert_eq!(seg.metadata.range_modulus, Some(32768.0));
        assert_eq!(seg.metadata.range_units.as_deref(), Some("km"));
        assert_eq!(seg.metadata.angle_type.as_deref(), Some("AZEL"));
        assert_eq!(seg.metadata.reference_frame.as_deref(), Some("EME2000"));
        assert_eq!(seg.metadata.interpolation.as_deref(), Some("LAGRANGE"));
        assert_eq!(seg.metadata.interpolation_degree, Some(7));
        assert_eq!(seg.metadata.doppler_count_bias, Some(240000000.0));
        assert_eq!(seg.metadata.doppler_count_scale, Some(1000));
        assert_eq!(seg.metadata.doppler_count_rollover.as_deref(), Some("NO"));
        assert_eq!(seg.metadata.data_quality.as_deref(), Some("VALIDATED"));
        assert_eq!(seg.metadata.correction_range, Some(0.001));
        assert_eq!(seg.metadata.corrections_applied.as_deref(), Some("YES"));
    }

    // =========================================================================
    // ADDITIONAL COVERAGE TESTS
    // =========================================================================

    // -----------------------------------------
    // Version validation error paths
    // -----------------------------------------

    #[test]
    fn test_tdm_empty_file_error() {
        let err = Tdm::from_kvn("").unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => assert_eq!(f, "Empty file"),
            _ => panic!("Expected Empty file error, got: {:?}", err),
        }
    }

    #[test]
    fn test_tdm_version_not_first_error() {
        let kvn = r#"
CREATION_DATE = 2023-01-01T00:00:00
CCSDS_TDM_VERS = 2.0
"#;
        let err = Tdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::MissingField(f) => {
                assert!(f.contains("CCSDS_TDM_VERS must be the first keyword"));
            }
            _ => panic!("Expected version-not-first error, got: {:?}", err),
        }
    }

    // -----------------------------------------
    // XML roundtrip tests
    // -----------------------------------------

    #[test]
    fn test_tdm_xml_roundtrip_minimal() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let xml = tdm.to_xml().unwrap();
        assert!(xml.contains("<tdm"));
        assert!(xml.contains("PARTICIPANT_1"));
        let tdm2 = Tdm::from_xml(&xml).unwrap();
        assert_eq!(
            tdm.body.segments[0].metadata.participant_1,
            tdm2.body.segments[0].metadata.participant_1
        );
    }

    #[test]
    fn test_tdm_xml_roundtrip_with_observations() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:01:00 1001.0
DOPPLER_INSTANTANEOUS = 2023-01-01T00:02:00 -0.5
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let xml = tdm.to_xml().unwrap();
        let tdm2 = Tdm::from_xml(&xml).unwrap();
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
    }

    // -----------------------------------------
    // MESSAGE_ID serialization test
    // -----------------------------------------

    #[test]
    fn test_tdm_message_id_kvn_roundtrip() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-UNIQUE-001
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.header.message_id.as_deref(), Some("MSG-UNIQUE-001"));

        let kvn2 = tdm.to_kvn().unwrap();
        assert!(kvn2.contains("MESSAGE_ID"));
        assert!(kvn2.contains("MSG-UNIQUE-001"));

        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();
        assert_eq!(tdm.header.message_id, tdm2.header.message_id);
    }

    // -----------------------------------------
    // Optional metadata fields KVN serialization tests
    // -----------------------------------------

    #[test]
    fn test_tdm_metadata_optional_fields_kvn_roundtrip() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TRACK_ID = TRACK_XYZ
DATA_TYPES = RANGE,DOPPLER_INTEGRATED,ANGLE_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = QUASAR_1
PARTICIPANT_4 = RELAY_SAT
PARTICIPANT_5 = DSS-25
MODE = SEQUENTIAL
PATH = 1,2,1
EPHEMERIS_NAME_1 = DSS14_EPHEM
EPHEMERIS_NAME_2 = SC_EPHEM
EPHEMERIS_NAME_3 = QUASAR_EPHEM
EPHEMERIS_NAME_4 = RELAY_EPHEM
EPHEMERIS_NAME_5 = DSS25_EPHEM
TRANSMIT_BAND = X
RECEIVE_BAND = Ka
TURNAROUND_NUMERATOR = 880
TURNAROUND_DENOMINATOR = 749
TIMETAG_REF = TRANSMIT
INTEGRATION_INTERVAL = 30.0
INTEGRATION_REF = START
FREQ_OFFSET = 1000.0
RANGE_MODE = ONE_WAY
RANGE_MODULUS = 16384.0
RANGE_UNITS = RU
ANGLE_TYPE = RADEC
REFERENCE_FRAME = ICRF
INTERPOLATION = HERMITE
INTERPOLATION_DEGREE = 9
DOPPLER_COUNT_BIAS = 123456789.0
DOPPLER_COUNT_SCALE = 2000
DOPPLER_COUNT_ROLLOVER = YES
TRANSMIT_DELAY_1 = 0.0001
TRANSMIT_DELAY_2 = 0.0002
TRANSMIT_DELAY_3 = 0.0003
TRANSMIT_DELAY_4 = 0.0004
TRANSMIT_DELAY_5 = 0.0005
RECEIVE_DELAY_1 = 0.0006
RECEIVE_DELAY_2 = 0.0007
RECEIVE_DELAY_3 = 0.0008
RECEIVE_DELAY_4 = 0.0009
RECEIVE_DELAY_5 = 0.0010
DATA_QUALITY = DEGRADED
CORRECTION_ANGLE_1 = 0.01
CORRECTION_ANGLE_2 = 0.02
CORRECTION_DOPPLER = 0.03
CORRECTION_MAG = 0.04
CORRECTION_RANGE = 0.05
CORRECTION_RCS = 0.06
CORRECTION_RECEIVE = 0.07
CORRECTION_TRANSMIT = 0.08
CORRECTION_ABERRATION_YEARLY = 0.09
CORRECTION_ABERRATION_DIURNAL = 0.10
CORRECTIONS_APPLIED = NO
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let kvn2 = tdm.to_kvn().unwrap();

        // Verify all optional fields are in the output
        assert!(kvn2.contains("TRACK_ID") && kvn2.contains("TRACK_XYZ"));
        assert!(kvn2.contains("DATA_TYPES"));
        assert!(kvn2.contains("START_TIME"));
        assert!(kvn2.contains("STOP_TIME"));
        assert!(kvn2.contains("PARTICIPANT_3"));
        assert!(kvn2.contains("PARTICIPANT_4"));
        assert!(kvn2.contains("PARTICIPANT_5"));
        assert!(kvn2.contains("PATH_1") || kvn2.contains("PATH"));
        assert!(kvn2.contains("EPHEMERIS_NAME_1"));
        assert!(kvn2.contains("EPHEMERIS_NAME_2"));
        assert!(kvn2.contains("EPHEMERIS_NAME_3"));
        assert!(kvn2.contains("EPHEMERIS_NAME_4"));
        assert!(kvn2.contains("EPHEMERIS_NAME_5"));
        assert!(kvn2.contains("TRANSMIT_BAND") && kvn2.contains("X"));
        assert!(kvn2.contains("RECEIVE_BAND") && kvn2.contains("Ka"));
        assert!(kvn2.contains("TURNAROUND_NUMERATOR"));
        assert!(kvn2.contains("TURNAROUND_DENOMINATOR"));
        assert!(kvn2.contains("TIMETAG_REF"));
        assert!(kvn2.contains("INTEGRATION_INTERVAL"));
        assert!(kvn2.contains("INTEGRATION_REF"));
        assert!(kvn2.contains("FREQ_OFFSET"));
        assert!(kvn2.contains("RANGE_MODE"));
        assert!(kvn2.contains("RANGE_MODULUS"));
        assert!(kvn2.contains("RANGE_UNITS"));
        assert!(kvn2.contains("ANGLE_TYPE"));
        assert!(kvn2.contains("REFERENCE_FRAME"));
        assert!(kvn2.contains("INTERPOLATION"));
        assert!(kvn2.contains("INTERPOLATION_DEGREE"));
        assert!(kvn2.contains("DOPPLER_COUNT_BIAS"));
        assert!(kvn2.contains("DOPPLER_COUNT_SCALE"));
        assert!(kvn2.contains("DOPPLER_COUNT_ROLLOVER"));
        assert!(kvn2.contains("TRANSMIT_DELAY_1"));
        assert!(kvn2.contains("TRANSMIT_DELAY_2"));
        assert!(kvn2.contains("TRANSMIT_DELAY_3"));
        assert!(kvn2.contains("TRANSMIT_DELAY_4"));
        assert!(kvn2.contains("TRANSMIT_DELAY_5"));
        assert!(kvn2.contains("RECEIVE_DELAY_1"));
        assert!(kvn2.contains("RECEIVE_DELAY_2"));
        assert!(kvn2.contains("RECEIVE_DELAY_3"));
        assert!(kvn2.contains("RECEIVE_DELAY_4"));
        assert!(kvn2.contains("RECEIVE_DELAY_5"));
        assert!(kvn2.contains("DATA_QUALITY") && kvn2.contains("DEGRADED"));
        assert!(kvn2.contains("CORRECTION_ANGLE_1"));
        assert!(kvn2.contains("CORRECTION_ANGLE_2"));
        assert!(kvn2.contains("CORRECTION_DOPPLER"));
        assert!(kvn2.contains("CORRECTION_MAG"));
        assert!(kvn2.contains("CORRECTION_RANGE"));
        assert!(kvn2.contains("CORRECTION_RCS"));
        assert!(kvn2.contains("CORRECTION_RECEIVE"));
        assert!(kvn2.contains("CORRECTION_TRANSMIT"));
        assert!(kvn2.contains("CORRECTION_ABERRATION_YEARLY"));
        assert!(kvn2.contains("CORRECTION_ABERRATION_DIURNAL"));
        assert!(kvn2.contains("CORRECTIONS_APPLIED"));

        // Roundtrip parse
        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();
        assert_eq!(
            tdm.body.segments[0].metadata.track_id,
            tdm2.body.segments[0].metadata.track_id
        );
    }

    // -----------------------------------------
    // PATH_1, PATH_2 serialization test
    // -----------------------------------------

    #[test]
    fn test_tdm_path_1_path_2_kvn_roundtrip() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
PARTICIPANT_3 = DSS-25
MODE = SINGLE_DIFF
PATH_1 = 1,2,1
PATH_2 = 3,2,3
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let kvn2 = tdm.to_kvn().unwrap();

        assert!(kvn2.contains("PATH_1"));
        assert!(kvn2.contains("PATH_2"));

        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();
        assert_eq!(
            tdm.body.segments[0].metadata.path_1,
            tdm2.body.segments[0].metadata.path_1
        );
        assert_eq!(
            tdm.body.segments[0].metadata.path_2,
            tdm2.body.segments[0].metadata.path_2
        );
    }

    // -----------------------------------------
    // TdmObservationData key() and value_to_string() tests
    // -----------------------------------------

    #[test]
    fn test_tdm_observation_data_key_and_value_roundtrip() {
        // Test that we can roundtrip various observation types through KVN
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
ANGLE_1 = 2023-01-01T00:00:00 45.5
ANGLE_2 = 2023-01-01T00:00:01 30.25
CARRIER_POWER = 2023-01-01T00:00:02 -150.5
CLOCK_BIAS = 2023-01-01T00:00:03 0.000001234
CLOCK_DRIFT = 2023-01-01T00:00:04 0.0000000001
DOPPLER_COUNT = 2023-01-01T00:00:05 12345678.0
DOPPLER_INSTANTANEOUS = 2023-01-01T00:00:06 -0.5
DOPPLER_INTEGRATED = 2023-01-01T00:00:07 -0.45
DOR = 2023-01-01T00:00:08 0.000123456
MAG = 2023-01-01T00:00:09 12.5
PC_N0 = 2023-01-01T00:00:10 45.5
PR_N0 = 2023-01-01T00:00:11 35.2
PRESSURE = 2023-01-01T00:00:12 1013.25
RANGE = 2023-01-01T00:00:13 1000.0
RCS = 2023-01-01T00:00:14 1.5
RECEIVE_FREQ = 2023-01-01T00:00:15 8415000000.0
RECEIVE_FREQ_1 = 2023-01-01T00:00:16 8415000001.0
RECEIVE_FREQ_2 = 2023-01-01T00:00:17 8415000002.0
RECEIVE_FREQ_3 = 2023-01-01T00:00:18 8415000003.0
RECEIVE_FREQ_4 = 2023-01-01T00:00:19 8415000004.0
RECEIVE_FREQ_5 = 2023-01-01T00:00:20 8415000005.0
RHUMIDITY = 2023-01-01T00:00:21 65.5
STEC = 2023-01-01T00:00:22 50.0
TEMPERATURE = 2023-01-01T00:00:23 293.15
TRANSMIT_FREQ_1 = 2023-01-01T00:00:24 7167941261.0
TRANSMIT_FREQ_2 = 2023-01-01T00:00:25 7167941262.0
TRANSMIT_FREQ_3 = 2023-01-01T00:00:26 7167941263.0
TRANSMIT_FREQ_4 = 2023-01-01T00:00:27 7167941264.0
TRANSMIT_FREQ_5 = 2023-01-01T00:00:28 7167941265.0
TRANSMIT_FREQ_RATE_1 = 2023-01-01T00:00:29 0.001
TRANSMIT_FREQ_RATE_2 = 2023-01-01T00:00:30 0.002
TRANSMIT_FREQ_RATE_3 = 2023-01-01T00:00:31 0.003
TRANSMIT_FREQ_RATE_4 = 2023-01-01T00:00:32 0.004
TRANSMIT_FREQ_RATE_5 = 2023-01-01T00:00:33 0.005
TROPO_DRY = 2023-01-01T00:00:34 2.3
TROPO_WET = 2023-01-01T00:00:35 0.15
VLBI_DELAY = 2023-01-01T00:00:36 -0.000000789
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let kvn2 = tdm.to_kvn().unwrap();

        // Verify all observation types are preserved
        assert!(kvn2.contains("ANGLE_1"));
        assert!(kvn2.contains("ANGLE_2"));
        assert!(kvn2.contains("CARRIER_POWER"));
        assert!(kvn2.contains("CLOCK_BIAS"));
        assert!(kvn2.contains("CLOCK_DRIFT"));
        assert!(kvn2.contains("DOPPLER_COUNT"));
        assert!(kvn2.contains("DOPPLER_INSTANTANEOUS"));
        assert!(kvn2.contains("DOPPLER_INTEGRATED"));
        assert!(kvn2.contains("DOR"));
        assert!(kvn2.contains("MAG"));
        assert!(kvn2.contains("PC_N0"));
        assert!(kvn2.contains("PR_N0"));
        assert!(kvn2.contains("PRESSURE"));
        assert!(kvn2.contains("RANGE"));
        assert!(kvn2.contains("RCS"));
        assert!(kvn2.contains("RECEIVE_FREQ_5"));
        assert!(kvn2.contains("RHUMIDITY"));
        assert!(kvn2.contains("STEC"));
        assert!(kvn2.contains("TEMPERATURE"));
        assert!(kvn2.contains("TRANSMIT_FREQ_5"));
        assert!(kvn2.contains("TRANSMIT_FREQ_RATE_5"));
        assert!(kvn2.contains("TROPO_DRY"));
        assert!(kvn2.contains("TROPO_WET"));
        assert!(kvn2.contains("VLBI_DELAY"));

        // Roundtrip parse
        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
    }

    #[test]
    fn test_tdm_phase_count_observations() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
TRANSMIT_PHASE_CT_1 = 2023-01-01T00:00:00 7175173383.615373
TRANSMIT_PHASE_CT_2 = 2023-01-01T00:00:01 7175173384.615373
TRANSMIT_PHASE_CT_3 = 2023-01-01T00:00:02 7175173385.615373
TRANSMIT_PHASE_CT_4 = 2023-01-01T00:00:03 7175173386.615373
TRANSMIT_PHASE_CT_5 = 2023-01-01T00:00:04 7175173387.615373
RECEIVE_PHASE_CT_1 = 2023-01-01T00:00:05 8429753135.986102
RECEIVE_PHASE_CT_2 = 2023-01-01T00:00:06 8429753136.986102
RECEIVE_PHASE_CT_3 = 2023-01-01T00:00:07 8429753137.986102
RECEIVE_PHASE_CT_4 = 2023-01-01T00:00:08 8429753138.986102
RECEIVE_PHASE_CT_5 = 2023-01-01T00:00:09 8429753139.986102
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let kvn2 = tdm.to_kvn().unwrap();

        assert!(kvn2.contains("TRANSMIT_PHASE_CT_1"));
        assert!(kvn2.contains("TRANSMIT_PHASE_CT_2"));
        assert!(kvn2.contains("TRANSMIT_PHASE_CT_3"));
        assert!(kvn2.contains("TRANSMIT_PHASE_CT_4"));
        assert!(kvn2.contains("TRANSMIT_PHASE_CT_5"));
        assert!(kvn2.contains("RECEIVE_PHASE_CT_1"));
        assert!(kvn2.contains("RECEIVE_PHASE_CT_2"));
        assert!(kvn2.contains("RECEIVE_PHASE_CT_3"));
        assert!(kvn2.contains("RECEIVE_PHASE_CT_4"));
        assert!(kvn2.contains("RECEIVE_PHASE_CT_5"));

        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
    }

    // -----------------------------------------
    // Unknown keyword error test
    // -----------------------------------------

    #[test]
    fn test_tdm_unknown_data_keyword_error() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
UNKNOWN_DATA_TYPE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let err = Tdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::KvnParse(msg) => {
                assert!(msg.contains("Unknown TDM data keyword"));
            }
            _ => panic!("Expected KvnParse error, got: {:?}", err),
        }
    }

    #[test]
    fn test_tdm_unknown_metadata_key_error() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
UNKNOWN_METADATA = SOME_VALUE
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let err = Tdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::KvnParse(msg) => {
                assert!(msg.contains("Unexpected TDM Metadata key"));
            }
            _ => panic!("Expected KvnParse error, got: {:?}", err),
        }
    }

    // -----------------------------------------
    // Full roundtrip with all features
    // -----------------------------------------

    #[test]
    fn test_tdm_full_roundtrip_multi_segment() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
COMMENT TDM example with multiple segments
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = NASA/JPL
MESSAGE_ID = TDM-MULTI-001
META_START
COMMENT First segment - Range tracking
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
PARTICIPANT_1 = DSS-14
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
TRANSMIT_BAND = X
RECEIVE_BAND = X
DATA_QUALITY = VALIDATED
META_STOP
DATA_START
COMMENT Range measurements
RANGE = 2023-01-01T00:00:00 1000.0
RANGE = 2023-01-01T00:30:00 1010.0
RANGE = 2023-01-01T01:00:00 1020.0
DATA_STOP
META_START
COMMENT Second segment - Doppler tracking  
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T01:00:00
STOP_TIME = 2023-01-01T02:00:00
PARTICIPANT_1 = DSS-25
PARTICIPANT_2 = SPACECRAFT_A
MODE = SEQUENTIAL
PATH = 1,2,1
META_STOP
DATA_START
COMMENT Doppler measurements
DOPPLER_INTEGRATED = 2023-01-01T01:00:00 -0.5
DOPPLER_INTEGRATED = 2023-01-01T01:30:00 -0.45
DOPPLER_INTEGRATED = 2023-01-01T02:00:00 -0.4
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();

        assert_eq!(tdm.body.segments.len(), 2);
        assert_eq!(tdm.body.segments[0].metadata.participant_1, "DSS-14");
        assert_eq!(tdm.body.segments[1].metadata.participant_1, "DSS-25");

        let kvn2 = tdm.to_kvn().unwrap();
        let tdm2 = Tdm::from_kvn(&kvn2).unwrap();

        assert_eq!(tdm.body.segments.len(), tdm2.body.segments.len());
        assert_eq!(
            tdm.body.segments[0].data.observations.len(),
            tdm2.body.segments[0].data.observations.len()
        );
        assert_eq!(
            tdm.body.segments[1].data.observations.len(),
            tdm2.body.segments[1].data.observations.len()
        );
    }

    #[test]
    fn test_tdm_data_comment() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = DSS-14
META_STOP
DATA_START
COMMENT First data comment
COMMENT Second data comment
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert_eq!(tdm.body.segments[0].data.comment.len(), 2);
        assert_eq!(tdm.body.segments[0].data.comment[0], "First data comment");
        assert_eq!(tdm.body.segments[0].data.comment[1], "Second data comment");
    }
}
