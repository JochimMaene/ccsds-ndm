// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for CDM (Conjunction Data Message).
//!
//! This module implements KVN parsing for CDM using winnow parser combinators.

use super::{
    AdditionalParameters, Cdm, CdmBody, CdmCovarianceMatrix, CdmData, CdmHeader, CdmMetadata,
    CdmSegment, CdmStateVector, RelativeMetadataData, RelativeStateVector,
};
use crate::common::OdParameters;
use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::traits::ToKvn;
use crate::types::*;
use std::borrow::Cow;
use winnow::combinator::peek;
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// CDM Version Parser
//----------------------------------------------------------------------

pub fn cdm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_CDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// CDM Header Parser
//----------------------------------------------------------------------

pub fn cdm_header(input: &mut &str) -> KvnResult<CdmHeader> {
    // Header comments are only permitted immediately after the version line.
    let comment = collect_comments.parse_next(input)?;
    let mut creation_date = None;

    let mut originator = None;
    let mut message_for = None;
    let mut message_id = None;

    loop {
        let checkpoint = input.checkpoint();
        let upcoming_comments = collect_comments.parse_next(input)?;
        if !upcoming_comments.is_empty() {
            input.reset(&checkpoint);
            break;
        }

        let key = match keyword.parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        if key == "TCA" || key == "META_START" {
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
            "MESSAGE_FOR" => {
                message_for = Some(kv_string.parse_next(input)?);
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

    Ok(CdmHeader {
        comment,
        creation_date: creation_date
            .ok_or_else(|| missing_field_err(input, "Header", "CREATION_DATE"))?,
        originator: originator.ok_or_else(|| missing_field_err(input, "Header", "ORIGINATOR"))?,
        message_for,
        message_id: message_id.ok_or_else(|| missing_field_err(input, "Header", "MESSAGE_ID"))?,
    })
}

//----------------------------------------------------------------------
// Relative Metadata Parser
//----------------------------------------------------------------------

pub fn relative_metadata_data(input: &mut &str) -> KvnResult<RelativeMetadataData> {
    let mut comment = Vec::new();
    let mut tca = None;
    let mut miss_distance = None;
    let mut relative_speed = None;
    let mut rel_pos_r = None;
    let mut rel_pos_t = None;
    let mut rel_pos_n = None;
    let mut rel_vel_r = None;
    let mut rel_vel_t = None;
    let mut rel_vel_n = None;
    let mut start_screen_period = None;
    let mut stop_screen_period = None;
    let mut screen_volume_frame = None;
    let mut screen_volume_shape = None;
    let mut screen_volume_x = None;
    let mut screen_volume_y = None;
    let mut screen_volume_z = None;
    let mut screen_entry_time = None;
    let mut screen_exit_time = None;
    let mut collision_probability = None;
    let mut collision_probability_method = None;

    parse_routed_block!(input, |_, comments| comment.extend(comments), {
        "TCA" => tca: kv_calendar_epoch,
        "MISS_DISTANCE" => miss_distance: kv_from_kvn,
        "RELATIVE_SPEED" => val: kv_from_kvn_opt => { relative_speed = val; },
        "RELATIVE_POSITION_R" => val: kv_from_kvn_opt => { rel_pos_r = val; },
        "RELATIVE_POSITION_T" => val: kv_from_kvn_opt => { rel_pos_t = val; },
        "RELATIVE_POSITION_N" => val: kv_from_kvn_opt => { rel_pos_n = val; },
        "RELATIVE_VELOCITY_R" => val: kv_from_kvn_opt => { rel_vel_r = val; },
        "RELATIVE_VELOCITY_T" => val: kv_from_kvn_opt => { rel_vel_t = val; },
        "RELATIVE_VELOCITY_N" => val: kv_from_kvn_opt => { rel_vel_n = val; },
        "START_SCREEN_PERIOD" => val: kv_calendar_epoch_opt => { start_screen_period = val; },
        "STOP_SCREEN_PERIOD" => val: kv_calendar_epoch_opt => { stop_screen_period = val; },
        "SCREEN_VOLUME_FRAME" => val: kv_enum_opt => { screen_volume_frame = val; },
        "SCREEN_VOLUME_SHAPE" => val: kv_enum_opt => { screen_volume_shape = val; },
        "SCREEN_VOLUME_X" => val: kv_from_kvn_opt => { screen_volume_x = val; },
        "SCREEN_VOLUME_Y" => val: kv_from_kvn_opt => { screen_volume_y = val; },
        "SCREEN_VOLUME_Z" => val: kv_from_kvn_opt => { screen_volume_z = val; },
        "SCREEN_ENTRY_TIME" => val: kv_calendar_epoch_opt => { screen_entry_time = val; },
        "SCREEN_EXIT_TIME" => val: kv_calendar_epoch_opt => { screen_exit_time = val; },
        "COLLISION_PROBABILITY" => val: kv_from_kvn_opt => { collision_probability = val; },
        "COLLISION_PROBABILITY_METHOD" => val: kv_string_opt => { collision_probability_method = val; },
    }, |i: &mut &str| at_block_start("META", i) || peek(key_token).parse_next(i).map(|k| k == "OBJECT").unwrap_or(false), "Unknown Relative Metadata key");

    let relative_state_vector = if rel_pos_r.is_some() || rel_pos_t.is_some() || rel_pos_n.is_some()
    {
        Some(RelativeStateVector {
            relative_position_r: rel_pos_r.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_POSITION_R")
            })?,
            relative_position_t: rel_pos_t.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_POSITION_T")
            })?,
            relative_position_n: rel_pos_n.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_POSITION_N")
            })?,
            relative_velocity_r: rel_vel_r.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_VELOCITY_R")
            })?,
            relative_velocity_t: rel_vel_t.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_VELOCITY_T")
            })?,
            relative_velocity_n: rel_vel_n.ok_or_else(|| {
                missing_field_err(input, "Relative Metadata", "RELATIVE_VELOCITY_N")
            })?,
        })
    } else {
        None
    };

    Ok(RelativeMetadataData {
        comment,
        tca: tca.ok_or_else(|| missing_field_err(input, "Relative Metadata", "TCA"))?,
        miss_distance: miss_distance
            .ok_or_else(|| missing_field_err(input, "Relative Metadata", "MISS_DISTANCE"))?,
        relative_speed,
        relative_state_vector,
        start_screen_period,
        stop_screen_period,
        screen_volume_frame,
        screen_volume_shape,

        screen_volume_x,
        screen_volume_y,
        screen_volume_z,
        screen_entry_time,
        screen_exit_time,
        collision_probability,
        collision_probability_method,
    })
}

//----------------------------------------------------------------------
// CDM Metadata Parser
//----------------------------------------------------------------------

fn is_cdm_data_key(key: &str) -> bool {
    matches!(
        key,
        "TIME_LASTOB_START"
            | "TIME_LASTOB_END"
            | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN"
            | "OBS_AVAILABLE"
            | "OBS_USED"
            | "TRACKS_AVAILABLE"
            | "TRACKS_USED"
            | "RESIDUALS_ACCEPTED"
            | "WEIGHTED_RMS"
            | "AREA_PC"
            | "AREA_DRG"
            | "AREA_SRP"
            | "MASS"
            | "CD_AREA_OVER_MASS"
            | "CR_AREA_OVER_MASS"
            | "THRUST_ACCELERATION"
            | "SEDR"
            | "X"
            | "Y"
            | "Z"
            | "X_DOT"
            | "Y_DOT"
            | "Z_DOT"
            | "CR_R"
            | "CT_R"
            | "CT_T"
            | "CN_R"
            | "CN_T"
            | "CN_N"
            | "CRDOT_R"
            | "CRDOT_T"
            | "CRDOT_N"
            | "CRDOT_RDOT"
            | "CTDOT_R"
            | "CTDOT_T"
            | "CTDOT_N"
            | "CTDOT_RDOT"
            | "CTDOT_TDOT"
            | "CNDOT_R"
            | "CNDOT_T"
            | "CNDOT_N"
            | "CNDOT_RDOT"
            | "CNDOT_TDOT"
            | "CNDOT_NDOT"
            | "CDRG_R"
            | "CDRG_T"
            | "CDRG_N"
            | "CDRG_RDOT"
            | "CDRG_TDOT"
            | "CDRG_NDOT"
            | "CDRG_DRG"
            | "CSRP_R"
            | "CSRP_T"
            | "CSRP_N"
            | "CSRP_RDOT"
            | "CSRP_TDOT"
            | "CSRP_NDOT"
            | "CSRP_DRG"
            | "CSRP_SRP"
            | "CTHR_R"
            | "CTHR_T"
            | "CTHR_N"
            | "CTHR_RDOT"
            | "CTHR_TDOT"
            | "CTHR_NDOT"
            | "CTHR_DRG"
            | "CTHR_SRP"
            | "CTHR_THR"
    )
}

fn is_od_parameter_key(key: &str) -> bool {
    matches!(
        key,
        "TIME_LASTOB_START"
            | "TIME_LASTOB_END"
            | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN"
            | "OBS_AVAILABLE"
            | "OBS_USED"
            | "TRACKS_AVAILABLE"
            | "TRACKS_USED"
            | "RESIDUALS_ACCEPTED"
            | "WEIGHTED_RMS"
    )
}

fn is_additional_parameter_key(key: &str) -> bool {
    matches!(
        key,
        "AREA_PC"
            | "AREA_DRG"
            | "AREA_SRP"
            | "MASS"
            | "CD_AREA_OVER_MASS"
            | "CR_AREA_OVER_MASS"
            | "THRUST_ACCELERATION"
            | "SEDR"
    )
}

fn is_state_vector_key(key: &str) -> bool {
    matches!(key, "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT")
}

pub fn cdm_metadata(input: &mut &str) -> KvnResult<CdmMetadata> {
    let has_meta_block = if at_block_start("META", input) {
        expect_block_start("META").parse_next(input)?;
        true
    } else {
        false
    };

    let mut comment = Vec::new();
    let mut object = None;
    let mut object_designator = None;
    let mut catalog_name = None;
    let mut object_name = None;
    let mut international_designator = None;
    let mut object_type = None;
    let mut operator_contact_position = None;
    let mut operator_organization = None;
    let mut operator_phone = None;
    let mut operator_email = None;
    let mut ephemeris_name = None;
    let mut covariance_method = None;
    let mut maneuverable = None;
    let mut orbit_center = None;
    let mut ref_frame = None;
    let mut gravity_model = None;
    let mut atmospheric_model = None;
    let mut n_body_perturbations = None;
    let mut solar_rad_pressure = None;
    let mut earth_tides = None;
    let mut intrack_thrust = None;

    parse_routed_block!(input, |_, comments| comment.extend(comments), {
        "OBJECT" => object: kv_enum,
        "OBJECT_DESIGNATOR" => object_designator: kv_string,
        "CATALOG_NAME" => val: kv_string_opt => { catalog_name = val; },
        "OBJECT_NAME" => object_name: kv_string,
        "INTERNATIONAL_DESIGNATOR" => international_designator: kv_string,
        "OBJECT_TYPE" => val: kv_enum_opt => { object_type = val; },
        "OPERATOR_CONTACT_POSITION" => val: kv_string_opt => { operator_contact_position = val; },
        "OPERATOR_ORGANIZATION" => val: kv_string_opt => { operator_organization = val; },
        "OPERATOR_PHONE" => val: kv_string_opt => { operator_phone = val; },
        "OPERATOR_EMAIL" => val: kv_string_opt => { operator_email = val; },
        "EPHEMERIS_NAME" => val: kv_string_opt => { ephemeris_name = val; },
        "COVARIANCE_METHOD" => covariance_method: kv_enum,
        "MANEUVERABLE" => maneuverable: kv_enum,
        "ORBIT_CENTER" => val: kv_string_opt => { orbit_center = val; },
        "REF_FRAME" => val: kv_enum_opt => { ref_frame = val; },
        "GRAVITY_MODEL" => val: kv_string_opt => { gravity_model = val; },
        "ATMOSPHERIC_MODEL" => val: kv_string_opt => { atmospheric_model = val; },
        "N_BODY_PERTURBATIONS" => val: kv_string_opt => { n_body_perturbations = val; },
        "SOLAR_RAD_PRESSURE" => val: kv_yes_no_opt => { solar_rad_pressure = val; },
        "EARTH_TIDES" => val: kv_yes_no_opt => { earth_tides = val; },
        "INTRACK_THRUST" => val: kv_yes_no_opt => { intrack_thrust = val; },
    }, |i: &mut &str| (has_meta_block && at_block_end("META", i)) || (!has_meta_block && is_cdm_data_key(peek(key_token).parse_next(i).unwrap_or(""))), "Unknown metadata key");

    if has_meta_block && at_block_end("META", input) {
        expect_block_end("META").parse_next(input)?;
    }

    Ok(CdmMetadata {
        comment,
        object: object.ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT"))?,
        object_designator: object_designator
            .ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT_DESIGNATOR"))?,
        catalog_name: catalog_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "CATALOG_NAME"))?,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT_NAME"))?,
        international_designator: international_designator
            .ok_or_else(|| missing_field_err(input, "Metadata", "INTERNATIONAL_DESIGNATOR"))?,
        object_type,
        operator_contact_position,
        operator_organization,
        operator_phone,
        operator_email,
        ephemeris_name: ephemeris_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "EPHEMERIS_NAME"))?,
        covariance_method: covariance_method
            .ok_or_else(|| missing_field_err(input, "Metadata", "COVARIANCE_METHOD"))?,
        maneuverable: maneuverable
            .ok_or_else(|| missing_field_err(input, "Metadata", "MANEUVERABLE"))?,
        orbit_center,
        ref_frame: ref_frame.ok_or_else(|| missing_field_err(input, "Metadata", "REF_FRAME"))?,
        gravity_model,
        atmospheric_model,
        n_body_perturbations,
        solar_rad_pressure,
        earth_tides,
        intrack_thrust,
    })
}

//----------------------------------------------------------------------
// CDM Data Parser
//----------------------------------------------------------------------

pub fn cdm_data(input: &mut &str) -> KvnResult<CdmData> {
    let mut comment = Vec::new();
    let mut od_params = OdParameters::default();
    let mut add_params = AdditionalParameters::default();
    let mut state_vector_comment = Vec::new();
    let mut covariance_comment = Vec::new();
    let mut saw_data_key = false;
    let mut x = None;
    let mut y = None;
    let mut z = None;
    let mut x_dot = None;
    let mut y_dot = None;
    let mut z_dot = None;

    // OD Parameters flags
    let mut has_od_params = false;
    let mut has_add_params = false;

    let mut cr_r = None;
    let mut ct_r = None;
    let mut ct_t = None;
    let mut cn_r = None;
    let mut cn_t = None;
    let mut cn_n = None;
    let mut crdot_r = None;
    let mut crdot_t = None;
    let mut crdot_n = None;
    let mut crdot_rdot = None;
    let mut ctdot_r = None;
    let mut ctdot_t = None;
    let mut ctdot_n = None;
    let mut ctdot_rdot = None;
    let mut ctdot_tdot = None;
    let mut cndot_r = None;
    let mut cndot_t = None;
    let mut cndot_n = None;
    let mut cndot_rdot = None;
    let mut cndot_tdot = None;
    let mut cndot_ndot = None;
    let mut cdrg_r = None;
    let mut cdrg_t = None;
    let mut cdrg_n = None;
    let mut cdrg_rdot = None;
    let mut cdrg_tdot = None;
    let mut cdrg_ndot = None;
    let mut cdrg_drg = None;
    let mut csrp_r = None;
    let mut csrp_t = None;
    let mut csrp_n = None;
    let mut csrp_rdot = None;
    let mut csrp_tdot = None;
    let mut csrp_ndot = None;
    let mut csrp_drg = None;
    let mut csrp_srp = None;
    let mut cthr_r = None;
    let mut cthr_t = None;
    let mut cthr_n = None;
    let mut cthr_rdot = None;
    let mut cthr_tdot = None;
    let mut cthr_ndot = None;
    let mut cthr_drg = None;
    let mut cthr_srp = None;
    let mut cthr_thr = None;
    let mut has_cov = false;

    parse_routed_block!(input, |key, comments| {
        // The outer data comment and the first nested block's comment are adjacent in KVN and
        // therefore intrinsically ambiguous. Keep that leading run on the outer data block so
        // generation preserves its position. Later runs have an unambiguous following block.
        if !saw_data_key {
            comment.extend(comments);
            saw_data_key = true;
        } else if is_od_parameter_key(key) {
            od_params.comment.extend(comments);
        } else if is_additional_parameter_key(key) {
            add_params.comment.extend(comments);
        } else if is_state_vector_key(key) {
            state_vector_comment.extend(comments);
        } else {
            covariance_comment.extend(comments);
        }
    }, {
        "TIME_LASTOB_START" => val: kv_calendar_epoch_opt => { od_params.time_lastob_start = val; has_od_params = true; },
        "TIME_LASTOB_END" => val: kv_calendar_epoch_opt => { od_params.time_lastob_end = val; has_od_params = true; },
        "RECOMMENDED_OD_SPAN" => val: kv_from_kvn_opt => { od_params.recommended_od_span = val; has_od_params = true; },
        "ACTUAL_OD_SPAN" => val: kv_from_kvn_opt => { od_params.actual_od_span = val; has_od_params = true; },
        "OBS_AVAILABLE" => val: kv_u32_opt => { od_params.obs_available = val.map(|v| v.into()); has_od_params = true; },
        "OBS_USED" => val: kv_u32_opt => { od_params.obs_used = val.map(|v| v.into()); has_od_params = true; },
        "TRACKS_AVAILABLE" => val: kv_u32_opt => { od_params.tracks_available = val.map(|v| v.into()); has_od_params = true; },
        "TRACKS_USED" => val: kv_u32_opt => { od_params.tracks_used = val.map(|v| v.into()); has_od_params = true; },
        "RESIDUALS_ACCEPTED" => val: kv_from_kvn_opt => { od_params.residuals_accepted = val; has_od_params = true; },
        "WEIGHTED_RMS" => val: kv_from_kvn_opt => { od_params.weighted_rms = val; has_od_params = true; },

        "AREA_PC" => val: kv_from_kvn_opt => { add_params.area_pc = val; has_add_params = true; },
        "AREA_DRG" => val: kv_from_kvn_opt => { add_params.area_drg = val; has_add_params = true; },
        "AREA_SRP" => val: kv_from_kvn_opt => { add_params.area_srp = val; has_add_params = true; },
        "MASS" => val: kv_from_kvn_opt => { add_params.mass = val; has_add_params = true; },
        "CD_AREA_OVER_MASS" => val: kv_from_kvn_opt => { add_params.cd_area_over_mass = val; has_add_params = true; },
        "CR_AREA_OVER_MASS" => val: kv_from_kvn_opt => { add_params.cr_area_over_mass = val; has_add_params = true; },
        "THRUST_ACCELERATION" => val: kv_from_kvn_opt => { add_params.thrust_acceleration = val; has_add_params = true; },
        "SEDR" => val: kv_from_kvn_opt => { add_params.sedr = val; has_add_params = true; },

        "X" => val: kv_from_kvn => { x = Some(val); },
        "Y" => val: kv_from_kvn => { y = Some(val); },
        "Z" => val: kv_from_kvn => { z = Some(val); },
        "X_DOT" => val: kv_from_kvn => { x_dot = Some(val); },
        "Y_DOT" => val: kv_from_kvn => { y_dot = Some(val); },
        "Z_DOT" => val: kv_from_kvn => { z_dot = Some(val); },

        "CR_R" => val: kv_from_kvn_opt => { cr_r = val; has_cov = true; },
        "CT_R" => val: kv_from_kvn_opt => { ct_r = val; has_cov = true; },
        "CT_T" => val: kv_from_kvn_opt => { ct_t = val; has_cov = true; },
        "CN_R" => val: kv_from_kvn_opt => { cn_r = val; has_cov = true; },
        "CN_T" => val: kv_from_kvn_opt => { cn_t = val; has_cov = true; },
        "CN_N" => val: kv_from_kvn_opt => { cn_n = val; has_cov = true; },
        "CRDOT_R" => val: kv_from_kvn_opt => { crdot_r = val; has_cov = true; },
        "CRDOT_T" => val: kv_from_kvn_opt => { crdot_t = val; has_cov = true; },
        "CRDOT_N" => val: kv_from_kvn_opt => { crdot_n = val; has_cov = true; },
        "CRDOT_RDOT" => val: kv_from_kvn_opt => { crdot_rdot = val; has_cov = true; },
        "CTDOT_R" => val: kv_from_kvn_opt => { ctdot_r = val; has_cov = true; },
        "CTDOT_T" => val: kv_from_kvn_opt => { ctdot_t = val; has_cov = true; },
        "CTDOT_N" => val: kv_from_kvn_opt => { ctdot_n = val; has_cov = true; },
        "CTDOT_RDOT" => val: kv_from_kvn_opt => { ctdot_rdot = val; has_cov = true; },
        "CTDOT_TDOT" => val: kv_from_kvn_opt => { ctdot_tdot = val; has_cov = true; },
        "CNDOT_R" => val: kv_from_kvn_opt => { cndot_r = val; has_cov = true; },
        "CNDOT_T" => val: kv_from_kvn_opt => { cndot_t = val; has_cov = true; },
        "CNDOT_N" => val: kv_from_kvn_opt => { cndot_n = val; has_cov = true; },
        "CNDOT_RDOT" => val: kv_from_kvn_opt => { cndot_rdot = val; has_cov = true; },
        "CNDOT_TDOT" => val: kv_from_kvn_opt => { cndot_tdot = val; has_cov = true; },
        "CNDOT_NDOT" => val: kv_from_kvn_opt => { cndot_ndot = val; has_cov = true; },
        "CDRG_R" => val: kv_from_kvn_opt => { cdrg_r = val; has_cov = true; },
        "CDRG_T" => val: kv_from_kvn_opt => { cdrg_t = val; has_cov = true; },
        "CDRG_N" => val: kv_from_kvn_opt => { cdrg_n = val; has_cov = true; },
        "CDRG_RDOT" => val: kv_from_kvn_opt => { cdrg_rdot = val; has_cov = true; },
        "CDRG_TDOT" => val: kv_from_kvn_opt => { cdrg_tdot = val; has_cov = true; },
        "CDRG_NDOT" => val: kv_from_kvn_opt => { cdrg_ndot = val; has_cov = true; },
        "CDRG_DRG" => val: kv_from_kvn_opt => { cdrg_drg = val; has_cov = true; },
        "CSRP_R" => val: kv_from_kvn_opt => { csrp_r = val; has_cov = true; },
        "CSRP_T" => val: kv_from_kvn_opt => { csrp_t = val; has_cov = true; },
        "CSRP_N" => val: kv_from_kvn_opt => { csrp_n = val; has_cov = true; },
        "CSRP_RDOT" => val: kv_from_kvn_opt => { csrp_rdot = val; has_cov = true; },
        "CSRP_TDOT" => val: kv_from_kvn_opt => { csrp_tdot = val; has_cov = true; },
        "CSRP_NDOT" => val: kv_from_kvn_opt => { csrp_ndot = val; has_cov = true; },
        "CSRP_DRG" => val: kv_from_kvn_opt => { csrp_drg = val; has_cov = true; },
        "CSRP_SRP" => val: kv_from_kvn_opt => { csrp_srp = val; has_cov = true; },
        "CTHR_R" => val: kv_from_kvn_opt => { cthr_r = val; has_cov = true; },
        "CTHR_T" => val: kv_from_kvn_opt => { cthr_t = val; has_cov = true; },
        "CTHR_N" => val: kv_from_kvn_opt => { cthr_n = val; has_cov = true; },
        "CTHR_RDOT" => val: kv_from_kvn_opt => { cthr_rdot = val; has_cov = true; },
        "CTHR_TDOT" => val: kv_from_kvn_opt => { cthr_tdot = val; has_cov = true; },
        "CTHR_NDOT" => val: kv_from_kvn_opt => { cthr_ndot = val; has_cov = true; },
        "CTHR_DRG" => val: kv_from_kvn_opt => { cthr_drg = val; has_cov = true; },
        "CTHR_SRP" => val: kv_from_kvn_opt => { cthr_srp = val; has_cov = true; },
        "CTHR_THR" => val: kv_from_kvn_opt => { cthr_thr = val; has_cov = true; },
    }, |i: &mut &str| at_block_start("META", i) || peek(key_token).parse_next(i).map(|k| k == "OBJECT").unwrap_or(false), "Unknown Data key");

    let covariance_matrix = if has_cov {
        Some(CdmCovarianceMatrix {
            comment: covariance_comment,
            cr_r: cr_r.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CR_R"))?,
            ct_r: ct_r.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CT_R"))?,
            ct_t: ct_t.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CT_T"))?,
            cn_r: cn_r.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CN_R"))?,
            cn_t: cn_t.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CN_T"))?,
            cn_n: cn_n.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CN_N"))?,
            crdot_r: crdot_r
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CRDOT_R"))?,
            crdot_t: crdot_t
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CRDOT_T"))?,
            crdot_n: crdot_n
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CRDOT_N"))?,
            crdot_rdot: crdot_rdot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CRDOT_RDOT"))?,
            ctdot_r: ctdot_r
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CTDOT_R"))?,
            ctdot_t: ctdot_t
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CTDOT_T"))?,
            ctdot_n: ctdot_n
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CTDOT_N"))?,
            ctdot_rdot: ctdot_rdot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CTDOT_RDOT"))?,
            ctdot_tdot: ctdot_tdot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CTDOT_TDOT"))?,
            cndot_r: cndot_r
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_R"))?,
            cndot_t: cndot_t
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_T"))?,
            cndot_n: cndot_n
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_N"))?,
            cndot_rdot: cndot_rdot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_RDOT"))?,
            cndot_tdot: cndot_tdot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_TDOT"))?,
            cndot_ndot: cndot_ndot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CNDOT_NDOT"))?,
            cdrg_r,
            cdrg_t,
            cdrg_n,
            cdrg_rdot,
            cdrg_tdot,
            cdrg_ndot,
            cdrg_drg,
            csrp_r,
            csrp_t,
            csrp_n,
            csrp_rdot,
            csrp_tdot,
            csrp_ndot,
            csrp_drg,
            csrp_srp,
            cthr_r,
            cthr_t,
            cthr_n,
            cthr_rdot,
            cthr_tdot,
            cthr_ndot,
            cthr_drg,
            cthr_srp,
            cthr_thr,
        })
    } else {
        None
    };

    Ok(CdmData {
        comment,
        od_parameters: if has_od_params { Some(od_params) } else { None },
        additional_parameters: if has_add_params {
            Some(add_params)
        } else {
            None
        },
        state_vector: CdmStateVector {
            comment: state_vector_comment,
            x: x.ok_or_else(|| missing_field_err(input, "Data", "X"))?,
            y: y.ok_or_else(|| missing_field_err(input, "Data", "Y"))?,
            z: z.ok_or_else(|| missing_field_err(input, "Data", "Z"))?,
            x_dot: x_dot.ok_or_else(|| missing_field_err(input, "Data", "X_DOT"))?,
            y_dot: y_dot.ok_or_else(|| missing_field_err(input, "Data", "Y_DOT"))?,
            z_dot: z_dot.ok_or_else(|| missing_field_err(input, "Data", "Z_DOT"))?,
        },
        covariance_matrix,
    })
}

//----------------------------------------------------------------------
// CDM Segment Parser
//----------------------------------------------------------------------

pub fn cdm_segment(input: &mut &str) -> KvnResult<CdmSegment> {
    // 1. Metadata
    // Metadata can start with optional META_START (for CDM 2.0) or just keys (for CDM 1.0)
    // However, if it's CDM 2.0, we might see META_START.

    // Collect comments before segment
    let pre_comments = collect_comments.parse_next(input)?;

    // winnow approach:
    // Check if we have META_START
    if at_block_start("META", input) {
        expect_block_start("META").parse_next(input)?;
    }

    let mut metadata = cdm_metadata.parse_next(input)?;
    metadata.comment.splice(0..0, pre_comments);

    // 2. Data
    let data = cdm_data.parse_next(input)?;

    Ok(CdmSegment { metadata, data })
}

//----------------------------------------------------------------------
// CDM Body Parser
//----------------------------------------------------------------------

pub fn cdm_body(input: &mut &str) -> KvnResult<CdmBody> {
    let relative_metadata_data = relative_metadata_data.parse_next(input)?;

    // Expecting 2 segments
    let segments = vec![
        cdm_segment.parse_next(input)?,
        cdm_segment.parse_next(input)?,
    ];

    Ok(CdmBody {
        relative_metadata_data,
        segments,
    })
}

//----------------------------------------------------------------------
// Complete CDM Parser
//----------------------------------------------------------------------

pub fn parse_cdm(input: &mut &str) -> KvnResult<Cdm> {
    let version = cdm_version.parse_next(input)?;
    let header = cdm_header.parse_next(input)?;
    let body = cdm_body.parse_next(input)?;

    Ok(Cdm {
        header,
        body,
        id: Some("CCSDS_CDM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Cdm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_cdm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

fn cdm_kvn_key(key: &str) -> Option<(u8, u16)> {
    const HEADER: &[&str] = &["CREATION_DATE", "ORIGINATOR", "MESSAGE_FOR", "MESSAGE_ID"];
    const RELATIVE: &[&str] = &[
        "TCA",
        "MISS_DISTANCE",
        "RELATIVE_SPEED",
        "RELATIVE_POSITION_R",
        "RELATIVE_POSITION_T",
        "RELATIVE_POSITION_N",
        "RELATIVE_VELOCITY_R",
        "RELATIVE_VELOCITY_T",
        "RELATIVE_VELOCITY_N",
        "START_SCREEN_PERIOD",
        "STOP_SCREEN_PERIOD",
        "SCREEN_VOLUME_FRAME",
        "SCREEN_VOLUME_SHAPE",
        "SCREEN_VOLUME_X",
        "SCREEN_VOLUME_Y",
        "SCREEN_VOLUME_Z",
        "SCREEN_ENTRY_TIME",
        "SCREEN_EXIT_TIME",
        "COLLISION_PROBABILITY",
        "COLLISION_PROBABILITY_METHOD",
    ];
    const METADATA: &[&str] = &[
        "OBJECT",
        "OBJECT_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "OBJECT_TYPE",
        "OPERATOR_CONTACT_POSITION",
        "OPERATOR_ORGANIZATION",
        "OPERATOR_PHONE",
        "OPERATOR_EMAIL",
        "EPHEMERIS_NAME",
        "COVARIANCE_METHOD",
        "MANEUVERABLE",
        "ORBIT_CENTER",
        "REF_FRAME",
        "GRAVITY_MODEL",
        "ATMOSPHERIC_MODEL",
        "N_BODY_PERTURBATIONS",
        "SOLAR_RAD_PRESSURE",
        "EARTH_TIDES",
        "INTRACK_THRUST",
    ];
    const OD: &[&str] = &[
        "TIME_LASTOB_START",
        "TIME_LASTOB_END",
        "RECOMMENDED_OD_SPAN",
        "ACTUAL_OD_SPAN",
        "OBS_AVAILABLE",
        "OBS_USED",
        "TRACKS_AVAILABLE",
        "TRACKS_USED",
        "RESIDUALS_ACCEPTED",
        "WEIGHTED_RMS",
    ];
    const ADDITIONAL: &[&str] = &[
        "AREA_PC",
        "AREA_DRG",
        "AREA_SRP",
        "MASS",
        "CD_AREA_OVER_MASS",
        "CR_AREA_OVER_MASS",
        "THRUST_ACCELERATION",
        "SEDR",
    ];
    const STATE: &[&str] = &["X", "Y", "Z", "X_DOT", "Y_DOT", "Z_DOT"];
    const COVARIANCE: &[&str] = &[
        "CR_R",
        "CT_R",
        "CT_T",
        "CN_R",
        "CN_T",
        "CN_N",
        "CRDOT_R",
        "CRDOT_T",
        "CRDOT_N",
        "CRDOT_RDOT",
        "CTDOT_R",
        "CTDOT_T",
        "CTDOT_N",
        "CTDOT_RDOT",
        "CTDOT_TDOT",
        "CNDOT_R",
        "CNDOT_T",
        "CNDOT_N",
        "CNDOT_RDOT",
        "CNDOT_TDOT",
        "CNDOT_NDOT",
        "CDRG_R",
        "CDRG_T",
        "CDRG_N",
        "CDRG_RDOT",
        "CDRG_TDOT",
        "CDRG_NDOT",
        "CDRG_DRG",
        "CSRP_R",
        "CSRP_T",
        "CSRP_N",
        "CSRP_RDOT",
        "CSRP_TDOT",
        "CSRP_NDOT",
        "CSRP_DRG",
        "CSRP_SRP",
        "CTHR_R",
        "CTHR_T",
        "CTHR_N",
        "CTHR_RDOT",
        "CTHR_TDOT",
        "CTHR_NDOT",
        "CTHR_DRG",
        "CTHR_SRP",
        "CTHR_THR",
    ];

    for (block, keys) in [
        (0, HEADER),
        (1, RELATIVE),
        (2, METADATA),
        (3, OD),
        (4, ADDITIONAL),
        (5, STATE),
        (6, COVARIANCE),
    ] {
        if let Some(rank) = keys.iter().position(|candidate| *candidate == key) {
            return Some((block, rank as u16));
        }
    }
    None
}

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating CDM KVN structure"],
            offset,
        }))))
    };
    let mut saw_version = false;
    let mut current_block = None;
    let mut previous_key = None;
    let mut pending_comments = false;
    let mut segment_count = 0u8;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
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
            pending_comments = true;
            offset += raw_line.len() + 1;
            continue;
        }
        if !line.contains('=') {
            return fail("expected exactly one CDM assignment");
        }
        let key = line.split_once('=').unwrap().0.trim();
        if key == "CCSDS_CDM_VERS" {
            if saw_version || current_block.is_some() || pending_comments {
                return fail("CCSDS_CDM_VERS must be the first record");
            }
            saw_version = true;
            offset += raw_line.len() + 1;
            continue;
        }
        if !saw_version {
            return fail("expected CCSDS_CDM_VERS as the first record");
        }
        let (block, rank) = cdm_kvn_key(key)
            .ok_or_else(|| invalid(number, offset, "unknown CDM keyword".into()))?;

        match current_block {
            None => {
                if block != 0 {
                    return fail("expected CDM header");
                }
            }
            Some(current) if block == current => {
                if pending_comments && previous_key.is_some() {
                    return fail("COMMENT is not at the beginning of a CDM logical block");
                }
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order CDM keyword");
                }
            }
            Some(5 | 6) if block == 2 => {
                segment_count += 1;
                if segment_count > 2 {
                    return fail("CDM contains more than two segments");
                }
            }
            Some(current) => {
                let allowed = matches!(
                    (current, block),
                    (0, 1) | (1, 2) | (2, 3..=5) | (3, 4..=5) | (4, 5) | (5, 6)
                );
                if !allowed {
                    return fail("out-of-order CDM logical block");
                }
            }
        }
        if block == 2 && key == "OBJECT" && current_block != Some(2) && segment_count == 0 {
            segment_count = 1;
        }
        current_block = Some(block);
        previous_key = Some(rank);
        pending_comments = false;
        offset += raw_line.len() + 1;
    }

    if pending_comments {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "trailing CDM COMMENT has no logical block".into(),
        ));
    }
    if !saw_version {
        return Err(invalid(1, 0, "missing CCSDS_CDM_VERS".into()));
    }
    Ok(())
}

impl ToKvn for Cdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_CDM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

impl ToKvn for CdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);

        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(v) = &self.message_for {
            writer.write_pair("MESSAGE_FOR", v);
        }
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

impl ToKvn for CdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.relative_metadata_data.write_kvn(writer);
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

impl ToKvn for RelativeMetadataData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("TCA", self.tca);
        writer.write_measure("MISS_DISTANCE", &self.miss_distance);
        if let Some(v) = &self.relative_speed {
            writer.write_measure("RELATIVE_SPEED", &v.to_unit_value());
        }
        if let Some(v) = &self.relative_state_vector {
            v.write_kvn(writer);
        }
        if let Some(v) = &self.start_screen_period {
            writer.write_pair("START_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.stop_screen_period {
            writer.write_pair("STOP_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.screen_volume_frame {
            writer.write_pair("SCREEN_VOLUME_FRAME", v.to_string());
        }
        if let Some(v) = &self.screen_volume_shape {
            writer.write_pair("SCREEN_VOLUME_SHAPE", v.to_string());
        }

        if let Some(v) = &self.screen_volume_x {
            writer.write_measure("SCREEN_VOLUME_X", v);
        }
        if let Some(v) = &self.screen_volume_y {
            writer.write_measure("SCREEN_VOLUME_Y", v);
        }
        if let Some(v) = &self.screen_volume_z {
            writer.write_measure("SCREEN_VOLUME_Z", v);
        }
        if let Some(v) = &self.screen_entry_time {
            writer.write_pair("SCREEN_ENTRY_TIME", v);
        }
        if let Some(v) = &self.screen_exit_time {
            writer.write_pair("SCREEN_EXIT_TIME", v);
        }
        if let Some(v) = &self.collision_probability {
            writer.write_pair("COLLISION_PROBABILITY", v.value);
        }
        if let Some(v) = &self.collision_probability_method {
            writer.write_pair("COLLISION_PROBABILITY_METHOD", v);
        }
    }
}

impl ToKvn for RelativeStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_measure("RELATIVE_POSITION_R", &self.relative_position_r);
        writer.write_measure("RELATIVE_POSITION_T", &self.relative_position_t);
        writer.write_measure("RELATIVE_POSITION_N", &self.relative_position_n);
        writer.write_measure(
            "RELATIVE_VELOCITY_R",
            &self.relative_velocity_r.to_unit_value(),
        );
        writer.write_measure(
            "RELATIVE_VELOCITY_T",
            &self.relative_velocity_t.to_unit_value(),
        );
        writer.write_measure(
            "RELATIVE_VELOCITY_N",
            &self.relative_velocity_n.to_unit_value(),
        );
    }
}

impl ToKvn for CdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for CdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair(
            "OBJECT",
            match self.object {
                CdmObjectType::Object1 => "OBJECT1",
                CdmObjectType::Object2 => "OBJECT2",
            },
        );
        writer.write_pair("OBJECT_DESIGNATOR", &self.object_designator);
        writer.write_pair("CATALOG_NAME", &self.catalog_name);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        if let Some(v) = &self.operator_contact_position {
            writer.write_pair("OPERATOR_CONTACT_POSITION", v);
        }
        if let Some(v) = &self.operator_organization {
            writer.write_pair("OPERATOR_ORGANIZATION", v);
        }
        if let Some(v) = &self.operator_phone {
            writer.write_pair("OPERATOR_PHONE", v);
        }
        if let Some(v) = &self.operator_email {
            writer.write_pair("OPERATOR_EMAIL", v);
        }
        writer.write_pair("EPHEMERIS_NAME", &self.ephemeris_name);
        writer.write_pair("COVARIANCE_METHOD", self.covariance_method.to_string());
        writer.write_pair("MANEUVERABLE", self.maneuverable.to_string());
        if let Some(v) = &self.orbit_center {
            writer.write_pair("ORBIT_CENTER", v);
        }
        writer.write_pair("REF_FRAME", self.ref_frame.to_string());
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", v.to_string());
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", v.to_string());
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", v.to_string());
        }
    }
}

impl ToKvn for CdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // OD Parameters
        if let Some(od) = &self.od_parameters {
            writer.write_comments(&od.comment);
            if let Some(v) = &od.time_lastob_start {
                writer.write_pair("TIME_LASTOB_START", v);
            }
            if let Some(v) = &od.time_lastob_end {
                writer.write_pair("TIME_LASTOB_END", v);
            }
            if let Some(v) = &od.recommended_od_span {
                writer.write_measure("RECOMMENDED_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.actual_od_span {
                writer.write_measure("ACTUAL_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.obs_available {
                writer.write_pair("OBS_AVAILABLE", v);
            }
            if let Some(v) = &od.obs_used {
                writer.write_pair("OBS_USED", v);
            }
            if let Some(v) = &od.tracks_available {
                writer.write_pair("TRACKS_AVAILABLE", v);
            }
            if let Some(v) = &od.tracks_used {
                writer.write_pair("TRACKS_USED", v);
            }
            if let Some(v) = &od.residuals_accepted {
                writer.write_measure("RESIDUALS_ACCEPTED", &v.to_unit_value());
            }
            if let Some(v) = &od.weighted_rms {
                writer.write_pair("WEIGHTED_RMS", v);
            }
        }
        // Additional Parameters
        if let Some(ap) = &self.additional_parameters {
            writer.write_comments(&ap.comment);
            if let Some(v) = &ap.area_pc {
                writer.write_measure("AREA_PC", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_drg {
                writer.write_measure("AREA_DRG", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_srp {
                writer.write_measure("AREA_SRP", &v.to_unit_value());
            }
            if let Some(v) = &ap.mass {
                writer.write_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.cd_area_over_mass {
                writer.write_measure("CD_AREA_OVER_MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.cr_area_over_mass {
                writer.write_measure("CR_AREA_OVER_MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", &v.to_unit_value());
            }
            if let Some(v) = &ap.sedr {
                writer.write_measure("SEDR", &v.to_unit_value());
            }
        }
        // State Vector
        self.state_vector.write_kvn(writer);
        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
    }
}

impl ToKvn for CdmStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_measure("X", &self.x.to_unit_value());
        writer.write_measure("Y", &self.y.to_unit_value());
        writer.write_measure("Z", &self.z.to_unit_value());
        writer.write_measure("X_DOT", &self.x_dot.to_unit_value());
        writer.write_measure("Y_DOT", &self.y_dot.to_unit_value());
        writer.write_measure("Z_DOT", &self.z_dot.to_unit_value());
    }
}

impl ToKvn for CdmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Required
        writer.write_measure("CR_R", &self.cr_r.to_unit_value());
        writer.write_measure("CT_R", &self.ct_r.to_unit_value());
        writer.write_measure("CT_T", &self.ct_t.to_unit_value());
        writer.write_measure("CN_R", &self.cn_r.to_unit_value());
        writer.write_measure("CN_T", &self.cn_t.to_unit_value());
        writer.write_measure("CN_N", &self.cn_n.to_unit_value());
        writer.write_measure("CRDOT_R", &self.crdot_r.to_unit_value());
        writer.write_measure("CRDOT_T", &self.crdot_t.to_unit_value());
        writer.write_measure("CRDOT_N", &self.crdot_n.to_unit_value());
        writer.write_measure("CRDOT_RDOT", &self.crdot_rdot.to_unit_value());
        writer.write_measure("CTDOT_R", &self.ctdot_r.to_unit_value());
        writer.write_measure("CTDOT_T", &self.ctdot_t.to_unit_value());
        writer.write_measure("CTDOT_N", &self.ctdot_n.to_unit_value());
        writer.write_measure("CTDOT_RDOT", &self.ctdot_rdot.to_unit_value());
        writer.write_measure("CTDOT_TDOT", &self.ctdot_tdot.to_unit_value());
        writer.write_measure("CNDOT_R", &self.cndot_r.to_unit_value());
        writer.write_measure("CNDOT_T", &self.cndot_t.to_unit_value());
        writer.write_measure("CNDOT_N", &self.cndot_n.to_unit_value());
        writer.write_measure("CNDOT_RDOT", &self.cndot_rdot.to_unit_value());
        writer.write_measure("CNDOT_TDOT", &self.cndot_tdot.to_unit_value());
        writer.write_measure("CNDOT_NDOT", &self.cndot_ndot.to_unit_value());

        // Optionals
        if let Some(v) = &self.cdrg_r {
            writer.write_measure("CDRG_R", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_t {
            writer.write_measure("CDRG_T", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_n {
            writer.write_measure("CDRG_N", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_rdot {
            writer.write_measure("CDRG_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_tdot {
            writer.write_measure("CDRG_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_ndot {
            writer.write_measure("CDRG_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cdrg_drg {
            writer.write_measure("CDRG_DRG", &v.to_unit_value());
        }

        if let Some(v) = &self.csrp_r {
            writer.write_measure("CSRP_R", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_t {
            writer.write_measure("CSRP_T", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_n {
            writer.write_measure("CSRP_N", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_rdot {
            writer.write_measure("CSRP_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_tdot {
            writer.write_measure("CSRP_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_ndot {
            writer.write_measure("CSRP_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_drg {
            writer.write_measure("CSRP_DRG", &v.to_unit_value());
        }
        if let Some(v) = &self.csrp_srp {
            writer.write_measure("CSRP_SRP", &v.to_unit_value());
        }

        if let Some(v) = &self.cthr_r {
            writer.write_measure("CTHR_R", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_t {
            writer.write_measure("CTHR_T", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_n {
            writer.write_measure("CTHR_N", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_rdot {
            writer.write_measure("CTHR_RDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_tdot {
            writer.write_measure("CTHR_TDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_ndot {
            writer.write_measure("CTHR_NDOT", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_drg {
            writer.write_measure("CTHR_DRG", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_srp {
            writer.write_measure("CTHR_SRP", &v.to_unit_value());
        }
        if let Some(v) = &self.cthr_thr {
            writer.write_measure("CTHR_THR", &v.to_unit_value());
        }
    }
}

impl Cdm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        for segment in &self.body.segments {
            let first_nested_comments = if let Some(od) = &segment.data.od_parameters {
                &od.comment
            } else if let Some(additional) = &segment.data.additional_parameters {
                &additional.comment
            } else {
                &segment.data.state_vector.comment
            };
            if !first_nested_comments.is_empty() {
                return Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "CDM KVN cannot distinguish the outer data COMMENT run from the first nested logical-block COMMENT run",
                    ),
                    line: None,
                }
                .into());
            }
        }

        // CDM has a fixed, bounded scalar shape. A private model fixed-point preflight therefore
        // catches every lossy KVN scalar spelling and lexical record without introducing an
        // unbounded history-sized allocation or emitting caller-visible bytes.
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        let output = writer.finish();
        validate_kvn_syntax(&output)?;
        let reparsed = Self::from_kvn_str(&output)?;
        crate::traits::Validate::validate(&reparsed)?;
        if reparsed != *self {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "CDM model cannot be represented in KVN without changing typed content",
                ),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}
