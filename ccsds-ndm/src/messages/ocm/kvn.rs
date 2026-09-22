// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OCM (Orbit Comprehensive Message).
//!
//! This module implements KVN parsing for OCM using winnow parser combinators.

use super::{
    CovLine, ManLine, Ocm, OcmBody, OcmCovarianceMatrix, OcmData, OcmManeuverParameters,
    OcmMetadata, OcmOdParameters, OcmPerturbations, OcmPhysicalDescription, OcmSegment,
    OcmTrajState, TrajLine,
};
use crate::common::OdmHeader;
use crate::error::{CcsdsNdmError, FormatError, InternalParserError, KvnParseError, Result};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;
use crate::types::*;
use winnow::ascii::{space1, till_line_ending};
use winnow::combinator::{peek, repeat};
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
        "EPOCH_TZERO" => epoch_tzero: kv_calendar_epoch,
        "OPS_STATUS" => ops_status: kv_string,
        "ORBIT_CATEGORY" => orbit_category: kv_string,
        "OCM_DATA_ELEMENTS" => ocm_data_elements: kv_string,
        "SCLK_OFFSET_AT_EPOCH" => sclk_offset_at_epoch: kv_from_kvn,
        "SCLK_SEC_PER_SI_SEC" => sclk_sec_per_si_sec: kv_from_kvn,
        "PREVIOUS_MESSAGE_EPOCH" => previous_message_epoch: kv_calendar_epoch,
        "NEXT_MESSAGE_EPOCH" => next_message_epoch: kv_calendar_epoch,
        "START_TIME" => start_time: kv_epoch,
        "STOP_TIME" => stop_time: kv_epoch,
        "TIME_SPAN" => time_span: kv_from_kvn,
        "TAIMUTC_AT_TZERO" => taimutc_at_tzero: kv_from_kvn,
        "NEXT_LEAP_EPOCH" => next_leap_epoch: kv_calendar_epoch,
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
        // These fields are conditional in ODM table 6-3. Preserve omission so validation can
        // enforce the SCLK-only requirement instead of manufacturing values on the KVN path.
        sclk_offset_at_epoch,
        sclk_sec_per_si_sec,
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
    let epoch = kv_epoch_token.parse_next(input)?;
    space1.parse_next(input)?;
    let raw_values = till_line_ending.parse_next(input)?;
    let mut values = Vec::with_capacity(raw_values.split_whitespace().count());
    for value in raw_values.split_whitespace() {
        values.push(
            fast_float::parse(value)
                .map_err(|_| cut_err(input, "Malformed OCM trajectory numeric value"))?,
        );
    }
    if values.is_empty() {
        return Err(cut_err(input, "Missing OCM trajectory values"));
    }
    opt_line_ending.parse_next(input)?;
    Ok(TrajLine { epoch, values })
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
        "TRAJ_FRAME_EPOCH" => traj_frame_epoch: kv_calendar_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_calendar_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_calendar_epoch,
        "ORB_REVNUM" => orb_revnum: kv_float,
        "ORB_REVNUM_BASIS" => orb_revnum_basis: kv_enum,
        "TRAJ_TYPE" => traj_type: kv_string,
        "ORB_AVERAGING" => orb_averaging: kv_string,
        "TRAJ_UNITS" => traj_units: kv_string,
    }, |i| at_block_end("TRAJ", i), "Unexpected OCM Trajectory key");

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("TRAJ", input) {
            expect_block_end("TRAJ").parse_next(input)?;
            break;
        }
        if input.is_empty() {
            return Err(cut_err(input, "Missing TRAJ_STOP before end of input"));
        }

        let checkpoint = input.checkpoint();
        if let Ok(line) = ocm_traj_line.parse_next(input) {
            traj_lines.push(line);
            continue;
        }

        input.reset(&checkpoint);
        let raw = peek(till_line_ending).parse_next(input)?;
        if !raw.trim().is_empty() {
            return Err(cut_err(input, "Malformed OCM trajectory record"));
        }
        let remaining = input.len();
        let _ = till_line_ending.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        if input.len() == remaining {
            return Err(cut_err(input, "OCM trajectory parser made no progress"));
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
        "OEB_PARENT_FRAME_EPOCH" => oeb_parent_frame_epoch: kv_calendar_epoch,
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
    let epoch = kv_epoch_token.parse_next(input)?;
    space1.parse_next(input)?;
    let raw_values = till_line_ending.parse_next(input)?;
    let mut values = Vec::with_capacity(raw_values.split_whitespace().count());
    for value in raw_values.split_whitespace() {
        values.push(
            fast_float::parse(value)
                .map_err(|_| cut_err(input, "Malformed OCM covariance numeric value"))?,
        );
    }
    if values.is_empty() {
        return Err(cut_err(input, "Missing OCM covariance values"));
    }
    opt_line_ending.parse_next(input)?;
    Ok(CovLine { epoch, values })
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
        "COV_FRAME_EPOCH" => cov_frame_epoch: kv_calendar_epoch,
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
    let epoch = kv_epoch_token.parse_next(input)?;
    let values =
        repeat(1.., (space1, till_space_or_eol).map(|(_, v)| v.to_string())).parse_next(input)?;
    opt_line_ending.parse_next(input)?;
    Ok(ManLine { epoch, values })
}

fn kv_man_composition(input: &mut &str) -> KvnResult<String> {
    let mut composition = kv_string.parse_next(input)?;
    loop {
        let trimmed = composition.trim_end();
        if !trimmed.ends_with(',') && !trimmed.ends_with("<cont.>") {
            break;
        }

        let continuation = peek(till_line_ending).parse_next(input)?.trim();
        if continuation.is_empty() || continuation.contains('=') || at_block_end("MAN", input) {
            return Err(cut_err(input, "Malformed MAN_COMPOSITION continuation"));
        }
        let _ = till_line_ending.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        let prefix = trimmed
            .strip_suffix("<cont.>")
            .unwrap_or(trimmed)
            .trim_end();
        composition = format!("{prefix} {continuation}");
    }
    Ok(composition)
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
        "MAN_FRAME_EPOCH" => man_frame_epoch: kv_calendar_epoch,
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
        "MAN_COMPOSITION" => man_composition: kv_man_composition,
        "MAN_UNITS" => man_units: kv_string,
    }, |i| at_block_end("MAN", i), "Unexpected OCM Maneuver key");

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        if at_block_end("MAN", input) {
            expect_block_end("MAN").parse_next(input)?;
            break;
        }
        if input.is_empty() {
            return Err(cut_err(input, "Missing MAN_STOP before end of input"));
        }

        let checkpoint = input.checkpoint();
        if let Ok(line) = ocm_man_line.parse_next(input) {
            man_lines.push(line);
            continue;
        }

        input.reset(&checkpoint);
        let raw = peek(till_line_ending).parse_next(input)?;
        if !raw.trim().is_empty() {
            return Err(cut_err(input, "Malformed OCM maneuver record"));
        }
        let remaining = input.len();
        let _ = till_line_ending.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        if input.len() == remaining {
            return Err(cut_err(input, "OCM maneuver parser made no progress"));
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

    parse_block!(input, comment, {
        "OD_ID" => od_id: kv_string,
        "OD_PREV_ID" => od_prev_id: kv_string,
        "OD_METHOD" => od_method: kv_string,
        "OD_EPOCH" => od_epoch: kv_epoch,
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

// The inline Epoch comparison key intentionally keeps history records allocation-free.  That
// makes the MAN variant large enough to trigger Clippy's layout heuristic; boxing it would add a
// heap allocation for every parsed block, which is worse for the hot KVN path.
#[allow(clippy::large_enum_variant)]
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
                creation_date = Some(kv_calendar_epoch.parse_next(input)?);
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

fn ocm_kvn_keys(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        "META" => &[
            "OBJECT_NAME",
            "INTERNATIONAL_DESIGNATOR",
            "CATALOG_NAME",
            "OBJECT_DESIGNATOR",
            "ALTERNATE_NAMES",
            "ORIGINATOR_POC",
            "ORIGINATOR_POSITION",
            "ORIGINATOR_PHONE",
            "ORIGINATOR_EMAIL",
            "ORIGINATOR_ADDRESS",
            "TECH_ORG",
            "TECH_POC",
            "TECH_POSITION",
            "TECH_PHONE",
            "TECH_EMAIL",
            "TECH_ADDRESS",
            "PREVIOUS_MESSAGE_ID",
            "NEXT_MESSAGE_ID",
            "ADM_MSG_LINK",
            "CDM_MSG_LINK",
            "PRM_MSG_LINK",
            "RDM_MSG_LINK",
            "TDM_MSG_LINK",
            "OPERATOR",
            "OWNER",
            "COUNTRY",
            "CONSTELLATION",
            "OBJECT_TYPE",
            "TIME_SYSTEM",
            "EPOCH_TZERO",
            "OPS_STATUS",
            "ORBIT_CATEGORY",
            "OCM_DATA_ELEMENTS",
            "SCLK_OFFSET_AT_EPOCH",
            "SCLK_SEC_PER_SI_SEC",
            "PREVIOUS_MESSAGE_EPOCH",
            "NEXT_MESSAGE_EPOCH",
            "START_TIME",
            "STOP_TIME",
            "TIME_SPAN",
            "TAIMUTC_AT_TZERO",
            "NEXT_LEAP_EPOCH",
            "NEXT_LEAP_TAIMUTC",
            "UT1MUTC_AT_TZERO",
            "EOP_SOURCE",
            "INTERP_METHOD_EOP",
            "CELESTIAL_SOURCE",
        ],
        "TRAJ" => &[
            "TRAJ_ID",
            "TRAJ_PREV_ID",
            "TRAJ_NEXT_ID",
            "TRAJ_BASIS",
            "TRAJ_BASIS_ID",
            "INTERPOLATION",
            "INTERPOLATION_DEGREE",
            "PROPAGATOR",
            "CENTER_NAME",
            "TRAJ_REF_FRAME",
            "TRAJ_FRAME_EPOCH",
            "USEABLE_START_TIME",
            "USEABLE_STOP_TIME",
            "ORB_REVNUM",
            "ORB_REVNUM_BASIS",
            "TRAJ_TYPE",
            "ORB_AVERAGING",
            "TRAJ_UNITS",
        ],
        "PHYS" => &[
            "MANUFACTURER",
            "BUS_MODEL",
            "DOCKED_WITH",
            "DRAG_CONST_AREA",
            "DRAG_COEFF_NOM",
            "DRAG_UNCERTAINTY",
            "INITIAL_WET_MASS",
            "WET_MASS",
            "DRY_MASS",
            "OEB_PARENT_FRAME",
            "OEB_PARENT_FRAME_EPOCH",
            "OEB_Q1",
            "OEB_Q2",
            "OEB_Q3",
            "OEB_QC",
            "OEB_MAX",
            "OEB_INT",
            "OEB_MIN",
            "AREA_ALONG_OEB_MAX",
            "AREA_ALONG_OEB_INT",
            "AREA_ALONG_OEB_MIN",
            "AREA_MIN_FOR_PC",
            "AREA_MAX_FOR_PC",
            "AREA_TYP_FOR_PC",
            "RCS",
            "RCS_MIN",
            "RCS_MAX",
            "SRP_CONST_AREA",
            "SOLAR_RAD_COEFF",
            "SOLAR_RAD_UNCERTAINTY",
            "VM_ABSOLUTE",
            "VM_APPARENT_MIN",
            "VM_APPARENT",
            "VM_APPARENT_MAX",
            "REFLECTANCE",
            "ATT_CONTROL_MODE",
            "ATT_ACTUATOR_TYPE",
            "ATT_KNOWLEDGE",
            "ATT_CONTROL",
            "ATT_POINTING",
            "AVG_MANEUVER_FREQ",
            "MAX_THRUST",
            "DV_BOL",
            "DV_REMAINING",
            "IXX",
            "IYY",
            "IZZ",
            "IXY",
            "IXZ",
            "IYZ",
        ],
        "COV" => &[
            "COV_ID",
            "COV_PREV_ID",
            "COV_NEXT_ID",
            "COV_BASIS",
            "COV_BASIS_ID",
            "COV_REF_FRAME",
            "COV_FRAME_EPOCH",
            "COV_SCALE_MIN",
            "COV_SCALE_MAX",
            "COV_CONFIDENCE",
            "COV_TYPE",
            "COV_ORDERING",
            "COV_UNITS",
        ],
        "MAN" => &[
            "MAN_ID",
            "MAN_PREV_ID",
            "MAN_NEXT_ID",
            "MAN_BASIS",
            "MAN_BASIS_ID",
            "MAN_DEVICE_ID",
            "MAN_PREV_EPOCH",
            "MAN_NEXT_EPOCH",
            "MAN_PURPOSE",
            "MAN_PRED_SOURCE",
            "MAN_REF_FRAME",
            "MAN_FRAME_EPOCH",
            "GRAV_ASSIST_NAME",
            "DC_TYPE",
            "DC_WIN_OPEN",
            "DC_WIN_CLOSE",
            "DC_MIN_CYCLES",
            "DC_MAX_CYCLES",
            "DC_EXEC_START",
            "DC_EXEC_STOP",
            "DC_REF_TIME",
            "DC_TIME_PULSE_DURATION",
            "DC_TIME_PULSE_PERIOD",
            "DC_REF_DIR",
            "DC_BODY_FRAME",
            "DC_BODY_TRIGGER",
            "DC_PA_START_ANGLE",
            "DC_PA_STOP_ANGLE",
            "MAN_COMPOSITION",
            "MAN_UNITS",
        ],
        "PERT" => &[
            "ATMOSPHERIC_MODEL",
            "GRAVITY_MODEL",
            "EQUATORIAL_RADIUS",
            "GM",
            "N_BODY_PERTURBATIONS",
            "CENTRAL_BODY_ROTATION",
            "OBLATE_FLATTENING",
            "OCEAN_TIDES_MODEL",
            "SOLID_TIDES_MODEL",
            "REDUCTION_THEORY",
            "ALBEDO_MODEL",
            "ALBEDO_GRID_SIZE",
            "SHADOW_MODEL",
            "SHADOW_BODIES",
            "SRP_MODEL",
            "SW_DATA_SOURCE",
            "SW_DATA_EPOCH",
            "SW_INTERP_METHOD",
            "FIXED_GEOMAG_KP",
            "FIXED_GEOMAG_AP",
            "FIXED_GEOMAG_DST",
            "FIXED_F10P7",
            "FIXED_F10P7_MEAN",
            "FIXED_M10P7",
            "FIXED_M10P7_MEAN",
            "FIXED_S10P7",
            "FIXED_S10P7_MEAN",
            "FIXED_Y10P7",
            "FIXED_Y10P7_MEAN",
        ],
        "OD" => &[
            "OD_ID",
            "OD_PREV_ID",
            "OD_METHOD",
            "OD_EPOCH",
            "DAYS_SINCE_FIRST_OBS",
            "DAYS_SINCE_LAST_OBS",
            "RECOMMENDED_OD_SPAN",
            "ACTUAL_OD_SPAN",
            "OBS_AVAILABLE",
            "OBS_USED",
            "TRACKS_AVAILABLE",
            "TRACKS_USED",
            "MAXIMUM_OBS_GAP",
            "OD_EPOCH_EIGMAJ",
            "OD_EPOCH_EIGINT",
            "OD_EPOCH_EIGMIN",
            "OD_MAX_PRED_EIGMAJ",
            "OD_MIN_PRED_EIGMIN",
            "OD_CONFIDENCE",
            "GDOP",
            "SOLVE_N",
            "SOLVE_STATES",
            "CONSIDER_N",
            "CONSIDER_PARAMS",
            "SEDR",
            "SENSORS_N",
            "SENSORS",
            "WEIGHTED_RMS",
            "DATA_TYPES",
        ],
        "USER" => &[],
        _ => return None,
    })
}

pub(super) fn validate_syntax(kvn: &str) -> Result<()> {
    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating OCM KVN structure"],
            offset,
        }))))
    };
    let block_rank = |block: &str| match block {
        "META" => Some((0u8, false)),
        "TRAJ" => Some((1, true)),
        "PHYS" => Some((2, false)),
        "COV" => Some((3, true)),
        "MAN" => Some((4, true)),
        "PERT" => Some((5, false)),
        "OD" => Some((6, false)),
        "USER" => Some((7, false)),
        _ => None,
    };
    let mut current_block: Option<&str> = None;
    let mut previous_key = None;
    let mut last_block_rank = None;
    let mut block_has_content = false;
    let mut history_started = false;
    let mut pending_block_comment = false;
    let mut top_rank = None;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
        if line.as_bytes().contains(&b'\r') {
            return fail("lone carriage return");
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
            if block_has_content {
                return fail("COMMENT is not at the beginning of a logical block");
            }
            if current_block.is_none()
                && top_rank != Some(0)
                && top_rank.is_none_or(|rank| rank < 3)
            {
                return fail("COMMENT is not at the beginning of the header or a data block");
            }
            pending_block_comment =
                current_block.is_none() && top_rank.is_some_and(|rank| rank >= 3);
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if current_block.is_some() {
                return if current_block == Some("USER") {
                    fail("Unexpected key in USER block")
                } else {
                    fail("nested OCM marked block")
                };
            }
            let (rank, repeatable) = block_rank(marker)
                .ok_or_else(|| invalid(number, offset, "unknown OCM block".into()))?;
            if last_block_rank.is_some_and(|last| rank < last || (rank == last && !repeatable)) {
                return fail("duplicate or out-of-order OCM block");
            }
            if marker != "META" && last_block_rank.is_none() {
                return fail("Expected META_START");
            }
            current_block = Some(marker);
            previous_key = None;
            block_has_content = false;
            history_started = false;
            pending_block_comment = false;
            last_block_rank = Some(rank);
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if current_block != Some(marker) {
                return fail("mismatched or unexpected OCM block end");
            }
            current_block = None;
            previous_key = None;
            block_has_content = false;
            history_started = false;
            offset += raw_line.len() + 1;
            continue;
        }

        if let Some(block) = current_block {
            if !line.contains('=') {
                if !matches!(block, "TRAJ" | "COV" | "MAN") {
                    return fail("unexpected non-assignment in OCM logical block");
                }
                block_has_content = true;
                history_started = true;
                offset += raw_line.len() + 1;
                continue;
            }
            if !line.contains('=') {
                return fail("expected an assignment");
            }
            let key = line.split_once('=').unwrap().0.trim();
            if block == "USER" {
                if !key.starts_with("USER_DEFINED_") || key.len() == "USER_DEFINED_".len() {
                    return fail("unknown OCM USER keyword");
                }
                block_has_content = true;
                offset += raw_line.len() + 1;
                continue;
            }
            let keys = ocm_kvn_keys(block).unwrap();
            let rank = keys
                .iter()
                .position(|candidate| *candidate == key)
                .ok_or_else(|| {
                    let block_name = match block {
                        "META" => "Metadata",
                        "TRAJ" => "Trajectory",
                        "PHYS" => "Physical",
                        "COV" => "Covariance",
                        "MAN" => "Maneuver",
                        "PERT" => "Perturbations",
                        "OD" => "Orbit Determination",
                        _ => block,
                    };
                    invalid(number, offset, format!("Unexpected OCM {block_name} key"))
                })?;
            if previous_key.is_some_and(|previous| rank <= previous) {
                return fail("duplicate or out-of-order OCM block keyword");
            }
            if history_started && !(block == "MAN" && key == "MAN_UNITS") {
                return fail("assignment after OCM history data");
            }
            previous_key = Some(rank);
            block_has_content = true;
        } else {
            if pending_block_comment {
                return fail("Unexpected OCM Data key");
            }
            if !line.contains('=') {
                return fail("expected exactly one top-level assignment");
            }
            let key = line.split_once('=').unwrap().0.trim();
            let rank = match key {
                "CCSDS_OCM_VERS" => 0,
                "CLASSIFICATION" => 1,
                "CREATION_DATE" => 2,
                "ORIGINATOR" => 3,
                "MESSAGE_ID" => 4,
                _ if last_block_rank.is_none() => return fail("Expected META_START"),
                _ => return fail("Unexpected OCM Data key"),
            };
            if top_rank.is_some_and(|previous| rank <= previous) || last_block_rank.is_some() {
                return fail("duplicate or out-of-order OCM top-level keyword");
            }
            top_rank = Some(rank);
        }
        offset += raw_line.len() + 1;
    }
    if current_block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unclosed OCM marked block".into(),
        ));
    }
    Ok(())
}

impl Ocm {
    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        for (trajectory_index, trajectory) in self.body.segment.data.traj.iter().enumerate() {
            for (line_index, line) in trajectory.traj_lines.iter().enumerate() {
                for (value_index, value) in line.values.iter().enumerate() {
                    if !crate::kvn::ser::OdmFloat::is_valid(*value) {
                        return Err(crate::validation::unrepresentable_number(
                            "trajLine",
                            *value,
                            format!(
                                "body.segment.data.traj[{trajectory_index}].traj_lines[{line_index}].values[{value_index}]"
                            ),
                        ));
                    }
                }
            }
        }
        for (covariance_index, covariance) in self.body.segment.data.cov.iter().enumerate() {
            for (line_index, line) in covariance.cov_lines.iter().enumerate() {
                for (value_index, value) in line.values.iter().enumerate() {
                    if !crate::kvn::ser::OdmFloat::is_valid(*value) {
                        return Err(crate::validation::unrepresentable_number(
                            "covLine",
                            *value,
                            format!(
                                "body.segment.data.cov[{covariance_index}].cov_lines[{line_index}].values[{value_index}]"
                            ),
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

impl ToKvn for Ocm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.allow_long_records();
        writer.write_pair("CCSDS_OCM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for OcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for OcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for OcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.object_name {
            writer.write_pair("OBJECT_NAME", v);
        }
        if let Some(v) = &self.international_designator {
            writer.write_pair("INTERNATIONAL_DESIGNATOR", v);
        }
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(v) = &self.alternate_names {
            writer.write_pair("ALTERNATE_NAMES", v);
        }
        if let Some(v) = &self.originator_poc {
            writer.write_pair("ORIGINATOR_POC", v);
        }
        if let Some(v) = &self.originator_position {
            writer.write_pair("ORIGINATOR_POSITION", v);
        }
        if let Some(v) = &self.originator_phone {
            writer.write_pair("ORIGINATOR_PHONE", v);
        }
        if let Some(v) = &self.originator_email {
            writer.write_pair("ORIGINATOR_EMAIL", v);
        }
        if let Some(v) = &self.originator_address {
            writer.write_pair("ORIGINATOR_ADDRESS", v);
        }
        if let Some(v) = &self.tech_org {
            writer.write_pair("TECH_ORG", v);
        }
        if let Some(v) = &self.tech_poc {
            writer.write_pair("TECH_POC", v);
        }
        if let Some(v) = &self.tech_position {
            writer.write_pair("TECH_POSITION", v);
        }
        if let Some(v) = &self.tech_phone {
            writer.write_pair("TECH_PHONE", v);
        }
        if let Some(v) = &self.tech_email {
            writer.write_pair("TECH_EMAIL", v);
        }
        if let Some(v) = &self.tech_address {
            writer.write_pair("TECH_ADDRESS", v);
        }
        if let Some(v) = &self.previous_message_id {
            writer.write_pair("PREVIOUS_MESSAGE_ID", v);
        }
        if let Some(v) = &self.next_message_id {
            writer.write_pair("NEXT_MESSAGE_ID", v);
        }
        if let Some(v) = &self.adm_msg_link {
            writer.write_pair("ADM_MSG_LINK", v);
        }
        if let Some(v) = &self.cdm_msg_link {
            writer.write_pair("CDM_MSG_LINK", v);
        }
        if let Some(v) = &self.prm_msg_link {
            writer.write_pair("PRM_MSG_LINK", v);
        }
        if let Some(v) = &self.rdm_msg_link {
            writer.write_pair("RDM_MSG_LINK", v);
        }
        if let Some(v) = &self.tdm_msg_link {
            writer.write_pair("TDM_MSG_LINK", v);
        }
        if let Some(v) = &self.operator {
            writer.write_pair("OPERATOR", v);
        }
        if let Some(v) = &self.owner {
            writer.write_pair("OWNER", v);
        }
        if let Some(v) = &self.country {
            writer.write_pair("COUNTRY", v);
        }
        if let Some(v) = &self.constellation {
            writer.write_pair("CONSTELLATION", v);
        }
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.ops_status {
            writer.write_pair("OPS_STATUS", v);
        }
        if let Some(v) = &self.orbit_category {
            writer.write_pair("ORBIT_CATEGORY", v);
        }
        if let Some(v) = &self.ocm_data_elements {
            writer.write_pair("OCM_DATA_ELEMENTS", v);
        }
        if let Some(v) = &self.sclk_offset_at_epoch {
            writer.write_measure("SCLK_OFFSET_AT_EPOCH", &v.to_unit_value());
        }
        if let Some(v) = &self.sclk_sec_per_si_sec {
            writer.write_measure("SCLK_SEC_PER_SI_SEC", &v.to_unit_value());
        }
        if let Some(v) = &self.previous_message_epoch {
            writer.write_pair("PREVIOUS_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.next_message_epoch {
            writer.write_pair("NEXT_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = &self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        if let Some(v) = &self.time_span {
            writer.write_measure("TIME_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.taimutc_at_tzero {
            writer.write_measure("TAIMUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.next_leap_epoch {
            writer.write_pair("NEXT_LEAP_EPOCH", v);
        }
        if let Some(v) = &self.next_leap_taimutc {
            writer.write_measure("NEXT_LEAP_TAIMUTC", &v.to_unit_value());
        }
        if let Some(v) = &self.ut1mutc_at_tzero {
            writer.write_measure("UT1MUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.eop_source {
            writer.write_pair("EOP_SOURCE", v);
        }
        if let Some(v) = &self.interp_method_eop {
            writer.write_pair("INTERP_METHOD_EOP", v);
        }
        if let Some(v) = &self.celestial_source {
            writer.write_pair("CELESTIAL_SOURCE", v);
        }
        writer.write_section("META_STOP");
    }
}

impl ToKvn for OcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for t in &self.traj {
            t.write_kvn(writer);
        }
        if let Some(p) = &self.phys {
            p.write_kvn(writer);
        }
        for c in &self.cov {
            c.write_kvn(writer);
        }
        for m in &self.man {
            m.write_kvn(writer);
        }
        if let Some(p) = &self.pert {
            p.write_kvn(writer);
        }
        if let Some(o) = &self.od {
            o.write_kvn(writer);
        }
        if let Some(u) = &self.user {
            writer.write_section("USER_START");
            writer.write_comments(&u.comment);
            for p in &u.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

impl ToKvn for OcmTrajState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("TRAJ_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.traj_id {
            writer.write_pair("TRAJ_ID", v);
        }
        if let Some(v) = &self.traj_prev_id {
            writer.write_pair("TRAJ_PREV_ID", v);
        }
        if let Some(v) = &self.traj_next_id {
            writer.write_pair("TRAJ_NEXT_ID", v);
        }
        if let Some(v) = &self.traj_basis {
            writer.write_pair("TRAJ_BASIS", v.to_string());
        }
        if let Some(v) = &self.traj_basis_id {
            writer.write_pair("TRAJ_BASIS_ID", v);
        }
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = &self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
        if let Some(v) = &self.propagator {
            writer.write_pair("PROPAGATOR", v);
        }
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TRAJ_REF_FRAME", &self.traj_ref_frame);
        if let Some(v) = &self.traj_frame_epoch {
            writer.write_pair("TRAJ_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = &self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        if let Some(v) = &self.orb_revnum {
            writer.write_pair("ORB_REVNUM", v);
        }
        if let Some(v) = &self.orb_revnum_basis {
            writer.write_pair(
                "ORB_REVNUM_BASIS",
                match v {
                    RevNumBasis::Zero => "0",
                    RevNumBasis::One => "1",
                },
            );
        }
        writer.write_pair("TRAJ_TYPE", &self.traj_type);
        if let Some(v) = &self.orb_averaging {
            writer.write_pair("ORB_AVERAGING", v);
        }
        if let Some(v) = &self.traj_units {
            writer.write_pair("TRAJ_UNITS", v);
        }
        for line in &self.traj_lines {
            writer.write_ocm_numeric_history(&line.epoch, &line.values);
        }
        writer.write_section("TRAJ_STOP");
    }
}

impl ToKvn for OcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.manufacturer {
            writer.write_pair("MANUFACTURER", v);
        }
        if let Some(v) = &self.bus_model {
            writer.write_pair("BUS_MODEL", v);
        }
        if let Some(v) = &self.docked_with {
            writer.write_pair("DOCKED_WITH", v);
        }
        if let Some(v) = &self.drag_const_area {
            writer.write_measure("DRAG_CONST_AREA", &v.to_unit_value());
        }
        if let Some(v) = &self.drag_coeff_nom {
            writer.write_pair("DRAG_COEFF_NOM", v);
        }
        if let Some(v) = &self.drag_uncertainty {
            writer.write_measure("DRAG_UNCERTAINTY", &v.to_unit_value());
        }
        if let Some(v) = &self.initial_wet_mass {
            writer.write_measure("INITIAL_WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.wet_mass {
            writer.write_measure("WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.dry_mass {
            writer.write_measure("DRY_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.oeb_parent_frame {
            writer.write_pair("OEB_PARENT_FRAME", v);
        }
        if let Some(v) = &self.oeb_parent_frame_epoch {
            writer.write_pair("OEB_PARENT_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.oeb_q1 {
            writer.write_pair("OEB_Q1", v);
        }
        if let Some(v) = &self.oeb_q2 {
            writer.write_pair("OEB_Q2", v);
        }
        if let Some(v) = &self.oeb_q3 {
            writer.write_pair("OEB_Q3", v);
        }
        if let Some(v) = &self.oeb_qc {
            writer.write_pair("OEB_QC", v);
        }
        if let Some(v) = &self.oeb_max {
            writer.write_measure("OEB_MAX", v);
        }
        if let Some(v) = &self.oeb_int {
            writer.write_measure("OEB_INT", v);
        }
        if let Some(v) = &self.oeb_min {
            writer.write_measure("OEB_MIN", v);
        }
        if let Some(v) = &self.area_along_oeb_max {
            writer.write_measure("AREA_ALONG_OEB_MAX", &v.to_unit_value());
        }
        if let Some(v) = &self.area_along_oeb_int {
            writer.write_measure("AREA_ALONG_OEB_INT", &v.to_unit_value());
        }
        if let Some(v) = &self.area_along_oeb_min {
            writer.write_measure("AREA_ALONG_OEB_MIN", &v.to_unit_value());
        }
        if let Some(v) = &self.area_min_for_pc {
            writer.write_measure("AREA_MIN_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.area_max_for_pc {
            writer.write_measure("AREA_MAX_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.area_typ_for_pc {
            writer.write_measure("AREA_TYP_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs {
            writer.write_measure("RCS", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs_min {
            writer.write_measure("RCS_MIN", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs_max {
            writer.write_measure("RCS_MAX", &v.to_unit_value());
        }
        if let Some(v) = &self.srp_const_area {
            writer.write_measure("SRP_CONST_AREA", &v.to_unit_value());
        }
        if let Some(v) = &self.solar_rad_coeff {
            writer.write_pair("SOLAR_RAD_COEFF", v);
        }
        if let Some(v) = &self.solar_rad_uncertainty {
            writer.write_measure("SOLAR_RAD_UNCERTAINTY", &v.to_unit_value());
        }
        if let Some(v) = &self.vm_absolute {
            writer.write_pair("VM_ABSOLUTE", v);
        }
        if let Some(v) = &self.vm_apparent_min {
            writer.write_pair("VM_APPARENT_MIN", v);
        }
        if let Some(v) = &self.vm_apparent {
            writer.write_pair("VM_APPARENT", v);
        }
        if let Some(v) = &self.vm_apparent_max {
            writer.write_pair("VM_APPARENT_MAX", v);
        }
        if let Some(v) = &self.reflectance {
            writer.write_pair("REFLECTANCE", v);
        }
        if let Some(v) = &self.att_control_mode {
            writer.write_pair("ATT_CONTROL_MODE", v);
        }
        if let Some(v) = &self.att_actuator_type {
            writer.write_pair("ATT_ACTUATOR_TYPE", v);
        }
        if let Some(v) = &self.att_knowledge {
            writer.write_measure("ATT_KNOWLEDGE", &v.to_unit_value());
        }
        if let Some(v) = &self.att_control {
            writer.write_measure("ATT_CONTROL", &v.to_unit_value());
        }
        if let Some(v) = &self.att_pointing {
            writer.write_measure("ATT_POINTING", &v.to_unit_value());
        }
        if let Some(v) = &self.avg_maneuver_freq {
            writer.write_measure("AVG_MANEUVER_FREQ", v);
        }
        if let Some(v) = &self.max_thrust {
            writer.write_measure("MAX_THRUST", v);
        }
        if let Some(v) = &self.dv_bol {
            writer.write_measure("DV_BOL", v);
        }
        if let Some(v) = &self.dv_remaining {
            writer.write_measure("DV_REMAINING", v);
        }
        if let Some(v) = &self.ixx {
            writer.write_measure("IXX", v);
        }
        if let Some(v) = &self.iyy {
            writer.write_measure("IYY", v);
        }
        if let Some(v) = &self.izz {
            writer.write_measure("IZZ", v);
        }
        if let Some(v) = &self.ixy {
            writer.write_measure("IXY", v);
        }
        if let Some(v) = &self.ixz {
            writer.write_measure("IXZ", v);
        }
        if let Some(v) = &self.iyz {
            writer.write_measure("IYZ", v);
        }
        writer.write_section("PHYS_STOP");
    }
}

impl ToKvn for OcmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COV_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.cov_id {
            writer.write_pair("COV_ID", v);
        }
        if let Some(v) = &self.cov_prev_id {
            writer.write_pair("COV_PREV_ID", v);
        }
        if let Some(v) = &self.cov_next_id {
            writer.write_pair("COV_NEXT_ID", v);
        }
        if let Some(v) = &self.cov_basis {
            writer.write_pair("COV_BASIS", v.to_string());
        }
        if let Some(v) = &self.cov_basis_id {
            writer.write_pair("COV_BASIS_ID", v);
        }
        writer.write_pair("COV_REF_FRAME", &self.cov_ref_frame);
        if let Some(v) = &self.cov_frame_epoch {
            writer.write_pair("COV_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.cov_scale_min {
            writer.write_pair("COV_SCALE_MIN", v);
        }
        if let Some(v) = &self.cov_scale_max {
            writer.write_pair("COV_SCALE_MAX", v);
        }
        if let Some(v) = &self.cov_confidence {
            writer.write_measure("COV_CONFIDENCE", &v.to_unit_value());
        }
        writer.write_pair("COV_TYPE", &self.cov_type);
        writer.write_pair("COV_ORDERING", self.cov_ordering.to_string());
        if let Some(v) = &self.cov_units {
            writer.write_pair("COV_UNITS", v);
        }
        for line in &self.cov_lines {
            writer.write_ocm_numeric_history(&line.epoch, &line.values);
        }
        writer.write_section("COV_STOP");
    }
}

impl ToKvn for OcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_ID", &self.man_id);
        if let Some(v) = &self.man_prev_id {
            writer.write_pair("MAN_PREV_ID", v);
        }
        if let Some(v) = &self.man_next_id {
            writer.write_pair("MAN_NEXT_ID", v);
        }
        if let Some(v) = &self.man_basis {
            writer.write_pair("MAN_BASIS", v.to_string());
        }
        if let Some(v) = &self.man_basis_id {
            writer.write_pair("MAN_BASIS_ID", v);
        }
        writer.write_pair("MAN_DEVICE_ID", &self.man_device_id);
        if let Some(v) = &self.man_prev_epoch {
            writer.write_pair("MAN_PREV_EPOCH", v);
        }
        if let Some(v) = &self.man_next_epoch {
            writer.write_pair("MAN_NEXT_EPOCH", v);
        }
        if let Some(v) = &self.man_purpose {
            writer.write_pair("MAN_PURPOSE", v);
        }
        if let Some(v) = &self.man_pred_source {
            writer.write_pair("MAN_PRED_SOURCE", v);
        }
        writer.write_pair("MAN_REF_FRAME", &self.man_ref_frame);
        if let Some(v) = &self.man_frame_epoch {
            writer.write_pair("MAN_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.grav_assist_name {
            writer.write_pair("GRAV_ASSIST_NAME", v);
        }
        writer.write_pair("DC_TYPE", self.dc_type.to_string());
        if let Some(v) = &self.dc_win_open {
            writer.write_pair("DC_WIN_OPEN", v);
        }
        if let Some(v) = &self.dc_win_close {
            writer.write_pair("DC_WIN_CLOSE", v);
        }
        if let Some(v) = &self.dc_min_cycles {
            writer.write_pair("DC_MIN_CYCLES", v);
        }
        if let Some(v) = &self.dc_max_cycles {
            writer.write_pair("DC_MAX_CYCLES", v);
        }
        if let Some(v) = &self.dc_exec_start {
            writer.write_pair("DC_EXEC_START", v);
        }
        if let Some(v) = &self.dc_exec_stop {
            writer.write_pair("DC_EXEC_STOP", v);
        }
        if let Some(v) = &self.dc_ref_time {
            writer.write_pair("DC_REF_TIME", v);
        }
        if let Some(v) = &self.dc_time_pulse_duration {
            writer.write_measure("DC_TIME_PULSE_DURATION", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_time_pulse_period {
            writer.write_measure("DC_TIME_PULSE_PERIOD", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_ref_dir {
            writer.write_pair("DC_REF_DIR", format!("{} {} {}", v.x, v.y, v.z));
        }
        if let Some(v) = &self.dc_body_frame {
            writer.write_pair("DC_BODY_FRAME", v);
        }
        if let Some(v) = &self.dc_body_trigger {
            writer.write_pair("DC_BODY_TRIGGER", format!("{} {} {}", v.x, v.y, v.z));
        }
        if let Some(v) = &self.dc_pa_start_angle {
            writer.write_measure("DC_PA_START_ANGLE", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_pa_stop_angle {
            writer.write_measure("DC_PA_STOP_ANGLE", &v.to_unit_value());
        }
        writer.write_pair("MAN_COMPOSITION", &self.man_composition);
        if let Some(v) = &self.man_units {
            writer.write_pair("MAN_UNITS", v);
        }
        for line in &self.man_lines {
            writer.write_ocm_text_history(&line.epoch, &line.values);
        }
        writer.write_section("MAN_STOP");
    }
}

impl ToKvn for OcmPerturbations {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PERT_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.equatorial_radius {
            writer.write_measure("EQUATORIAL_RADIUS", v);
        }
        if let Some(v) = &self.gm {
            writer.write_measure("GM", &v.to_unit_value());
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.central_body_rotation {
            writer.write_measure("CENTRAL_BODY_ROTATION", v);
        }
        if let Some(v) = &self.oblate_flattening {
            writer.write_pair("OBLATE_FLATTENING", v);
        }
        if let Some(v) = &self.ocean_tides_model {
            writer.write_pair("OCEAN_TIDES_MODEL", v);
        }
        if let Some(v) = &self.solid_tides_model {
            writer.write_pair("SOLID_TIDES_MODEL", v);
        }
        if let Some(v) = &self.reduction_theory {
            writer.write_pair("REDUCTION_THEORY", v);
        }
        if let Some(v) = &self.albedo_model {
            writer.write_pair("ALBEDO_MODEL", v);
        }
        if let Some(v) = &self.albedo_grid_size {
            writer.write_pair("ALBEDO_GRID_SIZE", v);
        }
        if let Some(v) = &self.shadow_model {
            writer.write_pair("SHADOW_MODEL", v);
        }
        if let Some(v) = &self.shadow_bodies {
            writer.write_pair("SHADOW_BODIES", v);
        }
        if let Some(v) = &self.srp_model {
            writer.write_pair("SRP_MODEL", v);
        }
        if let Some(v) = &self.sw_data_source {
            writer.write_pair("SW_DATA_SOURCE", v);
        }
        if let Some(v) = &self.sw_data_epoch {
            writer.write_pair("SW_DATA_EPOCH", v);
        }
        if let Some(v) = &self.sw_interp_method {
            writer.write_pair("SW_INTERP_METHOD", v);
        }
        if let Some(v) = &self.fixed_geomag_kp {
            writer.write_measure("FIXED_GEOMAG_KP", v);
        }
        if let Some(v) = &self.fixed_geomag_ap {
            writer.write_measure("FIXED_GEOMAG_AP", v);
        }
        if let Some(v) = &self.fixed_geomag_dst {
            writer.write_measure("FIXED_GEOMAG_DST", v);
        }
        if let Some(v) = &self.fixed_f10p7 {
            writer.write_measure("FIXED_F10P7", v);
        }
        if let Some(v) = &self.fixed_f10p7_mean {
            writer.write_measure("FIXED_F10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_m10p7 {
            writer.write_measure("FIXED_M10P7", v);
        }
        if let Some(v) = &self.fixed_m10p7_mean {
            writer.write_measure("FIXED_M10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_s10p7 {
            writer.write_measure("FIXED_S10P7", v);
        }
        if let Some(v) = &self.fixed_s10p7_mean {
            writer.write_measure("FIXED_S10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_y10p7 {
            writer.write_measure("FIXED_Y10P7", v);
        }
        if let Some(v) = &self.fixed_y10p7_mean {
            writer.write_measure("FIXED_Y10P7_MEAN", v);
        }
        writer.write_section("PERT_STOP");
    }
}

impl ToKvn for OcmOdParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("OD_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OD_ID", &self.od_id);
        if let Some(v) = &self.od_prev_id {
            writer.write_pair("OD_PREV_ID", v);
        }
        writer.write_pair("OD_METHOD", &self.od_method);
        writer.write_pair("OD_EPOCH", self.od_epoch);
        if let Some(v) = &self.days_since_first_obs {
            writer.write_measure("DAYS_SINCE_FIRST_OBS", &v.to_unit_value());
        }
        if let Some(v) = &self.days_since_last_obs {
            writer.write_measure("DAYS_SINCE_LAST_OBS", &v.to_unit_value());
        }
        if let Some(v) = &self.recommended_od_span {
            writer.write_measure("RECOMMENDED_OD_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.actual_od_span {
            writer.write_measure("ACTUAL_OD_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.obs_available {
            writer.write_pair("OBS_AVAILABLE", v);
        }
        if let Some(v) = &self.obs_used {
            writer.write_pair("OBS_USED", v);
        }
        if let Some(v) = &self.tracks_available {
            writer.write_pair("TRACKS_AVAILABLE", v);
        }
        if let Some(v) = &self.tracks_used {
            writer.write_pair("TRACKS_USED", v);
        }
        if let Some(v) = &self.maximum_obs_gap {
            writer.write_measure("MAXIMUM_OBS_GAP", &v.to_unit_value());
        }
        if let Some(v) = &self.od_epoch_eigmaj {
            writer.write_measure("OD_EPOCH_EIGMAJ", v);
        }
        if let Some(v) = &self.od_epoch_eigint {
            writer.write_measure("OD_EPOCH_EIGINT", v);
        }
        if let Some(v) = &self.od_epoch_eigmin {
            writer.write_measure("OD_EPOCH_EIGMIN", v);
        }
        if let Some(v) = &self.od_max_pred_eigmaj {
            writer.write_measure("OD_MAX_PRED_EIGMAJ", v);
        }
        if let Some(v) = &self.od_min_pred_eigmin {
            writer.write_measure("OD_MIN_PRED_EIGMIN", v);
        }
        if let Some(v) = &self.od_confidence {
            writer.write_measure("OD_CONFIDENCE", &v.to_unit_value());
        }
        if let Some(v) = &self.gdop {
            writer.write_pair("GDOP", v);
        }
        if let Some(v) = &self.solve_n {
            writer.write_pair("SOLVE_N", v);
        }
        if let Some(v) = &self.solve_states {
            writer.write_pair("SOLVE_STATES", v);
        }
        if let Some(v) = &self.consider_n {
            writer.write_pair("CONSIDER_N", v);
        }
        if let Some(v) = &self.consider_params {
            writer.write_pair("CONSIDER_PARAMS", v);
        }
        if let Some(v) = &self.sedr {
            writer.write_measure("SEDR", &v.to_unit_value());
        }
        if let Some(v) = &self.sensors_n {
            writer.write_pair("SENSORS_N", v);
        }
        if let Some(v) = &self.sensors {
            writer.write_pair("SENSORS", v);
        }
        if let Some(v) = &self.weighted_rms {
            writer.write_pair("WEIGHTED_RMS", v);
        }
        if let Some(v) = &self.data_types {
            writer.write_pair("DATA_TYPES", v);
        }
        writer.write_section("OD_STOP");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
