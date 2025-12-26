// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Contains Rust definitions for common structures
//! from `ndmxml-4.0.0-common-4.0.xsd` used by OEM.

use super::kvn::de::KvnLine;
use super::types::*;
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, ToKvn};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::str::FromStr;

/// Represents the `ndmHeader` complex type from the XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub creation_date: Epoch,
    pub originator: String,
}

impl ToKvn for NdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
    }
}

// impl FromKvnTokens for NdmHeader {
//     fn from_kvn_tokens<'a>(tokens: &mut impl Iterator<Item = Result<KvnLine<'a>>>) -> Result<Self> {
//         let mut comment = Vec::new();
//         let mut creation_date = None;
//         let mut originator = None;

//         for token in tokens {
//             match token? {
//                 KvnLine::Comment(c) => comment.push(c.to_string()),
//                 KvnLine::Pair { key, val, .. } => match key {
//                     "CREATION_DATE" => {
//                         creation_date = Some(Epoch::from_str(val)?);
//                     }
//                     "ORIGINATOR" => originator = Some(val.to_string()),
//                     _ => {
//                         return Err(CcsdsNdmError::KvnParse(format!(
//                             "Unexpected field in header: {}",
//                             key
//                         )));
//                     }
//                 },
//                 KvnLine::Empty => continue,
//                 _ => {
//                     return Err(CcsdsNdmError::KvnParse(
//                         "Unexpected token in header".to_string(),
//                     ));
//                 }
//             }
//         }

//         Ok(NdmHeader {
//             comment,
//             creation_date: creation_date.ok_or_else(|| {
//                 CcsdsNdmError::MissingField("CREATION_DATE is required".to_string())
//             })?,
//             originator: originator
//                 .ok_or_else(|| CcsdsNdmError::MissingField("ORIGINATOR is required".to_string()))?,
//         })
//     }
// }

/// Represents the `admHeader` complex type from the XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    pub creation_date: Epoch,
    pub originator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ToKvn for AdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(ref cls) = self.classification {
            writer.write_pair("CLASSIFICATION", cls);
        }
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(ref msg_id) = self.message_id {
            writer.write_pair("MESSAGE_ID", msg_id);
        }
    }
}

//// Represents the `odmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdmHeader {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    pub creation_date: Epoch,
    pub originator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ToKvn for OdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(ref cls) = self.classification {
            writer.write_pair("CLASSIFICATION", cls);
        }
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(ref msg_id) = self.message_id {
            writer.write_pair("MESSAGE_ID", msg_id);
        }
    }
}

impl FromKvnTokens for OdmHeader {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment = Vec::new();
        let mut classification = None;
        let mut creation_date = None;
        let mut originator = None;
        let mut message_id = None;

        while let Some(peeked) = tokens.peek() {
            // If the next token is an error, return it immediately
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }

            let token = peeked.as_ref().unwrap();
            match token {
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, .. } => {
                    // Check if this key belongs to the Header
                    match *key {
                        "CLASSIFICATION" | "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID" => {
                            // Valid header key, consume and parse
                            if let Some(Ok(KvnLine::Pair { key, val, .. })) = tokens.next() {
                                match key {
                                    "CLASSIFICATION" => classification = Some(val.to_string()),
                                    "CREATION_DATE" => creation_date = Some(Epoch::from_str(val)?),
                                    "ORIGINATOR" => originator = Some(val.to_string()),
                                    "MESSAGE_ID" => message_id = Some(val.to_string()),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        // If it's any other key (like OBJECT_NAME), it's not part of the header.
                        // Break and let the next parser handle it.
                        _ => break,
                    }
                }
                // Any other token (BlockStart, BlockEnd, Raw) signals end of header
                _ => break,
            }
        }

        Ok(OdmHeader {
            comment,
            classification,
            creation_date: creation_date.ok_or_else(|| {
                CcsdsNdmError::MissingField("CREATION_DATE is required".to_string())
            })?,
            originator: originator
                .ok_or_else(|| CcsdsNdmError::MissingField("ORIGINATOR is required".to_string()))?,
            message_id,
        })
    }
}

/// Represents the `spacecraftParametersType` from the XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpacecraftParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<f64>,
}

/// Represents the `odParametersType` from the XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_lastob_start: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_lastob_end: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_od_span: Option<DayInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_od_span: Option<DayInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_available: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_used: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_available: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_used: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub residuals_accepted: Option<Percentage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighted_rms: Option<f64>,
}

/// Represents the `stateVectorType` and `stateVectorAccType` from the XSD.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVectorAcc {
    pub epoch: Epoch,
    pub x: Position,
    pub y: Position,
    pub z: Position,
    pub x_dot: Velocity,
    pub y_dot: Velocity,
    pub z_dot: Velocity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_ddot: Option<Acc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_ddot: Option<Acc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_ddot: Option<Acc>,
}

impl ToKvn for StateVectorAcc {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = format!(
            "{} {:.14e} {:.14e} {:.14e} {:.14e} {:.14e} {:.14e}",
            self.epoch,
            self.x.value,
            self.y.value,
            self.z.value,
            self.x_dot.value,
            self.y_dot.value,
            self.z_dot.value
        );
        if let Some(acc) = &self.x_ddot {
            line.push_str(&format!(" {:.14e}", acc.value));
        }
        if let Some(acc) = &self.y_ddot {
            line.push_str(&format!(" {:.14e}", acc.value));
        }
        if let Some(acc) = &self.z_ddot {
            line.push_str(&format!(" {:.14e}", acc.value));
        }
        writer.write_line(line);
    }
}

impl FromKvnTokens for StateVectorAcc {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // State vectors are single lines in OEM (Raw), so we consume the next one
        if let Some(token) = tokens.next() {
            match token? {
                KvnLine::Raw(line) => crate::common::parse_state_vector_raw(line),
                KvnLine::Empty => Err(CcsdsNdmError::KvnParse(
                    "Unexpected empty line expecting state vector".into(),
                )),
                _ => Err(CcsdsNdmError::KvnParse(
                    "StateVector in KVN must be a raw line format".to_string(),
                )),
            }
        } else {
            Err(CcsdsNdmError::MissingField(
                "No state vector data found".to_string(),
            ))
        }
    }
}

/// Parses a raw OEM state vector line (KVN ephemeris section).
///
/// Expected tokens:
/// 7 tokens: EPOCH X Y Z X_DOT Y_DOT Z_DOT
/// 10 tokens: EPOCH X Y Z X_DOT Y_DOT Z_DOT X_DDOT Y_DDOT Z_DDOT
pub fn parse_state_vector_raw(line: &str) -> Result<StateVectorAcc> {
    let mut tokens = line.split_whitespace();

    // Parse epoch first (most likely to fail, fail fast)
    let epoch_str = tokens.next().ok_or_else(|| {
        CcsdsNdmError::KvnParse("State vector line is empty or missing EPOCH".to_string())
    })?;
    let epoch = Epoch::from_str(epoch_str)
        .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid epoch in state vector: {}", e)))?;

    // Helper to parse next f64
    let mut next_f64 = |field: &'static str| -> Result<f64> {
        tokens
            .next()
            .ok_or_else(|| CcsdsNdmError::KvnParse(format!("Missing field: {}", field)))?
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid {} value: {}", field, e)))
    };

    // Parse mandatory fields
    let x_val = next_f64("X")?;
    let y_val = next_f64("Y")?;
    let z_val = next_f64("Z")?;
    let x_dot_val = next_f64("X_DOT")?;
    let y_dot_val = next_f64("Y_DOT")?;
    let z_dot_val = next_f64("Z_DOT")?;

    // Drop the closure so we can use tokens directly
    drop(next_f64);

    // Check if acceleration exists
    let (x_ddot, y_ddot, z_ddot) = if let Some(x_ddot_str) = tokens.next() {
        let x_acc = x_ddot_str
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid X_DDOT: {}", e)))?;

        let y_acc = tokens
            .next()
            .ok_or_else(|| CcsdsNdmError::KvnParse("Missing Y_DDOT".to_string()))?
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid Y_DDOT: {}", e)))?;

        let z_acc = tokens
            .next()
            .ok_or_else(|| CcsdsNdmError::KvnParse("Missing Z_DDOT".to_string()))?
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid Z_DDOT: {}", e)))?;

        // Validate no extra tokens
        if tokens.next().is_some() {
            return Err(CcsdsNdmError::KvnParse(
                "State vector has extra tokens. Expected 7 or 10 tokens.".to_string(),
            ));
        }

        (
            Some(Acc {
                value: x_acc,
                units: Some(AccUnits::KmPerS2),
            }),
            Some(Acc {
                value: y_acc,
                units: Some(AccUnits::KmPerS2),
            }),
            Some(Acc {
                value: z_acc,
                units: Some(AccUnits::KmPerS2),
            }),
        )
    } else {
        (None, None, None)
    };

    Ok(StateVectorAcc {
        epoch,
        x: Position {
            value: x_val,
            units: Some(PositionUnits::Km),
        },
        y: Position {
            value: y_val,
            units: Some(PositionUnits::Km),
        },
        z: Position {
            value: z_val,
            units: Some(PositionUnits::Km),
        },
        x_dot: Velocity {
            value: x_dot_val,
            units: Some(VelocityUnits::KmPerS),
        },
        y_dot: Velocity {
            value: y_dot_val,
            units: Some(VelocityUnits::KmPerS),
        },
        z_dot: Velocity {
            value: z_dot_val,
            units: Some(VelocityUnits::KmPerS),
        },
        x_ddot,
        y_ddot,
        z_ddot,
    })
}

// Quaternion (components each in [-1, 1])
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quaternion {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub qc: f64,
}
impl Quaternion {
    pub fn new(q1: f64, q2: f64, q3: f64, qc: f64) -> Result<Self> {
        for (name, v) in [("Q1", q1), ("Q2", q2), ("Q3", q3), ("QC", qc)] {
            if v < -1.0 || v > 1.0 {
                return Err(CcsdsNdmError::Validation(format!(
                    "{} component out of range [-1,1]: {}",
                    name, v
                )));
            }
        }
        Ok(Self { q1, q2, q3, qc })
    }
}

// Quaternion derivative (dot components with units 1/s)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuaternionDot {
    pub q1_dot: QuaternionDotComponent,
    pub q2_dot: QuaternionDotComponent,
    pub q3_dot: QuaternionDotComponent,
    pub qc_dot: QuaternionDotComponent,
}

// Angular velocity triple (ANGVEL_X/Y/Z)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AngularVelocity {
    pub x: AngleRate,
    pub y: AngleRate,
    pub z: AngleRate,
}

// State vector (oem/opm common)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVector {
    pub epoch: Epoch,
    pub x: Position,
    pub y: Position,
    pub z: Position,
    pub x_dot: Velocity,
    pub y_dot: Velocity,
    pub z_dot: Velocity,
}

impl ToKvn for StateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("EPOCH", &self.epoch);
        writer.write_measure("X", &self.x);
        writer.write_measure("Y", &self.y);
        writer.write_measure("Z", &self.z);
        writer.write_measure("X_DOT", &self.x_dot);
        writer.write_measure("Y_DOT", &self.y_dot);
        writer.write_measure("Z_DOT", &self.z_dot);
    }
}

impl FromKvnTokens for StateVector {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut epoch: Option<Epoch> = None;
        let mut x: Option<Position> = None;
        let mut y: Option<Position> = None;
        let mut z: Option<Position> = None;
        let mut x_dot: Option<Velocity> = None;
        let mut y_dot: Option<Velocity> = None;
        let mut z_dot: Option<Velocity> = None;

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Comment(_) => {
                    // Comments are allowed but ignored at state vector level
                    tokens.next();
                }
                KvnLine::Pair { key, val, unit } => {
                    let key_str = *key;
                    // Only consume keys that belong to the state vector
                    match key_str {
                        "EPOCH" => {
                            let val = val.to_string();
                            tokens.next();
                            epoch = Some(Epoch::from_str(&val)?);
                        }
                        "X" => {
                            let v = Position::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            x = Some(v);
                        }
                        "Y" => {
                            let v = Position::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            y = Some(v);
                        }
                        "Z" => {
                            let v = Position::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            z = Some(v);
                        }
                        "X_DOT" => {
                            let v = Velocity::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            x_dot = Some(v);
                        }
                        "Y_DOT" => {
                            let v = Velocity::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            y_dot = Some(v);
                        }
                        "Z_DOT" => {
                            let v = Velocity::from_kvn(val, unit.as_deref())?;
                            tokens.next();
                            z_dot = Some(v);
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }

        Ok(StateVector {
            epoch: epoch
                .ok_or_else(|| CcsdsNdmError::MissingField("EPOCH is required".to_string()))?,
            x: x.ok_or_else(|| CcsdsNdmError::MissingField("X is required".to_string()))?,
            y: y.ok_or_else(|| CcsdsNdmError::MissingField("Y is required".to_string()))?,
            z: z.ok_or_else(|| CcsdsNdmError::MissingField("Z is required".to_string()))?,
            x_dot: x_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("X_DOT is required".to_string()))?,
            y_dot: y_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("Y_DOT is required".to_string()))?,
            z_dot: z_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("Z_DOT is required".to_string()))?,
        })
    }
}

/// Represents the `quaternionStateType` logical block in APM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub quaternion: Quaternion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quaternion_dot: Option<QuaternionDot>,
}

/// Represents the `eulerAngleStateType` logical block in APM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EulerAngleState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub euler_rot_seq: RotSeq,
    pub angle_1: Angle,
    pub angle_2: Angle,
    pub angle_3: Angle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_1_dot: Option<AngleRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_2_dot: Option<AngleRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_3_dot: Option<AngleRate>,
}

/// Represents the `angVelStateType` logical block in APM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AngVelState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub angvel_frame: AngVelFrameType,
    pub angvel_x: AngleRate,
    pub angvel_y: AngleRate,
    pub angvel_z: AngleRate,
}

/// Represents the `spinStateType` logical block in APM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpinState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub spin_alpha: Angle,
    pub spin_delta: Angle,
    pub spin_angle: Angle,
    pub spin_angle_vel: AngleRate,
    // Choice: either nutation group or momentum group (both optional at top-level)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation: Option<Angle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_per: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_phase: Option<Angle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum_alpha: Option<Angle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum_delta: Option<Angle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_vel: Option<AngleRate>,
}

/// Represents the `inertiaStateType` logical block in APM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InertiaState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub inertia_ref_frame: String,
    pub ixx: Moment,
    pub iyy: Moment,
    pub izz: Moment,
    pub ixy: Moment,
    pub ixz: Moment,
    pub iyz: Moment,
}

/// OPM covariance matrix block (opmCovarianceMatrixType).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OpmCovarianceMatrix {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_ref_frame: Option<String>,
    // 6 position covariances
    pub cx_x: PositionCovariance,
    pub cy_x: PositionCovariance,
    pub cy_y: PositionCovariance,
    pub cz_x: PositionCovariance,
    pub cz_y: PositionCovariance,
    pub cz_z: PositionCovariance,
    // cross pos/vel covariances
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

impl ToKvn for OpmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(ref frame) = self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", frame);
        }

        writer.write_pair("CX_X", &self.cx_x);
        writer.write_pair("CY_X", &self.cy_x);
        writer.write_pair("CY_Y", &self.cy_y);
        writer.write_pair("CZ_X", &self.cz_x);
        writer.write_pair("CZ_Y", &self.cz_y);
        writer.write_pair("CZ_Z", &self.cz_z);

        writer.write_pair("CX_DOT_X", &self.cx_dot_x);
        writer.write_pair("CX_DOT_Y", &self.cx_dot_y);
        writer.write_pair("CX_DOT_Z", &self.cx_dot_z);
        writer.write_pair("CX_DOT_X_DOT", &self.cx_dot_x_dot);

        writer.write_pair("CY_DOT_X", &self.cy_dot_x);
        writer.write_pair("CY_DOT_Y", &self.cy_dot_y);
        writer.write_pair("CY_DOT_Z", &self.cy_dot_z);
        writer.write_pair("CY_DOT_X_DOT", &self.cy_dot_x_dot);
        writer.write_pair("CY_DOT_Y_DOT", &self.cy_dot_y_dot);

        writer.write_pair("CZ_DOT_X", &self.cz_dot_x);
        writer.write_pair("CZ_DOT_Y", &self.cz_dot_y);
        writer.write_pair("CZ_DOT_Z", &self.cz_dot_z);
        writer.write_pair("CZ_DOT_X_DOT", &self.cz_dot_x_dot);
        writer.write_pair("CZ_DOT_Y_DOT", &self.cz_dot_y_dot);
        writer.write_pair("CZ_DOT_Z_DOT", &self.cz_dot_z_dot);
    }
}

impl FromKvnTokens for OpmCovarianceMatrix {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut comment: Vec<String> = Vec::new();
        let mut cov_ref_frame: Option<String> = None;

        macro_rules! req {
            ($name:ident : $ty:ty) => {
                let mut $name: Option<$ty> = None;
            };
        }

        req!(cx_x: PositionCovariance);
        req!(cy_x: PositionCovariance);
        req!(cy_y: PositionCovariance);
        req!(cz_x: PositionCovariance);
        req!(cz_y: PositionCovariance);
        req!(cz_z: PositionCovariance);

        req!(cx_dot_x: PositionVelocityCovariance);
        req!(cx_dot_y: PositionVelocityCovariance);
        req!(cx_dot_z: PositionVelocityCovariance);
        req!(cx_dot_x_dot: VelocityCovariance);

        req!(cy_dot_x: PositionVelocityCovariance);
        req!(cy_dot_y: PositionVelocityCovariance);
        req!(cy_dot_z: PositionVelocityCovariance);
        req!(cy_dot_x_dot: VelocityCovariance);
        req!(cy_dot_y_dot: VelocityCovariance);

        req!(cz_dot_x: PositionVelocityCovariance);
        req!(cz_dot_y: PositionVelocityCovariance);
        req!(cz_dot_z: PositionVelocityCovariance);
        req!(cz_dot_x_dot: VelocityCovariance);
        req!(cz_dot_y_dot: VelocityCovariance);
        req!(cz_dot_z_dot: VelocityCovariance);

        while let Some(peeked) = tokens.peek() {
            if peeked.is_err() {
                return Err(tokens.next().unwrap().unwrap_err());
            }
            match peeked.as_ref().unwrap() {
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        comment.push(c.to_string());
                    }
                }
                KvnLine::Pair { key, val, unit } => {
                    let k = *key;
                    let u = unit.as_deref();
                    let consumed = match k {
                        "COV_REF_FRAME" => {
                            cov_ref_frame = Some(val.to_string());
                            true
                        }
                        "CX_X" => {
                            cx_x = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_X" => {
                            cy_x = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_Y" => {
                            cy_y = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_X" => {
                            cz_x = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_Y" => {
                            cz_y = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_Z" => {
                            cz_z = Some(PositionCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CX_DOT_X" => {
                            cx_dot_x = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CX_DOT_Y" => {
                            cx_dot_y = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CX_DOT_Z" => {
                            cx_dot_z = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CX_DOT_X_DOT" => {
                            cx_dot_x_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_DOT_X" => {
                            cy_dot_x = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_DOT_Y" => {
                            cy_dot_y = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_DOT_Z" => {
                            cy_dot_z = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_DOT_X_DOT" => {
                            cy_dot_x_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CY_DOT_Y_DOT" => {
                            cy_dot_y_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_X" => {
                            cz_dot_x = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_Y" => {
                            cz_dot_y = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_Z" => {
                            cz_dot_z = Some(PositionVelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_X_DOT" => {
                            cz_dot_x_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_Y_DOT" => {
                            cz_dot_y_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        "CZ_DOT_Z_DOT" => {
                            cz_dot_z_dot = Some(VelocityCovariance::from_kvn(val, u)?);
                            true
                        }
                        _ => false,
                    };
                    if consumed {
                        tokens.next();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(OpmCovarianceMatrix {
            comment,
            cov_ref_frame,
            cx_x: cx_x.ok_or_else(|| CcsdsNdmError::MissingField("CX_X is required".into()))?,
            cy_x: cy_x.ok_or_else(|| CcsdsNdmError::MissingField("CY_X is required".into()))?,
            cy_y: cy_y.ok_or_else(|| CcsdsNdmError::MissingField("CY_Y is required".into()))?,
            cz_x: cz_x.ok_or_else(|| CcsdsNdmError::MissingField("CZ_X is required".into()))?,
            cz_y: cz_y.ok_or_else(|| CcsdsNdmError::MissingField("CZ_Y is required".into()))?,
            cz_z: cz_z.ok_or_else(|| CcsdsNdmError::MissingField("CZ_Z is required".into()))?,
            cx_dot_x: cx_dot_x
                .ok_or_else(|| CcsdsNdmError::MissingField("CX_DOT_X is required".into()))?,
            cx_dot_y: cx_dot_y
                .ok_or_else(|| CcsdsNdmError::MissingField("CX_DOT_Y is required".into()))?,
            cx_dot_z: cx_dot_z
                .ok_or_else(|| CcsdsNdmError::MissingField("CX_DOT_Z is required".into()))?,
            cx_dot_x_dot: cx_dot_x_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CX_DOT_X_DOT is required".into()))?,
            cy_dot_x: cy_dot_x
                .ok_or_else(|| CcsdsNdmError::MissingField("CY_DOT_X is required".into()))?,
            cy_dot_y: cy_dot_y
                .ok_or_else(|| CcsdsNdmError::MissingField("CY_DOT_Y is required".into()))?,
            cy_dot_z: cy_dot_z
                .ok_or_else(|| CcsdsNdmError::MissingField("CY_DOT_Z is required".into()))?,
            cy_dot_x_dot: cy_dot_x_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CY_DOT_X_DOT is required".into()))?,
            cy_dot_y_dot: cy_dot_y_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CY_DOT_Y_DOT is required".into()))?,
            cz_dot_x: cz_dot_x
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_X is required".into()))?,
            cz_dot_y: cz_dot_y
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_Y is required".into()))?,
            cz_dot_z: cz_dot_z
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_Z is required".into()))?,
            cz_dot_x_dot: cz_dot_x_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_X_DOT is required".into()))?,
            cz_dot_y_dot: cz_dot_y_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_Y_DOT is required".into()))?,
            cz_dot_z_dot: cz_dot_z_dot
                .ok_or_else(|| CcsdsNdmError::MissingField("CZ_DOT_Z_DOT is required".into()))?,
        })
    }
}

/// Atmospheric reentry parameters (atmosphericReentryParametersType, RDM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AtmosphericReentryParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub orbit_lifetime: DayIntervalRequired,
    pub reentry_altitude: PositionRequired,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_window_start: Option<DayIntervalRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_window_end: Option<DayIntervalRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_reentry_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_window_start: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_window_end: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_confidence_level: Option<PercentageRequired>,
}

/// Ground impact parameters (groundImpactParametersType, RDM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GroundImpactParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_impact: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_burn_up: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_break_up: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_land_impact: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_casualty: Option<Probability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_window_start: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_window_end: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_ref_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_alt: Option<AltitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_confidence: Option<PercentageRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_start_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_start_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_stop_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_stop_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_cross_track: Option<Distance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_confidence: Option<PercentageRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_start_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_start_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_stop_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_stop_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_cross_track: Option<Distance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_confidence: Option<PercentageRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_start_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_start_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_stop_lon: Option<LongitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_stop_lat: Option<LatitudeRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_cross_track: Option<Distance>,
}

/// RDM spacecraft parameters (rdmSpacecraftParametersType).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmSpacecraftParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hazardous_substances: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ballistic_coeff: Option<BallisticCoeffRequired>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrust_acceleration: Option<Ms2Required>,
}
