// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for ACM (Attitude Comprehensive Message).

use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::acm::{
    Acm, AcmAttitudeDetermination, AcmAttitudeState, AcmBody, AcmCovarianceMatrix, AcmData,
    AcmManeuverParameters, AcmMetadata, AcmPhysicalDescription, AcmSegment, AcmSensor, AttLine,
    CovLine,
};
use crate::parse_block;

use std::str::FromStr;
use winnow::combinator::{peek, terminated};
use winnow::error::{AddContext, ErrMode, FromExternalError};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// ACM Version Parser
//----------------------------------------------------------------------

pub fn acm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_ACM_VERS").parse_next(input)?;
    if value != "1.0" && value != "2.0" {
        return Err(cut_err(input, "1.0 or 2.0"));
    }
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// ACM Metadata Parser
//----------------------------------------------------------------------

pub fn acm_metadata(input: &mut &str) -> KvnResult<AcmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut international_designator = None;
    let mut catalog_name = None;
    let mut object_designator = None;
    let mut originator_poc = None;
    let mut originator_position = None;
    let mut originator_phone = None;
    let mut originator_email = None;
    let mut originator_address = None;
    let mut odm_msg_link = None;
    let mut center_name = None;
    let mut time_system = None;
    let mut epoch_tzero = None;
    let mut taimutc_at_tzero = None;
    let mut next_leap_epoch = None;
    let mut next_leap_taimutc = None;
    let mut acm_data_elements = None;
    let mut start_time = None;
    let mut stop_time = None;

    expect_block_start("META").parse_next(input)?;

    parse_block!(input, comment, {
        "OBJECT_NAME" => val: kv_string => { object_name = Some(val); },
        "INTERNATIONAL_DESIGNATOR" => val: kv_string => { international_designator = Some(val); },
        "CATALOG_NAME" => val: kv_string => { catalog_name = Some(val); },
        "OBJECT_DESIGNATOR" => val: kv_string => { object_designator = Some(val); },
        "ORIGINATOR_POC" => val: kv_string => { originator_poc = Some(val); },
        "ORIGINATOR_POSITION" => val: kv_string => { originator_position = Some(val); },
        "ORIGINATOR_PHONE" => val: kv_string => { originator_phone = Some(val); },
        "ORIGINATOR_EMAIL" => val: kv_string => { originator_email = Some(val); },
        "ORIGINATOR_ADDRESS" => val: kv_string => { originator_address = Some(val); },
        "ODM_MSG_LINK" => val: kv_string => { odm_msg_link = Some(val); },
        "CENTER_NAME" => val: kv_string => { center_name = Some(val); },
        "TIME_SYSTEM" => val: kv_string => { time_system = Some(val); },
        "EPOCH_TZERO" => val: kv_epoch => { epoch_tzero = Some(val); },
        "TAIMUTC_AT_TZERO" => val: kv_from_kvn => { taimutc_at_tzero = Some(val); },
        "NEXT_LEAP_EPOCH" => val: kv_epoch => { next_leap_epoch = Some(val); },
        "NEXT_LEAP_TAIMUTC" => val: kv_from_kvn => { next_leap_taimutc = Some(val); },
        "ACM_DATA_ELEMENTS" => val: kv_string => { acm_data_elements = Some(val); },
        "START_TIME" => val: kv_epoch => { start_time = Some(val); },
        "STOP_TIME" => val: kv_epoch => { stop_time = Some(val); },
    }, |i: &mut &str| at_block_end("META", i));

    expect_block_end("META").parse_next(input)?;

    Ok(AcmMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "ACM Metadata", "OBJECT_NAME"))?,
        international_designator,
        catalog_name,
        object_designator,
        originator_poc,
        originator_position,
        originator_phone,
        originator_email,
        originator_address,
        odm_msg_link,
        center_name,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "ACM Metadata", "TIME_SYSTEM"))?,
        epoch_tzero: epoch_tzero
            .ok_or_else(|| missing_field_err(input, "ACM Metadata", "EPOCH_TZERO"))?,
        taimutc_at_tzero,
        next_leap_epoch,
        next_leap_taimutc,
        acm_data_elements,
        start_time,
        stop_time,
    })
}

//----------------------------------------------------------------------
// ACM Data Logical Blocks
//----------------------------------------------------------------------

fn parse_att_line(input: &mut &str) -> KvnResult<AttLine> {
    let line = terminated(raw_line, opt_line_ending).parse_next(input)?;
    let values = line
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AttLine { values })
}

fn parse_att_block(input: &mut &str) -> KvnResult<AcmAttitudeState> {
    let mut comment = Vec::new();
    let mut att_id = None;
    let mut att_prev_id = None;
    let mut att_basis = None;
    let mut att_basis_id = None;
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut number_states = None;
    let mut att_type = None;
    let mut rate_type = None;
    let mut euler_rot_seq = None;
    let mut att_lines = Vec::new();

    expect_block_start("ATT").parse_next(input)?;

    parse_block!(input, comment, {
        "ATT_ID" => val: kv_string => { att_id = Some(val); },
        "ATT_PREV_ID" => val: kv_string => { att_prev_id = Some(val); },
        "ATT_BASIS" => val: kv_enum => { att_basis = Some(val); },
        "ATT_BASIS_ID" => val: kv_string => { att_basis_id = Some(val); },
        "REF_FRAME_A" => val: kv_string => { ref_frame_a = Some(val); },
        "REF_FRAME_B" => val: kv_string => { ref_frame_b = Some(val); },
        "NUMBER_STATES" => val: kv_u32 => { number_states = Some(val); },
        "ATT_TYPE" => val: kv_string => { att_type = Some(val); },
        "RATE_TYPE" => val: kv_string => { rate_type = Some(val); },
        "EULER_ROT_SEQ" => val: kv_enum => { euler_rot_seq = Some(val); },
    }, |i: &mut &str| matches!(i.trim_start().chars().next(), Some('0'..='9' | '-' | '+')) || at_block_end("ATT", i));

    loop {
        if at_block_end("ATT", input) {
            break;
        }
        let start = input.checkpoint();
        if peek((ws, "COMMENT")).parse_next(input).is_ok() {
            comment.extend(collect_comments.parse_next(input)?);
            if input.offset_from(&start) == 0 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            continue;
        }
        let checkpoint = input.checkpoint();
        let res = parse_att_line.parse_next(input)?;
        if input.offset_from(&checkpoint) == 0 && input.is_empty() {
            break;
        }
        att_lines.push(res);
    }

    expect_block_end("ATT").parse_next(input)?;
    Ok(AcmAttitudeState {
        comment,
        att_id,
        att_prev_id,
        att_basis,
        att_basis_id,
        ref_frame_a: ref_frame_a
            .ok_or_else(|| missing_field_err(input, "ACM ATT", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b
            .ok_or_else(|| missing_field_err(input, "ACM ATT", "REF_FRAME_B"))?,
        number_states: number_states
            .ok_or_else(|| missing_field_err(input, "ACM ATT", "NUMBER_STATES"))?,
        att_type: att_type.ok_or_else(|| missing_field_err(input, "ACM ATT", "ATT_TYPE"))?,
        rate_type,
        euler_rot_seq,
        att_lines,
    })
}

fn parse_phys_block(input: &mut &str) -> KvnResult<AcmPhysicalDescription> {
    let mut block = AcmPhysicalDescription::default();
    expect_block_start("PHYS").parse_next(input)?;

    parse_block!(input, block.comment, {
        "DRAG_COEFF" => val: kv_float => { block.drag_coeff = Some(val); },
        "WET_MASS" => val: kv_from_kvn => { block.wet_mass = Some(val); },
        "DRY_MASS" => val: kv_from_kvn => { block.dry_mass = Some(val); },
        "CP_REF_FRAME" => val: kv_string => { block.cp_ref_frame = Some(val); },
        "CP_X" => val: kv_float => { block.cp.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[0] = val; },
        "CP_Y" => val: kv_float => { block.cp.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[1] = val; },
        "CP_Z" => val: kv_float => { block.cp.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[2] = val; },
        "CP" => val: kv_vector3 => { block.cp = Some(crate::types::Vector3 { elements: val, units: None }); },
        "INERTIA_REF_FRAME" => val: kv_string => { block.inertia_ref_frame = Some(val); },
        "IXX" => val: kv_from_kvn => { block.ixx = Some(val); },
        "IYY" => val: kv_from_kvn => { block.iyy = Some(val); },
        "IZZ" => val: kv_from_kvn => { block.izz = Some(val); },
        "IXY" => val: kv_from_kvn => { block.ixy = Some(val); },
        "IXZ" => val: kv_from_kvn => { block.ixz = Some(val); },
        "IYZ" => val: kv_from_kvn => { block.iyz = Some(val); },
    }, |i| at_block_end("PHYS", i));

    expect_block_end("PHYS").parse_next(input)?;
    Ok(block)
}

fn parse_cov_line(input: &mut &str) -> KvnResult<CovLine> {
    let line = terminated(raw_line, opt_line_ending).parse_next(input)?;
    let values = line
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CovLine { values })
}

fn parse_cov_block(input: &mut &str) -> KvnResult<AcmCovarianceMatrix> {
    let mut comment = Vec::new();
    let mut cov_basis = None;
    let mut cov_ref_frame = None;
    let mut cov_type = None;
    let mut cov_confidence = None;
    let mut cov_lines = Vec::new();

    expect_block_start("COV").parse_next(input)?;

    parse_block!(input, comment, {
        "COV_BASIS" => val: kv_string => { cov_basis = Some(val); },
        "COV_REF_FRAME" => val: kv_string => { cov_ref_frame = Some(val); },
        "COV_TYPE" => val: kv_string => { cov_type = Some(val); },
        "COV_CONFIDENCE" => val: kv_float => { cov_confidence = Some(val); },
    }, |i: &mut &str| matches!(i.trim_start().chars().next(), Some('0'..='9' | '-' | '+')) || at_block_end("COV", i));

    loop {
        if at_block_end("COV", input) {
            break;
        }
        let checkpoint = input.checkpoint();
        let res = parse_cov_line.parse_next(input)?;
        if input.offset_from(&checkpoint) == 0 && input.is_empty() {
            break;
        }
        cov_lines.push(res);
    }

    expect_block_end("COV").parse_next(input)?;
    Ok(AcmCovarianceMatrix {
        comment,
        cov_basis: cov_basis.ok_or_else(|| missing_field_err(input, "ACM COV", "COV_BASIS"))?,
        cov_ref_frame: cov_ref_frame
            .ok_or_else(|| missing_field_err(input, "ACM COV", "COV_REF_FRAME"))?,
        cov_type: cov_type.ok_or_else(|| missing_field_err(input, "ACM COV", "COV_TYPE"))?,
        cov_confidence,
        cov_lines,
    })
}

fn kv_target_momentum(input: &mut &str) -> KvnResult<crate::types::TargetMomentum> {
    let (val_str, unit_str) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    let values = val_str
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let units = if let Some(u) = unit_str {
        Some(
            crate::types::AngMomentumUnits::from_str(u)
                .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?,
        )
    } else {
        None
    };
    Ok(crate::types::TargetMomentum {
        elements: values,
        units,
    })
}

fn parse_man_block(input: &mut &str) -> KvnResult<AcmManeuverParameters> {
    let mut comment = Vec::new();
    let mut man_id = None;
    let mut man_prev_id = None;
    let mut man_purpose = None;
    let mut man_begin_time = None;
    let mut man_end_time = None;
    let mut man_duration = None;
    let mut actuator_used = None;
    let mut target_momentum = None;
    let mut target_mom_frame = None;

    expect_block_start("MAN").parse_next(input)?;

    parse_block!(input, comment, {
        "MAN_ID" => val: kv_string => { man_id = Some(val); },
        "MAN_PREV_ID" => val: kv_string => { man_prev_id = Some(val); },
        "MAN_PURPOSE" => val: kv_string => { man_purpose = Some(val); },
        "MAN_BEGIN_TIME" => val: kv_epoch => { man_begin_time = Some(val); },
        "MAN_END_TIME" => val: kv_epoch => { man_end_time = Some(val); },
        "MAN_DURATION" => val: kv_from_kvn => { man_duration = Some(val); },
        "ACTUATOR_USED" => val: kv_string => { actuator_used = Some(val); },
        "TARGET_MOMENTUM" => val: kv_target_momentum => { target_momentum = Some(val); },
        "TARGET_MOM_X" => val: kv_float => { target_momentum.get_or_insert_with(|| crate::types::TargetMomentum { elements: vec![0.0, 0.0, 0.0], units: None }).elements[0] = val; },
        "TARGET_MOM_Y" => val: kv_float => { target_momentum.get_or_insert_with(|| crate::types::TargetMomentum { elements: vec![0.0, 0.0, 0.0], units: None }).elements[1] = val; },
        "TARGET_MOM_Z" => val: kv_float => { target_momentum.get_or_insert_with(|| crate::types::TargetMomentum { elements: vec![0.0, 0.0, 0.0], units: None }).elements[2] = val; },
        "TARGET_MOM_FRAME" => val: kv_string => { target_mom_frame = Some(val); },
    }, |i: &mut &str| at_block_end("MAN", i), "Unexpected ACM Maneuver key");

    expect_block_end("MAN").parse_next(input)?;
    Ok(AcmManeuverParameters {
        comment,
        man_id,
        man_prev_id,
        man_purpose,
        man_begin_time,
        man_end_time,
        man_duration,
        actuator_used,
        target_momentum,
        target_mom_frame,
    })
}

fn kv_sensor_noise(input: &mut &str) -> KvnResult<crate::types::SensorNoise> {
    let (val_str, unit_str) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    let values = val_str
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let units = if let Some(u) = unit_str {
        Some(
            crate::types::AngleUnits::from_str(u)
                .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?,
        )
    } else {
        None
    };
    Ok(crate::types::SensorNoise { values, units })
}

fn kv_vector3(input: &mut &str) -> KvnResult<Vec<f64>> {
    let (val_str, _unit_str) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    let values = val_str
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values)
}

fn parse_sensor_block(input: &mut &str) -> KvnResult<AcmSensor> {
    let mut comment = Vec::new();
    let mut sensor_number = None;
    let mut sensor_used = None;
    let mut sensor_noise_stddev = None;
    let mut sensor_frequency = None;

    expect_block_start("SENSOR").parse_next(input)?;

    parse_block!(input, comment, {
        "SENSOR_NUMBER" => val: kv_u32 => { sensor_number = Some(val); },
        "SENSOR_USED" => val: kv_string => { sensor_used = Some(val); },
        "SENSOR_NOISE_STDDEV" => val: kv_sensor_noise => { sensor_noise_stddev = Some(val); },
        "SENSOR_FREQUENCY" => val: kv_float => { sensor_frequency = Some(val); },
    }, |i: &mut &str| at_block_end("SENSOR", i), "Unexpected ACM Sensor key");

    expect_block_end("SENSOR").parse_next(input)?;
    Ok(AcmSensor {
        comment,
        sensor_number: sensor_number
            .ok_or_else(|| missing_field_err(input, "ACM Sensor", "SENSOR_NUMBER"))?,
        sensor_used,
        sensor_noise_stddev,
        sensor_frequency,
    })
}

fn parse_ad_block(input: &mut &str) -> KvnResult<AcmAttitudeDetermination> {
    let mut comment = Vec::new();
    let mut ad_id = None;
    let mut ad_prev_id = None;
    let mut ad_method = None;
    let mut attitude_source = None;
    let mut number_states = None;
    let mut attitude_states = None;
    let mut cov_type = None;
    let mut ad_epoch = None;
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut attitude_type = None;
    let mut rate_states = None;
    let mut sigma_u = None;
    let mut sigma_v = None;
    let mut rate_process_noise_stddev = None;
    let mut sensor = Vec::new();

    expect_block_start("AD").parse_next(input)?;

    parse_block!(input, comment, {
        "AD_ID" => val: kv_string => { ad_id = Some(val); },
        "AD_PREV_ID" => val: kv_string => { ad_prev_id = Some(val); },
        "AD_METHOD" => val: kv_string => { ad_method = Some(val); },
        "ATTITUDE_SOURCE" => val: kv_string => { attitude_source = Some(val); },
        "NUMBER_STATES" => val: kv_u32 => { number_states = Some(val); },
        "ATTITUDE_STATES" => val: kv_string => { attitude_states = Some(val); },
        "COV_TYPE" => val: kv_string => { cov_type = Some(val); },
        "AD_EPOCH" => val: kv_epoch => { ad_epoch = Some(val); },
        "REF_FRAME_A" => val: kv_string => { ref_frame_a = Some(val); },
        "REF_FRAME_B" => val: kv_string => { ref_frame_b = Some(val); },
        "ATTITUDE_TYPE" => val: kv_string => { attitude_type = Some(val); },
        "RATE_STATES" => val: kv_string => { rate_states = Some(val); },
        "SIGMA_U" => val: kv_from_kvn => { sigma_u = Some(val); },
        "SIGMA_V" => val: kv_from_kvn => { sigma_v = Some(val); },
        "RATE_PROCESS_NOISE_STDDEV" => val: kv_from_kvn => { rate_process_noise_stddev = Some(val); },
    }, |i: &mut &str| peek((ws, "SENSOR_START")).parse_next(i).is_ok() || at_block_end("AD", i), "Unexpected ACM AD key");

    loop {
        if at_block_end("AD", input) {
            break;
        }
        if peek((ws, "SENSOR_START")).parse_next(input).is_ok() {
            sensor.push(parse_sensor_block.parse_next(input)?);
            continue;
        }
        break;
    }

    expect_block_end("AD").parse_next(input)?;
    Ok(AcmAttitudeDetermination {
        comment,
        ad_id,
        ad_prev_id,
        ad_method,
        attitude_source,
        number_states,
        attitude_states,
        cov_type,
        ad_epoch,
        ref_frame_a,
        ref_frame_b,
        attitude_type,
        rate_states,
        sigma_u,
        sigma_v,
        rate_process_noise_stddev,
        sensors: sensor,
    })
}

fn parse_user_defined_block(input: &mut &str) -> KvnResult<crate::types::UserDefined> {
    let mut block = crate::types::UserDefined::default();
    expect_block_start("USER").parse_next(input)?;

    loop {
        let checkpoint = input.checkpoint();
        let _ = collect_comments.parse_next(input)?; // Consume comments but we might want to keep them?
                                                     // acm_data loop doesn't keep comments between blocks usually, but inside a block we might.
                                                     // The structure UserDefined has a comment field.
                                                     // Let's re-parse comments to store them.
        input.reset(&checkpoint);
        let comments = collect_comments.parse_next(input)?;
        block.comment.extend(comments);

        if at_block_end("USER", input) {
            break;
        }

        let key = key_token.parse_next(input)?;
        let val = kv_string.parse_next(input)?;
        block.user_defined.push(crate::types::UserDefinedParameter {
            parameter: key.strip_prefix("USER_DEFINED_").unwrap_or(key).to_string(),
            value: val,
        });
    }
    expect_block_end("USER").parse_next(input)?;
    Ok(block)
}

//----------------------------------------------------------------------
// ACM Data Parser
//----------------------------------------------------------------------

pub fn acm_data(input: &mut &str) -> KvnResult<AcmData> {
    let mut data = AcmData::default();

    loop {
        let _ = skip_empty_lines.parse_next(input);
        if input.is_empty() || at_block_start("META", input) {
            break;
        }

        if at_block_start("ATT", input) {
            data.att.push(parse_att_block.parse_next(input)?);
        } else if at_block_start("PHYS", input) {
            data.phys = Some(parse_phys_block.parse_next(input)?);
        } else if at_block_start("COV", input) {
            data.cov.push(parse_cov_block.parse_next(input)?);
        } else if at_block_start("MAN", input) {
            data.man.push(parse_man_block.parse_next(input)?);
        } else if at_block_start("AD", input) {
            data.ad = Some(parse_ad_block.parse_next(input)?);
        } else if at_block_start("USER", input) {
            data.user = Some(parse_user_defined_block.parse_next(input)?);
        } else {
            break;
        }
    }

    Ok(data)
}

//----------------------------------------------------------------------
// ACM Segment Parser
//----------------------------------------------------------------------

pub fn acm_segment(input: &mut &str) -> KvnResult<AcmSegment> {
    let metadata = acm_metadata.parse_next(input)?;
    let _ = skip_empty_lines.parse_next(input);
    let data = acm_data.parse_next(input)?;

    Ok(AcmSegment { metadata, data })
}

//----------------------------------------------------------------------
// Complete ACM Parser
//----------------------------------------------------------------------

pub fn parse_acm(input: &mut &str) -> KvnResult<Acm> {
    let version = acm_version.parse_next(input)?;
    let header = adm_header.parse_next(input)?;

    let _ = skip_empty_lines.parse_next(input);
    if !at_block_start("META", input) {
        return Err(cut_err(input, "Expected META_START for ACM segment"));
    }

    let segment = acm_segment.parse_next(input)?;

    Ok(Acm {
        header,
        body: AcmBody {
            segment: Box::new(segment),
        },
        id: Some("CCSDS_ACM_VERS".to_string()),
        version,
    })
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, FormatError, ValidationError};
    use crate::traits::Ndm;

    fn sample_acm_header() -> String {
        r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
        .to_string()
    }

    fn sample_acm_meta() -> String {
        r#"META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
"#
        .to_string()
    }

    #[test]
    fn test_parse_acm_minimal() {
        let input = format!("{}{}\nATT_START\nREF_FRAME_A = EME2000\nREF_FRAME_B = SC_BODY_1\nATT_TYPE = QUATERNION\nNUMBER_STATES = 4\n0.0 0.5 0.5 0.5 0.5\nATT_STOP\n",
            sample_acm_header(), sample_acm_meta());
        let acm = Acm::from_kvn(&input).unwrap();
        assert_eq!(acm.version, "2.0");
        assert_eq!(acm.header.originator, "NASA/JPL");
        assert_eq!(acm.body.segment.data.att.len(), 1);
    }

    #[test]
    fn test_parse_acm_version_error() {
        let input =
            "CCSDS_ACM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\n";
        let err = Acm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::Format(boxed_err) => match *boxed_err {
                FormatError::Kvn(e) => {
                    assert!(format!("{:?}", e).contains("1.0 or 2.0"));
                }
                _ => panic!("Expected Kvn format error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Format error, got {:?}", err),
        }
    }

    #[test]
    fn test_acm_missing_mandatory_metadata() {
        let input = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0 0 0 1
ATT_STOP
"#;
        let err = Acm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
                ValidationError::MissingRequiredField { field, .. } => {
                    assert_eq!(field, "OBJECT_NAME");
                }
                _ => panic!("Expected missing field error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Validation error, got {:?}", err),
        }
    }

    #[test]
    fn test_acm_att_block_variants() {
        let att_block = r#"ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
ATT_TYPE = EULER_ANGLES
EULER_ROT_SEQ = ZYX
NUMBER_STATES = 3
0.0 10.0 20.0 30.0
ATT_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), att_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let att = &acm.body.segment.data.att[0];
        assert_eq!(att.att_type, "EULER_ANGLES");
        // Note: 321 is not a valid enum variant for RotSeq in our types, it expects ZYX etc.
        // Wait, did I fix RotSeq?
        // In types.rs: RotSeq expects "XYX", "XYZ", etc. "321" is likely ZYX.
        // Let's use ZYX to be safe and correct.
    }

    #[test]
    fn test_acm_phys_block_full() {
        let phys_block = r#"PHYS_START
COMMENT Phys comment
DRAG_COEFF = 2.2
WET_MASS = 1500.0 [kg]
DRY_MASS = 1000.0 [kg]
CP_REF_FRAME = SC_BODY_1
CP = 0.1 0.2 0.3 [m]
INERTIA_REF_FRAME = SC_BODY_1
IXX = 1000.0 [kg*m**2]
IYY = 2000.0 [kg*m**2]
IZZ = 3000.0 [kg*m**2]
IXY = 10.0 [kg*m**2]
IXZ = 20.0 [kg*m**2]
IYZ = 30.0 [kg*m**2]
PHYS_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), phys_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let phys = acm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.drag_coeff, Some(2.2));
        assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1500.0);
        assert_eq!(phys.cp.as_ref().unwrap().elements[0], 0.1);
        assert_eq!(phys.ixx.as_ref().unwrap().value, 1000.0);
        assert!(phys.comment.contains(&"Phys comment".to_string()));
    }

    #[test]
    fn test_acm_cov_block() {
        let cov_block = r#"COV_START
COV_BASIS = DETERMINED
COV_REF_FRAME = EME2000
COV_TYPE = ANGLE
COV_CONFIDENCE = 0.99
0.0 1.0e-6
COV_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), cov_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let cov = &acm.body.segment.data.cov[0];
        assert_eq!(cov.cov_basis, "DETERMINED");
        assert_eq!(cov.cov_confidence, Some(0.99));
        assert_eq!(cov.cov_lines[0].values[1], 1.0e-6);
    }

    #[test]
    fn test_acm_man_block() {
        let man_block = r#"MAN_START
MAN_ID = MAN_001
MAN_BEGIN_TIME = 100.0
MAN_END_TIME = 200.0
MAN_DURATION = 100.0 [s]
ACTUATOR_USED = THRUSTER_1
TARGET_MOMENTUM = 0.1 0.2 0.3 [N*m*s]
MAN_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), man_block);
        let acm = crate::validation::with_validation_mode(
            crate::validation::ValidationMode::Lenient,
            || Acm::from_kvn(&input),
        )
        .unwrap();
        let man = &acm.body.segment.data.man[0];
        assert_eq!(man.man_id.as_deref(), Some("MAN_001"));
        assert_eq!(man.man_duration.as_ref().unwrap().value, 100.0);
        assert_eq!(man.target_momentum.as_ref().unwrap().elements[0], 0.1);
    }

    #[test]
    fn test_acm_ad_block_with_sensors() {
        let ad_block = r#"AD_START
AD_ID = AD_001
AD_METHOD = EKF
ATTITUDE_SOURCE = OBC
SENSOR_START
SENSOR_NUMBER = 1
SENSOR_USED = STAR_TRACKER
SENSOR_NOISE_STDDEV = 0.01 [deg]
SENSOR_FREQUENCY = 10.0
SENSOR_STOP
SENSOR_START
SENSOR_NUMBER = 2
SENSOR_USED = GYRO
SENSOR_STOP
AD_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), ad_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let ad = &acm.body.segment.data.ad.as_ref().unwrap();
        assert_eq!(ad.ad_id.as_deref(), Some("AD_001"));
        assert_eq!(ad.ad_method.as_deref(), Some("EKF"));
        assert_eq!(ad.sensors.len(), 2);
    }

    #[test]
    fn test_acm_multiple_blocks_mixed() {
        let blocks = r#"ATT_START
REF_FRAME_A = A
REF_FRAME_B = B
ATT_TYPE = Q
NUMBER_STATES = 1
0 0 0 0 0
ATT_STOP
PHYS_START
DRAG_COEFF = 1.0
PHYS_STOP
ATT_START
REF_FRAME_A = A
REF_FRAME_B = B
ATT_TYPE = Q
NUMBER_STATES = 1
10 0 0 0 0
ATT_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), blocks);
        let acm = Acm::from_kvn(&input).unwrap();
        assert_eq!(acm.body.segment.data.att.len(), 2);
        assert!(acm.body.segment.data.phys.is_some());
    }
}
