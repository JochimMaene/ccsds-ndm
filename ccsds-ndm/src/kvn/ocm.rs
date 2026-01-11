// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OCM (Orbit Comprehensive Message).
//!
//! This module implements KVN parsing for OCM using winnow parser combinators.

use crate::kvn::parser::*;
use crate::messages::ocm::*;
use crate::traits::FromKvnValue;
use crate::types::*;
use std::str::FromStr;
use winnow::ascii::till_line_ending;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// OCM Version Parser
//----------------------------------------------------------------------

pub fn ocm_version(input: &mut &str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OCM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OCM Metadata Parser
//----------------------------------------------------------------------

fn is_ocm_metadata_key(key: &str) -> bool {
    matches!(
        key,
        "OBJECT_NAME"
            | "INTERNATIONAL_DESIGNATOR"
            | "CATALOG_NAME"
            | "OBJECT_DESIGNATOR"
            | "ALTERNATE_NAMES"
            | "ORIGINATOR_POC"
            | "ORIGINATOR_POSITION"
            | "ORIGINATOR_PHONE"
            | "ORIGINATOR_EMAIL"
            | "ORIGINATOR_ADDRESS"
            | "TECH_ORG"
            | "TECH_POC"
            | "TECH_POSITION"
            | "TECH_PHONE"
            | "TECH_EMAIL"
            | "TECH_ADDRESS"
            | "PREVIOUS_MESSAGE_ID"
            | "NEXT_MESSAGE_ID"
            | "ADM_MSG_LINK"
            | "CDM_MSG_LINK"
            | "PRM_MSG_LINK"
            | "RDM_MSG_LINK"
            | "TDM_MSG_LINK"
            | "OPERATOR"
            | "OWNER"
            | "COUNTRY"
            | "CONSTELLATION"
            | "OBJECT_TYPE"
            | "TIME_SYSTEM"
            | "EPOCH_TZERO"
            | "OPS_STATUS"
            | "ORBIT_CATEGORY"
            | "OCM_DATA_ELEMENTS"
            | "SCLK_OFFSET_AT_EPOCH"
            | "SCLK_SEC_PER_SI_SEC"
            | "PREVIOUS_MESSAGE_EPOCH"
            | "NEXT_MESSAGE_EPOCH"
            | "START_TIME"
            | "STOP_TIME"
            | "TIME_SPAN"
            | "TAIMUTC_AT_TZERO"
            | "NEXT_LEAP_EPOCH"
            | "NEXT_LEAP_TAIMUTC"
            | "UT1MUTC_AT_TZERO"
            | "EOP_SOURCE"
            | "INTERP_METHOD_EOP"
            | "CELESTIAL_SOURCE"
    )
}

pub fn ocm_metadata(input: &mut &str) -> ModalResult<OcmMetadata> {
    expect_block_start("META").parse_next(input)?;

    let mut comment = Vec::new();
    let mut object_name = None;
    let mut international_designator = None;
    let mut catalog_name = None;
    let mut object_designator = None;
    let mut alternate_names = None;
    let mut originator_poc = None;
    let mut originator_position = None;
    let mut originator_phone = None;
    let mut originator_email = None;
    let mut originator_address = None;
    let mut tech_org = None;
    let mut tech_poc = None;
    let mut tech_position = None;
    let mut tech_phone = None;
    let mut tech_email = None;
    let mut tech_address = None;
    let mut previous_message_id = None;
    let mut next_message_id = None;
    let mut adm_msg_link = None;
    let mut cdm_msg_link = None;
    let mut prm_msg_link = None;
    let mut rdm_msg_link = None;
    let mut tdm_msg_link = None;
    let mut operator = None;
    let mut owner = None;
    let mut country = None;
    let mut constellation = None;
    let mut object_type = None;
    let mut time_system = None;
    let mut epoch_tzero = None;
    let mut ops_status = None;
    let mut orbit_category = None;
    let mut ocm_data_elements = None;
    let mut sclk_offset_at_epoch = None;
    let mut sclk_sec_per_si_sec = None;
    let mut previous_message_epoch = None;
    let mut next_message_epoch = None;
    let mut start_time = None;
    let mut stop_time = None;
    let mut time_span = None;
    let mut taimutc_at_tzero = None;
    let mut next_leap_epoch = None;
    let mut next_leap_taimutc = None;
    let mut ut1mutc_at_tzero = None;
    let mut eop_source = None;
    let mut interp_method_eop = None;
    let mut celestial_source = None;

    loop {
        if at_block_end("META", input) {
            expect_block_end("META").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("META", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_ocm_metadata_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OBJECT_NAME" => object_name = Some(v.to_string()),
                    "INTERNATIONAL_DESIGNATOR" => international_designator = Some(v.to_string()),
                    "CATALOG_NAME" => catalog_name = Some(v.to_string()),
                    "OBJECT_DESIGNATOR" => object_designator = Some(v.to_string()),
                    "ALTERNATE_NAMES" => alternate_names = Some(v.to_string()),
                    "ORIGINATOR_POC" => originator_poc = Some(v.to_string()),
                    "ORIGINATOR_POSITION" => originator_position = Some(v.to_string()),
                    "ORIGINATOR_PHONE" => originator_phone = Some(v.to_string()),
                    "ORIGINATOR_EMAIL" => originator_email = Some(v.to_string()),
                    "ORIGINATOR_ADDRESS" => originator_address = Some(v.to_string()),
                    "TECH_ORG" => tech_org = Some(v.to_string()),
                    "TECH_POC" => tech_poc = Some(v.to_string()),
                    "TECH_POSITION" => tech_position = Some(v.to_string()),
                    "TECH_PHONE" => tech_phone = Some(v.to_string()),
                    "TECH_EMAIL" => tech_email = Some(v.to_string()),
                    "TECH_ADDRESS" => tech_address = Some(v.to_string()),
                    "PREVIOUS_MESSAGE_ID" => previous_message_id = Some(v.to_string()),
                    "NEXT_MESSAGE_ID" => next_message_id = Some(v.to_string()),
                    "ADM_MSG_LINK" => adm_msg_link = Some(v.to_string()),
                    "CDM_MSG_LINK" => cdm_msg_link = Some(v.to_string()),
                    "PRM_MSG_LINK" => prm_msg_link = Some(v.to_string()),
                    "RDM_MSG_LINK" => rdm_msg_link = Some(v.to_string()),
                    "TDM_MSG_LINK" => tdm_msg_link = Some(v.to_string()),
                    "OPERATOR" => operator = Some(v.to_string()),
                    "OWNER" => owner = Some(v.to_string()),
                    "COUNTRY" => country = Some(v.to_string()),
                    "CONSTELLATION" => constellation = Some(v.to_string()),
                    "OBJECT_TYPE" => {
                        object_type = Some(
                            ObjectDescription::from_str(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TIME_SYSTEM" => time_system = Some(v.to_string()),
                    "EPOCH_TZERO" => {
                        epoch_tzero = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OPS_STATUS" => ops_status = Some(v.to_string()),
                    "ORBIT_CATEGORY" => orbit_category = Some(v.to_string()),
                    "OCM_DATA_ELEMENTS" => ocm_data_elements = Some(v.to_string()),
                    "SCLK_OFFSET_AT_EPOCH" => {
                        sclk_offset_at_epoch = Some(
                            TimeOffset::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SCLK_SEC_PER_SI_SEC" => {
                        sclk_sec_per_si_sec = Some(
                            Duration::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
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
                    "START_TIME" => {
                        start_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "STOP_TIME" => {
                        stop_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TIME_SPAN" => {
                        time_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TAIMUTC_AT_TZERO" => {
                        taimutc_at_tzero = Some(
                            TimeOffset::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "NEXT_LEAP_EPOCH" => {
                        next_leap_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "NEXT_LEAP_TAIMUTC" => {
                        next_leap_taimutc = Some(
                            TimeOffset::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "UT1MUTC_AT_TZERO" => {
                        ut1mutc_at_tzero = Some(
                            TimeOffset::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "EOP_SOURCE" => eop_source = Some(v.to_string()),
                    "INTERP_METHOD_EOP" => interp_method_eop = Some(v.to_string()),
                    "CELESTIAL_SOURCE" => celestial_source = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
            Some(_key) => {
                return Err(ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description(
                        "Unexpected OCM Metadata key",
                    )),
                )));
            }
            None => break,
        }
    }

    Ok(OcmMetadata {
        comment,
        object_name,
        international_designator,
        catalog_name,
        object_designator,
        alternate_names,
        originator_poc,
        originator_position,
        originator_phone,
        originator_email,
        originator_address,
        tech_org,
        tech_poc,
        tech_position,
        tech_phone,
        tech_email,
        tech_address,
        previous_message_id,
        next_message_id,
        adm_msg_link,
        cdm_msg_link,
        prm_msg_link,
        rdm_msg_link,
        tdm_msg_link,
        operator,
        owner,
        country,
        constellation,
        object_type,
        time_system: time_system.unwrap_or_else(|| "UTC".to_string()),
        epoch_tzero: epoch_tzero.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("EPOCH_TZERO")),
            ))
        })?,
        ops_status,
        orbit_category,
        ocm_data_elements,
        sclk_offset_at_epoch: sclk_offset_at_epoch.or(Some(TimeOffset {
            value: 0.0,
            units: None,
        })),
        sclk_sec_per_si_sec: sclk_sec_per_si_sec.or(Some(Duration {
            value: 1.0,
            units: None,
        })),
        previous_message_epoch,
        next_message_epoch,
        start_time,
        stop_time,
        time_span,
        taimutc_at_tzero,
        next_leap_epoch,
        next_leap_taimutc,
        ut1mutc_at_tzero,
        eop_source,
        interp_method_eop,
        celestial_source,
    })
}

//----------------------------------------------------------------------
// OCM Trajectory State Parser
//----------------------------------------------------------------------

fn is_traj_key(key: &str) -> bool {
    matches!(
        key,
        "TRAJ_ID"
            | "TRAJ_PREV_ID"
            | "TRAJ_NEXT_ID"
            | "TRAJ_BASIS"
            | "TRAJ_BASIS_ID"
            | "INTERPOLATION"
            | "INTERPOLATION_DEGREE"
            | "PROPAGATOR"
            | "CENTER_NAME"
            | "TRAJ_REF_FRAME"
            | "TRAJ_FRAME_EPOCH"
            | "USEABLE_START_TIME"
            | "USEABLE_STOP_TIME"
            | "ORB_REVNUM"
            | "ORB_REVNUM_BASIS"
            | "TRAJ_TYPE"
            | "ORB_AVERAGING"
            | "TRAJ_UNITS"
    )
}

pub fn ocm_traj_line(input: &mut &str) -> ModalResult<TrajLine> {
    let line = raw_line.parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    let mut parts = line.split_whitespace();
    let epoch = parts
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .to_string();
    let values: Vec<f64> = parts
        .map(|v| {
            v.parse::<f64>()
                .map_err(|_| ErrMode::Cut(ContextError::new()))
        })
        .collect::<Result<Vec<f64>, _>>()?;
    Ok(TrajLine { epoch, values })
}

pub fn ocm_traj_state(input: &mut &str) -> ModalResult<OcmTrajState> {
    expect_block_start("TRAJ").parse_next(input)?;

    let mut comment = Vec::new();
    let mut traj_id = None;
    let mut traj_prev_id = None;
    let mut traj_next_id = None;
    let mut traj_basis = None;
    let mut traj_basis_id = None;
    let mut interpolation = None;
    let mut interpolation_degree = None;
    let mut propagator = None;
    let mut center_name = None;
    let mut traj_ref_frame = None;
    let mut traj_frame_epoch = None;
    let mut useable_start_time = None;
    let mut useable_stop_time = None;
    let mut orb_revnum = None;
    let mut orb_revnum_basis = None;
    let mut traj_type = None;
    let mut orb_averaging = None;
    let mut traj_units = None;
    let mut traj_lines = Vec::new();

    loop {
        if at_block_end("TRAJ", input) {
            expect_block_end("TRAJ").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("TRAJ", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_traj_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "TRAJ_ID" => traj_id = Some(v.to_string()),
                    "TRAJ_PREV_ID" => traj_prev_id = Some(v.to_string()),
                    "TRAJ_NEXT_ID" => traj_next_id = Some(v.to_string()),
                    "TRAJ_BASIS" => {
                        traj_basis = Some(
                            TrajBasis::from_str(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TRAJ_BASIS_ID" => traj_basis_id = Some(v.to_string()),
                    "INTERPOLATION" => interpolation = Some(v.to_string()),
                    "INTERPOLATION_DEGREE" => {
                        interpolation_degree = Some(parse_u32(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "INTERPOLATION_DEGREE",
                                )),
                            ))
                        })?);
                    }
                    "PROPAGATOR" => propagator = Some(v.to_string()),
                    "CENTER_NAME" => center_name = Some(v.to_string()),
                    "TRAJ_REF_FRAME" => traj_ref_frame = Some(v.to_string()),
                    "TRAJ_FRAME_EPOCH" => {
                        traj_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "USEABLE_START_TIME" => {
                        useable_start_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "USEABLE_STOP_TIME" => {
                        useable_stop_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ORB_REVNUM" => {
                        orb_revnum = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("ORB_REVNUM")),
                            ))
                        })?);
                    }
                    "ORB_REVNUM_BASIS" => {
                        orb_revnum_basis = Some(
                            RevNumBasis::from_str(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TRAJ_TYPE" => traj_type = Some(v.to_string()),
                    "ORB_AVERAGING" => orb_averaging = Some(v.to_string()),
                    "TRAJ_UNITS" => traj_units = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Unknown key-value pair, ignore
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => {
                // Likely a raw line
                if let Ok(line) = ocm_traj_line.parse_next(input) {
                    traj_lines.push(line);
                } else {
                    break;
                }
            }
        }
    }

    if traj_lines.is_empty() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("trajLine")),
        )));
    }

    Ok(OcmTrajState {
        comment,
        traj_id,
        traj_prev_id,
        traj_next_id,
        traj_basis,
        traj_basis_id,
        interpolation,
        interpolation_degree: interpolation_degree.or(Some(3)),
        propagator,
        center_name: center_name.unwrap_or_else(|| "EARTH".to_string()),
        traj_ref_frame: traj_ref_frame.unwrap_or_else(|| "ICRF3".to_string()),
        traj_frame_epoch,
        useable_start_time,
        useable_stop_time,
        orb_revnum,
        orb_revnum_basis: orb_revnum_basis.or(Some(RevNumBasis::Zero)),
        traj_type: traj_type.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("TRAJ_TYPE")),
            ))
        })?,
        orb_averaging: orb_averaging.or(Some("OSCULATING".to_string())),
        traj_units,
        traj_lines,
    })
}

//----------------------------------------------------------------------
// OCM Physical Description Parser
//----------------------------------------------------------------------

fn is_phys_key(key: &str) -> bool {
    matches!(
        key,
        "MANUFACTURER"
            | "BUS_MODEL"
            | "DOCKED_WITH"
            | "DRAG_CONST_AREA"
            | "DRAG_COEFF_NOM"
            | "DRAG_UNCERTAINTY"
            | "INITIAL_WET_MASS"
            | "WET_MASS"
            | "DRY_MASS"
            | "OEB_PARENT_FRAME"
            | "OEB_PARENT_FRAME_EPOCH"
            | "OEB_Q1"
            | "OEB_Q2"
            | "OEB_Q3"
            | "OEB_QC"
            | "OEB_MAX"
            | "OEB_INT"
            | "OEB_MIN"
            | "AREA_ALONG_OEB_MAX"
            | "AREA_ALONG_OEB_INT"
            | "AREA_ALONG_OEB_MIN"
            | "AREA_MIN_FOR_PC"
            | "AREA_MAX_FOR_PC"
            | "AREA_TYP_FOR_PC"
            | "RCS"
            | "RCS_MIN"
            | "RCS_MAX"
            | "SRP_CONST_AREA"
            | "SOLAR_RAD_COEFF"
            | "SOLAR_RAD_UNCERTAINTY"
            | "VM_ABSOLUTE"
            | "VM_APPARENT_MIN"
            | "VM_APPARENT"
            | "VM_APPARENT_MAX"
            | "REFLECTANCE"
            | "ATT_CONTROL_MODE"
            | "ATT_ACTUATOR_TYPE"
            | "ATT_KNOWLEDGE"
            | "ATT_CONTROL"
            | "ATT_POINTING"
            | "AVG_MANEUVER_FREQ"
            | "MAX_THRUST"
            | "DV_BOL"
            | "DV_REMAINING"
            | "IXX"
            | "IYY"
            | "IZZ"
            | "IXY"
            | "IXZ"
            | "IYZ"
    )
}

pub fn ocm_phys(input: &mut &str) -> ModalResult<OcmPhysicalDescription> {
    expect_block_start("PHYS").parse_next(input)?;

    let mut phys = OcmPhysicalDescription::default();

    loop {
        if at_block_end("PHYS", input) {
            expect_block_end("PHYS").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        phys.comment.extend(comments);

        if at_block_end("PHYS", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_phys_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "MANUFACTURER" => phys.manufacturer = Some(v.to_string()),
                    "BUS_MODEL" => phys.bus_model = Some(v.to_string()),
                    "DOCKED_WITH" => phys.docked_with = Some(v.to_string()),
                    "DRAG_CONST_AREA" => {
                        phys.drag_const_area = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DRAG_COEFF_NOM" => {
                        phys.drag_coeff_nom = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "DRAG_COEFF_NOM",
                                )),
                            ))
                        })?);
                    }
                    "DRAG_UNCERTAINTY" => {
                        phys.drag_uncertainty = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "INITIAL_WET_MASS" => {
                        phys.initial_wet_mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "WET_MASS" => {
                        phys.wet_mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DRY_MASS" => {
                        phys.dry_mass = Some(
                            Mass::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OEB_PARENT_FRAME" => phys.oeb_parent_frame = Some(v.to_string()),
                    "OEB_PARENT_FRAME_EPOCH" => {
                        phys.oeb_parent_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OEB_Q1" => {
                        phys.oeb_q1 = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OEB_Q1")),
                            ))
                        })?)
                    }
                    "OEB_Q2" => {
                        phys.oeb_q2 = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OEB_Q2")),
                            ))
                        })?)
                    }
                    "OEB_Q3" => {
                        phys.oeb_q3 = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OEB_Q3")),
                            ))
                        })?)
                    }
                    "OEB_QC" => {
                        phys.oeb_qc = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OEB_QC")),
                            ))
                        })?)
                    }
                    "OEB_MAX" => {
                        phys.oeb_max = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OEB_INT" => {
                        phys.oeb_int = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OEB_MIN" => {
                        phys.oeb_min = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_ALONG_OEB_MAX" => {
                        phys.area_along_oeb_max = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_ALONG_OEB_INT" => {
                        phys.area_along_oeb_int = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_ALONG_OEB_MIN" => {
                        phys.area_along_oeb_min = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_MIN_FOR_PC" => {
                        phys.area_min_for_pc = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_MAX_FOR_PC" => {
                        phys.area_max_for_pc = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AREA_TYP_FOR_PC" => {
                        phys.area_typ_for_pc = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RCS" => {
                        phys.rcs = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RCS_MIN" => {
                        phys.rcs_min = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RCS_MAX" => {
                        phys.rcs_max = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SRP_CONST_AREA" => {
                        phys.srp_const_area = Some(
                            Area::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SOLAR_RAD_COEFF" => {
                        phys.solar_rad_coeff = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "SOLAR_RAD_COEFF",
                                )),
                            ))
                        })?);
                    }
                    "SOLAR_RAD_UNCERTAINTY" => {
                        phys.solar_rad_uncertainty = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "VM_ABSOLUTE" => {
                        phys.vm_absolute = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("VM_ABSOLUTE")),
                            ))
                        })?)
                    }
                    "VM_APPARENT_MIN" => {
                        phys.vm_apparent_min = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "VM_APPARENT_MIN",
                                )),
                            ))
                        })?)
                    }
                    "VM_APPARENT" => {
                        phys.vm_apparent = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("VM_APPARENT")),
                            ))
                        })?)
                    }
                    "VM_APPARENT_MAX" => {
                        phys.vm_apparent_max = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "VM_APPARENT_MAX",
                                )),
                            ))
                        })?)
                    }
                    "REFLECTANCE" => {
                        let val = parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("REFLECTANCE")),
                            ))
                        })?;
                        phys.reflectance = Some(Probability::new(val).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("REFLECTANCE")),
                            ))
                        })?);
                    }
                    "ATT_CONTROL_MODE" => phys.att_control_mode = Some(v.to_string()),
                    "ATT_ACTUATOR_TYPE" => phys.att_actuator_type = Some(v.to_string()),
                    "ATT_KNOWLEDGE" => {
                        phys.att_knowledge = Some(
                            Angle::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ATT_CONTROL" => {
                        phys.att_control = Some(
                            Angle::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ATT_POINTING" => {
                        phys.att_pointing = Some(
                            Angle::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "AVG_MANEUVER_FREQ" => {
                        phys.avg_maneuver_freq = Some(
                            ManeuverFreq::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAX_THRUST" => {
                        phys.max_thrust = Some(
                            Thrust::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DV_BOL" => {
                        phys.dv_bol = Some(
                            Velocity::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DV_REMAINING" => {
                        phys.dv_remaining = Some(
                            Velocity::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "IXX" => {
                        phys.ixx = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "IYY" => {
                        phys.iyy = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "IZZ" => {
                        phys.izz = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "IXY" => {
                        phys.ixy = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "IXZ" => {
                        phys.ixz = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    "IYZ" => {
                        phys.iyz = Some(
                            Moment::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        )
                    }
                    _ => unreachable!(),
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Ignore unknown physical description key
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => break,
        }
    }

    Ok(phys)
}

//----------------------------------------------------------------------
// OCM Covariance Parser
//----------------------------------------------------------------------

fn is_ocm_cov_key(key: &str) -> bool {
    matches!(
        key,
        "COV_ID"
            | "COV_PREV_ID"
            | "COV_NEXT_ID"
            | "COV_BASIS"
            | "COV_BASIS_ID"
            | "COV_REF_FRAME"
            | "COV_FRAME_EPOCH"
            | "COV_SCALE_MIN"
            | "COV_SCALE_MAX"
            | "COV_CONFIDENCE"
            | "COV_TYPE"
            | "COV_ORDERING"
            | "COV_UNITS"
    ) || key.starts_with("CX_")
        || key.starts_with("CY_")
        || key.starts_with("CZ_")
}

pub fn ocm_cov_line(input: &mut &str) -> ModalResult<CovLine> {
    let line = raw_line.parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    let mut parts = line.split_whitespace();
    let epoch = parts
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .to_string();
    let values: Vec<f64> = parts
        .map(|v| {
            v.parse::<f64>()
                .map_err(|_| ErrMode::Cut(ContextError::new()))
        })
        .collect::<Result<Vec<f64>, _>>()?;
    Ok(CovLine { epoch, values })
}

pub fn ocm_cov(input: &mut &str) -> ModalResult<OcmCovarianceMatrix> {
    expect_block_start("COV").parse_next(input)?;

    let mut comment = Vec::new();
    let mut cov_id = None;
    let mut cov_prev_id = None;
    let mut cov_next_id = None;
    let mut cov_basis = None;
    let mut cov_basis_id = None;
    let mut cov_ref_frame = None;
    let mut cov_frame_epoch = None;
    let mut cov_scale_min = None;
    let mut cov_scale_max = None;
    let mut cov_confidence = None;
    let mut cov_type = None;
    let mut cov_ordering = None;
    let mut cov_units = None;
    let mut cov_lines = Vec::new();

    loop {
        if at_block_end("COV", input) {
            expect_block_end("COV").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("COV", input) {
            continue;
        }

        let checkpoint = input.checkpoint();
        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_ocm_cov_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "COV_ID" => cov_id = Some(v.to_string()),
                    "COV_PREV_ID" => cov_prev_id = Some(v.to_string()),
                    "COV_NEXT_ID" => cov_next_id = Some(v.to_string()),
                    "COV_BASIS" => {
                        cov_basis = Some(
                            CovBasis::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "COV_BASIS_ID" => cov_basis_id = Some(v.to_string()),
                    "COV_REF_FRAME" => cov_ref_frame = Some(v.to_string()),
                    "COV_FRAME_EPOCH" => {
                        cov_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "COV_TYPE" => cov_type = Some(v.to_string()),
                    "COV_UNITS" => cov_units = Some(v.to_string()),
                    "COV_ORDERING" => {
                        cov_ordering = Some(
                            CovOrder::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "COV_SCALE_MIN" => {
                        cov_scale_min = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("COV_SCALE_MIN")),
                            ))
                        })?);
                    }
                    "COV_SCALE_MAX" => {
                        cov_scale_max = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("COV_SCALE_MAX")),
                            ))
                        })?);
                    }
                    "COV_CONFIDENCE" => {
                        cov_confidence = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    _ => {}
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Ignore unknown key-value pair unless it's a block tag
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => {
                if let Ok(line) = ocm_cov_line.parse_next(input) {
                    cov_lines.push(line);
                } else {
                    input.reset(&checkpoint);
                    break;
                }
            }
        }
    }

    Ok(OcmCovarianceMatrix {
        comment,
        cov_id,
        cov_prev_id,
        cov_next_id,
        cov_basis,
        cov_basis_id,
        cov_ref_frame: cov_ref_frame.unwrap_or_else(|| "TNW_INERTIAL".to_string()),
        cov_frame_epoch,
        cov_scale_min,
        cov_scale_max,
        cov_confidence,
        cov_type: cov_type.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("COV_TYPE")),
            ))
        })?,
        cov_ordering: cov_ordering.unwrap_or(CovOrder::Ltm),
        cov_units,
        cov_lines,
    })
}

//----------------------------------------------------------------------
// OCM Maneuver Parser
//----------------------------------------------------------------------

fn is_ocm_man_key(key: &str) -> bool {
    matches!(
        key,
        "MAN_ID"
            | "MAN_PREV_ID"
            | "MAN_NEXT_ID"
            | "MAN_BASIS"
            | "MAN_BASIS_ID"
            | "MAN_DEVICE_ID"
            | "MAN_PREV_EPOCH"
            | "MAN_NEXT_EPOCH"
            | "MAN_PURPOSE"
            | "MAN_PRED_SOURCE"
            | "MAN_REF_FRAME"
            | "MAN_FRAME_EPOCH"
            | "GRAV_ASSIST_NAME"
            | "DC_TYPE"
            | "DC_WIN_OPEN"
            | "DC_WIN_CLOSE"
            | "DC_MIN_CYCLES"
            | "DC_MAX_CYCLES"
            | "DC_EXEC_START"
            | "DC_EXEC_STOP"
            | "DC_REF_TIME"
            | "DC_TIME_PULSE_DURATION"
            | "DC_TIME_PULSE_PERIOD"
            | "DC_REF_DIR"
            | "DC_BODY_FRAME"
            | "DC_BODY_TRIGGER"
            | "DC_PA_START_ANGLE"
            | "DC_PA_STOP_ANGLE"
            | "MAN_COMPOSITION"
            | "MAN_UNITS"
    )
}

pub fn ocm_man_line(input: &mut &str) -> ModalResult<ManLine> {
    let line = raw_line.parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    let mut parts = line.split_whitespace();
    let epoch = parts
        .next()
        .ok_or_else(|| ErrMode::Cut(ContextError::new()))?
        .to_string();
    let values: Vec<String> = parts.map(|s| s.to_string()).collect();
    Ok(ManLine { epoch, values })
}

pub fn ocm_man(input: &mut &str) -> ModalResult<OcmManeuverParameters> {
    expect_block_start("MAN").parse_next(input)?;

    let mut comment = Vec::new();
    let mut man_id = None;
    let mut man_prev_id = None;
    let mut man_next_id = None;
    let mut man_basis = None;
    let mut man_basis_id = None;
    let mut man_device_id = None;
    let mut man_prev_epoch = None;
    let mut man_next_epoch = None;
    let mut man_purpose = None;
    let mut man_pred_source = None;
    let mut man_ref_frame = None;
    let mut man_frame_epoch = None;
    let mut grav_assist_name = None;
    let mut dc_type = None;
    let mut dc_win_open = None;
    let mut dc_win_close = None;
    let mut dc_min_cycles = None;
    let mut dc_max_cycles = None;
    let mut dc_exec_start = None;
    let mut dc_exec_stop = None;
    let mut dc_ref_time = None;
    let mut dc_time_pulse_duration = None;
    let mut dc_time_pulse_period = None;
    let mut dc_ref_dir = None;
    let mut dc_body_frame = None;
    let mut dc_body_trigger = None;
    let mut dc_pa_start_angle = None;
    let mut dc_pa_stop_angle = None;
    let mut man_composition = None;
    let mut man_units = None;
    let mut man_lines = Vec::new();

    loop {
        if at_block_end("MAN", input) {
            expect_block_end("MAN").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("MAN", input) {
            continue;
        }

        let checkpoint = input.checkpoint();
        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_ocm_man_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "MAN_ID" => man_id = Some(v.to_string()),
                    "MAN_PREV_ID" => man_prev_id = Some(v.to_string()),
                    "MAN_NEXT_ID" => man_next_id = Some(v.to_string()),
                    "MAN_BASIS" => {
                        man_basis = Some(
                            ManBasis::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_BASIS_ID" => man_basis_id = Some(v.to_string()),
                    "MAN_DEVICE_ID" => man_device_id = Some(v.to_string()),
                    "MAN_PREV_EPOCH" => {
                        man_prev_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_NEXT_EPOCH" => {
                        man_next_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_PURPOSE" => man_purpose = Some(v.to_string()),
                    "MAN_PRED_SOURCE" => man_pred_source = Some(v.to_string()),
                    "MAN_REF_FRAME" => man_ref_frame = Some(v.to_string()),
                    "MAN_FRAME_EPOCH" => {
                        man_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "GRAV_ASSIST_NAME" => grav_assist_name = Some(v.to_string()),
                    "DC_TYPE" => {
                        dc_type = Some(
                            ManDc::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_WIN_OPEN" => {
                        dc_win_open = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_WIN_CLOSE" => {
                        dc_win_close = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_MIN_CYCLES" => {
                        dc_min_cycles = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("DC_MIN_CYCLES")),
                            ))
                        })?);
                    }
                    "DC_MAX_CYCLES" => {
                        dc_max_cycles = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("DC_MAX_CYCLES")),
                            ))
                        })?);
                    }
                    "DC_EXEC_START" => {
                        dc_exec_start = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_EXEC_STOP" => {
                        dc_exec_stop = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_REF_TIME" => {
                        dc_ref_time = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_TIME_PULSE_DURATION" => {
                        dc_time_pulse_duration = Some(
                            Duration::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_TIME_PULSE_PERIOD" => {
                        dc_time_pulse_period = Some(
                            Duration::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_REF_DIR" => {
                        dc_ref_dir = Some(
                            Vec3Double::from_kvn_value(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_BODY_FRAME" => dc_body_frame = Some(v.to_string()),
                    "DC_BODY_TRIGGER" => {
                        dc_body_trigger = Some(
                            Vec3Double::from_kvn_value(v)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_PA_START_ANGLE" => {
                        dc_pa_start_angle = Some(
                            Angle::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DC_PA_STOP_ANGLE" => {
                        dc_pa_stop_angle = Some(
                            Angle::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_COMPOSITION" => man_composition = Some(v.to_string()),
                    "MAN_UNITS" => man_units = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Ignore unknown key-value pair unless it's a block tag
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => {
                if let Ok(line) = ocm_man_line.parse_next(input) {
                    man_lines.push(line);
                } else {
                    input.reset(&checkpoint);
                    break;
                }
            }
        }
    }

    Ok(OcmManeuverParameters {
        comment,
        man_id: man_id.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MAN_ID")),
            ))
        })?,
        man_prev_id,
        man_next_id,
        man_basis,
        man_basis_id,
        man_device_id: man_device_id.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MAN_DEVICE_ID")),
            ))
        })?,
        man_prev_epoch,
        man_next_epoch,
        man_purpose,
        man_pred_source,
        man_ref_frame: man_ref_frame.unwrap_or_else(|| "TNW_INERTIAL".to_string()),
        man_frame_epoch,
        grav_assist_name,
        dc_type: dc_type.unwrap_or(ManDc::Continuous),
        dc_win_open,
        dc_win_close,
        dc_min_cycles,
        dc_max_cycles,
        dc_exec_start,
        dc_exec_stop,
        dc_ref_time,
        dc_time_pulse_duration,
        dc_time_pulse_period,
        dc_ref_dir,
        dc_body_frame,
        dc_body_trigger,
        dc_pa_start_angle,
        dc_pa_stop_angle,
        man_composition: man_composition.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MAN_COMPOSITION")),
            ))
        })?,
        man_units,
        man_lines,
    })
}

//----------------------------------------------------------------------
// OCM Perturbations Parser
//----------------------------------------------------------------------

fn is_pert_key(key: &str) -> bool {
    matches!(
        key,
        "ATMOSPHERIC_MODEL"
            | "GRAVITY_MODEL"
            | "EQUATORIAL_RADIUS"
            | "GM"
            | "N_BODY_PERTURBATIONS"
            | "CENTRAL_BODY_ROTATION"
            | "OBLATE_FLATTENING"
            | "OCEAN_TIDES_MODEL"
            | "SOLID_TIDES_MODEL"
            | "REDUCTION_THEORY"
            | "ALBEDO_MODEL"
            | "ALBEDO_GRID_SIZE"
            | "SHADOW_MODEL"
            | "SHADOW_BODIES"
            | "SRP_MODEL"
            | "SW_DATA_SOURCE"
            | "SW_DATA_EPOCH"
            | "SW_INTERP_METHOD"
            | "FIXED_GEOMAG_KP"
            | "FIXED_GEOMAG_AP"
            | "FIXED_GEOMAG_DST"
            | "FIXED_F10P7"
            | "FIXED_F10P7_MEAN"
            | "FIXED_M10P7"
            | "FIXED_M10P7_MEAN"
            | "FIXED_S10P7"
            | "FIXED_S10P7_MEAN"
            | "FIXED_Y10P7"
            | "FIXED_Y10P7_MEAN"
    )
}

pub fn ocm_pert(input: &mut &str) -> ModalResult<OcmPerturbations> {
    expect_block_start("PERT").parse_next(input)?;

    let mut pert = OcmPerturbations::default();

    loop {
        if at_block_end("PERT", input) {
            expect_block_end("PERT").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        pert.comment.extend(comments);

        if at_block_end("PERT", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_pert_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "ATMOSPHERIC_MODEL" => pert.atmospheric_model = Some(v.to_string()),
                    "GRAVITY_MODEL" => pert.gravity_model = Some(v.to_string()),
                    "EQUATORIAL_RADIUS" => {
                        pert.equatorial_radius = Some(
                            Position::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "GM" => {
                        pert.gm = Some(
                            Gm::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "N_BODY_PERTURBATIONS" => pert.n_body_perturbations = Some(v.to_string()),
                    "CENTRAL_BODY_ROTATION" => {
                        pert.central_body_rotation = Some(
                            AngleRate::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OBLATE_FLATTENING" => {
                        pert.oblate_flattening = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "OBLATE_FLATTENING",
                                )),
                            ))
                        })?);
                    }
                    "OCEAN_TIDES_MODEL" => pert.ocean_tides_model = Some(v.to_string()),
                    "SOLID_TIDES_MODEL" => pert.solid_tides_model = Some(v.to_string()),
                    "REDUCTION_THEORY" => pert.reduction_theory = Some(v.to_string()),
                    "ALBEDO_MODEL" => pert.albedo_model = Some(v.to_string()),
                    "ALBEDO_GRID_SIZE" => {
                        pert.albedo_grid_size = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "ALBEDO_GRID_SIZE",
                                )),
                            ))
                        })?);
                    }
                    "SHADOW_MODEL" => pert.shadow_model = Some(v.to_string()),
                    "SHADOW_BODIES" => pert.shadow_bodies = Some(v.to_string()),
                    "SRP_MODEL" => pert.srp_model = Some(v.to_string()),
                    "SW_DATA_SOURCE" => pert.sw_data_source = Some(v.to_string()),
                    "SW_DATA_EPOCH" => {
                        pert.sw_data_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SW_INTERP_METHOD" => pert.sw_interp_method = Some(v.to_string()),
                    "FIXED_GEOMAG_KP" => {
                        pert.fixed_geomag_kp = Some(
                            Geomag::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_GEOMAG_AP" => {
                        pert.fixed_geomag_ap = Some(
                            Geomag::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_GEOMAG_DST" => {
                        pert.fixed_geomag_dst = Some(
                            Geomag::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_F10P7" => {
                        pert.fixed_f10p7 = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_F10P7_MEAN" => {
                        pert.fixed_f10p7_mean = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_M10P7" => {
                        pert.fixed_m10p7 = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_M10P7_MEAN" => {
                        pert.fixed_m10p7_mean = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_S10P7" => {
                        pert.fixed_s10p7 = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_S10P7_MEAN" => {
                        pert.fixed_s10p7_mean = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_Y10P7" => {
                        pert.fixed_y10p7 = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "FIXED_Y10P7_MEAN" => {
                        pert.fixed_y10p7_mean = Some(
                            SolarFlux::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    _ => unreachable!(),
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Ignore unknown perturbation key
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => break,
        }
    }

    Ok(pert)
}

//----------------------------------------------------------------------
// OCM OD Parameters Parser
//----------------------------------------------------------------------

fn is_ocm_od_key(key: &str) -> bool {
    matches!(
        key,
        "OD_ID"
            | "OD_PREV_ID"
            | "OD_METHOD"
            | "OD_EPOCH"
            | "DAYS_SINCE_FIRST_OBS"
            | "DAYS_SINCE_LAST_OBS"
            | "RECOMMENDED_OD_SPAN"
            | "ACTUAL_OD_SPAN"
            | "OBS_AVAILABLE"
            | "OBS_USED"
            | "TRACKS_AVAILABLE"
            | "TRACKS_USED"
            | "MAXIMUM_OBS_GAP"
            | "OD_EPOCH_EIGMAJ"
            | "OD_EPOCH_EIGINT"
            | "OD_EPOCH_EIGMIN"
            | "OD_MAX_PRED_EIGMAJ"
            | "OD_MIN_PRED_EIGMIN"
            | "OD_CONFIDENCE"
            | "GDOP"
            | "SOLVE_N"
            | "SOLVE_STATES"
            | "CONSIDER_N"
            | "CONSIDER_PARAMS"
            | "SEDR"
            | "SENSORS_N"
            | "SENSORS"
            | "WEIGHTED_RMS"
            | "DATA_TYPES"
    )
}

pub fn ocm_od(input: &mut &str) -> ModalResult<OcmOdParameters> {
    expect_block_start("OD").parse_next(input)?;

    let mut comment = Vec::new();
    let mut od_id = None;
    let mut od_prev_id = None;
    let mut od_method = None;
    let mut od_epoch = None;
    let mut days_since_first_obs = None;
    let mut days_since_last_obs = None;
    let mut recommended_od_span = None;
    let mut actual_od_span = None;
    let mut obs_available = None;
    let mut obs_used = None;
    let mut tracks_available = None;
    let mut tracks_used = None;
    let mut maximum_obs_gap = None;
    let mut od_epoch_eigmaj = None;
    let mut od_epoch_eigint = None;
    let mut od_epoch_eigmin = None;
    let mut od_max_pred_eigmaj = None;
    let mut od_min_pred_eigmin = None;
    let mut od_confidence = None;
    let mut gdop = None;
    let mut solve_n = None;
    let mut solve_states = None;
    let mut consider_n = None;
    let mut consider_params = None;
    let mut sedr = None;
    let mut sensors_n = None;
    let mut sensors = None;
    let mut weighted_rms = None;
    let mut data_types = None;

    loop {
        if at_block_end("OD", input) {
            expect_block_end("OD").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("OD", input) {
            continue;
        }

        let next_key = peek_key(input)?;
        match next_key {
            Some(key) if is_ocm_od_key(key) => {
                let (k, v, u) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OD_ID" => od_id = Some(v.to_string()),
                    "OD_PREV_ID" => od_prev_id = Some(v.to_string()),
                    "OD_METHOD" => od_method = Some(v.to_string()),
                    "OD_EPOCH" => {
                        od_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DAYS_SINCE_FIRST_OBS" => {
                        days_since_first_obs = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DAYS_SINCE_LAST_OBS" => {
                        days_since_last_obs = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RECOMMENDED_OD_SPAN" => {
                        recommended_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ACTUAL_OD_SPAN" => {
                        actual_od_span = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OBS_AVAILABLE" => {
                        obs_available = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OBS_AVAILABLE")),
                            ))
                        })?);
                    }
                    "OBS_USED" => {
                        obs_used = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("OBS_USED")),
                            ))
                        })?);
                    }
                    "TRACKS_AVAILABLE" => {
                        tracks_available = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description(
                                    "TRACKS_AVAILABLE",
                                )),
                            ))
                        })?);
                    }
                    "TRACKS_USED" => {
                        tracks_used = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("TRACKS_USED")),
                            ))
                        })?);
                    }
                    "MAXIMUM_OBS_GAP" => {
                        maximum_obs_gap = Some(
                            DayInterval::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_EPOCH_EIGMAJ" => {
                        od_epoch_eigmaj = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_EPOCH_EIGINT" => {
                        od_epoch_eigint = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_EPOCH_EIGMIN" => {
                        od_epoch_eigmin = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_MAX_PRED_EIGMAJ" => {
                        od_max_pred_eigmaj = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_MIN_PRED_EIGMIN" => {
                        od_min_pred_eigmin = Some(
                            Length::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "OD_CONFIDENCE" => {
                        od_confidence = Some(
                            Percentage::from_kvn(v, u)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "GDOP" => {
                        gdop = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("GDOP")),
                            ))
                        })?);
                    }
                    "SOLVE_N" => {
                        solve_n = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("SOLVE_N")),
                            ))
                        })?);
                    }
                    "SOLVE_STATES" => solve_states = Some(v.to_string()),
                    "CONSIDER_N" => {
                        consider_n = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("CONSIDER_N")),
                            ))
                        })?);
                    }
                    "CONSIDER_PARAMS" => consider_params = Some(v.to_string()),
                    "SEDR" => {
                        sedr = Some(
                            Wkg::from_kvn(v, u).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SENSORS_N" => {
                        sensors_n = Some(parse_u64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("SENSORS_N")),
                            ))
                        })?);
                    }
                    "SENSORS" => sensors = Some(v.to_string()),
                    "WEIGHTED_RMS" => {
                        weighted_rms = Some(parse_f64(v).map_err(|_| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Expected(StrContextValue::Description("WEIGHTED_RMS")),
                            ))
                        })?);
                    }
                    "DATA_TYPES" => data_types = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
            Some(key) if !key.ends_with("_STOP") && !key.ends_with("_START") => {
                // Ignore unknown OD key
                let _ = till_line_ending.parse_next(input)?;
                opt_line_ending.parse_next(input)?;
            }
            _ => break,
        }
    }

    Ok(OcmOdParameters {
        comment,
        od_id: od_id.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OD_ID")),
            ))
        })?,
        od_prev_id,
        od_method: od_method.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OD_METHOD")),
            ))
        })?,
        od_epoch: od_epoch.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OD_EPOCH")),
            ))
        })?,
        days_since_first_obs,
        days_since_last_obs,
        recommended_od_span,
        actual_od_span,
        obs_available,
        obs_used,
        tracks_available,
        tracks_used,
        maximum_obs_gap,
        od_epoch_eigmaj,
        od_epoch_eigint,
        od_epoch_eigmin,
        od_max_pred_eigmaj,
        od_min_pred_eigmin,
        od_confidence,
        gdop,
        solve_n,
        solve_states,
        consider_n,
        consider_params,
        sedr,
        sensors_n,
        sensors,
        weighted_rms,
        data_types,
    })
}

//----------------------------------------------------------------------
// OCM User Parser
//----------------------------------------------------------------------

pub fn ocm_user(input: &mut &str) -> ModalResult<UserDefined> {
    expect_block_start("USER").parse_next(input)?;

    let mut comment = Vec::new();
    let mut user_defined = Vec::new();

    loop {
        if at_block_end("USER", input) {
            expect_block_end("USER").parse_next(input)?;
            break;
        }

        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("USER", input) {
            continue;
        }

        let checkpoint = input.checkpoint();
        match key_value_line.parse_next(input) {
            Ok((k, v, _)) => {
                opt_line_ending.parse_next(input)?;
                user_defined.push(UserDefinedParameter {
                    parameter: k.to_string(),
                    value: v.to_string(),
                });
            }
            Err(_) => {
                input.reset(&checkpoint);
                return Err(ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description("Unexpected in USER")),
                )));
            }
        }
    }

    Ok(UserDefined {
        comment,
        user_defined,
    })
}

//----------------------------------------------------------------------
// OCM Data Parser
//----------------------------------------------------------------------

pub fn ocm_data(input: &mut &str) -> ModalResult<OcmData> {
    let mut data = OcmData::default();
    let mut pending_comments = Vec::new();

    loop {
        let comments = collect_comments.parse_next(input)?;
        pending_comments.extend(comments);

        if input.is_empty() {
            break;
        }

        if at_block_start("TRAJ", input) {
            let mut block = ocm_traj_state.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.traj.push(block);
        } else if at_block_start("PHYS", input) {
            let mut block = ocm_phys.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.phys = Some(block);
        } else if at_block_start("COV", input) {
            let mut block = ocm_cov.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.cov.push(block);
        } else if at_block_start("MAN", input) {
            let mut block = ocm_man.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.man.push(block);
        } else if at_block_start("PERT", input) {
            let mut block = ocm_pert.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.pert = Some(block);
        } else if at_block_start("OD", input) {
            let mut block = ocm_od.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.od = Some(block);
        } else if at_block_start("USER", input) {
            let mut block = ocm_user.parse_next(input)?;
            block.comment.splice(0..0, pending_comments.drain(..));
            data.user = Some(block);
        } else {
            // Check if it's an unexpected key that isn't a block start
            // If we are here, we are not at a block start, and comments have been consumed.
            // If there is anything else than empty, it's an error.

            // However, collect_comments skips whitespace.
            // If input is not empty, it means we have something unknown.
            if !input.is_empty() {
                let checkpoint = input.checkpoint();
                let next = peek_key(input)?;
                input.reset(&checkpoint);

                if let Some(k) = next {
                    return Err(ErrMode::Cut(ContextError::new().add_context(
                        input,
                        &input.checkpoint(),
                        StrContext::Expected(StrContextValue::Description(
                            format!("Unexpected key: {}", k).leak(),
                        )),
                    )));
                } else {
                    // Could be garbage, break and let trailing checker handle it or fail
                    break;
                }
            }
            break;
        }
    }

    Ok(data)
}

//----------------------------------------------------------------------
// Complete OCM Parser
//----------------------------------------------------------------------

pub fn parse_ocm(input: &mut &str) -> ModalResult<Ocm> {
    let version = ocm_version.parse_next(input)?;
    let header = odm_header.parse_next(input)?;
    let metadata = ocm_metadata.parse_next(input)?;
    let data = ocm_data.parse_next(input)?;

    Ok(Ocm {
        header,
        body: OcmBody {
            segment: Box::new(OcmSegment { metadata, data }),
        },
        id: Some("CCSDS_OCM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Ocm {
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        if input.trim().is_empty() {
            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("Empty file")),
            )));
        }
        parse_ocm.parse_next(input)
    }
}
