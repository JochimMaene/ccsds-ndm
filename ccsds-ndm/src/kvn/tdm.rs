// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for TDM (Tracking Data Message).
//!
//! This module implements KVN parsing for TDM using winnow parser combinators.

use crate::kvn::parser::*;
use crate::messages::tdm::{
    Tdm, TdmBody, TdmData, TdmHeader, TdmMetadata, TdmObservation, TdmObservationData, TdmSegment,
};
use crate::types::*;
use std::str::FromStr;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helper: Check if key belongs to TDM Metadata section
//----------------------------------------------------------------------

fn is_tdm_metadata_key(key: &str) -> bool {
    matches!(
        key,
        "TRACK_ID"
            | "DATA_TYPES"
            | "TIME_SYSTEM"
            | "START_TIME"
            | "STOP_TIME"
            | "PARTICIPANT_1"
            | "PARTICIPANT_2"
            | "PARTICIPANT_3"
            | "PARTICIPANT_4"
            | "PARTICIPANT_5"
            | "MODE"
            | "PATH"
            | "PATH_1"
            | "PATH_2"
            | "TRANSMIT_BAND"
            | "RECEIVE_BAND"
            | "TURNAROUND_NUMERATOR"
            | "TURNAROUND_DENOMINATOR"
            | "TIMETAG_REF"
            | "INTEGRATION_INTERVAL"
            | "INTEGRATION_REF"
            | "FREQ_OFFSET"
            | "RANGE_MODE"
            | "RANGE_MODULUS"
            | "RANGE_UNITS"
            | "ANGLE_TYPE"
            | "REFERENCE_FRAME"
            | "INTERPOLATION"
            | "INTERPOLATION_DEGREE"
            | "DOPPLER_COUNT_BIAS"
            | "DOPPLER_COUNT_SCALE"
            | "DOPPLER_COUNT_ROLLOVER"
            | "TRANSMIT_DELAY_1"
            | "TRANSMIT_DELAY_2"
            | "TRANSMIT_DELAY_3"
            | "TRANSMIT_DELAY_4"
            | "TRANSMIT_DELAY_5"
            | "RECEIVE_DELAY_1"
            | "RECEIVE_DELAY_2"
            | "RECEIVE_DELAY_3"
            | "RECEIVE_DELAY_4"
            | "RECEIVE_DELAY_5"
            | "DATA_QUALITY"
            | "CORRECTION_ANGLE_1"
            | "CORRECTION_ANGLE_2"
            | "CORRECTION_DOPPLER"
            | "CORRECTION_MAG"
            | "CORRECTION_RANGE"
            | "CORRECTION_RCS"
            | "CORRECTION_RECEIVE"
            | "CORRECTION_TRANSMIT"
            | "CORRECTION_ABERRATION_YEARLY"
            | "CORRECTION_ABERRATION_DIURNAL"
            | "CORRECTIONS_APPLIED"
            | "EPHEMERIS_NAME_1"
            | "EPHEMERIS_NAME_2"
            | "EPHEMERIS_NAME_3"
            | "EPHEMERIS_NAME_4"
            | "EPHEMERIS_NAME_5"
    )
}

fn is_tdm_header_key(key: &str) -> bool {
    matches!(key, "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID")
}

//----------------------------------------------------------------------
// TDM Version Parser
//----------------------------------------------------------------------

pub fn tdm_version(input: &mut &str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_TDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// TDM Header Parser
//----------------------------------------------------------------------

pub fn tdm_header(input: &mut &str) -> ModalResult<TdmHeader> {
    let mut comment = Vec::new();
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_tdm_header_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CREATION_DATE" => {
                        creation_date = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORIGINATOR" => originator = Some(v.to_string()),
                    "MESSAGE_ID" => message_id = Some(v.to_string()),
                    _ => {}
                }
            }
            _ => break,
        }
    }

    Ok(TdmHeader {
        comment,
        creation_date: creation_date.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CREATION_DATE")),
            ))
        })?,
        originator: originator.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("ORIGINATOR")),
            ))
        })?,
        message_id,
    })
}

//----------------------------------------------------------------------
// TDM Metadata Parser
//----------------------------------------------------------------------

pub fn tdm_metadata(input: &mut &str) -> ModalResult<TdmMetadata> {
    expect_block_start("META").parse_next(input)?;

    let mut meta = TdmMetadata::default();

    loop {
        if at_block_end("META", input) {
            expect_block_end("META").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        meta.comment.extend(comments);

        if at_block_end("META", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_tdm_metadata_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "TRACK_ID" => meta.track_id = Some(v.to_string()),
                    "DATA_TYPES" => meta.data_types = Some(v.to_string()),
                    "TIME_SYSTEM" => meta.time_system = v.to_string(),
                    "START_TIME" => {
                        meta.start_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "STOP_TIME" => {
                        meta.stop_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "PARTICIPANT_1" => meta.participant_1 = v.to_string(),
                    "PARTICIPANT_2" => meta.participant_2 = Some(v.to_string()),
                    "PARTICIPANT_3" => meta.participant_3 = Some(v.to_string()),
                    "PARTICIPANT_4" => meta.participant_4 = Some(v.to_string()),
                    "PARTICIPANT_5" => meta.participant_5 = Some(v.to_string()),
                    "MODE" => meta.mode = Some(v.to_string()),
                    "PATH" => meta.path = Some(v.to_string()),
                    "PATH_1" => meta.path_1 = Some(v.to_string()),
                    "PATH_2" => meta.path_2 = Some(v.to_string()),
                    "TRANSMIT_BAND" => meta.transmit_band = Some(v.to_string()),
                    "RECEIVE_BAND" => meta.receive_band = Some(v.to_string()),
                    "TURNAROUND_NUMERATOR" => {
                        meta.turnaround_numerator =
                            Some(parse_i32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TURNAROUND_DENOMINATOR" => {
                        meta.turnaround_denominator =
                            Some(parse_i32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TIMETAG_REF" => meta.timetag_ref = Some(v.to_string()),
                    "INTEGRATION_INTERVAL" => {
                        meta.integration_interval =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "INTEGRATION_REF" => meta.integration_ref = Some(v.to_string()),
                    "FREQ_OFFSET" => {
                        meta.freq_offset =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RANGE_MODE" => meta.range_mode = Some(v.to_string()),
                    "RANGE_MODULUS" => {
                        meta.range_modulus =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RANGE_UNITS" => meta.range_units = Some(v.to_string()),
                    "ANGLE_TYPE" => meta.angle_type = Some(v.to_string()),
                    "REFERENCE_FRAME" => meta.reference_frame = Some(v.to_string()),
                    "INTERPOLATION" => meta.interpolation = Some(v.to_string()),
                    "INTERPOLATION_DEGREE" => {
                        meta.interpolation_degree =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "DOPPLER_COUNT_BIAS" => {
                        meta.doppler_count_bias =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "DOPPLER_COUNT_SCALE" => {
                        meta.doppler_count_scale =
                            Some(parse_u64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "DOPPLER_COUNT_ROLLOVER" => meta.doppler_count_rollover = Some(v.to_string()),
                    "TRANSMIT_DELAY_1" => {
                        meta.transmit_delay_1 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TRANSMIT_DELAY_2" => {
                        meta.transmit_delay_2 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TRANSMIT_DELAY_3" => {
                        meta.transmit_delay_3 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TRANSMIT_DELAY_4" => {
                        meta.transmit_delay_4 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "TRANSMIT_DELAY_5" => {
                        meta.transmit_delay_5 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RECEIVE_DELAY_1" => {
                        meta.receive_delay_1 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RECEIVE_DELAY_2" => {
                        meta.receive_delay_2 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RECEIVE_DELAY_3" => {
                        meta.receive_delay_3 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RECEIVE_DELAY_4" => {
                        meta.receive_delay_4 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "RECEIVE_DELAY_5" => {
                        meta.receive_delay_5 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "DATA_QUALITY" => meta.data_quality = Some(v.to_string()),
                    "CORRECTION_ANGLE_1" => {
                        meta.correction_angle_1 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_ANGLE_2" => {
                        meta.correction_angle_2 =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_DOPPLER" => {
                        meta.correction_doppler =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_MAG" => {
                        meta.correction_mag =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_RANGE" => {
                        meta.correction_range =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_RCS" => {
                        meta.correction_rcs =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_RECEIVE" => {
                        meta.correction_receive =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_TRANSMIT" => {
                        meta.correction_transmit =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_ABERRATION_YEARLY" => {
                        meta.correction_aberration_yearly =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTION_ABERRATION_DIURNAL" => {
                        meta.correction_aberration_diurnal =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "CORRECTIONS_APPLIED" => meta.corrections_applied = Some(v.to_string()),
                    "EPHEMERIS_NAME_1" => meta.ephemeris_name_1 = Some(v.to_string()),
                    "EPHEMERIS_NAME_2" => meta.ephemeris_name_2 = Some(v.to_string()),
                    "EPHEMERIS_NAME_3" => meta.ephemeris_name_3 = Some(v.to_string()),
                    "EPHEMERIS_NAME_4" => meta.ephemeris_name_4 = Some(v.to_string()),
                    "EPHEMERIS_NAME_5" => meta.ephemeris_name_5 = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
            Some(key) => {
                return Err(ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Label(format!("Unexpected TDM Metadata key: {}", key).leak()),
                )));
            }
            None => break,
        }
    }

    if meta.time_system.is_empty() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("TIME_SYSTEM")),
        )));
    }
    if meta.participant_1.is_empty() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("PARTICIPANT_1")),
        )));
    }

    Ok(meta)
}

//----------------------------------------------------------------------
// TDM Observation Parser
//----------------------------------------------------------------------

pub fn tdm_observation(input: &mut &str) -> ModalResult<TdmObservation> {
    let (key, val, _unit) = key_value_line.parse_next(input)?;
    opt_line_ending.parse_next(input)?;

    let parts: Vec<&str> = val.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Data line value must contain 'EPOCH MEASUREMENT'"),
        )));
    }

    let epoch_str = parts[0];
    let measure_str = parts[1..].join(" ");

    let epoch = Epoch::from_str(epoch_str).map_err(|e| {
        ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label(e.to_string().leak()),
        ))
    })?;

    let data = TdmObservationData::from_key_val(key, &measure_str, 0).map_err(|e| {
        ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label(e.to_string().leak()),
        ))
    })?;

    Ok(TdmObservation { epoch, data })
}

//----------------------------------------------------------------------
// TDM Data Parser
//----------------------------------------------------------------------

pub fn tdm_data(input: &mut &str) -> ModalResult<TdmData> {
    expect_block_start("DATA").parse_next(input)?;

    let mut comment = Vec::new();
    let mut observations = Vec::new();

    loop {
        if at_block_end("DATA", input) {
            expect_block_end("DATA").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("DATA", input) {
            continue;
        }

        observations.push(tdm_observation.parse_next(input)?);
    }

    if observations.is_empty() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("TDM data section must contain at least one observation"),
        )));
    }

    Ok(TdmData {
        comment,
        observations,
    })
}

//----------------------------------------------------------------------
// TDM Segment Parser
//----------------------------------------------------------------------

pub fn tdm_segment(input: &mut &str) -> ModalResult<TdmSegment> {
    let metadata = tdm_metadata.parse_next(input)?;
    let data = tdm_data.parse_next(input)?;

    Ok(TdmSegment { metadata, data })
}

//----------------------------------------------------------------------
// TDM Body Parser
//----------------------------------------------------------------------

pub fn tdm_body(input: &mut &str) -> ModalResult<TdmBody> {
    let mut segments = Vec::new();

    loop {
        let _ = collect_comments.parse_next(input)?;

        if input.is_empty() || !at_block_start("META", input) {
            break;
        }

        segments.push(tdm_segment.parse_next(input)?);
    }

    if segments.is_empty() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("TDM body must contain at least one segment"),
        )));
    }

    Ok(TdmBody { segments })
}

//----------------------------------------------------------------------
// Complete TDM Parser
//----------------------------------------------------------------------

pub fn parse_tdm(input: &mut &str) -> ModalResult<Tdm> {
    let version = tdm_version.parse_next(input)?;
    let header = tdm_header.parse_next(input)?;
    let body = tdm_body.parse_next(input)?;

    Ok(Tdm {
        header,
        body,
        id: Some("CCSDS_TDM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Tdm {
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        parse_tdm.parse_next(input)
    }
}

pub fn parse_u64(s: &str) -> crate::error::Result<u64> {
    s.trim()
        .parse::<u64>()
        .map_err(|e| crate::error::CcsdsNdmError::KvnParse {
            line: 0,
            message: format!("Invalid u64 '{}': {}", s, e),
        })
}
