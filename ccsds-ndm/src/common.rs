// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Contains Rust definitions for common structures
//! from `ndmxml-4.0.0-common-4.0.xsd` used by OEM.

use super::types::*;
use crate::kvn::ser::KvnWriter;
use crate::traits::ToKvn;
use serde::{Deserialize, Serialize};

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
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
    }
}

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
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(ref msg_id) = self.message_id {
            writer.write_pair("MESSAGE_ID", msg_id);
        }
    }
}

/// Represents the `odmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdmHeader {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator. The format and content of the
    /// message identifier value are at the discretion of the originator.
    ///
    /// **Examples**: OPM_201113719185, ABC-12_34
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

/// Spacecraft physical parameters (mass, area, coefficients).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SpacecraftParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<f64>,
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
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdParameters {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub comment: Vec<String>,

    /// The start of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_END).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub time_lastob_start: Option<Epoch>,

    /// The end of a time interval (UTC) that contains the time of the last accepted
    /// observation. (See 6.3.2.6 for formatting rules.) For an exact time, the time interval is
    /// of zero duration (i.e., same value as that of TIME_LASTOB_START).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub time_lastob_end: Option<Epoch>,

    /// The recommended OD time span calculated for the object.
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: days
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub recommended_od_span: Option<DayInterval>,

    /// Based on the observations available and the RECOMMENDED_OD_SPAN, the actual
    /// time span used for the OD of the object. (See annex E for definition.)
    ///
    /// **Examples**: 14, 20.0
    ///
    /// **Units**: days
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub actual_od_span: Option<DayInterval>,

    /// The total number of observations available for orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub obs_available: Option<u32>,

    /// The number of observations used in the orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub obs_used: Option<u32>,

    /// The total number of tracks available for orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub tracks_available: Option<u32>,

    /// The number of tracks used in the orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub tracks_used: Option<u32>,

    /// The percentage of residuals accepted during orbit determination.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub residuals_accepted: Option<Percentage>,

    /// The weighted root mean square (RMS) of the residuals.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    pub weighted_rms: Option<f64>,
}

/// Represents the `stateVectorType` and `stateVectorAccType` from the XSD.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVectorAcc {
    /// Epoch of state vector (see 7.5.10 for formatting rules).
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub epoch: Epoch,

    /// Position vector X-component.
    ///
    /// **Examples**: 6653.148
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x: Position,

    /// Position vector Y-component.
    ///
    /// **Examples**: -20.0
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y: Position,

    /// Position vector Z-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub z: Position,

    /// Velocity vector X-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x_dot: Velocity,

    /// Velocity vector Y-component.
    ///
    /// **Examples**: 7.7
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y_dot: Velocity,

    /// Velocity vector Z-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub z_dot: Velocity,

    /// Acceleration vector X-component.
    ///
    /// **Examples**: 0.001
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_ddot: Option<Acc>,

    /// Acceleration vector Y-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_ddot: Option<Acc>,

    /// Acceleration vector Z-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 5.2.4.
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
                return Err(crate::error::CcsdsNdmError::OutOfRange {
                    name: name.to_string(),
                    value: v.to_string(),
                    expected: "[-1, 1]".to_string(),
                });
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVector {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Epoch of state vector (see 7.5.10 for formatting rules).
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub epoch: Epoch,
    /// Position vector X-component.
    ///
    /// **Examples**: 6653.148
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x: Position,
    /// Position vector Y-component.
    ///
    /// **Examples**: -20.0
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y: Position,
    /// Position vector Z-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub z: Position,
    /// Velocity vector X-component.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub x_dot: Velocity,
    /// Velocity vector Y-component.
    ///
    /// **Examples**: 7.7
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub y_dot: Velocity,
    /// Velocity vector Z-component.
    ///
    /// **Examples**: 0.0
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OpmCovarianceMatrix {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Reference frame in which the covariance data are given. Select from the accepted set of
    /// values indicated in 3.2.4.11.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_ref_frame: Option<String>,
    /// Position X covariance \[1,1\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_x: PositionCovariance,
    /// Position Y / Position X covariance \[2,1\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_x: PositionCovariance,
    /// Position Y covariance \[2,2\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_y: PositionCovariance,
    /// Position Z / Position X covariance \[3,1\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_x: PositionCovariance,
    /// Position Z / Position Y covariance \[3,2\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_y: PositionCovariance,
    /// Position Z covariance \[3,3\].
    ///
    /// **Units**: km²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_z: PositionCovariance,

    /// Velocity X / Position X covariance \[4,1\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_x: PositionVelocityCovariance,
    /// Velocity X / Position Y covariance \[4,2\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_y: PositionVelocityCovariance,
    /// Velocity X / Position Z covariance \[4,3\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_z: PositionVelocityCovariance,
    /// Velocity X covariance \[4,4\].
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cx_dot_x_dot: VelocityCovariance,

    /// Velocity Y / Position X covariance \[5,1\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_x: PositionVelocityCovariance,
    /// Velocity Y / Position Y covariance \[5,2\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_y: PositionVelocityCovariance,
    /// Velocity Y / Position Z covariance \[5,3\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_z: PositionVelocityCovariance,
    /// Velocity Y / Velocity X covariance \[5,4\].
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_x_dot: VelocityCovariance,
    /// Velocity Y covariance \[5,5\].
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cy_dot_y_dot: VelocityCovariance,

    /// Velocity Z / Position X covariance \[6,1\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_x: PositionVelocityCovariance,
    /// Velocity Z / Position Y covariance \[6,2\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_y: PositionVelocityCovariance,
    /// Velocity Z / Position Z covariance \[6,3\].
    ///
    /// **Units**: km²/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_z: PositionVelocityCovariance,
    /// Velocity Z / Velocity X covariance \[6,4\].
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_x_dot: VelocityCovariance,
    /// Velocity Z / Velocity Y covariance \[6,5\].
    ///
    /// **Units**: km²/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub cz_dot_y_dot: VelocityCovariance,
    /// Velocity Z covariance \[6,6\].
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
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
