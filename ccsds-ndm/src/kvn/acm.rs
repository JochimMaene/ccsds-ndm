// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for ACM (Attitude Comprehensive Message).

use crate::kvn::parser::*;
use crate::messages::acm::{Acm, AcmBody, AcmData, AcmMetadata, AcmSegment, AcmAttitudeState, AttLine, AcmPhysicalDescription, AcmCovarianceMatrix, CovLine, AcmManeuverParameters, AcmAttitudeDetermination, AcmSensor};
use crate::parse_block;
use crate::error::InternalParserError;
use crate::traits::Ndm;
use std::str::FromStr;
use winnow::error::{ErrMode, ContextError, ParserError, AddContext, FromExternalError};
use winnow::combinator::terminated;
use winnow::prelude::*;

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
    let mut metadata = AcmMetadata::default();

    expect_block_start("META").parse_next(input)?;

    parse_block!(input, metadata.comment, {
        "OBJECT_NAME" => val: kv_string => { metadata.object_name = val; },
        "INTERNATIONAL_DESIGNATOR" => val: kv_string => { metadata.international_designator = val; },
        "CATALOG_NAME" => val: kv_string => { metadata.catalog_name = Some(val); },
        "OBJECT_DESIGNATOR" => val: kv_string => { metadata.object_designator = Some(val); },
        "ALTERNATE_NAMES" => val: kv_string => { metadata.alternate_names = Some(val); },
        "ORIGINATOR_POC" => val: kv_string => { metadata.originator_poc = Some(val); },
        "ORIGINATOR_POSITION" => val: kv_string => { metadata.originator_position = Some(val); },
        "ORIGINATOR_PHONE" => val: kv_string => { metadata.originator_phone = Some(val); },
        "ORIGINATOR_EMAIL" => val: kv_string => { metadata.originator_email = Some(val); },
        "ORIGINATOR_ADDRESS" => val: kv_string => { metadata.originator_address = Some(val); },
        "TECH_ORG" => val: kv_string => { metadata.tech_org = Some(val); },
        "TECH_POC" => val: kv_string => { metadata.tech_poc = Some(val); },
        "TECH_POSITION" => val: kv_string => { metadata.tech_position = Some(val); },
        "TECH_PHONE" => val: kv_string => { metadata.tech_phone = Some(val); },
        "TECH_EMAIL" => val: kv_string => { metadata.tech_email = Some(val); },
        "TECH_ADDRESS" => val: kv_string => { metadata.tech_address = Some(val); },
        "PREVIOUS_MESSAGE_ID" => val: kv_string => { metadata.previous_message_id = Some(val); },
        "NEXT_MESSAGE_ID" => val: kv_string => { metadata.next_message_id = Some(val); },
        "ADM_MSG_LINK" => val: kv_string => { metadata.adm_msg_link = Some(val); },
        "ODM_MSG_LINK" => val: kv_string => { metadata.odm_msg_link = Some(val); },
        "CDM_MSG_LINK" => val: kv_string => { metadata.cdm_msg_link = Some(val); },
        "PRM_MSG_LINK" => val: kv_string => { metadata.prm_msg_link = Some(val); },
        "RDM_MSG_LINK" => val: kv_string => { metadata.rdm_msg_link = Some(val); },
        "TDM_MSG_LINK" => val: kv_string => { metadata.tdm_msg_link = Some(val); },
        "OPERATOR" => val: kv_string => { metadata.operator = Some(val); },
        "OWNER" => val: kv_string => { metadata.owner = Some(val); },
        "COUNTRY" => val: kv_string => { metadata.country = Some(val); },
        "CONSTELLATION" => val: kv_string => { metadata.constellation = Some(val); },
        "OBJECT_TYPE" => val: kv_string => { metadata.object_type = Some(val); },
        "TIME_SYSTEM" => val: kv_string => { metadata.time_system = val; },
        "EPOCH_TZERO" => val: kv_epoch => { metadata.epoch_tzero = val; },
        "TAIMUTC_AT_TZERO" => val: kv_from_kvn => { metadata.taimutc_at_tzero = Some(val); },
        "NEXT_LEAP_EPOCH" => val: kv_epoch => { metadata.next_leap_epoch = Some(val); },
        "NEXT_LEAP_TAIMUTC" => val: kv_from_kvn => { metadata.next_leap_taimutc = Some(val); },
        "UT1MUTC_AT_TZERO" => val: kv_from_kvn => { metadata.ut1mutc_at_tzero = Some(val); },
        "EOP_SOURCE" => val: kv_string => { metadata.eop_source = Some(val); },
        "INTERP_METHOD_EOP" => val: kv_string => { metadata.interp_method_eop = Some(val); },
        "CELESTIAL_SOURCE" => val: kv_string => { metadata.celestial_source = Some(val); },
        "ACM_DATA_ELEMENTS" => val: kv_string => { metadata.acm_data_elements = Some(val); },
    }, |i: &mut &str| at_block_end("META", i));

    expect_block_end("META").parse_next(input)?;

    Ok(metadata)
}

//----------------------------------------------------------------------
// ACM Data Logical Blocks
//----------------------------------------------------------------------

fn parse_att_line(input: &mut &str) -> KvnResult<AttLine> {
    let line = terminated(raw_line, opt_line_ending).parse_next(input)?;
    let values = line.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|_| ErrMode::Cut(InternalParserError::from_input(input))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AttLine { values })
}

fn parse_att_block(input: &mut &str) -> KvnResult<AcmAttitudeState> {
    let mut block = AcmAttitudeState::default();
    expect_block_start("ATT").parse_next(input)?;

    parse_block!(input, block.comment, {
        "REF_FRAME_A" => val: kv_string => { block.ref_frame_a = val; },
        "REF_FRAME_B" => val: kv_string => { block.ref_frame_b = val; },
        "ATTITUDE_DIR" => val: kv_string => { block.attitude_dir = Some(val); },
        "NUMBER_STATES" => val: kv_u32 => { block.number_states = val; },
        "ATT_TYPE" => val: kv_string => { block.att_type = val; },
        "RATE_TYPE" => val: kv_string => { block.rate_type = Some(val); },
        "EULER_ROT_SEQ" => val: kv_enum => { block.euler_rot_seq = Some(val); },
    }, |i: &mut &str| matches!(i.trim_start().chars().next(), Some('0'..='9' | '-' | '+')) || at_block_end("ATT", i));

    loop {
        if at_block_end("ATT", input) { break; }
        if input.trim_start().starts_with("COMMENT") {
            block.comment.extend(collect_comments.parse_next(input)?);
            continue;
        }
        block.att_lines.push(parse_att_line.parse_next(input)?);
    }

    expect_block_end("ATT").parse_next(input)?;
    Ok(block)
}

fn parse_phys_block(input: &mut &str) -> KvnResult<AcmPhysicalDescription> {
    let mut block = AcmPhysicalDescription::default();
    expect_block_start("PHYS").parse_next(input)?;

    parse_block!(input, block.comment, {
        "WET_MASS" => val: kv_from_kvn => { block.wet_mass = Some(val); },
        "DRY_MASS" => val: kv_from_kvn => { block.dry_mass = Some(val); },
        "CP_X" => val: kv_float => { block.center_of_pressure.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[0] = val; },
        "CP_Y" => val: kv_float => { block.center_of_pressure.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[1] = val; },
        "CP_Z" => val: kv_float => { block.center_of_pressure.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[2] = val; },
        "BUS_AREA" => val: kv_from_kvn => { block.bus_area = Some(val); },
        "BUS_RAD_COEFF" => val: kv_from_kvn => { block.bus_rad_coeff = Some(val); },
        "BUS_DRAG_COEFF" => val: kv_from_kvn => { block.bus_drag_coeff = Some(val); },
        "SA_AREA" => val: kv_from_kvn => { block.sa_area = Some(val); },
        "SA_RAD_COEFF" => val: kv_from_kvn => { block.sa_rad_coeff = Some(val); },
        "SA_DRAG_COEFF" => val: kv_from_kvn => { block.sa_drag_coeff = Some(val); },
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
    let values = line.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|_| ErrMode::Cut(InternalParserError::from_input(input))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CovLine { values })
}

fn parse_cov_block(input: &mut &str) -> KvnResult<AcmCovarianceMatrix> {
    let mut block = AcmCovarianceMatrix::default();
    expect_block_start("COV").parse_next(input)?;

    parse_block!(input, block.comment, {
        "COV_BASIS" => val: kv_string => { block.cov_basis = val; },
        "COV_REF_FRAME" => val: kv_string => { block.cov_ref_frame = val; },
        "COV_TYPE" => val: kv_string => { block.cov_type = val; },
        "COV_CONFIDENCE" => val: kv_float => { block.cov_confidence = Some(val); },
    }, |i: &mut &str| matches!(i.trim_start().chars().next(), Some('0'..='9' | '-' | '+')) || at_block_end("COV", i));

    loop {
        if at_block_end("COV", input) { break; }
        if input.trim_start().starts_with("COMMENT") {
            block.comment.extend(collect_comments.parse_next(input)?);
            continue;
        }
        block.cov_lines.push(parse_cov_line.parse_next(input)?);
    }

    expect_block_end("COV").parse_next(input)?;
    Ok(block)
}

fn parse_man_block(input: &mut &str) -> KvnResult<AcmManeuverParameters> {
    let mut block = AcmManeuverParameters::default();
    expect_block_start("MAN").parse_next(input)?;

    parse_block!(input, block.comment, {
        "MAN_ID" => val: kv_string => { block.man_id = val; },
        "MAN_PREV_ID" => val: kv_string => { block.man_prev_id = Some(val); },
        "MAN_PURPOSE" => val: kv_string => { block.man_purpose = Some(val); },
        "MAN_BEGIN_TIME" => val: kv_epoch => { block.man_begin_time = Some(val); },
        "MAN_END_TIME" => val: kv_epoch => { block.man_end_time = Some(val); },
        "MAN_DURATION" => val: kv_from_kvn => { block.man_duration = Some(val); },
        "ACTUATOR_USED" => val: kv_string => { block.actuator_used = Some(val); },
        "TARGET_MOM_X" => val: kv_float => { block.target_momentum.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[0] = val; },
        "TARGET_MOM_Y" => val: kv_float => { block.target_momentum.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[1] = val; },
        "TARGET_MOM_Z" => val: kv_float => { block.target_momentum.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[2] = val; },
        "TARGET_MOM_FRAME" => val: kv_string => { block.target_mom_frame = Some(val); },
        "MAN_TOR_X" => val: kv_float => { block.man_torque.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[0] = val; },
        "MAN_TOR_Y" => val: kv_float => { block.man_torque.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[1] = val; },
        "MAN_TOR_Z" => val: kv_float => { block.man_torque.get_or_insert_with(|| crate::types::Vector3 { elements: vec![0.0, 0.0, 0.0], units: None }).elements[2] = val; },
        "MAN_TOR_FRAME" => val: kv_string => { block.man_tor_frame = Some(val); },
        "MAN_DELTA_MASS" => val: kv_from_kvn => { block.man_delta_mass = Some(val); },
    }, |i| at_block_end("MAN", i));

    expect_block_end("MAN").parse_next(input)?;
    Ok(block)
}

fn parse_sensor_block(input: &mut &str) -> KvnResult<AcmSensor> {
    let mut block = AcmSensor::default();
    expect_block_start("SENSOR").parse_next(input)?;

    parse_block!(input, block.comment, {
        "SENSOR_NUMBER" => val: kv_u32 => { block.sensor_number = val; },
        "SENSOR_USED" => val: kv_string => { block.sensor_used = Some(val); },
        "SENSOR_NOISE_STDDEV" => val: kv_float => { block.sensor_noise_stddev = Some(val); },
        "SENSOR_FREQUENCY" => val: kv_float => { block.sensor_frequency = Some(val); },
    }, |i| at_block_end("SENSOR", i));

    expect_block_end("SENSOR").parse_next(input)?;
    Ok(block)
}

fn parse_ad_block(input: &mut &str) -> KvnResult<AcmAttitudeDetermination> {
    let mut block = AcmAttitudeDetermination::default();
    expect_block_start("AD").parse_next(input)?;

    parse_block!(input, block.comment, {
        "AD_ID" => val: kv_string => { block.ad_id = val; },
        "AD_PREV_ID" => val: kv_string => { block.ad_prev_id = Some(val); },
        "AD_METHOD" => val: kv_string => { block.ad_method = Some(val); },
        "ATTITUDE_SOURCE" => val: kv_string => { block.attitude_source = Some(val); },
        "ATTITUDE_STATES" => val: kv_string => { block.attitude_states = Some(val); },
        "AD_EPOCH" => val: kv_epoch => { block.ad_epoch = Some(val); },
        "REF_FRAME_A" => val: kv_string => { block.ref_frame_a = Some(val); },
        "REF_FRAME_B" => val: kv_string => { block.ref_frame_b = Some(val); },
        "ATTITUDE_TYPE" => val: kv_string => { block.attitude_type = Some(val); },
        "RATE_STATES" => val: kv_string => { block.rate_states = Some(val); },
        "SIGMA_U" => val: kv_from_kvn => { block.sigma_u = Some(val); },
        "SIGMA_V" => val: kv_from_kvn => { block.sigma_v = Some(val); },
    }, |i| at_block_start("SENSOR", i) || at_block_end("AD", i));

    loop {
        if at_block_end("AD", input) { break; }
        if at_block_start("SENSOR", input) {
            block.sensors.push(parse_sensor_block.parse_next(input)?);
            continue;
        }
        break;
    }

    expect_block_end("AD").parse_next(input)?;
    Ok(block)
}

//----------------------------------------------------------------------
// ACM Data Parser
//----------------------------------------------------------------------

pub fn acm_data(input: &mut &str) -> KvnResult<AcmData> {
    let mut data = AcmData::default();

    loop {
        let _ = skip_empty_lines.parse_next(input);
        if input.is_empty() || at_block_start("META", input) { break; }

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
        } else {
            // Check for user defined or just comments?
            // User defined is not explicitly START/STOP wrapped in Book?
            // But usually it's at the end.
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
        body: AcmBody { segment: Box::new(segment) },
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

    #[test]
    fn test_parse_acm_minimal() {
        let input = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 2023-001A
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP

ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 1
ATT_TYPE = QUATERNION
0.5 0.5 0.5 0.5
ATT_STOP
"#;
        let acm = Acm::from_kvn(input).unwrap();
        assert_eq!(acm.version, "2.0");
        assert_eq!(acm.header.originator, "TEST");
        assert_eq!(acm.body.segment.metadata.object_name, "SAT1");
        assert_eq!(acm.body.segment.data.att.len(), 1);
        assert_eq!(acm.body.segment.data.att[0].att_lines.len(), 1);
        assert_eq!(acm.body.segment.data.att[0].att_lines[0].values[0], 0.5);
    }
}
