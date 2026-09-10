// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OEM (Orbit Ephemeris Message).
//!
//! This module implements KVN parsing for OEM using winnow parser combinators.

use super::{
    absolute_epoch_error, validate_within_path, Oem, OemBody, OemCovarianceMatrix, OemData,
    OemEpochRangeCheck, OemMetadata, OemSegment,
};
use crate::common::StateVectorAcc;
use crate::error::{InternalParserError, Result, ValidationError};
use crate::kvn::parser::*;
use crate::kvn::ser::{KvnWriter, OdmFloat};
use crate::parse_block;
use crate::traits::{ToKvn, Validate};
use crate::types::*;
use std::num::NonZeroU32;
use winnow::ascii::space1;
use winnow::combinator::preceded;
use winnow::error::{AddContext, ErrMode};
use winnow::prelude::*;
use winnow::stream::Offset;

pub(super) fn to_string(oem: &Oem) -> Result<String> {
    (|| {
        crate::generation::validate_output_version(
            crate::validation::MessageKind::Oem,
            &oem.version,
            crate::generation::OutputFormat::Kvn,
        )?;
        let records = oem.body.segment.iter().fold(0usize, |total, segment| {
            total
                .saturating_add(segment.data.state_vector.len())
                .saturating_add(segment.data.covariance_matrix.len().saturating_mul(7))
        });
        let mut writer = KvnWriter::with_capacity(records.saturating_mul(150).saturating_add(4096));
        oem.write_validated_kvn(&mut writer)?;
        writer.finish_checked()
    })()
    .map_err(|error: crate::error::CcsdsNdmError| {
        error.with_generation_context(
            crate::validation::MessageKind::Oem,
            crate::error::DiagnosticNotation::Kvn,
            &oem.version,
        )
    })
}

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
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_calendar_epoch,
        "TIME_SYSTEM" => time_system: kv_string,
        "START_TIME" => start_time: kv_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_epoch,
        "STOP_TIME" => stop_time: kv_epoch,
        "INTERPOLATION" => interpolation: kv_string,
        "INTERPOLATION_DEGREE" => val: kv_u32 => {
            let nz = NonZeroU32::new(val).ok_or_else(|| {
                cut_err(input, "positive integer")
            })?;
            interpolation_degree = Some(InterpolationDegree(nz));
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

/// True when only a line ending, or the end of input, remains of the current record.
fn at_record_end(input: &str) -> bool {
    input.is_empty() || input.starts_with('\n') || input.starts_with('\r')
}

fn parse_odm_f64(input: &mut &str) -> KvnResult<f64> {
    let token = till_space_or_eol.parse_next(input)?;
    if !valid_ccsds_number(token) {
        return Err(cut_err(input, "Invalid ODM number"));
    }
    fast_float::parse(token).map_err(|_| cut_err(input, "Invalid ODM number"))
}

/// Parses a raw state vector line.
/// Format: EPOCH X Y Z X_DOT Y_DOT Z_DOT [X_DDOT Y_DDOT Z_DDOT]
fn parse_state_vector_line(input: &mut &str) -> KvnResult<StateVectorAcc> {
    let epoch = preceded(ws, kv_epoch_token).parse_next(input)?;

    let mut floats = [0.0f64; 9];
    let mut count = 0;

    for f in &mut floats {
        let checkpoint = input.checkpoint();
        // Consume the separator on its own so that a missing separator (the record ended) stays
        // distinguishable from a separator followed by a malformed token (a committed error).
        let separator: KvnResult<&str> = space1.parse_next(input);
        // Horizontal whitespace before the line ending is padding, not a missing component.
        if separator.is_err() || at_record_end(input) {
            input.reset(&checkpoint);
            break;
        }
        *f = parse_odm_f64.parse_next(input)?;
        count += 1;
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

    // An ephemeris record occupies exactly one line, so only padding may follow its components.
    // Without this anchor, leftover tokens are re-read as another record, which both accepts
    // several records packed onto one line.
    ws.parse_next(input)?;
    if !at_record_end(input) {
        return Err(cut_err(
            input,
            "State vector must have either 6 or 9 components",
        ));
    }
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
    let next = input.trim_start_matches([' ', '\t']);
    if next
        .strip_prefix("COV_REF_FRAME")
        .is_some_and(|rest| rest.starts_with([' ', '\t', '=']))
    {
        key_token.parse_next(input)?;
        cov_ref_frame = Some(kv_string.parse_next(input)?);
    }

    // Parse 6 lines of raw covariance data (1, 2, 3, 4, 5, 6 elements per line)
    let mut floats = [0.0; 21];
    let mut offset = 0;

    for expected_count in 1..=6 {
        blank_lines.parse_next(input)?;
        floats[offset] = preceded(ws, parse_odm_f64).parse_next(input)?;
        for value in &mut floats[offset + 1..offset + expected_count] {
            *value = preceded(space1, parse_odm_f64).parse_next(input)?;
        }
        offset += expected_count;
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

/// Parses the OEM data section (state vectors and optional covariance matrices).
pub fn oem_data(input: &mut &str) -> KvnResult<OemData> {
    let mut data = OemData {
        comment: Vec::new(),
        state_vector: Vec::new(),
        covariance_matrix: Vec::new(),
    };

    let mut covariance_started = false;

    loop {
        let _ = ws.parse_next(input);
        if input.is_empty() || at_block_start("META", input) {
            break;
        }

        let checkpoint = input.checkpoint();
        // Fast-path: state vectors do not need comment collection and its Vec allocation.
        let first_char = input.chars().next();
        let result: KvnResult<()> = (|| {
            if matches!(first_char, Some('0'..='9' | '-' | '+')) {
                let sv = parse_state_vector_line.parse_next(input)?;
                if covariance_started {
                    return Err(cut_err(
                        input,
                        "State vectors cannot appear after covariance matrix block",
                    ));
                }
                data.state_vector.push(sv);
                Ok(())
            } else {
                let comments = collect_comments.parse_next(input)?;
                if input.is_empty() || at_block_start("META", input) {
                    if comments.is_empty() {
                        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
                    }
                    data.comment.extend(comments);
                    Ok(())
                } else if at_block_start("COVARIANCE", input) {
                    expect_block_start("COVARIANCE").parse_next(input)?;
                    let mut matrices = parse_covariance_block.parse_next(input)?;
                    expect_block_end("COVARIANCE").parse_next(input)?;
                    covariance_started = true;

                    if let Some(first) = matrices.get_mut(0) {
                        first.comment.splice(0..0, comments);
                    } else {
                        data.comment.extend(comments);
                    }
                    data.covariance_matrix.append(&mut matrices);
                    Ok(())
                } else {
                    let sv = parse_state_vector_line.parse_next(input)?;
                    if covariance_started {
                        return Err(cut_err(
                            input,
                            "State vectors cannot appear after covariance matrix block",
                        ));
                    }
                    data.comment.extend(comments);
                    data.state_vector.push(sv);
                    Ok(())
                }
            }
        })();

        match result {
            Ok(()) => continue,
            Err(e) => {
                if e.is_backtrack() || input.offset_from(&checkpoint) == 0 {
                    input.reset(&checkpoint);
                    break;
                }
                return Err(e);
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
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_oem.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

/// Reject a record wider than the normative KVN line limit.
fn line_length_error(
    name: &'static str,
    line_len: usize,
    path: impl FnOnce() -> String,
) -> Result<()> {
    if line_len <= 254 {
        return Ok(());
    }
    Err(ValidationError::OutOfRange {
        name: name.into(),
        value: line_len.to_string(),
        expected: "a KVN line no longer than 254 characters".into(),
        line: None,
    }
    .at_path(path())
    .into())
}

/// Split the 21 lower-triangular values into the normative 1/2/3/4/5/6-element rows.
fn covariance_rows<T>(values: &[T; 21]) -> [&[T]; 6] {
    [
        &values[0..1],
        &values[1..3],
        &values[3..6],
        &values[6..10],
        &values[10..15],
        &values[15..21],
    ]
}

impl Oem {
    /// Run the generation pass against a discarding sink.
    ///
    /// `write_validated_kvn` is the single description of the OEM KVN layout, so validating
    /// through it keeps the checks and the emitted bytes from drifting apart. Streaming callers
    /// preflight here first so a rejected message never reaches the caller's sink half-written.
    pub(crate) fn validate_kvn_generation(&self) -> Result<()> {
        let mut sink = std::io::sink();
        let mut writer = KvnWriter::from_io(&mut sink);
        self.write_validated_kvn(&mut writer)?;
        writer.finish_io()
    }

    /// Write the complete OEM KVN document, validating each record as it is emitted.
    fn write_validated_kvn(&self, writer: &mut KvnWriter<'_>) -> Result<()> {
        fn text(field: &'static str, value: &str, path: String) -> Result<()> {
            if !value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                return Err(ValidationError::InvalidValue {
                    field: field.into(),
                    value: value.into(),
                    expected: "printable ASCII characters and blanks".into(),
                    line: None,
                }
                .at_path(path)
                .into());
            }
            // `KvnWriter::write_pair` left-pads the key to 20 columns and adds " = ".
            let line_len = field.len().max(20) + 3 + value.len();
            if line_len > 254 {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: line_len.to_string(),
                    expected: "a KVN line no longer than 254 characters".into(),
                    line: None,
                }
                .at_path(path)
                .into());
            }
            Ok(())
        }
        fn comments(values: &[String], path: String) -> Result<()> {
            for value in values {
                if let Some(error) = crate::validation::kvn_comment_error(value) {
                    return Err(error.at_path(path.clone()).into());
                }
            }
            Ok(())
        }
        crate::versioning::validate_root(
            crate::validation::MessageKind::Oem,
            &self.id,
            &self.version,
        )?;
        crate::versioning::validate_oem_edition(self)?;
        self.header.validate()?;
        validate_within_path(self.body.validate_identity(), || "body".into())?;
        validate_within_path(self.body.validate_useable_spans(), || "body".into())?;

        comments(&self.header.comment, "header.comment".into())?;
        if let Some(value) = &self.header.classification {
            text("CLASSIFICATION", value, "header.classification".into())?;
        }
        text(
            "ORIGINATOR",
            &self.header.originator,
            "header.originator".into(),
        )?;
        if let Some(value) = &self.header.message_id {
            text("MESSAGE_ID", value, "header.message_id".into())?;
        }
        writer.write_pair("CCSDS_OEM_VERS", &self.version);
        self.header.write_kvn(writer);
        for (segment_index, segment) in self.body.segment.iter().enumerate() {
            let base = format!("body.segment[{segment_index}]");
            let metadata = &segment.metadata;
            validate_within_path(metadata.validate(), || format!("{base}.metadata").into())?;
            validate_within_path(segment.data.validate_presence(), || {
                format!("{base}.data").into()
            })?;
            let mut epoch_range = OemEpochRangeCheck::new(metadata);
            comments(&metadata.comment, format!("{base}.metadata.comment"))?;
            for (field, value, member) in [
                ("OBJECT_NAME", metadata.object_name.as_str(), "object_name"),
                ("OBJECT_ID", metadata.object_id.as_str(), "object_id"),
                ("CENTER_NAME", metadata.center_name.as_str(), "center_name"),
                ("REF_FRAME", metadata.ref_frame.as_str(), "ref_frame"),
                ("TIME_SYSTEM", metadata.time_system.as_str(), "time_system"),
            ] {
                text(field, value, format!("{base}.metadata.{member}"))?;
            }
            if let Some(value) = &metadata.interpolation {
                text(
                    "INTERPOLATION",
                    value,
                    format!("{base}.metadata.interpolation"),
                )?;
            }
            comments(&segment.data.comment, format!("{base}.data.comment"))?;
            writer.write_section("META_START");
            metadata.write_kvn(writer);
            writer.write_section("META_STOP");
            writer.write_comments(&segment.data.comment);
            writer.write_empty();
            for (state_index, state) in segment.data.state_vector.iter().enumerate() {
                // The fused generation pass below applies the stronger OEM absolute/range/order
                // epoch checks and checks every numeric component through `OdmFloat`. Calling the
                // generic state validator here would scan the same epoch and values a second time.
                let mut epoch_error = None;
                epoch_range.state(state_index, state, &mut |error| {
                    epoch_error.get_or_insert(error);
                });
                if let Some(error) = epoch_error {
                    return Err(error.within_path(base.clone()).into());
                }
                let acceleration_count = [&state.x_ddot, &state.y_ddot, &state.z_ddot]
                    .into_iter()
                    .filter(|value| value.is_some())
                    .count();
                if acceleration_count != 0 && acceleration_count != 3 {
                    return Err(ValidationError::InvalidValue {
                        field: "X_DDOT/Y_DDOT/Z_DDOT".into(),
                        value: format!("{acceleration_count} acceleration components present"),
                        expected: "either no acceleration components or all three for OEM KVN"
                            .into(),
                        line: None,
                    }
                    .at_path(format!("{base}.data.state_vector[{state_index}]"))
                    .into());
                }
                let required_values = [
                    ("X", state.x.value, "x"),
                    ("Y", state.y.value, "y"),
                    ("Z", state.z.value, "z"),
                    ("X_DOT", state.x_dot.value, "x_dot"),
                    ("Y_DOT", state.y_dot.value, "y_dot"),
                    ("Z_DOT", state.z_dot.value, "z_dot"),
                ];
                let acceleration_values = [
                    ("X_DDOT", &state.x_ddot, "x_ddot"),
                    ("Y_DDOT", &state.y_ddot, "y_ddot"),
                    ("Z_DDOT", &state.z_ddot, "z_ddot"),
                ];
                writer.try_write_validated_line(|line, line_start| -> Result<()> {
                    line.push_str(state.epoch.as_str());
                    for (field, value, member) in required_values {
                        line.push(' ');
                        if !OdmFloat::write_if_valid(value, line) {
                            return Err(crate::validation::unrepresentable_number(
                                field,
                                value,
                                format!("{base}.data.state_vector[{state_index}].{member}"),
                            ));
                        }
                    }
                    for (field, value, member) in
                        acceleration_values
                            .into_iter()
                            .flat_map(|(field, value, member)| {
                                value.as_ref().map(|value| (field, value, member))
                            })
                    {
                        line.push(' ');
                        if !OdmFloat::write_if_valid(value.value, line) {
                            return Err(crate::validation::unrepresentable_number(
                                field,
                                value.value,
                                format!("{base}.data.state_vector[{state_index}].{member}"),
                            ));
                        }
                    }
                    line_length_error("stateVector", line.len() - line_start, || {
                        format!("{base}.data.state_vector[{state_index}]")
                    })
                })?;
            }
            if !segment.data.covariance_matrix.is_empty() {
                writer.write_empty();
                writer.write_section("COVARIANCE_START");
                for (covariance_index, covariance) in
                    segment.data.covariance_matrix.iter().enumerate()
                {
                    if covariance_index > 0 && !covariance.comment.is_empty() {
                        return Err(ValidationError::InvalidValue {
                            field: "COMMENT".into(),
                            value: covariance.comment.join(" | "),
                            expected:
                                "comments attached only to the first covariance matrix for OEM KVN"
                                    .into(),
                            line: None,
                        }
                        .at_path(format!(
                            "{base}.data.covariance_matrix[{covariance_index}].comment"
                        ))
                        .into());
                    }
                    comments(
                        &covariance.comment,
                        format!("{base}.data.covariance_matrix[{covariance_index}].comment"),
                    )?;
                    writer.write_comments(&covariance.comment);
                }
            }
            for (covariance_index, covariance) in segment.data.covariance_matrix.iter().enumerate()
            {
                // Numeric finiteness is subsumed by the representability checks used for every
                // emitted matrix value. Keep the non-numeric absolute-epoch rule explicitly.
                if let Some(error) = absolute_epoch_error(&covariance.epoch, "EPOCH") {
                    return Err(error
                        .at_path(format!(
                            "{base}.data.covariance_matrix[{covariance_index}].epoch"
                        ))
                        .into());
                }
                let mut epoch_error = None;
                epoch_range.covariance(covariance_index, covariance, &mut |error| {
                    epoch_error.get_or_insert(error);
                });
                if let Some(error) = epoch_error {
                    return Err(error.within_path(base.clone()).into());
                }
                if let Some(value) = &covariance.cov_ref_frame {
                    text(
                        "COV_REF_FRAME",
                        value,
                        format!("{base}.data.covariance_matrix[{covariance_index}].cov_ref_frame"),
                    )?;
                }
                writer.write_pair("EPOCH", covariance.epoch);
                if let Some(value) = &covariance.cov_ref_frame {
                    writer.write_pair("COV_REF_FRAME", value);
                }
                let values = covariance.values();
                for (row_index, row) in covariance_rows(&values).into_iter().enumerate() {
                    writer.try_write_validated_line(|line, line_start| -> Result<()> {
                        for (value_index, (field, value)) in row.iter().enumerate() {
                            if value_index > 0 {
                                line.push(' ');
                            }
                            let path = || {
                                format!(
                                    "{base}.data.covariance_matrix[{covariance_index}].{}",
                                    field.to_ascii_lowercase()
                                )
                            };
                            if let Some(error) =
                                crate::common::covariance_value_error(field, *value)
                            {
                                return Err(error.at_path(path()).into());
                            }
                            if !OdmFloat::write_if_valid(*value, line) {
                                return Err(crate::validation::unrepresentable_number(
                                    field,
                                    *value,
                                    path(),
                                ));
                            }
                        }
                        line_length_error("covariance row", line.len() - line_start, || {
                            format!(
                                "{base}.data.covariance_matrix[{covariance_index}].row[{}]",
                                row_index + 1
                            )
                        })
                    })?;
                }
            }
            if !segment.data.covariance_matrix.is_empty() {
                writer.write_section("COVARIANCE_STOP");
            }
        }
        Ok(())
    }

    /// Strictly parse and validate an OEM KVN document.
    pub(crate) fn from_kvn_strict(kvn: &str) -> Result<Self> {
        let source_edition = kvn.split(['\r', '\n']).find_map(|line| {
            line.split_once('=')
                .filter(|(key, _)| key.trim() == "CCSDS_OEM_VERS")
                .map(|(_, value)| value.trim())
        });
        (|| {
            let normalized = crate::kvn::normalize_line_endings(kvn);
            validate_syntax(&normalized)?;
            let oem = Self::from_kvn_str(&normalized)?;
            crate::traits::Validate::validate(&oem)?;
            Ok(oem)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Oem,
                crate::error::DiagnosticNotation::Kvn,
                kvn,
                source_edition,
            )
        })
    }
}

impl ToKvn for Oem {
    fn validate_kvn(&self) -> Result<()> {
        self.validate_kvn_generation()
    }

    fn write_kvn(&self, writer: &mut KvnWriter) {
        // Callers reach this only after `validate_kvn` has run the same pass against a sink, so
        // the layout cannot fail here; the writer records any late I/O or lexical fault itself.
        let _ = self.write_validated_kvn(writer);
    }
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
        writer.write_pair("START_TIME", self.start_time);
        if let Some(v) = &self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = &self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        writer.write_pair("STOP_TIME", self.stop_time);
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = &self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

/// Validate the OEM KVN record structure.
///
/// Expects `kvn` to have gone through [`crate::kvn::normalize_line_endings`], so every remaining
/// carriage return is part of a CRLF pair.
pub(super) fn validate_syntax(kvn: &str) -> Result<()> {
    use crate::error::{CcsdsNdmError, FormatError};

    fn invalid(line: usize, offset: usize, message: impl AsRef<str>) -> CcsdsNdmError {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(
            crate::error::KvnParseError {
                line,
                column: 1,
                message: message.as_ref().to_owned(),
                contexts: vec!["strict OEM KVN"],
                offset,
            },
        ))))
    }

    fn header_rank(key: &str) -> Option<u8> {
        Some(match key {
            "CCSDS_OEM_VERS" => 0,
            "CLASSIFICATION" => 1,
            "CREATION_DATE" => 2,
            "ORIGINATOR" => 3,
            "MESSAGE_ID" => 4,
            _ => return None,
        })
    }

    fn metadata_rank(key: &str) -> Option<u8> {
        Some(match key {
            "OBJECT_NAME" => 1,
            "OBJECT_ID" => 2,
            "CENTER_NAME" => 3,
            "REF_FRAME" => 4,
            "REF_FRAME_EPOCH" => 5,
            "TIME_SYSTEM" => 6,
            "START_TIME" => 7,
            "USEABLE_START_TIME" => 8,
            "USEABLE_STOP_TIME" => 9,
            "STOP_TIME" => 10,
            "INTERPOLATION" => 11,
            "INTERPOLATION_DEGREE" => 12,
            _ => return None,
        })
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Phase {
        Header,
        Metadata,
        Ephemeris,
        Covariance,
    }

    let mut phase = Phase::Header;
    let mut header_rank_seen = None;
    let mut metadata_rank_seen = 0u8;
    let mut segments = 0usize;
    let mut state_records = 0usize;
    let mut covariance_records = 0usize;
    let mut covariance_row = 0usize;
    let mut covariance_epoch_seen = false;
    let mut covariance_frame_seen = false;
    let mut covariance_closed = false;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.len() > 254 {
            return Err(invalid(
                line_number,
                offset,
                "line exceeds the normative 254-character limit",
            ));
        }
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return Err(invalid(
                line_number,
                offset,
                "non-printable or non-ASCII character",
            ));
        }
        let line = line.trim_start();
        if line.trim_end().is_empty() {
            offset += raw_line.len() + 1;
            continue;
        }
        if line == "COMMENT" || line.starts_with("COMMENT ") {
            let allowed = match phase {
                Phase::Header => header_rank_seen == Some(0),
                Phase::Metadata => metadata_rank_seen == 0,
                Phase::Ephemeris => state_records == 0,
                Phase::Covariance => !covariance_epoch_seen && covariance_row == 0,
            };
            if !allowed {
                return Err(invalid(
                    line_number,
                    offset,
                    "COMMENT is not at the beginning of an allowed OEM block",
                ));
            }
            offset += raw_line.len() + 1;
            continue;
        }
        let line = line.trim_end();

        match line {
            "META_START" => {
                let allowed = match phase {
                    Phase::Header => header_rank_seen.is_some(),
                    Phase::Ephemeris => state_records > 0,
                    _ => false,
                };
                if !allowed {
                    return Err(invalid(line_number, offset, "unexpected META_START"));
                }
                phase = Phase::Metadata;
                metadata_rank_seen = 0;
                state_records = 0;
                covariance_records = 0;
                covariance_closed = false;
                segments += 1;
            }
            "META_STOP" => {
                if phase != Phase::Metadata {
                    return Err(invalid(line_number, offset, "unexpected META_STOP"));
                }
                phase = Phase::Ephemeris;
            }
            "COVARIANCE_START" => {
                if phase != Phase::Ephemeris || state_records == 0 || covariance_closed {
                    return Err(invalid(
                        line_number,
                        offset,
                        "COVARIANCE_START must follow ephemeris records",
                    ));
                }
                phase = Phase::Covariance;
                covariance_row = 0;
                covariance_epoch_seen = false;
                covariance_frame_seen = false;
            }
            "COVARIANCE_STOP" => {
                if phase != Phase::Covariance || covariance_records == 0 || covariance_row != 6 {
                    return Err(invalid(
                        line_number,
                        offset,
                        "COVARIANCE_STOP must follow a complete covariance matrix",
                    ));
                }
                phase = Phase::Ephemeris;
                covariance_closed = true;
            }
            _ if line.contains('=') => {
                let (key, _) = line
                    .split_once('=')
                    .expect("the match guard found an equals");
                let key = key.trim();
                match phase {
                    Phase::Header => {
                        let rank = header_rank(key).ok_or_else(|| {
                            invalid(line_number, offset, "unknown OEM header keyword")
                        })?;
                        if header_rank_seen.is_some_and(|previous| rank <= previous) {
                            return Err(invalid(
                                line_number,
                                offset,
                                "duplicate or out-of-order OEM header keyword",
                            ));
                        }
                        if header_rank_seen.is_none() && rank != 0 {
                            return Err(invalid(
                                line_number,
                                offset,
                                "CCSDS_OEM_VERS must be the first record",
                            ));
                        }
                        header_rank_seen = Some(rank);
                    }
                    Phase::Metadata => {
                        let rank = metadata_rank(key).ok_or_else(|| {
                            invalid(line_number, offset, "unknown OEM metadata keyword")
                        })?;
                        if rank <= metadata_rank_seen {
                            return Err(invalid(
                                line_number,
                                offset,
                                "duplicate or out-of-order OEM metadata keyword",
                            ));
                        }
                        metadata_rank_seen = rank;
                    }
                    Phase::Covariance => match key {
                        "EPOCH" if covariance_row == 0 || covariance_row == 6 => {
                            covariance_row = 0;
                            covariance_epoch_seen = true;
                            covariance_frame_seen = false;
                            covariance_records += 1;
                        }
                        "COV_REF_FRAME"
                            if covariance_epoch_seen
                                && covariance_row == 0
                                && !covariance_frame_seen =>
                        {
                            covariance_frame_seen = true;
                        }
                        _ => {
                            return Err(invalid(
                                line_number,
                                offset,
                                "unexpected covariance keyword",
                            ));
                        }
                    },
                    Phase::Ephemeris => {
                        return Err(invalid(
                            line_number,
                            offset,
                            "assignments are not allowed in OEM ephemeris records",
                        ));
                    }
                }
            }
            _ => match phase {
                Phase::Ephemeris if !covariance_closed => {
                    state_records += 1;
                }
                Phase::Covariance if covariance_epoch_seen && covariance_row < 6 => {
                    covariance_row += 1;
                }
                _ => return Err(invalid(line_number, offset, "unexpected OEM record")),
            },
        }
        offset += raw_line.len() + 1;
    }

    if segments == 0 || phase != Phase::Ephemeris || state_records == 0 {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "incomplete OEM document",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const METADATA: &str = r#"OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
REF_FRAME_EPOCH = 2000-01-01T00:00:00
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T01:00:00
USEABLE_STOP_TIME = 2023-01-01T23:00:00
STOP_TIME = 2023-01-02T00:00:00
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 5
META_STOP
"#;

    const COVARIANCE: &str = r#"EPOCH = 2023-01-01T00:00:00
COV_REF_FRAME = RTN
1
2 3
4 5 6
7 8 9 10
11 12 13 14 15
16 17 18 19 20 21
"#;

    #[test]
    fn parses_the_oem_version_line() {
        let mut input = "CCSDS_OEM_VERS = 3.0\n";
        assert_eq!(oem_version.parse_next(&mut input).unwrap(), "3.0");
    }

    #[test]
    fn parses_state_vectors_with_optional_acceleration() {
        for (input, has_acceleration) in [
            ("2023-01-01T00:00:00 1 2 3 4 5 6\n", false),
            ("2023-01-01T00:00:00 1 2 3 4 5 6 7 8 9\n", true),
        ] {
            let mut input = input;
            let state = parse_state_vector_line.parse_next(&mut input).unwrap();
            assert_eq!(state.x.value, 1.0);
            assert_eq!(state.x_dot.value, 4.0);
            assert_eq!(state.x_ddot.is_some(), has_acceleration);
        }
    }

    #[test]
    fn rejects_invalid_state_vector_records() {
        for input in [
            "2023-01-01T00:00:00 1 2 3 4 5\n",
            "2023-01-01T00:00:00 1 2 3 4 5 6 7\n",
            "INVALID_EPOCH 1 2 3 4 5 6\n",
            "2023-01-01T00:00:00 BAD 2 3 4 5 6\n",
            "2023-01-01T00:00:00 1 2 3 4 5 6 extra\n",
        ] {
            let mut input = input;
            assert!(parse_state_vector_line.parse_next(&mut input).is_err());
        }
    }

    #[test]
    fn parses_covariance_matrices_and_rejects_malformed_rows() {
        let mut input = COVARIANCE;
        let matrix = parse_covariance_matrix.parse_next(&mut input).unwrap();
        assert_eq!(matrix.cov_ref_frame.as_deref(), Some("RTN"));
        assert_eq!(matrix.cx_x.value, 1.0);
        assert_eq!(matrix.cz_dot_z_dot.value, 21.0);

        for input in [
            "EPOCH = INVALID\n",
            "EPOCH = 2023-01-01T00:00:00\n1\n2\n",
            "EPOCH = 2023-01-01T00:00:00\n1\n2 3\n4 5 6\n7 8 9 10\n11 12 13 BAD\n16 17 18 19 20 21\n",
        ] {
            let mut input = input;
            assert!(parse_covariance_matrix.parse_next(&mut input).is_err());
        }
    }

    #[test]
    fn parses_data_comments_blank_lines_and_covariance() {
        let input = format!(
            "COMMENT data comment\n\
             2023-01-01T00:00:00 1 2 3 4 5 6\n\
             \n\
             COVARIANCE_START\nCOMMENT covariance comment\n{COVARIANCE}COVARIANCE_STOP\n"
        );
        let mut input = input.as_str();
        let data = oem_data.parse_next(&mut input).unwrap();

        assert_eq!(data.comment, vec!["data comment"]);
        assert_eq!(data.state_vector.len(), 1);
        assert_eq!(data.covariance_matrix.len(), 1);
        assert_eq!(data.covariance_matrix[0].comment, ["covariance comment"]);
    }

    #[test]
    fn rejects_unknown_data_keys_and_state_vectors_after_covariance() {
        for input in ["UNKNOWN_KEY = value\n", "X_KEY = value\n"] {
            let mut input = input;
            assert!(oem_data.parse_next(&mut input).is_err());
        }

        let input = format!(
            "2023-01-01T00:00:00 1 2 3 4 5 6\n\
             COVARIANCE_START\n{COVARIANCE}COVARIANCE_STOP\n\
             2023-01-01T00:01:00 1 2 3 4 5 6\n"
        );
        let mut input = input.as_str();
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn rejects_data_without_state_vectors() {
        let input = format!("COVARIANCE_START\n{COVARIANCE}COVARIANCE_STOP\n");
        let mut input = input.as_str();
        assert!(oem_data.parse_next(&mut input).is_err());
    }

    #[test]
    fn parses_metadata_and_reports_each_missing_required_field() {
        let mut input = METADATA;
        let metadata = oem_metadata.parse_next(&mut input).unwrap();
        assert_eq!(metadata.object_name, "SAT");
        assert_eq!(
            metadata.interpolation_degree.map(|degree| degree.0.get()),
            Some(5)
        );
        assert!(metadata.ref_frame_epoch.is_some());

        for line in [
            "OBJECT_NAME = SAT\n",
            "OBJECT_ID = 1\n",
            "CENTER_NAME = EARTH\n",
            "REF_FRAME = GCRF\n",
            "TIME_SYSTEM = UTC\n",
            "START_TIME = 2023-01-01T00:00:00\n",
            "STOP_TIME = 2023-01-02T00:00:00\n",
        ] {
            let input = METADATA.replace(line, "");
            let mut input = input.as_str();
            assert!(
                oem_metadata.parse_next(&mut input).is_err(),
                "missing {line:?}"
            );
        }
    }

    #[test]
    fn rejects_invalid_metadata_values() {
        for input in [
            METADATA.replace("INTERPOLATION_DEGREE = 5", "INTERPOLATION_DEGREE = 0"),
            METADATA.replace(
                "REF_FRAME_EPOCH = 2000-01-01T00:00:00",
                "REF_FRAME_EPOCH = INVALID",
            ),
            METADATA.replace("META_STOP", "UNKNOWN_KEY = value\nMETA_STOP"),
        ] {
            let mut input = input.as_str();
            assert!(oem_metadata.parse_next(&mut input).is_err());
        }
    }

    #[test]
    fn parses_the_shared_odm_header_and_rejects_bad_dates() {
        let mut input =
            "COMMENT C1\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = ME\nMETA_START";
        let header = odm_header.parse_next(&mut input).unwrap();
        assert_eq!(header.comment, vec!["C1"]);
        assert_eq!(input, "META_START");

        for value in ["INVALID", "123.5", "2023-02-29T00:00:00"] {
            let source = format!("CREATION_DATE = {value}\nORIGINATOR = TEST\n");
            let mut input = source.as_str();
            assert!(odm_header.parse_next(&mut input).is_err());
        }
    }

    #[test]
    fn rejects_invalid_block_boundaries() {
        assert!(!at_block_start("META", "META_START_EXTRA"));
        assert!(!at_block_end("META", "META_STOP_EXTRA"));
        assert!(!at_block_end("META", "META_END_EXTRA"));
    }
}
