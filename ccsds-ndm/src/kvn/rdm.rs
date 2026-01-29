// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for RDM (Re-entry Data Message).
//!
//! This module implements KVN parsing for RDM using winnow parser combinators.

use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, OpmCovarianceMatrix,
    RdmSpacecraftParameters, StateVector,
};
use crate::kvn::parser::*;
use crate::messages::rdm::{Rdm, RdmBody, RdmData, RdmHeader, RdmMetadata, RdmSegment};
use crate::parse_block;
use crate::types::*;
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// RDM Version Parser
//----------------------------------------------------------------------

pub fn rdm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_RDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// RDM Header Parser
//----------------------------------------------------------------------

pub fn rdm_header(input: &mut &str) -> KvnResult<RdmHeader> {
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

        // RDM metadata fields might follow immediately without META_START
        if key == "OBJECT_NAME" {
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

    Ok(RdmHeader {
        comment,
        creation_date: creation_date
            .ok_or_else(|| missing_field_err(input, "Header", "CREATION_DATE"))?,
        originator: originator.ok_or_else(|| missing_field_err(input, "Header", "ORIGINATOR"))?,
        message_id: message_id.ok_or_else(|| missing_field_err(input, "Header", "MESSAGE_ID"))?,
    })
}

//----------------------------------------------------------------------
// RDM Metadata Parser
//----------------------------------------------------------------------

pub fn rdm_metadata(input: &mut &str) -> KvnResult<RdmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut international_designator = None;
    let mut catalog_name = None;
    let mut object_designator = None;
    let mut object_type = None;
    let mut object_owner = None;
    let mut object_operator = None;
    let mut controlled_reentry = None;
    let mut center_name = None;
    let mut time_system = None;
    let mut epoch_tzero = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut ephemeris_name = None;
    let mut gravity_model = None;
    let mut atmospheric_model = None;
    let mut solar_flux_prediction = None;
    let mut n_body_perturbations = None;
    let mut solar_rad_pressure = None;
    let mut earth_tides = None;
    let mut intrack_thrust = None;
    let mut drag_parameters_source = None;
    let mut drag_parameters_altitude = None;
    let mut reentry_uncertainty_method = None;
    let mut reentry_disintegration = None;
    let mut impact_uncertainty_method = None;
    let mut previous_message_id = None;
    let mut previous_message_epoch = None;
    let mut next_message_epoch = None;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "INTERNATIONAL_DESIGNATOR" => international_designator: kv_string,
        "CATALOG_NAME" => catalog_name: kv_string,
        "OBJECT_DESIGNATOR" => object_designator: kv_string,
        "OBJECT_TYPE" => object_type: kv_enum,
        "OBJECT_OWNER" => object_owner: kv_string,
        "OBJECT_OPERATOR" => object_operator: kv_string,
        "CONTROLLED_REENTRY" => controlled_reentry: kv_enum,
        "CENTER_NAME" => center_name: kv_string,
        "TIME_SYSTEM" => time_system: kv_string,
        "EPOCH_TZERO" => epoch_tzero: kv_epoch,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_epoch,
        "EPHEMERIS_NAME" => ephemeris_name: kv_string,
        "GRAVITY_MODEL" => gravity_model: kv_string,
        "ATMOSPHERIC_MODEL" => atmospheric_model: kv_string,
        "SOLAR_FLUX_PREDICTION" => solar_flux_prediction: kv_string,
        "N_BODY_PERTURBATIONS" => n_body_perturbations: kv_string,
        "SOLAR_RAD_PRESSURE" => solar_rad_pressure: kv_string,
        "EARTH_TIDES" => earth_tides: kv_string,
        "INTRACK_THRUST" => intrack_thrust: kv_yes_no,
        "DRAG_PARAMETERS_SOURCE" => drag_parameters_source: kv_string,
        "DRAG_PARAMETERS_ALTITUDE" => drag_parameters_altitude: kv_from_kvn,
        "REENTRY_UNCERTAINTY_METHOD" => reentry_uncertainty_method: kv_enum,
        "REENTRY_DISINTEGRATION" => reentry_disintegration: kv_enum,
        "IMPACT_UNCERTAINTY_METHOD" => impact_uncertainty_method: kv_enum,
        "PREVIOUS_MESSAGE_ID" => previous_message_id: kv_string,
        "PREVIOUS_MESSAGE_EPOCH" => previous_message_epoch: kv_epoch,
        "NEXT_MESSAGE_EPOCH" => next_message_epoch: kv_epoch,
    }, |_| false);

    Ok(RdmMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "OBJECT_NAME"))?,
        international_designator: international_designator
            .ok_or_else(|| missing_field_err(input, "Metadata", "INTERNATIONAL_DESIGNATOR"))?,
        catalog_name,
        object_designator,
        object_type,
        object_owner,
        object_operator,
        controlled_reentry: controlled_reentry
            .ok_or_else(|| missing_field_err(input, "Metadata", "CONTROLLED_REENTRY"))?,
        center_name: center_name
            .ok_or_else(|| missing_field_err(input, "Metadata", "CENTER_NAME"))?,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "Metadata", "TIME_SYSTEM"))?,
        epoch_tzero: epoch_tzero
            .ok_or_else(|| missing_field_err(input, "Metadata", "EPOCH_TZERO"))?,
        ref_frame,
        ref_frame_epoch,
        ephemeris_name,
        gravity_model,
        atmospheric_model,
        solar_flux_prediction,
        n_body_perturbations,
        solar_rad_pressure,
        earth_tides,
        intrack_thrust,
        drag_parameters_source,
        drag_parameters_altitude,
        reentry_uncertainty_method,
        reentry_disintegration,
        impact_uncertainty_method,
        previous_message_id,
        previous_message_epoch,
        next_message_epoch,
    })
}

//----------------------------------------------------------------------
// RDM Data Parser
//----------------------------------------------------------------------
pub fn rdm_data(input: &mut &str) -> KvnResult<RdmData> {
    let mut comment = Vec::new();

    let mut orbit_lifetime = None;
    let mut reentry_altitude = None;
    let mut orbit_lifetime_window_start = None;
    let mut orbit_lifetime_window_end = None;
    let mut nominal_reentry_epoch = None;
    let mut reentry_window_start = None;
    let mut reentry_window_end = None;
    let mut orbit_lifetime_confidence_level = None;

    let mut ground_params = GroundImpactParameters::default();
    let mut have_ground = false;

    let sv_comment = Vec::new();
    let mut sv_epoch = None;
    let mut sv_x = None;
    let mut sv_y = None;
    let mut sv_z = None;
    let mut sv_x_dot = None;
    let mut sv_y_dot = None;
    let mut sv_z_dot = None;

    let cov_comment = Vec::new();
    let mut cov_ref_frame = None;
    let mut cx_x = None;
    let mut cy_x = None;
    let mut cy_y = None;
    let mut cz_x = None;
    let mut cz_y = None;
    let mut cz_z = None;
    let mut cx_dot_x = None;
    let mut cx_dot_y = None;
    let mut cx_dot_z = None;
    let mut cx_dot_x_dot = None;
    let mut cy_dot_x = None;
    let mut cy_dot_y = None;
    let mut cy_dot_z = None;
    let mut cy_dot_x_dot = None;
    let mut cy_dot_y_dot = None;
    let mut cz_dot_x = None;
    let mut cz_dot_y = None;
    let mut cz_dot_z = None;
    let mut cz_dot_x_dot = None;
    let mut cz_dot_y_dot = None;
    let mut cz_dot_z_dot = None;

    let mut spacecraft_params = RdmSpacecraftParameters::default();
    let mut have_sp = false;

    let mut od_params = OdParameters::default();
    let mut have_od = false;

    let mut user_defined = UserDefined::default();

    parse_block!(input, comment, {
        "ORBIT_LIFETIME" => val: kv_from_kvn => { orbit_lifetime = Some(val); },
        "REENTRY_ALTITUDE" => val: kv_from_kvn => { reentry_altitude = Some(val); },
        "ORBIT_LIFETIME_WINDOW_START" => val: kv_from_kvn => { orbit_lifetime_window_start = Some(val); },
        "ORBIT_LIFETIME_WINDOW_END" => val: kv_from_kvn => { orbit_lifetime_window_end = Some(val); },
        "NOMINAL_REENTRY_EPOCH" => val: kv_epoch => { nominal_reentry_epoch = Some(val); },
        "REENTRY_WINDOW_START" => val: kv_epoch => { reentry_window_start = Some(val); },
        "REENTRY_WINDOW_END" => val: kv_epoch => { reentry_window_end = Some(val); },
        "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => val: kv_from_kvn => { orbit_lifetime_confidence_level = Some(val); },

        "PROBABILITY_OF_IMPACT" => val: kv_from_kvn => { ground_params.probability_of_impact = Some(val); have_ground = true; },
        "PROBABILITY_OF_BURN_UP" => val: kv_from_kvn => { ground_params.probability_of_burn_up = Some(val); have_ground = true; },
        "PROBABILITY_OF_BREAK_UP" => val: kv_from_kvn => { ground_params.probability_of_break_up = Some(val); have_ground = true; },
        "PROBABILITY_OF_LAND_IMPACT" => val: kv_from_kvn => { ground_params.probability_of_land_impact = Some(val); have_ground = true; },
        "PROBABILITY_OF_CASUALTY" => val: kv_from_kvn => { ground_params.probability_of_casualty = Some(val); have_ground = true; },
        "NOMINAL_IMPACT_EPOCH" => val: kv_epoch => { ground_params.nominal_impact_epoch = Some(val); have_ground = true; },
        "IMPACT_WINDOW_START" => val: kv_epoch => { ground_params.impact_window_start = Some(val); have_ground = true; },
        "IMPACT_WINDOW_END" => val: kv_epoch => { ground_params.impact_window_end = Some(val); have_ground = true; },
        "IMPACT_REF_FRAME" => val: kv_string => { ground_params.impact_ref_frame = Some(val); have_ground = true; },
        "NOMINAL_IMPACT_LON" => val: kv_from_kvn => { ground_params.nominal_impact_lon = Some(val); have_ground = true; },
        "NOMINAL_IMPACT_LAT" => val: kv_from_kvn => { ground_params.nominal_impact_lat = Some(val); have_ground = true; },
        "NOMINAL_IMPACT_ALT" => val: kv_from_kvn => { ground_params.nominal_impact_alt = Some(val); have_ground = true; },
        "IMPACT_1_CONFIDENCE" => val: kv_from_kvn => { ground_params.impact_1_confidence = Some(val); have_ground = true; },
        "IMPACT_1_START_LON" => val: kv_from_kvn => { ground_params.impact_1_start_lon = Some(val); have_ground = true; },
        "IMPACT_1_START_LAT" => val: kv_from_kvn => { ground_params.impact_1_start_lat = Some(val); have_ground = true; },
        "IMPACT_1_STOP_LON" => val: kv_from_kvn => { ground_params.impact_1_stop_lon = Some(val); have_ground = true; },
        "IMPACT_1_STOP_LAT" => val: kv_from_kvn => { ground_params.impact_1_stop_lat = Some(val); have_ground = true; },
        "IMPACT_1_CROSS_TRACK" => val: kv_from_kvn => { ground_params.impact_1_cross_track = Some(val); have_ground = true; },
        "IMPACT_2_CONFIDENCE" => val: kv_from_kvn => { ground_params.impact_2_confidence = Some(val); have_ground = true; },
        "IMPACT_2_START_LON" => val: kv_from_kvn => { ground_params.impact_2_start_lon = Some(val); have_ground = true; },
        "IMPACT_2_START_LAT" => val: kv_from_kvn => { ground_params.impact_2_start_lat = Some(val); have_ground = true; },
        "IMPACT_2_STOP_LON" => val: kv_from_kvn => { ground_params.impact_2_stop_lon = Some(val); have_ground = true; },
        "IMPACT_2_STOP_LAT" => val: kv_from_kvn => { ground_params.impact_2_stop_lat = Some(val); have_ground = true; },
        "IMPACT_2_CROSS_TRACK" => val: kv_from_kvn => { ground_params.impact_2_cross_track = Some(val); have_ground = true; },
        "IMPACT_3_CONFIDENCE" => val: kv_from_kvn => { ground_params.impact_3_confidence = Some(val); have_ground = true; },
        "IMPACT_3_START_LON" => val: kv_from_kvn => { ground_params.impact_3_start_lon = Some(val); have_ground = true; },
        "IMPACT_3_START_LAT" => val: kv_from_kvn => { ground_params.impact_3_start_lat = Some(val); have_ground = true; },
        "IMPACT_3_STOP_LON" => val: kv_from_kvn => { ground_params.impact_3_stop_lon = Some(val); have_ground = true; },
        "IMPACT_3_STOP_LAT" => val: kv_from_kvn => { ground_params.impact_3_stop_lat = Some(val); have_ground = true; },
        "IMPACT_3_CROSS_TRACK" => val: kv_from_kvn => { ground_params.impact_3_cross_track = Some(val); have_ground = true; },

        "EPOCH" => val: kv_epoch => { sv_epoch = Some(val); },
        "X" => val: kv_from_kvn => { sv_x = Some(val); },
        "Y" => val: kv_from_kvn => { sv_y = Some(val); },
        "Z" => val: kv_from_kvn => { sv_z = Some(val); },
        "X_DOT" => val: kv_from_kvn => { sv_x_dot = Some(val); },
        "Y_DOT" => val: kv_from_kvn => { sv_y_dot = Some(val); },
        "Z_DOT" => val: kv_from_kvn => { sv_z_dot = Some(val); },

        "COV_REF_FRAME" => val: kv_string => { cov_ref_frame = Some(val); },
        "CX_X" => val: kv_from_kvn => { cx_x = Some(val); },
        "CY_X" => val: kv_from_kvn => { cy_x = Some(val); },
        "CY_Y" => val: kv_from_kvn => { cy_y = Some(val); },
        "CZ_X" => val: kv_from_kvn => { cz_x = Some(val); },
        "CZ_Y" => val: kv_from_kvn => { cz_y = Some(val); },
        "CZ_Z" => val: kv_from_kvn => { cz_z = Some(val); },
        "CX_DOT_X" => val: kv_from_kvn => { cx_dot_x = Some(val); },
        "CX_DOT_Y" => val: kv_from_kvn => { cx_dot_y = Some(val); },
        "CX_DOT_Z" => val: kv_from_kvn => { cx_dot_z = Some(val); },
        "CX_DOT_X_DOT" => val: kv_from_kvn => { cx_dot_x_dot = Some(val); },
        "CY_DOT_X" => val: kv_from_kvn => { cy_dot_x = Some(val); },
        "CY_DOT_Y" => val: kv_from_kvn => { cy_dot_y = Some(val); },
        "CY_DOT_Z" => val: kv_from_kvn => { cy_dot_z = Some(val); },
        "CY_DOT_X_DOT" => val: kv_from_kvn => { cy_dot_x_dot = Some(val); },
        "CY_DOT_Y_DOT" => val: kv_from_kvn => { cy_dot_y_dot = Some(val); },
        "CZ_DOT_X" => val: kv_from_kvn => { cz_dot_x = Some(val); },
        "CZ_DOT_Y" => val: kv_from_kvn => { cz_dot_y = Some(val); },
        "CZ_DOT_Z" => val: kv_from_kvn => { cz_dot_z = Some(val); },
        "CZ_DOT_X_DOT" => val: kv_from_kvn => { cz_dot_x_dot = Some(val); },
        "CZ_DOT_Y_DOT" => val: kv_from_kvn => { cz_dot_y_dot = Some(val); },
        "CZ_DOT_Z_DOT" => val: kv_from_kvn => { cz_dot_z_dot = Some(val); },

        "WET_MASS" => val: kv_from_kvn => { spacecraft_params.wet_mass = Some(val); have_sp = true; },
        "DRY_MASS" => val: kv_from_kvn => { spacecraft_params.dry_mass = Some(val); have_sp = true; },
        "HAZARDOUS_SUBSTANCES" => val: kv_string => { spacecraft_params.hazardous_substances = Some(val); have_sp = true; },
        "SOLAR_RAD_AREA" => val: kv_from_kvn => { spacecraft_params.solar_rad_area = Some(val); have_sp = true; },
        "SOLAR_RAD_COEFF" => solar_rad_coeff: kv_from_kvn => { spacecraft_params.solar_rad_coeff = Some(solar_rad_coeff); have_sp = true; },
        "DRAG_AREA" => val: kv_from_kvn => { spacecraft_params.drag_area = Some(val); have_sp = true; },
        "DRAG_COEFF" => drag_coeff: kv_from_kvn => { spacecraft_params.drag_coeff = Some(drag_coeff); have_sp = true; },
        "RCS" => val: kv_from_kvn => { spacecraft_params.rcs = Some(val); have_sp = true; },
        "BALLISTIC_COEFF" => val: kv_from_kvn => { spacecraft_params.ballistic_coeff = Some(val); have_sp = true; },
        "THRUST_ACCELERATION" => val: kv_from_kvn => { spacecraft_params.thrust_acceleration = Some(val); have_sp = true; },

        "TIME_LASTOB_START" => val: kv_epoch => { od_params.time_lastob_start = Some(val); have_od = true; },
        "TIME_LASTOB_END" => val: kv_epoch => { od_params.time_lastob_end = Some(val); have_od = true; },
        "RECOMMENDED_OD_SPAN" => val: kv_from_kvn => { od_params.recommended_od_span = Some(val); have_od = true; },
        "ACTUAL_OD_SPAN" => val: kv_from_kvn => { od_params.actual_od_span = Some(val); have_od = true; },
        "OBS_AVAILABLE" => val: kv_u32 => { od_params.obs_available = Some(val.into()); have_od = true; },
        "OBS_USED" => val: kv_u32 => { od_params.obs_used = Some(val.into()); have_od = true; },
        "TRACKS_AVAILABLE" => val: kv_u32 => { od_params.tracks_available = Some(val.into()); have_od = true; },
        "TRACKS_USED" => val: kv_u32 => { od_params.tracks_used = Some(val.into()); have_od = true; },
        "RESIDUALS_ACCEPTED" => val: kv_from_kvn => { od_params.residuals_accepted = Some(val); have_od = true; },
        "WEIGHTED_RMS" => weighted_rms: kv_from_kvn => { od_params.weighted_rms = Some(weighted_rms); have_od = true; },
    }, |i: &mut &str| {
        let checkpoint = i.checkpoint();
        let _ = collect_comments.parse_next(i);
        let res = winnow::combinator::peek(key_token).parse_next(i).map(|k| k.starts_with("USER_DEFINED_")).unwrap_or(false);
        i.reset(&checkpoint);
        res
    });

    let have_sv = sv_epoch.is_some()
        || sv_x.is_some()
        || sv_y.is_some()
        || sv_z.is_some()
        || sv_x_dot.is_some()
        || sv_y_dot.is_some()
        || sv_z_dot.is_some();

    let state_vector = if have_sv {
        Some(StateVector {
            comment: sv_comment,
            epoch: sv_epoch.ok_or_else(|| missing_field_err(input, "State Vector", "EPOCH"))?,
            x: sv_x.ok_or_else(|| missing_field_err(input, "State Vector", "X"))?,
            y: sv_y.ok_or_else(|| missing_field_err(input, "State Vector", "Y"))?,
            z: sv_z.ok_or_else(|| missing_field_err(input, "State Vector", "Z"))?,
            x_dot: sv_x_dot.ok_or_else(|| missing_field_err(input, "State Vector", "X_DOT"))?,
            y_dot: sv_y_dot.ok_or_else(|| missing_field_err(input, "State Vector", "Y_DOT"))?,
            z_dot: sv_z_dot.ok_or_else(|| missing_field_err(input, "State Vector", "Z_DOT"))?,
        })
    } else {
        None
    };

    let have_cov = cx_x.is_some()
        || cy_x.is_some()
        || cy_y.is_some()
        || cz_x.is_some()
        || cz_y.is_some()
        || cz_z.is_some()
        || cx_dot_x.is_some()
        || cx_dot_y.is_some()
        || cx_dot_z.is_some()
        || cx_dot_x_dot.is_some()
        || cy_dot_x.is_some()
        || cy_dot_y.is_some()
        || cy_dot_z.is_some()
        || cy_dot_x_dot.is_some()
        || cy_dot_y_dot.is_some()
        || cz_dot_x.is_some()
        || cz_dot_y.is_some()
        || cz_dot_z.is_some()
        || cz_dot_x_dot.is_some()
        || cz_dot_y_dot.is_some()
        || cz_dot_z_dot.is_some();

    let covariance_matrix = if have_cov {
        Some(OpmCovarianceMatrix {
            comment: cov_comment,
            cov_ref_frame,
            cx_x: cx_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_X"))?,
            cy_x: cy_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_X"))?,
            cy_y: cy_y.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_Y"))?,
            cz_x: cz_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_X"))?,
            cz_y: cz_y.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_Y"))?,
            cz_z: cz_z.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_Z"))?,
            cx_dot_x: cx_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_X"))?,
            cx_dot_y: cx_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_Y"))?,
            cx_dot_z: cx_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_Z"))?,
            cx_dot_x_dot: cx_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_X_DOT"))?,
            cy_dot_x: cy_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_X"))?,
            cy_dot_y: cy_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Y"))?,
            cy_dot_z: cy_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Z"))?,
            cy_dot_x_dot: cy_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_X_DOT"))?,
            cy_dot_y_dot: cy_dot_y_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Y_DOT"))?,
            cz_dot_x: cz_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_X"))?,
            cz_dot_y: cz_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Y"))?,
            cz_dot_z: cz_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Z"))?,
            cz_dot_x_dot: cz_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_X_DOT"))?,
            cz_dot_y_dot: cz_dot_y_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Y_DOT"))?,
            cz_dot_z_dot: cz_dot_z_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Z_DOT"))?,
        })
    } else {
        None
    };

    loop {
        let checkpoint = input.checkpoint();
        let loop_comments = collect_comments.parse_next(input)?;

        let key = match key_token.parse_next(input) {
            Ok(k) if k.starts_with("USER_DEFINED_") => k,
            _ => {
                input.reset(&checkpoint);
                break;
            }
        };
        user_defined.comment.extend(loop_comments);
        let v = kv_string.parse_next(input)?;
        user_defined.user_defined.push(UserDefinedParameter {
            parameter: key.strip_prefix("USER_DEFINED_").unwrap().to_string(),
            value: v,
        });

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    let atmospheric_reentry_parameters = AtmosphericReentryParameters {
        comment: Vec::new(),
        orbit_lifetime: orbit_lifetime
            .ok_or_else(|| missing_field_err(input, "Atmospheric Reentry", "ORBIT_LIFETIME"))?,
        reentry_altitude: reentry_altitude
            .ok_or_else(|| missing_field_err(input, "Atmospheric Reentry", "REENTRY_ALTITUDE"))?,
        orbit_lifetime_window_start,
        orbit_lifetime_window_end,
        nominal_reentry_epoch,
        reentry_window_start,
        reentry_window_end,
        orbit_lifetime_confidence_level,
    };

    Ok(RdmData {
        comment,
        atmospheric_reentry_parameters,
        ground_impact_parameters: if have_ground {
            Some(ground_params)
        } else {
            None
        },
        state_vector,
        covariance_matrix,
        spacecraft_parameters: if have_sp {
            Some(spacecraft_params)
        } else {
            None
        },
        od_parameters: if have_od { Some(od_params) } else { None },
        user_defined_parameters: if user_defined.user_defined.is_empty() {
            None
        } else {
            Some(user_defined)
        },
    })
}

//----------------------------------------------------------------------
// RDM Segment Parser
//----------------------------------------------------------------------

pub fn rdm_segment(input: &mut &str) -> KvnResult<RdmSegment> {
    let metadata = rdm_metadata.parse_next(input)?;
    let data = rdm_data.parse_next(input)?;

    Ok(RdmSegment { metadata, data })
}

//----------------------------------------------------------------------
// RDM Body Parser
//----------------------------------------------------------------------

pub fn rdm_body(input: &mut &str) -> KvnResult<RdmBody> {
    let segment = rdm_segment.parse_next(input)?;
    Ok(RdmBody {
        segment: Box::new(segment),
    })
}

//----------------------------------------------------------------------
// Complete RDM Parser
//----------------------------------------------------------------------

pub fn parse_rdm(input: &mut &str) -> KvnResult<Rdm> {
    let version = rdm_version.parse_next(input)?;
    let header = rdm_header.parse_next(input)?;
    let body = rdm_body.parse_next(input)?;

    Ok(Rdm {
        header,
        body,
        id: Some("CCSDS_RDM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Rdm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_rdm.parse_next(input)
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
    #[test]
    fn test_xsd_rdm_root_attributes() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(rdm.id, Some("CCSDS_RDM_VERS".to_string()));
        assert_eq!(rdm.version, "1.0");
    }

    #[test]
    fn test_rdm_full_roundtrip_all_blocks() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = COMPREHENSIVE_TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T09:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2023-01-06T19:45:33
REENTRY_WINDOW_START = 2023-01-06T11:45:33
REENTRY_WINDOW_END = 2023-01-06T22:12:56
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.75
EPOCH = 2023-01-01T09:30:12
X = 4000.000000 [km]
Y = 4000.000000 [km]
Z = 4000.000000 [km]
X_DOT = 7.000000 [km/s]
Y_DOT = 7.000000 [km/s]
Z_DOT = 7.000000 [km/s]
COV_REF_FRAME = RTN
CX_X = 0.10000 [km**2]
CY_X = 0.10000 [km**2]
CY_Y = 0.10000 [km**2]
CZ_X = 0.10000 [km**2]
CZ_Y = 0.10000 [km**2]
CZ_Z = 0.10000 [km**2]
CX_DOT_X = 0.02000 [km**2/s]
CX_DOT_Y = 0.02000 [km**2/s]
CX_DOT_Z = 0.02000 [km**2/s]
CX_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_X = 0.02000 [km**2/s]
CY_DOT_Y = 0.02000 [km**2/s]
CY_DOT_Z = 0.02000 [km**2/s]
CY_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
CZ_DOT_X = 0.02000 [km**2/s]
CZ_DOT_Y = 0.02000 [km**2/s]
CZ_DOT_Z = 0.02000 [km**2/s]
CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
WET_MASS = 3582 [kg]
DRAG_AREA = 23.3565 [m**2]
DRAG_COEFF = 2.2634
ACTUAL_OD_SPAN = 3.4554 [d]
TRACKS_AVAILABLE = 18
TRACKS_USED = 17
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();

        assert!(rdm.body.segment.data.state_vector.is_some());
        assert!(rdm.body.segment.data.covariance_matrix.is_some());

        let kvn2 = rdm.to_kvn().unwrap();
        let rdm2 = Rdm::from_kvn(&kvn2).unwrap();

        assert_eq!(
            rdm.body.segment.metadata.object_name,
            rdm2.body.segment.metadata.object_name
        );
    }

    // ==========================================
    // Migrated XSD Compliance Tests
    // ==========================================

    #[test]
    fn test_xsd_rdm_header_mandatory_fields() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = ESA
MESSAGE_ID = ESA-20231113-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.header.message_id, "ESA-20231113-001");
    }

    #[test]
    fn test_xsd_rdm_header_optional_comments() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert!(rdm.header.comment.is_empty());
    }

    #[test]
    fn test_xsd_rdm_metadata_mandatory_fields() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = SENTINEL-1A
INTERNATIONAL_DESIGNATOR = 2014-016A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let meta = &rdm.body.segment.metadata;
        assert_eq!(meta.object_name, "SENTINEL-1A");
        assert_eq!(meta.international_designator, "2014-016A");
        assert_eq!(meta.center_name, "EARTH");
        assert_eq!(meta.time_system, "TAI");
    }

    #[test]
    fn test_xsd_rdm_controlled_type_values() {
        for (val, expected) in [
            ("YES", ControlledType::Yes),
            ("yes", ControlledType::Yes),
            ("NO", ControlledType::No),
            ("no", ControlledType::No),
            ("UNKNOWN", ControlledType::Unknown),
            ("unknown", ControlledType::Unknown),
        ] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = {}
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                val
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert_eq!(rdm.body.segment.metadata.controlled_reentry, expected);
        }
    }

    #[test]
    fn test_xsd_rdm_object_type_enum() {
        for obj_type in ["PAYLOAD", "ROCKET BODY", "DEBRIS", "UNKNOWN", "OTHER"] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
OBJECT_TYPE = {}
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                obj_type
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.metadata.object_type.is_some());
        }
    }

    #[test]
    fn test_xsd_rdm_intrack_thrust_yesno() {
        for val in ["YES", "NO"] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
INTRACK_THRUST = {}
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#,
                val
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.metadata.intrack_thrust.is_some());
        }
    }

    #[test]
    fn test_xsd_rdm_metadata_optional_fields() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
OBJECT_OWNER = ESA
OBJECT_OPERATOR = EUMETSAT
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
EPHEMERIS_NAME = NONE
GRAVITY_MODEL = EGM-96: 36D 360
ATMOSPHERIC_MODEL = NRLMSISE-00
SOLAR_FLUX_PREDICTION = PREDICTED
N_BODY_PERTURBATIONS = MOON, SUN
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
DRAG_PARAMETERS_SOURCE = OD
DRAG_PARAMETERS_ALTITUDE = 200 [km]
PREVIOUS_MESSAGE_ID = PREV-001
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let meta = &rdm.body.segment.metadata;
        assert_eq!(meta.catalog_name, Some("SATCAT".to_string()));
        assert_eq!(meta.object_designator, Some("12345".to_string()));
        assert_eq!(meta.object_owner, Some("ESA".to_string()));
        assert_eq!(meta.ref_frame, Some("EME2000".to_string()));
        assert_eq!(meta.gravity_model, Some("EGM-96: 36D 360".to_string()));
        assert_eq!(meta.atmospheric_model, Some("NRLMSISE-00".to_string()));
    }

    #[test]
    fn test_xsd_rdm_atmospheric_mandatory_fields() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 23.5 [d]
REENTRY_ALTITUDE = 150.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!((atmos.orbit_lifetime.value - 23.5).abs() < 1e-9);
        assert!((atmos.reentry_altitude.value - 150.0).abs() < 1e-9);
    }

    #[test]
    fn test_xsd_rdm_day_interval_units_required() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_WINDOW_START = 4.0 [d]
ORBIT_LIFETIME_WINDOW_END = 7.0 [d]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!(atmos.orbit_lifetime_window_start.is_some());
        assert!(atmos.orbit_lifetime_window_end.is_some());
    }

    #[test]
    fn test_xsd_rdm_percentage_type() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
ORBIT_LIFETIME_CONFIDENCE_LEVEL = 95.0 [%]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let atmos = &rdm.body.segment.data.atmospheric_reentry_parameters;
        assert!(atmos.orbit_lifetime_confidence_level.is_some());
    }

    #[test]
    fn test_xsd_rdm_probability_type_range() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.5
PROBABILITY_OF_BURN_UP = 0.0
PROBABILITY_OF_CASUALTY = 1.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let ground = rdm
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_ref()
            .unwrap();
        assert!((ground.probability_of_impact.as_ref().unwrap().value - 0.5).abs() < 1e-9);
        assert!((ground.probability_of_burn_up.as_ref().unwrap().value - 0.0).abs() < 1e-9);
        assert!((ground.probability_of_casualty.as_ref().unwrap().value - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_xsd_rdm_latitude_range() {
        for lat in ["-90.0", "0.0", "45.5", "90.0"] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_LAT = {}
"#,
                lat
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    #[test]
    fn test_xsd_rdm_longitude_range() {
        for lon in ["-180.0", "-45.5", "0.0", "90.0", "180.0"] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_LON = {}
"#,
                lon
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    #[test]
    fn test_xsd_rdm_altitude_range() {
        for alt in ["-430.0", "0.0", "1000.0", "8000.0"] {
            let kvn = format!(
                r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
NOMINAL_IMPACT_ALT = {}
"#,
                alt
            );
            let rdm = Rdm::from_kvn(&kvn).unwrap();
            assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        }
    }

    #[test]
    fn test_xsd_rdm_impact_confidence_intervals() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_1_CONFIDENCE = 50.0 [%]
IMPACT_1_START_LON = -10.0
IMPACT_1_START_LAT = 40.0
IMPACT_1_STOP_LON = 10.0
IMPACT_1_STOP_LAT = 45.0
IMPACT_1_CROSS_TRACK = 100.0 [km]
IMPACT_2_CONFIDENCE = 90.0 [%]
IMPACT_2_START_LON = -15.0
IMPACT_2_START_LAT = 38.0
IMPACT_2_STOP_LON = 15.0
IMPACT_2_STOP_LAT = 47.0
IMPACT_2_CROSS_TRACK = 200.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let ground = rdm.body.segment.data.ground_impact_parameters.unwrap();
        assert!(ground.impact_1_confidence.is_some());
        assert!(ground.impact_2_confidence.is_some());
    }

    #[test]
    fn test_xsd_rdm_state_vector_type() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sv = rdm.body.segment.data.state_vector.as_ref().unwrap();
        assert!((sv.x.value - 7000.0).abs() < 1e-9);
    }

    #[test]
    fn test_xsd_rdm_covariance_matrix_type() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
EPOCH = 2023-01-01T12:00:00
X = 7000.0 [km]
Y = 0.0 [km]
Z = 0.0 [km]
X_DOT = 0.0 [km/s]
Y_DOT = 7.5 [km/s]
Z_DOT = 0.0 [km/s]
COV_REF_FRAME = RTN
CX_X = 1.0e-4 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0e-4 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0e-4 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 1.0e-6 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 1.0e-6 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 1.0e-6 [km**2/s**2]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let cov = rdm.body.segment.data.covariance_matrix.as_ref().unwrap();
        assert_eq!(cov.cov_ref_frame, Some("RTN".to_string()));
        assert!((cov.cx_x.value - 1.0e-4).abs() < 1e-15);
    }

    #[test]
    fn test_xsd_rdm_spacecraft_parameters() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
WET_MASS = 3500 [kg]
DRY_MASS = 2000 [kg]
HAZARDOUS_SUBSTANCES = Hydrazine, Nuclear
SOLAR_RAD_AREA = 25.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 20.0 [m**2]
DRAG_COEFF = 2.2
RCS = 15.0 [m**2]
BALLISTIC_COEFF = 150.0
THRUST_ACCELERATION = 0.001
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sp = rdm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert!((sp.wet_mass.as_ref().unwrap().value - 3500.0).abs() < 1e-9);
        assert!((sp.dry_mass.as_ref().unwrap().value - 2000.0).abs() < 1e-9);
        assert_eq!(
            sp.hazardous_substances,
            Some("Hydrazine, Nuclear".to_string())
        );
        assert!(sp.ballistic_coeff.is_some());
        assert!(sp.thrust_acceleration.is_some());
    }

    #[test]
    fn test_xsd_rdm_coefficients_nonnegative() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
SOLAR_RAD_COEFF = 0.0
DRAG_COEFF = 0.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let sp = rdm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert!((sp.solar_rad_coeff.unwrap().value - 0.0).abs() < 1e-9);
        assert!((sp.drag_coeff.unwrap().value - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_xsd_rdm_od_parameters() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
TIME_LASTOB_START = 2022-12-31T00:00:00
TIME_LASTOB_END = 2022-12-31T23:59:59
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 20
TRACKS_USED = 18
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.234
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let od = rdm.body.segment.data.od_parameters.as_ref().unwrap();
        assert!(od.time_lastob_start.is_some());
        assert!(od.time_lastob_end.is_some());
        assert_eq!(od.obs_available, Some(100.into()));
        assert_eq!(od.obs_used, Some(95.into()));
        assert_eq!(od.tracks_available, Some(20.into()));
        assert_eq!(od.tracks_used, Some(18.into()));
    }

    #[test]
    fn test_xsd_rdm_sample_c1_kvn() {
        let kvn = std::fs::read_to_string("../data/kvn/rdm_c1.kvn").unwrap();
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    }

    #[test]
    fn test_xsd_rdm_sample_c2_kvn() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2018-04-22T09:31:34.00
ORIGINATOR = ESA
MESSAGE_ID = ESA/20180422-001
OBJECT_NAME = SPACEOBJECT
INTERNATIONAL_DESIGNATOR = 2018-099B
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 81594
OBJECT_TYPE = ROCKET BODY
OBJECT_OWNER = ESA
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2018-04-22T09:00:00.00
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96: 36D 36O
ATMOSPHERIC_MODEL = NRLMSISE-00
N_BODY_PERTURBATIONS = MOON
SOLAR_RAD_PRESSURE = NO
EARTH_TIDES = ESR
INTRACK_THRUST = NO
REENTRY_DISINTEGRATION = MASS-LOSS + BREAK-UP
PREVIOUS_MESSAGE_ID = ESA/20180421-007
NEXT_MESSAGE_EPOCH = 2018-04-23T09:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2018-04-27T19:45:33
REENTRY_WINDOW_START = 2018-04-27T11:45:33
REENTRY_WINDOW_END = 2018-04-27T22:12:56
PROBABILITY_OF_IMPACT = 0.0
PROBABILITY_OF_BURN_UP = 1.0
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.catalog_name,
            Some("SATCAT".to_string())
        );
        assert!(rdm
            .body
            .segment
            .data
            .atmospheric_reentry_parameters
            .nominal_reentry_epoch
            .is_some());
    }

    #[test]
    fn test_rdm_basic_kvn_roundtrip() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST-CENTER
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.message_id, "RDM-001");
        assert_eq!(rdm.body.segment.metadata.object_name, "TEST-SAT");
        let kvn2 = rdm.to_kvn().unwrap();
        assert!(kvn2.contains("CCSDS_RDM_VERS"));
        assert!(kvn2.contains("ORBIT_LIFETIME"));
    }

    #[test]
    fn test_rdm_header_requires_fields() {
        let kvn_missing_creation = r#"CCSDS_RDM_VERS = 1.0
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_creation).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "CREATION_DATE");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_originator = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_originator).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "ORIGINATOR");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_msgid = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_msgid).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "MESSAGE_ID");
        } else {
            panic!("Unexpected: {:?}", err);
        }
    }

    #[test]
    fn test_rdm_metadata_requires_mandatory_fields() {
        let kvn_missing_object_name = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_object_name).unwrap_err();
        println!("RDM ERROR: {:?}", err);
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "OBJECT_NAME");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_intl = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_intl).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "INTERNATIONAL_DESIGNATOR");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_center = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_center).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "CENTER_NAME");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_timesys = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_timesys).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "TIME_SYSTEM");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_controlled = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_controlled).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "CONTROLLED_REENTRY");
        } else {
            panic!("Unexpected: {:?}", err);
        }

        let kvn_missing_epoch = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
ORBIT_LIFETIME = 1 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_epoch).unwrap_err();
        if let Some(ValidationError::MissingRequiredField { field, .. }) = err.as_validation_error()
        {
            assert_eq!(field, "EPOCH_TZERO");
        } else {
            panic!("Unexpected: {:?}", err);
        }
    }

    #[test]
    fn test_rdm_data_requires_atmospheric_fields() {
        let kvn_missing_orbit_life = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REENTRY_ALTITUDE = 80 [km]
"#;
        let err = Rdm::from_kvn(kvn_missing_orbit_life).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField {
                    ref block,
                    ref field,
                    ..
                } => {
                    assert_eq!(block, "Atmospheric Reentry");
                    assert_eq!(field, "ORBIT_LIFETIME");
                }
                _ => panic!("Unexpected validation error: {:?}", val_err),
            },
            _ => panic!("Unexpected: {:?}", err),
        }

        let kvn_missing_reentry_alt = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 1 [d]
"#;
        let err = Rdm::from_kvn(kvn_missing_reentry_alt).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField {
                    ref block,
                    ref field,
                    ..
                } => {
                    assert_eq!(block, "Atmospheric Reentry");
                    assert_eq!(field, "REENTRY_ALTITUDE");
                }
                _ => panic!("Unexpected validation error: {:?}", val_err),
            },
            _ => panic!("Unexpected: {:?}", err),
        }
    }

    #[test]
    fn test_rdm_empty_file_error() {
        let err = Rdm::from_kvn("").unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {}
            e if e.is_kvn_error() => {}
            _ => panic!("Expected error, got: {:?}", err),
        }
    }

    #[test]
    fn test_rdm_version_not_first_error() {
        let kvn = r#"OBJECT_NAME = TEST
CCSDS_RDM_VERS = 1.0
"#;
        let err = Rdm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(err
                        .message
                        .to_lowercase()
                        .contains("expected ccsds_rdm_vers"));
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected version-not-first error, got: {:?}", err),
        }
    }

    #[test]
    fn test_rdm_metadata_optional_fields_kvn_roundtrip() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = CATALOG123
OBJECT_DESIGNATOR = DES456
OBJECT_TYPE = DEBRIS
OBJECT_OWNER = OWNER789
OBJECT_OPERATOR = OPERATOR012
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = TEME
REF_FRAME_EPOCH = 2023-01-01T12:00:00
EPHEMERIS_NAME = EPHEM_TEST
GRAVITY_MODEL = JGM-3: 20D 20O
ATMOSPHERIC_MODEL = JACCHIA-71
SOLAR_FLUX_PREDICTION = MEASURED
N_BODY_PERTURBATIONS = MOON,SUN,VENUS
SOLAR_RAD_PRESSURE = YES
EARTH_TIDES = NONE
INTRACK_THRUST = NO
DRAG_PARAMETERS_SOURCE = ESTIMATED
DRAG_PARAMETERS_ALTITUDE = 250.5 [km]
REENTRY_UNCERTAINTY_METHOD = COVARIANCE
REENTRY_DISINTEGRATION = BREAK-UP
IMPACT_UNCERTAINTY_METHOD = STATISTICAL
PREVIOUS_MESSAGE_ID = MSG-PREV-001
PREVIOUS_MESSAGE_EPOCH = 2022-12-25T00:00:00
NEXT_MESSAGE_EPOCH = 2023-01-08T00:00:00
ORBIT_LIFETIME = 10 [d]
REENTRY_ALTITUDE = 120 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let kvn2 = rdm.to_kvn().unwrap();

        assert!(kvn2.contains("CATALOG_NAME") && kvn2.contains("CATALOG123"));
        assert!(kvn2.contains("OBJECT_DESIGNATOR") && kvn2.contains("DES456"));
        assert!(kvn2.contains("OBJECT_TYPE") && kvn2.contains("DEBRIS"));
        assert!(kvn2.contains("OBJECT_OWNER") && kvn2.contains("OWNER789"));
        assert!(kvn2.contains("OBJECT_OPERATOR") && kvn2.contains("OPERATOR012"));
        assert!(kvn2.contains("REF_FRAME") && kvn2.contains("TEME"));
        assert!(kvn2.contains("EPHEMERIS_NAME") && kvn2.contains("EPHEM_TEST"));
        assert!(kvn2.contains("GRAVITY_MODEL") && kvn2.contains("JGM-3: 20D 20O"));
        assert!(kvn2.contains("ATMOSPHERIC_MODEL") && kvn2.contains("JACCHIA-71"));
        assert!(kvn2.contains("SOLAR_FLUX_PREDICTION") && kvn2.contains("MEASURED"));
        assert!(kvn2.contains("N_BODY_PERTURBATIONS") && kvn2.contains("MOON,SUN,VENUS"));
        assert!(kvn2.contains("SOLAR_RAD_PRESSURE") && kvn2.contains("YES"));
        assert!(kvn2.contains("EARTH_TIDES") && kvn2.contains("NONE"));
        assert!(kvn2.contains("INTRACK_THRUST") && kvn2.contains("NO"));
        assert!(kvn2.contains("DRAG_PARAMETERS_SOURCE") && kvn2.contains("ESTIMATED"));
        assert!(kvn2.contains("REENTRY_UNCERTAINTY_METHOD") && kvn2.contains("COVARIANCE"));
        assert!(kvn2.contains("REENTRY_DISINTEGRATION") && kvn2.contains("BREAK-UP"));
        assert!(kvn2.contains("IMPACT_UNCERTAINTY_METHOD") && kvn2.contains("STATISTICAL"));
        assert!(kvn2.contains("PREVIOUS_MESSAGE_ID") && kvn2.contains("MSG-PREV-001"));

        let rdm2 = Rdm::from_kvn(&kvn2).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.catalog_name,
            rdm2.body.segment.metadata.catalog_name
        );
    }

    #[test]
    fn test_rdm_ground_impact_all_probabilities() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.60
PROBABILITY_OF_BREAK_UP = 0.35
PROBABILITY_OF_LAND_IMPACT = 0.15
PROBABILITY_OF_CASUALTY = 0.001
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let g = rdm
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_ref()
            .unwrap();
        assert!((g.probability_of_impact.as_ref().unwrap().value - 0.25).abs() < 1e-9);
        assert!((g.probability_of_burn_up.as_ref().unwrap().value - 0.60).abs() < 1e-9);
        assert!((g.probability_of_break_up.as_ref().unwrap().value - 0.35).abs() < 1e-9);
        assert!((g.probability_of_land_impact.as_ref().unwrap().value - 0.15).abs() < 1e-9);
        assert!((g.probability_of_casualty.as_ref().unwrap().value - 0.001).abs() < 1e-9);

        let kvn2 = rdm.to_kvn().unwrap();
        assert!(kvn2.contains("PROBABILITY_OF_IMPACT"));
        assert!(kvn2.contains("PROBABILITY_OF_BURN_UP"));
        assert!(kvn2.contains("PROBABILITY_OF_BREAK_UP"));
        assert!(kvn2.contains("PROBABILITY_OF_LAND_IMPACT"));
        assert!(kvn2.contains("PROBABILITY_OF_CASUALTY"));
    }

    #[test]
    fn test_rdm_ground_impact_nominal_and_windows() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.5
NOMINAL_IMPACT_EPOCH = 2023-01-06T15:30:00
IMPACT_WINDOW_START = 2023-01-06T12:00:00
IMPACT_WINDOW_END = 2023-01-06T18:00:00
IMPACT_REF_FRAME = EFG
NOMINAL_IMPACT_LON = -120.5
NOMINAL_IMPACT_LAT = 35.2
NOMINAL_IMPACT_ALT = 0.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let g = rdm
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_ref()
            .unwrap();
        assert!(g.nominal_impact_epoch.is_some());
        assert!(g.impact_window_start.is_some());
        assert!(g.impact_window_end.is_some());
        assert_eq!(g.impact_ref_frame.as_deref(), Some("EFG"));
        assert!((g.nominal_impact_lon.as_ref().unwrap().value - (-120.5)).abs() < 1e-9);
        assert!((g.nominal_impact_lat.as_ref().unwrap().value - 35.2).abs() < 1e-9);

        let kvn2 = rdm.to_kvn().unwrap();
        assert!(kvn2.contains("NOMINAL_IMPACT_EPOCH"));
        assert!(kvn2.contains("IMPACT_WINDOW_START"));
        assert!(kvn2.contains("IMPACT_WINDOW_END"));
        assert!(kvn2.contains("IMPACT_REF_FRAME"));
        assert!(kvn2.contains("NOMINAL_IMPACT_LON"));
        assert!(kvn2.contains("NOMINAL_IMPACT_LAT"));
        assert!(kvn2.contains("NOMINAL_IMPACT_ALT"));
    }

    #[test]
    fn test_rdm_ground_impact_confidence_intervals_1() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_1_CONFIDENCE = 68.3 [%]
IMPACT_1_START_LON = -125.0
IMPACT_1_START_LAT = 30.0
IMPACT_1_STOP_LON = -115.0
IMPACT_1_STOP_LAT = 40.0
IMPACT_1_CROSS_TRACK = 50.0 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let g = rdm
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_ref()
            .unwrap();
        assert!((g.impact_1_confidence.as_ref().unwrap().value - 68.3).abs() < 1e-9);
        assert!((g.impact_1_start_lon.as_ref().unwrap().value - (-125.0)).abs() < 1e-9);
        assert!((g.impact_1_start_lat.as_ref().unwrap().value - 30.0).abs() < 1e-9);
        assert!((g.impact_1_stop_lon.as_ref().unwrap().value - (-115.0)).abs() < 1e-9);
        assert!((g.impact_1_stop_lat.as_ref().unwrap().value - 40.0).abs() < 1e-9);
        assert!((g.impact_1_cross_track.as_ref().unwrap().value - 50.0).abs() < 1e-9);

        let kvn2 = rdm.to_kvn().unwrap();
        assert!(kvn2.contains("IMPACT_1_CONFIDENCE"));
        assert!(kvn2.contains("IMPACT_1_START_LON"));
        assert!(kvn2.contains("IMPACT_1_START_LAT"));
        assert!(kvn2.contains("IMPACT_1_STOP_LON"));
        assert!(kvn2.contains("IMPACT_1_STOP_LAT"));
    }
}
