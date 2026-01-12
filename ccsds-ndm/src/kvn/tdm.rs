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
                            Epoch::from_str(v)
                                .map_err(|_| cut_err(input, "Invalid CREATION_DATE"))?,
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
                            Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid START_TIME"))?,
                        );
                    }
                    "STOP_TIME" => {
                        meta.stop_time = Some(
                            Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid STOP_TIME"))?,
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
                        meta.turnaround_numerator = Some(
                            parse_i32(v)
                                .map_err(|_| cut_err(input, "Invalid TURNAROUND_NUMERATOR"))?,
                        );
                    }
                    "TURNAROUND_DENOMINATOR" => {
                        meta.turnaround_denominator = Some(
                            parse_i32(v)
                                .map_err(|_| cut_err(input, "Invalid TURNAROUND_DENOMINATOR"))?,
                        );
                    }
                    "TIMETAG_REF" => meta.timetag_ref = Some(v.to_string()),
                    "INTEGRATION_INTERVAL" => {
                        meta.integration_interval = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid INTEGRATION_INTERVAL"))?,
                        );
                    }
                    "INTEGRATION_REF" => meta.integration_ref = Some(v.to_string()),
                    "FREQ_OFFSET" => {
                        meta.freq_offset =
                            Some(parse_f64(v).map_err(|_| cut_err(input, "Invalid FREQ_OFFSET"))?);
                    }
                    "RANGE_MODE" => meta.range_mode = Some(v.to_string()),
                    "RANGE_MODULUS" => {
                        meta.range_modulus = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RANGE_MODULUS"))?,
                        );
                    }
                    "RANGE_UNITS" => meta.range_units = Some(v.to_string()),
                    "ANGLE_TYPE" => meta.angle_type = Some(v.to_string()),
                    "REFERENCE_FRAME" => meta.reference_frame = Some(v.to_string()),
                    "INTERPOLATION" => meta.interpolation = Some(v.to_string()),
                    "INTERPOLATION_DEGREE" => {
                        meta.interpolation_degree = Some(
                            parse_u32(v)
                                .map_err(|_| cut_err(input, "Invalid INTERPOLATION_DEGREE"))?,
                        );
                    }
                    "DOPPLER_COUNT_BIAS" => {
                        meta.doppler_count_bias = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid DOPPLER_COUNT_BIAS"))?,
                        );
                    }
                    "DOPPLER_COUNT_SCALE" => {
                        meta.doppler_count_scale = Some(
                            parse_u64(v)
                                .map_err(|_| cut_err(input, "Invalid DOPPLER_COUNT_SCALE"))?,
                        );
                    }
                    "DOPPLER_COUNT_ROLLOVER" => meta.doppler_count_rollover = Some(v.to_string()),
                    "TRANSMIT_DELAY_1" => {
                        meta.transmit_delay_1 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid TRANSMIT_DELAY_1"))?,
                        );
                    }
                    "TRANSMIT_DELAY_2" => {
                        meta.transmit_delay_2 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid TRANSMIT_DELAY_2"))?,
                        );
                    }
                    "TRANSMIT_DELAY_3" => {
                        meta.transmit_delay_3 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid TRANSMIT_DELAY_3"))?,
                        );
                    }
                    "TRANSMIT_DELAY_4" => {
                        meta.transmit_delay_4 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid TRANSMIT_DELAY_4"))?,
                        );
                    }
                    "TRANSMIT_DELAY_5" => {
                        meta.transmit_delay_5 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid TRANSMIT_DELAY_5"))?,
                        );
                    }
                    "RECEIVE_DELAY_1" => {
                        meta.receive_delay_1 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RECEIVE_DELAY_1"))?,
                        );
                    }
                    "RECEIVE_DELAY_2" => {
                        meta.receive_delay_2 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RECEIVE_DELAY_2"))?,
                        );
                    }
                    "RECEIVE_DELAY_3" => {
                        meta.receive_delay_3 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RECEIVE_DELAY_3"))?,
                        );
                    }
                    "RECEIVE_DELAY_4" => {
                        meta.receive_delay_4 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RECEIVE_DELAY_4"))?,
                        );
                    }
                    "RECEIVE_DELAY_5" => {
                        meta.receive_delay_5 = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid RECEIVE_DELAY_5"))?,
                        );
                    }
                    "DATA_QUALITY" => meta.data_quality = Some(v.to_string()),
                    "CORRECTION_ANGLE_1" => {
                        meta.correction_angle_1 = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid CORRECTION_ANGLE_1"))?,
                        );
                    }
                    "CORRECTION_ANGLE_2" => {
                        meta.correction_angle_2 = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid CORRECTION_ANGLE_2"))?,
                        );
                    }
                    "CORRECTION_DOPPLER" => {
                        meta.correction_doppler = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid CORRECTION_DOPPLER"))?,
                        );
                    }
                    "CORRECTION_MAG" => {
                        meta.correction_mag = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid CORRECTION_MAG"))?,
                        );
                    }
                    "CORRECTION_RANGE" => {
                        meta.correction_range = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid CORRECTION_RANGE"))?,
                        );
                    }
                    "CORRECTION_RCS" => {
                        meta.correction_rcs = Some(
                            parse_f64(v).map_err(|_| cut_err(input, "Invalid CORRECTION_RCS"))?,
                        );
                    }
                    "CORRECTION_RECEIVE" => {
                        meta.correction_receive = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid CORRECTION_RECEIVE"))?,
                        );
                    }
                    "CORRECTION_TRANSMIT" => {
                        meta.correction_transmit = Some(
                            parse_f64(v)
                                .map_err(|_| cut_err(input, "Invalid CORRECTION_TRANSMIT"))?,
                        );
                    }
                    "CORRECTION_ABERRATION_YEARLY" => {
                        meta.correction_aberration_yearly =
                            Some(parse_f64(v).map_err(|_| {
                                cut_err(input, "Invalid CORRECTION_ABERRATION_YEARLY")
                            })?);
                    }
                    "CORRECTION_ABERRATION_DIURNAL" => {
                        meta.correction_aberration_diurnal = Some(parse_f64(v).map_err(|_| {
                            cut_err(input, "Invalid CORRECTION_ABERRATION_DIURNAL")
                        })?);
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
            Some(_key) => {
                return Err(ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Label("Unexpected TDM Metadata key"),
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

    let data = TdmObservationData::from_key_val(key, &measure_str).map_err(|e| {
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
        .map_err(crate::error::CcsdsNdmError::ParseInt)
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;
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
        assert_eq!(seg.metadata.angle_type.as_deref(), Some("RADEC"));
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
            TdmObservationData::TransmitPhaseCt1(s) => assert_eq!(s, "7175173383.615373"),
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
        assert_eq!(tdm.body.segments[0].metadata.path.as_deref(), Some("1,2,1"));
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
META_STOP
DATA_START
RECEIVE_FREQ = 2023-01-01T00:00:00 8415000000.0
DATA_STOP
"#;
        let tdm = Tdm::from_kvn(kvn).unwrap();
        let seg = &tdm.body.segments[0];
        assert!(seg.metadata.path.is_none());
        assert_eq!(seg.metadata.path_1.as_deref(), Some("1,2,1"));
        assert_eq!(seg.metadata.path_2.as_deref(), Some("3,2,3"));
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
            tdm.body.segments[0].metadata.data_quality.as_deref(),
            Some("VALIDATED")
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
                assert_eq!(s, "7175173383.615373");
            }
            _ => panic!("Wrong type"),
        }
        match &tdm.body.segments[0].data.observations[1].data {
            TdmObservationData::ReceivePhaseCt1(s) => {
                assert_eq!(s, "8429753135.986102");
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
        assert_eq!(seg.metadata.mode.as_deref(), Some("SEQUENTIAL"));
        assert_eq!(seg.metadata.transmit_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.receive_band.as_deref(), Some("X"));
        assert_eq!(seg.metadata.turnaround_numerator, Some(880));
        assert_eq!(seg.metadata.turnaround_denominator, Some(749));
        assert_eq!(seg.metadata.timetag_ref.as_deref(), Some("RECEIVE"));
        assert_eq!(seg.metadata.integration_interval, Some(60.0));
        assert_eq!(seg.metadata.integration_ref.as_deref(), Some("MIDDLE"));
        assert_eq!(seg.metadata.range_mode.as_deref(), Some("COHERENT"));
        assert_eq!(seg.metadata.range_modulus, Some(32768.0));
        assert_eq!(seg.metadata.range_units.as_deref(), Some("km"));
        assert_eq!(seg.metadata.angle_type.as_deref(), Some("AZEL"));
        assert_eq!(seg.metadata.reference_frame.as_deref(), Some("EME2000"));
        assert_eq!(seg.metadata.interpolation.as_deref(), Some("LAGRANGE"));
        assert_eq!(seg.metadata.interpolation_degree, Some(7));
        assert_eq!(seg.metadata.doppler_count_bias, Some(240000000.0));
        assert_eq!(seg.metadata.doppler_count_scale, Some(1000));
        assert_eq!(seg.metadata.doppler_count_rollover.as_deref(), Some("NO"));
        assert_eq!(seg.metadata.data_quality.as_deref(), Some("VALIDATED"));
        assert_eq!(seg.metadata.correction_range, Some(0.001));
        assert_eq!(seg.metadata.corrections_applied.as_deref(), Some("YES"));
    }

    #[test]
    fn test_tdm_empty_file_error() {
        let err = Tdm::from_kvn("").unwrap_err();
        match err {
            crate::error::CcsdsNdmError::UnexpectedEof { .. } => {}
            crate::error::CcsdsNdmError::KvnParse { .. } => {}
            _ => panic!("Expected Empty file error, got: {:?}", err),
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
            crate::error::CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected ccsds_tdm_vers"));
            }
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
            crate::error::CcsdsNdmError::KvnParse {
                message: msg,
                contexts,
                ..
            } => {
                assert!(
                    msg.to_lowercase().contains("unknown tdm data keyword")
                        || contexts
                            .iter()
                            .any(|c| c.to_lowercase().contains("unknown tdm data keyword"))
                );
            }
            _ => panic!("Expected KvnParse error, got: {:?}", err),
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
            crate::error::CcsdsNdmError::KvnParse {
                message: msg,
                contexts,
                ..
            } => {
                assert!(
                    msg.to_lowercase().contains("unexpected tdm metadata key")
                        || contexts
                            .iter()
                            .any(|c| c.to_lowercase().contains("unexpected tdm metadata key"))
                );
            }
            _ => panic!("Expected KvnParse error, got: {:?}", err),
        }
    }
}
