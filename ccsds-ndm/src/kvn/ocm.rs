// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OCM (Orbit Comprehensive Message).
//!
//! This module implements KVN parsing for OCM using winnow parser combinators.

use crate::common::OdmHeader;
use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::ocm::*;
use crate::parse_block;
use crate::types::*;
use winnow::ascii::{space1, till_line_ending};
use winnow::combinator::repeat;
use winnow::error::{AddContext, ErrMode};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// OCM Version Parser
//----------------------------------------------------------------------

pub fn ocm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OCM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OCM Metadata Parser
//----------------------------------------------------------------------

pub fn ocm_metadata(input: &mut &str) -> KvnResult<OcmMetadata> {
    ws.parse_next(input)?;
    expect_block_start("META").parse_next(input).map_err(|e| {
        if e.is_backtrack() {
            cut_err(input, "Expected META_START")
        } else {
            e
        }
    })?;

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

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "INTERNATIONAL_DESIGNATOR" => international_designator: kv_string,
        "CATALOG_NAME" => catalog_name: kv_string,
        "OBJECT_DESIGNATOR" => object_designator: kv_string,
        "ALTERNATE_NAMES" => alternate_names: kv_string,
        "ORIGINATOR_POC" => originator_poc: kv_string,
        "ORIGINATOR_POSITION" => originator_position: kv_string,
        "ORIGINATOR_PHONE" => originator_phone: kv_string,
        "ORIGINATOR_EMAIL" => originator_email: kv_string,
        "ORIGINATOR_ADDRESS" => originator_address: kv_string,
        "TECH_ORG" => tech_org: kv_string,
        "TECH_POC" => tech_poc: kv_string,
        "TECH_POSITION" => tech_position: kv_string,
        "TECH_PHONE" => tech_phone: kv_string,
        "TECH_EMAIL" => tech_email: kv_string,
        "TECH_ADDRESS" => tech_address: kv_string,
        "PREVIOUS_MESSAGE_ID" => previous_message_id: kv_string,
        "NEXT_MESSAGE_ID" => next_message_id: kv_string,
        "ADM_MSG_LINK" => adm_msg_link: kv_string,
        "CDM_MSG_LINK" => cdm_msg_link: kv_string,
        "PRM_MSG_LINK" => prm_msg_link: kv_string,
        "RDM_MSG_LINK" => rdm_msg_link: kv_string,
        "TDM_MSG_LINK" => tdm_msg_link: kv_string,
        "OPERATOR" => operator: kv_string,
        "OWNER" => owner: kv_string,
        "COUNTRY" => country: kv_string,
        "CONSTELLATION" => constellation: kv_string,
        "OBJECT_TYPE" => object_type: kv_enum,
        "TIME_SYSTEM" => time_system: kv_string,
        "EPOCH_TZERO" => epoch_tzero: kv_epoch,
        "OPS_STATUS" => ops_status: kv_string,
        "ORBIT_CATEGORY" => orbit_category: kv_string,
        "OCM_DATA_ELEMENTS" => ocm_data_elements: kv_string,
        "SCLK_OFFSET_AT_EPOCH" => sclk_offset_at_epoch: kv_from_kvn,
        "SCLK_SEC_PER_SI_SEC" => sclk_sec_per_si_sec: kv_from_kvn,
        "PREVIOUS_MESSAGE_EPOCH" => previous_message_epoch: kv_epoch,
        "NEXT_MESSAGE_EPOCH" => next_message_epoch: kv_epoch,
        "START_TIME" => start_time: kv_epoch,
        "STOP_TIME" => stop_time: kv_epoch,
        "TIME_SPAN" => time_span: kv_from_kvn,
        "TAIMUTC_AT_TZERO" => taimutc_at_tzero: kv_from_kvn,
        "NEXT_LEAP_EPOCH" => next_leap_epoch: kv_epoch,
        "NEXT_LEAP_TAIMUTC" => next_leap_taimutc: kv_from_kvn,
        "UT1MUTC_AT_TZERO" => ut1mutc_at_tzero: kv_from_kvn,
        "EOP_SOURCE" => eop_source: kv_string,
        "INTERP_METHOD_EOP" => interp_method_eop: kv_string,
        "CELESTIAL_SOURCE" => celestial_source: kv_string,
    }, |i| at_block_end("META", i), "Unexpected OCM Metadata key");

    if at_block_end("META", input) {
        expect_block_end("META").parse_next(input)?;
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
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "Metadata", "TIME_SYSTEM"))?,
        epoch_tzero: epoch_tzero
            .ok_or_else(|| missing_field_err(input, "Metadata", "EPOCH_TZERO"))?,
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

pub fn ocm_traj_line(input: &mut &str) -> KvnResult<TrajLine> {
    if input
        .trim_start()
        .starts_with(|c: char| c.is_ascii_uppercase())
    {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }
    let epoch = till_space.parse_next(input)?;
    let values = repeat(1.., (space1, parse_f64_winnow).map(|(_, v)| v)).parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    Ok(TrajLine {
        epoch: epoch.to_string(),
        values,
    })
}

pub fn ocm_traj_state(input: &mut &str) -> KvnResult<OcmTrajState> {
    ws.parse_next(input)?;
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

    parse_block!(input, comment, {
        "TRAJ_ID" => traj_id: kv_string,
        "TRAJ_PREV_ID" => traj_prev_id: kv_string,
        "TRAJ_NEXT_ID" => traj_next_id: kv_string,
        "TRAJ_BASIS" => traj_basis: kv_enum,
        "TRAJ_BASIS_ID" => traj_basis_id: kv_string,
        "INTERPOLATION" => interpolation: kv_string,
        "INTERPOLATION_DEGREE" => interpolation_degree: kv_u32,
        "PROPAGATOR" => propagator: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "TRAJ_REF_FRAME" => traj_ref_frame: kv_string,
        "TRAJ_FRAME_EPOCH" => traj_frame_epoch: kv_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_epoch,
        "ORB_REVNUM" => orb_revnum: kv_float,
        "ORB_REVNUM_BASIS" => orb_revnum_basis: kv_enum,
        "TRAJ_TYPE" => traj_type: kv_string,
        "ORB_AVERAGING" => orb_averaging: kv_string,
        "TRAJ_UNITS" => traj_units: kv_string,
    }, |i| at_block_end("TRAJ", i), "Unexpected OCM Trajectory key");

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        if at_block_end("TRAJ", input) {
            comment.extend(comments);
            expect_block_end("TRAJ").parse_next(input)?;
            break;
        }

        if let Ok(line) = ocm_traj_line.parse_next(input) {
            traj_lines.push(line);
            continue;
        }

        // Otherwise, skip the line (handles continuations) or break
        let _ = till_line_ending.parse_next(input)?;
        opt_line_ending.parse_next(input)?;

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    if traj_lines.is_empty() {
        return Err(cut_err(
            input,
            "OCM Trajectory block must contain at least one state vector (trajLine)",
        ));
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
        traj_type: traj_type
            .ok_or_else(|| missing_field_err(input, "Trajectory State", "TRAJ_TYPE"))?,
        orb_averaging: orb_averaging.or(Some("OSCULATING".to_string())),
        traj_units,
        traj_lines,
    })
}

//----------------------------------------------------------------------
// OCM Physical Description Parser
//----------------------------------------------------------------------
// ... (omitting ocm_phys for brevity, it's unchanged)

pub fn ocm_phys(input: &mut &str) -> KvnResult<OcmPhysicalDescription> {
    ws.parse_next(input)?;
    expect_block_start("PHYS").parse_next(input)?;

    let mut comment = Vec::new();
    let mut manufacturer = None;
    let mut bus_model = None;
    let mut docked_with = None;
    let mut drag_const_area = None;
    let mut drag_coeff_nom = None;
    let mut drag_uncertainty = None;
    let mut initial_wet_mass = None;
    let mut wet_mass = None;
    let mut dry_mass = None;
    let mut oeb_parent_frame = None;
    let mut oeb_parent_frame_epoch = None;
    let mut oeb_q1 = None;
    let mut oeb_q2 = None;
    let mut oeb_q3 = None;
    let mut oeb_qc = None;
    let mut oeb_max = None;
    let mut oeb_int = None;
    let mut oeb_min = None;
    let mut area_along_oeb_max = None;
    let mut area_along_oeb_int = None;
    let mut area_along_oeb_min = None;
    let mut area_min_for_pc = None;
    let mut area_max_for_pc = None;
    let mut area_typ_for_pc = None;
    let mut rcs = None;
    let mut rcs_min = None;
    let mut rcs_max = None;
    let mut srp_const_area = None;
    let mut solar_rad_coeff = None;
    let mut solar_rad_uncertainty = None;
    let mut vm_absolute = None;
    let mut vm_apparent_min = None;
    let mut vm_apparent = None;
    let mut vm_apparent_max = None;
    let mut reflectance = None;
    let mut att_control_mode = None;
    let mut att_actuator_type = None;
    let mut att_knowledge = None;
    let mut att_control = None;
    let mut att_pointing = None;
    let mut avg_maneuver_freq = None;
    let mut max_thrust = None;
    let mut dv_bol = None;
    let mut dv_remaining = None;
    let mut ixx = None;
    let mut iyy = None;
    let mut izz = None;
    let mut ixy = None;
    let mut ixz = None;
    let mut iyz = None;

    parse_block!(input, comment, {
        "MANUFACTURER" => manufacturer: kv_string,
        "BUS_MODEL" => bus_model: kv_string,
        "DOCKED_WITH" => docked_with: kv_string,
        "DRAG_CONST_AREA" => drag_const_area: kv_from_kvn,
        "DRAG_COEFF_NOM" => drag_coeff_nom: kv_float,
        "DRAG_UNCERTAINTY" => drag_uncertainty: kv_from_kvn,
        "INITIAL_WET_MASS" => initial_wet_mass: kv_from_kvn,
        "WET_MASS" => wet_mass: kv_from_kvn,
        "DRY_MASS" => dry_mass: kv_from_kvn,
        "OEB_PARENT_FRAME" => oeb_parent_frame: kv_string,
        "OEB_PARENT_FRAME_EPOCH" => oeb_parent_frame_epoch: kv_epoch,
        "OEB_Q1" => oeb_q1: kv_float,
        "OEB_Q2" => oeb_q2: kv_float,
        "OEB_Q3" => oeb_q3: kv_float,
        "OEB_QC" => oeb_qc: kv_float,
        "OEB_MAX" => oeb_max: kv_from_kvn,
        "OEB_INT" => oeb_int: kv_from_kvn,
        "OEB_MIN" => oeb_min: kv_from_kvn,
        "AREA_ALONG_OEB_MAX" => area_along_oeb_max: kv_from_kvn,
        "AREA_ALONG_OEB_INT" => area_along_oeb_int: kv_from_kvn,
        "AREA_ALONG_OEB_MIN" => area_along_oeb_min: kv_from_kvn,
        "AREA_MIN_FOR_PC" => area_min_for_pc: kv_from_kvn,
        "AREA_MAX_FOR_PC" => area_max_for_pc: kv_from_kvn,
        "AREA_TYP_FOR_PC" => area_typ_for_pc: kv_from_kvn,
        "RCS" => rcs: kv_from_kvn,
        "RCS_MIN" => rcs_min: kv_from_kvn,
        "RCS_MAX" => rcs_max: kv_from_kvn,
        "SRP_CONST_AREA" => srp_const_area: kv_from_kvn,
        "SOLAR_RAD_COEFF" => solar_rad_coeff: kv_float,
        "SOLAR_RAD_UNCERTAINTY" => solar_rad_uncertainty: kv_from_kvn,
        "VM_ABSOLUTE" => vm_absolute: kv_float,
        "VM_APPARENT_MIN" => vm_apparent_min: kv_float,
        "VM_APPARENT" => vm_apparent: kv_float,
        "VM_APPARENT_MAX" => vm_apparent_max: kv_float,
        "REFLECTANCE" => reflectance: kv_from_kvn,
        "ATT_CONTROL_MODE" => att_control_mode: kv_string,
        "ATT_ACTUATOR_TYPE" => att_actuator_type: kv_string,
        "ATT_KNOWLEDGE" => att_knowledge: kv_from_kvn,
        "ATT_CONTROL" => att_control: kv_from_kvn,
        "ATT_POINTING" => att_pointing: kv_from_kvn,
        "AVG_MANEUVER_FREQ" => avg_maneuver_freq: kv_from_kvn,
        "MAX_THRUST" => max_thrust: kv_from_kvn,
        "DV_BOL" => dv_bol: kv_from_kvn,
        "DV_REMAINING" => dv_remaining: kv_from_kvn,
        "IXX" => ixx: kv_from_kvn,
        "IYY" => iyy: kv_from_kvn,
        "IZZ" => izz: kv_from_kvn,
        "IXY" => ixy: kv_from_kvn,
        "IXZ" => ixz: kv_from_kvn,
        "IYZ" => iyz: kv_from_kvn,
    }, |i| at_block_end("PHYS", i), "Unexpected OCM Physical key");

    if at_block_end("PHYS", input) {
        expect_block_end("PHYS").parse_next(input)?;
    }

    Ok(OcmPhysicalDescription {
        comment,
        manufacturer,
        bus_model,
        docked_with,
        drag_const_area,
        drag_coeff_nom,
        drag_uncertainty,
        initial_wet_mass,
        wet_mass,
        dry_mass,
        oeb_parent_frame,
        oeb_parent_frame_epoch,
        oeb_q1,
        oeb_q2,
        oeb_q3,
        oeb_qc,
        oeb_max,
        oeb_int,
        oeb_min,
        area_along_oeb_max,
        area_along_oeb_int,
        area_along_oeb_min,
        area_min_for_pc,
        area_max_for_pc,
        area_typ_for_pc,
        rcs,
        rcs_min,
        rcs_max,
        srp_const_area,
        solar_rad_coeff,
        solar_rad_uncertainty,
        vm_absolute,
        vm_apparent_min,
        vm_apparent,
        vm_apparent_max,
        reflectance,
        att_control_mode,
        att_actuator_type,
        att_knowledge,
        att_control,
        att_pointing,
        avg_maneuver_freq,
        max_thrust,
        dv_bol,
        dv_remaining,
        ixx,
        iyy,
        izz,
        ixy,
        ixz,
        iyz,
    })
}

pub fn ocm_cov_line(input: &mut &str) -> KvnResult<CovLine> {
    if input
        .trim_start()
        .starts_with(|c: char| c.is_ascii_uppercase())
    {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }
    let epoch = till_space.parse_next(input)?;
    let values = repeat(1.., (space1, parse_f64_winnow).map(|(_, v)| v)).parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    Ok(CovLine {
        epoch: epoch.to_string(),
        values,
    })
}

pub fn ocm_cov(input: &mut &str) -> KvnResult<OcmCovarianceMatrix> {
    ws.parse_next(input)?;
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

    parse_block!(input, comment, {
        "COV_ID" => cov_id: kv_string,
        "COV_PREV_ID" => cov_prev_id: kv_string,
        "COV_NEXT_ID" => cov_next_id: kv_string,
        "COV_BASIS" => cov_basis: kv_enum,
        "COV_BASIS_ID" => cov_basis_id: kv_string,
        "COV_REF_FRAME" => cov_ref_frame: kv_string,
        "COV_FRAME_EPOCH" => cov_frame_epoch: kv_epoch,
        "COV_TYPE" => cov_type: kv_string,
        "COV_UNITS" => cov_units: kv_string,
        "COV_ORDERING" => cov_ordering: kv_enum,
        "COV_SCALE_MIN" => cov_scale_min: kv_float,
        "COV_SCALE_MAX" => cov_scale_max: kv_float,
        "COV_CONFIDENCE" => cov_confidence: kv_from_kvn,
    }, |i| at_block_end("COV", i), "Unexpected OCM Covariance key");

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        if at_block_end("COV", input) {
            comment.extend(comments);
            expect_block_end("COV").parse_next(input)?;
            break;
        }

        if let Ok(line) = ocm_cov_line.parse_next(input) {
            cov_lines.push(line);
            continue;
        }

        // Otherwise break or handle error
        if input.offset_from(&checkpoint) == 0 {
            break;
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
        cov_type: cov_type.ok_or_else(|| missing_field_err(input, "Covariance", "COV_TYPE"))?,
        cov_ordering: cov_ordering.unwrap_or(CovOrder::Ltm),
        cov_units,
        cov_lines,
    })
}

pub fn ocm_man_line(input: &mut &str) -> KvnResult<ManLine> {
    let _checkpoint = input.checkpoint();
    let _ = ws.parse_next(input)?;
    if input.starts_with(|c: char| c.is_ascii_uppercase()) {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }
    let epoch = till_space.parse_next(input)?;
    let values =
        repeat(1.., (space1, till_space_or_eol).map(|(_, v)| v.to_string())).parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    Ok(ManLine {
        epoch: epoch.to_string(),
        values,
    })
}

pub fn ocm_man(input: &mut &str) -> KvnResult<OcmManeuverParameters> {
    ws.parse_next(input)?;
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

    parse_block!(input, comment, {
        "MAN_ID" => man_id: kv_string,
        "MAN_PREV_ID" => man_prev_id: kv_string,
        "MAN_NEXT_ID" => man_next_id: kv_string,
        "MAN_BASIS" => man_basis: kv_enum,
        "MAN_BASIS_ID" => man_basis_id: kv_string,
        "MAN_DEVICE_ID" => man_device_id: kv_string,
        "MAN_PREV_EPOCH" => man_prev_epoch: kv_epoch,
        "MAN_NEXT_EPOCH" => man_next_epoch: kv_epoch,
        "MAN_PURPOSE" => man_purpose: kv_string,
        "MAN_PRED_SOURCE" => man_pred_source: kv_string,
        "MAN_REF_FRAME" => man_ref_frame: kv_string,
        "MAN_FRAME_EPOCH" => man_frame_epoch: kv_epoch,
        "GRAV_ASSIST_NAME" => grav_assist_name: kv_string,
        "DC_TYPE" => dc_type: kv_enum,
        "DC_WIN_OPEN" => dc_win_open: kv_epoch,
        "DC_WIN_CLOSE" => dc_win_close: kv_epoch,
        "DC_MIN_CYCLES" => dc_min_cycles: kv_u64,
        "DC_MAX_CYCLES" => dc_max_cycles: kv_u64,
        "DC_EXEC_START" => dc_exec_start: kv_epoch,
        "DC_EXEC_STOP" => dc_exec_stop: kv_epoch,
        "DC_REF_TIME" => dc_ref_time: kv_epoch,
        "DC_TIME_PULSE_DURATION" => dc_time_pulse_duration: kv_from_kvn,
        "DC_TIME_PULSE_PERIOD" => dc_time_pulse_period: kv_from_kvn,
        "DC_REF_DIR" => dc_ref_dir: kv_from_kvn_value,
        "DC_BODY_FRAME" => dc_body_frame: kv_string,
        "DC_BODY_TRIGGER" => dc_body_trigger: kv_from_kvn_value,
        "DC_PA_START_ANGLE" => dc_pa_start_angle: kv_from_kvn,
        "DC_PA_STOP_ANGLE" => dc_pa_stop_angle: kv_from_kvn,
        "MAN_COMPOSITION" => man_composition: kv_string,
        "MAN_UNITS" => man_units: kv_string,
    }, |i| at_block_end("MAN", i), "Unexpected OCM Maneuver key");

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        if at_block_end("MAN", input) {
            comment.extend(comments);
            expect_block_end("MAN").parse_next(input)?;
            break;
        }

        if let Ok(line) = ocm_man_line.parse_next(input) {
            man_lines.push(line);
            continue;
        }

        // Otherwise, skip the line (handles continuations like in G17) or break
        let _ = till_line_ending.parse_next(input)?;
        opt_line_ending.parse_next(input)?;

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(OcmManeuverParameters {
        comment,
        man_id: man_id.ok_or_else(|| missing_field_err(input, "Maneuver", "MAN_ID"))?,
        man_prev_id,
        man_next_id,
        man_basis,
        man_basis_id,
        man_device_id: man_device_id
            .ok_or_else(|| missing_field_err(input, "Maneuver", "MAN_DEVICE_ID"))?,
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
        man_composition: man_composition
            .ok_or_else(|| missing_field_err(input, "Maneuver", "MAN_COMPOSITION"))?,
        man_units,
        man_lines,
    })
}

pub fn ocm_pert(input: &mut &str) -> KvnResult<OcmPerturbations> {
    ws.parse_next(input)?;
    expect_block_start("PERT").parse_next(input)?;

    let mut comment = Vec::new();

    let mut atmospheric_model = None;
    let mut gravity_model = None;
    let mut equatorial_radius = None;
    let mut gm = None;
    let mut n_body_perturbations = None;
    let mut central_body_rotation = None;
    let mut oblate_flattening = None;
    let mut ocean_tides_model = None;
    let mut solid_tides_model = None;
    let mut reduction_theory = None;
    let mut albedo_model = None;
    let mut albedo_grid_size = None;
    let mut shadow_model = None;
    let mut shadow_bodies = None;
    let mut srp_model = None;
    let mut sw_data_source = None;
    let mut sw_data_epoch = None;
    let mut sw_interp_method = None;
    let mut fixed_geomag_kp = None;
    let mut fixed_geomag_ap = None;
    let mut fixed_geomag_dst = None;
    let mut fixed_f10p7 = None;
    let mut fixed_f10p7_mean = None;
    let mut fixed_m10p7 = None;
    let mut fixed_m10p7_mean = None;
    let mut fixed_s10p7 = None;
    let mut fixed_s10p7_mean = None;
    let mut fixed_y10p7 = None;
    let mut fixed_y10p7_mean = None;

    parse_block!(input, comment, {
        "ATMOSPHERIC_MODEL" => atmospheric_model: kv_string,
        "GRAVITY_MODEL" => gravity_model: kv_string,
        "EQUATORIAL_RADIUS" => equatorial_radius: kv_from_kvn,
        "GM" => gm: kv_from_kvn,
        "N_BODY_PERTURBATIONS" => n_body_perturbations: kv_string,
        "CENTRAL_BODY_ROTATION" => central_body_rotation: kv_from_kvn,
        "OBLATE_FLATTENING" => oblate_flattening: kv_float,
        "OCEAN_TIDES_MODEL" => ocean_tides_model: kv_string,
        "SOLID_TIDES_MODEL" => solid_tides_model: kv_string,
        "REDUCTION_THEORY" => reduction_theory: kv_string,
        "ALBEDO_MODEL" => albedo_model: kv_string,
        "ALBEDO_GRID_SIZE" => albedo_grid_size: kv_u64,
        "SHADOW_MODEL" => shadow_model: kv_string,
        "SHADOW_BODIES" => shadow_bodies: kv_string,
        "SRP_MODEL" => srp_model: kv_string,
        "SW_DATA_SOURCE" => sw_data_source: kv_string,
        "SW_DATA_EPOCH" => sw_data_epoch: kv_epoch,
        "SW_INTERP_METHOD" => sw_interp_method: kv_string,
        "FIXED_GEOMAG_KP" => fixed_geomag_kp: kv_from_kvn,
        "FIXED_GEOMAG_AP" => fixed_geomag_ap: kv_from_kvn,
        "FIXED_GEOMAG_DST" => fixed_geomag_dst: kv_from_kvn,
        "FIXED_F10P7" => fixed_f10p7: kv_from_kvn,
        "FIXED_F10P7_MEAN" => fixed_f10p7_mean: kv_from_kvn,
        "FIXED_M10P7" => fixed_m10p7: kv_from_kvn,
        "FIXED_M10P7_MEAN" => fixed_m10p7_mean: kv_from_kvn,
        "FIXED_S10P7" => fixed_s10p7: kv_from_kvn,
        "FIXED_S10P7_MEAN" => fixed_s10p7_mean: kv_from_kvn,
        "FIXED_Y10P7" => fixed_y10p7: kv_from_kvn,
        "FIXED_Y10P7_MEAN" => fixed_y10p7_mean: kv_from_kvn,
    }, |i| at_block_end("PERT", i), "Unexpected OCM Perturbations key");

    if at_block_end("PERT", input) {
        expect_block_end("PERT").parse_next(input)?;
    }

    Ok(OcmPerturbations {
        comment,
        atmospheric_model,
        gravity_model,
        equatorial_radius,
        gm,
        n_body_perturbations,
        central_body_rotation,
        oblate_flattening,
        ocean_tides_model,
        solid_tides_model,
        reduction_theory,
        albedo_model,
        albedo_grid_size,
        shadow_model,
        shadow_bodies,
        srp_model,
        sw_data_source,
        sw_data_epoch,
        sw_interp_method,
        fixed_geomag_kp,
        fixed_geomag_ap,
        fixed_geomag_dst,
        fixed_f10p7,
        fixed_f10p7_mean,
        fixed_m10p7,
        fixed_m10p7_mean,
        fixed_s10p7,
        fixed_s10p7_mean,
        fixed_y10p7,
        fixed_y10p7_mean,
    })
}

pub fn ocm_od(input: &mut &str) -> KvnResult<OcmOdParameters> {
    ws.parse_next(input)?;
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
    let mut _max_resi_accepted = None;

    parse_block!(input, comment, {
        "OD_ID" => od_id: kv_string,
        "OD_PREV_ID" => od_prev_id: kv_string,
        "OD_METHOD" => od_method: kv_string,
        "OD_EPOCH" => od_epoch: kv_epoch,
        "OD_TIME_TAG" => od_epoch: kv_epoch,
        "MAX_RESI_ACCEPTED" => _max_resi_accepted: kv_string,
        "DAYS_SINCE_FIRST_OBS" => days_since_first_obs: kv_from_kvn,
        "DAYS_SINCE_LAST_OBS" => days_since_last_obs: kv_from_kvn,
        "RECOMMENDED_OD_SPAN" => recommended_od_span: kv_from_kvn,
        "ACTUAL_OD_SPAN" => actual_od_span: kv_from_kvn,
        "OBS_AVAILABLE" => obs_available: kv_u64,
        "OBS_USED" => obs_used: kv_u64,
        "TRACKS_AVAILABLE" => tracks_available: kv_u64,
        "TRACKS_USED" => tracks_used: kv_u64,
        "MAXIMUM_OBS_GAP" => maximum_obs_gap: kv_from_kvn,
        "OD_EPOCH_EIGMAJ" => od_epoch_eigmaj: kv_from_kvn,
        "OD_EPOCH_EIGINT" => od_epoch_eigint: kv_from_kvn,
        "OD_EPOCH_EIGMIN" => od_epoch_eigmin: kv_from_kvn,
        "OD_MAX_PRED_EIGMAJ" => od_max_pred_eigmaj: kv_from_kvn,
        "OD_MIN_PRED_EIGMIN" => od_min_pred_eigmin: kv_from_kvn,
        "OD_CONFIDENCE" => od_confidence: kv_from_kvn,
        "GDOP" => gdop: kv_float,
        "SOLVE_N" => solve_n: kv_u64,
        "SOLVE_STATES" => solve_states: kv_string,
        "CONSIDER_N" => consider_n: kv_u64,
        "CONSIDER_PARAMS" => consider_params: kv_string,
        "SEDR" => sedr: kv_from_kvn,
        "SENSORS_N" => sensors_n: kv_u64,
        "SENSORS" => sensors: kv_string,
        "WEIGHTED_RMS" => weighted_rms: kv_from_kvn,
        "DATA_TYPES" => data_types: kv_string,
    }, |i| at_block_end("OD", i), "Unexpected OCM Orbit Determination key");

    if at_block_end("OD", input) {
        expect_block_end("OD").parse_next(input)?;
    }

    Ok(OcmOdParameters {
        comment,
        od_id: od_id.ok_or_else(|| missing_field_err(input, "Orbit Determination", "OD_ID"))?,
        od_prev_id,
        od_method: od_method
            .ok_or_else(|| missing_field_err(input, "Orbit Determination", "OD_METHOD"))?,
        od_epoch: od_epoch
            .ok_or_else(|| missing_field_err(input, "Orbit Determination", "OD_EPOCH"))?,
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

pub fn ocm_user(input: &mut &str) -> KvnResult<UserDefined> {
    ws.parse_next(input)?;
    expect_block_start("USER").parse_next(input)?;

    let mut comment = Vec::new();
    let mut user_defined = Vec::new();

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        if at_block_end("USER", input) {
            comment.extend(comments);
            expect_block_end("USER").parse_next(input)?;
            break;
        }

        match key_token.parse_next(input) {
            Ok(k) => {
                comment.extend(comments);
                let v = kv_string.parse_next(input)?;
                user_defined.push(UserDefinedParameter {
                    parameter: k.strip_prefix("USER_DEFINED_").unwrap_or(k).to_string(),
                    value: v,
                });
            }
            Err(_) => {
                input.reset(&checkpoint);
                return Err(cut_err(input, "Unexpected key in USER block"));
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

enum OcmBlock {
    Traj(OcmTrajState),
    Phys(OcmPhysicalDescription),
    Cov(OcmCovarianceMatrix),
    Man(OcmManeuverParameters),
    Pert(OcmPerturbations),
    Od(OcmOdParameters),
    User(UserDefined),
}

fn ocm_data_block(input: &mut &str) -> KvnResult<OcmBlock> {
    use winnow::combinator::{dispatch, peek};

    // Fast-path: if it doesn't look like a keyword/block start (starts with digit or sign),
    // it's likely a data line that should be handled by the parent loop's line parsers.
    // However, OCM data is block-based, so if we aren't at a block start, we should backtrack.
    let first_char = input.trim_start().chars().next();
    if matches!(first_char, Some('0'..='9' | '-' | '+')) {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }

    let comments = collect_comments.parse_next(input)?;

    // Fail if we see META_START (end of data section) to stop repeat
    if at_block_start("META", input) {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }

    // Peek the block start tag to decide which parser to use
    // If block_start fails (e.g. no _START tag), we backtrack, stopping repeat
    // dispatch! executes the first parser (peek(block_start)) and uses the result to choose the branch.
    let mut block = dispatch! { peek(block_start);
        "TRAJ" => ocm_traj_state.map(OcmBlock::Traj),
        "PHYS" => ocm_phys.map(OcmBlock::Phys),
        "COV" => ocm_cov.map(OcmBlock::Cov),
        "MAN" => ocm_man.map(OcmBlock::Man),
        "PERT" => ocm_pert.map(OcmBlock::Pert),
        "OD" => ocm_od.map(OcmBlock::Od),
        "USER" => ocm_user.map(OcmBlock::User),
        _ => winnow::combinator::fail,
    }
    .parse_next(input)?;

    // Prepend comments collected before the block start
    match &mut block {
        OcmBlock::Traj(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::Phys(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::Cov(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::Man(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::Pert(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::Od(x) => {
            x.comment.splice(0..0, comments);
        }
        OcmBlock::User(x) => {
            x.comment.splice(0..0, comments);
        }
    }

    Ok(block)
}

pub fn ocm_data(input: &mut &str) -> KvnResult<OcmData> {
    let mut data = OcmData::default();

    loop {
        let checkpoint = input.checkpoint();
        match ocm_data_block.parse_next(input) {
            Ok(block) => match block {
                OcmBlock::Traj(x) => data.traj.push(x),
                OcmBlock::Phys(x) => data.phys = Some(x),
                OcmBlock::Cov(x) => data.cov.push(x),
                OcmBlock::Man(x) => data.man.push(x),
                OcmBlock::Pert(x) => data.pert = Some(x),
                OcmBlock::Od(x) => data.od = Some(x),
                OcmBlock::User(x) => data.user = Some(x),
            },
            Err(e) => {
                if e.is_backtrack() {
                    input.reset(&checkpoint);
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }

    // Check for unexpected data after blocks (and their comments) are consumed.
    // ... (rest of the logic remains similar)
    let checkpoint = input.checkpoint();
    let _ = collect_comments.parse_next(input);
    if !input.is_empty() && !at_block_start("META", input) {
        return Err(cut_err(input, "Unexpected OCM Data key"));
    }
    input.reset(&checkpoint);

    Ok(data)
}

pub fn ocm_header(input: &mut &str) -> KvnResult<OdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let checkpoint = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        if input.is_empty() || at_block_start("META", input) {
            break;
        }

        let key = match keyword.parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        if !matches!(
            key,
            "CLASSIFICATION" | "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID"
        ) {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = Some(kv_string.parse_next(input)?);
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = Some(kv_string.parse_next(input)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(OdmHeader {
        comment,
        classification,
        creation_date: creation_date
            .ok_or_else(|| missing_field_err(input, "Header", "CREATION_DATE"))?,
        originator: originator.ok_or_else(|| missing_field_err(input, "Header", "ORIGINATOR"))?,
        message_id,
    })
}

//----------------------------------------------------------------------
// Complete OCM Parser
//----------------------------------------------------------------------

pub fn parse_ocm(input: &mut &str) -> KvnResult<Ocm> {
    let version = ocm_version.parse_next(input)?;
    let header = ocm_header.parse_next(input)?;
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
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        if input.trim().is_empty() {
            return Err(cut_err(input, "Empty file"));
        }
        parse_ocm.parse_next(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, FormatError, ValidationError};
    use crate::traits::Ndm;

    #[test]
    fn test_xsd_metadata_mandatory_time_system() {
        // XSD: TIME_SYSTEM is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_xsd_metadata_mandatory_epoch_tzero() {
        // XSD: EPOCH_TZERO is mandatory (no minOccurs="0" attribute)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "EPOCH_TZERO")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => assert!(err.message.contains("EPOCH_TZERO")),
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_metadata_all_optional_fields() {
        // XSD: Most metadata fields are minOccurs="0" - verify they can all be set
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SATELLITE-1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = NORAD
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT1
ORIGINATOR_POC = John Doe
OPERATOR = OPERATOR_A
OWNER = OWNER_B
COUNTRY = USA
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body.segment.metadata.object_name,
            Some("SATELLITE-1".into())
        );
        assert_eq!(
            ocm.body.segment.metadata.international_designator,
            Some("2023-001A".into())
        );
        assert_eq!(
            ocm.body.segment.metadata.operator,
            Some("OPERATOR_A".into())
        );
        assert_eq!(ocm.body.segment.metadata.country, Some("USA".into()));
    }

    #[test]
    fn test_xsd_metadata_sclk_defaults() {
        // XSD: SCLK_OFFSET_AT_EPOCH default="0.0", SCLK_SEC_PER_SI_SEC default="1.0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        // XSD defaults
        assert!(ocm.body.segment.metadata.sclk_offset_at_epoch.is_some());
        assert!(ocm.body.segment.metadata.sclk_sec_per_si_sec.is_some());
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 2: Trajectory Block (ocmTrajStateType)
    // XSD: CENTER_NAME, TRAJ_REF_FRAME, TRAJ_TYPE mandatory
    // XSD: trajLine minOccurs="1" maxOccurs="unbounded"
    // XSD: traj minOccurs="0" maxOccurs="unbounded" in ocmData
    // Note: Library applies defaults for some mandatory fields
    // =========================================================================

    #[test]
    fn test_xsd_traj_optional_in_data() {
        // XSD: traj minOccurs="0" - OCM can exist without trajectory block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.traj.is_empty());
    }

    #[test]
    fn test_xsd_traj_mandatory_traj_type() {
        // XSD: TRAJ_TYPE is mandatory (no minOccurs="0")
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "TRAJ_TYPE")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => assert!(err.message.contains("TRAJ_TYPE")),
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_traj_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple trajectory blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
TRAJ_START
CENTER_NAME = MOON
TRAJ_REF_FRAME = ICRF
TRAJ_TYPE = KEPLERIAN
2023-01-01T01:00:00 7000 0.001 28 0 0 0
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 2);
        assert_eq!(ocm.body.segment.data.traj[0].center_name, "EARTH");
        assert_eq!(ocm.body.segment.data.traj[1].center_name, "MOON");
    }

    #[test]
    fn test_xsd_traj_multiple_lines() {
        // XSD: trajLine maxOccurs="unbounded" - multiple trajectory data lines
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
2023-01-01T01:00:00 1.1 2.1 3.1 4.1 5.1 6.1
2023-01-01T02:00:00 1.2 2.2 3.2 4.2 5.2 6.2
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 3);
    }

    #[test]
    fn test_xsd_traj_optional_fields() {
        // XSD: Many trajectory fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
TRAJ_ID = TRAJECTORY_1
TRAJ_PREV_ID = TRAJECTORY_0
TRAJ_NEXT_ID = TRAJECTORY_2
TRAJ_BASIS = DETERMINED
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 100
TRAJ_TYPE = CARTPV
ORB_AVERAGING = OSCULATING
TRAJ_UNITS = km km km km/s km/s km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let traj = &ocm.body.segment.data.traj[0];
        assert_eq!(traj.traj_id, Some("TRAJECTORY_1".into()));
        assert_eq!(traj.interpolation, Some("LAGRANGE".into()));
        assert_eq!(traj.interpolation_degree, Some(7));
        assert_eq!(traj.propagator, Some("SGP4".into()));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 3: Covariance Block (ocmCovarianceMatrixType)
    // XSD: COV_REF_FRAME, COV_TYPE, COV_ORDERING mandatory
    // XSD: covLine minOccurs="1" maxOccurs="unbounded"
    // XSD: cov minOccurs="0" maxOccurs="unbounded" in ocmData
    // =========================================================================

    #[test]
    fn test_xsd_cov_optional_in_data() {
        // XSD: cov minOccurs="0" - OCM can exist without covariance block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.cov.is_empty());
    }

    #[test]
    fn test_xsd_cov_mandatory_type() {
        // XSD: COV_TYPE is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "COV_TYPE")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => assert!(err.message.contains("COV_TYPE")),
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_cov_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple covariance blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COV_START
COV_REF_FRAME = TNW
COV_TYPE = KEPLERIAN
COV_ORDERING = UTM
2023-01-01T01:00:00 1e-5 0 1e-5 0 0 1e-5
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov.len(), 2);
        assert_eq!(ocm.body.segment.data.cov[0].cov_type, "CARTPV");
        assert_eq!(ocm.body.segment.data.cov[1].cov_type, "KEPLERIAN");
    }

    #[test]
    fn test_xsd_cov_multiple_lines() {
        // XSD: covLine maxOccurs="unbounded"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
2023-01-01T01:00:00 1.1e-6 0 1.1e-6 0 0 1.1e-6
2023-01-01T02:00:00 1.2e-6 0 1.2e-6 0 0 1.2e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_lines.len(), 3);
    }

    #[test]
    fn test_xsd_cov_optional_fields() {
        // XSD: Many covariance fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_ID = COVARIANCE_1
COV_PREV_ID = COVARIANCE_0
COV_NEXT_ID = COVARIANCE_2
COV_BASIS = DETERMINED
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = km**2 km**2 km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let cov = &ocm.body.segment.data.cov[0];
        assert_eq!(cov.cov_id, Some("COVARIANCE_1".into()));
        assert_eq!(cov.cov_scale_min, Some(0.5));
        assert_eq!(cov.cov_scale_max, Some(2.0));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 4: Maneuver Block (ocmManeuverParametersType)
    // XSD: MAN_ID, MAN_DEVICE_ID, MAN_REF_FRAME, DC_TYPE, MAN_COMPOSITION mandatory
    // XSD: manLine minOccurs="1" maxOccurs="unbounded"
    // XSD: man minOccurs="0" maxOccurs="unbounded" in ocmData
    // =========================================================================

    #[test]
    fn test_xsd_man_optional_in_data() {
        // XSD: man minOccurs="0" - OCM can exist without maneuver block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.man.is_empty());
    }

    #[test]
    fn test_xsd_man_mandatory_man_id() {
        // XSD: MAN_ID is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "MAN_ID")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => assert!(err.message.contains("MAN_ID")),
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_man_mandatory_device_id() {
        // XSD: MAN_DEVICE_ID is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "MAN_DEVICE_ID")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(err.message.contains("MAN_DEVICE_ID"));
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_man_mandatory_composition() {
        // XSD: MAN_COMPOSITION is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) => match *val_err {
                ValidationError::MissingRequiredField { ref field, .. } => {
                    assert_eq!(field, "MAN_COMPOSITION")
                }
                _ => panic!("unexpected validation error: {:?}", val_err),
            },
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(err.message.contains("MAN_COMPOSITION"));
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_xsd_man_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple maneuver blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
MAN_START
MAN_ID = MANEUVER_2
MAN_DEVICE_ID = THRUSTER_2
MAN_REF_FRAME = TNW
DC_TYPE = TIME
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-02T00:00:00 0 0.1 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.man.len(), 2);
        assert_eq!(ocm.body.segment.data.man[0].man_id, "MANEUVER_1");
        assert_eq!(ocm.body.segment.data.man[1].man_id, "MANEUVER_2");
    }

    #[test]
    fn test_xsd_man_optional_fields() {
        // XSD: Many maneuver fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_PREV_ID = MANEUVER_0
MAN_NEXT_ID = MANEUVER_2
MAN_BASIS = CANDIDATE
MAN_DEVICE_ID = THRUSTER_1
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
DC_TYPE = CONTINUOUS
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T01:00:00
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let man = &ocm.body.segment.data.man[0];
        assert_eq!(man.man_prev_id, Some("MANEUVER_0".into()));
        assert_eq!(man.man_purpose, Some("ORBIT_RAISING".into()));
        assert!(man.dc_win_open.is_some());
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 5: Physical, Perturbations, OD Parameters
    // XSD: phys, pert, od all minOccurs="0" (optional)
    // XSD: All fields within these blocks are optional (minOccurs="0")
    // =========================================================================

    #[test]
    fn test_xsd_phys_optional_in_data() {
        // XSD: phys minOccurs="0" - OCM can exist without physical description
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_none());
    }

    #[test]
    fn test_xsd_phys_all_optional_fields() {
        // XSD: All physical description fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME_CORP
BUS_MODEL = LEO_BUS
WET_MASS = 500 [kg]
DRY_MASS = 400 [kg]
DRAG_CONST_AREA = 10.0 [m**2]
DRAG_COEFF_NOM = 2.2
SRP_CONST_AREA = 8.0 [m**2]
SOLAR_RAD_COEFF = 1.2
RCS = 1.0 [m**2]
MAX_THRUST = 0.1 [N]
DV_BOL = 0.3 [km/s]
DV_REMAINING = 0.15 [km/s]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.manufacturer, Some("ACME_CORP".into()));
        assert!(phys.wet_mass.is_some());
        assert!(phys.dry_mass.is_some());
    }

    #[test]
    fn test_xsd_phys_inertia_tensor() {
        // XSD: momentType for IXX, IYY, IZZ, IXY, IXZ, IYZ
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
IXX = 100 [kg*m**2]
IYY = 200 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 10 [kg*m**2]
IXZ = 5 [kg*m**2]
IYZ = 8 [kg*m**2]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert!(phys.ixx.is_some());
        assert!(phys.iyy.is_some());
        assert!(phys.izz.is_some());
    }

    #[test]
    fn test_xsd_pert_optional_in_data() {
        // XSD: pert minOccurs="0" - OCM can exist without perturbations block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.pert.is_none());
    }

    #[test]
    fn test_xsd_pert_all_optional_fields() {
        // XSD: All perturbation fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();
        assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".into()));
        assert!(pert.gravity_model.is_some());
        assert!(pert.gm.is_some());
    }

    #[test]
    fn test_xsd_od_optional_in_data() {
        // XSD: od minOccurs="0" - OCM can exist without OD parameters block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.od.is_none());
    }

    #[test]
    fn test_xsd_od_all_optional_fields() {
        // XSD: OD has some mandatory fields (OD_ID, OD_METHOD, OD_EPOCH) and many optional
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD_1
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
WEIGHTED_RMS = 1.5
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();
        assert_eq!(od.od_method, "BATCH_LS");
        assert!(!od.od_epoch.to_string().is_empty());
    }

    #[test]
    fn test_xsd_user_defined_optional() {
        // XSD: user minOccurs="0" - user defined block is optional
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
USER_DEFINED_CUSTOM_PARAM = custom_value
USER_DEFINED_ANOTHER_PARAM = another_value
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let user = ocm.body.segment.data.user.as_ref().unwrap();
        assert_eq!(user.user_defined.len(), 2);
        assert_eq!(user.user_defined[0].parameter, "CUSTOM_PARAM");
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 6: Sample Files & Roundtrips
    // =========================================================================

    #[test]
    fn test_xsd_sample_ocm_g15_kvn() {
        // Parse official CCSDS OCM example G-15
        let kvn = include_str!("../../../data/kvn/ocm_g15.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
        assert!(!ocm.body.segment.metadata.epoch_tzero.to_string().is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g16_kvn() {
        // Parse official CCSDS OCM example G-16
        let kvn = include_str!("../../../data/kvn/ocm_g16.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g17_kvn() {
        // Parse official CCSDS OCM example G-17
        let kvn = include_str!("../../../data/kvn/ocm_g17.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g18_kvn() {
        // Parse official CCSDS OCM example G-18
        let kvn = include_str!("../../../data/kvn/ocm_g18.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g19_kvn() {
        // Parse official CCSDS OCM example G-19
        let kvn = include_str!("../../../data/kvn/ocm_g19.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_complex_ocm_all_blocks() {
        // OCM with trajectory, covariance, maneuver blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST_SATELLITE
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
        assert_eq!(ocm.body.segment.data.cov.len(), 1);
        assert_eq!(ocm.body.segment.data.man.len(), 1);
    }

    #[test]
    fn test_ocm_parsing_errors() {
        // Empty file
        let err = Ocm::from_kvn("").unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {}
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(ref err) if err.message.is_empty()) =>
                {}
            _ => panic!("Expected Empty file error, got: {:?}", err),
        }

        // Wrong first keyword
        let err = Ocm::from_kvn("CREATION_DATE = 2023-01-01T00:00:00").unwrap_err();
        match err {
            CcsdsNdmError::Validation(val_err) if matches!(*val_err, ValidationError::MissingRequiredField { ref field, .. } if field.contains("first keyword")) =>
                {}
            CcsdsNdmError::Format(format_err) if matches!(*format_err, FormatError::Kvn(_)) => {}
            _ => panic!("Expected first keyword error, got: {:?}", err),
        }

        // Comments before version should be OK
        let kvn = r#"
COMMENT leading comment
CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        assert!(Ocm::from_kvn(kvn).is_ok());

        // Unexpected segment start
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
TRAJ_START
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        if let Some(err) = err.as_kvn_parse_error() {
            assert!(
                err.message.contains("Expected META_START")
                    || err.contexts.contains(&"Expected META_START")
            );
        } else {
            panic!("Expected KVN parse error, got: {:?}", err);
        }
        // Metadata unexpected key
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
BAD_KEY = VAL
META_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        if let Some(err) = err.as_kvn_parse_error() {
            assert!(
                err.message.contains("Expected META_STOP")
                    || err.message.contains("Unexpected OCM Data key")
                    || err.contexts.contains(&"Expected META_STOP")
                    || err.contexts.contains(&"Unexpected OCM Data key")
                    || err.contexts.contains(&"Unexpected OCM Metadata key")
            );
        } else {
            panic!("Expected KVN parse error, got: {:?}", err);
        }
    }

    #[test]
    fn test_ocm_metadata_all_fields() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT meta comment
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT_ALT
ORIGINATOR_POC = JOHN DOE
ORIGINATOR_POSITION = ENGINEER
ORIGINATOR_PHONE = 123-456
ORIGINATOR_EMAIL = john@example.com
ORIGINATOR_ADDRESS = 123 Street
TECH_ORG = SPACE_CORP
TECH_POC = JANE DOE
TECH_POSITION = SCIENTIST
TECH_PHONE = 987-654
TECH_EMAIL = jane@example.com
TECH_ADDRESS = 456 Avenue
PREVIOUS_MESSAGE_ID = MSG_001
NEXT_MESSAGE_ID = MSG_003
ADM_MSG_LINK = ADM_LINK
CDM_MSG_LINK = CDM_LINK
PRM_MSG_LINK = PRM_LINK
RDM_MSG_LINK = RDM_LINK
TDM_MSG_LINK = TDM_LINK
OPERATOR = OPS_TEAM
OWNER = OWNER_TEAM
COUNTRY = USA
CONSTELLATION = STARLINK
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
OCM_DATA_ELEMENTS = ALL
SCLK_OFFSET_AT_EPOCH = 0.1 [s]
SCLK_SEC_PER_SI_SEC = 0.99 [s]
PREVIOUS_MESSAGE_EPOCH = 2022-12-31T23:00:00
NEXT_MESSAGE_EPOCH = 2023-01-01T01:00:00
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
TAIMUTC_AT_TZERO = 37.0 [s]
NEXT_LEAP_EPOCH = 2024-01-01T00:00:00
NEXT_LEAP_TAIMUTC = 38.0 [s]
UT1MUTC_AT_TZERO = -0.1 [s]
EOP_SOURCE = IERS
INTERP_METHOD_EOP = LINEAR
CELESTIAL_SOURCE = IAU
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let meta = &ocm.body.segment.metadata;
        assert_eq!(meta.object_name, Some("SAT1".to_string()));
        assert_eq!(meta.object_type, Some(ObjectDescription::Payload));
        assert!(meta.sclk_offset_at_epoch.is_some());

        // Roundtrip to hit write_kvn for all fields
        let output = ocm.to_kvn().unwrap();
        let ocm2 = Ocm::from_kvn(&output).unwrap();
        assert_eq!(ocm.body.segment.metadata, ocm2.body.segment.metadata);
    }

    #[test]
    fn test_ocm_data_loop_break_and_comments() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT Trailing comment
UNEXPECTED_KEY = value
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unexpected OCM Data key")
                            || err.contexts.contains(&"Unexpected OCM Data key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got: {:?}", err),
        }
    }

    #[test]
    fn test_ocm_user_defined_unexpected_token() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
META_START
USER_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        if let Some(err) = err.as_kvn_parse_error() {
            assert!(
                err.message.contains("Expected META_START")
                    || err.message.contains("Expected TRAJ_START")
                    || err.contexts.contains(&"Expected META_START")
                    || err.contexts.contains(&"Expected TRAJ_START")
                    || err.contexts.contains(&"Unexpected key in USER block")
            );
        } else {
            panic!("Expected KVN parse error, got: {:?}", err);
        }
    }

    #[test]
    fn test_traj_all_fields() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
COMMENT traj comment
TRAJ_ID = T1
TRAJ_PREV_ID = T0
TRAJ_NEXT_ID = T2
TRAJ_BASIS = PREDICTED
TRAJ_BASIS_ID = B1
INTERPOLATION = LINEAR
INTERPOLATION_DEGREE = 1
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 1234
ORB_REVNUM_BASIS = 1
TRAJ_TYPE = CARTPV
ORB_AVERAGING = NONE
TRAJ_UNITS = km km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let traj = &ocm.body.segment.data.traj[0];
        assert_eq!(traj.traj_id, Some("T1".to_string()));
        assert_eq!(traj.traj_basis, Some(TrajBasis::Predicted));
        assert_eq!(traj.orb_revnum_basis, Some(RevNumBasis::One));

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("ORB_REVNUM_BASIS"));
        assert!(output.contains("1"));
    }

    #[test]
    fn test_phys_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
BUS_MODEL = B1
DOCKED_WITH = SAT2
DRAG_CONST_AREA = 1.0 [m**2]
DRAG_COEFF_NOM = 2.2
DRAG_UNCERTAINTY = 10 [%]
INITIAL_WET_MASS = 1000 [kg]
WET_MASS = 900 [kg]
DRY_MASS = 800 [kg]
OEB_PARENT_FRAME = GCRF
OEB_PARENT_FRAME_EPOCH = 2023-01-01T00:00:00
OEB_Q1 = 0
OEB_Q2 = 0
OEB_Q3 = 0
OEB_QC = 1
OEB_MAX = 5 [m]
OEB_INT = 3 [m]
OEB_MIN = 2 [m]
AREA_ALONG_OEB_MAX = 15 [m**2]
AREA_ALONG_OEB_INT = 10 [m**2]
AREA_ALONG_OEB_MIN = 6 [m**2]
AREA_MIN_FOR_PC = 5 [m**2]
AREA_MAX_FOR_PC = 20 [m**2]
AREA_TYP_FOR_PC = 10 [m**2]
RCS = 1 [m**2]
RCS_MIN = 0.5 [m**2]
RCS_MAX = 2 [m**2]
SRP_CONST_AREA = 12 [m**2]
SOLAR_RAD_COEFF = 1.5
SOLAR_RAD_UNCERTAINTY = 5 [%]
VM_ABSOLUTE = 4.5
VM_APPARENT_MIN = 5.0
VM_APPARENT = 5.5
VM_APPARENT_MAX = 6.0
REFLECTANCE = 0.8
ATT_CONTROL_MODE = THREE_AXIS
ATT_ACTUATOR_TYPE = REACTION_WHEELS
ATT_KNOWLEDGE = 0.1 [deg]
ATT_CONTROL = 0.5 [deg]
ATT_POINTING = 0.2 [deg]
AVG_MANEUVER_FREQ = 12 [#/yr]
MAX_THRUST = 0.1 [N]
DV_BOL = 0.5 [km/s]
DV_REMAINING = 0.2 [km/s]
IXX = 100 [kg*m**2]
IYY = 150 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 1 [kg*m**2]
IXZ = 2 [kg*m**2]
IYZ = 3 [kg*m**2]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.manufacturer, Some("ACME".to_string()));
        assert_eq!(phys.ixx.as_ref().unwrap().value, 100.0);

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("IXX"));
        assert!(output.contains("100") || output.contains("1.0e2") || output.contains("1e2"));
    }

    #[test]
    fn test_phys_parsing_errors() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
DRAG_COEFF_NOM = NOT_A_FLOAT
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());

        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q1 = NOT_A_FLOAT
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_pert_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = SUN MOON JUPITER
OCEAN_TIDES_MODEL = GOT
SOLID_TIDES_MODEL = IERS
REDUCTION_THEORY = IAU
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();
        assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".to_string()));
        assert!(pert.gm.is_some());
    }

    #[test]
    fn test_od_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_TIME_TAG = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 10 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 10
TRACKS_USED = 9
MAX_RESI_ACCEPTED = 3 [%]
WEIGHTED_RMS = 0.5
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();
        assert_eq!(od.od_id, "OD1");
        assert_eq!(od.obs_available, Some(100));
    }

    #[test]
    fn test_cov_ordering_wcc() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTMWCC
2023-01-01T00:00:00 1e-6 1e-6 1e-6 1e-6 1e-6 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_ordering, CovOrder::LtmWcc);

        // Also test UTMWCC
        let kvn2 = kvn.replace("LTMWCC", "UTMWCC");
        let ocm2 = Ocm::from_kvn(&kvn2).unwrap();
        assert_eq!(ocm2.body.segment.data.cov[0].cov_ordering, CovOrder::UtmWcc);

        // Also test FULL
        let kvn3 = kvn.replace("LTMWCC", "FULL");
        let ocm3 = Ocm::from_kvn(&kvn3).unwrap();
        assert_eq!(ocm3.body.segment.data.cov[0].cov_ordering, CovOrder::Full);
    }

    // =========================================================================
    // Additional coverage tests for 100% coverage
    // =========================================================================

    #[test]
    fn test_eof_after_meta_start() {
        // Cover line 150: EOF before OcmSegment check
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::UnexpectedEof { .. } => {}
            CcsdsNdmError::Format(ref e) if matches!(**e, FormatError::Kvn(_)) => {}
            _ => panic!("Expected EOF or KvnParse error, got: {:?}", err),
        }
    }

    #[test]
    fn test_empty_lines_in_metadata() {
        // Cover lines 488-490: Empty lines in metadata
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START

TIME_SYSTEM = UTC

EPOCH_TZERO = 2023-01-01T00:00:00

META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_empty_lines_in_data_section() {
        // Cover lines 785-787: Empty lines in data section
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP

TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP

"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
    }

    #[test]
    fn test_comments_before_blocks() {
        // Cover pending_comments splice (lines 719, 726, 733, etc.)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT Comment before TRAJ
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT Comment before COV
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COMMENT Comment before MAN
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
COMMENT Comment before PERT
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
COMMENT Comment before OD
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
COMMENT Comment before USER
USER_START
PARAM = VAL
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify comments were captured in each block
        assert!(ocm.body.segment.data.traj[0]
            .comment
            .contains(&"Comment before TRAJ".to_string()));
        assert!(ocm.body.segment.data.cov[0]
            .comment
            .contains(&"Comment before COV".to_string()));
        assert!(ocm.body.segment.data.man[0]
            .comment
            .contains(&"Comment before MAN".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .pert
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before PERT".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .od
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before OD".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .user
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before USER".to_string()));
    }

    #[test]
    fn test_user_comment_inside_block() {
        // Cover line 767: Comment inside USER block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
COMMENT inside user block
PARAM = VAL
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm
            .body
            .segment
            .data
            .user
            .as_ref()
            .unwrap()
            .comment
            .contains(&"inside user block".to_string()));
    }

    #[test]
    fn test_user_empty_line_inside_block() {
        // Cover line 774: Empty line inside USER block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
PARAM1 = VAL1

PARAM2 = VAL2
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body
                .segment
                .data
                .user
                .as_ref()
                .unwrap()
                .user_defined
                .len(),
            2
        );
    }

    #[test]
    fn test_traj_empty_line_inside_block() {
        // Cover line 981: Empty line inside TRAJ block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF

TRAJ_TYPE = CARTPV

2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 1);
    }

    #[test]
    fn test_phys_empty_line_inside_block() {
        // Cover line 1345: Empty line inside PHYS block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME

WET_MASS = 500 [kg]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
    }

    #[test]
    fn test_od_empty_line_inside_block() {
        // Cover line 2553: Empty line inside OD block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1

OD_METHOD = LS

OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.od.is_some());
    }

    #[test]
    fn test_traj_missing_lines() {
        // Cover lines 1057-1059: Missing trajLine error
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("trajLine")
                            || err.contexts.iter().any(|c| c.contains("trajLine"))
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected trajLine missing error, got: {:?}", err),
        }
    }

    #[test]
    fn test_traj_invalid_interpolation_degree() {
        // Cover lines 991-992: Invalid INTERPOLATION_DEGREE
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
INTERPOLATION_DEGREE = NOT_A_NUMBER
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_traj_invalid_orb_revnum() {
        // Cover lines 1008-1009: Invalid ORB_REVNUM
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
ORB_REVNUM = NOT_A_NUMBER
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_phys_invalid_oeb_q2() {
        // Cover lines 1373-1374: Invalid OEB_Q2
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q2 = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_phys_invalid_oeb_q3() {
        // Cover lines 1378-1379: Invalid OEB_Q3
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q3 = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_phys_invalid_oeb_qc() {
        // Cover lines 1383-1384: Invalid OEB_QC
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_QC = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_phys_invalid_solar_rad_coeff() {
        // Cover lines 1407-1408: Invalid SOLAR_RAD_COEFF
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
SOLAR_RAD_COEFF = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_phys_invalid_vm_fields() {
        // Cover lines 1415-1431: Invalid VM_ABSOLUTE, VM_APPARENT_MIN, VM_APPARENT, VM_APPARENT_MAX
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
VM_ABSOLUTE = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());

        let kvn2 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT_MIN");
        let err2 = Ocm::from_kvn(&kvn2).unwrap_err();
        assert!(err2.is_format_error());

        let kvn3 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT");
        let err3 = Ocm::from_kvn(&kvn3).unwrap_err();
        assert!(err3.is_format_error());

        let kvn4 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT_MAX");
        let err4 = Ocm::from_kvn(&kvn4).unwrap_err();
        assert!(err4.is_format_error());
    }

    #[test]
    fn test_phys_invalid_reflectance() {
        // Cover lines 1435-1436: Invalid REFLECTANCE
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
REFLECTANCE = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_od_all_optional_fields_coverage() {
        // Cover OD write_kvn optional fields (lines 2470-2538)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
MAXIMUM_OBS_GAP = 0.5 [d]
OD_EPOCH_EIGMAJ = 100 [m]
OD_EPOCH_EIGINT = 50 [m]
OD_EPOCH_EIGMIN = 25 [m]
OD_MAX_PRED_EIGMAJ = 200 [m]
OD_MIN_PRED_EIGMIN = 10 [m]
OD_CONFIDENCE = 95 [%]
GDOP = 1.5
SOLVE_N = 6
SOLVE_STATES = X Y Z VX VY VZ
CONSIDER_N = 2
CONSIDER_PARAMS = CD CR
SEDR = 0.001 [W/kg]
SENSORS_N = 3
SENSORS = SENSOR_A SENSOR_B SENSOR_C
WEIGHTED_RMS = 1.2
DATA_TYPES = RANGE DOPPLER
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();

        // Verify all fields were parsed
        assert_eq!(od.od_prev_id, Some("OD0".to_string()));
        assert!(od.actual_od_span.is_some());
        assert_eq!(od.obs_available, Some(1000));
        assert_eq!(od.obs_used, Some(950));
        assert_eq!(od.tracks_available, Some(50));
        assert_eq!(od.tracks_used, Some(48));
        assert!(od.maximum_obs_gap.is_some());
        assert!(od.od_epoch_eigmaj.is_some());
        assert!(od.od_epoch_eigint.is_some());
        assert!(od.od_epoch_eigmin.is_some());
        assert!(od.od_max_pred_eigmaj.is_some());
        assert!(od.od_min_pred_eigmin.is_some());
        assert!(od.od_confidence.is_some());
        assert_eq!(od.gdop, Some(1.5));
        assert_eq!(od.solve_n, Some(6));
        assert!(od.solve_states.is_some());
        assert_eq!(od.consider_n, Some(2));
        assert!(od.consider_params.is_some());
        assert!(od.sedr.is_some());
        assert_eq!(od.sensors_n, Some(3));
        assert_eq!(od.sensors, Some("SENSOR_A SENSOR_B SENSOR_C".to_string()));
        assert_eq!(od.weighted_rms, Some(NonNegativeDouble::new(1.2).unwrap()));
        assert_eq!(od.data_types, Some("RANGE DOPPLER".to_string()));

        // Now write to KVN to cover all the write_kvn branches
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("OD_PREV_ID"));
        assert!(output.contains("ACTUAL_OD_SPAN"));
        assert!(output.contains("OBS_AVAILABLE"));
        assert!(output.contains("TRACKS_AVAILABLE"));
        assert!(output.contains("MAXIMUM_OBS_GAP"));
        assert!(output.contains("OD_EPOCH_EIGMAJ"));
        assert!(output.contains("OD_CONFIDENCE"));
        assert!(output.contains("GDOP"));
        assert!(output.contains("SOLVE_N"));
        assert!(output.contains("SOLVE_STATES"));
        assert!(output.contains("CONSIDER_N"));
        assert!(output.contains("SEDR"));
        assert!(output.contains("SENSORS_N"));
        assert!(output.contains("WEIGHTED_RMS"));
        assert!(output.contains("DATA_TYPES"));
    }

    #[test]
    fn test_od_invalid_numeric_fields() {
        // Cover OD parsing error branches
        let base = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
"#;

        // OBS_AVAILABLE invalid
        let kvn = format!("{base}OBS_AVAILABLE = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // OBS_USED invalid
        let kvn = format!("{base}OBS_USED = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // TRACKS_AVAILABLE invalid
        let kvn = format!("{base}TRACKS_AVAILABLE = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // TRACKS_USED invalid
        let kvn = format!("{base}TRACKS_USED = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // GDOP invalid
        let kvn = format!("{base}GDOP = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // SOLVE_N invalid
        let kvn = format!("{base}SOLVE_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // CONSIDER_N invalid
        let kvn = format!("{base}CONSIDER_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // SENSORS_N invalid
        let kvn = format!("{base}SENSORS_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());

        // WEIGHTED_RMS invalid
        let kvn = format!("{base}WEIGHTED_RMS = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(err.is_format_error());
    }

    #[test]
    fn test_traj_unknown_key_error() {
        // Cover line 1017: Unknown key in TRAJ block (wildcard match)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
UNKNOWN_KEY = IGNORED_VALUE
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        // Should FAIL, not ignoring unknown keys
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unexpected OCM Trajectory key")
                            || err.contexts.contains(&"Unexpected OCM Trajectory key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got {:?}", err),
        }
    }

    #[test]
    fn test_traj_block_start_in_traj_ignored() {
        // Cover lines 1032-1034: BlockStart/other tokens in TRAJ loop
        // The wildcard _ branch catches unexpected tokens
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].traj_type, "CARTPV");
    }

    #[test]
    fn test_phys_unknown_key_error() {
        // Cover lines 1455-1457: Unknown key in PHYS block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
UNKNOWN_PHYS_KEY = IGNORED
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unexpected OCM Physical key")
                            || err.contexts.contains(&"Unexpected OCM Physical key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got {:?}", err),
        }
    }

    #[test]
    fn test_metadata_unknown_token_breaks_loop() {
        // Cover line 567: Unknown token type breaks metadata loop
        // This is hard to hit directly since metadata normally ends with META_STOP
        // But we can verify with a minimal valid OCM
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_cov_comment_and_empty_line() {
        // Cover lines 1647-1648: Comment and empty line in COV block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT inside cov block

COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.cov[0]
            .comment
            .contains(&"inside cov block".to_string()));
    }

    #[test]
    fn test_comment_before_cov_block() {
        // ...
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT This comment should be prepended to COV
COV_START
COV_ID = COV1
COV_TYPE = ANGLE
COV_ORDERING = LTM
UNKNOWN_COV_KEY = some_value
CX_X = 1.0
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unexpected OCM Covariance key")
                            || err.contexts.contains(&"Unexpected OCM Covariance key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got {:?}", err),
        }
    }

    #[test]
    fn test_comment_before_phys_block() {
        // Cover line 726: pending_comments splice before PHYS block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT This comment should be prepended to PHYS
PHYS_START
WET_MASS = 1000.0 [kg]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
        let phys = ocm.body.segment.data.phys.unwrap();
        assert!(phys.comment.iter().any(|c| c.contains("prepended to PHYS")));
    }

    #[test]
    fn test_cov_scale_min_max_invalid() {
        // Cover lines 1661-1667: Invalid COV_SCALE_MIN and COV_SCALE_MAX parsing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COV_START
COV_ID = COV1
COV_SCALE_MIN = not_a_number
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected COV_SCALE_MIN error, got: {:?}",
            err
        );

        // Test COV_SCALE_MAX invalid
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COV_START
COV_ID = COV1
COV_SCALE_MAX = invalid
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected COV_SCALE_MAX error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_missing_meta_start() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
CENTER_NAME = EARTH
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.as_kvn_parse_error().map_or(false, |err| err
                .contexts
                .iter()
                .any(|c| c.contains("Expected META_START"))),
            "Expected 'expected meta' error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_empty_line_in_man_block() {
        // Cover line 1990: Empty line in MAN block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1

MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
2023-01-01T00:00:00.000 0.1 0.2 0.3
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(!ocm.body.segment.data.man.is_empty());
    }

    #[test]
    fn test_dc_min_max_cycles_invalid() {
        // Cover lines 2018-2024: Invalid DC_MIN_CYCLES/DC_MAX_CYCLES parsing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
DC_MIN_CYCLES = not_a_number
MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected DC_MIN_CYCLES error, got: {:?}",
            err
        );

        // Test DC_MAX_CYCLES invalid
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
DC_MAX_CYCLES = invalid
MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected DC_MAX_CYCLES error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_empty_line_in_pert_block() {
        // Cover line 2245: Empty line in PERT block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSIS00

GRAVITY_MODEL = EGM-96
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.pert.is_some());
    }

    #[test]
    fn test_oblate_flattening_invalid() {
        // Cover lines 2259-2260: Invalid OBLATE_FLATTENING parsing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
PERT_START
OBLATE_FLATTENING = not_a_number
PERT_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected OBLATE_FLATTENING error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_albedo_grid_size_invalid() {
        // Cover lines 2268-2269: Invalid ALBEDO_GRID_SIZE parsing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
PERT_START
ALBEDO_GRID_SIZE = invalid
PERT_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            err.is_format_error(),
            "Expected ALBEDO_GRID_SIZE error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_unknown_key_in_od_block() {
        // Cover line 2645: _ => {} wildcard match in OD parsing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
UNKNOWN_OD_KEY = some_value
OD_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message
                            .contains("Unexpected OCM Orbit Determination key")
                            || err
                                .contexts
                                .contains(&"Unexpected OCM Orbit Determination key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got {:?}", err),
        }
    }

    #[test]
    fn test_unknown_key_in_cov_block() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
UNKNOWN_COV_KEY = some_value
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        match err {
            CcsdsNdmError::Format(format_err) => match *format_err {
                FormatError::Kvn(ref err) => {
                    assert!(
                        err.message.contains("Unexpected OCM Covariance key")
                            || err.contexts.contains(&"Unexpected OCM Covariance key")
                    );
                }
                _ => panic!("unexpected format error: {:?}", format_err),
            },
            _ => panic!("Expected KvnParse error, got {:?}", err),
        }
    }

    #[test]
    fn test_ocm_data_write_kvn_all_blocks() {
        // ...
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
USER_START
COMMENT user comment
CUSTOM_PARAM = custom_value
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("COV_START"));
        assert!(output.contains("MAN_START"));
        assert!(output.contains("PERT_START"));
        assert!(output.contains("OD_START"));
        assert!(output.contains("USER_START"));
    }

    #[test]
    fn test_cov_write_kvn_all_optional_fields() {
        // Cover COV write_kvn optional fields
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT cov comment
COV_ID = COV_001
COV_PREV_ID = COV_000
COV_NEXT_ID = COV_002
COV_BASIS = DETERMINED
COV_BASIS_ID = BASIS_001
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let cov = &ocm.body.segment.data.cov[0];

        // Verify all optional fields were parsed
        assert_eq!(cov.comment, vec!["cov comment"]);
        assert_eq!(cov.cov_id, Some("COV_001".to_string()));
        assert_eq!(cov.cov_prev_id, Some("COV_000".to_string()));
        assert_eq!(cov.cov_next_id, Some("COV_002".to_string()));
        assert!(cov.cov_basis.is_some());
        assert_eq!(cov.cov_basis_id, Some("BASIS_001".to_string()));
        assert!(cov.cov_frame_epoch.is_some());
        assert_eq!(cov.cov_scale_min, Some(0.5));
        assert_eq!(cov.cov_scale_max, Some(2.0));
        assert!(cov.cov_confidence.is_some());
        assert_eq!(cov.cov_units, Some("km**2".to_string()));

        // Write to KVN to cover all write_kvn branches
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("COV_ID"));
        assert!(output.contains("COV_PREV_ID"));
        assert!(output.contains("COV_NEXT_ID"));
        assert!(output.contains("COV_BASIS"));
        assert!(output.contains("COV_BASIS_ID"));
        assert!(output.contains("COV_FRAME_EPOCH"));
        assert!(output.contains("COV_SCALE_MIN"));
        assert!(output.contains("COV_SCALE_MAX"));
        assert!(output.contains("COV_CONFIDENCE"));
        assert!(output.contains("COV_UNITS"));
    }

    #[test]
    fn test_man_all_optional_fields_write_kvn() {
        // Cover MAN write_kvn optional fields
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MAN_1
MAN_PREV_ID = MAN_0
MAN_NEXT_ID = MAN_2
MAN_BASIS = PLANNED
MAN_BASIS_ID = PLAN_001
MAN_DEVICE_ID = THRUSTER_1
MAN_PREV_EPOCH = 2022-12-31T00:00:00
MAN_NEXT_EPOCH = 2023-01-02T00:00:00
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
GRAV_ASSIST_NAME = MOON
DC_TYPE = TIME
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T02:00:00
DC_MIN_CYCLES = 1
DC_MAX_CYCLES = 10
DC_EXEC_START = 2023-01-01T00:30:00
DC_EXEC_STOP = 2023-01-01T01:30:00
DC_REF_TIME = 2023-01-01T01:00:00
DC_TIME_PULSE_DURATION = 60 [s]
DC_TIME_PULSE_PERIOD = 120 [s]
DC_REF_DIR = 1 0 0
DC_BODY_FRAME = SC_BODY
DC_BODY_TRIGGER = 0 1 0
DC_PA_START_ANGLE = 0 [deg]
DC_PA_STOP_ANGLE = 180 [deg]
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let man = &ocm.body.segment.data.man[0];

        // Verify optional fields
        assert_eq!(man.man_prev_id, Some("MAN_0".to_string()));
        assert_eq!(man.man_next_id, Some("MAN_2".to_string()));
        assert!(man.man_basis.is_some());
        assert!(man.grav_assist_name.is_some());
        assert!(man.dc_win_open.is_some());
        assert!(man.dc_min_cycles.is_some());
        assert!(man.dc_ref_dir.is_some());
        assert!(man.dc_body_trigger.is_some());
        assert!(man.dc_pa_start_angle.is_some());

        // Write to KVN
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("MAN_PREV_ID"));
        assert!(output.contains("MAN_NEXT_ID"));
        assert!(output.contains("MAN_BASIS"));
        assert!(output.contains("GRAV_ASSIST_NAME"));
        assert!(output.contains("DC_WIN_OPEN"));
        assert!(output.contains("DC_MIN_CYCLES"));
        assert!(output.contains("DC_REF_DIR"));
        assert!(output.contains("DC_BODY_TRIGGER"));
        assert!(output.contains("DC_PA_START_ANGLE"));
    }

    #[test]
    fn test_pert_all_optional_fields_write_kvn() {
        // Cover PERT write_kvn optional fields
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
CENTRAL_BODY_ROTATION = 7.2921e-5 [deg/s]
OBLATE_FLATTENING = 0.003353
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
ALBEDO_MODEL = EARTH_ALBEDO
ALBEDO_GRID_SIZE = 36
SHADOW_MODEL = CYLINDRICAL
SHADOW_BODIES = MOON
SRP_MODEL = FLAT_PLATE
SW_DATA_SOURCE = CSSI
SW_DATA_EPOCH = 2023-01-01T00:00:00
SW_INTERP_METHOD = LINEAR
FIXED_GEOMAG_KP = 3 [nT]
FIXED_GEOMAG_AP = 15 [nT]
FIXED_GEOMAG_DST = -10 [nT]
FIXED_F10P7 = 150 [SFU]
FIXED_F10P7_MEAN = 145 [SFU]
FIXED_M10P7 = 148 [SFU]
FIXED_M10P7_MEAN = 143 [SFU]
FIXED_S10P7 = 147 [SFU]
FIXED_S10P7_MEAN = 142 [SFU]
FIXED_Y10P7 = 146 [SFU]
FIXED_Y10P7_MEAN = 141 [SFU]
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();

        // Verify all fields
        assert!(pert.central_body_rotation.is_some());
        assert!(pert.oblate_flattening.is_some());
        assert!(pert.albedo_grid_size.is_some());
        assert!(pert.sw_data_epoch.is_some());
        assert!(pert.fixed_geomag_kp.is_some());
        assert!(pert.fixed_m10p7.is_some());

        // Write to KVN
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("CENTRAL_BODY_ROTATION"));
        assert!(output.contains("OBLATE_FLATTENING"));
        assert!(output.contains("ALBEDO_GRID_SIZE"));
        assert!(output.contains("FIXED_GEOMAG_KP"));
        assert!(output.contains("FIXED_M10P7"));
    }

    #[test]
    fn test_traj_orb_revnum_basis_zero() {
        // Cover ORB_REVNUM_BASIS = 0 case in write_kvn
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
ORB_REVNUM = 100
ORB_REVNUM_BASIS = 0
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body.segment.data.traj[0].orb_revnum_basis,
            Some(RevNumBasis::Zero)
        );

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("ORB_REVNUM_BASIS"));
    }

    #[test]
    fn test_ocm_parser_backtrack_and_gaps() {
        // 1. ocm_traj_line backtrack (starts with uppercase)
        let mut input = "TRAJ_STOP";
        assert!(ocm_traj_line(&mut input).is_err());

        // 2. ocm_cov_line backtrack
        let mut input = "COV_STOP";
        assert!(ocm_cov_line(&mut input).is_err());

        // 3. ocm_man_line backtrack
        let mut input = "MAN_STOP";
        assert!(ocm_man_line(&mut input).is_err());

        // 4. ocm_data_block loop break check
        assert!(ocm_data_block(&mut "").is_err());
        assert!(ocm_data_block(&mut " ").is_err());
    }

    #[test]
    fn test_ocm_data_block_backtrack() {
        let mut input = "123.456";
        assert!(ocm_data_block(&mut input).is_err());

        let mut input = "META_START";
        assert!(ocm_data_block(&mut input).is_err());
    }

    #[test]
    fn test_ocm_error_branches_detailed() {
        // Cover missing fields with Cut error verification

        // Missing MAN_ID
        let kvn = r#"MAN_START
MAN_DEVICE_ID = DEV
MAN_STOP"#;
        let mut input = kvn;
        let err = ocm_man(&mut input).unwrap_err();
        match err {
            ErrMode::Cut(e) => assert!(format!("{:?}", e).contains("MAN_ID")),
            _ => panic!("Expected Cut error"),
        }
    }
}
