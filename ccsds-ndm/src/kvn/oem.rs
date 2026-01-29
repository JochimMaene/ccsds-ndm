// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
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
use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::oem::{Oem, OemBody, OemCovarianceMatrix, OemData, OemMetadata, OemSegment};
use crate::parse_block;
use crate::types::*;
use std::num::NonZeroU32;
use winnow::ascii::space1;
use winnow::combinator::{preceded, repeat};
use winnow::error::{AddContext, ErrMode};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// OEM Version Parser
//----------------------------------------------------------------------

/// Parses the OEM version line: `CCSDS_OEM_VERS = 3.0`
pub fn oem_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_OEM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

pub fn oem_header(input: &mut &str) -> KvnResult<OdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let checkpoint_loop = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        let key = match preceded(ws, keyword).parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint_loop);
                break;
            }
        };

        if key == "META_START" {
            input.reset(&checkpoint_loop);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = Some(kv_string.parse_next(input)?);
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = Some(kv_string.parse_next(input)?);
            }
            _ => {
                input.reset(&checkpoint_loop);
                break;
            }
        }

        if input.offset_from(&checkpoint_loop) == 0 {
            break;
        }
    }

    Ok(OdmHeader {
        comment,
        classification,
        creation_date: creation_date
            .ok_or_else(|| missing_field_err(input, "Header", "CREATION_DATE"))?,
        originator: originator.ok_or_else(|| missing_field_err(input, "Header", "ORIGINATOR"))?,
        message_id,
    })
}

//----------------------------------------------------------------------
// OEM Metadata Parser
//----------------------------------------------------------------------

/// Parses the OEM metadata section (between META_START and META_STOP).
pub fn oem_metadata(input: &mut &str) -> KvnResult<OemMetadata> {
    ws.parse_next(input)?;
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

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_epoch,
        "TIME_SYSTEM" => time_system: kv_string,
        "START_TIME" => start_time: kv_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_epoch,
        "STOP_TIME" => stop_time: kv_epoch,
        "INTERPOLATION" => interpolation: kv_string,
        "INTERPOLATION_DEGREE" => val: kv_u32 => {
            interpolation_degree = Some(NonZeroU32::new(val).ok_or_else(|| {
                cut_err(input, "positive integer")
            })?);
        },
    }, |i| at_block_end("META", i), "Unexpected OEM Metadata key");

    // Validation: INTERPOLATION_DEGREE required if INTERPOLATION present
    if interpolation.is_some() && interpolation_degree.is_none() {
        return Err(cut_err(input, "INTERPOLATION_DEGREE"));
    }

    Ok(OemMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT_NAME"))?,
        object_id: object_id.ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT_ID"))?,
        center_name: center_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "CENTER_NAME"))?,
        ref_frame: ref_frame.ok_or_else(|| missing_field_err(input, "Metadata", "REF_FRAME"))?,
        ref_frame_epoch,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "Metadata", "TIME_SYSTEM"))?,
        start_time: start_time.ok_or_else(|| missing_field_err(input, "Metadata", "START_TIME"))?,
        useable_start_time,
        useable_stop_time,
        stop_time: stop_time.ok_or_else(|| missing_field_err(input, "Metadata", "STOP_TIME"))?,
        interpolation,
        interpolation_degree,
    })
}

//----------------------------------------------------------------------
// State Vector (Raw Line) Parser
//----------------------------------------------------------------------

/// Parses a raw state vector line.
/// Format: EPOCH X Y Z X_DOT Y_DOT Z_DOT [X_DDOT Y_DDOT Z_DDOT]
fn parse_state_vector_line(input: &mut &str) -> KvnResult<StateVectorAcc> {
    let epoch = preceded(ws, kv_epoch_token).parse_next(input)?;

    let mut floats = [0.0f64; 9];
    let mut count = 0;

    for f in &mut floats {
        let checkpoint = input.checkpoint();
        match preceded(space1, parse_f64_winnow).parse_next(input) {
            Ok(val) => {
                *f = val;
                count += 1;
            }
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        }
    }

    if count < 6 {
        return Err(cut_err(
            input,
            "State vector must have at least 6 components (X, Y, Z, X_DOT, Y_DOT, Z_DOT)",
        ));
    }

    if count > 6 && count < 9 {
        return Err(cut_err(
            input,
            "State vector must have either 6 or 9 components",
        ));
    }

    let x_ddot = if count >= 7 {
        Some(Acc::new(floats[6], Some(AccUnits::KmPerS2)))
    } else {
        None
    };
    let y_ddot = if count >= 8 {
        Some(Acc::new(floats[7], Some(AccUnits::KmPerS2)))
    } else {
        None
    };
    let z_ddot = if count >= 9 {
        Some(Acc::new(floats[8], Some(AccUnits::KmPerS2)))
    } else {
        None
    };

    // Consume trailing line ending if not already at EOF
    opt_line_ending.parse_next(input)?;

    Ok(StateVectorAcc {
        epoch,
        x: Position::new(floats[0], Some(PositionUnits::Km)),
        y: Position::new(floats[1], Some(PositionUnits::Km)),
        z: Position::new(floats[2], Some(PositionUnits::Km)),
        x_dot: Velocity::new(floats[3], Some(VelocityUnits::KmPerS)),
        y_dot: Velocity::new(floats[4], Some(VelocityUnits::KmPerS)),
        z_dot: Velocity::new(floats[5], Some(VelocityUnits::KmPerS)),
        x_ddot,
        y_ddot,
        z_ddot,
    })
}

//----------------------------------------------------------------------
// Covariance Matrix Parser
//----------------------------------------------------------------------

/// Parses a single covariance matrix (within COVARIANCE_START/STOP block).
fn parse_covariance_matrix(input: &mut &str) -> KvnResult<OemCovarianceMatrix> {
    let mut comment = collect_comments.parse_next(input)?;

    let checkpoint = input.checkpoint();
    let key = key_token
        .parse_next(input)
        .map_err(|_| cut_err(input, "Expected EPOCH in covariance matrix"))?;

    if key != "EPOCH" {
        input.reset(&checkpoint);
        return Err(cut_err(input, "Expected EPOCH in covariance matrix"));
    }

    let epoch = kv_epoch.parse_next(input)?;

    // Once we have the epoch, the rest of the covariance matrix follows
    // Check for optional COV_REF_FRAME
    let mut cov_ref_frame = None;
    comment.extend(collect_comments.parse_next(input)?);
    let checkpoint_inner = input.checkpoint();
    if let Ok("COV_REF_FRAME") = key_token.parse_next(input) {
        cov_ref_frame = Some(kv_string.parse_next(input)?);
    } else {
        input.reset(&checkpoint_inner);
    }

    // Parse 6 lines of raw covariance data (1, 2, 3, 4, 5, 6 elements per line)
    let expected_counts = [1, 2, 3, 4, 5, 6];
    let mut floats = Vec::with_capacity(21);

    for expected_count in expected_counts {
        comment.extend(collect_comments.parse_next(input)?);

        let line_vals = (
            preceded(ws, parse_f64_winnow),
            repeat(expected_count - 1, preceded(space1, parse_f64_winnow)),
        )
            .map(|(first, rest): (f64, Vec<f64>)| {
                let mut all = vec![first];
                all.extend(rest);
                all
            })
            .parse_next(input)?;

        if line_vals.len() != expected_count {
            return Err(cut_err(input, "Unexpected key or invalid format"));
        }

        floats.extend(line_vals);
        opt_line_ending.parse_next(input)?;
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
        cx_dot_x_dot: VelocityCovariance::new(floats[9], Some(VelocityCovarianceUnits::Km2PerS2)),
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
        cy_dot_x_dot: VelocityCovariance::new(floats[13], Some(VelocityCovarianceUnits::Km2PerS2)),
        cy_dot_y_dot: VelocityCovariance::new(floats[14], Some(VelocityCovarianceUnits::Km2PerS2)),
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
        cz_dot_x_dot: VelocityCovariance::new(floats[18], Some(VelocityCovarianceUnits::Km2PerS2)),
        cz_dot_y_dot: VelocityCovariance::new(floats[19], Some(VelocityCovarianceUnits::Km2PerS2)),
        cz_dot_z_dot: VelocityCovariance::new(floats[20], Some(VelocityCovarianceUnits::Km2PerS2)),
    })
}

/// Parses all covariance matrices within a COVARIANCE block.
fn parse_covariance_block(input: &mut &str) -> KvnResult<Vec<OemCovarianceMatrix>> {
    let mut matrices: Vec<OemCovarianceMatrix> = Vec::new();

    loop {
        let checkpoint = input.checkpoint();
        comment_line.parse_next(input).ok(); // Consume any comments
        if at_block_end("COVARIANCE", input) {
            break;
        }
        let matrix = parse_covariance_matrix.parse_next(input)?;
        matrices.push(matrix);

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(matrices)
}

//----------------------------------------------------------------------
// OEM Data Parser
//----------------------------------------------------------------------

enum OemDataItem {
    Comment(Vec<String>),
    StateVec(StateVectorAcc),
    StateVecWithComments(StateVectorAcc, Vec<String>),
    Cov(Vec<OemCovarianceMatrix>),
    CovWithComments(Vec<OemCovarianceMatrix>, Vec<String>),
}

fn oem_data_item(input: &mut &str) -> KvnResult<OemDataItem> {
    // Fast-path: if it looks like a state vector line (starts with digit or sign after possible whitespace),
    // skip comment collection to avoid Vec allocation.
    let _ = ws.parse_next(input);
    let remaining = *input;
    let first_char = remaining.chars().next();
    if matches!(first_char, Some('0'..='9' | '-' | '+')) {
        let sv = parse_state_vector_line.parse_next(input)?;
        return Ok(OemDataItem::StateVec(sv));
    }

    let comments = collect_comments.parse_next(input)?;

    let remaining = *input;
    if remaining.is_empty() || at_block_start("META", input) {
        if !comments.is_empty() {
            // Trailing comments before META/EOF
            return Ok(OemDataItem::Comment(comments));
        }
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }

    if at_block_start("COVARIANCE", input) {
        expect_block_start("COVARIANCE").parse_next(input)?;
        let mut matrices = parse_covariance_block.parse_next(input)?;
        expect_block_end("COVARIANCE").parse_next(input)?;

        // Attach comments to first matrix if possible
        if let Some(first) = matrices.get_mut(0) {
            first.comment.splice(0..0, comments);
            return Ok(OemDataItem::Cov(matrices));
        } else {
            // Empty covariance block? Unusual but valid structurally.
            // Comments go to global if they can't be attached.
            return Ok(OemDataItem::CovWithComments(matrices, comments));
        }
    }

    // Try state vector
    let sv = parse_state_vector_line.parse_next(input)?;
    if !comments.is_empty() {
        return Ok(OemDataItem::StateVecWithComments(sv, comments));
    }
    Ok(OemDataItem::StateVec(sv))
}

/// Parses the OEM data section (state vectors and optional covariance matrices).
pub fn oem_data(input: &mut &str) -> KvnResult<OemData> {
    // Optimization: Pre-allocate vectors based on input size.
    // Typical OEM state vector line is ~80-100 characters.
    let estimated_records = (input.len() / 80).max(10);

    let mut data = OemData {
        comment: Vec::new(),
        state_vector: Vec::with_capacity(estimated_records),
        covariance_matrix: Vec::new(),
    };

    let mut covariance_started = false;

    loop {
        let _ = ws.parse_next(input);
        if input.is_empty() || at_block_start("META", input) {
            break;
        }

        let checkpoint = input.checkpoint();
        match oem_data_item.parse_next(input) {
            Ok(item) => match item {
                OemDataItem::Comment(c) => data.comment.extend(c),
                OemDataItem::StateVec(sv) => {
                    if covariance_started {
                        input.reset(&checkpoint);
                        return Err(cut_err(
                            input,
                            "State vectors cannot appear after covariance matrix block",
                        ));
                    }
                    data.state_vector.push(sv);
                }
                OemDataItem::StateVecWithComments(sv, c) => {
                    if covariance_started {
                        input.reset(&checkpoint);
                        return Err(cut_err(
                            input,
                            "State vectors cannot appear after covariance matrix block",
                        ));
                    }
                    data.comment.extend(c);
                    data.state_vector.push(sv);
                }
                OemDataItem::Cov(covs) => {
                    covariance_started = true;
                    data.covariance_matrix.extend(covs);
                }
                OemDataItem::CovWithComments(covs, c) => {
                    covariance_started = true;
                    data.comment.extend(c);
                    data.covariance_matrix.extend(covs);
                }
            },
            Err(e) => {
                if e.is_backtrack() || input.offset_from(&checkpoint) == 0 {
                    input.reset(&checkpoint);
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }

    if data.state_vector.is_empty() {
        return Err(cut_err(input, "OEM must contain at least one state vector"));
    }

    Ok(data)
}

//----------------------------------------------------------------------
// OEM Segment Parser
//----------------------------------------------------------------------

/// Parses a single OEM segment (META_START ... META_STOP + data).
pub fn oem_segment(input: &mut &str) -> KvnResult<OemSegment> {
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
pub fn oem_body(input: &mut &str) -> KvnResult<OemBody> {
    let mut segments = Vec::new();

    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    // Parse first segment (required)
    if !at_block_start("META", input) {
        return Err(cut_err(input, "Unexpected key or invalid format"));
    }

    let segment = oem_segment.parse_next(input)?;
    segments.push(segment);

    // Parse additional segments
    loop {
        let checkpoint = input.checkpoint();
        // Skip comments/empty lines
        let _ = collect_comments.parse_next(input)?;

        // Check if there's another segment
        if at_block_start("META", input) {
            let segment = oem_segment.parse_next(input)?;
            segments.push(segment);
        } else {
            break;
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(OemBody { segment: segments })
}

//----------------------------------------------------------------------
// Complete OEM Parser
//----------------------------------------------------------------------

/// Parses a complete OEM message.
pub fn parse_oem(input: &mut &str) -> KvnResult<Oem> {
    // 1. Version
    let version = oem_version.parse_next(input)?;

    // 2. Header
    let header = oem_header.parse_next(input)?;

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
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_oem.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, ValidationError};
    use crate::traits::Ndm;

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
        assert_eq!(
            oem.body.segment[0].metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
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

    #[test]
    fn test_oem_errors() {
        // Missing CREATION_DATE in header
        let kvn = "CCSDS_OEM_VERS = 3.0\nORIGINATOR = TEST\n";
        assert!(Oem::from_kvn_str(kvn).is_err());

        // Missing ORIGINATOR in header
        let kvn = "CCSDS_OEM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\n";
        assert!(Oem::from_kvn_str(kvn).is_err());

        // Invalid epoch in metadata
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = INVALID_EPOCH
STOP_TIME = 2023-01-02T00:00:00
META_STOP
"#;
        assert!(Oem::from_kvn_str(kvn).is_err());

        // Invalid START_TIME/STOP_TIME format
        let kvn_base = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
"#;
        assert!(Oem::from_kvn_str(&format!(
            "{}START_TIME = INVALID\nSTOP_TIME = 2023-01-02T00:00:00\nMETA_STOP\n",
            kvn_base
        ))
        .is_err());
        assert!(Oem::from_kvn_str(&format!(
            "{}START_TIME = 2023-01-01T00:00:00\nSTOP_TIME = INVALID\nMETA_STOP\n",
            kvn_base
        ))
        .is_err());
        assert!(Oem::from_kvn_str(&format!("{}START_TIME = 2023-01-01T00:00:00\nSTOP_TIME = 2023-01-02T00:00:00\nUSEABLE_START_TIME = INVALID\nMETA_STOP\n", kvn_base)).is_err());
        assert!(Oem::from_kvn_str(&format!("{}START_TIME = 2023-01-01T00:00:00\nSTOP_TIME = 2023-01-02T00:00:00\nUSEABLE_STOP_TIME = INVALID\nMETA_STOP\n", kvn_base)).is_err());

        // Unknown key in metadata
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
UNKNOWN_KEY = VAL
"#;
        assert!(Oem::from_kvn_str(kvn).is_err());

        // Missing interpolation degree when interpolation present
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = LINEAR
META_STOP
"#;
        assert!(Oem::from_kvn_str(kvn).is_err());

        // State vector line errors
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0\n"; // Missing one velocity component
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());

        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0 3.0 4.0 5.0\n"; // Missing one acceleration
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());

        let mut input = "INVALID_EPOCH 1000.0 2000.0 3000.0 1.0 2.0 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());

        let mut input = "2023-01-01T00:00:00 BAD 2000.0 3000.0 1.0 2.0 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        let mut input = "2023-01-01T00:00:00 1000.0 BAD 3000.0 1.0 2.0 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 BAD 1.0 2.0 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 BAD 2.0 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 BAD 3.0\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        let mut input = "2023-01-01T00:00:00 1000.0 2000.0 3000.0 1.0 2.0 BAD\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());

        // Covariance errors
        let mut input = "EPOCH = 2023-01-01T00:00:00\n1.0\n0.1\n"; // Row 2 has only 1 element instead of 2
        assert!(parse_covariance_matrix.parse_next(&mut input).is_err());

        let mut input = "COVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\n1.0\nCOVARIANCE_STOP\n"; // Incomplete matrix
        assert!(parse_covariance_block.parse_next(&mut input).is_err());

        let mut input = "COVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\nUNKNOWN_KEY = VAL\n";
        assert!(parse_covariance_block.parse_next(&mut input).is_err());

        // Body must have at least one segment
        let kvn = "CCSDS_OEM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\n";
        assert!(Oem::from_kvn_str(kvn).is_err());

        // oem_data must have state vectors
        let mut input = "COVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\n1.0\n2.0 3.0\n4.0 5.0 6.0\n7.0 8.0 9.0 10.0\n11.0 12.0 13.0 14.0 15.0\n16.0 17.0 18.0 19.0 20.0 21.0\nCOVARIANCE_STOP\n";
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_data_branches() {
        // Test oem_data loop branches (comments, empty lines, block starts)
        let kvn = r#"META_STOP
COMMENT data comment
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
        let mut input = &kvn[10..]; // Skip META_STOP
        let data = oem_data.parse_next(&mut input).unwrap();
        assert_eq!(data.state_vector.len(), 1);
        assert_eq!(data.covariance_matrix.len(), 1);
        assert_eq!(data.comment, vec!["data comment"]);

        // Test unknown key starting with 'C' in oem_data loop
        let kvn = "COOL_KEY = VAL\n";
        let mut input = kvn;
        assert!(oem_data.parse_next(&mut input).is_err());

        // Test unknown key starting with other letter
        let kvn = "X_KEY = VAL\n";
        let mut input = kvn;
        assert!(oem_data.parse_next(&mut input).is_err());

        // Test empty line in oem_data (just spaces)
        let kvn = "   \n2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0\n";
        let mut input = kvn;
        let data = oem_data.parse_next(&mut input).unwrap();
        assert_eq!(data.state_vector.len(), 1);
    }

    #[test]
    fn test_oem_data_interleaved_error() {
        // State vectors cannot appear after covariance block
        let kvn = r#"2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
0.1 1.0
0.1 0.1 1.0
0.01 0.01 0.01 1.0
0.01 0.01 0.01 0.1 1.0
0.01 0.01 0.01 0.1 0.1 1.0
COVARIANCE_STOP
2023-01-01T00:01:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let mut input = kvn;
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_metadata_more_errors() {
        // Invalid interpolation degree (not a number)
        let kvn = r#"META_START
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = LINEAR
INTERPOLATION_DEGREE = NOT_A_NUMBER
META_STOP
"#;
        let mut input = kvn;
        assert!(oem_segment.parse_next(&mut input).is_err());

        // Missing metadata fields
        let kvn = "META_START\nOBJECT_NAME = SAT\nMETA_STOP\n";
        let mut input = kvn;
        assert!(oem_metadata.parse_next(&mut input).is_err());

        // REF_FRAME_EPOCH invalid
        let kvn = "REF_FRAME_EPOCH = INVALID\n";
        let mut input = kvn;
        assert!(oem_metadata.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_body_errors() {
        // Body doesn't start with META_START
        let kvn = "CCSDS_OEM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\nNOT_META_START\n";
        assert!(Oem::from_kvn_str(kvn).is_err());
    }

    #[test]
    fn test_block_checks() {
        let mut input = "META_STAR";
        assert!(!at_block_start("META", &mut input));
        let mut input = "META_START_EXTRA";
        assert!(!at_block_start("META", &mut input));
        let mut input = "META_STOP_EXTRA";
        assert!(!at_block_end("META", &mut input));
        let mut input = "META_END_EXTRA";
        assert!(!at_block_end("META", &mut input));
    }

    #[test]
    fn test_odm_header_errors() {
        // Invalid creation date
        let kvn = "CREATION_DATE = INVALID\nORIGINATOR = TEST\n";
        let mut input = kvn;
        assert!(odm_header.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_covariance_matrix_errors() {
        // Invalid epoch
        let mut input = "EPOCH = INVALID\n";
        assert!(parse_covariance_matrix.parse_next(&mut input).is_err());

        // Invalid float in data
        let mut input = "EPOCH = 2023-01-01T00:00:00\nNOT_A_FLOAT\n";
        assert!(parse_covariance_matrix.parse_next(&mut input).is_err());
    }
    // Tests moved from messages/oem.rs
    #[test]
    fn test_parse_oem_simple_moved() {
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
        if let Some(ValidationError::MissingRequiredField { field, .. }) =
            err1.as_validation_error()
        {
            assert_eq!(field, "CREATION_DATE");
        } else {
            panic!("unexpected error: {:?}", err1);
        }

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
        if let Some(ValidationError::MissingRequiredField { field, .. }) =
            err2.as_validation_error()
        {
            assert_eq!(field, "ORIGINATOR");
        } else {
            panic!("unexpected error: {:?}", err2);
        }
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
            e if e.is_kvn_error() => true, // winnow parser error is acceptable
            e if e.is_validation_error() => true,
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
        // The parser might return out of range or line context error
        // Just assert error for now as message might vary slightly
        assert!(err.is_validation_error() || err.is_kvn_error());
    }

    #[test]
    fn test_covariance_block_start_stop_and_optional_ref_frame() {
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
    fn test_parse_oem_with_covariance_basic() {
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
        assert_eq!(data.covariance_matrix[0].cz_z.value, 1.0);
    }

    #[test]
    fn test_version_must_be_first_moved() {
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
        let ok = err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref msg, .. } if msg.contains("CCSDS_OEM_VERS")))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_missing_required_metadata_fields() {
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
        match err {
            e if e.is_validation_error() => {
                if let Some(ValidationError::MissingRequiredField { field: k, .. }) =
                    e.as_validation_error()
                {
                    assert_eq!(k, "OBJECT_NAME");
                }
            }
            e if e.is_kvn_error() => {}
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_body_must_have_at_least_one_segment() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k.contains("segment")))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_segment_requires_meta_start_stop() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        let ok = false || err.is_validation_error() || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_data_requires_at_least_one_state_vector() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k.contains("must contain at least one state vector")))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_covariance_requires_epoch_and_21_values() {
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
        assert!(err1.is_kvn_error());

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
        assert!(err2.is_kvn_error());
    }

    #[test]
    fn test_invalid_epoch_in_state_vector() {
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
        assert!(matches!(err, CcsdsNdmError::Epoch(_)) | err.is_kvn_error() | false);
    }

    #[test]
    fn test_xsd_missing_object_id() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "OBJECT_ID"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_missing_center_name() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "CENTER_NAME"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_missing_ref_frame() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "REF_FRAME"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_missing_time_system() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "TIME_SYSTEM"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_missing_start_time() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "START_TIME"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_missing_stop_time() {
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
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k == "STOP_TIME"))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_body_min_one_segment() {
        let kvn = r#"CCSDS_OEM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Oem::from_kvn(kvn).unwrap_err();
        let ok = false             || err.as_validation_error().map_or(false, |e| matches!(e, ValidationError::MissingRequiredField { field: ref k, .. } if k.contains("segment")))
            || err.is_kvn_error();
        assert!(ok, "unexpected error: {:?}", err);
    }

    #[test]
    fn test_xsd_body_multiple_segments() {
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
    fn test_xsd_version_attribute_fixed() {
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

    #[test]
    fn test_xsd_data_min_one_state_vector() {
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
        assert!(err.is_kvn_error() || err.is_validation_error());
    }

    #[test]
    fn test_xsd_data_multiple_state_vectors() {
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
    }

    #[test]
    fn test_xsd_state_vector_with_acceleration() {
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
    }

    #[test]
    fn test_xsd_data_comments_unbounded() {
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

    #[test]
    fn test_xsd_covariance_optional() {
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
        assert!(err.is_kvn_error());
    }

    #[test]
    fn test_xsd_covariance_21_values_required() {
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
        assert!(err.is_kvn_error());
    }

    #[test]
    fn test_xsd_covariance_cov_ref_frame_optional() {
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
        assert_eq!(cov.cx_x.value, 1.0);
        assert_eq!(cov.cy_x.value, 2.0);
        assert_eq!(cov.cz_dot_z_dot.value, 21.0);
    }

    #[test]
    fn test_xsd_parse_sample_oem_g11() {
        let kvn = include_str!("../../../data/kvn/oem_g11.kvn");
        let oem = Oem::from_kvn(kvn).expect("Failed to parse oem_g11.kvn");
        assert_eq!(oem.version, "3.0");
        assert_eq!(oem.header.originator, "NASA/JPL");
    }

    #[test]
    fn test_xsd_metadata_optional_ref_frame_epoch() {
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
    }

    #[test]
    fn test_xsd_interpolation_degree_positive_integer() {
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
        assert!(err.is_validation_error() || err.is_kvn_error());
    }

    #[test]
    fn test_xsd_metadata_comments_unbounded() {
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

    #[test]
    fn test_oem_header_loops() {
        let mut input =
            "COMMENT C1\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = ME\nMETA_START";
        let header = oem_header.parse_next(&mut input).unwrap();
        assert_eq!(header.comment, vec!["C1"]);
        assert_eq!(header.originator, "ME");
        assert_eq!(input, "META_START");
    }

    #[test]
    fn test_oem_state_vector_error_counts() {
        // Less than 6
        let mut input = "2023-01-01T00:00:00 1 2 3 4 5\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());

        // Between 6 and 9
        let mut input = "2023-01-01T00:00:00 1 2 3 4 5 6 7\n";
        assert!(parse_state_vector_line.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_data_invalid_order() {
        let kvn = r#"2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1.0
2.0 3.0
4.0 5.0 6.0
7.0 8.0 9.0 10.0
11.0 12.0 13.0 14.0 15.0
16.0 17.0 18.0 19.0 20.0 21.0
COVARIANCE_STOP
2023-01-01T00:01:00 1000 2000 3000 1.0 2.0 3.0
"#;
        let mut input = kvn;
        // Should error because state vector appears AFTER covariance
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_data_empty_sv() {
        let mut input = "COVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\n1\n2 3\n4 5 6\n7 8 9 10\n11 12 13 14 15\n16 17 18 19 20 21\nCOVARIANCE_STOP\n";
        // Should error because no state vectors
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_oem_data_interleaved_comments() {
        let kvn = r#"COMMENT C_GLOBAL
2023-01-01T00:00:00 1 2 3 4 5 6
COMMENT C_DATA
2023-01-01T00:01:00 1 2 3 4 5 6
COVARIANCE_START
EPOCH = 2023-01-01T00:00:00
1
2 3
4 5 6
7 8 9 10
11 12 13 14 15
16 17 18 19 20 21
COVARIANCE_STOP
COMMENT C_TRAILING
"#;
        let mut input = kvn;
        let data = oem_data.parse_next(&mut input).unwrap();
        assert!(data.comment.contains(&"C_GLOBAL".to_string()));
        assert!(data.comment.contains(&"C_DATA".to_string()));
        assert!(data.comment.contains(&"C_TRAILING".to_string()));
    }

    #[test]
    fn test_oem_parser_error_paths() {
        // Missing EPOCH in COVARIANCE
        let mut input = "COVARIANCE_START\n1\nCOVARIANCE_STOP";
        assert!(oem_data.parse_next(&mut input).is_err());

        // Unexpected key in COVARIANCE
        let mut input =
            "COVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\nINVALID_KEY = VAL\n1\nCOVARIANCE_STOP";
        assert!(oem_data.parse_next(&mut input).is_err());

        // State vector after covariance (triggering cut_err)
        let mut input = "2023-01-01T00:00:00 1 2 3 4 5 6\nCOVARIANCE_START\nEPOCH = 2023-01-01T00:00:00\n1\n2 3\n4 5 6\n7 8 9 10\n11 12 13 14 15\n16 17 18 19 20 21\nCOVARIANCE_STOP\n2023-01-01T00:01:00 1 2 3 4 5 6";
        assert!(oem_data.parse_next(&mut input).is_err());
    }
}
