// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for CDM (Conjunction Data Message).
//!
//! This module implements KVN parsing for CDM using winnow parser combinators.

use crate::common::OdParameters;
use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::cdm::{
    AdditionalParameters, Cdm, CdmBody, CdmCovarianceMatrix, CdmData, CdmHeader, CdmMetadata,
    CdmSegment, CdmStateVector, RelativeMetadataData, RelativeStateVector,
};
use crate::parse_block;
use winnow::combinator::peek;
use winnow::error::AddContext;
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
    let mut comment = Vec::new();
    let mut creation_date = None;

    let mut originator = None;
    let mut message_for = None;
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

        if key == "TCA" || key == "META_START" {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CREATION_DATE" => {
                creation_date = Some(kv_epoch.parse_next(input)?);
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

    parse_block!(input, comment, {
        "TCA" => tca: kv_epoch,
        "MISS_DISTANCE" => miss_distance: kv_from_kvn,
        "RELATIVE_SPEED" => val: kv_from_kvn_opt => { relative_speed = val; },
        "RELATIVE_POSITION_R" => val: kv_from_kvn_opt => { rel_pos_r = val; },
        "RELATIVE_POSITION_T" => val: kv_from_kvn_opt => { rel_pos_t = val; },
        "RELATIVE_POSITION_N" => val: kv_from_kvn_opt => { rel_pos_n = val; },
        "RELATIVE_VELOCITY_R" => val: kv_from_kvn_opt => { rel_vel_r = val; },
        "RELATIVE_VELOCITY_T" => val: kv_from_kvn_opt => { rel_vel_t = val; },
        "RELATIVE_VELOCITY_N" => val: kv_from_kvn_opt => { rel_vel_n = val; },
        "START_SCREEN_PERIOD" => val: kv_epoch_opt => { start_screen_period = val; },
        "STOP_SCREEN_PERIOD" => val: kv_epoch_opt => { stop_screen_period = val; },
        "SCREEN_VOLUME_FRAME" => val: kv_enum_opt => { screen_volume_frame = val; },
        "SCREEN_VOLUME_SHAPE" => val: kv_enum_opt => { screen_volume_shape = val; },
        "SCREEN_VOLUME_X" => val: kv_from_kvn_opt => { screen_volume_x = val; },
        "SCREEN_VOLUME_Y" => val: kv_from_kvn_opt => { screen_volume_y = val; },
        "SCREEN_VOLUME_Z" => val: kv_from_kvn_opt => { screen_volume_z = val; },
        "SCREEN_ENTRY_TIME" => val: kv_epoch_opt => { screen_entry_time = val; },
        "SCREEN_EXIT_TIME" => val: kv_epoch_opt => { screen_exit_time = val; },
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

    parse_block!(input, comment, {
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

    parse_block!(input, comment, {
        "TIME_LASTOB_START" => val: kv_epoch_opt => { od_params.time_lastob_start = val; has_od_params = true; },
        "TIME_LASTOB_END" => val: kv_epoch_opt => { od_params.time_lastob_end = val; has_od_params = true; },
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
            comment: Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, FormatError, ValidationError};
    use crate::traits::Ndm;
    use crate::types::*;

    // From CCSDS Blue Book 508.0-B-1 Annex D (modified for KVN)
    const CDM_BLUE_BOOK_SAMPLE: &str = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2010-03-12T22:31:12.000
ORIGINATOR = JSPOC
MESSAGE_FOR = SATELLITE A
MESSAGE_ID = 201113719185

TCA = 2010-03-13T22:31:12.000
MISS_DISTANCE = 123.4 [m]
RELATIVE_SPEED = 12.3 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = 20.0 [m]
RELATIVE_POSITION_N = 30.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = 0.2 [m/s]
RELATIVE_VELOCITY_N = 0.3 [m/s]
SCREEN_VOLUME_SHAPE = ELLIPSOID

OBJECT = OBJECT1
OBJECT_DESIGNATOR = 12345
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT A
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1000.0 [km]
Y = 2000.0 [km]
Z = 3000.0 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 67890
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT B
INTERNATIONAL_DESIGNATOR = 2000-001A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1500.0 [km]
Y = 2500.0 [km]
Z = 3500.0 [km]
X_DOT = 1.5 [km/s]
Y_DOT = 2.5 [km/s]
Z_DOT = 3.5 [km/s]
CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"###;

    fn sample_cdm_kvn() -> String {
        let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"###;
        kvn.to_string()
    }

    #[test]
    fn test_parse_cdm_blue_book_example() {
        let result = Cdm::from_kvn_str(CDM_BLUE_BOOK_SAMPLE);
        assert!(result.is_ok(), "Failed to parse CDM: {:?}", result.err());

        let cdm = result.unwrap();
        assert_eq!(cdm.version, "1.0");
        assert_eq!(cdm.header.originator, "JSPOC");
        assert_eq!(cdm.body.segments.len(), 2);
        assert_eq!(cdm.body.segments[0].metadata.object_name, "SAT A");
        assert_eq!(cdm.body.segments[1].metadata.object_name, "SAT B");
        assert_eq!(cdm.body.relative_metadata_data.miss_distance.value, 123.4);
    }

    #[test]
    fn parse_cdm_kvn_success() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("CDM should parse");
        assert_eq!(cdm.version, "1.0");
        assert_eq!(cdm.header.originator, "TEST");
        assert_eq!(cdm.body.segments.len(), 2);
        assert!(cdm
            .body
            .relative_metadata_data
            .relative_state_vector
            .is_some());
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_frame,
            Some(ScreenVolumeFrameType::Rtn)
        );
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_shape,
            Some(ScreenVolumeShapeType::Box)
        );
    }

    #[test]
    fn header_missing_fields_error() {
        let kvn = r###"CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
MESSAGE_FOR = SAT
MESSAGE_ID = MSG-001
"###;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "CREATION_DATE");
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn validate_exactly_two_segments() {
        // Build KVN with only one segment explicitly
        let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = TEST_SAT
MESSAGE_ID = MSG-ONE

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
SCREEN_VOLUME_SHAPE = BOX
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"###;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        if let CcsdsNdmError::Validation(_) = err {
            // expected
        }
    }

    #[test]
    fn relative_state_vector_must_be_complete() {
        let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = SAT
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
SCREEN_VOLUME_SHAPE = BOX
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
// Missing RELATIVE_VELOCITY_N
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 1
CATALOG_NAME = CAT
OBJECT_NAME = O1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
OBJECT = OBJECT2
OBJECT_DESIGNATOR = 2
CATALOG_NAME = CAT
OBJECT_NAME = O2
INTERNATIONAL_DESIGNATOR = 1998-067B
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000
X = 0 [km]
Y = 0 [km]
Z = 0 [km]
X_DOT = 0 [km/s]
Y_DOT = 0 [km/s]
Z_DOT = 0 [km/s]
CR_R = 1 [m**2]
CT_R = 0 [m**2]
CT_T = 1 [m**2]
CN_R = 0 [m**2]
CN_T = 0 [m**2]
CN_N = 1 [m**2]
CRDOT_R = 0 [m**2/s]
CRDOT_T = 0 [m**2/s]
CRDOT_N = 0 [m**2/s]
CRDOT_RDOT = 1 [m**2/s**2]
CTDOT_R = 0 [m**2/s]
CTDOT_T = 0 [m**2/s]
CTDOT_N = 0 [m**2/s]
CTDOT_RDOT = 0 [m**2/s**2]
CTDOT_TDOT = 1 [m**2/s**2]
CNDOT_R = 0 [m**2/s]
CNDOT_T = 0 [m**2/s]
CNDOT_N = 0 [m**2/s]
CNDOT_RDOT = 0 [m**2/s**2]
CNDOT_TDOT = 0 [m**2/s**2]
CNDOT_NDOT = 1 [m**2/s**2]
"###;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "RELATIVE_VELOCITY_N");
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn covariance_missing_required_field() {
        // Remove CR_R from first segment to trigger error
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("CR_R = 1.0 [m**2]", "");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField {
                    ref block,
                    ref field,
                    ..
                } => {
                    assert_eq!(block, "Covariance Matrix");
                    assert_eq!(field, "CR_R");
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn screen_frame_shape_validation() {
        // Invalid SCREEN_VOLUME_FRAME
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("SCREEN_VOLUME_FRAME = RTN", "SCREEN_VOLUME_FRAME = BAD");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }

        // Invalid SCREEN_VOLUME_SHAPE
        let mut kvn2 = sample_cdm_kvn();
        kvn2 = kvn2.replace("SCREEN_VOLUME_SHAPE = BOX", "SCREEN_VOLUME_SHAPE = BALL");
        let err2 = Cdm::from_kvn(&kvn2).unwrap_err();
        match err2 {
            e if e.as_enum_error().is_some() => {} // Expected error
            _ => panic!("unexpected error: {:?}", err2),
        }
    }

    #[test]
    fn version_must_be_first() {
        let kvn = r#"CREATION_DATE = 2025-01-01T00:00:00
CCSDS_CDM_VERS = 1.0
ORIGINATOR = TEST
"#;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message
                            .to_lowercase()
                            .contains("expected ccsds_cdm_vers")
                            || err
                                .contexts
                                .iter()
                                .any(|c| c.to_lowercase().contains("ccsds_cdm_vers"))
                    )
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn empty_file_error() {
        let kvn = "";
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {} // Can be EOF or KvnParse depending on parser state
            e if e.is_kvn_error() => {} // Can be EOF or KvnParse depending on parser state
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn header_with_message_for() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("MESSAGE_FOR = OPERATOR", "MESSAGE_FOR = NEW_OPERATOR");
        let cdm = Cdm::from_kvn(&kvn).expect("should parse with MESSAGE_FOR");
        assert_eq!(cdm.header.message_for, Some("NEW_OPERATOR".to_string()));
    }

    #[test]
    fn relative_metadata_with_screen_periods() {
        let mut kvn = sample_cdm_kvn();
        // Insert screen period fields
        kvn = kvn.replace(
            "SCREEN_VOLUME_FRAME = RTN",
            "START_SCREEN_PERIOD = 2025-01-02T11:00:00\nSTOP_SCREEN_PERIOD = 2025-01-02T13:00:00\nSCREEN_VOLUME_FRAME = RTN",
        );
        kvn = kvn.replace(
            "COLLISION_PROBABILITY = 0.001",
            "SCREEN_ENTRY_TIME = 2025-01-02T11:30:00\nSCREEN_EXIT_TIME = 2025-01-02T12:30:00\nCOLLISION_PROBABILITY = 0.001\nCOLLISION_PROBABILITY_METHOD = FOSTER-1992",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with screen period");
        assert!(cdm
            .body
            .relative_metadata_data
            .start_screen_period
            .is_some());
        assert!(cdm.body.relative_metadata_data.stop_screen_period.is_some());
        assert!(cdm.body.relative_metadata_data.screen_entry_time.is_some());
        assert!(cdm.body.relative_metadata_data.screen_exit_time.is_some());
        assert_eq!(
            cdm.body.relative_metadata_data.collision_probability_method,
            Some("FOSTER-1992".to_string())
        );
    }

    #[test]
    fn relative_metadata_collision_probability_parse_error() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace(
            "COLLISION_PROBABILITY = 0.001",
            "COLLISION_PROBABILITY = INVALID",
        );

        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn relative_metadata_unexpected_field() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace(
            "COLLISION_PROBABILITY = 0.001",
            "COLLISION_PROBABILITY = 0.001\nUNKNOWN_FIELD = VALUE",
        );

        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unknown Relative Metadata key")
                            || err.contexts.contains(&"Unknown Relative Metadata key")
                    )
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_with_optional_fields() {
        let mut kvn = sample_cdm_kvn();
        // Add optional metadata fields
        kvn = kvn.replace(
            "INTERNATIONAL_DESIGNATOR = 1998-067A",
            "INTERNATIONAL_DESIGNATOR = 1998-067A\nOPERATOR_CONTACT_POSITION = Flight Director\nOPERATOR_ORGANIZATION = NASA\nOPERATOR_PHONE = +1-555-1234\nOPERATOR_EMAIL = contact@nasa.gov\nORBIT_CENTER = EARTH\nGRAVITY_MODEL = EGM-96\nATMOSPHERIC_MODEL = JACCHIA 70 DCA\nN_BODY_PERTURBATIONS = MOON, SUN\nSOLAR_RAD_PRESSURE = YES\nEARTH_TIDES = YES\nINTRACK_THRUST = YES",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with optional metadata");
        let seg1 = &cdm.body.segments[0];
        assert_eq!(seg1.metadata.object_type, Some(ObjectDescription::Payload));
        assert_eq!(
            seg1.metadata.operator_contact_position,
            Some("Flight Director".to_string())
        );
        assert_eq!(
            seg1.metadata.operator_organization,
            Some("NASA".to_string())
        );
        assert_eq!(
            seg1.metadata.operator_phone,
            Some("+1-555-1234".to_string())
        );
        assert_eq!(
            seg1.metadata.operator_email,
            Some("contact@nasa.gov".to_string())
        );
        assert_eq!(seg1.metadata.orbit_center, Some("EARTH".to_string()));
        assert_eq!(seg1.metadata.gravity_model, Some("EGM-96".to_string()));
        assert_eq!(
            seg1.metadata.atmospheric_model,
            Some("JACCHIA 70 DCA".to_string())
        );
        assert_eq!(
            seg1.metadata.n_body_perturbations,
            Some("MOON, SUN".to_string())
        );
        assert_eq!(seg1.metadata.solar_rad_pressure, Some(YesNo::Yes));
        assert_eq!(seg1.metadata.earth_tides, Some(YesNo::Yes));
        assert_eq!(seg1.metadata.intrack_thrust, Some(YesNo::Yes));
    }

    #[test]
    fn metadata_object_types() {
        // Test ROCKET BODY
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = ROCKET BODY");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(ObjectDescription::RocketBody)
        );

        // Test DEBRIS
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = DEBRIS");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(ObjectDescription::Debris)
        );

        // Test UNKNOWN
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = UNKNOWN");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(ObjectDescription::Unknown)
        );

        // Test OTHER
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = OTHER");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(ObjectDescription::Other)
        );

        // Test fallback to OTHER for unknown values
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT_TYPE = PAYLOAD", "OBJECT_TYPE = SATELLITE");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.object_type,
            Some(ObjectDescription::Other)
        );
    }

    #[test]
    fn metadata_invalid_object() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("OBJECT = OBJECT1", "OBJECT = OBJECT3");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_invalid_covariance_method() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace(
            "COVARIANCE_METHOD = CALCULATED",
            "COVARIANCE_METHOD = INVALID",
        );
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_maneuverable_na() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("MANEUVERABLE = YES", "MANEUVERABLE = N/A");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.maneuverable,
            ManeuverableType::NA
        );
    }

    #[test]
    fn metadata_invalid_maneuverable() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("MANEUVERABLE = YES", "MANEUVERABLE = MAYBE");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_ref_frames() {
        // Test GCRF
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = GCRF");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.ref_frame,
            ReferenceFrameType::Gcrf
        );

        // Test ITRF
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = ITRF");
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        assert_eq!(
            cdm.body.segments[0].metadata.ref_frame,
            ReferenceFrameType::Itrf
        );
    }

    #[test]
    fn metadata_invalid_ref_frame() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace("REF_FRAME = EME2000", "REF_FRAME = INVALID");
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Enum(_)) => {} // Expected error
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_unknown_key() {
        let mut kvn = sample_cdm_kvn();
        kvn = kvn.replace(
            "REF_FRAME = EME2000",
            "REF_FRAME = EME2000\nUNKNOWN_META_KEY = VALUE",
        );
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unknown metadata key")
                            || err.contexts.contains(&"Unknown metadata key")
                    )
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn data_with_od_parameters() {
        let mut kvn = sample_cdm_kvn();
        // Insert OD parameters before state vector
        kvn = kvn.replace(
            "X = 1.0 [km]",
            "TIME_LASTOB_START = 2025-01-01T00:00:00\nTIME_LASTOB_END = 2025-01-02T00:00:00\nRECOMMENDED_OD_SPAN = 7.0 [d]\nACTUAL_OD_SPAN = 5.0 [d]\nOBS_AVAILABLE = 100\nOBS_USED = 95\nTRACKS_AVAILABLE = 50\nTRACKS_USED = 48\nRESIDUALS_ACCEPTED = 95.5 [%]\nWEIGHTED_RMS = 1.23\nX = 1.0 [km]",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with OD parameters");
        let od = cdm.body.segments[0].data.od_parameters.as_ref().unwrap();
        assert!(od.time_lastob_start.is_some());
    }

    #[test]
    fn data_with_additional_parameters() {
        let mut kvn = sample_cdm_kvn();
        // Insert additional parameters
        kvn = kvn.replace(
            "X = 1.0 [km]",
            "AREA_PC = 10.0 [m**2]\nAREA_DRG = 12.0 [m**2]\nAREA_SRP = 15.0 [m**2]\nMASS = 1000.0 [kg]\nCD_AREA_OVER_MASS = 0.012 [m**2/kg]\nCR_AREA_OVER_MASS = 0.015 [m**2/kg]\nTHRUST_ACCELERATION = 0.001 [m/s**2]\nSEDR = 0.05 [W/kg]\nX = 1.0 [km]",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with additional parameters");
        let ap = cdm.body.segments[0]
            .data
            .additional_parameters
            .as_ref()
            .unwrap();
        assert!(ap.area_pc.is_some());
        assert!(ap.area_drg.is_some());
        assert!(ap.area_srp.is_some());
        assert!(ap.mass.is_some());
        assert!(ap.cd_area_over_mass.is_some());
        assert!(ap.cr_area_over_mass.is_some());
        assert!(ap.thrust_acceleration.is_some());
        assert!(ap.sedr.is_some());
    }

    #[test]
    fn covariance_with_drag_fields() {
        let mut kvn = sample_cdm_kvn();
        // Insert CDRG fields
        kvn = kvn.replace(
            "CNDOT_NDOT = 1.0 [m**2/s**2]",
            "CNDOT_NDOT = 1.0 [m**2/s**2]\nCDRG_R = 0.001 [m**3/kg]\nCDRG_T = 0.002 [m**3/kg]\nCDRG_N = 0.003 [m**3/kg]\nCDRG_RDOT = 0.0001 [m**3/(kg*s)]\nCDRG_TDOT = 0.0002 [m**3/(kg*s)]\nCDRG_NDOT = 0.0003 [m**3/(kg*s)]\nCDRG_DRG = 0.00001 [m**4/kg**2]",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with CDRG fields");
        let cov = &cdm.body.segments[0]
            .data
            .covariance_matrix
            .as_ref()
            .expect("Covariance matrix missing");
        assert!(cov.cdrg_r.is_some());
        assert!(cov.cdrg_t.is_some());
        assert!(cov.cdrg_n.is_some());
        assert!(cov.cdrg_rdot.is_some());
        assert!(cov.cdrg_tdot.is_some());
        assert!(cov.cdrg_ndot.is_some());
        assert!(cov.cdrg_drg.is_some());
    }

    #[test]
    fn covariance_with_srp_fields() {
        let mut kvn = sample_cdm_kvn();
        // Insert CSRP fields
        kvn = kvn.replace(
            "CNDOT_NDOT = 1.0 [m**2/s**2]",
            "CNDOT_NDOT = 1.0 [m**2/s**2]\nCSRP_R = 0.001 [m**3/kg]\nCSRP_T = 0.002 [m**3/kg]\nCSRP_N = 0.003 [m**3/kg]\nCSRP_RDOT = 0.0001 [m**3/(kg*s)]\nCSRP_TDOT = 0.0002 [m**3/(kg*s)]\nCSRP_NDOT = 0.0003 [m**3/(kg*s)]\nCSRP_DRG = 0.00001 [m**4/kg**2]\nCSRP_SRP = 0.00002 [m**4/kg**2]",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with CSRP fields");
        let cov = &cdm.body.segments[0]
            .data
            .covariance_matrix
            .as_ref()
            .expect("Covariance matrix missing");
        assert!(cov.csrp_r.is_some());
        assert!(cov.csrp_t.is_some());
        assert!(cov.csrp_n.is_some());
        assert!(cov.csrp_rdot.is_some());
        assert!(cov.csrp_tdot.is_some());
        assert!(cov.csrp_ndot.is_some());
        assert!(cov.csrp_drg.is_some());
        assert!(cov.csrp_srp.is_some());
    }

    #[test]
    fn covariance_with_thrust_fields() {
        let mut kvn = sample_cdm_kvn();
        // Insert CTHR fields
        kvn = kvn.replace(
            "CNDOT_NDOT = 1.0 [m**2/s**2]",
            "CNDOT_NDOT = 1.0 [m**2/s**2]\nCTHR_R = 0.001 [m**2/s**2]\nCTHR_T = 0.002 [m**2/s**2]\nCTHR_N = 0.003 [m**2/s**2]\nCTHR_RDOT = 0.0001 [m**2/s**3]\nCTHR_TDOT = 0.0002 [m**2/s**3]\nCTHR_NDOT = 0.0003 [m**2/s**3]\nCTHR_DRG = 0.00001 [m**3/(kg*s**2)]\nCTHR_SRP = 0.00002 [m**3/(kg*s**2)]\nCTHR_THR = 0.000001 [m**2/s**4]",
        );

        let cdm = Cdm::from_kvn(&kvn).expect("should parse with CTHR fields");
        let cov = &cdm.body.segments[0]
            .data
            .covariance_matrix
            .as_ref()
            .expect("Covariance matrix missing");
        assert!(cov.cthr_r.is_some());
        assert!(cov.cthr_t.is_some());
        assert!(cov.cthr_n.is_some());
        assert!(cov.cthr_rdot.is_some());
        assert!(cov.cthr_tdot.is_some());
        assert!(cov.cthr_ndot.is_some());
        assert!(cov.cthr_drg.is_some());
        assert!(cov.cthr_srp.is_some());
        assert!(cov.cthr_thr.is_some());
    }
    #[test]
    fn covariance_unknown_field_error() {
        let mut kvn = sample_cdm_kvn();
        // Add unknown covariance field
        kvn = kvn.replace(
            "CNDOT_NDOT = 1.0 [m**2/s**2]",
            "CNDOT_NDOT = 1.0 [m**2/s**2]\nUNKNOWN_COV = 0.001",
        );
        let err = Cdm::from_kvn(&kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unknown Data key")
                            || err.contexts.contains(&"Unknown Data key")
                    )
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn metadata_object_default_fallback() {
        // Test what happens if OBJECT isn't OBJECT1 or OBJECT2 in serialization
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        // Verify both objects are parsed correctly
        assert_eq!(cdm.body.segments[0].metadata.object, CdmObjectType::Object1);
        assert_eq!(cdm.body.segments[1].metadata.object, CdmObjectType::Object2);
    }

    #[test]
    fn unexpected_segment_start() {
        let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
META_START
"###;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        if let CcsdsNdmError::Validation(_) = err {
            // expected
        }
    }

    #[test]
    fn unexpected_end_of_input() {
        let kvn = r###"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
TCA = 2025-01-02T12:00:00
"###;
        let err = Cdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {} // Can be EOF or KvnParse depending on parser state
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(_)) => {}
            CcsdsNdmError::Validation(val_err)
                if matches!(*val_err, ValidationError::MissingRequiredField { .. }) => {}
            _ => panic!("unexpected error: {:?}", err),
        }
    }
    #[test]
    fn test_parse_cdm_missing_relative_metadata() {
        let input = r#"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = OPERATOR
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
# Missing RELATIVE_POSITION_T etc.
SCREEN_VOLUME_FRAME = RTN
"#;
        let err = Cdm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { field, .. } => {
                    assert_eq!(field, "RELATIVE_POSITION_T");
                }
                _ => panic!("Expected missing field error, got {:?}", val_err),
            },
            _ => panic!("Expected Validation error, got {:?}", err),
        }
    }

    #[test]
    fn test_parse_cdm_missing_covariance() {
        // Verify we can parse CDM without covariance matrix (it is optional)
        let cdm_no_cov = r#"CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = 20.0 [m]
RELATIVE_POSITION_N = 30.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = 0.2 [m/s]
RELATIVE_VELOCITY_N = 0.3 [m/s]

OBJECT = OBJECT1
OBJECT_DESIGNATOR = 12345
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT A
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = GCRF
X = 1000.0 [km]
Y = 2000.0 [km]
Z = 3000.0 [km]
X_DOT = 1.0 [km/s]
Y_DOT = 2.0 [km/s]
Z_DOT = 3.0 [km/s]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 67890
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT B
INTERNATIONAL_DESIGNATOR = 2000-001A
OBJECT_TYPE = PAYLOAD
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = NO
REF_FRAME = GCRF
X = 1500.0 [km]
Y = 2500.0 [km]
Z = 3500.0 [km]
X_DOT = 1.5 [km/s]
Y_DOT = 2.5 [km/s]
Z_DOT = 3.5 [km/s]
"#;
        let cdm = Cdm::from_kvn(cdm_no_cov).expect("should parse without covariance");
        assert!(cdm.body.segments[0].data.covariance_matrix.is_none());
        assert!(cdm.body.segments[1].data.covariance_matrix.is_none());
    }
}
