// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, StateVectorAcc};
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, FromKvnValue, Ndm, ToKvn};
use crate::types::{
    Epoch, PositionCovariance, PositionCovarianceUnits, PositionVelocityCovariance,
    PositionVelocityCovarianceUnits, VelocityCovariance, VelocityCovarianceUnits,
};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::num::NonZeroU32;

//----------------------------------------------------------------------
// Root OEM Structure
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "oem")]
pub struct Oem {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
    pub header: OdmHeader,
    pub body: OemBody,
}

impl Ndm for Oem {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        // 1. Header (Common to ODM)
        writer.write_pair("CCSDS_OEM_VERS", &self.version);
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
                    key: "CCSDS_OEM_VERS",
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
                        "CCSDS_OEM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        // 2. Header (Now correctly peeks and stops at META_START)
        let header = OdmHeader::from_kvn_tokens(&mut tokens)?;

        // 3. Body
        let body = OemBody::from_kvn_tokens(&mut tokens)?;

        Ok(Oem {
            header,
            body,
            id: Some("CCSDS_OEM_VERS".to_string()),
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
// Body & Segments
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OemBody {
    #[serde(rename = "segment")]
    pub segment: Vec<OemSegment>,
}

impl ToKvn for OemBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for seg in &self.segment {
            seg.write_kvn(writer);
        }
    }
}

impl OemBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut segment = Vec::new();
        while let Some(token) = tokens.peek() {
            match token {
                Ok(KvnLine::BlockStart("META")) => {
                    segment.push(OemSegment::from_kvn_tokens(tokens)?);
                }
                Ok(KvnLine::Empty) | Ok(KvnLine::Comment(_)) => {
                    tokens.next();
                }
                Ok(_) => break,
                Err(_) => {
                    if let Some(Err(e)) = tokens.next() {
                        return Err(e);
                    }
                    unreachable!(); // We peeked an error, so this is unreachable.
                }
            }
        }
        if segment.is_empty() {
            return Err(CcsdsNdmError::MissingField(
                "OEM body must contain at least one segment".into(),
            ));
        }
        Ok(OemBody { segment })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OemSegment {
    pub metadata: OemMetadata,
    pub data: OemData,
}

impl ToKvn for OemSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        self.metadata.write_kvn(writer);
        writer.write_section("META_STOP");
        // No explicit DATA_START in standard OEM, data follows immediately
        self.data.write_kvn(writer);
    }
}

impl OemSegment {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // Expect META_START
        match tokens.next() {
            Some(Ok(KvnLine::BlockStart("META"))) => {}
            _ => return Err(CcsdsNdmError::KvnParse("Expected META_START".into())),
        }
        // ...
        let metadata = OemMetadata::from_kvn_tokens(tokens)?;
        let data = OemData::from_kvn_tokens(tokens)?;
        Ok(OemSegment { metadata, data })
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OemMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub object_id: String,
    pub center_name: String,
    pub ref_frame: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    pub time_system: String,
    pub start_time: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    pub stop_time: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<NonZeroU32>,
}

impl ToKvn for OemMetadata {
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
        writer.write_pair("START_TIME", &self.start_time);
        if let Some(v) = &self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = &self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        writer.write_pair("STOP_TIME", &self.stop_time);
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = &self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

impl OemMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut builder = OemMetadataBuilder::default();
        while let Some(token) = tokens.next() {
            match token? {
                KvnLine::BlockEnd("META") => break,
                KvnLine::Comment(c) => builder.comment.push(c.to_string()),
                KvnLine::Pair { key, val, .. } => builder.match_pair(key, val)?,
                KvnLine::Empty => continue,
                t => {
                    return Err(CcsdsNdmError::KvnParse(format!(
                        "Unexpected token in META: {:?}",
                        t
                    )))
                }
            }
        }
        builder.build()
    }
}

#[derive(Default)]
struct OemMetadataBuilder {
    comment: Vec<String>,
    object_name: Option<String>,
    object_id: Option<String>,
    center_name: Option<String>,
    ref_frame: Option<String>,
    ref_frame_epoch: Option<Epoch>,
    time_system: Option<String>,
    start_time: Option<Epoch>,
    useable_start_time: Option<Epoch>,
    useable_stop_time: Option<Epoch>,
    stop_time: Option<Epoch>,
    interpolation: Option<String>,
    interpolation_degree: Option<NonZeroU32>,
}

impl OemMetadataBuilder {
    fn match_pair(&mut self, key: &str, val: &str) -> Result<()> {
        match key {
            "OBJECT_NAME" => self.object_name = Some(val.into()),
            "OBJECT_ID" => self.object_id = Some(val.into()),
            "CENTER_NAME" => self.center_name = Some(val.into()),
            "REF_FRAME" => self.ref_frame = Some(val.into()),
            "REF_FRAME_EPOCH" => self.ref_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?),
            "TIME_SYSTEM" => self.time_system = Some(val.into()),
            "START_TIME" => self.start_time = Some(FromKvnValue::from_kvn_value(val)?),
            "USEABLE_START_TIME" => {
                self.useable_start_time = Some(FromKvnValue::from_kvn_value(val)?)
            }
            "USEABLE_STOP_TIME" => {
                self.useable_stop_time = Some(FromKvnValue::from_kvn_value(val)?)
            }
            "STOP_TIME" => self.stop_time = Some(FromKvnValue::from_kvn_value(val)?),
            "INTERPOLATION" => self.interpolation = Some(val.into()),
            "INTERPOLATION_DEGREE" => {
                let parsed_u32: u32 = FromKvnValue::from_kvn_value(val)?;
                self.interpolation_degree = Some(NonZeroU32::new(parsed_u32).ok_or_else(|| {
                    CcsdsNdmError::KvnParse(
                        "INTERPOLATION_DEGREE must be a positive integer but was 0".into(),
                    )
                })?);
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

    fn build(self) -> Result<OemMetadata> {
        Ok(OemMetadata {
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
            start_time: self
                .start_time
                .ok_or(CcsdsNdmError::MissingField("START_TIME".into()))?,
            useable_start_time: self.useable_start_time,
            useable_stop_time: self.useable_stop_time,
            stop_time: self
                .stop_time
                .ok_or(CcsdsNdmError::MissingField("STOP_TIME".into()))?,
            interpolation: self.interpolation,
            interpolation_degree: self.interpolation_degree,
        })
    }
}

//----------------------------------------------------------------------
// Data Section
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OemData {
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,

    // XSD sequence: comments -> state vectors -> covariance matrices.
    #[serde(rename = "stateVector", default)]
    pub state_vector: Vec<StateVectorAcc>,

    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub covariance_matrix: Vec<OemCovarianceMatrix>,
}

impl ToKvn for OemData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if !self.state_vector.is_empty() {
            writer.write_empty();
        }
        for sv in &self.state_vector {
            sv.write_kvn(writer);
        }
        for cov in &self.covariance_matrix {
            writer.write_empty();
            cov.write_kvn(writer);
        }
    }
}

impl OemData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut state_vector = Vec::new();
        let mut covariance_matrix = Vec::new();

        // 1. Parse Top-Level Comments first, as per XSD sequence
        while let Some(peek_res) = tokens.peek() {
            if let Err(_) = peek_res {
                if let Some(Err(e)) = tokens.next() {
                    return Err(e);
                }
                unreachable!(); // We peeked an error, so this is unreachable.
            }
            match peek_res.as_ref().unwrap() {
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next(); // Skip empty lines
                }
                _ => break, // Found something that isn't a comment
            }
        }

        // 2. Parse State Vectors
        while let Some(peek_res) = tokens.peek() {
            if let Err(_) = peek_res {
                if let Some(Err(e)) = tokens.next() {
                    return Err(e);
                }
                unreachable!(); // We peeked an error, so this is unreachable.
            }
            match peek_res.as_ref().unwrap() {
                KvnLine::BlockStart("META") | KvnLine::BlockStart("COVARIANCE") => break,
                KvnLine::Raw(_) => {
                    let sv = StateVectorAcc::from_kvn_tokens(tokens)?;
                    state_vector.push(sv);
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Comment(_) => {
                    // Allow comments to be interleaved with state vectors
                    tokens.next();
                }
                _ => break, // Only break on unexpected tokens that are not comments or empty
            }
        }

        // 3. Enforce that at least one state vector exists
        if state_vector.is_empty() {
            return Err(CcsdsNdmError::MissingField(
                "OEM data section must contain at least one state vector".into(),
            ));
        }

        // 4. Parse Covariance Matrices
        while let Some(peek_res) = tokens.peek() {
            if let Err(_) = peek_res {
                if let Some(Err(e)) = tokens.next() {
                    return Err(e);
                }
                unreachable!(); // We peeked an error, so this is unreachable.
            }
            match peek_res.as_ref().unwrap() {
                KvnLine::BlockStart("COVARIANCE") => {
                    tokens.next(); // Consume COVARIANCE_START
                    loop {
                        // Peek to see if we've reached the end of the covariance block or a new matrix begins
                        match tokens.peek() {
                            Some(Ok(KvnLine::BlockEnd("COVARIANCE"))) => {
                                tokens.next(); // Consume COVARIANCE_STOP
                                break;
                            }
                            Some(Ok(KvnLine::Empty)) | Some(Ok(KvnLine::Comment(_))) => {
                                tokens.next(); // Skip empty lines and comments within the block
                            }
                            Some(Ok(_)) => {
                                // Assume a new covariance matrix starts
                                let cov =
                                    OemCovarianceMatrix::parse_single_covariance_matrix(tokens)?;
                                covariance_matrix.push(cov);
                            }
                            None => {
                                return Err(CcsdsNdmError::KvnParse(
                                    "Unexpected EOF within COVARIANCE block".into(),
                                ))
                            }
                            Some(Err(_)) => {
                                if let Some(Err(e)) = tokens.next() {
                                    return Err(e);
                                }
                                unreachable!(); // We peeked an error, so this is unreachable.
                            }
                        }
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Comment(_) => {
                    tokens.next();
                }
                _ => break, // Likely META_START or EOF
            }
        }

        Ok(OemData {
            comment,
            state_vector,
            covariance_matrix,
        })
    }
}

//----------------------------------------------------------------------
// Covariance Matrix
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OemCovarianceMatrix {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub epoch: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_ref_frame: Option<String>,

    // Flattened fields for internal storage, mapped from triangular input
    pub cx_x: PositionCovariance,
    pub cy_x: PositionCovariance,
    pub cy_y: PositionCovariance,
    pub cz_x: PositionCovariance,
    pub cz_y: PositionCovariance,
    pub cz_z: PositionCovariance,
    pub cx_dot_x: PositionVelocityCovariance,
    pub cx_dot_y: PositionVelocityCovariance,
    pub cx_dot_z: PositionVelocityCovariance,
    pub cx_dot_x_dot: VelocityCovariance,
    pub cy_dot_x: PositionVelocityCovariance,
    pub cy_dot_y: PositionVelocityCovariance,
    pub cy_dot_z: PositionVelocityCovariance,
    pub cy_dot_x_dot: VelocityCovariance,
    pub cy_dot_y_dot: VelocityCovariance,
    pub cz_dot_x: PositionVelocityCovariance,
    pub cz_dot_y: PositionVelocityCovariance,
    pub cz_dot_z: PositionVelocityCovariance,
    pub cz_dot_x_dot: VelocityCovariance,
    pub cz_dot_y_dot: VelocityCovariance,
    pub cz_dot_z_dot: VelocityCovariance,
}

impl ToKvn for OemCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COVARIANCE_START");
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", &self.epoch);
        if let Some(rf) = &self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", rf);
        }

        // Write triangular format with scientific notation
        let f = |v: f64| format!("{:.14e}", v);

        writer.write_line(&f(self.cx_x.value));
        writer.write_line(format!("{} {}", f(self.cy_x.value), f(self.cy_y.value)));
        writer.write_line(format!(
            "{} {} {}",
            f(self.cz_x.value),
            f(self.cz_y.value),
            f(self.cz_z.value)
        ));
        writer.write_line(format!(
            "{} {} {} {}",
            f(self.cx_dot_x.value),
            f(self.cx_dot_y.value),
            f(self.cx_dot_z.value),
            f(self.cx_dot_x_dot.value)
        ));
        writer.write_line(format!(
            "{} {} {} {} {}",
            f(self.cy_dot_x.value),
            f(self.cy_dot_y.value),
            f(self.cy_dot_z.value),
            f(self.cy_dot_x_dot.value),
            f(self.cy_dot_y_dot.value)
        ));
        writer.write_line(format!(
            "{} {} {} {} {} {}",
            f(self.cz_dot_x.value),
            f(self.cz_dot_y.value),
            f(self.cz_dot_z.value),
            f(self.cz_dot_x_dot.value),
            f(self.cz_dot_y_dot.value),
            f(self.cz_dot_z_dot.value)
        ));

        writer.write_section("COVARIANCE_STOP");
    }
}

impl OemCovarianceMatrix {
    fn parse_single_covariance_matrix<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let epoch; // Changed from `let mut epoch = None;`
        let mut cov_ref_frame = None;
        let mut floats = Vec::new();

        // Parse comments preceding the epoch/matrix data
        while let Some(peeked) = tokens.peek() {
            match peeked {
                Ok(KvnLine::Comment(_)) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                Ok(KvnLine::Empty) => {
                    tokens.next();
                }
                _ => break, // Found something that isn't a comment or empty line
            }
        }

        // Parse EPOCH
        match tokens.next() {
            Some(Ok(KvnLine::Pair {
                key: "EPOCH", val, ..
            })) => {
                epoch = Some(FromKvnValue::from_kvn_value(val)?);
            }
            t => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Expected EPOCH for covariance matrix, found {:?}",
                    t
                )))
            }
        }

        // Parse optional COV_REF_FRAME
        if let Some(Ok(KvnLine::Pair {
            key: "COV_REF_FRAME",
            val,
            ..
        })) = tokens.peek()
        {
            cov_ref_frame = Some(val.to_string());
            tokens.next(); // Consume COV_REF_FRAME
        }

        // Parse 6 lines of raw float data
        for _ in 0..6 {
            match tokens.next() {
                Some(Ok(KvnLine::Raw(line))) => {
                    for part in line.split_whitespace() {
                        floats.push(part.parse::<f64>().map_err(|_| {
                            CcsdsNdmError::KvnParse(format!(
                                "Invalid float in covariance: {}",
                                part
                            ))
                        })?);
                    }
                }
                t => {
                    return Err(CcsdsNdmError::KvnParse(format!(
                        "Expected raw float data line for covariance matrix, found {:?}",
                        t
                    )))
                }
            }
        }

        if floats.len() != 21 {
            return Err(CcsdsNdmError::KvnParse(format!(
                "Covariance matrix requires 21 values, found {}",
                floats.len()
            )));
        }

        let epoch = epoch.ok_or(CcsdsNdmError::MissingField("EPOCH in covariance".into()))?;

        Ok(OemCovarianceMatrix {
            comment,
            epoch,
            cov_ref_frame,
            cx_x: PositionCovariance::new(floats[0], Some(PositionCovarianceUnits::Km2)),
            cy_x: PositionCovariance::new(floats[1], Some(PositionCovarianceUnits::Km2)),
            cy_y: PositionCovariance::new(floats[2], Some(PositionCovarianceUnits::Km2)),
            cz_x: PositionCovariance::new(floats[3], Some(PositionCovarianceUnits::Km2)),
            cz_y: PositionCovariance::new(floats[4], Some(PositionCovarianceUnits::Km2)),
            cz_z: PositionCovariance::new(floats[5], Some(PositionCovarianceUnits::Km2)),
            cx_dot_x: PositionVelocityCovariance::new(
                floats[6],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cx_dot_y: PositionVelocityCovariance::new(
                floats[7],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cx_dot_z: PositionVelocityCovariance::new(
                floats[8],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cx_dot_x_dot: VelocityCovariance::new(
                floats[9],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
            cy_dot_x: PositionVelocityCovariance::new(
                floats[10],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cy_dot_y: PositionVelocityCovariance::new(
                floats[11],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cy_dot_z: PositionVelocityCovariance::new(
                floats[12],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cy_dot_x_dot: VelocityCovariance::new(
                floats[13],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
            cy_dot_y_dot: VelocityCovariance::new(
                floats[14],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
            cz_dot_x: PositionVelocityCovariance::new(
                floats[15],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cz_dot_y: PositionVelocityCovariance::new(
                floats[16],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cz_dot_z: PositionVelocityCovariance::new(
                floats[17],
                Some(PositionVelocityCovarianceUnits::Km2PerS),
            ),
            cz_dot_x_dot: VelocityCovariance::new(
                floats[18],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
            cz_dot_y_dot: VelocityCovariance::new(
                floats[19],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
            cz_dot_z_dot: VelocityCovariance::new(
                floats[20],
                Some(VelocityCovarianceUnits::Km2PerS2),
            ),
        })
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Annex A2.5.3 OEM Requirements List compliance tests
    // References in comments point to CCSDS 502.0-B-3 Annex A, item numbers.

    #[test]
    fn test_parse_oem_simple() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
2023-01-01T00:01:00 1060 2120 3180 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).expect("Failed to parse OEM");
        assert_eq!(oem.body.segment.len(), 1);
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 2);
        assert_eq!(oem.body.segment[0].data.state_vector[0].x.value, 1000.0);
    }

    #[test]
    fn test_header_requires_creation_date_and_originator() {
        // A2.5.3 Items 5 and 6: CREATION_DATE and ORIGINATOR are mandatory
        let kvn_missing_creation = r#"CCSDS_OEM_VERS = 3.0
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let err1 = Oem::from_kvn(kvn_missing_creation).unwrap_err();
        assert!(matches!(err1, CcsdsNdmError::MissingField(k) if k.contains("CREATION_DATE")));

        let kvn_missing_originator = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let err2 = Oem::from_kvn(kvn_missing_originator).unwrap_err();
        assert!(matches!(err2, CcsdsNdmError::MissingField(k) if k.contains("ORIGINATOR")));
    }

    #[test]
    fn test_header_optional_fields_roundtrip() {
        // A2.5.3 Items 3,4,7: COMMENT, CLASSIFICATION, MESSAGE_ID optional
        let kvn = r#"CCSDS_OEM_VERS = 3.0
    COMMENT This is a header comment
    CLASSIFICATION = SBU
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    MESSAGE_ID = MSG-001
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let out = oem.to_kvn().unwrap();
        assert!(out.contains("CLASSIFICATION"));
        assert!(out.contains("MESSAGE_ID"));
    }

    #[test]
    fn test_metadata_optional_fields() {
        // A2.5.3 Items 10, 15, 18, 19: Optional metadata fields
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT This is a metadata comment
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
REF_FRAME_EPOCH = 2000-01-01T00:00:00
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T01:00:00
USEABLE_STOP_TIME = 2023-01-01T23:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T01:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let meta = &oem.body.segment[0].metadata;
        assert_eq!(meta.comment, vec!["This is a metadata comment"]);
        assert!(meta.ref_frame_epoch.is_some());
        assert!(meta.useable_start_time.is_some());
        assert!(meta.useable_stop_time.is_some());

        let out = oem.to_kvn().unwrap();
        assert!(out.contains("COMMENT This is a metadata comment"));
        assert!(out.contains("REF_FRAME_EPOCH"));
        assert!(out.contains("USEABLE_START_TIME"));
        assert!(out.contains("USEABLE_STOP_TIME"));
    }

    #[test]
    fn test_data_comments() {
        // Test for comments within the data section
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
COMMENT This is a data section comment
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COMMENT Another data comment
2023-01-01T00:01:00 1060 2120 3180 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let data = &oem.body.segment[0].data;
        assert_eq!(data.comment, vec!["This is a data section comment"]);
        assert_eq!(data.state_vector.len(), 2);

        let out = oem.to_kvn().unwrap();
        assert!(out.contains("COMMENT This is a data section comment"));
        // Note: Interleaved comments are not preserved on write, only header comments for the block.
    }

    #[test]
    fn test_meta_stop_required() {
        // A2.5.3 Item 23: META_STOP required
        let kvn = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        let ok = match &err {
            CcsdsNdmError::KvnParse(msg)
                if msg.contains("Unexpected token in META")
                    || msg.contains("Expected raw float data line for covariance matrix") =>
            {
                true
            }
            CcsdsNdmError::MissingField(_) => true,
            _ => false,
        };
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_optional_interpolation_fields() {
        // A2.5.3 Items 21–22: INTERPOLATION optional, INTERPOLATION_DEGREE conditional positive integer
        let kvn = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    INTERPOLATION = LAGRANGE
    INTERPOLATION_DEGREE = 5
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let meta = &oem.body.segment[0].metadata;
        assert_eq!(meta.interpolation.as_deref(), Some("LAGRANGE"));
        assert_eq!(meta.interpolation_degree.map(|v| v.get()), Some(5));

        let kvn_bad_degree = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    INTERPOLATION_DEGREE = 0
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    "#;
        let err = Oem::from_kvn(kvn_bad_degree).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("INTERPOLATION_DEGREE"))
        );
    }

    #[test]
    fn test_covariance_block_start_stop_and_optional_ref_frame() {
        // A2.5.3 Items 26–31: Covariance block optional; start/stop required; COV_REF_FRAME optional
        let kvn = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    COVARIANCE_START
    EPOCH = 2023-01-01T00:00:00
    1.0
    0.1 1.0
    0.1 0.1 1.0
    0.01 0.01 0.01 1.0
    0.01 0.01 0.01 0.1 1.0
    0.01 0.01 0.01 0.1 0.1 1.0
    COVARIANCE_STOP
    "#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let cov = &oem.body.segment[0].data.covariance_matrix[0];
        assert!(cov.cov_ref_frame.is_none());

        let kvn_with_ref = r#"CCSDS_OEM_VERS = 3.0
    CREATION_DATE = 2023-01-01T00:00:00
    ORIGINATOR = TEST
    META_START
    OBJECT_NAME = SAT1
    OBJECT_ID = 999
    CENTER_NAME = EARTH
    REF_FRAME = GCRF
    TIME_SYSTEM = UTC
    START_TIME = 2023-01-01T00:00:00
    STOP_TIME = 2023-01-02T00:00:00
    META_STOP
    2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
    COVARIANCE_START
    EPOCH = 2023-01-01T00:00:00
    COV_REF_FRAME = RTN
    1.0
    0.1 1.0
    0.1 0.1 1.0
    0.01 0.01 0.01 1.0
    0.01 0.01 0.01 0.1 1.0
    0.01 0.01 0.01 0.1 0.1 1.0
    COVARIANCE_STOP
    "#;
        let oem2 = Oem::from_kvn(kvn_with_ref).unwrap();
        let cov2 = &oem2.body.segment[0].data.covariance_matrix[0];
        assert_eq!(cov2.cov_ref_frame.as_deref(), Some("RTN"));
    }

    #[test]
    fn test_parse_oem_with_covariance() {
        // Simulates data/kvn/oem_with_cov.kvn
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 1996-11-04T17:22:31
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
COV_REF_FRAME = GCRF
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn).expect("Failed to parse OEM with covariance");
        let data = &oem.body.segment[0].data;
        assert_eq!(data.state_vector.len(), 1);
        assert_eq!(data.covariance_matrix.len(), 1);
        assert_eq!(data.covariance_matrix[0].cx_x.value, 1.0);
        assert_eq!(data.covariance_matrix[0].cz_z.value, 1.0); // 6th element
    }

    #[test]
    fn test_write_kvn() {
        // Parse then Write then Parse check
        let kvn_in = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-11-26T12:00:00
ORIGINATOR = RUST_TEST
META_START
OBJECT_NAME = TEST_SAT
OBJECT_ID = 12345
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
START_TIME = 2023-11-26T12:00:00
STOP_TIME = 2023-11-26T13:00:00
META_STOP
2023-11-26T12:00:00 6000.0 0.0 0.0 0.0 7.5 0.0
"#;
        let oem = Oem::from_kvn(kvn_in).unwrap();
        let kvn_out = oem.to_kvn().unwrap();

        let oem2 = Oem::from_kvn(&kvn_out).unwrap();
        assert_eq!(oem.header.originator, oem2.header.originator);
        assert_eq!(
            oem.body.segment[0].data.state_vector[0].epoch,
            oem2.body.segment[0].data.state_vector[0].epoch
        );
    }

    #[test]
    fn test_version_must_be_first() {
        // A2.5.3 Item 1: CCSDS_OEM_VERS shall be first
        let kvn = r#"CREATION_DATE = 2023-01-01T00:00:00
CCSDS_OEM_VERS = 3.0
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(msg) if msg.contains("CCSDS_OEM_VERS must be the first keyword"))
        );
    }

    #[test]
    fn test_missing_required_metadata_fields() {
        // A2.5.3 Items 11–16: Mandatory metadata keywords present
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "OBJECT_NAME"));
    }

    #[test]
    fn test_body_must_have_at_least_one_segment() {
        // A2.5.3 Item 8: At least one segment
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(k) if k.contains("OEM body must contain at least one segment"))
        );
    }

    #[test]
    fn test_segment_requires_meta_start_stop() {
        // A2.5.3 Items 9/10: Segment gated by META_START/META_STOP
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(_))
                | matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Expected META_START"))
        );
    }

    #[test]
    fn test_data_requires_at_least_one_state_vector() {
        // A2.5.3 Item 25: At least one state vector in data
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(k) if k.contains("must contain at least one state vector"))
        );
    }

    #[test]
    fn test_covariance_requires_epoch_and_21_values() {
        // A2.5.3 Items 28–31: Covariance block requires EPOCH and 21 values
        let kvn_missing_epoch = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let err1 = Oem::from_kvn(kvn_missing_epoch).unwrap_err();
        assert!(matches!(err1, CcsdsNdmError::KvnParse(msg) if msg.contains("Expected EPOCH")));

        let kvn_wrong_count = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1
COVARIANCE_STOP
"#;
        let err2 = Oem::from_kvn(kvn_wrong_count).unwrap_err();
        assert!(matches!(err2, CcsdsNdmError::KvnParse(msg) if msg.contains("requires 21 values")));
    }

    #[test]
    fn test_invalid_epoch_in_state_vector() {
        // A2.5.3 Item 17: Epoch format must be valid
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
bad-epoch 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::Epoch(_)) | matches!(err, CcsdsNdmError::KvnParse(_)));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 1: Mandatory Metadata Fields
    // XSD: oemMetadata defines mandatory fields without minOccurs="0"
    // =========================================================================

    #[test]
    fn test_xsd_missing_object_id() {
        // XSD: OBJECT_ID is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "OBJECT_ID"));
    }

    #[test]
    fn test_xsd_missing_center_name() {
        // XSD: CENTER_NAME is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "CENTER_NAME"));
    }

    #[test]
    fn test_xsd_missing_ref_frame() {
        // XSD: REF_FRAME is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "REF_FRAME"));
    }

    #[test]
    fn test_xsd_missing_time_system() {
        // XSD: TIME_SYSTEM is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "TIME_SYSTEM"));
    }

    #[test]
    fn test_xsd_missing_start_time() {
        // XSD: START_TIME is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "START_TIME"));
    }

    #[test]
    fn test_xsd_missing_stop_time() {
        // XSD: STOP_TIME is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "STOP_TIME"));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 2: Body/Segment Constraints
    // XSD: oemBody requires minOccurs="1" maxOccurs="unbounded" segments
    // =========================================================================

    #[test]
    fn test_xsd_body_min_one_segment() {
        // XSD: oemBody requires minOccurs="1" for segment
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(k) if k.contains("at least one segment"))
        );
    }

    #[test]
    fn test_xsd_body_multiple_segments() {
        // XSD: oemBody allows maxOccurs="unbounded" segments
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T01:00:00
STOP_TIME = 2023-01-01T02:00:00
META_STOP
2023-01-01T01:00:00 1100 2100 3100 1.1 2.1 3.1
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment.len(), 2);
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 1);
        assert_eq!(oem.body.segment[1].data.state_vector.len(), 1);
    }

    #[test]
    fn test_xsd_segment_structure() {
        // XSD: oemSegment must have metadata followed by data
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 5
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
2023-01-01T00:01:00 1060 2120 3180 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment.len(), 1);
        let seg = &oem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "SAT1");
        assert_eq!(seg.metadata.interpolation.as_deref(), Some("LAGRANGE"));
        assert_eq!(seg.metadata.interpolation_degree.map(|v| v.get()), Some(5));
        assert_eq!(seg.data.state_vector.len(), 2);
    }

    #[test]
    fn test_xsd_version_attribute_fixed() {
        // XSD: oemType has version attribute fixed="3.0"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.version, "3.0");
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 3: Data Section Tests
    // XSD: oemData requires stateVector minOccurs="1", accelerations optional
    // =========================================================================

    #[test]
    fn test_xsd_data_min_one_state_vector() {
        // XSD: oemData requires stateVector minOccurs="1"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::MissingField(k) if k.contains("at least one state vector"))
        );
    }

    #[test]
    fn test_xsd_data_multiple_state_vectors() {
        // XSD: oemData allows maxOccurs="unbounded" for stateVector
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
2023-01-01T00:01:00 1060 2120 3180 1.0 2.0 3.0
2023-01-01T00:02:00 1120 2240 3360 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 3);
    }

    #[test]
    fn test_xsd_state_vector_position_velocity_mandatory() {
        // XSD: stateVectorAccType has mandatory EPOCH, X, Y, Z, X_DOT, Y_DOT, Z_DOT
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000.123 2000.456 3000.789 1.111 2.222 3.333
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let sv = &oem.body.segment[0].data.state_vector[0];
        assert_eq!(sv.x.value, 1000.123);
        assert_eq!(sv.y.value, 2000.456);
        assert_eq!(sv.z.value, 3000.789);
        assert_eq!(sv.x_dot.value, 1.111);
        assert_eq!(sv.y_dot.value, 2.222);
        assert_eq!(sv.z_dot.value, 3.333);
    }

    #[test]
    fn test_xsd_state_vector_acceleration_optional() {
        // XSD: stateVectorAccType has optional X_DDOT, Y_DDOT, Z_DDOT (minOccurs="0")
        let kvn_without_acc = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_without_acc).unwrap();
        let sv = &oem.body.segment[0].data.state_vector[0];
        assert!(sv.x_ddot.is_none());
        assert!(sv.y_ddot.is_none());
        assert!(sv.z_ddot.is_none());
    }

    #[test]
    fn test_xsd_state_vector_with_acceleration() {
        // XSD: stateVectorAccType can include optional accelerations
        let kvn_with_acc = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0 0.001 0.002 0.003
"#;
        let oem = Oem::from_kvn(kvn_with_acc).unwrap();
        let sv = &oem.body.segment[0].data.state_vector[0];
        assert_eq!(sv.x_ddot.as_ref().map(|v| v.value), Some(0.001));
        assert_eq!(sv.y_ddot.as_ref().map(|v| v.value), Some(0.002));
        assert_eq!(sv.z_ddot.as_ref().map(|v| v.value), Some(0.003));
    }

    #[test]
    fn test_xsd_data_comments_unbounded() {
        // XSD: oemData allows COMMENT minOccurs="0" maxOccurs="unbounded"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
COMMENT First comment in data section
COMMENT Second comment in data section
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment[0].data.comment.len(), 2);
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 4: Covariance Matrix Tests
    // XSD: oemCovarianceMatrixType with 21 elements, optional COV_REF_FRAME
    // =========================================================================

    #[test]
    fn test_xsd_covariance_optional() {
        // XSD: oemData has covarianceMatrix minOccurs="0"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert!(oem.body.segment[0].data.covariance_matrix.is_empty());
    }

    #[test]
    fn test_xsd_covariance_multiple() {
        // XSD: oemData allows covarianceMatrix maxOccurs="unbounded"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
EPOCH = 2023-01-01T01:00:00
2.0
0.2 2.0
0.2 0.2 2.0
0.02 0.02 0.02 2.0
0.02 0.02 0.02 0.2 2.0
0.02 0.02 0.02 0.2 0.2 2.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment[0].data.covariance_matrix.len(), 2);
    }

    #[test]
    fn test_xsd_covariance_epoch_mandatory() {
        // XSD: oemCovarianceMatrixType requires EPOCH
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Expected EPOCH")));
    }

    #[test]
    fn test_xsd_covariance_21_values_required() {
        // XSD: covarianceMatrixElementsGroup requires all 21 covariance elements
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1
COVARIANCE_STOP
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("requires 21 values")));
    }

    #[test]
    fn test_xsd_covariance_cov_ref_frame_optional() {
        // XSD: oemCovarianceMatrixAbstractType has COV_REF_FRAME minOccurs="0"
        let kvn_without = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn_without).unwrap();
        assert!(oem.body.segment[0].data.covariance_matrix[0]
            .cov_ref_frame
            .is_none());

        let kvn_with = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
COV_REF_FRAME = RTN
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn_with).unwrap();
        assert_eq!(
            oem.body.segment[0].data.covariance_matrix[0]
                .cov_ref_frame
                .as_deref(),
            Some("RTN")
        );
    }

    #[test]
    fn test_xsd_covariance_all_21_elements() {
        // XSD: Full 6x6 lower triangular covariance matrix
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
2.0 3.0
4.0 5.0 6.0
7.0 8.0 9.0 10.0
11.0 12.0 13.0 14.0 15.0
16.0 17.0 18.0 19.0 20.0 21.0
COVARIANCE_STOP
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        let cov = &oem.body.segment[0].data.covariance_matrix[0];
        // Row 1
        assert_eq!(cov.cx_x.value, 1.0);
        // Row 2
        assert_eq!(cov.cy_x.value, 2.0);
        assert_eq!(cov.cy_y.value, 3.0);
        // Row 3
        assert_eq!(cov.cz_x.value, 4.0);
        assert_eq!(cov.cz_y.value, 5.0);
        assert_eq!(cov.cz_z.value, 6.0);
        // Row 4
        assert_eq!(cov.cx_dot_x.value, 7.0);
        assert_eq!(cov.cx_dot_y.value, 8.0);
        assert_eq!(cov.cx_dot_z.value, 9.0);
        assert_eq!(cov.cx_dot_x_dot.value, 10.0);
        // Row 5
        assert_eq!(cov.cy_dot_x.value, 11.0);
        assert_eq!(cov.cy_dot_y.value, 12.0);
        assert_eq!(cov.cy_dot_z.value, 13.0);
        assert_eq!(cov.cy_dot_x_dot.value, 14.0);
        assert_eq!(cov.cy_dot_y_dot.value, 15.0);
        // Row 6
        assert_eq!(cov.cz_dot_x.value, 16.0);
        assert_eq!(cov.cz_dot_y.value, 17.0);
        assert_eq!(cov.cz_dot_z.value, 18.0);
        assert_eq!(cov.cz_dot_x_dot.value, 19.0);
        assert_eq!(cov.cz_dot_y_dot.value, 20.0);
        assert_eq!(cov.cz_dot_z_dot.value, 21.0);
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 5: Sample File & Optional Field Tests
    // =========================================================================

    #[test]
    fn test_xsd_parse_sample_oem_g11() {
        // Parse official CCSDS sample file oem_g11.kvn
        let kvn = include_str!("../../../data/kvn/oem_g11.kvn");
        let oem = Oem::from_kvn(kvn).expect("Failed to parse oem_g11.kvn");
        assert_eq!(oem.version, "3.0");
        assert_eq!(oem.header.originator, "NASA/JPL");
        assert_eq!(oem.body.segment.len(), 2);
        // First segment
        assert_eq!(
            oem.body.segment[0].metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
        assert_eq!(oem.body.segment[0].metadata.object_id, "1996-062A");
        assert_eq!(oem.body.segment[0].metadata.center_name, "MARS BARYCENTER");
        assert_eq!(oem.body.segment[0].metadata.ref_frame, "EME2000");
        assert_eq!(oem.body.segment[0].metadata.time_system, "UTC");
        assert_eq!(
            oem.body.segment[0].metadata.interpolation.as_deref(),
            Some("HERMITE")
        );
        assert_eq!(
            oem.body.segment[0]
                .metadata
                .interpolation_degree
                .map(|v| v.get()),
            Some(7)
        );
        assert!(oem.body.segment[0].metadata.useable_start_time.is_some());
        assert!(oem.body.segment[0].metadata.useable_stop_time.is_some());
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 4);
        // Second segment
        assert_eq!(oem.body.segment[1].data.state_vector.len(), 4);
        assert!(!oem.body.segment[1].data.comment.is_empty());
    }

    #[test]
    fn test_xsd_metadata_optional_ref_frame_epoch() {
        // XSD: REF_FRAME_EPOCH has minOccurs="0"
        let kvn_without = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_without).unwrap();
        assert!(oem.body.segment[0].metadata.ref_frame_epoch.is_none());

        let kvn_with = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_with).unwrap();
        assert!(oem.body.segment[0].metadata.ref_frame_epoch.is_some());
    }

    #[test]
    fn test_xsd_metadata_optional_useable_times() {
        // XSD: USEABLE_START_TIME and USEABLE_STOP_TIME have minOccurs="0"
        let kvn_without = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_without).unwrap();
        assert!(oem.body.segment[0].metadata.useable_start_time.is_none());
        assert!(oem.body.segment[0].metadata.useable_stop_time.is_none());

        let kvn_with = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T01:00:00
USEABLE_STOP_TIME = 2023-01-01T23:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T01:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_with).unwrap();
        assert!(oem.body.segment[0].metadata.useable_start_time.is_some());
        assert!(oem.body.segment[0].metadata.useable_stop_time.is_some());
    }

    #[test]
    fn test_xsd_interpolation_degree_positive_integer() {
        // XSD: INTERPOLATION_DEGREE is xsd:positiveInteger (> 0)
        let kvn_valid = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn_valid).unwrap();
        assert_eq!(
            oem.body.segment[0]
                .metadata
                .interpolation_degree
                .map(|v| v.get()),
            Some(7)
        );

        // Zero is not a positive integer
        let kvn_zero = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
INTERPOLATION_DEGREE = 0
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let err = Oem::from_kvn(kvn_zero).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("INTERPOLATION_DEGREE"))
        );
    }

    #[test]
    fn test_xsd_metadata_comments_unbounded() {
        // XSD: oemMetadata COMMENT has minOccurs="0" maxOccurs="unbounded"
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT First metadata comment
COMMENT Second metadata comment
COMMENT Third metadata comment
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let oem = Oem::from_kvn(kvn).unwrap();
        assert_eq!(oem.body.segment[0].metadata.comment.len(), 3);
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 6: XML Parsing and Roundtrip Tests
    // =========================================================================

    #[test]
    fn test_xsd_parse_xml_oem_g14() {
        // Parse official CCSDS sample file oem_g14.xml
        let xml = include_str!("../../../data/xml/oem_g14.xml");
        let oem = Oem::from_xml(xml).expect("Failed to parse oem_g14.xml");
        assert_eq!(oem.version, "3.0");
        assert_eq!(oem.header.originator, "NASA/JPL");
        assert!(oem.header.message_id.is_some());
        assert_eq!(oem.body.segment.len(), 1);
        // Verify state vectors with optional accelerations
        let seg = &oem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert_eq!(seg.data.state_vector.len(), 4);
        // XML sample has accelerations
        assert!(seg.data.state_vector[0].x_ddot.is_some());
        // XML sample has covariance
        assert_eq!(seg.data.covariance_matrix.len(), 1);
        assert!(seg.data.covariance_matrix[0].cov_ref_frame.is_some());
    }

    #[test]
    fn test_xsd_kvn_roundtrip() {
        // Parse KVN -> Write KVN -> Parse KVN should produce same result
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = HERMITE
INTERPOLATION_DEGREE = 5
META_STOP
2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0 0.001 0.002 0.003
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
COV_REF_FRAME = RTN
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
"#;
        let oem1 = Oem::from_kvn(kvn).unwrap();
        let kvn_out = oem1.to_kvn().unwrap();
        let oem2 = Oem::from_kvn(&kvn_out).unwrap();

        assert_eq!(oem1.version, oem2.version);
        assert_eq!(oem1.header.originator, oem2.header.originator);
        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());

        let meta1 = &oem1.body.segment[0].metadata;
        let meta2 = &oem2.body.segment[0].metadata;
        assert_eq!(meta1.object_name, meta2.object_name);
        assert_eq!(meta1.interpolation, meta2.interpolation);
        assert_eq!(meta1.interpolation_degree, meta2.interpolation_degree);

        let data1 = &oem1.body.segment[0].data;
        let data2 = &oem2.body.segment[0].data;
        assert_eq!(data1.state_vector.len(), data2.state_vector.len());
        assert_eq!(data1.covariance_matrix.len(), data2.covariance_matrix.len());
    }

    #[test]
    fn test_xsd_xml_roundtrip() {
        // Parse XML -> Write XML -> Parse XML should produce same result
        let xml = include_str!("../../../data/xml/oem_g14.xml");
        let oem1 = Oem::from_xml(xml).unwrap();
        let xml_out = oem1.to_xml().unwrap();
        let oem2 = Oem::from_xml(&xml_out).unwrap();

        assert_eq!(oem1.version, oem2.version);
        assert_eq!(oem1.header.originator, oem2.header.originator);
        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());

        let seg1 = &oem1.body.segment[0];
        let seg2 = &oem2.body.segment[0];
        assert_eq!(seg1.metadata.object_name, seg2.metadata.object_name);
        assert_eq!(seg1.data.state_vector.len(), seg2.data.state_vector.len());
        assert_eq!(
            seg1.data.covariance_matrix.len(),
            seg2.data.covariance_matrix.len()
        );
    }

    #[test]
    fn test_xsd_kvn_sample_file_roundtrip() {
        // Parse sample KVN file and verify roundtrip
        let kvn = include_str!("../../../data/kvn/oem_g11.kvn");
        let oem1 = Oem::from_kvn(kvn).unwrap();
        let kvn_out = oem1.to_kvn().unwrap();
        let oem2 = Oem::from_kvn(&kvn_out).unwrap();

        assert_eq!(oem1.body.segment.len(), oem2.body.segment.len());
        for (seg1, seg2) in oem1.body.segment.iter().zip(oem2.body.segment.iter()) {
            assert_eq!(seg1.metadata.object_name, seg2.metadata.object_name);
            assert_eq!(seg1.data.state_vector.len(), seg2.data.state_vector.len());
        }
    }
}
