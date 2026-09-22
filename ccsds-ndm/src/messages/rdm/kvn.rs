// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for RDM (Re-entry Data Message).
//!
//! This module implements KVN parsing for RDM using winnow parser combinators.

use super::{Rdm, RdmBody, RdmData, RdmHeader, RdmMetadata, RdmSegment};
use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, OpmCovarianceMatrix,
    RdmSpacecraftParameters, StateVector,
};
use crate::error::Result;
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;
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
        "EPOCH_TZERO" => epoch_tzero: kv_calendar_epoch,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => val: kv_calendar_epoch_opt => { ref_frame_epoch = val; },
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
        "PREVIOUS_MESSAGE_EPOCH" => val: kv_calendar_epoch_opt => { previous_message_epoch = val; },
        "NEXT_MESSAGE_EPOCH" => val: kv_calendar_epoch_opt => { next_message_epoch = val; },
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
    let mut atmospheric_comment = Vec::new();

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

    let mut sv_comment = Vec::new();
    let mut sv_epoch = None;
    let mut sv_x = None;
    let mut sv_y = None;
    let mut sv_z = None;
    let mut sv_x_dot = None;
    let mut sv_y_dot = None;
    let mut sv_z_dot = None;

    let mut cov_comment = Vec::new();
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

    parse_routed_block!(input, |key, comments| {
        match key {
            "ORBIT_LIFETIME"
            | "REENTRY_ALTITUDE"
            | "ORBIT_LIFETIME_WINDOW_START"
            | "ORBIT_LIFETIME_WINDOW_END"
            | "NOMINAL_REENTRY_EPOCH"
            | "REENTRY_WINDOW_START"
            | "REENTRY_WINDOW_END"
            | "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => atmospheric_comment.extend(comments),
            "PROBABILITY_OF_IMPACT"
            | "PROBABILITY_OF_BURN_UP"
            | "PROBABILITY_OF_BREAK_UP"
            | "PROBABILITY_OF_LAND_IMPACT"
            | "PROBABILITY_OF_CASUALTY"
            | "NOMINAL_IMPACT_EPOCH"
            | "IMPACT_WINDOW_START"
            | "IMPACT_WINDOW_END"
            | "IMPACT_REF_FRAME"
            | "NOMINAL_IMPACT_LON"
            | "NOMINAL_IMPACT_LAT"
            | "NOMINAL_IMPACT_ALT"
            | "IMPACT_1_CONFIDENCE"
            | "IMPACT_1_START_LON"
            | "IMPACT_1_START_LAT"
            | "IMPACT_1_STOP_LON"
            | "IMPACT_1_STOP_LAT"
            | "IMPACT_1_CROSS_TRACK"
            | "IMPACT_2_CONFIDENCE"
            | "IMPACT_2_START_LON"
            | "IMPACT_2_START_LAT"
            | "IMPACT_2_STOP_LON"
            | "IMPACT_2_STOP_LAT"
            | "IMPACT_2_CROSS_TRACK"
            | "IMPACT_3_CONFIDENCE"
            | "IMPACT_3_START_LON"
            | "IMPACT_3_START_LAT"
            | "IMPACT_3_STOP_LON"
            | "IMPACT_3_STOP_LAT"
            | "IMPACT_3_CROSS_TRACK" => ground_params.comment.extend(comments),
            "EPOCH" | "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT" => {
                sv_comment.extend(comments)
            }
            "COV_REF_FRAME" | "CX_X" | "CY_X" | "CY_Y" | "CZ_X" | "CZ_Y" | "CZ_Z"
            | "CX_DOT_X" | "CX_DOT_Y" | "CX_DOT_Z" | "CX_DOT_X_DOT" | "CY_DOT_X"
            | "CY_DOT_Y" | "CY_DOT_Z" | "CY_DOT_X_DOT" | "CY_DOT_Y_DOT" | "CZ_DOT_X"
            | "CZ_DOT_Y" | "CZ_DOT_Z" | "CZ_DOT_X_DOT" | "CZ_DOT_Y_DOT"
            | "CZ_DOT_Z_DOT" => cov_comment.extend(comments),
            "WET_MASS" | "DRY_MASS" | "HAZARDOUS_SUBSTANCES" | "SOLAR_RAD_AREA"
            | "SOLAR_RAD_COEFF" | "DRAG_AREA" | "DRAG_COEFF" | "RCS"
            | "BALLISTIC_COEFF" | "THRUST_ACCELERATION" => {
                spacecraft_params.comment.extend(comments)
            }
            "TIME_LASTOB_START" | "TIME_LASTOB_END" | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN" | "OBS_AVAILABLE" | "OBS_USED" | "TRACKS_AVAILABLE"
            | "TRACKS_USED" | "RESIDUALS_ACCEPTED" | "WEIGHTED_RMS" => {
                od_params.comment.extend(comments)
            }
            _ => unreachable!("RDM routed parser only invokes known keyword arms"),
        }
    }, {
        "ORBIT_LIFETIME" => val: kv_from_kvn => { orbit_lifetime = Some(val); },
        "REENTRY_ALTITUDE" => val: kv_from_kvn => { reentry_altitude = Some(val); },
        "ORBIT_LIFETIME_WINDOW_START" => val: kv_from_kvn => { orbit_lifetime_window_start = Some(val); },
        "ORBIT_LIFETIME_WINDOW_END" => val: kv_from_kvn => { orbit_lifetime_window_end = Some(val); },
        "NOMINAL_REENTRY_EPOCH" => val: kv_calendar_epoch => { nominal_reentry_epoch = Some(val); },
        "REENTRY_WINDOW_START" => val: kv_calendar_epoch => { reentry_window_start = Some(val); },
        "REENTRY_WINDOW_END" => val: kv_calendar_epoch => { reentry_window_end = Some(val); },
        "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => val: kv_from_kvn => { orbit_lifetime_confidence_level = Some(val); },

        "PROBABILITY_OF_IMPACT" => val: kv_from_kvn => { ground_params.probability_of_impact = Some(val); have_ground = true; },
        "PROBABILITY_OF_BURN_UP" => val: kv_from_kvn => { ground_params.probability_of_burn_up = Some(val); have_ground = true; },
        "PROBABILITY_OF_BREAK_UP" => val: kv_from_kvn => { ground_params.probability_of_break_up = Some(val); have_ground = true; },
        "PROBABILITY_OF_LAND_IMPACT" => val: kv_from_kvn => { ground_params.probability_of_land_impact = Some(val); have_ground = true; },
        "PROBABILITY_OF_CASUALTY" => val: kv_from_kvn => { ground_params.probability_of_casualty = Some(val); have_ground = true; },
        "NOMINAL_IMPACT_EPOCH" => val: kv_calendar_epoch => { ground_params.nominal_impact_epoch = Some(val); have_ground = true; },
        "IMPACT_WINDOW_START" => val: kv_calendar_epoch => { ground_params.impact_window_start = Some(val); have_ground = true; },
        "IMPACT_WINDOW_END" => val: kv_calendar_epoch => { ground_params.impact_window_end = Some(val); have_ground = true; },
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

        "EPOCH" => val: kv_calendar_epoch => { sv_epoch = Some(val); },
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

        "TIME_LASTOB_START" => val: kv_calendar_epoch => { od_params.time_lastob_start = Some(val); have_od = true; },
        "TIME_LASTOB_END" => val: kv_calendar_epoch => { od_params.time_lastob_end = Some(val); have_od = true; },
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
    }, "RDM data");

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
        comment: atmospheric_comment,
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
        // KVN has no distinct outer data-comment position; its comments belong to one of the
        // seven logical blocks defined by RDM section 3.5.
        comment: Vec::new(),
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

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const KEYS: &[&str] = &[
        "CCSDS_RDM_VERS",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_DESIGNATOR",
        "OBJECT_TYPE",
        "OBJECT_OWNER",
        "OBJECT_OPERATOR",
        "CONTROLLED_REENTRY",
        "CENTER_NAME",
        "TIME_SYSTEM",
        "EPOCH_TZERO",
        "REF_FRAME",
        "REF_FRAME_EPOCH",
        "EPHEMERIS_NAME",
        "GRAVITY_MODEL",
        "ATMOSPHERIC_MODEL",
        "SOLAR_FLUX_PREDICTION",
        "N_BODY_PERTURBATIONS",
        "SOLAR_RAD_PRESSURE",
        "EARTH_TIDES",
        "INTRACK_THRUST",
        "DRAG_PARAMETERS_SOURCE",
        "DRAG_PARAMETERS_ALTITUDE",
        "REENTRY_UNCERTAINTY_METHOD",
        "REENTRY_DISINTEGRATION",
        "IMPACT_UNCERTAINTY_METHOD",
        "PREVIOUS_MESSAGE_ID",
        "PREVIOUS_MESSAGE_EPOCH",
        "NEXT_MESSAGE_EPOCH",
        "ORBIT_LIFETIME",
        "REENTRY_ALTITUDE",
        "ORBIT_LIFETIME_WINDOW_START",
        "ORBIT_LIFETIME_WINDOW_END",
        "NOMINAL_REENTRY_EPOCH",
        "REENTRY_WINDOW_START",
        "REENTRY_WINDOW_END",
        "ORBIT_LIFETIME_CONFIDENCE_LEVEL",
        "PROBABILITY_OF_IMPACT",
        "PROBABILITY_OF_BURN_UP",
        "PROBABILITY_OF_BREAK_UP",
        "PROBABILITY_OF_LAND_IMPACT",
        "PROBABILITY_OF_CASUALTY",
        "NOMINAL_IMPACT_EPOCH",
        "IMPACT_WINDOW_START",
        "IMPACT_WINDOW_END",
        "IMPACT_REF_FRAME",
        "NOMINAL_IMPACT_LON",
        "NOMINAL_IMPACT_LAT",
        "NOMINAL_IMPACT_ALT",
        "IMPACT_1_CONFIDENCE",
        "IMPACT_1_START_LON",
        "IMPACT_1_START_LAT",
        "IMPACT_1_STOP_LON",
        "IMPACT_1_STOP_LAT",
        "IMPACT_1_CROSS_TRACK",
        "IMPACT_2_CONFIDENCE",
        "IMPACT_2_START_LON",
        "IMPACT_2_START_LAT",
        "IMPACT_2_STOP_LON",
        "IMPACT_2_STOP_LAT",
        "IMPACT_2_CROSS_TRACK",
        "IMPACT_3_CONFIDENCE",
        "IMPACT_3_START_LON",
        "IMPACT_3_START_LAT",
        "IMPACT_3_STOP_LON",
        "IMPACT_3_STOP_LAT",
        "IMPACT_3_CROSS_TRACK",
        "EPOCH",
        "X",
        "Y",
        "Z",
        "X_DOT",
        "Y_DOT",
        "Z_DOT",
        "COV_REF_FRAME",
        "CX_X",
        "CY_X",
        "CY_Y",
        "CZ_X",
        "CZ_Y",
        "CZ_Z",
        "CX_DOT_X",
        "CX_DOT_Y",
        "CX_DOT_Z",
        "CX_DOT_X_DOT",
        "CY_DOT_X",
        "CY_DOT_Y",
        "CY_DOT_Z",
        "CY_DOT_X_DOT",
        "CY_DOT_Y_DOT",
        "CZ_DOT_X",
        "CZ_DOT_Y",
        "CZ_DOT_Z",
        "CZ_DOT_X_DOT",
        "CZ_DOT_Y_DOT",
        "CZ_DOT_Z_DOT",
        "WET_MASS",
        "DRY_MASS",
        "HAZARDOUS_SUBSTANCES",
        "SOLAR_RAD_AREA",
        "SOLAR_RAD_COEFF",
        "DRAG_AREA",
        "DRAG_COEFF",
        "RCS",
        "BALLISTIC_COEFF",
        "THRUST_ACCELERATION",
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

    fn rank(key: &str) -> Option<u16> {
        if key.starts_with("USER_DEFINED_") {
            return Some(KEYS.len() as u16);
        }
        KEYS.iter()
            .position(|candidate| *candidate == key)
            .map(|rank| rank as u16)
    }

    fn group(rank: u16) -> u8 {
        match rank {
            0 => 0,
            1..=3 => 1,
            4..=32 => 2,
            33..=40 => 3,
            41..=70 => 4,
            71..=77 => 5,
            78..=99 => 6,
            100..=109 => 7,
            110..=119 => 8,
            _ => 9,
        }
    }

    fn comments_start_block(previous: u16, key: &str) -> bool {
        let Some(current) = rank(key) else {
            return false;
        };
        match current {
            1 => previous == 0,
            4 => previous == 3,
            33 => group(previous) == 2,
            _ => group(current) >= 4 && group(previous) < group(current),
        }
    }

    crate::kvn::strict::validate_odm_assignments(
        kvn,
        &crate::kvn::strict::OdmAssignmentRules {
            context: "while validating RDM KVN structure",
            message_name: "RDM",
            rank,
            comment_starts_block: comments_start_block,
            allows_non_increasing: |previous, current| {
                current.rank == KEYS.len() as u16
                    && previous.rank == current.rank
                    && current.key.starts_with("USER_DEFINED_")
            },
        },
    )
}

impl ToKvn for Rdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_RDM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for RdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

impl ToKvn for RdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for RdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for RdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(ref v) = self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        if let Some(v) = &self.object_owner {
            writer.write_pair("OBJECT_OWNER", v);
        }
        if let Some(v) = &self.object_operator {
            writer.write_pair("OBJECT_OPERATOR", v);
        }
        writer.write_pair("CONTROLLED_REENTRY", format!("{}", self.controlled_reentry));
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.ref_frame {
            writer.write_pair("REF_FRAME", v);
        }
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.ephemeris_name {
            writer.write_pair("EPHEMERIS_NAME", v);
        }
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.solar_flux_prediction {
            writer.write_pair("SOLAR_FLUX_PREDICTION", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", v);
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", v);
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", format!("{}", v));
        }
        if let Some(v) = &self.drag_parameters_source {
            writer.write_pair("DRAG_PARAMETERS_SOURCE", v);
        }
        if let Some(v) = &self.drag_parameters_altitude {
            writer.write_pair("DRAG_PARAMETERS_ALTITUDE", v);
        }
        if let Some(v) = &self.reentry_uncertainty_method {
            writer.write_pair("REENTRY_UNCERTAINTY_METHOD", v.to_string());
        }
        if let Some(v) = &self.reentry_disintegration {
            writer.write_pair("REENTRY_DISINTEGRATION", format!("{}", v));
        }
        if let Some(v) = &self.impact_uncertainty_method {
            writer.write_pair("IMPACT_UNCERTAINTY_METHOD", v.to_string());
        }
        if let Some(v) = &self.previous_message_id {
            writer.write_pair("PREVIOUS_MESSAGE_ID", v);
        }
        if let Some(v) = &self.previous_message_epoch {
            writer.write_pair("PREVIOUS_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.next_message_epoch {
            writer.write_pair("NEXT_MESSAGE_EPOCH", v);
        }
    }
}

impl Rdm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        if !self.body.segment.data.comment.is_empty() {
            return Err(crate::error::ValidationError::Generic {
                message: "RDM XML data-level COMMENT cannot be represented distinctly from the first KVN logical-block COMMENT".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl RdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // No DATA_START
        writer.write_comments(&self.comment);
        // Atmospheric (mandatory)
        self.atmospheric_reentry_parameters.write_kvn(writer);

        // Ground impact (optional)
        if let Some(g) = &self.ground_impact_parameters {
            g.write_kvn(writer);
        }

        // Optional blocks: write when present
        if let Some(sv) = &self.state_vector {
            sv.write_kvn(writer);
        }
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.wet_mass {
                writer.write_measure("WET_MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.dry_mass {
                writer.write_measure("DRY_MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.hazardous_substances {
                writer.write_pair("HAZARDOUS_SUBSTANCES", v);
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_measure("SOLAR_RAD_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_pair("SOLAR_RAD_COEFF", v);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_measure("DRAG_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_pair("DRAG_COEFF", v);
            }
            if let Some(v) = &sp.rcs {
                writer.write_measure("RCS", &v.to_unit_value());
            }
            if let Some(v) = &sp.ballistic_coeff {
                writer.write_measure("BALLISTIC_COEFF", v);
            }
            if let Some(v) = &sp.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", &v.to_unit_value());
            }
        }
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

        if let Some(ud) = &self.user_defined_parameters {
            writer.write_comments(&ud.comment);
            for p in &ud.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
        }
    }
}
