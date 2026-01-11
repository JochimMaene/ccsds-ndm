// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OEM (Orbit Ephemeris Message).
//!
//! This module implements KVN parsing for OEM using winnow parser combinators.
//! The parsing follows the CCSDS 502.0-B-3 specification structure:
//!
//! ```text
//! OEM
//! ├── Version (CCSDS_OEM_VERS)
//! ├── Header (OdmHeader)
//! │   ├── COMMENT* (optional, multiple)
//! │   ├── CLASSIFICATION (optional)
//! │   ├── CREATION_DATE (required)
//! │   ├── ORIGINATOR (required)
//! │   └── MESSAGE_ID (optional)
//! └── Body (OemBody)
//!     └── Segment* (OemSegment, one or more)
//!         ├── META_START
//!         ├── Metadata (OemMetadata)
//!         │   ├── COMMENT* (optional)
//!         │   ├── OBJECT_NAME (required)
//!         │   ├── OBJECT_ID (required)
//!         │   ├── CENTER_NAME (required)
//!         │   ├── REF_FRAME (required)
//!         │   ├── REF_FRAME_EPOCH (optional)
//!         │   ├── TIME_SYSTEM (required)
//!         │   ├── START_TIME (required)
//!         │   ├── USEABLE_START_TIME (optional)
//!         │   ├── USEABLE_STOP_TIME (optional)
//!         │   ├── STOP_TIME (required)
//!         │   ├── INTERPOLATION (optional)
//!         │   └── INTERPOLATION_DEGREE (conditional)
//!         ├── META_STOP
//!         └── Data (OemData)
//!             ├── COMMENT* (optional)
//!             ├── StateVectorAcc* (raw data lines)
//!             └── CovarianceMatrix* (optional, within COVARIANCE_START/STOP)
//! ```

use crate::common::{OdmHeader, StateVectorAcc};
use crate::kvn::parser::*;
use crate::messages::oem::{Oem, OemBody, OemCovarianceMatrix, OemData, OemMetadata, OemSegment};
use crate::types::*;
use std::num::NonZeroU32;
use std::str::FromStr;
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helper: Check if key belongs to OEM Header section
//----------------------------------------------------------------------

fn is_header_key(key: &str) -> bool {
    matches!(
        key,
        "CLASSIFICATION" | "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID"
    )
}

fn is_metadata_key(key: &str) -> bool {
    matches!(
        key,
        "OBJECT_NAME"
            | "OBJECT_ID"
            | "CENTER_NAME"
            | "REF_FRAME"
            | "REF_FRAME_EPOCH"
            | "TIME_SYSTEM"
            | "START_TIME"
            | "USEABLE_START_TIME"
            | "USEABLE_STOP_TIME"
            | "STOP_TIME"
            | "INTERPOLATION"
            | "INTERPOLATION_DEGREE"
    )
}

//----------------------------------------------------------------------
// OEM Version Parser
//----------------------------------------------------------------------

/// Parses the OEM version line: `CCSDS_OEM_VERS = 3.0`
pub fn oem_version<'a>(input: &mut &'a str) -> ModalResult<String> {
    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_OEM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// ODM Header Parser (shared with OPM)
//----------------------------------------------------------------------

/// Parses the ODM header section.
pub fn odm_header<'a>(input: &mut &'a str) -> ModalResult<OdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        // Collect any comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        // Check what's next
        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_header_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CLASSIFICATION" => classification = Some(v.to_string()),
                    "CREATION_DATE" => {
                        creation_date = Some(
                            Epoch::from_str(v).map_err(|_e| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORIGINATOR" => originator = Some(v.to_string()),
                    "MESSAGE_ID" => message_id = Some(v.to_string()),
                    _ => {}
                }
            }
            // META_START or any other key signals end of header
            _ => break,
        }
    }

    let creation_date = creation_date.ok_or_else(|| ErrMode::Cut(ContextError::new()))?;
    let originator = originator.ok_or_else(|| ErrMode::Cut(ContextError::new()))?;

    Ok(OdmHeader {
        comment,
        classification,
        creation_date,
        originator,
        message_id,
    })
}

//----------------------------------------------------------------------
// OEM Metadata Parser
//----------------------------------------------------------------------

/// Parses the OEM metadata section (between META_START and META_STOP).
pub fn oem_metadata<'a>(input: &mut &'a str) -> ModalResult<OemMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;
    let mut start_time = None;
    let mut useable_start_time = None;
    let mut useable_stop_time = None;
    let mut stop_time = None;
    let mut interpolation = None;
    let mut interpolation_degree = None;

    loop {
        // Collect any comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        // Check if we're at META_STOP
        if at_block_end("META", input) {
            break;
        }

        // Check what's next
        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_metadata_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OBJECT_NAME" => object_name = Some(v.to_string()),
                    "OBJECT_ID" => object_id = Some(v.to_string()),
                    "CENTER_NAME" => center_name = Some(v.to_string()),
                    "REF_FRAME" => ref_frame = Some(v.to_string()),
                    "REF_FRAME_EPOCH" => {
                        ref_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TIME_SYSTEM" => time_system = Some(v.to_string()),
                    "START_TIME" => {
                        start_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "USEABLE_START_TIME" => {
                        useable_start_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "USEABLE_STOP_TIME" => {
                        useable_stop_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "STOP_TIME" => {
                        stop_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "INTERPOLATION" => interpolation = Some(v.to_string()),
                    "INTERPOLATION_DEGREE" => {
                        let val: u32 =
                            v.parse().map_err(|_| ErrMode::Cut(ContextError::new()))?;
                        interpolation_degree = Some(
                            NonZeroU32::new(val).ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    _ => {}
                }
            }
            Some(_unknown) => {
                // Unknown key in metadata - error
                return Err(ErrMode::Cut(ContextError::new()));
            }
            None => break,
        }
    }

    // Validation: INTERPOLATION_DEGREE required if INTERPOLATION present
    if interpolation.is_some() && interpolation_degree.is_none() {
        return Err(ErrMode::Cut(ContextError::new()));
    }

    Ok(OemMetadata {
        comment,
        object_name: object_name.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        object_id: object_id.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        center_name: center_name.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        ref_frame: ref_frame.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        ref_frame_epoch,
        time_system: time_system.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        start_time: start_time.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        useable_start_time,
        useable_stop_time,
        stop_time: stop_time.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        interpolation,
        interpolation_degree,
    })
}

//----------------------------------------------------------------------
// State Vector (Raw Line) Parser
//----------------------------------------------------------------------

/// Parses a raw state vector line.
/// Format: EPOCH X Y Z X_DOT Y_DOT Z_DOT [X_DDOT Y_DDOT Z_DDOT]
fn parse_state_vector_line<'a>(input: &mut &'a str) -> ModalResult<StateVectorAcc> {
    let line = raw_line.parse_next(input)?;
    opt_line_ending.parse_next(input)?;

    let mut tokens = line.split_whitespace();

    // Parse epoch
    let epoch_str = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?;
    let epoch = Epoch::from_str(epoch_str).map_err(|_| ErrMode::Cut(ContextError::new()))?;

    // Parse position components
    let x_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;
    let y_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;
    let z_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;

    // Parse velocity components
    let x_dot_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;
    let y_dot_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;
    let z_dot_val: f64 = tokens
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .parse()
        .map_err(|_| ErrMode::Cut(ContextError::new()))?;

    // Parse optional acceleration components
    let (x_ddot, y_ddot, z_ddot) = if let Some(x_ddot_str) = tokens.next() {
        let x_acc: f64 = x_ddot_str
            .parse()
            .map_err(|_| ErrMode::Cut(ContextError::new()))?;
        let y_acc: f64 = tokens
            .next()
            .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
            .parse()
            .map_err(|_| ErrMode::Cut(ContextError::new()))?;
        let z_acc: f64 = tokens
            .next()
            .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
            .parse()
            .map_err(|_| ErrMode::Cut(ContextError::new()))?;

        // Check no extra tokens
        if tokens.next().is_some() {
            return Err(ErrMode::Cut(ContextError::new()));
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

//----------------------------------------------------------------------
// Covariance Matrix Parser
//----------------------------------------------------------------------

/// Parses a single covariance matrix (within COVARIANCE_START/STOP block).
fn parse_covariance_matrix<'a>(input: &mut &'a str) -> ModalResult<OemCovarianceMatrix> {
    let mut comment = Vec::new();
    let mut cov_ref_frame = None;

    // Collect comments
    let comments = collect_comments.parse_next(input)?;
    comment.extend(comments);

    // Parse EPOCH (required)
    let (val, _) = expect_key("EPOCH").parse_next(input)?;
    let epoch = Epoch::from_str(val).map_err(|_| ErrMode::Cut(ContextError::new()))?;

    // Check for optional COV_REF_FRAME
    let comments = collect_comments.parse_next(input)?;
    comment.extend(comments);

    let next_key = peek_key(input)?;
    if matches!(next_key, Some("COV_REF_FRAME")) {
        let (val, _) = expect_key("COV_REF_FRAME").parse_next(input)?;
        cov_ref_frame = Some(val.to_string());
    }

    // Parse 6 lines of raw covariance data (1, 2, 3, 4, 5, 6 elements per line)
    let expected_counts = [1, 2, 3, 4, 5, 6];
    let mut floats = Vec::with_capacity(21);

    for (row_idx, expected_count) in expected_counts.iter().enumerate() {
        // Skip empty lines and comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let line = raw_line.parse_next(input)?;
        opt_line_ending.parse_next(input)?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != *expected_count {
            return Err(ErrMode::Cut(ContextError::new()));
        }

        for part in parts {
            let val: f64 = part.parse().map_err(|_| ErrMode::Cut(ContextError::new()))?;
            floats.push(val);
        }

        // Check for more raw lines or break if we've read enough
        if row_idx == 5 {
            break;
        }
    }

    if floats.len() != 21 {
        return Err(ErrMode::Cut(ContextError::new()));
    }

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
        cx_dot_x: PositionVelocityCovariance::new(floats[6], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cx_dot_y: PositionVelocityCovariance::new(floats[7], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cx_dot_z: PositionVelocityCovariance::new(floats[8], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cx_dot_x_dot: VelocityCovariance::new(floats[9], Some(VelocityCovarianceUnits::Km2PerS2)),
        cy_dot_x: PositionVelocityCovariance::new(floats[10], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cy_dot_y: PositionVelocityCovariance::new(floats[11], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cy_dot_z: PositionVelocityCovariance::new(floats[12], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cy_dot_x_dot: VelocityCovariance::new(floats[13], Some(VelocityCovarianceUnits::Km2PerS2)),
        cy_dot_y_dot: VelocityCovariance::new(floats[14], Some(VelocityCovarianceUnits::Km2PerS2)),
        cz_dot_x: PositionVelocityCovariance::new(floats[15], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cz_dot_y: PositionVelocityCovariance::new(floats[16], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cz_dot_z: PositionVelocityCovariance::new(floats[17], Some(PositionVelocityCovarianceUnits::Km2PerS)),
        cz_dot_x_dot: VelocityCovariance::new(floats[18], Some(VelocityCovarianceUnits::Km2PerS2)),
        cz_dot_y_dot: VelocityCovariance::new(floats[19], Some(VelocityCovarianceUnits::Km2PerS2)),
        cz_dot_z_dot: VelocityCovariance::new(floats[20], Some(VelocityCovarianceUnits::Km2PerS2)),
    })
}

/// Parses all covariance matrices within a COVARIANCE block.
fn parse_covariance_block<'a>(input: &mut &'a str) -> ModalResult<Vec<OemCovarianceMatrix>> {
    let mut matrices = Vec::new();

    // We're inside the COVARIANCE block, parse matrices until COVARIANCE_STOP
    loop {
        // Skip comments and empty lines
        let _ = collect_comments.parse_next(input)?;

        // Check if we're at COVARIANCE_STOP
        if at_block_end("COVARIANCE", input) {
            break;
        }

        // Check if there's an EPOCH key (start of a covariance matrix)
        let next_key = peek_key(input)?;
        if matches!(next_key, Some("EPOCH")) {
            let cov = parse_covariance_matrix.parse_next(input)?;
            matrices.push(cov);
        } else if next_key.is_none() {
            // End of input
            break;
        } else {
            // Unexpected key
            return Err(ErrMode::Cut(ContextError::new()));
        }
    }

    Ok(matrices)
}

//----------------------------------------------------------------------
// OEM Data Parser
//----------------------------------------------------------------------

/// Checks if we're at a raw data line (starts with a date-like pattern).
fn is_raw_data_line(input: &str) -> bool {
    let trimmed = input.trim_start();
    // Get just the first line
    let first_line = trimmed.lines().next().unwrap_or("");
    // Raw data lines start with a digit (epoch timestamp) and have no '='
    first_line.chars().next().is_some_and(|c| c.is_ascii_digit())
        && !first_line.contains('=')
}

/// Parses the OEM data section (state vectors and optional covariance matrices).
pub fn oem_data<'a>(input: &mut &'a str) -> ModalResult<OemData> {
    let mut comment = Vec::new();
    let mut state_vector = Vec::new();
    let mut covariance_matrix = Vec::new();

    // Parse comments and state vectors
    loop {
        // Collect comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        // Check if we're at META_START (next segment) or COVARIANCE_START
        if at_block_start("META", input) || at_block_start("COVARIANCE", input) {
            break;
        }

        // Check if this is a raw data line
        if is_raw_data_line(input) {
            let sv = parse_state_vector_line.parse_next(input)?;
            state_vector.push(sv);
        } else if input.trim().is_empty() {
            // End of input
            break;
        } else {
            // Check for key-value line (e.g., COMMENT that wasn't caught)
            let next_key = peek_key(input)?;
            if next_key.is_some() {
                // Unexpected key-value in data section - probably belongs to next section
                break;
            }
            // Empty or whitespace - skip
            if let Some(pos) = input.find('\n') {
                *input = &input[pos + 1..];
            } else {
                break;
            }
        }
    }

    // Parse optional covariance blocks
    while at_block_start("COVARIANCE", input) {
        // Consume COVARIANCE_START
        expect_block_start("COVARIANCE").parse_next(input)?;

        // Parse covariance matrices
        let cov_matrices = parse_covariance_block.parse_next(input)?;
        covariance_matrix.extend(cov_matrices);

        // Consume COVARIANCE_STOP
        expect_block_end("COVARIANCE").parse_next(input)?;

        // Skip any trailing comments/empty lines
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);
    }

    if state_vector.is_empty() {
        return Err(ErrMode::Cut(ContextError::new()));
    }

    Ok(OemData {
        comment,
        state_vector,
        covariance_matrix,
    })
}

//----------------------------------------------------------------------
// OEM Segment Parser
//----------------------------------------------------------------------

/// Parses a single OEM segment (META_START ... META_STOP + data).
pub fn oem_segment<'a>(input: &mut &'a str) -> ModalResult<OemSegment> {
    // Expect META_START
    expect_block_start("META").parse_next(input)?;

    // Parse metadata
    let metadata = oem_metadata.parse_next(input)?;

    // Expect META_STOP
    expect_block_end("META").parse_next(input)?;

    // Parse data
    let data = oem_data.parse_next(input)?;

    Ok(OemSegment { metadata, data })
}

//----------------------------------------------------------------------
// OEM Body Parser
//----------------------------------------------------------------------

/// Parses the OEM body (one or more segments).
pub fn oem_body<'a>(input: &mut &'a str) -> ModalResult<OemBody> {
    let mut segments = Vec::new();

    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    // Parse first segment (required)
    if !at_block_start("META", input) {
        return Err(ErrMode::Cut(ContextError::new()));
    }

    let segment = oem_segment.parse_next(input)?;
    segments.push(segment);

    // Parse additional segments
    loop {
        // Skip comments/empty lines
        let _ = collect_comments.parse_next(input)?;

        // Check if there's another segment
        if at_block_start("META", input) {
            let segment = oem_segment.parse_next(input)?;
            segments.push(segment);
        } else {
            break;
        }
    }

    Ok(OemBody { segment: segments })
}

//----------------------------------------------------------------------
// Complete OEM Parser
//----------------------------------------------------------------------

/// Parses a complete OEM message.
pub fn parse_oem<'a>(input: &mut &'a str) -> ModalResult<Oem> {
    // 1. Version
    let version = oem_version.parse_next(input)?;

    // 2. Header
    let header = odm_header.parse_next(input)?;

    // 3. Body (segments)
    let body = oem_body.parse_next(input)?;

    Ok(Oem {
        header,
        body,
        id: Some("CCSDS_OEM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Oem {
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        parse_oem.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_OEM: &str = r#"CCSDS_OEM_VERS = 3.0
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

    const OEM_WITH_COMMENTS: &str = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 1996-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME = EME2000
TIME_SYSTEM = UTC
START_TIME = 2019-12-18T12:00:00.331
STOP_TIME = 2019-12-28T21:28:00.331
META_STOP
COMMENT This is a data section comment
2019-12-18T12:00:00.331 2789.619 -280.045 -1746.755 4.73372 -2.49586 -1.04195
"#;

    const OEM_MULTI_SEGMENT: &str = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 1996-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME = EME2000
TIME_SYSTEM = UTC
START_TIME = 2019-12-18T12:00:00.331
STOP_TIME = 2019-12-28T21:28:00.331
META_STOP
2019-12-18T12:00:00.331 2789.619 -280.045 -1746.755 4.73372 -2.49586 -1.04195
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME = EME2000
TIME_SYSTEM = UTC
START_TIME = 2019-12-28T21:29:07.267
STOP_TIME = 2019-12-30T01:28:02.267
META_STOP
2019-12-28T21:29:07.267 -2432.166 -063.042 1742.754 7.33702 -3.495867 -1.041945
"#;

    #[test]
    fn test_parse_minimal_oem() {
        let result = Oem::from_kvn_str(MINIMAL_OEM);
        assert!(
            result.is_ok(),
            "Failed to parse minimal OEM: {:?}",
            result.err()
        );

        let oem = result.unwrap();
        assert_eq!(oem.version, "3.0");
        assert_eq!(oem.header.originator, "TEST");
        assert_eq!(oem.body.segment.len(), 1);
        assert_eq!(oem.body.segment[0].metadata.object_name, "SAT1");
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 2);
        assert_eq!(oem.body.segment[0].data.state_vector[0].x.value, 1000.0);
    }

    #[test]
    fn test_parse_oem_version() {
        let mut input = "CCSDS_OEM_VERS = 3.0\n";
        let version = oem_version.parse_next(&mut input).unwrap();
        assert_eq!(version, "3.0");
    }

    #[test]
    fn test_parse_oem_with_comments() {
        let result = Oem::from_kvn_str(OEM_WITH_COMMENTS);
        assert!(
            result.is_ok(),
            "Failed to parse OEM with comments: {:?}",
            result.err()
        );

        let oem = result.unwrap();
        assert_eq!(oem.body.segment[0].metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert!(!oem.body.segment[0].data.comment.is_empty());
    }

    #[test]
    fn test_parse_multi_segment_oem() {
        let result = Oem::from_kvn_str(OEM_MULTI_SEGMENT);
        assert!(
            result.is_ok(),
            "Failed to parse multi-segment OEM: {:?}",
            result.err()
        );

        let oem = result.unwrap();
        assert_eq!(oem.body.segment.len(), 2);
        assert_eq!(oem.body.segment[0].data.state_vector.len(), 1);
        assert_eq!(oem.body.segment[1].data.state_vector.len(), 1);
    }

    #[test]
    fn test_parse_state_vector_line() {
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0 3.0\n";
        let sv = parse_state_vector_line.parse_next(&mut input).unwrap();
        assert_eq!(sv.x.value, 1000.0);
        assert_eq!(sv.y.value, 2000.0);
        assert_eq!(sv.z.value, 3000.0);
        assert_eq!(sv.x_dot.value, 1.0);
        assert_eq!(sv.y_dot.value, 2.0);
        assert_eq!(sv.z_dot.value, 3.0);
        assert!(sv.x_ddot.is_none());
    }

    #[test]
    fn test_parse_state_vector_with_acceleration() {
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0 3.0 0.001 0.002 0.003\n";
        let sv = parse_state_vector_line.parse_next(&mut input).unwrap();
        assert_eq!(sv.x.value, 1000.0);
        assert!(sv.x_ddot.is_some());
        assert_eq!(sv.x_ddot.unwrap().value, 0.001);
        assert_eq!(sv.y_ddot.unwrap().value, 0.002);
        assert_eq!(sv.z_ddot.unwrap().value, 0.003);
    }
}
