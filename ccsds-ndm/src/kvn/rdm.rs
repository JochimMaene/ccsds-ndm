// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for RDM (Re-entry Data Message).
//!
//! This module implements KVN parsing for RDM using winnow parser combinators.

use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, RdmSpacecraftParameters,
};
use crate::kvn::parser::*;
use crate::messages::rdm::{Rdm, RdmBody, RdmData, RdmHeader, RdmMetadata, RdmSegment};
use crate::types::*;
use std::str::FromStr;
use winnow::error::{AddContext, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;

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
        comment.extend(collect_comments.parse_next(input)?);

        let checkpoint = input.checkpoint();
        let key = match key_token.parse_next(input) {
            Ok(k) => k,
            Err(_) => break,
        };

        match key {
            "CREATION_DATE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                creation_date =
                    Some(Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid CREATION_DATE"))?);
            }
            "ORIGINATOR" => {
                let (v, _) = kv_rest.parse_next(input)?;
                originator = Some(v.to_string());
            }
            "MESSAGE_ID" => {
                let (v, _) = kv_rest.parse_next(input)?;
                message_id = Some(v.to_string());
            }
            _ => {
                // End of header
                input.reset(&checkpoint);
                break;
            }
        }
    }

    Ok(RdmHeader {
        comment,
        creation_date: creation_date.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CREATION_DATE")),
            ))
        })?,
        originator: originator.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("ORIGINATOR")),
            ))
        })?,
        message_id: message_id.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MESSAGE_ID")),
            ))
        })?,
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

    loop {
        comment.extend(collect_comments.parse_next(input)?);

        let checkpoint = input.checkpoint();
        let key = match key_token.parse_next(input) {
            Ok(k) => k,
            Err(_) => break,
        };

        match key {
            "OBJECT_NAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                object_name = Some(v.to_string());
            }
            "INTERNATIONAL_DESIGNATOR" => {
                let (v, _) = kv_rest.parse_next(input)?;
                international_designator = Some(v.to_string());
            }
            "CATALOG_NAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                catalog_name = Some(v.to_string());
            }
            "OBJECT_DESIGNATOR" => {
                let (v, _) = kv_rest.parse_next(input)?;
                object_designator = Some(v.to_string());
            }
            "OBJECT_TYPE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                object_type = Some(
                    ObjectDescription::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid OBJECT_TYPE"))?,
                );
            }
            "OBJECT_OWNER" => {
                let (v, _) = kv_rest.parse_next(input)?;
                object_owner = Some(v.to_string());
            }
            "OBJECT_OPERATOR" => {
                let (v, _) = kv_rest.parse_next(input)?;
                object_operator = Some(v.to_string());
            }
            "CONTROLLED_REENTRY" => {
                let (v, _) = kv_rest.parse_next(input)?;
                controlled_reentry = Some(
                    ControlledType::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid CONTROLLED_REENTRY"))?,
                );
            }
            "CENTER_NAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                center_name = Some(v.to_string());
            }
            "TIME_SYSTEM" => {
                let (v, _) = kv_rest.parse_next(input)?;
                time_system = Some(v.to_string());
            }
            "EPOCH_TZERO" => {
                let (v, _) = kv_rest.parse_next(input)?;
                epoch_tzero =
                    Some(Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid EPOCH_TZERO"))?);
            }
            "REF_FRAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ref_frame = Some(v.to_string());
            }
            "REF_FRAME_EPOCH" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ref_frame_epoch = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid REF_FRAME_EPOCH"))?,
                );
            }
            "EPHEMERIS_NAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ephemeris_name = Some(v.to_string());
            }
            "GRAVITY_MODEL" => {
                let (v, _) = kv_rest.parse_next(input)?;
                gravity_model = Some(v.to_string());
            }
            "ATMOSPHERIC_MODEL" => {
                let (v, _) = kv_rest.parse_next(input)?;
                atmospheric_model = Some(v.to_string());
            }
            "SOLAR_FLUX_PREDICTION" => {
                let (v, _) = kv_rest.parse_next(input)?;
                solar_flux_prediction = Some(v.to_string());
            }
            "N_BODY_PERTURBATIONS" => {
                let (v, _) = kv_rest.parse_next(input)?;
                n_body_perturbations = Some(v.to_string());
            }
            "SOLAR_RAD_PRESSURE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                solar_rad_pressure = Some(v.to_string());
            }
            "EARTH_TIDES" => {
                let (v, _) = kv_rest.parse_next(input)?;
                earth_tides = Some(v.to_string());
            }
            "INTRACK_THRUST" => {
                let (v, _) = kv_rest.parse_next(input)?;
                intrack_thrust =
                    Some(YesNo::from_str(v).map_err(|_| cut_err(input, "Invalid INTRACK_THRUST"))?);
            }
            "DRAG_PARAMETERS_SOURCE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                drag_parameters_source = Some(v.to_string());
            }
            "DRAG_PARAMETERS_ALTITUDE" => {
                let (v, u) = kv_rest.parse_next(input)?;
                drag_parameters_altitude = Some(
                    PositionRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid DRAG_PARAMETERS_ALTITUDE"))?,
                );
            }
            "REENTRY_UNCERTAINTY_METHOD" => {
                let (v, _) = kv_rest.parse_next(input)?;
                reentry_uncertainty_method = Some(v.to_string())
            }
            "REENTRY_DISINTEGRATION" => {
                let (v, _) = kv_rest.parse_next(input)?;
                reentry_disintegration = Some(v.to_string());
            }
            "IMPACT_UNCERTAINTY_METHOD" => {
                let (v, _) = kv_rest.parse_next(input)?;
                impact_uncertainty_method = Some(v.to_string());
            }
            "PREVIOUS_MESSAGE_ID" => {
                let (v, _) = kv_rest.parse_next(input)?;
                previous_message_id = Some(v.to_string());
            }
            "PREVIOUS_MESSAGE_EPOCH" => {
                let (v, _) = kv_rest.parse_next(input)?;
                previous_message_epoch = Some(
                    Epoch::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid PREVIOUS_MESSAGE_EPOCH"))?,
                );
            }
            "NEXT_MESSAGE_EPOCH" => {
                let (v, _) = kv_rest.parse_next(input)?;
                next_message_epoch = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid NEXT_MESSAGE_EPOCH"))?,
                );
            }
            _ => {
                // backtrack and end metadata section
                input.reset(&checkpoint);
                break;
            }
        }
    }

    Ok(RdmMetadata {
        comment,
        object_name: object_name.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT_NAME")),
            ))
        })?,
        international_designator: international_designator.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("INTERNATIONAL_DESIGNATOR")),
            ))
        })?,
        catalog_name,
        object_designator,
        object_type,
        object_owner,
        object_operator,
        controlled_reentry: controlled_reentry.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CONTROLLED_REENTRY")),
            ))
        })?,
        center_name: center_name.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CENTER_NAME")),
            ))
        })?,
        time_system: time_system.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("TIME_SYSTEM")),
            ))
        })?,
        epoch_tzero: epoch_tzero.ok_or_else(|| {
            ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("EPOCH_TZERO")),
            ))
        })?,
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

    let mut state_vector_data = None;

    let mut covariance_matrix_data = None;
    let mut spacecraft_params = RdmSpacecraftParameters::default();
    let mut have_sp = false;

    let mut od_params = OdParameters::default();
    let mut have_od = false;

    let mut user_defined_parameters = Vec::new();

    loop {
        comment.extend(collect_comments.parse_next(input)?);

        let checkpoint = input.checkpoint();
        let key = match key_token.parse_next(input) {
            Ok(k) => k,
            Err(_) => break,
        };

        match key {
            "EPOCH" => {
                input.reset(&checkpoint);
                let (sv_comment, sv) = crate::kvn::parser::state_vector.parse_next(input)?;
                let mut sv = sv;
                sv.comment = sv_comment;
                state_vector_data = Some(sv);
            }
            "COV_REF_FRAME" | "CX_X" | "CY_X" | "CY_Y" | "CZ_X" | "CZ_Y" | "CZ_Z" | "CX_DOT_X"
            | "CX_DOT_Y" | "CX_DOT_Z" | "CX_DOT_X_DOT" | "CY_DOT_X" | "CY_DOT_Y" | "CY_DOT_Z"
            | "CY_DOT_X_DOT" | "CY_DOT_Y_DOT" | "CZ_DOT_X" | "CZ_DOT_Y" | "CZ_DOT_Z"
            | "CZ_DOT_X_DOT" | "CZ_DOT_Y_DOT" | "CZ_DOT_Z_DOT" => {
                input.reset(&checkpoint);
                covariance_matrix_data = crate::kvn::parser::covariance_matrix.parse_next(input)?;
            }
            "ORBIT_LIFETIME" => {
                let (v, u) = kv_rest.parse_next(input)?;
                orbit_lifetime = Some(
                    DayIntervalRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid ORBIT_LIFETIME"))?,
                );
            }
            "REENTRY_ALTITUDE" => {
                let (v, u) = kv_rest.parse_next(input)?;
                reentry_altitude = Some(
                    PositionRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid REENTRY_ALTITUDE"))?,
                );
            }
            "ORBIT_LIFETIME_WINDOW_START" => {
                let (v, u) = kv_rest.parse_next(input)?;
                orbit_lifetime_window_start = Some(
                    DayIntervalRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid ORBIT_LIFETIME_WINDOW_START"))?,
                );
            }
            "ORBIT_LIFETIME_WINDOW_END" => {
                let (v, u) = kv_rest.parse_next(input)?;
                orbit_lifetime_window_end = Some(
                    DayIntervalRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid ORBIT_LIFETIME_WINDOW_END"))?,
                );
            }
            "NOMINAL_REENTRY_EPOCH" => {
                let (v, _) = kv_rest.parse_next(input)?;
                nominal_reentry_epoch = Some(
                    Epoch::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid NOMINAL_REENTRY_EPOCH"))?,
                );
            }
            "REENTRY_WINDOW_START" => {
                let (v, _) = kv_rest.parse_next(input)?;
                reentry_window_start = Some(
                    Epoch::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid REENTRY_WINDOW_START"))?,
                );
            }
            "REENTRY_WINDOW_END" => {
                let (v, _) = kv_rest.parse_next(input)?;
                reentry_window_end = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid REENTRY_WINDOW_END"))?,
                );
            }
            "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => {
                let (v, u) = kv_rest.parse_next(input)?;
                orbit_lifetime_confidence_level = Some(
                    PercentageRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid ORBIT_LIFETIME_CONFIDENCE_LEVEL"))?,
                );
            }
            "PROBABILITY_OF_IMPACT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.probability_of_impact = Some(
                    Probability::new(
                        parse_f64(v)
                            .map_err(|_| cut_err(input, "Invalid PROBABILITY_OF_IMPACT"))?,
                    )
                    .map_err(|_| {
                        cut_err(input, "Probability out of range for PROBABILITY_OF_IMPACT")
                    })?,
                );
                have_ground = true;
            }
            "PROBABILITY_OF_BURN_UP" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.probability_of_burn_up = Some(
                    Probability::new(
                        parse_f64(v)
                            .map_err(|_| cut_err(input, "Invalid PROBABILITY_OF_BURN_UP"))?,
                    )
                    .map_err(|_| {
                        cut_err(input, "Probability out of range for PROBABILITY_OF_BURN_UP")
                    })?,
                );
                have_ground = true;
            }
            "PROBABILITY_OF_BREAK_UP" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.probability_of_break_up = Some(
                    Probability::new(
                        parse_f64(v)
                            .map_err(|_| cut_err(input, "Invalid PROBABILITY_OF_BREAK_UP"))?,
                    )
                    .map_err(|_| {
                        cut_err(
                            input,
                            "Probability out of range for PROBABILITY_OF_BREAK_UP",
                        )
                    })?,
                );
                have_ground = true;
            }
            "PROBABILITY_OF_LAND_IMPACT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.probability_of_land_impact = Some(
                    Probability::new(
                        parse_f64(v)
                            .map_err(|_| cut_err(input, "Invalid PROBABILITY_OF_LAND_IMPACT"))?,
                    )
                    .map_err(|_| {
                        cut_err(
                            input,
                            "Probability out of range for PROBABILITY_OF_LAND_IMPACT",
                        )
                    })?,
                );
                have_ground = true;
            }
            "PROBABILITY_OF_CASUALTY" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.probability_of_casualty = Some(
                    Probability::new(
                        parse_f64(v)
                            .map_err(|_| cut_err(input, "Invalid PROBABILITY_OF_CASUALTY"))?,
                    )
                    .map_err(|_| {
                        cut_err(
                            input,
                            "Probability out of range for PROBABILITY_OF_CASUALTY",
                        )
                    })?,
                );
                have_ground = true;
            }
            "NOMINAL_IMPACT_EPOCH" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.nominal_impact_epoch = Some(
                    Epoch::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid NOMINAL_IMPACT_EPOCH"))?,
                );
                have_ground = true;
            }
            "IMPACT_WINDOW_START" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_window_start = Some(
                    Epoch::from_str(v)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_WINDOW_START"))?,
                );
                have_ground = true;
            }
            "IMPACT_WINDOW_END" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_window_end = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid IMPACT_WINDOW_END"))?,
                );
                have_ground = true;
            }
            "IMPACT_REF_FRAME" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_ref_frame = Some(v.to_string());
                have_ground = true;
            }
            "NOMINAL_IMPACT_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.nominal_impact_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid NOMINAL_IMPACT_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for NOMINAL_IMPACT_LON"))?,
                );
                have_ground = true;
            }
            "NOMINAL_IMPACT_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.nominal_impact_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid NOMINAL_IMPACT_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for NOMINAL_IMPACT_LAT"))?,
                );
                have_ground = true;
            }
            "NOMINAL_IMPACT_ALT" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.nominal_impact_alt = Some(
                    AltitudeRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid NOMINAL_IMPACT_ALT"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_CONFIDENCE" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_1_confidence = Some(
                    PercentageRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_1_CONFIDENCE"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_START_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_1_start_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_1_START_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_1_START_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_START_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_1_start_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_1_START_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_1_START_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_STOP_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_1_stop_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_1_STOP_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_1_STOP_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_STOP_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_1_stop_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_1_STOP_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_1_STOP_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_1_CROSS_TRACK" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_1_cross_track = Some(
                    Distance::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_1_CROSS_TRACK"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_CONFIDENCE" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_2_confidence = Some(
                    PercentageRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_2_CONFIDENCE"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_START_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_2_start_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_2_START_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_2_START_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_START_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_2_start_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_2_START_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_2_START_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_STOP_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_2_stop_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_2_STOP_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_2_STOP_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_STOP_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_2_stop_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_2_STOP_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_2_STOP_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_2_CROSS_TRACK" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_2_cross_track = Some(
                    Distance::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_2_CROSS_TRACK"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_CONFIDENCE" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_3_confidence = Some(
                    PercentageRequired::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_3_CONFIDENCE"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_START_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_3_start_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_3_START_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_3_START_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_START_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_3_start_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_3_START_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_3_START_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_STOP_LON" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_3_stop_lon = Some(
                    LongitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_3_STOP_LON"))?,
                    )
                    .map_err(|_| cut_err(input, "Longitude out of range for IMPACT_3_STOP_LON"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_STOP_LAT" => {
                let (v, _) = kv_rest.parse_next(input)?;
                ground_params.impact_3_stop_lat = Some(
                    LatitudeRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid IMPACT_3_STOP_LAT"))?,
                    )
                    .map_err(|_| cut_err(input, "Latitude out of range for IMPACT_3_STOP_LAT"))?,
                );
                have_ground = true;
            }
            "IMPACT_3_CROSS_TRACK" => {
                let (v, u) = kv_rest.parse_next(input)?;
                ground_params.impact_3_cross_track = Some(
                    Distance::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid IMPACT_3_CROSS_TRACK"))?,
                );
                have_ground = true;
            }

            "WET_MASS" => {
                let (v, u) = kv_rest.parse_next(input)?;
                spacecraft_params.wet_mass =
                    Some(Mass::from_kvn(v, u).map_err(|_| cut_err(input, "Invalid WET_MASS"))?);
                have_sp = true;
            }
            "DRY_MASS" => {
                let (v, u) = kv_rest.parse_next(input)?;
                spacecraft_params.dry_mass =
                    Some(Mass::from_kvn(v, u).map_err(|_| cut_err(input, "Invalid DRY_MASS"))?);
                have_sp = true;
            }
            "HAZARDOUS_SUBSTANCES" => {
                let (v, _) = kv_rest.parse_next(input)?;
                spacecraft_params.hazardous_substances = Some(v.to_string());
                have_sp = true;
            }
            "SOLAR_RAD_AREA" => {
                let (v, u) = kv_rest.parse_next(input)?;
                spacecraft_params.solar_rad_area = Some(
                    Area::from_kvn(v, u).map_err(|_| cut_err(input, "Invalid SOLAR_RAD_AREA"))?,
                );
                have_sp = true;
            }
            "SOLAR_RAD_COEFF" => {
                let (v, _) = kv_rest.parse_next(input)?;
                spacecraft_params.solar_rad_coeff =
                    Some(parse_f64(v).map_err(|_| cut_err(input, "Invalid SOLAR_RAD_COEFF"))?);
                have_sp = true;
            }
            "DRAG_AREA" => {
                let (v, u) = kv_rest.parse_next(input)?;
                spacecraft_params.drag_area =
                    Some(Area::from_kvn(v, u).map_err(|_| cut_err(input, "Invalid DRAG_AREA"))?);
                have_sp = true;
            }
            "DRAG_COEFF" => {
                let (v, _) = kv_rest.parse_next(input)?;
                spacecraft_params.drag_coeff =
                    Some(parse_f64(v).map_err(|_| cut_err(input, "Invalid DRAG_COEFF"))?);
                have_sp = true;
            }
            "RCS" => {
                let (v, u) = kv_rest.parse_next(input)?;
                spacecraft_params.rcs =
                    Some(Area::from_kvn(v, u).map_err(|_| cut_err(input, "Invalid RCS"))?);
                have_sp = true;
            }
            "BALLISTIC_COEFF" => {
                let (v, _) = kv_rest.parse_next(input)?;
                spacecraft_params.ballistic_coeff = Some(
                    BallisticCoeffRequired::new(
                        parse_f64(v).map_err(|_| cut_err(input, "Invalid BALLISTIC_COEFF"))?,
                    )
                    .map_err(|_| cut_err(input, "Ballistic coefficient out of range"))?,
                );
                have_sp = true;
            }
            "THRUST_ACCELERATION" => {
                let (v, _) = kv_rest.parse_next(input)?;
                spacecraft_params.thrust_acceleration =
                    Some(Ms2Required::new(parse_f64(v).map_err(|_| {
                        cut_err(input, "Invalid THRUST_ACCELERATION")
                    })?));
                have_sp = true;
            }

            "TIME_LASTOB_START" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.time_lastob_start = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid TIME_LASTOB_START"))?,
                );
                have_od = true;
            }
            "TIME_LASTOB_END" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.time_lastob_end = Some(
                    Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid TIME_LASTOB_END"))?,
                );
                have_od = true;
            }
            "RECOMMENDED_OD_SPAN" => {
                let (v, u) = kv_rest.parse_next(input)?;
                od_params.recommended_od_span = Some(
                    DayInterval::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid RECOMMENDED_OD_SPAN"))?,
                );
                have_od = true;
            }
            "ACTUAL_OD_SPAN" => {
                let (v, u) = kv_rest.parse_next(input)?;
                od_params.actual_od_span = Some(
                    DayInterval::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid ACTUAL_OD_SPAN"))?,
                );
                have_od = true;
            }
            "OBS_AVAILABLE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.obs_available =
                    Some(parse_u32(v).map_err(|_| cut_err(input, "Invalid OBS_AVAILABLE"))?);
                have_od = true;
            }
            "OBS_USED" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.obs_used =
                    Some(parse_u32(v).map_err(|_| cut_err(input, "Invalid OBS_USED"))?);
                have_od = true;
            }
            "TRACKS_AVAILABLE" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.tracks_available =
                    Some(parse_u32(v).map_err(|_| cut_err(input, "Invalid TRACKS_AVAILABLE"))?);
                have_od = true;
            }
            "TRACKS_USED" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.tracks_used =
                    Some(parse_u32(v).map_err(|_| cut_err(input, "Invalid TRACKS_USED"))?);
                have_od = true;
            }
            "RESIDUALS_ACCEPTED" => {
                let (v, u) = kv_rest.parse_next(input)?;
                od_params.residuals_accepted = Some(
                    Percentage::from_kvn(v, u)
                        .map_err(|_| cut_err(input, "Invalid RESIDUALS_ACCEPTED"))?,
                );
                have_od = true;
            }
            "WEIGHTED_RMS" => {
                let (v, _) = kv_rest.parse_next(input)?;
                od_params.weighted_rms =
                    Some(parse_f64(v).map_err(|_| cut_err(input, "Invalid WEIGHTED_RMS"))?);
                have_od = true;
            }

            _ if key.starts_with("USER_DEFINED_") => {
                let (v, _) = kv_rest.parse_next(input)?;
                user_defined_parameters.push((key.to_string(), v.to_string()));
            }

            _ => {
                // backtrack and end data section
                input.reset(&checkpoint);
                break;
            }
        }
    }

    let atmospheric_reentry_parameters = AtmosphericReentryParameters {
        comment: Vec::new(),
        orbit_lifetime: orbit_lifetime.ok_or_else(|| cut_err(input, "Missing ORBIT_LIFETIME"))?,
        reentry_altitude: reentry_altitude
            .ok_or_else(|| cut_err(input, "Missing REENTRY_ALTITUDE"))?,
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
        state_vector: state_vector_data,
        covariance_matrix: covariance_matrix_data,
        spacecraft_parameters: if have_sp {
            Some(spacecraft_params)
        } else {
            None
        },
        od_parameters: if have_od { Some(od_params) } else { None },
        user_defined_parameters,
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
    use crate::error::CcsdsNdmError;
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
        assert!((sp.solar_rad_coeff.unwrap() - 0.0).abs() < 1e-9);
        assert!((sp.drag_coeff.unwrap() - 0.0).abs() < 1e-9);
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
        assert_eq!(od.obs_available, Some(100));
        assert_eq!(od.obs_used, Some(95));
        assert_eq!(od.tracks_available, Some(20));
        assert_eq!(od.tracks_used, Some(18));
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected creation_date"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected originator"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected message_id"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected object_name"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message
                    .to_lowercase()
                    .contains("expected international_designator"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected center_name"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected time_system"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message
                    .to_lowercase()
                    .contains("expected controlled_reentry"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
        match err {
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected epoch_tzero"))
            }
            _ => panic!("Unexpected: {:?}", err),
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
            CcsdsNdmError::KvnParse { .. } => {}
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
            CcsdsNdmError::KvnParse { .. } => {}
            _ => panic!("Unexpected: {:?}", err),
        }
    }

    #[test]
    fn test_rdm_empty_file_error() {
        let err = Rdm::from_kvn("").unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {}
            CcsdsNdmError::KvnParse { .. } => {}
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
            CcsdsNdmError::KvnParse { message, .. } => {
                assert!(message.to_lowercase().contains("expected ccsds_rdm_vers"));
            }
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
