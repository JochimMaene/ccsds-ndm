// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for ACM (Attitude Comprehensive Message).

use super::{
    Acm, AcmAttitudeDetermination, AcmAttitudeState, AcmBody, AcmCovarianceMatrix, AcmData,
    AcmManeuverParameters, AcmMetadata, AcmPhysicalDescription, AcmSegment, AcmSensor, AttLine,
    CovLine,
};
use crate::error::{
    CcsdsNdmError, FormatError, InternalParserError, KvnParseError, Result, ValidationError,
};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;

use std::borrow::Cow;
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
        "EPOCH_TZERO" => val: kv_calendar_epoch => { epoch_tzero = Some(val); },
        "TAIMUTC_AT_TZERO" => val: kv_from_kvn => { taimutc_at_tzero = Some(val); },
        "NEXT_LEAP_EPOCH" => val: kv_calendar_epoch => { next_leap_epoch = Some(val); },
        "NEXT_LEAP_TAIMUTC" => val: kv_from_kvn => { next_leap_taimutc = Some(val); },
        "ACM_DATA_ELEMENTS" => val: kv_string => { acm_data_elements = Some(val); },
        "START_TIME" => val: kv_epoch => { start_time = Some(val); },
        "STOP_TIME" => val: kv_epoch => { stop_time = Some(val); },
    }, |i: &mut &str| at_block_end("META", i));

    expect_block_end("META").parse_next(input)?;

    Ok(AcmMetadata {
        comment,
        object_name: require_field(input, "ACM Metadata", "OBJECT_NAME", object_name)?,
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
        time_system: require_field(input, "ACM Metadata", "TIME_SYSTEM", time_system)?,
        epoch_tzero: require_field(input, "ACM Metadata", "EPOCH_TZERO", epoch_tzero)?,
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
    let mut values = Vec::with_capacity(line.split_whitespace().count());
    for value in line.split_whitespace() {
        values.push(
            value
                .parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))?,
        );
    }
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
        "ATT_TYPE" => val: kv_enum => { att_type = Some(val); },
        "RATE_TYPE" => val: kv_enum => { rate_type = Some(val); },
        "EULER_ROT_SEQ" => val: kv_enum => { euler_rot_seq = Some(val); },
    }, |i: &mut &str| matches!(i.trim_start().chars().next(), Some('0'..='9' | '-' | '+')) || at_block_end("ATT", i));

    loop {
        if at_block_end("ATT", input) {
            break;
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
        ref_frame_a: require_field(input, "ACM ATT", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "ACM ATT", "REF_FRAME_B", ref_frame_b)?,
        number_states: require_field(input, "ACM ATT", "NUMBER_STATES", number_states)?,
        att_type: require_field(input, "ACM ATT", "ATT_TYPE", att_type)?,
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
        "CP" => val: kv_cp => { block.cp = Some(val); },
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
    let mut values = Vec::with_capacity(line.split_whitespace().count());
    for value in line.split_whitespace() {
        values.push(
            value
                .parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))?,
        );
    }
    Ok(CovLine { values })
}

fn parse_cov_block(input: &mut &str) -> KvnResult<AcmCovarianceMatrix> {
    let mut comment = Vec::new();
    let mut cov_id = None;
    let mut cov_prev_id = None;
    let mut cov_basis = None;
    let mut cov_basis_id = None;
    let mut cov_ref_frame = None;
    let mut cov_type = None;
    let mut cov_lines = Vec::new();

    expect_block_start("COV").parse_next(input)?;

    parse_block!(input, comment, {
        "COV_ID" => val: kv_string => { cov_id = Some(val); },
        "COV_PREV_ID" => val: kv_string => { cov_prev_id = Some(val); },
        "COV_BASIS" => val: kv_enum => { cov_basis = Some(val); },
        "COV_BASIS_ID" => val: kv_string => { cov_basis_id = Some(val); },
        "COV_REF_FRAME" => val: kv_string => { cov_ref_frame = Some(val); },
        "COV_TYPE" => val: kv_enum => { cov_type = Some(val); },
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
        cov_id,
        cov_prev_id,
        cov_basis,
        cov_basis_id,
        cov_ref_frame,
        cov_type: cov_type.ok_or_else(|| missing_field_err(input, "ACM COV", "COV_TYPE"))?,
        cov_confidence: None,
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
        .collect::<std::result::Result<Vec<_>, _>>()?;

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
    let mut target_attitude = None;
    let mut target_spinrate = None;

    expect_block_start("MAN").parse_next(input)?;

    parse_block!(input, comment, {
        "MAN_ID" => val: kv_string => { man_id = Some(val); },
        "MAN_PREV_ID" => val: kv_string => { man_prev_id = Some(val); },
        "MAN_PURPOSE" => val: kv_string => { man_purpose = Some(val); },
        "MAN_BEGIN_TIME" => val: kv_relative_time => { man_begin_time = Some(val); },
        "MAN_END_TIME" => val: kv_relative_time => { man_end_time = Some(val); },
        "MAN_DURATION" => val: kv_from_kvn => { man_duration = Some(val); },
        "ACTUATOR_USED" => val: kv_string => { actuator_used = Some(val); },
        "TARGET_MOMENTUM" => val: kv_target_momentum => { target_momentum = Some(val); },
        "TARGET_MOM_FRAME" => val: kv_string => { target_mom_frame = Some(val); },
        "TARGET_ATTITUDE" => val: kv_from_kvn_value => { target_attitude = Some(val); },
        "TARGET_SPINRATE" => val: kv_from_kvn => { target_spinrate = Some(val); },
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
        target_attitude,
        target_spinrate,
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
        .collect::<std::result::Result<Vec<_>, _>>()?;

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

fn kv_cp(input: &mut &str) -> KvnResult<crate::types::Vector3> {
    let (val_str, unit_str) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    let values = val_str
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Cut(InternalParserError::from_input(input)))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let units = unit_str
        .map(crate::types::LengthUnits::from_str)
        .transpose()
        .map_err(|error| ErrMode::Cut(InternalParserError::from_external_error(input, error)))?;
    Ok(crate::types::Vector3 {
        elements: values,
        units,
    })
}

fn parse_sensor_block(input: &mut &str) -> KvnResult<AcmSensor> {
    let mut comment = Vec::new();
    let mut sensor_number = None;
    let mut sensor_used = None;
    let mut number_sensor_noise_covariance = None;
    let mut sensor_noise_stddev = None;
    let mut sensor_frequency = None;

    expect_block_start("SENSOR").parse_next(input)?;

    parse_block!(input, comment, {
        "SENSOR_NUMBER" => val: kv_u32 => { sensor_number = Some(val); },
        "SENSOR_USED" => val: kv_string => { sensor_used = Some(val); },
        "NUMBER_SENSOR_NOISE_COVARIANCE" => val: kv_u32 => { number_sensor_noise_covariance = Some(val); },
        "SENSOR_NOISE_STDDEV" => val: kv_sensor_noise => { sensor_noise_stddev = Some(val); },
        "SENSOR_FREQUENCY" => val: kv_from_kvn => { sensor_frequency = Some(val); },
    }, |i: &mut &str| at_block_end("SENSOR", i), "Unexpected ACM Sensor key");

    expect_block_end("SENSOR").parse_next(input)?;
    Ok(AcmSensor {
        comment,
        sensor_number,
        sensor_used,
        number_sensor_noise_covariance,
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
    let mut euler_rot_seq = None;
    let mut cov_type = None;
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
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
        "ATTITUDE_STATES" => val: kv_enum => { attitude_states = Some(val); },
        "EULER_ROT_SEQ" => val: kv_enum => { euler_rot_seq = Some(val); },
        "COV_TYPE" => val: kv_enum => { cov_type = Some(val); },
        "REF_FRAME_A" => val: kv_string => { ref_frame_a = Some(val); },
        "REF_FRAME_B" => val: kv_string => { ref_frame_b = Some(val); },
        "RATE_STATES" => val: kv_enum => { rate_states = Some(val); },
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
        euler_rot_seq,
        cov_type,
        ad_epoch: None,
        ref_frame_a,
        ref_frame_b,
        attitude_type: None,
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
        let _ = collect_comments.parse_next(input)?;
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

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const HEADER: &[&str] = &[
        "CLASSIFICATION",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
    ];
    const META: &[&str] = &[
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_DESIGNATOR",
        "ORIGINATOR_POC",
        "ORIGINATOR_POSITION",
        "ORIGINATOR_PHONE",
        "ORIGINATOR_EMAIL",
        "ORIGINATOR_ADDRESS",
        "ODM_MSG_LINK",
        "CENTER_NAME",
        "TIME_SYSTEM",
        "EPOCH_TZERO",
        "ACM_DATA_ELEMENTS",
        "START_TIME",
        "STOP_TIME",
        "TAIMUTC_AT_TZERO",
        "NEXT_LEAP_EPOCH",
        "NEXT_LEAP_TAIMUTC",
    ];
    const ATT: &[&str] = &[
        "ATT_ID",
        "ATT_PREV_ID",
        "ATT_BASIS",
        "ATT_BASIS_ID",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "NUMBER_STATES",
        "ATT_TYPE",
        "EULER_ROT_SEQ",
        "RATE_TYPE",
    ];
    const PHYS: &[&str] = &[
        "DRAG_COEFF",
        "WET_MASS",
        "DRY_MASS",
        "CP_REF_FRAME",
        "CP",
        "INERTIA_REF_FRAME",
        "IXX",
        "IYY",
        "IZZ",
        "IXY",
        "IXZ",
        "IYZ",
    ];
    const COV: &[&str] = &[
        "COV_ID",
        "COV_PREV_ID",
        "COV_BASIS",
        "COV_BASIS_ID",
        "COV_REF_FRAME",
        "COV_TYPE",
    ];
    const MAN: &[&str] = &[
        "MAN_ID",
        "MAN_PREV_ID",
        "MAN_PURPOSE",
        "MAN_BEGIN_TIME",
        "MAN_END_TIME",
        "MAN_DURATION",
        "ACTUATOR_USED",
        "TARGET_MOMENTUM",
        "TARGET_MOM_FRAME",
        "TARGET_ATTITUDE",
        "TARGET_SPINRATE",
    ];
    const AD: &[&str] = &[
        "AD_ID",
        "AD_PREV_ID",
        "AD_METHOD",
        "ATTITUDE_SOURCE",
        "NUMBER_STATES",
        "ATTITUDE_STATES",
        "EULER_ROT_SEQ",
        "COV_TYPE",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "RATE_STATES",
        "SIGMA_U",
        "SIGMA_V",
        "RATE_PROCESS_NOISE_STDDEV",
    ];
    const SENSOR: &[&str] = &[
        "SENSOR_NUMBER",
        "SENSOR_USED",
        "NUMBER_SENSOR_NOISE_COVARIANCE",
        "SENSOR_NOISE_STDDEV",
        "SENSOR_FREQUENCY",
    ];

    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating ACM KVN structure"],
            offset,
        }))))
    };
    let block_keys = |block: &str| -> Option<&[&str]> {
        Some(match block {
            "META" => META,
            "ATT" => ATT,
            "PHYS" => PHYS,
            "COV" => COV,
            "MAN" => MAN,
            "AD" => AD,
            "SENSOR" => SENSOR,
            "USER" => return None,
            _ => return None,
        })
    };
    let outer_rank = |block: &str| match block {
        "META" => Some(0usize),
        "ATT" => Some(1),
        "PHYS" => Some(2),
        "COV" => Some(3),
        "MAN" => Some(4),
        "AD" => Some(5),
        "USER" => Some(6),
        _ => None,
    };

    let mut block: Option<&str> = None;
    let mut previous_key = None;
    let mut top_rank = None;
    let mut last_outer_rank = None;
    let mut seen_nonrepeatable = 0u8;
    let mut block_has_content = false;
    let mut history_started = false;
    let mut ad_sensor_started = false;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
        if line.as_bytes().contains(&b'\r') {
            return fail("lone carriage return");
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
            if block.is_none() && top_rank != Some(0) {
                return fail("ACM header COMMENT must immediately follow the version record");
            }
            if block_has_content && block.is_some() {
                return fail("COMMENT is not at the beginning of an ACM logical block");
            }
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if marker == "SENSOR" {
                if block != Some("AD") || !ad_sensor_started && history_started {
                    return fail("SENSOR block must be nested directly in AD");
                }
                block = Some("SENSOR");
                previous_key = None;
                block_has_content = false;
                history_started = false;
                ad_sensor_started = true;
                offset += raw_line.len() + 1;
                continue;
            }
            if block.is_some() {
                return fail("unknown or nested ACM marked block");
            }
            let rank = outer_rank(marker)
                .ok_or_else(|| invalid(number, offset, "unknown ACM marked block".into()))?;
            if rank == 0 && last_outer_rank.is_some() {
                return fail("duplicate ACM metadata block");
            }
            if last_outer_rank.is_some_and(|previous| rank < previous) {
                return fail("out-of-order ACM marked block");
            }
            if !matches!(marker, "ATT" | "COV" | "MAN") {
                let bit = 1u8 << rank;
                if seen_nonrepeatable & bit != 0 {
                    return fail("duplicate non-repeatable ACM marked block");
                }
                seen_nonrepeatable |= bit;
            }
            if rank != 0 && last_outer_rank.is_none() {
                return fail("META must be the first ACM marked block");
            }
            last_outer_rank = Some(rank);
            block = Some(marker);
            previous_key = None;
            block_has_content = false;
            history_started = false;
            ad_sensor_started = false;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if marker == "SENSOR" {
                if block != Some("SENSOR") {
                    return fail("mismatched ACM SENSOR block end");
                }
                block = Some("AD");
                previous_key = None;
                block_has_content = true;
                history_started = false;
                offset += raw_line.len() + 1;
                continue;
            }
            if block != Some(marker) {
                return fail("mismatched ACM marked block end");
            }
            block = None;
            previous_key = None;
            block_has_content = false;
            history_started = false;
            offset += raw_line.len() + 1;
            continue;
        }

        match block {
            None => {
                if last_outer_rank.is_some() {
                    return fail("content outside an ACM marked block");
                }
                if !line.contains('=') {
                    return fail("expected one ACM header assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = if key == "CCSDS_ACM_VERS" {
                    0
                } else {
                    HEADER
                        .iter()
                        .position(|candidate| *candidate == key)
                        .map(|rank| rank + 1)
                        .ok_or_else(|| {
                            invalid(number, offset, "unknown ACM header keyword".into())
                        })?
                };
                if top_rank.is_none() && rank != 0 {
                    return fail("CCSDS_ACM_VERS must be the first record");
                }
                if top_rank.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order ACM header keyword");
                }
                top_rank = Some(rank);
                block_has_content = true;
            }
            Some("ATT" | "COV") if !line.contains('=') => {
                history_started = true;
                block_has_content = true;
            }
            Some("USER") => {
                if !line.contains('=')
                    || !line
                        .split_once('=')
                        .unwrap()
                        .0
                        .trim()
                        .starts_with("USER_DEFINED_")
                {
                    return fail("invalid ACM user-defined assignment");
                }
                block_has_content = true;
            }
            Some(current) => {
                if history_started {
                    return fail("assignment after ACM history data");
                }
                if current == "AD" && ad_sensor_started {
                    return fail("AD assignment after SENSOR block");
                }
                if !line.contains('=') {
                    return fail("expected one ACM block assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let keys = block_keys(current)
                    .ok_or_else(|| invalid(number, offset, "unknown ACM marked block".into()))?;
                let rank = keys
                    .iter()
                    .position(|candidate| *candidate == key)
                    .ok_or_else(|| invalid(number, offset, "unknown ACM keyword".into()))?;
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order ACM keyword");
                }
                previous_key = Some(rank);
                block_has_content = true;
            }
        }
        offset += raw_line.len() + 1;
    }
    if block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unclosed ACM marked block".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Acm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.allow_long_records();
        writer.write_pair("CCSDS_ACM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for AcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for AcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for AcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        if let Some(v) = &self.international_designator {
            writer.write_pair("INTERNATIONAL_DESIGNATOR", v);
        }
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(v) = &self.originator_poc {
            writer.write_pair("ORIGINATOR_POC", v);
        }
        if let Some(v) = &self.originator_position {
            writer.write_pair("ORIGINATOR_POSITION", v);
        }
        if let Some(v) = &self.originator_phone {
            writer.write_pair("ORIGINATOR_PHONE", v);
        }
        if let Some(v) = &self.originator_email {
            writer.write_pair("ORIGINATOR_EMAIL", v);
        }
        if let Some(v) = &self.originator_address {
            writer.write_pair("ORIGINATOR_ADDRESS", v);
        }
        if let Some(v) = &self.odm_msg_link {
            writer.write_pair("ODM_MSG_LINK", v);
        }
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.acm_data_elements {
            writer.write_pair("ACM_DATA_ELEMENTS", v);
        }
        if let Some(v) = self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        if let Some(v) = &self.taimutc_at_tzero {
            writer.write_odm_float_measure("TAIMUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.next_leap_epoch {
            writer.write_pair("NEXT_LEAP_EPOCH", v);
        }
        if let Some(v) = &self.next_leap_taimutc {
            writer.write_odm_float_measure("NEXT_LEAP_TAIMUTC", &v.to_unit_value());
        }
        writer.write_section("META_STOP");
    }
}

impl ToKvn for AcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for att in &self.att {
            att.write_kvn(writer);
        }
        if let Some(phys) = &self.phys {
            phys.write_kvn(writer);
        }
        for cov in &self.cov {
            cov.write_kvn(writer);
        }
        for man in &self.man {
            man.write_kvn(writer);
        }
        if let Some(ad) = &self.ad {
            ad.write_kvn(writer);
        }
        if let Some(user) = &self.user {
            writer.write_section("USER_START");
            writer.write_comments(&user.comment);
            for p in &user.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

impl ToKvn for AcmAttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("ATT_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.att_id {
            writer.write_pair("ATT_ID", v);
        }
        if let Some(v) = &self.att_prev_id {
            writer.write_pair("ATT_PREV_ID", v);
        }
        if let Some(v) = &self.att_basis {
            writer.write_pair("ATT_BASIS", v);
        }
        if let Some(v) = &self.att_basis_id {
            writer.write_pair("ATT_BASIS_ID", v);
        }
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("NUMBER_STATES", self.number_states);
        writer.write_pair("ATT_TYPE", &self.att_type);
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.rate_type {
            writer.write_pair("RATE_TYPE", v);
        }
        for line in &self.att_lines {
            writer.write_numeric_record(&line.values);
        }
        writer.write_section("ATT_STOP");
    }
}

impl ToKvn for AcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = self.drag_coeff {
            writer.write_odm_float_pair("DRAG_COEFF", v);
        }
        if let Some(v) = &self.wet_mass {
            writer.write_odm_float_measure("WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.dry_mass {
            writer.write_odm_float_measure("DRY_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.cp_ref_frame {
            writer.write_pair("CP_REF_FRAME", v);
        }
        if let Some(v) = &self.cp {
            writer.write_numeric_vector("CP", &v.elements, v.units.as_ref());
        }
        if let Some(v) = &self.inertia_ref_frame {
            writer.write_pair("INERTIA_REF_FRAME", v);
        }
        if let Some(v) = &self.ixx {
            writer.write_odm_float_measure("IXX", v);
        }
        if let Some(v) = &self.iyy {
            writer.write_odm_float_measure("IYY", v);
        }
        if let Some(v) = &self.izz {
            writer.write_odm_float_measure("IZZ", v);
        }
        if let Some(v) = &self.ixy {
            writer.write_odm_float_measure("IXY", v);
        }
        if let Some(v) = &self.ixz {
            writer.write_odm_float_measure("IXZ", v);
        }
        if let Some(v) = &self.iyz {
            writer.write_odm_float_measure("IYZ", v);
        }
        writer.write_section("PHYS_STOP");
    }
}

impl ToKvn for AcmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COV_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.cov_id {
            writer.write_pair("COV_ID", v);
        }
        if let Some(v) = &self.cov_prev_id {
            writer.write_pair("COV_PREV_ID", v);
        }
        if let Some(v) = &self.cov_basis {
            writer.write_pair("COV_BASIS", v);
        }
        if let Some(v) = &self.cov_basis_id {
            writer.write_pair("COV_BASIS_ID", v);
        }
        if let Some(v) = &self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", v);
        }
        writer.write_pair("COV_TYPE", &self.cov_type);
        for line in &self.cov_lines {
            writer.write_numeric_record(&line.values);
        }
        writer.write_section("COV_STOP");
    }
}

impl ToKvn for AcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.man_id {
            writer.write_pair("MAN_ID", v);
        }
        if let Some(v) = &self.man_prev_id {
            writer.write_pair("MAN_PREV_ID", v);
        }
        if let Some(v) = &self.man_purpose {
            writer.write_pair("MAN_PURPOSE", v);
        }
        if let Some(v) = &self.man_begin_time {
            writer.write_pair("MAN_BEGIN_TIME", v);
        }
        if let Some(v) = &self.man_end_time {
            writer.write_pair("MAN_END_TIME", v);
        }
        if let Some(v) = &self.man_duration {
            writer.write_odm_float_measure("MAN_DURATION", &v.to_unit_value());
        }
        if let Some(v) = &self.actuator_used {
            writer.write_pair("ACTUATOR_USED", v);
        }
        if let Some(v) = &self.target_momentum {
            writer.write_numeric_vector("TARGET_MOMENTUM", &v.elements, v.units.as_ref());
        }
        if let Some(v) = &self.target_mom_frame {
            writer.write_pair("TARGET_MOM_FRAME", v);
        }
        if let Some(v) = &self.target_attitude {
            writer.write_numeric_vector("TARGET_ATTITUDE", &v.values, None::<&&str>);
        }
        if let Some(v) = &self.target_spinrate {
            writer.write_odm_float_measure("TARGET_SPINRATE", v);
        }
        writer.write_section("MAN_STOP");
    }
}

impl ToKvn for AcmAttitudeDetermination {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("AD_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.ad_id {
            writer.write_pair("AD_ID", v);
        }
        if let Some(v) = &self.ad_prev_id {
            writer.write_pair("AD_PREV_ID", v);
        }
        if let Some(v) = &self.ad_method {
            writer.write_pair("AD_METHOD", v);
        }
        if let Some(v) = &self.attitude_source {
            writer.write_pair("ATTITUDE_SOURCE", v);
        }
        if let Some(v) = &self.number_states {
            writer.write_pair("NUMBER_STATES", v);
        }
        if let Some(v) = &self.attitude_states {
            writer.write_pair("ATTITUDE_STATES", v);
        }
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.cov_type {
            writer.write_pair("COV_TYPE", v);
        }
        if let Some(v) = &self.ref_frame_a {
            writer.write_pair("REF_FRAME_A", v);
        }
        if let Some(v) = &self.ref_frame_b {
            writer.write_pair("REF_FRAME_B", v);
        }
        if let Some(v) = &self.rate_states {
            writer.write_pair("RATE_STATES", v);
        }
        if let Some(v) = &self.sigma_u {
            writer.write_odm_float_measure("SIGMA_U", v);
        }
        if let Some(v) = &self.sigma_v {
            writer.write_odm_float_measure("SIGMA_V", v);
        }
        if let Some(v) = &self.rate_process_noise_stddev {
            writer.write_odm_float_measure("RATE_PROCESS_NOISE_STDDEV", v);
        }
        for sensor in &self.sensors {
            sensor.write_kvn(writer);
        }
        writer.write_section("AD_STOP");
    }
}

impl ToKvn for AcmSensor {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("SENSOR_START");
        writer.write_comments(&self.comment);
        if let Some(v) = self.sensor_number {
            writer.write_pair("SENSOR_NUMBER", v);
        }
        if let Some(v) = &self.sensor_used {
            writer.write_pair("SENSOR_USED", v);
        }
        if let Some(v) = self.number_sensor_noise_covariance {
            writer.write_pair("NUMBER_SENSOR_NOISE_COVARIANCE", v);
        }
        if let Some(v) = &self.sensor_noise_stddev {
            writer.write_numeric_vector("SENSOR_NOISE_STDDEV", &v.values, v.units.as_ref());
        }
        if let Some(v) = &self.sensor_frequency {
            writer.write_odm_float_measure("SENSOR_FREQUENCY", &v.to_unit_value());
        }
        writer.write_section("SENSOR_STOP");
    }
}

impl ParseKvn for Acm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_acm(input)
    }
}

impl std::fmt::Display for AttLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for CovLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

impl Acm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        let invalid_number = || {
            CcsdsNdmError::Validation(Box::new(ValidationError::Generic {
                message: Cow::Borrowed("ACM KVN numbers must be representable CCSDS numbers"),
                line: None,
            }))
        };
        let check_number = |value: f64| {
            if crate::kvn::ser::OdmFloat::is_valid(value) {
                Ok(())
            } else {
                Err(invalid_number())
            }
        };
        let check_values = |values: &[f64]| {
            if values
                .iter()
                .all(|value| crate::kvn::ser::OdmFloat::is_valid(*value))
            {
                Ok(())
            } else {
                Err(invalid_number())
            }
        };
        let check_text = |value: &str| -> Result<()> {
            if value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                Ok(())
            } else {
                Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "ACM KVN free text must contain printable ASCII without line breaks",
                    ),
                    line: None,
                }
                .into())
            }
        };

        for value in self
            .header
            .comment
            .iter()
            .chain(self.body.segment.metadata.comment.iter())
        {
            check_text(value)?;
        }
        for value in [
            self.header.classification.as_ref(),
            Some(&self.header.originator),
            self.header.message_id.as_ref(),
            Some(&self.body.segment.metadata.object_name),
            self.body.segment.metadata.international_designator.as_ref(),
            self.body.segment.metadata.catalog_name.as_ref(),
            self.body.segment.metadata.object_designator.as_ref(),
            self.body.segment.metadata.originator_poc.as_ref(),
            self.body.segment.metadata.originator_position.as_ref(),
            self.body.segment.metadata.originator_phone.as_ref(),
            self.body.segment.metadata.originator_email.as_ref(),
            self.body.segment.metadata.originator_address.as_ref(),
            self.body.segment.metadata.odm_msg_link.as_ref(),
            self.body.segment.metadata.center_name.as_ref(),
            Some(&self.body.segment.metadata.time_system),
            self.body.segment.metadata.acm_data_elements.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            check_text(value)?;
        }
        if let Some(value) = &self.body.segment.metadata.taimutc_at_tzero {
            check_number(value.value)?;
        }
        if let Some(value) = &self.body.segment.metadata.next_leap_taimutc {
            check_number(value.value)?;
        }

        let data = &self.body.segment.data;
        for attitude in &data.att {
            for value in attitude.comment.iter().chain(
                [
                    attitude.att_id.as_ref(),
                    attitude.att_prev_id.as_ref(),
                    attitude.att_basis_id.as_ref(),
                    Some(&attitude.ref_frame_a),
                    Some(&attitude.ref_frame_b),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for line in &attitude.att_lines {
                check_values(&line.values)?;
            }
        }
        if let Some(physical) = &data.phys {
            for value in physical.comment.iter().chain(
                [
                    physical.cp_ref_frame.as_ref(),
                    physical.inertia_ref_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for value in [
                physical.drag_coeff,
                physical.wet_mass.as_ref().map(|value| value.value),
                physical.dry_mass.as_ref().map(|value| value.value),
                physical.ixx.as_ref().map(|value| value.value),
                physical.iyy.as_ref().map(|value| value.value),
                physical.izz.as_ref().map(|value| value.value),
                physical.ixy.as_ref().map(|value| value.value),
                physical.ixz.as_ref().map(|value| value.value),
                physical.iyz.as_ref().map(|value| value.value),
            ]
            .into_iter()
            .flatten()
            {
                check_number(value)?;
            }
            if let Some(value) = &physical.cp {
                check_values(&value.elements)?;
            }
        }
        for covariance in &data.cov {
            for value in covariance.comment.iter().chain(
                [
                    covariance.cov_id.as_ref(),
                    covariance.cov_prev_id.as_ref(),
                    covariance.cov_basis_id.as_ref(),
                    covariance.cov_ref_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for line in &covariance.cov_lines {
                check_values(&line.values)?;
            }
        }
        for maneuver in &data.man {
            for value in maneuver.comment.iter().chain(
                [
                    maneuver.man_id.as_ref(),
                    maneuver.man_prev_id.as_ref(),
                    maneuver.man_purpose.as_ref(),
                    maneuver.actuator_used.as_ref(),
                    maneuver.target_mom_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            if let Some(value) = &maneuver.man_duration {
                check_number(value.value)?;
            }
            if let Some(value) = &maneuver.target_momentum {
                check_values(&value.elements)?;
            }
            if let Some(value) = &maneuver.target_attitude {
                check_values(&value.values)?;
            }
            if let Some(value) = &maneuver.target_spinrate {
                check_number(value.value)?;
            }
        }
        if let Some(determination) = &data.ad {
            for value in determination.comment.iter().chain(
                [
                    determination.ad_id.as_ref(),
                    determination.ad_prev_id.as_ref(),
                    determination.ad_method.as_ref(),
                    determination.attitude_source.as_ref(),
                    determination.ref_frame_a.as_ref(),
                    determination.ref_frame_b.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for value in [
                determination.sigma_u.as_ref().map(|value| value.value),
                determination.sigma_v.as_ref().map(|value| value.value),
                determination
                    .rate_process_noise_stddev
                    .as_ref()
                    .map(|value| value.value),
            ]
            .into_iter()
            .flatten()
            {
                check_number(value)?;
            }
            for sensor in &determination.sensors {
                for value in sensor.comment.iter().chain(sensor.sensor_used.iter()) {
                    check_text(value)?;
                }
                if let Some(value) = &sensor.sensor_noise_stddev {
                    check_values(&value.values)?;
                }
                if let Some(value) = &sensor.sensor_frequency {
                    check_number(value.value)?;
                }
            }
        }
        if let Some(user) = &data.user {
            for value in user.comment.iter().chain(
                user.user_defined
                    .iter()
                    .flat_map(|parameter| [&parameter.parameter, &parameter.value]),
            ) {
                check_text(value)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, ValidationError};
    use crate::traits::Ndm;
    use crate::types::AcmAttitudeType;

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
        let input = format!("{}{}\nATT_START\nREF_FRAME_A = EME2000\nREF_FRAME_B = SC_BODY_1\nNUMBER_STATES = 4\nATT_TYPE = QUATERNION\n0.0 0.5 0.5 0.5 0.5\nATT_STOP\n",
            sample_acm_header(), sample_acm_meta());
        let acm = Acm::from_kvn(&input).unwrap();
        assert_eq!(acm.version, "2.0");
        assert_eq!(acm.header.originator, "NASA/JPL");
        assert_eq!(acm.body.segment.data.att.len(), 1);
    }

    #[test]
    fn test_parse_acm_version_error() {
        let input = r#"CCSDS_ACM_VERS = 3.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0.5 0.5 0.5 0.5
ATT_STOP
"#;
        let err = Acm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::UnsupportedInputVersion { version, .. } => {
                assert_eq!(version, "3.0");
            }
            _ => panic!("Expected unsupported input version, got {:?}", err),
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
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
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
NUMBER_STATES = 3
ATT_TYPE = EULER_ANGLES
EULER_ROT_SEQ = ZYX
0.0 10.0 20.0 30.0
ATT_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), att_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let att = &acm.body.segment.data.att[0];
        assert_eq!(att.att_type, AcmAttitudeType::EulerAngles);
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
COV_BASIS = DETERMINED_OBC
COV_REF_FRAME = EME2000
COV_TYPE = ANGLE
0.0 1.0e-6
COV_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), cov_block);
        let acm = Acm::from_kvn(&input).unwrap();
        let cov = &acm.body.segment.data.cov[0];
        assert_eq!(
            cov.cov_basis.as_ref().unwrap().to_string(),
            "DETERMINED_OBC"
        );
        assert_eq!(cov.cov_confidence, None);
        assert_eq!(cov.cov_lines[0].values[1], 1.0e-6);
    }

    #[test]
    fn test_acm_man_block() {
        let man_block = r#"MAN_START
MAN_ID = MAN_001
MAN_PURPOSE = ATT_ADJUST
MAN_BEGIN_TIME = 100.0
MAN_DURATION = 100.0 [s]
ACTUATOR_USED = THRUSTER_1
TARGET_MOMENTUM = 0.1 0.2 0.3 [N*m*s]
TARGET_MOM_FRAME = J2000
MAN_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), man_block);
        let acm = Acm::from_kvn(&input).unwrap();
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
ATTITUDE_STATES = QUATERNION
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
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
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0 0 0 0 0
ATT_STOP
ATT_START
REF_FRAME_A = A
REF_FRAME_B = B
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
10 0 0 0 0
ATT_STOP
PHYS_START
DRAG_COEFF = 1.0
PHYS_STOP
"#;
        let input = format!("{}{}{}", sample_acm_header(), sample_acm_meta(), blocks);
        let acm = Acm::from_kvn(&input).unwrap();
        assert_eq!(acm.body.segment.data.att.len(), 2);
        assert!(acm.body.segment.data.phys.is_some());
    }
}
