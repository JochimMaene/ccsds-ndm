// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Contains Rust definitions for common structures
//! from `ndmxml-4.0.0-common-4.0.xsd` used by OEM.

use super::types::*;
use crate::error::Result;
use crate::kvn::ser::KvnWriter;
use crate::traits::ToKvn;
use serde::{Deserialize, Serialize};

/// Represents the `ndmHeader` complex type from the XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NdmHeader {
    /// User-defined comments.
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 505.0-B-3, Section 3.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// File creation date/time in UTC.
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 505.0-B-3, Section 3.2.
    pub creation_date: Epoch,
    /// Creating agency or operator.
    ///
    /// **Examples**: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
    ///
    /// **CCSDS Reference**: 505.0-B-3, Section 3.2.
    #[builder(into)]
    pub originator: String,
}

impl ToKvn for NdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
    }
}

impl crate::traits::Validate for NdmHeader {
    fn validate(&self) -> Result<()> {
        if self.originator.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "NDM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.creation_date.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "NDM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdmHeader {
    /// User-defined comments. (See 7.8 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// User-defined free-text message classification/caveats of this ADM. It is recommended
    /// that selected values be pre-coordinated between exchanging entities by mutual agreement.
    ///
    /// **Examples**: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// File creation date/time in UTC. (For format specification, see 6.8.9.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.2.
    pub creation_date: Epoch,
    /// Creating agency or operator. Select from the accepted set of values indicated in annex B,
    /// subsection B1 from the ‘Abbreviation’ column (when present), or the ‘Name’ column when an
    /// Abbreviation column is not populated. If desired organization is not listed there, follow
    /// procedures to request that originator be added to SANA registry.
    ///
    /// **Examples**: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.2.
    #[builder(into)]
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator. The format and content of
    /// the message identifier value are at the discretion of the originator.
    ///
    /// **Examples**: APM_201113719185, ABC-12_34
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_id: Option<String>,
}

impl ToKvn for AdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(ref cls) = self.classification {
            writer.write_pair("CLASSIFICATION", cls);
        }
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(ref msg_id) = self.message_id {
            writer.write_pair("MESSAGE_ID", msg_id);
        }
    }
}

impl crate::traits::Validate for AdmHeader {
    fn validate(&self) -> Result<()> {
        if self.originator.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "ADM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.creation_date.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "ADM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

/// Represents the `odmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdmHeader {
    /// Comments (allowed in the ODM Header only immediately after the ODM version number).
    /// (See 7.8 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// User-defined free-text message classification/caveats of this ODM. It is recommended
    /// that selected values be pre-coordinated between exchanging entities by mutual agreement.
    ///
    /// **Examples**: SBU, ‘Operator-proprietary data; secondary distribution not permitted’
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// File creation date/time in UTC. (For format specification, see 7.5.10.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    pub creation_date: Epoch,
    /// Creating agency or operator. Select from the accepted set of values indicated in annex B,
    /// subsection B1 from the ‘Abbreviation’ column (when present), or the ‘Name’ column when an
    /// Abbreviation column is not populated. If desired organization is not listed there, follow
    /// procedures to request that originator be added to SANA registry.
    ///
    /// **Examples**: CNES, ESOC, GSFC, GSOC, JPL, JAXA, INTELSAT, USAF, INMARSAT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[builder(into)]
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator. The format and content of
    /// the message identifier value are at the discretion of the originator.
    ///
    /// **Examples**: OPM_201113719185, ABC-12_34
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_id: Option<String>,
}

impl ToKvn for OdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(ref cls) = self.classification {
            writer.write_pair("CLASSIFICATION", cls);
        }
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(ref msg_id) = self.message_id {
            writer.write_pair("MESSAGE_ID", msg_id);
        }
    }
}

impl crate::traits::Validate for OdmHeader {
    fn validate(&self) -> Result<()> {
        if self.originator.trim().is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "ODM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.creation_date.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "ODM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

/// Spacecraft Parameters (if maneuver is specified, then mass must be provided).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpacecraftParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft mass.
    ///
    /// **Examples**: 1850.2, 3352.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Mass>,
    /// Solar Radiation Pressure Area (AR).
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_area: Option<Area>,
    /// Solar Radiation Pressure Coefficient (CR).
    ///
    /// **Examples**: 1, 1.34
    ///
    /// **Units**: n/a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<NonNegativeDouble>,
    /// Drag Area (AD).
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_area: Option<Area>,
    /// Drag Coefficient (CD).
    ///
    /// **Examples**: 2, 2.1
    ///
    /// **Units**: n/a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<NonNegativeDouble>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdParameters {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,

    /// The start of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_END).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub time_lastob_start: Option<Epoch>,

    /// The end of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_START).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub time_lastob_end: Option<Epoch>,

    /// The recommended OD time span calculated for the object.
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: days
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub recommended_od_span: Option<DayInterval>,

    /// Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
    /// time span used for the OD of the object. (See annex E for definition.)
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: days
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub actual_od_span: Option<DayInterval>,

    /// The total number of observations available for orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub obs_available: Option<PositiveInteger>,

    /// The number of observations used in the orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub obs_used: Option<PositiveInteger>,

    /// The total number of tracks available for orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub tracks_available: Option<PositiveInteger>,

    /// The number of tracks used in the orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub tracks_used: Option<PositiveInteger>,

    /// The percentage of residuals accepted during orbit determination.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub residuals_accepted: Option<Percentage>,

    /// The weighted root mean square (RMS) of the residuals.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2 / 508.1-B-1, Section 3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable_value"
    )]
    pub weighted_rms: Option<NonNegativeDouble>,
}

/// State Vector Components in the Specified Coordinate System.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVectorAcc {
    /// Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub epoch: Epoch,

    /// Position vector X-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub x: Position,

    /// Position vector Y-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub y: Position,

    /// Position vector Z-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub z: Position,

    /// Velocity vector X-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub x_dot: Velocity,

    /// Velocity vector Y-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub y_dot: Velocity,

    /// Velocity vector Z-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    pub z_dot: Velocity,

    /// Acceleration vector X-component.
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_ddot: Option<Acc>,

    /// Acceleration vector Y-component.
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_ddot: Option<Acc>,

    /// Acceleration vector Z-component.
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_ddot: Option<Acc>,
}

impl ToKvn for StateVectorAcc {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut buffer = zmij::Buffer::new();
        let mut line_buf = [0u8; 256];
        let mut cursor = 0;

        macro_rules! append {
            ($s:expr) => {
                let bytes = $s.as_bytes();
                line_buf[cursor..cursor + bytes.len()].copy_from_slice(bytes);
                cursor += bytes.len();
            };
        }

        append!(self.epoch.as_str());
        append!(" ");
        append!(buffer.format_finite(self.x.value));
        append!(" ");
        append!(buffer.format_finite(self.y.value));
        append!(" ");
        append!(buffer.format_finite(self.z.value));
        append!(" ");
        append!(buffer.format_finite(self.x_dot.value));
        append!(" ");
        append!(buffer.format_finite(self.y_dot.value));
        append!(" ");
        append!(buffer.format_finite(self.z_dot.value));

        if let Some(acc) = &self.x_ddot {
            append!(" ");
            append!(buffer.format_finite(acc.value));
        }
        if let Some(acc) = &self.y_ddot {
            append!(" ");
            append!(buffer.format_finite(acc.value));
        }
        if let Some(acc) = &self.z_ddot {
            append!(" ");
            append!(buffer.format_finite(acc.value));
        }

        // We only append valid UTF-8 fragments (epoch, float digits, spaces)
        let line = std::str::from_utf8(&line_buf[..cursor])
            .expect("Formatted KVN line must be valid UTF-8");
        writer.write_line(line);
    }
}

// Quaternion (components each in [-1, 1])
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Quaternion {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub qc: f64,
}
impl Quaternion {
    pub fn new(q1: f64, q2: f64, q3: f64, qc: f64) -> crate::error::Result<Self> {
        for (name, v) in [("Q1", q1), ("Q2", q2), ("Q3", q3), ("QC", qc)] {
            if !(-1.0..=1.0).contains(&v) {
                return Err(crate::error::ValidationError::OutOfRange {
                    name: name.into(),
                    value: v.to_string(),
                    expected: "[-1, 1]".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(Self { q1, q2, q3, qc })
    }
}

impl crate::traits::Validate for Quaternion {
    fn validate(&self) -> Result<()> {
        let sum_sq = self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3 + self.qc * self.qc;
        if !(0.999..=1.001).contains(&sum_sq) {
            return Err(crate::error::ValidationError::Generic {
                message: format!("Quaternion not normalized: sum of squares = {}", sum_sq).into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

// Quaternion derivative (dot components with units 1/s)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionDot {
    pub q1_dot: QuaternionDotComponent,
    pub q2_dot: QuaternionDotComponent,
    pub q3_dot: QuaternionDotComponent,
    pub qc_dot: QuaternionDotComponent,
}

// Angular velocity triple (ANGVEL_X/Y/Z)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AngularVelocity {
    pub x: AngleRate,
    pub y: AngleRate,
    pub z: AngleRate,
}

/// State Vector Components in the Specified Coordinate System.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVector {
    /// Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of state vector & optional Keplerian elements (see 7.5.10 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub epoch: Epoch,
    /// Position vector X-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x: Position,
    /// Position vector Y-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y: Position,
    /// Position vector Z-component.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub z: Position,
    /// Velocity vector X-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x_dot: Velocity,
    /// Velocity vector Y-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y_dot: Velocity,
    /// Velocity vector Z-component.
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub z_dot: Velocity,
}

impl ToKvn for StateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        writer.write_measure("X", &self.x);
        writer.write_measure("Y", &self.y);
        writer.write_measure("Z", &self.z);
        writer.write_measure("X_DOT", &self.x_dot);
        writer.write_measure("Y_DOT", &self.y_dot);
        writer.write_measure("Z_DOT", &self.z_dot);
    }
}

impl crate::traits::Validate for StateVector {
    fn validate(&self) -> Result<()> {
        if self.epoch.is_empty() {
            return Err(crate::error::ValidationError::MissingRequiredField {
                block: "State Vector".into(),
                field: "EPOCH".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

/// Attitude quaternion.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Quaternion components Q1, Q2, Q3, QC.
    ///
    /// **Units**: dimensionless
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "quaternion")]
    pub quaternion: Quaternion,
    /// Time derivatives of quaternion components Q1, Q2, Q3, QC.
    ///
    /// **Units**: 1/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "quaternionDot"
    )]
    pub quaternion_dot: Option<QuaternionDot>,
}

/// Euler angle elements.
///
/// All mandatory elements of the logical block are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EulerAngleState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub euler_rot_seq: RotSeq,
    /// Angle of the first rotation.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angle_1: Angle,
    /// Angle of the second rotation.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angle_2: Angle,
    /// Angle of the third rotation.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angle_3: Angle,
    /// Time derivative of angle of the first rotation.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_1_dot: Option<AngleRate>,
    /// Time derivative of angle of the second rotation.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_2_dot: Option<AngleRate>,
    /// Time derivative of angle of the third rotation.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_3_dot: Option<AngleRate>,
}

/// Angular velocity vector.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AngVelState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Reference frame in which the components of the angular velocity vector are given. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angvel_frame: AngVelFrameType,
    /// Component of the angular velocity vector on the X axis.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angvel_x: AngleRate,
    /// Component of the angular velocity vector on the Y axis.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angvel_y: AngleRate,
    /// Component of the angular velocity vector on the Z axis.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub angvel_z: AngleRate,
}

/// Spin block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpinState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Right ascension of spin axis vector in frame A.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub spin_alpha: Angle,
    /// Declination of the spin axis vector in frame A.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub spin_delta: Angle,
    /// Phase of the satellite about the spin axis.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub spin_angle: Angle,
    /// Angular velocity of satellite around spin axis.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub spin_angle_vel: AngleRate,
    /// Nutation angle of spin axis.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation: Option<Angle>,
    /// Body nutation period of the spin axis.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_per: Option<Duration>,
    /// Inertial nutation phase.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_phase: Option<Angle>,
    /// Right ascension of angular momentum vector in frame A.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum_alpha: Option<Angle>,
    /// Declination of angular momentum vector in frame A.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub momentum_delta: Option<Angle>,
    /// Angular velocity of spin vector around the angular momentum vector.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nutation_vel: Option<AngleRate>,
}

/// Inertia block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InertiaState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Coordinate system for the inertia tensor. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[builder(into)]
    pub inertia_ref_frame: String,
    /// Moment of Inertia about the X-axis.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub ixx: Moment,
    /// Moment of Inertia about the Y-axis.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub iyy: Moment,
    /// Moment of Inertia about the Z-axis.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub izz: Moment,
    /// Inertia Cross Product of the X and Y axes.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub ixy: Moment,
    /// Inertia Cross Product of the X and Z axes.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub ixz: Moment,
    /// Inertia Cross Product of the Y and Z axes.
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub iyz: Moment,
}

/// Maneuver Parameters block.
///
/// All mandatory elements are to be provided if the block is present.
/// (See annex F for conventions and further detail.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AttManeuverState {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of start of maneuver. (For format specification, see 6.8.9.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_epoch_start: Epoch,
    /// Maneuver duration.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_duration: Duration,
    /// Coordinate system for the torque vector. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_ref_frame: String,
    /// 1st component of the torque vector.
    ///
    /// **Units**: N*m
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_tor_x: Torque,
    /// 2nd component of the torque vector.
    ///
    /// **Units**: N*m
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_tor_y: Torque,
    /// 3rd component of the torque vector.
    ///
    /// **Units**: N*m
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub man_tor_z: Torque,
    /// Mass change during maneuver (value is <= 0).
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_delta_mass: Option<DeltaMassZ>,
}

impl ToKvn for AttManeuverState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_EPOCH_START", self.man_epoch_start);
        writer.write_measure("MAN_DURATION", &self.man_duration.to_unit_value());
        writer.write_pair("MAN_REF_FRAME", &self.man_ref_frame);
        writer.write_measure("MAN_TOR_X", &self.man_tor_x);
        writer.write_measure("MAN_TOR_Y", &self.man_tor_y);
        writer.write_measure("MAN_TOR_Z", &self.man_tor_z);
        if let Some(m) = &self.man_delta_mass {
            writer.write_measure("MAN_DELTA_MASS", &m.to_unit_value());
        }
    }
}

//----------------------------------------------------------------------
// AEM Specific Types
//----------------------------------------------------------------------

/// Represents the `attitudeStateType` choice in AEM.
///
/// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AemAttitudeState {
    /// Epoch, Q1, Q2, Q3, QC.
    #[serde(rename = "quaternionEphemeris")]
    QuaternionEphemeris(QuaternionEphemeris),
    /// Epoch, Q1, Q2, Q3, QC, Q1_DOT, Q2_DOT, Q3_DOT, QC_DOT.
    #[serde(rename = "quaternionDerivative")]
    QuaternionDerivative(QuaternionDerivative),
    /// Epoch, Q1, Q2, Q3, QC, ANGVEL_X, ANGVEL_Y, ANGVEL_Z.
    #[serde(rename = "quaternionAngVel")]
    QuaternionAngVel(QuaternionAngVel),
    /// Epoch, ANGLE_1, ANGLE_2, ANGLE_3.
    #[serde(rename = "eulerAngle")]
    EulerAngle(EulerAngle),
    /// Epoch, ANGLE_1, ANGLE_2, ANGLE_3, ANGLE_1_DOT, ANGLE_2_DOT, ANGLE_3_DOT.
    #[serde(rename = "eulerAngleDerivative")]
    EulerAngleDerivative(EulerAngleDerivative),
    /// Epoch, ANGLE_1, ANGLE_2, ANGLE_3, ANGVEL_X, ANGVEL_Y, ANGVEL_Z.
    #[serde(rename = "eulerAngleAngVel")]
    EulerAngleAngVel(EulerAngleAngVel),
    /// Epoch, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL.
    #[serde(rename = "spin")]
    Spin(Spin),
    /// Epoch, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL, NUTATION, NUTATION_PER,
    /// NUTATION_PHASE.
    #[serde(rename = "spinNutation")]
    SpinNutation(SpinNutation),
    /// Epoch, SPIN_ALPHA, SPIN_DELTA, SPIN_ANGLE, SPIN_ANGLE_VEL, MOMENTUM_ALPHA, MOMENTUM_DELTA,
    /// NUTATION_VEL.
    #[serde(rename = "spinNutationMom")]
    SpinNutationMom(SpinNutationMom),
}

impl ToKvn for AemAttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        match self {
            Self::QuaternionEphemeris(v) => v.write_kvn(writer),
            Self::QuaternionDerivative(v) => v.write_kvn(writer),
            Self::QuaternionAngVel(v) => v.write_kvn(writer),
            Self::EulerAngle(v) => v.write_kvn(writer),
            Self::EulerAngleDerivative(v) => v.write_kvn(writer),
            Self::EulerAngleAngVel(v) => v.write_kvn(writer),
            Self::Spin(v) => v.write_kvn(writer),
            Self::SpinNutation(v) => v.write_kvn(writer),
            Self::SpinNutationMom(v) => v.write_kvn(writer),
        }
    }
}

/// AEM Attitude Ephemeris Data Line: Quaternion.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionEphemeris {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Quaternion components Q1, Q2, Q3, QC.
    #[serde(rename = "quaternion")]
    pub quaternion: Quaternion,
}

impl ToKvn for QuaternionEphemeris {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {}",
            self.quaternion.q1, self.quaternion.q2, self.quaternion.q3, self.quaternion.qc
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: Quaternion/Derivative.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionDerivative {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Quaternion components Q1, Q2, Q3, QC.
    #[serde(rename = "quaternion")]
    pub quaternion: Quaternion,
    /// Quaternion derivatives Q1_DOT, Q2_DOT, Q3_DOT, QC_DOT.
    #[serde(rename = "quaternionDot")]
    pub quaternion_dot: QuaternionDot,
}

impl ToKvn for QuaternionDerivative {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {}",
            self.quaternion.q1, self.quaternion.q2, self.quaternion.q3, self.quaternion.qc
        ));
        line.push_str(&format!(
            " {} {} {} {}",
            self.quaternion_dot.q1_dot.value,
            self.quaternion_dot.q2_dot.value,
            self.quaternion_dot.q3_dot.value,
            self.quaternion_dot.qc_dot.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: Quaternion/AngVel.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuaternionAngVel {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Quaternion components Q1, Q2, Q3, QC.
    #[serde(rename = "quaternion")]
    pub quaternion: Quaternion,
    /// Angular velocity components ANGVEL_X, ANGVEL_Y, ANGVEL_Z.
    #[serde(rename = "angVel")]
    pub ang_vel: AngVel,
}

impl ToKvn for QuaternionAngVel {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {}",
            self.quaternion.q1, self.quaternion.q2, self.quaternion.q3, self.quaternion.qc
        ));
        line.push_str(&format!(
            " {} {} {}",
            self.ang_vel.angvel_x.value, self.ang_vel.angvel_y.value, self.ang_vel.angvel_z.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: EulerAngle.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EulerAngle {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Angle of the first rotation.
    pub angle_1: Angle,
    /// Angle of the second rotation.
    pub angle_2: Angle,
    /// Angle of the third rotation.
    pub angle_3: Angle,
}

impl ToKvn for EulerAngle {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {}",
            self.angle_1.value, self.angle_2.value, self.angle_3.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: EulerAngle/Derivative.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EulerAngleDerivative {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Angle of the first rotation.
    pub angle_1: Angle,
    /// Angle of the second rotation.
    pub angle_2: Angle,
    /// Angle of the third rotation.
    pub angle_3: Angle,
    /// Time derivative of angle of the first rotation.
    pub angle_1_dot: AngleRate,
    /// Time derivative of angle of the second rotation.
    pub angle_2_dot: AngleRate,
    /// Time derivative of angle of the third rotation.
    pub angle_3_dot: AngleRate,
}

impl ToKvn for EulerAngleDerivative {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {}",
            self.angle_1.value, self.angle_2.value, self.angle_3.value
        ));
        line.push_str(&format!(
            " {} {} {}",
            self.angle_1_dot.value, self.angle_2_dot.value, self.angle_3_dot.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: EulerAngle/AngVel.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EulerAngleAngVel {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Angle of the first rotation.
    pub angle_1: Angle,
    /// Angle of the second rotation.
    pub angle_2: Angle,
    /// Angle of the third rotation.
    pub angle_3: Angle,
    /// Angular velocity component X.
    #[serde(rename = "ANGVEL_X")]
    pub angvel_x: AngleRate,
    /// Angular velocity component Y.
    #[serde(rename = "ANGVEL_Y")]
    pub angvel_y: AngleRate,
    /// Angular velocity component Z.
    #[serde(rename = "ANGVEL_Z")]
    pub angvel_z: AngleRate,
}

impl ToKvn for EulerAngleAngVel {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {}",
            self.angle_1.value, self.angle_2.value, self.angle_3.value
        ));
        line.push_str(&format!(
            " {} {} {}",
            self.angvel_x.value, self.angvel_y.value, self.angvel_z.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: Spin.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Spin {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Right ascension of spin axis vector in frame A.
    pub spin_alpha: Angle,
    /// Declination of the spin axis vector in frame A.
    pub spin_delta: Angle,
    /// Phase of the satellite about the spin axis.
    pub spin_angle: Angle,
    /// Angular velocity of satellite around spin axis.
    pub spin_angle_vel: AngleRate,
}

impl ToKvn for Spin {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {}",
            self.spin_alpha.value,
            self.spin_delta.value,
            self.spin_angle.value,
            self.spin_angle_vel.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: Spin/Nutation.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpinNutation {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Right ascension of spin axis vector in frame A.
    pub spin_alpha: Angle,
    /// Declination of the spin axis vector in frame A.
    pub spin_delta: Angle,
    /// Phase of the satellite about the spin axis.
    pub spin_angle: Angle,
    /// Angular velocity of satellite around spin axis.
    pub spin_angle_vel: AngleRate,
    /// Nutation angle of spin axis.
    pub nutation: Angle,
    /// Body nutation period of the spin axis.
    pub nutation_per: Duration,
    /// Inertial nutation phase.
    pub nutation_phase: Angle,
}

impl ToKvn for SpinNutation {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {} {} {} {}",
            self.spin_alpha.value,
            self.spin_delta.value,
            self.spin_angle.value,
            self.spin_angle_vel.value,
            self.nutation.value,
            self.nutation_per.value,
            self.nutation_phase.value
        ));
        writer.write_line(&line);
    }
}

/// AEM Attitude Ephemeris Data Line: Spin/Nutation_Mom.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpinNutationMom {
    /// Epoch of the attitude state.
    pub epoch: Epoch,
    /// Right ascension of spin axis vector in frame A.
    pub spin_alpha: Angle,
    /// Declination of the spin axis vector in frame A.
    pub spin_delta: Angle,
    /// Phase of the satellite about the spin axis.
    pub spin_angle: Angle,
    /// Angular velocity of satellite around spin axis.
    pub spin_angle_vel: AngleRate,
    /// Right ascension of angular momentum vector in frame A.
    pub momentum_alpha: Angle,
    /// Declination of angular momentum vector in frame A.
    pub momentum_delta: Angle,
    /// Angular velocity of spin vector around the angular momentum vector.
    pub nutation_vel: AngleRate,
}

impl ToKvn for SpinNutationMom {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        let mut line = self.epoch.to_string();
        line.push_str(&format!(
            " {} {} {} {} {} {} {}",
            self.spin_alpha.value,
            self.spin_delta.value,
            self.spin_angle.value,
            self.spin_angle_vel.value,
            self.momentum_alpha.value,
            self.momentum_delta.value,
            self.nutation_vel.value
        ));
        writer.write_line(&line);
    }
}

/// Represents the `angVelType` from XSD.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AngVel {
    pub angvel_x: AngleRate,
    pub angvel_y: AngleRate,
    pub angvel_z: AngleRate,
}

impl ToKvn for QuaternionState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("Q1", self.quaternion.q1);
        writer.write_pair("Q2", self.quaternion.q2);
        writer.write_pair("Q3", self.quaternion.q3);
        writer.write_pair("QC", self.quaternion.qc);
        if let Some(dot) = &self.quaternion_dot {
            writer.write_pair("Q1_DOT", dot.q1_dot.value);
            writer.write_pair("Q2_DOT", dot.q2_dot.value);
            writer.write_pair("Q3_DOT", dot.q3_dot.value);
            writer.write_pair("QC_DOT", dot.qc_dot.value);
        }
    }
}

impl ToKvn for EulerAngleState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("EULER_ROT_SEQ", &self.euler_rot_seq);
        writer.write_measure(
            "ANGLE_1",
            &UnitValue {
                value: self.angle_1.value,
                units: self.angle_1.units.clone(),
            },
        );
        writer.write_measure(
            "ANGLE_2",
            &UnitValue {
                value: self.angle_2.value,
                units: self.angle_2.units.clone(),
            },
        );
        writer.write_measure(
            "ANGLE_3",
            &UnitValue {
                value: self.angle_3.value,
                units: self.angle_3.units.clone(),
            },
        );
        if let Some(v) = &self.angle_1_dot {
            writer.write_measure(
                "ANGLE_1_DOT",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.angle_2_dot {
            writer.write_measure(
                "ANGLE_2_DOT",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.angle_3_dot {
            writer.write_measure(
                "ANGLE_3_DOT",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
    }
}

impl ToKvn for AngVelState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("ANGVEL_FRAME", &self.angvel_frame.0);
        // XSD says angVelFrameType is restriction of string. Check struct definition.
        writer.write_measure("ANGVEL_X", &self.angvel_x);
        writer.write_measure("ANGVEL_Y", &self.angvel_y);
        writer.write_measure("ANGVEL_Z", &self.angvel_z);
    }
}
// I need `AngVelFrameType` implies Display? Or ToKvn?
// It is empty restriction in XSD shown in view_file Step 32 line 194?
// Ah, common.rs defines `AngVelFrameType`. I need to check its definition.

impl ToKvn for SpinState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_measure(
            "SPIN_ALPHA",
            &UnitValue {
                value: self.spin_alpha.value,
                units: self.spin_alpha.units.clone(),
            },
        );
        writer.write_measure(
            "SPIN_DELTA",
            &UnitValue {
                value: self.spin_delta.value,
                units: self.spin_delta.units.clone(),
            },
        );
        writer.write_measure(
            "SPIN_ANGLE",
            &UnitValue {
                value: self.spin_angle.value,
                units: self.spin_angle.units.clone(),
            },
        );
        writer.write_measure(
            "SPIN_ANGLE_VEL",
            &UnitValue {
                value: self.spin_angle_vel.value,
                units: self.spin_angle_vel.units.clone(),
            },
        );

        if let Some(v) = &self.nutation {
            writer.write_measure(
                "NUTATION",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.nutation_per {
            writer.write_measure(
                "NUTATION_PER",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.nutation_phase {
            writer.write_measure(
                "NUTATION_PHASE",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.momentum_alpha {
            writer.write_measure(
                "MOMENTUM_ALPHA",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.momentum_delta {
            writer.write_measure(
                "MOMENTUM_DELTA",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
        if let Some(v) = &self.nutation_vel {
            writer.write_measure(
                "NUTATION_VEL",
                &UnitValue {
                    value: v.value,
                    units: v.units.clone(),
                },
            );
        }
    }
}

impl ToKvn for InertiaState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("INERTIA_REF_FRAME", &self.inertia_ref_frame);
        writer.write_measure("IXX", &self.ixx);
        writer.write_measure("IYY", &self.iyy);
        writer.write_measure("IZZ", &self.izz);
        writer.write_measure("IXY", &self.ixy);
        writer.write_measure("IXZ", &self.ixz);
        writer.write_measure("IYZ", &self.iyz);
    }
}

/// Position/Velocity Covariance Matrix (6x6 Lower Triangular Form. None or all parameters of the
/// matrix must be given. COV_REF_FRAME may be omitted if it is the same as REF_FRAME.)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OpmCovarianceMatrix {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Reference frame in which the covariance data are given. Select from the accepted set of
    /// values indicated in 3.2.4.11.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_ref_frame: Option<String>,
    /// Covariance matrix `[1,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_x: PositionCovariance,
    /// Covariance matrix `[2,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_x: PositionCovariance,
    /// Covariance matrix `[2,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_y: PositionCovariance,
    /// Covariance matrix `[3,1]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_x: PositionCovariance,
    /// Covariance matrix `[3,2]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_y: PositionCovariance,
    /// Covariance matrix `[3,3]`
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_z: PositionCovariance,

    /// Covariance matrix `[4,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[4,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[4,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[4,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_x_dot: VelocityCovariance,

    /// Covariance matrix `[5,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[5,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[5,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[5,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[5,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_y_dot: VelocityCovariance,

    /// Covariance matrix `[6,1]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_x: PositionVelocityCovariance,
    /// Covariance matrix `[6,2]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_y: PositionVelocityCovariance,
    /// Covariance matrix `[6,3]`
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_z: PositionVelocityCovariance,
    /// Covariance matrix `[6,4]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_x_dot: VelocityCovariance,
    /// Covariance matrix `[6,5]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_y_dot: VelocityCovariance,
    /// Covariance matrix `[6,6]`
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
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

/// Atmospheric reentry parameters (atmosphericReentryParametersType, RDM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AtmosphericReentryParameters {
    /// Comments (allowed only at the beginning of each RDM data logical block).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Time until re-entry: from the EPOCH_TZERO epoch in the metadata (days—double precision
    /// values allowed; integer values assumed to have .0 fractional part) to permanently
    /// crossing the altitude specified in REENTRY_ALTITUDE. If the NOMINAL_REENTRY_EPOCH
    /// keyword is present, the ORBIT_LIFETIME and NOMINAL_REENTRY_EPOCH should resolve to the
    /// same value.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    pub orbit_lifetime: DayIntervalRequired,
    /// Defined re-entry altitude over a spherical central body—once an object’s altitude
    /// permanently drops below this value, it is considered to be captured by the central
    /// body’s atmosphere.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    pub reentry_altitude: PositionRequired,
    /// Start of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the
    /// metadata (days—double precision values allowed; integer values assumed to have .0
    /// fractional part). To be used for long-term predictions; REENTRY_WINDOW_START and _END
    /// should be used for accurate results.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_window_start: Option<DayIntervalRequired>,
    /// End of the predicted orbital lifetime window from the EPOCH_TZERO epoch in the metadata
    /// (days—double precision values allowed; integer values assumed to have .0 fractional
    /// part). To be used for long-term predictions; REENTRY_WINDOW_START and _END should be
    /// used for accurate results.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_window_end: Option<DayIntervalRequired>,
    /// Predicted epoch at which the object’s altitude permanently drops below
    /// NOMINAL_REENTRY_ALTITUDE (formatting rules specified in 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_reentry_epoch: Option<Epoch>,
    /// Start epoch of the predicted atmospheric re-entry window (formatting rules specified in
    /// 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_window_start: Option<Epoch>,
    /// End epoch of the predicted atmospheric re-entry window (formatting rules specified in
    /// 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_window_end: Option<Epoch>,
    /// Confidence level of the orbit lifetime or re-entry epoch being inside the window
    /// defined by ORBIT_LIFETIME_WINDOW_START and ORBIT_LIFETIME_WINDOW_END or
    /// REENTRY_WINDOW_START and REENTRY_WINDOW_END.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_lifetime_confidence_level: Option<PercentageRequired>,
}

impl ToKvn for AtmosphericReentryParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("ORBIT_LIFETIME", &self.orbit_lifetime);
        writer.write_pair("REENTRY_ALTITUDE", &self.reentry_altitude);
        if let Some(v) = &self.orbit_lifetime_window_start {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_START", v);
        }
        if let Some(v) = &self.orbit_lifetime_window_end {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_END", v);
        }
        if let Some(v) = &self.nominal_reentry_epoch {
            writer.write_pair("NOMINAL_REENTRY_EPOCH", v);
        }
        if let Some(v) = &self.reentry_window_start {
            writer.write_pair("REENTRY_WINDOW_START", v);
        }
        if let Some(v) = &self.reentry_window_end {
            writer.write_pair("REENTRY_WINDOW_END", v);
        }
        if let Some(v) = &self.orbit_lifetime_confidence_level {
            writer.write_pair("ORBIT_LIFETIME_CONFIDENCE_LEVEL", v);
        }
    }
}

/// Ground impact parameters (groundImpactParametersType, RDM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GroundImpactParameters {
    /// Comments (allowed only at the beginning of each RDM data logical block).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Probability that any fragment will impact the Earth (either land or sea; 0 to 1).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_impact: Option<Probability>,
    /// Probability that the entire object and any fragments will burn up during atmospheric
    /// re-entry (0 to 1).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_burn_up: Option<Probability>,
    /// Probability that the object will break up during re-entry (0 to 1).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_break_up: Option<Probability>,
    /// Probability that any fragment will impact solid ground (0 to 1).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_land_impact: Option<Probability>,
    /// Probability that the re-entry event will cause any casualties (severe injuries or
    /// deaths—0 to 1).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability_of_casualty: Option<Probability>,
    /// Epoch of the predicted impact (formatting rules specified in 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_epoch: Option<Epoch>,
    /// Start epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_window_start: Option<Epoch>,
    /// End epoch of the predicted impact window (formatting rules specified in 5.3.3.5).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_window_end: Option<Epoch>,
    /// Reference frame of the impact location data. The value should be taken from the keyword
    /// value name column in the SANA celestial body reference frames registry, reference `[11]`.
    /// Only frames with the value ‘Body-Fixed’ in the Frame Type column shall be used.
    /// Mandatory if NOMINAL_IMPACT_LON and NOMINAL_IMPACT_LAT are present.
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_ref_frame: Option<String>,
    /// Longitude of the predicted impact location with respect to the value of
    /// IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
    /// 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_lon: Option<LongitudeRequired>,
    /// Latitude of the predicted impact location with respect to the value of
    /// IMPACT_REF_FRAME. Values shall be double precision and follow the rules specified in
    /// 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_lat: Option<LatitudeRequired>,
    /// Altitude of the impact location with respect to the value of IMPACT_REF_FRAME.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_impact_alt: Option<AltitudeRequired>,
    /// First (lowest) confidence interval for the impact location.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_confidence: Option<PercentageRequired>,
    /// Longitude of the start of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_start_lon: Option<LongitudeRequired>,
    /// Latitude of the start of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_start_lat: Option<LatitudeRequired>,
    /// Longitude of the end of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_stop_lon: Option<LongitudeRequired>,
    /// Latitude of the end of the first confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_stop_lat: Option<LatitudeRequired>,
    /// Cross-track size of the first confidence interval.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_1_cross_track: Option<Distance>,
    /// Second confidence interval for the impact location. The IMPACT_1_* block must be
    /// present if IMPACT_2_* is used.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_confidence: Option<PercentageRequired>,
    /// Longitude of the start of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_start_lon: Option<LongitudeRequired>,
    /// Latitude of the start of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_start_lat: Option<LatitudeRequired>,
    /// Longitude of the end of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_stop_lon: Option<LongitudeRequired>,
    /// Latitude of the end of the second confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_stop_lat: Option<LatitudeRequired>,
    /// Cross-track size of the second confidence interval.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_2_cross_track: Option<Distance>,
    /// Third (highest) confidence interval for the impact location. The IMPACT_2_* block must
    /// be present if IMPACT_3_* is used.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_confidence: Option<PercentageRequired>,
    /// Longitude of the start of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_start_lon: Option<LongitudeRequired>,
    /// Latitude of the start of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_start_lat: Option<LatitudeRequired>,
    /// Longitude of the end of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.11.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_stop_lon: Option<LongitudeRequired>,
    /// Latitude of the end of the third confidence interval along the ground track with
    /// respect to the value of IMPACT_REF_FRAME. Values shall be double precision and follow
    /// the rules specified in 3.5.12.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_stop_lat: Option<LatitudeRequired>,
    /// Cross-track size of the third confidence interval.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_3_cross_track: Option<Distance>,
}

impl ToKvn for GroundImpactParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(v) = &self.probability_of_impact {
            writer.write_pair("PROBABILITY_OF_IMPACT", v.value);
        }
        if let Some(v) = &self.probability_of_burn_up {
            writer.write_pair("PROBABILITY_OF_BURN_UP", v.value);
        }
        if let Some(v) = &self.probability_of_break_up {
            writer.write_pair("PROBABILITY_OF_BREAK_UP", v.value);
        }
        if let Some(v) = &self.probability_of_land_impact {
            writer.write_pair("PROBABILITY_OF_LAND_IMPACT", v.value);
        }
        if let Some(v) = &self.probability_of_casualty {
            writer.write_pair("PROBABILITY_OF_CASUALTY", v.value);
        }
        if let Some(v) = &self.nominal_impact_epoch {
            writer.write_pair("NOMINAL_IMPACT_EPOCH", v);
        }
        if let Some(v) = &self.impact_window_start {
            writer.write_pair("IMPACT_WINDOW_START", v);
        }
        if let Some(v) = &self.impact_window_end {
            writer.write_pair("IMPACT_WINDOW_END", v);
        }
        if let Some(v) = &self.impact_ref_frame {
            writer.write_pair("IMPACT_REF_FRAME", v);
        }
        if let Some(v) = &self.nominal_impact_lon {
            writer.write_pair("NOMINAL_IMPACT_LON", v);
        }
        if let Some(v) = &self.nominal_impact_lat {
            writer.write_pair("NOMINAL_IMPACT_LAT", v);
        }
        if let Some(v) = &self.nominal_impact_alt {
            writer.write_pair("NOMINAL_IMPACT_ALT", v);
        }

        if let Some(v) = &self.impact_1_confidence {
            writer.write_pair("IMPACT_1_CONFIDENCE", v);
        }
        if let Some(v) = &self.impact_1_start_lon {
            writer.write_pair("IMPACT_1_START_LON", v);
        }
        if let Some(v) = &self.impact_1_start_lat {
            writer.write_pair("IMPACT_1_START_LAT", v);
        }
        if let Some(v) = &self.impact_1_stop_lon {
            writer.write_pair("IMPACT_1_STOP_LON", v);
        }
        if let Some(v) = &self.impact_1_stop_lat {
            writer.write_pair("IMPACT_1_STOP_LAT", v);
        }
        if let Some(v) = &self.impact_1_cross_track {
            writer.write_pair("IMPACT_1_CROSS_TRACK", v);
        }

        if let Some(v) = &self.impact_2_confidence {
            writer.write_pair("IMPACT_2_CONFIDENCE", v);
        }
        if let Some(v) = &self.impact_2_start_lon {
            writer.write_pair("IMPACT_2_START_LON", v);
        }
        if let Some(v) = &self.impact_2_start_lat {
            writer.write_pair("IMPACT_2_START_LAT", v);
        }
        if let Some(v) = &self.impact_2_stop_lon {
            writer.write_pair("IMPACT_2_STOP_LON", v);
        }
        if let Some(v) = &self.impact_2_stop_lat {
            writer.write_pair("IMPACT_2_STOP_LAT", v);
        }
        if let Some(v) = &self.impact_2_cross_track {
            writer.write_pair("IMPACT_2_CROSS_TRACK", v);
        }

        if let Some(v) = &self.impact_3_confidence {
            writer.write_pair("IMPACT_3_CONFIDENCE", v);
        }
        if let Some(v) = &self.impact_3_start_lon {
            writer.write_pair("IMPACT_3_START_LON", v);
        }
        if let Some(v) = &self.impact_3_start_lat {
            writer.write_pair("IMPACT_3_START_LAT", v);
        }
        if let Some(v) = &self.impact_3_stop_lon {
            writer.write_pair("IMPACT_3_STOP_LON", v);
        }
        if let Some(v) = &self.impact_3_stop_lat {
            writer.write_pair("IMPACT_3_STOP_LAT", v);
        }
        if let Some(v) = &self.impact_3_cross_track {
            writer.write_pair("IMPACT_3_CROSS_TRACK", v);
        }
    }
}

/// RDM spacecraft parameters (rdmSpacecraftParametersType).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmSpacecraftParameters {
    /// Comments (allowed only at the beginning of each RDM data logical block).
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Total object mass at EPOCH_TZERO.
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    /// Object dry mass (without propellant).
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    /// Comma separated list of hazardous substances contained by the object.
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hazardous_substances: Option<String>,
    /// Object area exposed to Solar Radiation Pressure (SRP).
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_area: Option<Area>,
    /// Object solar radiation coefficient.
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<NonNegativeDouble>,
    /// Object cross-sectional area.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_area: Option<Area>,
    /// Object drag coefficient.
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<NonNegativeDouble>,
    /// Object radar cross section.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs: Option<Area>,
    /// Object ballistic coefficient.
    ///
    /// **Units**: kg/m²
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ballistic_coeff: Option<BallisticCoeff>,
    /// The object’s acceleration due to in-track thrust used to propagate the state vector and
    /// covariance to NOMINAL_RENTRY_EPOCH (if a controlled re-entry).
    ///
    /// **Units**: m/s²
    ///
    /// **CCSDS Reference**: 508.1-B-1, Section 3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrust_acceleration: Option<Ms2>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ToKvn;
    use crate::traits::Validate;

    #[test]
    fn test_ndm_header_validation() {
        let h = NdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("  ") // empty originator
            .build();
        assert!(h.validate().is_err());

        let h2 = NdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("NASA")
            .build();
        assert!(h2.validate().is_ok());
    }

    #[test]
    fn test_ndm_header_kvn() {
        let h = NdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("NASA")
            .comment(vec!["msg".into()])
            .build();
        let mut w = KvnWriter::new();
        h.write_kvn(&mut w);
        let s = w.finish();
        // Check for presence of key and value separately to avoid whitespace issues
        assert!(s.contains("COMMENT msg"));
        assert!(s.contains("CREATION_DATE"));
        assert!(s.contains("2000-01-01T00:00:00"));
        assert!(s.contains("ORIGINATOR"));
        assert!(s.contains("NASA"));
    }

    #[test]
    fn test_adm_header_validation() {
        let h = AdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("")
            .build();
        assert!(h.validate().is_err());

        // modify directly if possible or rebuild? Struct fields are public.
        let mut h2 = h.clone();
        h2.originator = "ESA".into();
        assert!(h2.validate().is_ok());
    }

    #[test]
    fn test_adm_header_kvn() {
        let h = AdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("ESA")
            .message_id("MSG1")
            .classification("SECURE".to_string())
            .build();
        let mut w = KvnWriter::new();
        h.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("CLASSIFICATION"));
        assert!(s.contains("SECURE"));
        assert!(s.contains("MESSAGE_ID"));
        assert!(s.contains("MSG1"));
    }

    #[test]
    fn test_odm_header_validation() {
        let h = OdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("")
            .build();
        assert!(h.validate().is_err());
    }

    #[test]
    fn test_odm_header_kvn() {
        let h = OdmHeader::builder()
            .creation_date("2000-01-01T00:00:00".parse().unwrap())
            .originator("JAXA")
            .build();
        let mut w = KvnWriter::new();
        h.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("ORIGINATOR"));
        assert!(s.contains("JAXA"));
    }

    #[test]
    fn test_state_vector_kvn() {
        let sv = StateVector::builder()
            .epoch("2000-01-01T00:00:00".parse().unwrap())
            .x(Position::new(1.0, None))
            .y(Position::new(2.0, None))
            .z(Position::new(3.0, None))
            .x_dot(Velocity::new(4.0, None))
            .y_dot(Velocity::new(5.0, None))
            .z_dot(Velocity::new(6.0, None))
            .build();

        let mut w = KvnWriter::new();
        sv.write_kvn(&mut w);
        let s = w.finish();
        // check output
        assert!(s.contains("EPOCH"));
        assert!(s.contains("2000-01-01T00:00:00"));
        assert!(s.contains("X"));
        assert!(s.contains("1"));
        assert!(s.contains("Z_DOT"));
        assert!(s.contains("6"));
    }

    #[test]
    fn test_state_vector_acc_kvn() {
        let sv = StateVectorAcc::builder()
            .epoch("2000-01-01T00:00:00".parse().unwrap())
            .x(Position::new(1.0, None))
            .y(Position::new(2.0, None))
            .z(Position::new(3.0, None))
            .x_dot(Velocity::new(4.0, None))
            .y_dot(Velocity::new(5.0, None))
            .z_dot(Velocity::new(6.0, None))
            .build();
        let mut w = KvnWriter::new();
        // StateVectorAcc uses a custom write format in write_kvn?
        // Looking at the code: it writes a raw line "epoch x y z ..."
        sv.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("2000-01-01T00:00:00"));
        assert!(s.contains("1"));
        assert!(s.contains("2"));
        assert!(s.contains("3"));
    }

    #[test]
    fn test_quaternion_state_kvn() {
        let qs = QuaternionState::builder()
            .ref_frame_a("A")
            .ref_frame_b("B")
            .quaternion(Quaternion {
                q1: 1.0,
                q2: 0.0,
                q3: 0.0,
                qc: 0.0,
            })
            .build();
        let mut w = KvnWriter::new();
        qs.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("REF_FRAME_A"));
        assert!(s.contains("A"));
        assert!(s.contains("Q1"));
        assert!(s.contains("1"));
    }

    #[test]
    fn test_euler_angle_state_kvn() {
        let es = EulerAngleState::builder()
            .ref_frame_a("A")
            .ref_frame_b("B")
            .euler_rot_seq(RotSeq::XYZ)
            .angle_1(Angle::new(10.0, None).unwrap())
            .angle_2(Angle::new(20.0, None).unwrap())
            .angle_3(Angle::new(30.0, None).unwrap())
            .build();
        let mut w = KvnWriter::new();
        es.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("EULER_ROT_SEQ"));
        assert!(s.contains("XYZ"));
        assert!(s.contains("ANGLE_1"));
        assert!(s.contains("10"));
    }

    #[test]
    fn test_ang_vel_state_kvn() {
        let avs = AngVelState::builder()
            .ref_frame_a("A")
            .ref_frame_b("B")
            .angvel_frame(AngVelFrameType("FRAME".into()))
            .angvel_x(AngleRate::new(0.1, None))
            .angvel_y(AngleRate::new(0.2, None))
            .angvel_z(AngleRate::new(0.3, None))
            .build();
        let mut w = KvnWriter::new();
        avs.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("ANGVEL_FRAME"));
        assert!(s.contains("FRAME"));
        assert!(s.contains("ANGVEL_X"));
        assert!(s.contains("0.1"));
    }

    #[test]
    fn test_spin_state_kvn() {
        let ss = SpinState::builder()
            .ref_frame_a("A")
            .ref_frame_b("B")
            .spin_alpha(Angle::new(10.0, None).unwrap())
            .spin_delta(Angle::new(20.0, None).unwrap())
            .spin_angle(Angle::new(30.0, None).unwrap())
            .spin_angle_vel(AngleRate::new(0.1, None))
            .build();
        let mut w = KvnWriter::new();
        ss.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("SPIN_ALPHA"));
        assert!(s.contains("10"));
        assert!(s.contains("SPIN_ANGLE_VEL"));
        assert!(s.contains("0.1"));
    }

    #[test]
    fn test_inertia_state_kvn() {
        let is = InertiaState::builder()
            .inertia_ref_frame("FRAME")
            .ixx(Moment::new(100.0, None))
            .iyy(Moment::new(200.0, None))
            .izz(Moment::new(300.0, None))
            .ixy(Moment::new(10.0, None))
            .ixz(Moment::new(20.0, None))
            .iyz(Moment::new(30.0, None))
            .build();
        let mut w = KvnWriter::new();
        is.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("IXX"));
        assert!(s.contains("100"));
        assert!(s.contains("IXY"));
        assert!(s.contains("10"));
    }

    #[test]
    fn test_ephemeris_kvn() {
        let epoch = "2000-01-01T00:00:00".parse().unwrap();
        let qe = QuaternionEphemeris {
            epoch,
            quaternion: Quaternion {
                q1: 1.0,
                q2: 0.0,
                q3: 0.0,
                qc: 0.0,
            },
        };
        let mut w = KvnWriter::new();
        qe.write_kvn(&mut w);
        assert!(w.finish().contains("2000-01-01T00:00:00 1 0 0 0"));

        let qd = QuaternionDerivative {
            epoch,
            quaternion: Quaternion {
                q1: 1.0,
                q2: 0.0,
                q3: 0.0,
                qc: 0.0,
            },
            quaternion_dot: QuaternionDot {
                q1_dot: QuaternionDotComponent::new(0.1, None),
                q2_dot: QuaternionDotComponent::new(0.2, None),
                q3_dot: QuaternionDotComponent::new(0.3, None),
                qc_dot: QuaternionDotComponent::new(0.4, None),
            },
        };
        let mut w = KvnWriter::new();
        qd.write_kvn(&mut w);
        assert!(w
            .finish()
            .contains("2000-01-01T00:00:00 1 0 0 0 0.1 0.2 0.3 0.4"));
    }

    #[test]
    fn test_opm_covariance_kvn() {
        let cov = OpmCovarianceMatrix::builder()
            .cx_x(PositionCovariance::new(1.0, None))
            .cy_x(PositionCovariance::new(2.0, None))
            .cy_y(PositionCovariance::new(3.0, None))
            .cz_x(PositionCovariance::new(4.0, None))
            .cz_y(PositionCovariance::new(5.0, None))
            .cz_z(PositionCovariance::new(6.0, None))
            .cx_dot_x(PositionVelocityCovariance::new(7.0, None))
            .cx_dot_y(PositionVelocityCovariance::new(8.0, None))
            .cx_dot_z(PositionVelocityCovariance::new(9.0, None))
            .cx_dot_x_dot(VelocityCovariance::new(10.0, None))
            .cy_dot_x(PositionVelocityCovariance::new(11.0, None))
            .cy_dot_y(PositionVelocityCovariance::new(12.0, None))
            .cy_dot_z(PositionVelocityCovariance::new(13.0, None))
            .cy_dot_x_dot(VelocityCovariance::new(14.0, None))
            .cy_dot_y_dot(VelocityCovariance::new(15.0, None))
            .cz_dot_x(PositionVelocityCovariance::new(16.0, None))
            .cz_dot_y(PositionVelocityCovariance::new(17.0, None))
            .cz_dot_z(PositionVelocityCovariance::new(18.0, None))
            .cz_dot_x_dot(VelocityCovariance::new(19.0, None))
            .cz_dot_y_dot(VelocityCovariance::new(20.0, None))
            .cz_dot_z_dot(VelocityCovariance::new(21.0, None))
            .build();
        let mut w = KvnWriter::new();
        cov.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("CX_X"));
        assert!(s.contains("1"));
        assert!(s.contains("CZ_DOT_Z_DOT"));
        assert!(s.contains("21"));
    }

    #[test]
    fn test_reentry_params_kvn() {
        // AtmosphericReentryParameters
        // GroundImpactParameters
        let gi = GroundImpactParameters::builder()
            .comment(vec![])
            .probability_of_impact(Probability::new(0.5).unwrap())
            .build();
        let mut w = KvnWriter::new();
        gi.write_kvn(&mut w);
        let s = w.finish();
        assert!(s.contains("PROBABILITY_OF_IMPACT"));
        assert!(s.contains("0.5"));
    }

    #[test]
    fn test_header_validation_missing_date() {
        let h = NdmHeader::builder()
            .creation_date(Epoch::new("").unwrap())
            .originator("NASA")
            .build();
        assert!(h.validate().is_err());

        let h = AdmHeader::builder()
            .creation_date(Epoch::new("").unwrap())
            .originator("ESA")
            .build();
        assert!(h.validate().is_err());

        let h = OdmHeader::builder()
            .creation_date(Epoch::new("").unwrap())
            .originator("JAXA")
            .build();
        assert!(h.validate().is_err());
    }

    #[test]
    fn test_aem_attitude_state_variants_kvn() {
        let epoch = "2000-01-01T00:00:00".parse().unwrap();

        // QuaternionAngVel
        let qav = QuaternionAngVel {
            epoch,
            quaternion: Quaternion {
                q1: 1.0,
                q2: 0.0,
                q3: 0.0,
                qc: 0.0,
            },
            ang_vel: AngVel {
                angvel_x: AngleRate::new(0.1, None),
                angvel_y: AngleRate::new(0.2, None),
                angvel_z: AngleRate::new(0.3, None),
            },
        };
        let mut w = KvnWriter::new();
        AemAttitudeState::QuaternionAngVel(qav).write_kvn(&mut w);
        assert!(w.finish().contains("1 0 0 0"));

        // EulerAngleDerivative
        let ead = EulerAngleDerivative {
            epoch,
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
            angle_1_dot: AngleRate::new(0.1, None),
            angle_2_dot: AngleRate::new(0.2, None),
            angle_3_dot: AngleRate::new(0.3, None),
        };
        let mut w = KvnWriter::new();
        AemAttitudeState::EulerAngleDerivative(ead).write_kvn(&mut w);
        assert!(w.finish().contains("10 20 30"));

        // SpinNutation
        let sn = SpinNutation {
            epoch,
            spin_alpha: Angle::new(10.0, None).unwrap(),
            spin_delta: Angle::new(20.0, None).unwrap(),
            spin_angle: Angle::new(30.0, None).unwrap(),
            spin_angle_vel: AngleRate::new(0.1, None),
            nutation: Angle::new(5.0, None).unwrap(),
            nutation_per: Duration::new(1.0, Some(TimeUnits::Day)).unwrap(),
            nutation_phase: Angle::new(0.0, None).unwrap(),
        };
        let mut w = KvnWriter::new();
        AemAttitudeState::SpinNutation(sn).write_kvn(&mut w);
        let kvn = w.finish();
        assert!(kvn.contains("10 20 30 0.1 5 1 0"));

        // SpinNutationMom
        let snm = SpinNutationMom {
            epoch,
            spin_alpha: Angle::new(10.0, None).unwrap(),
            spin_delta: Angle::new(20.0, None).unwrap(),
            spin_angle: Angle::new(30.0, None).unwrap(),
            spin_angle_vel: AngleRate::new(0.1, None),
            momentum_alpha: Angle::new(5.0, None).unwrap(),
            momentum_delta: Angle::new(5.0, None).unwrap(),
            nutation_vel: AngleRate::new(0.01, None),
        };
        let mut w = KvnWriter::new();
        AemAttitudeState::SpinNutationMom(snm).write_kvn(&mut w);
        assert!(w.finish().contains("0.01"));
    }

    #[test]
    fn test_quaternion_validation() {
        let q = Quaternion {
            q1: 2.0,
            q2: 0.0,
            q3: 0.0,
            qc: 0.0,
        };
        assert!(q.validate().is_err()); // Not normalized
        let q2 = Quaternion {
            q1: 1.0,
            q2: 0.0,
            q3: 0.0,
            qc: 0.0,
        };
        assert!(q2.validate().is_ok());
    }
}
