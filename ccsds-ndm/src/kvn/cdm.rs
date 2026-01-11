// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for CDM (Conjunction Data Message).
//!
//! This module implements KVN parsing for CDM using winnow parser combinators.

use crate::common::OdParameters;
use crate::kvn::parser::*;
use crate::messages::cdm::{
    AdditionalParameters, Cdm, CdmBody, CdmData, CdmHeader, CdmMetadata, CdmSegment,
    CdmStateVector, RelativeMetadataData, RelativeStateVector,
};
use crate::types::*;
use std::str::FromStr;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helper: Check if key belongs to CDM Data section
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

fn is_header_key(key: &str) -> bool {
    matches!(
        key,
        "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_FOR" | "MESSAGE_ID"
    )
}

//----------------------------------------------------------------------
// CDM Version Parser
//----------------------------------------------------------------------

pub fn cdm_version(input: &mut &str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_CDM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// CDM Header Parser
//----------------------------------------------------------------------

pub fn cdm_header(input: &mut &str) -> ModalResult<CdmHeader> {
    let mut comment = Vec::new();
    let mut creation_date = None;
    let mut originator = None;
    let mut message_for = None;
    let mut message_id = None;

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_header_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CREATION_DATE" => {
                        creation_date = Some(
                            Epoch::from_str(v).map_err(|_e| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORIGINATOR" => originator = Some(v.to_string()),
                    "MESSAGE_FOR" => message_for = Some(v.to_string()),
                    "MESSAGE_ID" => message_id = Some(v.to_string()),
                    _ => {}
                }
            }
            _ => break,
        }
    }

    Ok(CdmHeader {
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
        message_for,
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
// Relative Metadata Parser
//----------------------------------------------------------------------

pub fn relative_metadata_data(input: &mut &str) -> ModalResult<RelativeMetadataData> {
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

    loop {
        // If we hit META_START, that belongs to a segment
        if at_block_start("META", input) {
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some("OBJECT") => break, // Start of segment
            Some(_key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "TCA" => {
                        tca = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MISS_DISTANCE" => {
                        miss_distance = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_SPEED" => {
                        relative_speed = Some(
                            Dv::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_POSITION_R" => {
                        rel_pos_r = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_POSITION_T" => {
                        rel_pos_t = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_POSITION_N" => {
                        rel_pos_n = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_VELOCITY_R" => {
                        rel_vel_r = Some(
                            Dv::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_VELOCITY_T" => {
                        rel_vel_t = Some(
                            Dv::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RELATIVE_VELOCITY_N" => {
                        rel_vel_n = Some(
                            Dv::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "START_SCREEN_PERIOD" => {
                        start_screen_period = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "STOP_SCREEN_PERIOD" => {
                        stop_screen_period = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCREEN_VOLUME_FRAME" => {
                        screen_volume_frame = Some(match v.to_uppercase().as_str() {
                            "RTN" => ScreenVolumeFrameType::Rtn,
                            "TVN" => ScreenVolumeFrameType::Tvn,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "SCREEN_VOLUME_SHAPE" => {
                        screen_volume_shape = Some(match v.to_uppercase().as_str() {
                            "ELLIPSOID" => ScreenVolumeShapeType::Ellipsoid,
                            "BOX" => ScreenVolumeShapeType::Box,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "SCREEN_VOLUME_X" => {
                        screen_volume_x = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCREEN_VOLUME_Y" => {
                        screen_volume_y = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCREEN_VOLUME_Z" => {
                        screen_volume_z = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCREEN_ENTRY_TIME" => {
                        screen_entry_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCREEN_EXIT_TIME" => {
                        screen_exit_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "COLLISION_PROBABILITY" => {
                        collision_probability = Some(
                            Probability::new(
                                parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            )
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "COLLISION_PROBABILITY_METHOD" => {
                        collision_probability_method = Some(v.to_string());
                    }
                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label("Unknown Relative Metadata key"),
                        )));
                    }
                }
            }
            None => break,
        }
    }

    let relative_state_vector = if rel_pos_r.is_some() || rel_pos_t.is_some() || rel_pos_n.is_some()
    {
        Some(RelativeStateVector {
            relative_position_r: rel_pos_r.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_POSITION_R")),
                ))
            })?,
            relative_position_t: rel_pos_t.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_POSITION_T")),
                ))
            })?,
            relative_position_n: rel_pos_n.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_POSITION_N")),
                ))
            })?,
            relative_velocity_r: rel_vel_r.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_VELOCITY_R")),
                ))
            })?,
            relative_velocity_t: rel_vel_t.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_VELOCITY_T")),
                ))
            })?,
            relative_velocity_n: rel_vel_n.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("RELATIVE_VELOCITY_N")),
                ))
            })?,
        })
    } else {
        None
    };

    Ok(RelativeMetadataData {
        comment,
        tca: tca.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("TCA")),
            ))
        })?,
        miss_distance: miss_distance.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MISS_DISTANCE")),
            ))
        })?,
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

pub fn cdm_metadata(input: &mut &str) -> ModalResult<CdmMetadata> {
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

    loop {
        if at_block_end("META", input) {
            expect_block_end("META").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_cdm_data_key(key) => break,
            Some(_key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OBJECT" => {
                        object = Some(match v.to_uppercase().as_str() {
                            "OBJECT1" => CdmObjectType::Object1,
                            "OBJECT2" => CdmObjectType::Object2,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "OBJECT_DESIGNATOR" => object_designator = Some(v.to_string()),
                    "CATALOG_NAME" => catalog_name = Some(v.to_string()),
                    "OBJECT_NAME" => object_name = Some(v.to_string()),
                    "INTERNATIONAL_DESIGNATOR" => international_designator = Some(v.to_string()),
                    "OBJECT_TYPE" => {
                        object_type = Some(match v.to_uppercase().as_str() {
                            "PAYLOAD" => ObjectDescription::Payload,
                            "ROCKET BODY" => ObjectDescription::RocketBody,
                            "DEBRIS" => ObjectDescription::Debris,
                            "UNKNOWN" => ObjectDescription::Unknown,
                            "OTHER" => ObjectDescription::Other,
                            _ => ObjectDescription::Other,
                        });
                    }
                    "OPERATOR_CONTACT_POSITION" => operator_contact_position = Some(v.to_string()),
                    "OPERATOR_ORGANIZATION" => operator_organization = Some(v.to_string()),
                    "OPERATOR_PHONE" => operator_phone = Some(v.to_string()),
                    "OPERATOR_EMAIL" => operator_email = Some(v.to_string()),
                    "EPHEMERIS_NAME" => ephemeris_name = Some(v.to_string()),
                    "COVARIANCE_METHOD" => {
                        covariance_method = Some(match v.to_uppercase().as_str() {
                            "CALCULATED" => CovarianceMethodType::Calculated,
                            "DEFAULT" => CovarianceMethodType::Default,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "MANEUVERABLE" => {
                        maneuverable = Some(match v.to_uppercase().as_str() {
                            "YES" => ManeuverableType::Yes,
                            "NO" => ManeuverableType::No,
                            "N/A" => ManeuverableType::NA,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "ORBIT_CENTER" => orbit_center = Some(v.to_string()),
                    "REF_FRAME" => {
                        ref_frame = Some(match v.to_uppercase().as_str() {
                            "EME2000" => ReferenceFrameType::Eme2000,
                            "GCRF" => ReferenceFrameType::Gcrf,
                            "ITRF" => ReferenceFrameType::Itrf,
                            _ => return Err(ErrMode::Cut(ContextError::new())),
                        });
                    }
                    "GRAVITY_MODEL" => gravity_model = Some(v.to_string()),
                    "ATMOSPHERIC_MODEL" => atmospheric_model = Some(v.to_string()),
                    "N_BODY_PERTURBATIONS" => n_body_perturbations = Some(v.to_string()),
                    "SOLAR_RAD_PRESSURE" => {
                        solar_rad_pressure = Some(if v.eq_ignore_ascii_case("YES") {
                            YesNo::Yes
                        } else {
                            YesNo::No
                        });
                    }
                    "EARTH_TIDES" => {
                        earth_tides = Some(if v.eq_ignore_ascii_case("YES") {
                            YesNo::Yes
                        } else {
                            YesNo::No
                        });
                    }
                    "INTRACK_THRUST" => {
                        intrack_thrust = Some(if v.eq_ignore_ascii_case("YES") {
                            YesNo::Yes
                        } else {
                            YesNo::No
                        });
                    }
                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label("Unknown metadata key"),
                        )));
                    }
                }
            }
            None => break,
        }
    }

    Ok(CdmMetadata {
        comment,
        object: object.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT")),
            ))
        })?,
        object_designator: object_designator.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT_DESIGNATOR")),
            ))
        })?,
        catalog_name: catalog_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CATALOG_NAME")),
            ))
        })?,
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
        object_type,
        operator_contact_position,
        operator_organization,
        operator_phone,
        operator_email,
        ephemeris_name: ephemeris_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("EPHEMERIS_NAME")),
            ))
        })?,
        covariance_method: covariance_method.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("COVARIANCE_METHOD")),
            ))
        })?,
        maneuverable: maneuverable.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MANEUVERABLE")),
            ))
        })?,
        orbit_center,
        ref_frame: ref_frame.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("REF_FRAME")),
            ))
        })?,
        gravity_model,
        atmospheric_model,
        n_body_perturbations,
        solar_rad_pressure,
        earth_tides,
        intrack_thrust,
    })
}

//----------------------------------------------------------------------
// CDM Covariance Matrix Parser
//----------------------------------------------------------------------

fn is_cdm_covariance_key(key: &str) -> bool {
    matches!(
        key,
        "CR_R"
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

pub fn cdm_covariance_matrix(
    input: &mut &str,
) -> ModalResult<crate::messages::cdm::CdmCovarianceMatrix> {
    let mut comment = Vec::new();
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

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_cdm_covariance_key(key) => {
                comment.extend(comments);
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CR_R" => {
                        cr_r = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CT_R" => {
                        ct_r = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CT_T" => {
                        ct_t = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CN_R" => {
                        cn_r = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CN_T" => {
                        cn_t = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CN_N" => {
                        cn_n = Some(
                            M2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CRDOT_R" => {
                        crdot_r = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CRDOT_T" => {
                        crdot_t = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CRDOT_N" => {
                        crdot_n = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CRDOT_RDOT" => {
                        crdot_rdot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTDOT_R" => {
                        ctdot_r = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTDOT_T" => {
                        ctdot_t = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTDOT_N" => {
                        ctdot_n = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTDOT_RDOT" => {
                        ctdot_rdot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTDOT_TDOT" => {
                        ctdot_tdot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_R" => {
                        cndot_r = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_T" => {
                        cndot_t = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_N" => {
                        cndot_n = Some(
                            M2s::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_RDOT" => {
                        cndot_rdot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_TDOT" => {
                        cndot_tdot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CNDOT_NDOT" => {
                        cndot_ndot = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }

                    "CDRG_R" => {
                        cdrg_r = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_T" => {
                        cdrg_t = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_N" => {
                        cdrg_n = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_RDOT" => {
                        cdrg_rdot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_TDOT" => {
                        cdrg_tdot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_NDOT" => {
                        cdrg_ndot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CDRG_DRG" => {
                        cdrg_drg = Some(
                            M4kg2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }

                    "CSRP_R" => {
                        csrp_r = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_T" => {
                        csrp_t = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_N" => {
                        csrp_n = Some(
                            M3kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_RDOT" => {
                        csrp_rdot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_TDOT" => {
                        csrp_tdot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_NDOT" => {
                        csrp_ndot = Some(
                            M3kgs::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_DRG" => {
                        csrp_drg = Some(
                            M4kg2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CSRP_SRP" => {
                        csrp_srp = Some(
                            M4kg2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }

                    "CTHR_R" => {
                        cthr_r = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_T" => {
                        cthr_t = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_N" => {
                        cthr_n = Some(
                            M2s2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_RDOT" => {
                        cthr_rdot = Some(
                            M2s3::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_TDOT" => {
                        cthr_tdot = Some(
                            M2s3::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_NDOT" => {
                        cthr_ndot = Some(
                            M2s3::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_DRG" => {
                        cthr_drg = Some(
                            M3kgs2::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_SRP" => {
                        cthr_srp = Some(
                            M3kgs2::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "CTHR_THR" => {
                        cthr_thr = Some(
                            M2s4::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }
    }

    Ok(crate::messages::cdm::CdmCovarianceMatrix {
        comment,
        cr_r: cr_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CR_R")),
            ))
        })?,
        ct_r: ct_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CT_R")),
            ))
        })?,
        ct_t: ct_t.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CT_T")),
            ))
        })?,
        cn_r: cn_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CN_R")),
            ))
        })?,
        cn_t: cn_t.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CN_T")),
            ))
        })?,
        cn_n: cn_n.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CN_N")),
            ))
        })?,
        crdot_r: crdot_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CRDOT_R")),
            ))
        })?,
        crdot_t: crdot_t.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CRDOT_T")),
            ))
        })?,
        crdot_n: crdot_n.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CRDOT_N")),
            ))
        })?,
        crdot_rdot: crdot_rdot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CRDOT_RDOT")),
            ))
        })?,
        ctdot_r: ctdot_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CTDOT_R")),
            ))
        })?,
        ctdot_t: ctdot_t.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CTDOT_T")),
            ))
        })?,
        ctdot_n: ctdot_n.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CTDOT_N")),
            ))
        })?,
        ctdot_rdot: ctdot_rdot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CTDOT_RDOT")),
            ))
        })?,
        ctdot_tdot: ctdot_tdot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CTDOT_TDOT")),
            ))
        })?,
        cndot_r: cndot_r.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_R")),
            ))
        })?,
        cndot_t: cndot_t.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_T")),
            ))
        })?,
        cndot_n: cndot_n.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_N")),
            ))
        })?,
        cndot_rdot: cndot_rdot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_RDOT")),
            ))
        })?,
        cndot_tdot: cndot_tdot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_TDOT")),
            ))
        })?,
        cndot_ndot: cndot_ndot.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CNDOT_NDOT")),
            ))
        })?,
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
}

//----------------------------------------------------------------------
// CDM Data Parser
//----------------------------------------------------------------------

pub fn cdm_data(input: &mut &str) -> ModalResult<CdmData> {
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

    let mut covariance_matrix_val = None;

    loop {
        // If we hit META block for NEXT segment, stop
        if at_block_start("META", input) {
            break;
        }

        let comments = collect_comments.parse_next(input)?;

        let next_key = peek_key(input)?;

        match next_key {
            Some("OBJECT") => {
                comment.extend(comments);
                break; // Start of next segment
            }
            Some(key) => {
                comment.extend(comments);
                if is_cdm_covariance_key(key) {
                    covariance_matrix_val = Some(cdm_covariance_matrix.parse_next(input)?);
                    continue;
                }

                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    // OD Parameters
                    "TIME_LASTOB_START" => {
                        od_params.time_lastob_start = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_od_params = true;
                    }
                    "TIME_LASTOB_END" => {
                        od_params.time_lastob_end = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_od_params = true;
                    }
                    "RECOMMENDED_OD_SPAN" => {
                        od_params.recommended_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_od_params = true;
                    }
                    "ACTUAL_OD_SPAN" => {
                        od_params.actual_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_od_params = true;
                    }
                    "OBS_AVAILABLE" => {
                        od_params.obs_available =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        has_od_params = true;
                    }
                    "OBS_USED" => {
                        od_params.obs_used =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        has_od_params = true;
                    }
                    "TRACKS_AVAILABLE" => {
                        od_params.tracks_available =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        has_od_params = true;
                    }
                    "TRACKS_USED" => {
                        od_params.tracks_used =
                            Some(parse_u32(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        has_od_params = true;
                    }
                    "RESIDUALS_ACCEPTED" => {
                        od_params.residuals_accepted = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_od_params = true;
                    }
                    "WEIGHTED_RMS" => {
                        od_params.weighted_rms =
                            Some(parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                        has_od_params = true;
                    }

                    // Additional Parameters
                    "AREA_PC" => {
                        add_params.area_pc = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "AREA_DRG" => {
                        add_params.area_drg = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "AREA_SRP" => {
                        add_params.area_srp = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "MASS" => {
                        add_params.mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "CD_AREA_OVER_MASS" => {
                        add_params.cd_area_over_mass = Some(
                            M2kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "CR_AREA_OVER_MASS" => {
                        add_params.cr_area_over_mass = Some(
                            M2kg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "THRUST_ACCELERATION" => {
                        add_params.thrust_acceleration = Some(
                            Ms2::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }
                    "SEDR" => {
                        add_params.sedr = Some(
                            Wkg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                        has_add_params = true;
                    }

                    // State Vector
                    "X" => {
                        x = Some(PositionRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: PositionUnits::Km,
                        });
                    }
                    "Y" => {
                        y = Some(PositionRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: PositionUnits::Km,
                        });
                    }
                    "Z" => {
                        z = Some(PositionRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: PositionUnits::Km,
                        });
                    }
                    "X_DOT" => {
                        x_dot = Some(VelocityRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: VelocityUnits::KmPerS,
                        });
                    }
                    "Y_DOT" => {
                        y_dot = Some(VelocityRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: VelocityUnits::KmPerS,
                        });
                    }
                    "Z_DOT" => {
                        z_dot = Some(VelocityRequired {
                            value: parse_f64(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                            units: VelocityUnits::KmPerS,
                        });
                    }

                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label("Unknown Data key"),
                        )));
                    }
                }
            }
            _ => {
                comment.extend(comments);
                break;
            }
        }
    }

    Ok(CdmData {
        comment,
        od_parameters: if has_od_params { Some(od_params) } else { None },
        additional_parameters: if has_add_params {
            Some(add_params)
        } else {
            None
        },
        state_vector: CdmStateVector {
            x: x.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("X")),
                ))
            })?,
            y: y.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("Y")),
                ))
            })?,
            z: z.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("Z")),
                ))
            })?,
            x_dot: x_dot.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("X_DOT")),
                ))
            })?,
            y_dot: y_dot.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("Y_DOT")),
                ))
            })?,
            z_dot: z_dot.ok_or_else(|| {
                ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("Z_DOT")),
                ))
            })?,
        },
        covariance_matrix: covariance_matrix_val.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("Covariance Matrix keys")),
            ))
        })?,
    })
}

//----------------------------------------------------------------------
// CDM Segment Parser
//----------------------------------------------------------------------

pub fn cdm_segment(input: &mut &str) -> ModalResult<CdmSegment> {
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

pub fn cdm_body(input: &mut &str) -> ModalResult<CdmBody> {
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

pub fn parse_cdm(input: &mut &str) -> ModalResult<Cdm> {
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
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        parse_cdm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const CDM_SAMPLE: &str = r#"CCSDS_CDM_VERS = 1.0
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

OBJECT = OBJECT1
OBJECT_DESIGNATOR = 12345
CATALOG_NAME = SATCAT
OBJECT_NAME = SAT A
INTERNATIONAL_DESIGNATOR = 1998-067A
EPHEMERIS_NAME = EPH
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
EPHEMERIS_NAME = EPH
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
"#;

    #[test]
    fn test_parse_cdm() {
        let result = Cdm::from_kvn_str(CDM_SAMPLE);
        assert!(result.is_ok(), "Failed to parse CDM: {:?}", result.err());

        let cdm = result.unwrap();
        assert_eq!(cdm.version, "1.0");
        assert_eq!(cdm.header.originator, "JSPOC");
        assert_eq!(cdm.body.segments.len(), 2);
        assert_eq!(cdm.body.segments[0].metadata.object_name, "SAT A");
        assert_eq!(cdm.body.segments[1].metadata.object_name, "SAT B");
        assert_eq!(cdm.body.relative_metadata_data.miss_distance.value, 123.4);
    }
}
