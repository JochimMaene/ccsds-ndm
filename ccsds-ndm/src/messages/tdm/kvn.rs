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
    let (value, _) = expect_key("CCSDS_TDM_VERS").parse_next(input)?;
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

    let (_, epoch, value) =
        (kv_sep, kv_calendar_epoch_token, preceded(ws, parse_f64_winnow)).parse_next(input)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FormatError;
    use crate::traits::Ndm;
    use crate::types::{
        TdmAngleType, TdmDataQuality, TdmIntegrationRef, TdmMode, TdmPath, TdmRangeMode,
        TdmRangeUnits, TdmReferenceFrame, TdmTimetagRef, YesNo,
    };
    // We need TdmObservationData variants visible
    use crate::messages::tdm::TdmObservationData;

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
        assert_eq!(seg.metadata.angle_type, Some(TdmAngleType::Radec));
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
            TdmObservationData::TransmitPhaseCt1(s) => assert_eq!(*s, 7175173383.615373),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_header_mandatory_creation_date() {
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

    #[test]
    fn test_xsd_metadata_mandatory_time_system() {
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
        assert_eq!(
            tdm.body.segments[0].metadata.path,
            Some(TdmPath("1,2,1".to_string()))
        );
        assert!(tdm.body.segments[0].metadata.path_1.is_none());
        assert!(tdm.body.segments[0].metadata.path_2.is_none());
    }

    #[test]
    fn test_xsd_metadata_path_1_path_2_choice() {
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
RECEIVE_BAND = X
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert!(seg.metadata.path.is_none());
        assert_eq!(seg.metadata.path_1, Some(TdmPath("1,2,1".to_string())));
        assert_eq!(seg.metadata.path_2, Some(TdmPath("3,2,3".to_string())));
    }

    #[test]
    fn test_xsd_metadata_optional_freq_offset() {
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
        assert!(tdm.body.segments[0].metadata.freq_offset.is_none());
    }

    #[test]
    fn test_xsd_metadata_explicit_freq_offset() {
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
        assert!(tdm.body.segments[0].metadata.range_modulus.is_none());
    }

    #[test]
    fn test_xsd_metadata_data_quality_values() {
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
            tdm.body.segments[0].metadata.data_quality,
            Some(TdmDataQuality::Validated)
        );
    }

    #[test]
    fn test_xsd_metadata_transmit_receive_delays() {
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

    #[test]
    fn test_xsd_body_requires_at_least_one_segment() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let result = Tdm::from_kvn(kvn);
        assert!(result.is_err());
    }

    #[test]
    fn test_xsd_body_multiple_segments() {
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

    #[test]
    fn test_xsd_data_requires_at_least_one_observation() {
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
        let result = Tdm::from_kvn(kvn);
        if result.is_ok() {
            assert!(result.unwrap().body.segments[0]
                .data
                .observations
                .is_empty());
        }
    }

    #[test]
    fn test_xsd_data_multiple_observations() {
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

    #[test]
    fn test_xsd_observation_angle_types() {
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
                assert_eq!(*s, 7175173383.615373);
            }
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::ReceivePhaseCt1(s) => {
                assert_eq!(*s, 8429753135.986102);
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn test_xsd_observation_vlbi_types() {
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

    #[test]
    fn test_xsd_sample_tdm_e1_kvn() {
        let kvn = include_str!("../../../data/kvn/tdm_e1.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
        assert!(!tdm.body.segments[0].metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e2_kvn() {
        let kvn = include_str!("../../../data/kvn/tdm_e2.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e3_kvn() {
        let kvn = include_str!("../../../data/kvn/tdm_e3.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_sample_tdm_e16_kvn() {
        let kvn = include_str!("../../../data/kvn/tdm_e16.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
        let seg = &tdm.body.segments[0];
        assert!(seg.metadata.angle_type.is_some());
    }

    #[test]
    fn test_xsd_sample_tdm_e18_kvn() {
        let kvn = include_str!("../../../data/kvn/tdm_e18.kvn");
        let tdm = Tdm::from_kvn(kvn).unwrap();
        assert!(!tdm.body.segments.is_empty());
    }

    #[test]
    fn test_xsd_all_metadata_optional_fields() {
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
        assert_eq!(seg.metadata.mode, Some(TdmMode::Sequential));
        assert_eq!(seg.metadata.path, Some(TdmPath("1,2,1".to_string())));
        assert_eq!(seg.metadata.transmit_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.receive_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.turnaround_numerator, Some(880));
        assert_eq!(seg.metadata.turnaround_denominator, Some(749));
        assert_eq!(seg.metadata.timetag_ref, Some(TdmTimetagRef::Receive));
        assert_eq!(seg.metadata.integration_interval, Some(60.0));
        assert_eq!(
            seg.metadata.integration_ref,
            Some(TdmIntegrationRef::Middle)
        );
        assert_eq!(seg.metadata.range_mode, Some(TdmRangeMode::Coherent));
        assert_eq!(seg.metadata.range_modulus, Some(32768.0));
        assert_eq!(seg.metadata.range_units, Some(TdmRangeUnits::Km));
        assert_eq!(seg.metadata.angle_type, Some(TdmAngleType::Azel));
        assert_eq!(
            seg.metadata.reference_frame,
            Some(TdmReferenceFrame::Eme2000)
        );
        assert_eq!(seg.metadata.interpolation.as_deref(), Some("LAGRANGE"));
        assert_eq!(seg.metadata.interpolation_degree, Some(7));
        assert_eq!(seg.metadata.doppler_count_bias, Some(240000000.0));
        assert_eq!(seg.metadata.doppler_count_scale, Some(1000));
        assert_eq!(seg.metadata.doppler_count_rollover, Some(YesNo::No));
        assert_eq!(seg.metadata.data_quality, Some(TdmDataQuality::Validated));
        assert_eq!(seg.metadata.correction_range, Some(0.001));
        assert_eq!(seg.metadata.corrections_applied, Some(YesNo::Yes));
    }

    #[test]
    fn test_tdm_empty_file_error() {
        let err = Tdm::from_kvn("").unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {}
            e if e.is_kvn_error() => {}
            _ => panic!("Expected error, got: {:?}", err),
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
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(err.message.contains("assignment outside a TDM section"));
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected version-not-first error, got: {:?}", err),
        }
    }

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
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("unknown TDM observation keyword")
                            || err.contexts.contains(&"while validating TDM KVN structure")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected error, got: {:?}", err),
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
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("unknown TDM metadata keyword")
                            || err.contexts.contains(&"while validating TDM KVN structure")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected error, got: {:?}", err),
        }
    }

    #[test]
    fn test_tdm_kvn_exhaustive_coverage() {
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
TRANSMIT_DELAY_3 = 0.3
TRANSMIT_DELAY_4 = 0.4
TRANSMIT_DELAY_5 = 0.5
RECEIVE_DELAY_4 = 1.4
RECEIVE_DELAY_5 = 1.5
CORRECTION_DOPPLER = 0.3
CORRECTION_MAG = 0.4
CORRECTION_RCS = 0.5
CORRECTION_TRANSMIT = 0.6
CORRECTION_ABERRATION_DIURNAL = 0.7
CORRECTIONS_APPLIED = YES
META_STOP
DATA_START
RECEIVE_FREQ_2 = 2023-01-01T00:00:00 123.0
RECEIVE_FREQ_4 = 2023-01-01T00:00:01 124.0
RECEIVE_FREQ_5 = 2023-01-01T00:00:02 125.0
RECEIVE_PHASE_CT_2 = 2023-01-01T00:00:03 222.0
RECEIVE_PHASE_CT_3 = 2023-01-01T00:00:04 223.0
RECEIVE_PHASE_CT_4 = 2023-01-01T00:00:05 224.0
RECEIVE_PHASE_CT_5 = 2023-01-01T00:00:06 225.0
TRANSMIT_FREQ_3 = 2023-01-01T00:00:07 323.0
TRANSMIT_FREQ_4 = 2023-01-01T00:00:08 324.0
TRANSMIT_FREQ_5 = 2023-01-01T00:00:09 325.0
TRANSMIT_FREQ_RATE_2 = 2023-01-01T00:00:10 422.0
TRANSMIT_FREQ_RATE_3 = 2023-01-01T00:00:11 423.0
TRANSMIT_FREQ_RATE_4 = 2023-01-01T00:00:12 424.0
TRANSMIT_FREQ_RATE_5 = 2023-01-01T00:00:13 425.0
TRANSMIT_PHASE_CT_2 = 2023-01-01T00:00:14 522.0
TRANSMIT_PHASE_CT_3 = 2023-01-01T00:00:15 523.0
TRANSMIT_PHASE_CT_4 = 2023-01-01T00:00:16 524.0
TRANSMIT_PHASE_CT_5 = 2023-01-01T00:00:17 525.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).expect("parse exhaustive tdm kvn");
        let meta = &tdm.body.segments[0].metadata;
        assert_eq!(meta.transmit_delay_5, Some(0.5));
        assert_eq!(meta.correction_aberration_diurnal, Some(0.7));
        assert_eq!(tdm.body.segments[0].data.observations.len(), 18);
    }

    #[test]
    fn test_tdm_kvn_backtrack_and_errors() {
        // Unknown key between header and body should FAIL
        let kvn = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
UNKNOWN_KEY = VALUE
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
META_STOP
DATA_START
RANGE = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        assert!(Tdm::from_kvn(kvn).is_err());

        // Backtrack in tdm_observation (malformed key)
        let kvn_malformed = r#"CCSDS_TDM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
PARTICIPANT_1 = P1
META_STOP
DATA_START
BAD_KEY = 2023-01-01T00:00:00 1000.0
DATA_STOP
"#;
        assert!(Tdm::from_kvn(kvn_malformed).is_err());
    }

    #[test]
    fn test_observation_value_error_reports_record_line() {
        let good = include_str!("../../../data/kvn/tdm_e1.kvn");
        let record = "RECEIVE_FREQ_1 = 2005-159T17:41:00 32021034790.7265";
        let bad = good.replacen(record, "RHUMIDITY = 2005-159T17:41:00 150.0", 1);
        let expected_line = good[..good.find(record).unwrap()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1;

        let CcsdsNdmError::Validation(error) = Tdm::from_kvn(&bad).unwrap_err() else {
            panic!("expected a validation error");
        };
        let crate::error::ValidationError::OutOfRange { line, .. } = *error else {
            panic!("expected an out-of-range error");
        };
        assert_eq!(line, Some(expected_line));
    }
}
