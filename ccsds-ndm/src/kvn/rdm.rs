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
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helpers
//----------------------------------------------------------------------

fn is_rdm_data_keyword(key: &str) -> bool {
    matches!(
        key,
        "ORBIT_LIFETIME"
            | "REENTRY_ALTITUDE"
            | "ORBIT_LIFETIME_WINDOW_START"
            | "ORBIT_LIFETIME_WINDOW_END"
            | "NOMINAL_REENTRY_EPOCH"
            | "REENTRY_WINDOW_START"
            | "REENTRY_WINDOW_END"
            | "ORBIT_LIFETIME_CONFIDENCE_LEVEL"
            | "PROBABILITY_OF_IMPACT"
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
            | "IMPACT_3_CROSS_TRACK"
            | "EPOCH"
            | "X"
            | "Y"
            | "Z"
            | "X_DOT"
            | "Y_DOT"
            | "Z_DOT"
            | "COV_REF_FRAME"
            | "CX_X"
            | "CY_X"
            | "CY_Y"
            | "CZ_X"
            | "CZ_Y"
            | "CZ_Z"
            | "CX_DOT_X"
            | "CX_DOT_Y"
            | "CX_DOT_Z"
            | "CX_DOT_X_DOT"
            | "CY_DOT_X"
            | "CY_DOT_Y"
            | "CY_DOT_Z"
            | "CY_DOT_X_DOT"
            | "CY_DOT_Y_DOT"
            | "CZ_DOT_X"
            | "CZ_DOT_Y"
            | "CZ_DOT_Z"
            | "CZ_DOT_X_DOT"
            | "CZ_DOT_Y_DOT"
            | "CZ_DOT_Z_DOT"
            | "WET_MASS"
            | "DRY_MASS"
            | "HAZARDOUS_SUBSTANCES"
            | "SOLAR_RAD_AREA"
            | "SOLAR_RAD_COEFF"
            | "DRAG_AREA"
            | "DRAG_COEFF"
            | "RCS"
            | "BALLISTIC_COEFF"
            | "THRUST_ACCELERATION"
            | "TIME_LASTOB_START"
            | "TIME_LASTOB_END"
            | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN"
            | "OBS_AVAILABLE"
            | "OBS_USED"
            | "TRACKS_AVAILABLE"
            | "TRACKS_USED"
            | "RESIDUALS_ACCEPTED"
            | "WEIGHTED_RMS"
    ) || key.starts_with("USER_DEFINED_")
}

fn is_rdm_header_key(key: &str) -> bool {
    matches!(key, "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID")
}

//----------------------------------------------------------------------
// RDM Version Parser
//----------------------------------------------------------------------

pub fn rdm_version(input: &mut &str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_RDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// RDM Header Parser
//----------------------------------------------------------------------

pub fn rdm_header(input: &mut &str) -> ModalResult<RdmHeader> {
    let mut comment = Vec::new();
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_rdm_header_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CREATION_DATE" => {
                        creation_date = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
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

    Ok(RdmHeader {
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
        message_id: message_id.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
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

pub fn rdm_metadata(input: &mut &str) -> ModalResult<RdmMetadata> {
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
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_rdm_data_keyword(key) => break,
            Some(_key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OBJECT_NAME" => object_name = Some(v.to_string()),
                    "INTERNATIONAL_DESIGNATOR" => international_designator = Some(v.to_string()),
                    "CATALOG_NAME" => catalog_name = Some(v.to_string()),
                    "OBJECT_DESIGNATOR" => object_designator = Some(v.to_string()),
                    "OBJECT_TYPE" => {
                        object_type = Some(
                            ObjectDescription::from_str(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OBJECT_OWNER" => object_owner = Some(v.to_string()),
                    "OBJECT_OPERATOR" => object_operator = Some(v.to_string()),
                    "CONTROLLED_REENTRY" => {
                        controlled_reentry = Some(
                            ControlledType::from_str(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "CENTER_NAME" => center_name = Some(v.to_string()),
                    "TIME_SYSTEM" => time_system = Some(v.to_string()),
                    "EPOCH_TZERO" => {
                        epoch_tzero = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "REF_FRAME" => ref_frame = Some(v.to_string()),
                    "REF_FRAME_EPOCH" => {
                        ref_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "EPHEMERIS_NAME" => ephemeris_name = Some(v.to_string()),
                    "GRAVITY_MODEL" => gravity_model = Some(v.to_string()),
                    "ATMOSPHERIC_MODEL" => atmospheric_model = Some(v.to_string()),
                    "SOLAR_FLUX_PREDICTION" => solar_flux_prediction = Some(v.to_string()),
                    "N_BODY_PERTURBATIONS" => n_body_perturbations = Some(v.to_string()),
                    "SOLAR_RAD_PRESSURE" => solar_rad_pressure = Some(v.to_string()),
                    "EARTH_TIDES" => earth_tides = Some(v.to_string()),
                    "INTRACK_THRUST" => {
                        intrack_thrust = Some(
                            YesNo::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DRAG_PARAMETERS_SOURCE" => drag_parameters_source = Some(v.to_string()),
                    "DRAG_PARAMETERS_ALTITUDE" => {
                        drag_parameters_altitude = Some(
                            PositionRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "REENTRY_UNCERTAINTY_METHOD" => {
                        reentry_uncertainty_method = Some(v.to_string())
                    }
                    "REENTRY_DISINTEGRATION" => reentry_disintegration = Some(v.to_string()),
                    "IMPACT_UNCERTAINTY_METHOD" => impact_uncertainty_method = Some(v.to_string()),
                    "PREVIOUS_MESSAGE_ID" => previous_message_id = Some(v.to_string()),
                    "PREVIOUS_MESSAGE_EPOCH" => {
                        previous_message_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "NEXT_MESSAGE_EPOCH" => {
                        next_message_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label(format!("Unexpected RDM Metadata key: {}", k).leak()),
                        )));
                    }
                }
            }
            None => break,
        }
    }

    Ok(RdmMetadata {
        comment,
        object_name: object_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT_NAME")),
            ))
        })?,
        international_designator: international_designator.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
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
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CONTROLLED_REENTRY")),
            ))
        })?,
        center_name: center_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CENTER_NAME")),
            ))
        })?,
        time_system: time_system.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("TIME_SYSTEM")),
            ))
        })?,
        epoch_tzero: epoch_tzero.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
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

pub fn rdm_data(input: &mut &str) -> ModalResult<RdmData> {
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
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_rdm_data_keyword(key) => {
                match key {
                    "EPOCH" | "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT" => {
                        let (sv_comment, sv) =
                            crate::kvn::parser::state_vector.parse_next(input)?;
                        let mut sv = sv;
                        sv.comment = sv_comment;
                        state_vector_data = Some(sv);
                        continue;
                    }
                    "COV_REF_FRAME" | "CX_X" | "CY_X" | "CY_Y" | "CZ_X" | "CZ_Y" | "CZ_Z"
                    | "CX_DOT_X" | "CX_DOT_Y" | "CX_DOT_Z" | "CX_DOT_X_DOT" | "CY_DOT_X"
                    | "CY_DOT_Y" | "CY_DOT_Z" | "CY_DOT_X_DOT" | "CY_DOT_Y_DOT" | "CZ_DOT_X"
                    | "CZ_DOT_Y" | "CZ_DOT_Z" | "CZ_DOT_X_DOT" | "CZ_DOT_Y_DOT"
                    | "CZ_DOT_Z_DOT" => {
                        covariance_matrix_data =
                            crate::kvn::parser::covariance_matrix.parse_next(input)?;
                        continue;
                    }
                    _ => {}
                }

                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    // Atmospheric
                    "ORBIT_LIFETIME" => {
                        orbit_lifetime = Some(
                            DayIntervalRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "REENTRY_ALTITUDE" => {
                        reentry_altitude = Some(
                            PositionRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORBIT_LIFETIME_WINDOW_START" => {
                        orbit_lifetime_window_start = Some(
                            DayIntervalRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORBIT_LIFETIME_WINDOW_END" => {
                        orbit_lifetime_window_end = Some(
                            DayIntervalRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "NOMINAL_REENTRY_EPOCH" => {
                        nominal_reentry_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "REENTRY_WINDOW_START" => {
                        reentry_window_start = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "REENTRY_WINDOW_END" => {
                        reentry_window_end = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORBIT_LIFETIME_CONFIDENCE_LEVEL" => {
                        orbit_lifetime_confidence_level = Some(
                            PercentageRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }

                    // Ground Impact
                    "PROBABILITY_OF_IMPACT" => {
                        ground_params.probability_of_impact = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "PROBABILITY_OF_BURN_UP" => {
                        ground_params.probability_of_burn_up = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "PROBABILITY_OF_BREAK_UP" => {
                        ground_params.probability_of_break_up = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "PROBABILITY_OF_LAND_IMPACT" => {
                        ground_params.probability_of_land_impact = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "PROBABILITY_OF_CASUALTY" => {
                        ground_params.probability_of_casualty = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "NOMINAL_IMPACT_EPOCH" => {
                        ground_params.nominal_impact_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_WINDOW_START" => {
                        ground_params.impact_window_start = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_WINDOW_END" => {
                        ground_params.impact_window_end = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_REF_FRAME" => {
                        ground_params.impact_ref_frame = Some(v.to_string());
                        have_ground = true;
                    }
                    "NOMINAL_IMPACT_LON" => {
                        ground_params.nominal_impact_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "NOMINAL_IMPACT_LAT" => {
                        ground_params.nominal_impact_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "NOMINAL_IMPACT_ALT" => {
                        ground_params.nominal_impact_alt = Some(
                            AltitudeRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_CONFIDENCE" => {
                        ground_params.impact_1_confidence = Some(
                            PercentageRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_START_LON" => {
                        ground_params.impact_1_start_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_START_LAT" => {
                        ground_params.impact_1_start_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_STOP_LON" => {
                        ground_params.impact_1_stop_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_STOP_LAT" => {
                        ground_params.impact_1_stop_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_1_CROSS_TRACK" => {
                        ground_params.impact_1_cross_track = Some(
                            Distance::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    // ... and so on for IMPACT_2 and IMPACT_3 ...
                    "IMPACT_2_CONFIDENCE" => {
                        ground_params.impact_2_confidence = Some(
                            PercentageRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_2_START_LON" => {
                        ground_params.impact_2_start_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_2_START_LAT" => {
                        ground_params.impact_2_start_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_2_STOP_LON" => {
                        ground_params.impact_2_stop_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_2_STOP_LAT" => {
                        ground_params.impact_2_stop_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_2_CROSS_TRACK" => {
                        ground_params.impact_2_cross_track = Some(
                            Distance::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_CONFIDENCE" => {
                        ground_params.impact_3_confidence = Some(
                            PercentageRequired::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_START_LON" => {
                        ground_params.impact_3_start_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_START_LAT" => {
                        ground_params.impact_3_start_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_STOP_LON" => {
                        ground_params.impact_3_stop_lon = Some(
                            LongitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_STOP_LAT" => {
                        ground_params.impact_3_stop_lat = Some(
                            LatitudeRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }
                    "IMPACT_3_CROSS_TRACK" => {
                        ground_params.impact_3_cross_track = Some(
                            Distance::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_ground = true;
                    }

                    // Spacecraft Parameters
                    "WET_MASS" => {
                        spacecraft_params.wet_mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "DRY_MASS" => {
                        spacecraft_params.dry_mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "HAZARDOUS_SUBSTANCES" => {
                        spacecraft_params.hazardous_substances = Some(v.to_string());
                        have_sp = true;
                    }
                    "SOLAR_RAD_AREA" => {
                        spacecraft_params.solar_rad_area = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "SOLAR_RAD_COEFF" => {
                        spacecraft_params.solar_rad_coeff =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_sp = true;
                    }
                    "DRAG_AREA" => {
                        spacecraft_params.drag_area = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "DRAG_COEFF" => {
                        spacecraft_params.drag_coeff =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_sp = true;
                    }
                    "RCS" => {
                        spacecraft_params.rcs = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "BALLISTIC_COEFF" => {
                        spacecraft_params.ballistic_coeff = Some(
                            BallisticCoeffRequired::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_sp = true;
                    }
                    "THRUST_ACCELERATION" => {
                        spacecraft_params.thrust_acceleration = Some(Ms2Required::new(
                            parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        ));
                        have_sp = true;
                    }

                    // OD Parameters
                    "TIME_LASTOB_START" => {
                        od_params.time_lastob_start = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_od = true;
                    }
                    "TIME_LASTOB_END" => {
                        od_params.time_lastob_end = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_od = true;
                    }
                    "RECOMMENDED_OD_SPAN" => {
                        od_params.recommended_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_od = true;
                    }
                    "ACTUAL_OD_SPAN" => {
                        od_params.actual_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_od = true;
                    }
                    "OBS_AVAILABLE" => {
                        od_params.obs_available =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_od = true;
                    }
                    "OBS_USED" => {
                        od_params.obs_used =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_od = true;
                    }
                    "TRACKS_AVAILABLE" => {
                        od_params.tracks_available =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_od = true;
                    }
                    "TRACKS_USED" => {
                        od_params.tracks_used =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_od = true;
                    }
                    "RESIDUALS_ACCEPTED" => {
                        od_params.residuals_accepted = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        have_od = true;
                    }
                    "WEIGHTED_RMS" => {
                        od_params.weighted_rms =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        have_od = true;
                    }

                    _ if k.starts_with("USER_DEFINED_") => {
                        user_defined_parameters.push((k.to_string(), v.to_string()));
                    }

                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label(format!("Unexpected RDM Data key: {}", k).leak()),
                        )));
                    }
                }
            }
            _ => break,
        }
    }

    let atmospheric_reentry_parameters = AtmosphericReentryParameters {
        comment: Vec::new(),
        orbit_lifetime: orbit_lifetime.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        reentry_altitude: reentry_altitude.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
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

pub fn rdm_segment(input: &mut &str) -> ModalResult<RdmSegment> {
    let metadata = rdm_metadata.parse_next(input)?;
    let data = rdm_data.parse_next(input)?;

    Ok(RdmSegment { metadata, data })
}

//----------------------------------------------------------------------
// RDM Body Parser
//----------------------------------------------------------------------

pub fn rdm_body(input: &mut &str) -> ModalResult<RdmBody> {
    let segment = rdm_segment.parse_next(input)?;
    Ok(RdmBody {
        segment: Box::new(segment),
    })
}

//----------------------------------------------------------------------
// Complete RDM Parser
//----------------------------------------------------------------------

pub fn parse_rdm(input: &mut &str) -> ModalResult<Rdm> {
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
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        parse_rdm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
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
}
