// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for TDM (Tracking Data Message).
//!
//! This module implements KVN parsing for TDM using winnow parser combinators.

use super::{
    Tdm, TdmBody, TdmData, TdmHeader, TdmMetadata, TdmObservation, TdmObservationData, TdmSegment,
};
use crate::error::{
    CcsdsNdmError, FormatError, InternalParserError, KvnParseError, Result, ValidationError,
};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;
use winnow::combinator::preceded;
use winnow::error::{AddContext, ErrMode, FromExternalError, StrContext};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// TDM Version Parser
//----------------------------------------------------------------------

pub fn tdm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let value = expect_unitless_key("CCSDS_TDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// TDM Header Parser
//----------------------------------------------------------------------

pub fn tdm_header(input: &mut &str) -> KvnResult<TdmHeader> {
    let mut comment = Vec::new();
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let checkpoint = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        let key = match keyword.parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        if key == "META_START" {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CREATION_DATE" => {
                creation_date = Some(kv_calendar_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = Some(kv_string.parse_next(input)?);
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(TdmHeader {
        comment,
        creation_date: creation_date.ok_or_else(|| cut_err(input, "Expected CREATION_DATE"))?,
        originator: originator.ok_or_else(|| cut_err(input, "Expected ORIGINATOR"))?,
        message_id,
    })
}

//----------------------------------------------------------------------
// TDM Metadata Parser
//----------------------------------------------------------------------

pub fn tdm_metadata(input: &mut &str) -> KvnResult<TdmMetadata> {
    expect_block_start("META").parse_next(input)?;

    let mut comment = Vec::new();
    let mut track_id = None;
    let mut data_types = None;
    let mut time_system = None;
    let mut start_time = None;
    let mut stop_time = None;
    let mut participant_1 = None;
    let mut participant_2 = None;
    let mut participant_3 = None;
    let mut participant_4 = None;
    let mut participant_5 = None;
    let mut mode = None;
    let mut path = None;
    let mut path_1 = None;
    let mut path_2 = None;
    let mut transmit_band = None;
    let mut receive_band = None;
    let mut turnaround_numerator = None;
    let mut turnaround_denominator = None;
    let mut timetag_ref = None;
    let mut integration_interval = None;
    let mut integration_ref = None;
    let mut freq_offset = None;
    let mut range_mode = None;
    let mut range_modulus = None;
    let mut range_units = None;
    let mut angle_type = None;
    let mut reference_frame = None;
    let mut interpolation = None;
    let mut interpolation_degree = None;
    let mut doppler_count_bias = None;
    let mut doppler_count_scale = None;
    let mut doppler_count_rollover = None;
    let mut transmit_delay_1 = None;
    let mut transmit_delay_2 = None;
    let mut transmit_delay_3 = None;
    let mut transmit_delay_4 = None;
    let mut transmit_delay_5 = None;
    let mut receive_delay_1 = None;
    let mut receive_delay_2 = None;
    let mut receive_delay_3 = None;
    let mut receive_delay_4 = None;
    let mut receive_delay_5 = None;
    let mut data_quality = None;
    let mut correction_angle_1 = None;
    let mut correction_angle_2 = None;
    let mut correction_doppler = None;
    let mut correction_mag = None;
    let mut correction_range = None;
    let mut correction_rcs = None;
    let mut correction_receive = None;
    let mut correction_transmit = None;
    let mut correction_aberration_yearly = None;
    let mut correction_aberration_diurnal = None;
    let mut corrections_applied = None;
    let mut ephemeris_name_1 = None;
    let mut ephemeris_name_2 = None;
    let mut ephemeris_name_3 = None;
    let mut ephemeris_name_4 = None;
    let mut ephemeris_name_5 = None;

    parse_block!(input, comment, {
        "TRACK_ID" => val: kv_string => { track_id = Some(val); },
        "DATA_TYPES" => val: kv_string => { data_types = Some(val); },
        "TIME_SYSTEM" => val: kv_string => { time_system = Some(val); },
        "START_TIME" => val: kv_calendar_epoch => { start_time = Some(val); },
        "STOP_TIME" => val: kv_calendar_epoch => { stop_time = Some(val); },
        "PARTICIPANT_1" => val: kv_string => { participant_1 = Some(val); },
        "PARTICIPANT_2" => val: kv_string => { participant_2 = Some(val); },
        "PARTICIPANT_3" => val: kv_string => { participant_3 = Some(val); },
        "PARTICIPANT_4" => val: kv_string => { participant_4 = Some(val); },
        "PARTICIPANT_5" => val: kv_string => { participant_5 = Some(val); },
        "MODE" => val: kv_enum => { mode = Some(val); },
        "PATH" => val: kv_from_kvn_value => { path = Some(val); },
        "PATH_1" => val: kv_from_kvn_value => { path_1 = Some(val); },
        "PATH_2" => val: kv_from_kvn_value => { path_2 = Some(val); },
        "TRANSMIT_BAND" => val: kv_string => { transmit_band = Some(val); },
        "RECEIVE_BAND" => val: kv_string => { receive_band = Some(val); },
        "TURNAROUND_NUMERATOR" => val: kv_i32 => { turnaround_numerator = Some(val); },
        "TURNAROUND_DENOMINATOR" => val: kv_i32 => { turnaround_denominator = Some(val); },
        "TIMETAG_REF" => val: kv_enum => { timetag_ref = Some(val); },
        "INTEGRATION_INTERVAL" => val: kv_float => { integration_interval = Some(val); },
        "INTEGRATION_REF" => val: kv_enum => { integration_ref = Some(val); },
        "FREQ_OFFSET" => val: kv_float => { freq_offset = Some(val); },
        "RANGE_MODE" => val: kv_enum => { range_mode = Some(val); },
        "RANGE_MODULUS" => val: kv_float => { range_modulus = Some(val); },
        "RANGE_UNITS" => val: kv_enum => { range_units = Some(val); },
        "ANGLE_TYPE" => val: kv_enum => { angle_type = Some(val); },
        "REFERENCE_FRAME" => val: kv_enum => { reference_frame = Some(val); },
        "INTERPOLATION" => val: kv_string => { interpolation = Some(val); },
        "INTERPOLATION_DEGREE" => val: kv_u32 => { interpolation_degree = Some(val); },
        "DOPPLER_COUNT_BIAS" => val: kv_float => { doppler_count_bias = Some(val); },
        "DOPPLER_COUNT_SCALE" => val: kv_u64 => { doppler_count_scale = Some(val); },
        "DOPPLER_COUNT_ROLLOVER" => val: kv_enum => { doppler_count_rollover = Some(val); },
        "TRANSMIT_DELAY_1" => val: kv_float => { transmit_delay_1 = Some(val); },
        "TRANSMIT_DELAY_2" => val: kv_float => { transmit_delay_2 = Some(val); },
        "TRANSMIT_DELAY_3" => val: kv_float => { transmit_delay_3 = Some(val); },
        "TRANSMIT_DELAY_4" => val: kv_float => { transmit_delay_4 = Some(val); },
        "TRANSMIT_DELAY_5" => val: kv_float => { transmit_delay_5 = Some(val); },
        "RECEIVE_DELAY_1" => val: kv_float => { receive_delay_1 = Some(val); },
        "RECEIVE_DELAY_2" => val: kv_float => { receive_delay_2 = Some(val); },
        "RECEIVE_DELAY_3" => val: kv_float => { receive_delay_3 = Some(val); },
        "RECEIVE_DELAY_4" => val: kv_float => { receive_delay_4 = Some(val); },
        "RECEIVE_DELAY_5" => val: kv_float => { receive_delay_5 = Some(val); },
        "DATA_QUALITY" => val: kv_enum => { data_quality = Some(val); },
        "CORRECTION_ANGLE_1" => val: kv_float => { correction_angle_1 = Some(val); },
        "CORRECTION_ANGLE_2" => val: kv_float => { correction_angle_2 = Some(val); },
        "CORRECTION_DOPPLER" => val: kv_float => { correction_doppler = Some(val); },
        "CORRECTION_MAG" => val: kv_float => { correction_mag = Some(val); },
        "CORRECTION_RANGE" => val: kv_float => { correction_range = Some(val); },
        "CORRECTION_RCS" => val: kv_float => { correction_rcs = Some(val); },
        "CORRECTION_RECEIVE" => val: kv_float => { correction_receive = Some(val); },
        "CORRECTION_TRANSMIT" => val: kv_float => { correction_transmit = Some(val); },
        "CORRECTION_ABERRATION_YEARLY" => val: kv_float => { correction_aberration_yearly = Some(val); },
        "CORRECTION_ABERRATION_DIURNAL" => val: kv_float => { correction_aberration_diurnal = Some(val); },
        "CORRECTIONS_APPLIED" => val: kv_enum => { corrections_applied = Some(val); },
        "EPHEMERIS_NAME_1" => val: kv_string => { ephemeris_name_1 = Some(val); },
        "EPHEMERIS_NAME_2" => val: kv_string => { ephemeris_name_2 = Some(val); },
        "EPHEMERIS_NAME_3" => val: kv_string => { ephemeris_name_3 = Some(val); },
        "EPHEMERIS_NAME_4" => val: kv_string => { ephemeris_name_4 = Some(val); },
        "EPHEMERIS_NAME_5" => val: kv_string => { ephemeris_name_5 = Some(val); },
    }, |i| at_block_end("META", i), "Unexpected TDM Metadata key");

    if at_block_end("META", input) {
        expect_block_end("META").parse_next(input)?;
    }

    Ok(TdmMetadata {
        comment,
        track_id,
        data_types,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "TDM Metadata", "TIME_SYSTEM"))?,
        start_time,
        stop_time,
        participant_1: participant_1
            .ok_or_else(|| missing_field_err(input, "TDM Metadata", "PARTICIPANT_1"))?,
        participant_2,
        participant_3,
        participant_4,
        participant_5,
        mode,
        path,
        path_1,
        path_2,
        transmit_band,
        receive_band,
        turnaround_numerator,
        turnaround_denominator,
        timetag_ref,
        integration_interval,
        integration_ref,
        freq_offset,
        range_mode,
        range_modulus,
        range_units,
        angle_type,
        reference_frame,
        interpolation,
        interpolation_degree,
        doppler_count_bias,
        doppler_count_scale,
        doppler_count_rollover,
        transmit_delay_1,
        transmit_delay_2,
        transmit_delay_3,
        transmit_delay_4,
        transmit_delay_5,
        receive_delay_1,
        receive_delay_2,
        receive_delay_3,
        receive_delay_4,
        receive_delay_5,
        data_quality,
        correction_angle_1,
        correction_angle_2,
        correction_doppler,
        correction_mag,
        correction_range,
        correction_rcs,
        correction_receive,
        correction_transmit,
        correction_aberration_yearly,
        correction_aberration_diurnal,
        corrections_applied,
        ephemeris_name_1,
        ephemeris_name_2,
        ephemeris_name_3,
        ephemeris_name_4,
        ephemeris_name_5,
    })
}

//----------------------------------------------------------------------
// TDM Observation Parser
//----------------------------------------------------------------------

pub fn tdm_observation(input: &mut &str) -> KvnResult<TdmObservation> {
    let checkpoint = input.checkpoint();
    let key = preceded(ws, keyword).parse_next(input).map_err(|e| {
        if e.is_backtrack() {
            ErrMode::Backtrack(InternalParserError::from_input(input).add_context(
                input,
                &checkpoint,
                StrContext::Label("Expected TDM observation key"),
            ))
        } else {
            e
        }
    })?;

    let (_, epoch, value) = (
        kv_sep,
        kv_calendar_epoch_token,
        preceded(ws, parse_f64_winnow),
    )
        .parse_next(input)?;

    let data = TdmObservationData::from_key_value(key, value).map_err(|error| {
        input.reset(&checkpoint);
        ErrMode::Cut(InternalParserError::from_external_error(input, error))
    })?;

    opt_line_ending.parse_next(input)?;

    Ok(TdmObservation { epoch, data })
}

//----------------------------------------------------------------------
// TDM Data Parser
//----------------------------------------------------------------------

pub fn tdm_data(input: &mut &str) -> KvnResult<TdmData> {
    expect_block_start("DATA").parse_next(input)?;

    let mut comment = Vec::new();
    let mut observations = Vec::new();

    loop {
        if at_block_end("DATA", input) {
            expect_block_end("DATA").parse_next(input)?;
            break;
        }

        let checkpoint = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        if at_block_end("DATA", input) {
            continue;
        }

        observations.push(tdm_observation.parse_next(input)?);

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    if observations.is_empty() {
        return Err(cut_err(
            input,
            "TDM data section must contain at least one observation",
        ));
    }

    Ok(TdmData {
        comment,
        observations,
    })
}

//----------------------------------------------------------------------
// TDM Segment Parser
//----------------------------------------------------------------------

pub fn tdm_segment(input: &mut &str) -> KvnResult<TdmSegment> {
    let metadata = tdm_metadata.parse_next(input)?;
    let data = tdm_data.parse_next(input)?;

    Ok(TdmSegment { metadata, data })
}

//----------------------------------------------------------------------
// TDM Body Parser
//----------------------------------------------------------------------

pub fn tdm_body(input: &mut &str) -> KvnResult<TdmBody> {
    let mut segments = Vec::new();

    loop {
        let checkpoint = input.checkpoint();
        let _ = collect_comments.parse_next(input)?;

        if input.is_empty() || !at_block_start("META", input) {
            break;
        }

        segments.push(tdm_segment.parse_next(input)?);

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    if segments.is_empty() {
        return Err(cut_err(input, "TDM body must contain at least one segment"));
    }

    Ok(TdmBody { segments })
}

//----------------------------------------------------------------------
// Complete TDM Parser
//----------------------------------------------------------------------

pub fn parse_tdm(input: &mut &str) -> KvnResult<Tdm> {
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
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_tdm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

fn is_tdm_observation_key(key: &str) -> bool {
    super::is_tdm_observation_key_bytes(key.as_bytes())
}

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    #[derive(Clone, Copy, PartialEq)]
    enum Section {
        Header,
        Metadata,
        Data,
    }

    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating TDM KVN structure"],
            offset,
        }))))
    };
    let header_rank = |key: &str| match key {
        "CCSDS_TDM_VERS" => Some(0),
        "CREATION_DATE" => Some(1),
        "ORIGINATOR" => Some(2),
        "MESSAGE_ID" => Some(3),
        _ => None,
    };

    let mut section = Section::Header;
    let mut header_previous = None;
    let mut metadata_seen = 0u128;
    let mut metadata_has_content = false;
    let mut data_has_observation = false;
    let mut metadata_closed = false;
    let mut completed_segments = 0usize;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(line_number, offset, message.into()));
        if line.as_bytes().contains(&b'\r') {
            return fail("lone carriage return");
        }
        if line.len() > 254 {
            return fail("line exceeds the normative 254-character limit");
        }
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return fail("non-printable or non-ASCII character");
        }
        let line = line.trim();
        if line.is_empty() {
            offset += raw_line.len() + 1;
            continue;
        }
        if line == "COMMENT" || line.starts_with("COMMENT ") {
            let allowed = match section {
                Section::Header => header_previous == Some(0),
                Section::Metadata => !metadata_has_content,
                Section::Data => !data_has_observation,
            };
            if !allowed {
                return fail("COMMENT is not at the beginning of its TDM logical section");
            }
            offset += raw_line.len() + 1;
            continue;
        }

        match line {
            "META_START" => {
                let after_header =
                    completed_segments == 0 && matches!(header_previous, Some(2 | 3));
                let after_segment = completed_segments > 0 && header_previous.is_none();
                if section != Section::Header || !(after_header || after_segment) {
                    return fail("META_START is out of order");
                }
                section = Section::Metadata;
                metadata_seen = 0;
                metadata_has_content = false;
                metadata_closed = false;
                offset += raw_line.len() + 1;
                continue;
            }
            "META_STOP" => {
                if section != Section::Metadata {
                    return fail("META_STOP without matching META_START");
                }
                section = Section::Header;
                // A sentinel distinguishes the between-segment state from the message header.
                header_previous = None;
                metadata_closed = true;
                offset += raw_line.len() + 1;
                continue;
            }
            "DATA_START" => {
                if section != Section::Header || header_previous.is_some() || !metadata_closed {
                    return fail("DATA_START must immediately follow a metadata section");
                }
                section = Section::Data;
                data_has_observation = false;
                metadata_closed = false;
                offset += raw_line.len() + 1;
                continue;
            }
            "DATA_STOP" => {
                if section != Section::Data {
                    return fail("DATA_STOP without matching DATA_START");
                }
                section = Section::Header;
                header_previous = None;
                completed_segments += 1;
                offset += raw_line.len() + 1;
                continue;
            }
            _ => {}
        }

        if !line.contains('=') {
            return fail("expected an assignment or TDM section delimiter");
        }
        let key = line
            .split_once('=')
            .expect("assignment count checked")
            .0
            .trim();
        match section {
            Section::Header => {
                if completed_segments > 0 || header_previous.is_none() && key != "CCSDS_TDM_VERS" {
                    return fail("assignment outside a TDM section");
                }
                let rank = header_rank(key).ok_or_else(|| {
                    invalid(line_number, offset, "unknown TDM header keyword".into())
                })?;
                if header_previous.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order TDM header keyword");
                }
                header_previous = Some(rank);
            }
            Section::Metadata => {
                let rank = super::tdm_metadata_rank(key).ok_or_else(|| {
                    invalid(line_number, offset, "unknown TDM metadata keyword".into())
                })?;
                if rank == 0 {
                    return fail("COMMENT must use COMMENT line syntax");
                }
                let bit = 1u128 << rank;
                if metadata_seen & bit != 0 {
                    return fail("duplicate or mutually exclusive TDM metadata keyword");
                }
                metadata_seen |= bit;
                metadata_has_content = true;
            }
            Section::Data => {
                if !is_tdm_observation_key(key) {
                    return fail("unknown TDM observation keyword");
                }
                data_has_observation = true;
            }
        }
        offset += raw_line.len() + 1;
    }

    if section != Section::Header || completed_segments == 0 || header_previous.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unterminated or incomplete TDM section sequence".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Tdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_TDM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for TdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(v) = &self.message_id {
            writer.write_pair("MESSAGE_ID", v);
        }
    }
}

impl ToKvn for TdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

impl ToKvn for TdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
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
            writer.write_pair("MODE", v.to_string());
        }
        if let Some(v) = &self.path {
            writer.write_pair("PATH", v.0.as_str());
        }
        if let Some(v) = &self.path_1 {
            writer.write_pair("PATH_1", v.0.as_str());
        }
        if let Some(v) = &self.path_2 {
            writer.write_pair("PATH_2", v.0.as_str());
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
            writer.write_pair("TIMETAG_REF", v.to_string());
        }
        if let Some(v) = self.integration_interval {
            writer.write_pair("INTEGRATION_INTERVAL", v);
        }
        if let Some(v) = &self.integration_ref {
            writer.write_pair("INTEGRATION_REF", v.to_string());
        }
        if let Some(v) = self.freq_offset {
            writer.write_pair("FREQ_OFFSET", v);
        }
        if let Some(v) = &self.range_mode {
            writer.write_pair("RANGE_MODE", v.to_string());
        }
        if let Some(v) = self.range_modulus {
            writer.write_pair("RANGE_MODULUS", v);
        }
        if let Some(v) = &self.range_units {
            writer.write_pair("RANGE_UNITS", v.to_string());
        }
        if let Some(v) = &self.angle_type {
            writer.write_pair("ANGLE_TYPE", v.to_string());
        }
        if let Some(v) = &self.reference_frame {
            writer.write_pair("REFERENCE_FRAME", v.to_string());
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
            writer.write_pair("DOPPLER_COUNT_ROLLOVER", format!("{}", v));
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
            writer.write_pair("DATA_QUALITY", v.to_string());
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
            writer.write_pair("CORRECTIONS_APPLIED", format!("{}", v));
        }
        writer.write_section("META_STOP");
    }
}

impl ToKvn for TdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("DATA_START");
        writer.write_comments(&self.comment);
        for obs in &self.observations {
            writer.write_tdm_observation(obs.data.key(), &obs.epoch, obs.data.value());
        }
        writer.write_section("DATA_STOP");
    }
}

impl Tdm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        let check = |field: &'static str, value: &str| -> Result<()> {
            if value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                Ok(())
            } else {
                Err(ValidationError::InvalidValue {
                    field: field.into(),
                    value: value.to_string(),
                    expected: "printable ASCII for TDM KVN".into(),
                    line: None,
                }
                .into())
            }
        };
        for comment in &self.header.comment {
            check("COMMENT", comment)?;
        }
        check("ORIGINATOR", &self.header.originator)?;
        if let Some(value) = &self.header.message_id {
            check("MESSAGE_ID", value)?;
        }
        for segment in &self.body.segments {
            let metadata = &segment.metadata;
            for comment in &metadata.comment {
                check("COMMENT", comment)?;
            }
            for (field, value) in [
                ("TRACK_ID", metadata.track_id.as_deref()),
                ("DATA_TYPES", metadata.data_types.as_deref()),
                ("TIME_SYSTEM", Some(metadata.time_system.as_str())),
                ("PARTICIPANT_1", Some(metadata.participant_1.as_str())),
                ("PARTICIPANT_2", metadata.participant_2.as_deref()),
                ("PARTICIPANT_3", metadata.participant_3.as_deref()),
                ("PARTICIPANT_4", metadata.participant_4.as_deref()),
                ("PARTICIPANT_5", metadata.participant_5.as_deref()),
                ("EPHEMERIS_NAME_1", metadata.ephemeris_name_1.as_deref()),
                ("EPHEMERIS_NAME_2", metadata.ephemeris_name_2.as_deref()),
                ("EPHEMERIS_NAME_3", metadata.ephemeris_name_3.as_deref()),
                ("EPHEMERIS_NAME_4", metadata.ephemeris_name_4.as_deref()),
                ("EPHEMERIS_NAME_5", metadata.ephemeris_name_5.as_deref()),
                ("TRANSMIT_BAND", metadata.transmit_band.as_deref()),
                ("RECEIVE_BAND", metadata.receive_band.as_deref()),
                ("INTERPOLATION", metadata.interpolation.as_deref()),
            ] {
                if let Some(value) = value {
                    check(field, value)?;
                }
            }
            for comment in &segment.data.comment {
                check("COMMENT", comment)?;
            }
        }
        Ok(())
    }
}
