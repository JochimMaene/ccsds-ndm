// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for APM (Attitude Parameter Message).

use crate::common::{
    AngVelState, EulerAngleState, InertiaState, Quaternion, QuaternionDot, QuaternionState,
    SpinState,
};
// But QuaternionState etc are in common.
use super::{Apm, ApmBody, ApmData, ApmMetadata, ApmSegment};
use crate::common::AttManeuverState;
use crate::error::{CcsdsNdmError, FormatError, InternalParserError, KvnParseError, Result};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;
use std::str::FromStr;
use winnow::error::{ErrMode, FromExternalError};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// APM Version Parser
//----------------------------------------------------------------------

pub fn apm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;

    let value = expect_unitless_key("CCSDS_APM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// APM Metadata Parser
//----------------------------------------------------------------------

pub fn apm_metadata(input: &mut &str) -> KvnResult<ApmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut time_system = None;

    // META_START is optional in some APM examples (e.g. apm_g1.kvn)
    let _ = winnow::combinator::opt(expect_block_start("META")).parse_next(input)?;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "TIME_SYSTEM" => time_system: kv_string,
    }, |i: &mut &str| at_block_end("META", i) || at_block_start("QUAT", i) || at_block_start("EULER", i) || at_block_start("ANGVEL", i) || at_block_start("SPIN", i) || at_block_start("INERTIA", i) || at_block_start("MAN", i) || expect_key("EPOCH").parse_peek(i).is_ok());

    let _ = winnow::combinator::opt(expect_block_end("META")).parse_next(input)?;

    Ok(ApmMetadata {
        comment,
        object_name: require_field(input, "APM Metadata", "OBJECT_NAME", object_name)?,
        object_id: require_field(input, "APM Metadata", "OBJECT_ID", object_id)?,
        center_name,
        time_system: require_field(input, "APM Metadata", "TIME_SYSTEM", time_system)?,
    })
}

//----------------------------------------------------------------------
// Logical Block Parsers
//----------------------------------------------------------------------

pub fn quaternion_state(input: &mut &str) -> KvnResult<QuaternionState> {
    expect_block_start("QUAT").parse_next(input)?;
    let mut comment = Vec::new();
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut q1 = None;
    let mut q2 = None;
    let mut q3 = None;
    let mut qc = None;
    let mut q1_dot = None;
    let mut q2_dot = None;
    let mut q3_dot = None;
    let mut qc_dot = None;

    parse_block!(input, comment, {
        "REF_FRAME_A" => ref_frame_a: kv_string,
        "REF_FRAME_B" => ref_frame_b: kv_string,
        "Q1" => q1: kv_float,
        "Q2" => q2: kv_float,
        "Q3" => q3: kv_float,
        "QC" => qc: kv_float,
        "Q1_DOT" => q1_dot: kv_from_kvn,
        "Q2_DOT" => q2_dot: kv_from_kvn,
        "Q3_DOT" => q3_dot: kv_from_kvn,
        "QC_DOT" => qc_dot: kv_from_kvn,
    }, |i: &mut &str| at_block_end("QUAT", i));

    expect_block_end("QUAT").parse_next(input)?;

    let quaternion = Quaternion::new(
        require_field(input, "QUAT", "Q1", q1)?,
        require_field(input, "QUAT", "Q2", q2)?,
        require_field(input, "QUAT", "Q3", q3)?,
        require_field(input, "QUAT", "QC", qc)?,
    )
    .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;

    let quaternion_dot =
        if q1_dot.is_some() || q2_dot.is_some() || q3_dot.is_some() || qc_dot.is_some() {
            Some(QuaternionDot {
                q1_dot: require_field(input, "QUAT", "Q1_DOT", q1_dot)?,
                q2_dot: require_field(input, "QUAT", "Q2_DOT", q2_dot)?,
                q3_dot: require_field(input, "QUAT", "Q3_DOT", q3_dot)?,
                qc_dot: require_field(input, "QUAT", "QC_DOT", qc_dot)?,
            })
        } else {
            None
        };

    Ok(QuaternionState {
        comment,
        ref_frame_a: require_field(input, "QUAT", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "QUAT", "REF_FRAME_B", ref_frame_b)?,
        quaternion,
        quaternion_dot,
    })
}

pub fn euler_angle_state(input: &mut &str) -> KvnResult<EulerAngleState> {
    expect_block_start("EULER").parse_next(input)?;
    let mut comment = Vec::new();
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut euler_rot_seq = None;
    let mut angle_1 = None;
    let mut angle_2 = None;
    let mut angle_3 = None;
    let mut angle_1_dot = None;
    let mut angle_2_dot = None;
    let mut angle_3_dot = None;

    parse_block!(input, comment, {
        "REF_FRAME_A" => ref_frame_a: kv_string,
        "REF_FRAME_B" => ref_frame_b: kv_string,
        "EULER_ROT_SEQ" => euler_rot_seq: kv_enum,
        "ANGLE_1" => angle_1: kv_from_kvn,
        "ANGLE_2" => angle_2: kv_from_kvn,
        "ANGLE_3" => angle_3: kv_from_kvn,
        "ANGLE_1_DOT" => angle_1_dot: kv_from_kvn,
        "ANGLE_2_DOT" => angle_2_dot: kv_from_kvn,
        "ANGLE_3_DOT" => angle_3_dot: kv_from_kvn,
    }, |i: &mut &str| at_block_end("EULER", i));

    expect_block_end("EULER").parse_next(input)?;

    Ok(EulerAngleState {
        comment,
        ref_frame_a: require_field(input, "EULER", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "EULER", "REF_FRAME_B", ref_frame_b)?,
        euler_rot_seq: require_field(input, "EULER", "EULER_ROT_SEQ", euler_rot_seq)?,
        angle_1: require_field(input, "EULER", "ANGLE_1", angle_1)?,
        angle_2: require_field(input, "EULER", "ANGLE_2", angle_2)?,
        angle_3: require_field(input, "EULER", "ANGLE_3", angle_3)?,
        angle_1_dot,
        angle_2_dot,
        angle_3_dot,
    })
}

pub fn ang_vel_state(input: &mut &str) -> KvnResult<AngVelState> {
    expect_block_start("ANGVEL").parse_next(input)?;
    let mut comment = Vec::new();
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut angvel_frame = None;
    let mut angvel_x = None;
    let mut angvel_y = None;
    let mut angvel_z = None;

    parse_block!(input, comment, {
        "REF_FRAME_A" => ref_frame_a: kv_string,
        "REF_FRAME_B" => ref_frame_b: kv_string,
        // KVN uses ANGVEL_FRAME
        "ANGVEL_FRAME" => val: kv_string => { angvel_frame = Some(crate::types::AngVelFrameType(val)); },
        "ANGVEL_X" => angvel_x: kv_from_kvn,
        "ANGVEL_Y" => angvel_y: kv_from_kvn,
        "ANGVEL_Z" => angvel_z: kv_from_kvn,
    }, |i: &mut &str| at_block_end("ANGVEL", i));

    expect_block_end("ANGVEL").parse_next(input)?;

    Ok(AngVelState {
        comment,
        ref_frame_a: require_field(input, "ANGVEL", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "ANGVEL", "REF_FRAME_B", ref_frame_b)?,
        angvel_frame: require_field(input, "ANGVEL", "ANGVEL_FRAME", angvel_frame)?,
        angvel_x: require_field(input, "ANGVEL", "ANGVEL_X", angvel_x)?,
        angvel_y: require_field(input, "ANGVEL", "ANGVEL_Y", angvel_y)?,
        angvel_z: require_field(input, "ANGVEL", "ANGVEL_Z", angvel_z)?,
    })
}

pub fn spin_state(input: &mut &str) -> KvnResult<SpinState> {
    expect_block_start("SPIN").parse_next(input)?;
    let mut comment = Vec::new();
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut spin_alpha = None;
    let mut spin_delta = None;
    let mut spin_angle = None;
    let mut spin_angle_vel = None;
    let mut nutation = None;
    let mut nutation_per = None;
    let mut nutation_phase = None;
    let mut momentum_alpha = None;
    let mut momentum_delta = None;
    let mut nutation_vel = None;

    parse_block!(input, comment, {
        "REF_FRAME_A" => ref_frame_a: kv_string,
        "REF_FRAME_B" => ref_frame_b: kv_string,
        "SPIN_ALPHA" => spin_alpha: kv_from_kvn,
        "SPIN_DELTA" => spin_delta: kv_from_kvn,
        "SPIN_ANGLE" => spin_angle: kv_from_kvn,
        "SPIN_ANGLE_VEL" => spin_angle_vel: kv_from_kvn,
        "NUTATION" => nutation: kv_from_kvn,
        "NUTATION_PER" => nutation_per: kv_from_kvn,
        "NUTATION_PHASE" => nutation_phase: kv_from_kvn,
        "MOMENTUM_ALPHA" => momentum_alpha: kv_from_kvn,
        "MOMENTUM_DELTA" => momentum_delta: kv_from_kvn,
        "NUTATION_VEL" => nutation_vel: kv_from_kvn,
    }, |i: &mut &str| at_block_end("SPIN", i));

    expect_block_end("SPIN").parse_next(input)?;

    Ok(SpinState {
        comment,
        ref_frame_a: require_field(input, "SPIN", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "SPIN", "REF_FRAME_B", ref_frame_b)?,
        spin_alpha: require_field(input, "SPIN", "SPIN_ALPHA", spin_alpha)?,
        spin_delta: require_field(input, "SPIN", "SPIN_DELTA", spin_delta)?,
        spin_angle: require_field(input, "SPIN", "SPIN_ANGLE", spin_angle)?,
        spin_angle_vel: require_field(input, "SPIN", "SPIN_ANGLE_VEL", spin_angle_vel)?,
        nutation,
        nutation_per,
        nutation_phase,
        momentum_alpha,
        momentum_delta,
        nutation_vel,
    })
}

pub fn inertia_state(input: &mut &str) -> KvnResult<InertiaState> {
    expect_block_start("INERTIA").parse_next(input)?;
    let mut comment = Vec::new();
    let mut inertia_ref_frame = None;
    let mut ixx = None;
    let mut iyy = None;
    let mut izz = None;
    let mut ixy = None;
    let mut ixz = None;
    let mut iyz = None;

    parse_block!(input, comment, {
        "INERTIA_REF_FRAME" => inertia_ref_frame: kv_string,
        "IXX" => ixx: kv_from_kvn,
        "IYY" => iyy: kv_from_kvn,
        "IZZ" => izz: kv_from_kvn,
        "IXY" => ixy: kv_from_kvn,
        "IXZ" => ixz: kv_from_kvn,
        "IYZ" => iyz: kv_from_kvn,
    }, |i: &mut &str| at_block_end("INERTIA", i));

    expect_block_end("INERTIA").parse_next(input)?;

    Ok(InertiaState {
        comment,
        inertia_ref_frame: require_field(input, "INERTIA", "INERTIA_REF_FRAME", inertia_ref_frame)?,
        ixx: require_field(input, "INERTIA", "IXX", ixx)?,
        iyy: require_field(input, "INERTIA", "IYY", iyy)?,
        izz: require_field(input, "INERTIA", "IZZ", izz)?,
        ixy: require_field(input, "INERTIA", "IXY", ixy)?,
        ixz: require_field(input, "INERTIA", "IXZ", ixz)?,
        iyz: require_field(input, "INERTIA", "IYZ", iyz)?,
    })
}

// Maneuver block
pub fn maneuver_parameters(input: &mut &str) -> KvnResult<AttManeuverState> {
    expect_block_start("MAN").parse_next(input)?;
    let mut comment = Vec::new();
    let mut man_epoch_start = None;
    let mut man_duration = None;
    let mut man_ref_frame = None;
    let mut man_tor_x = None;
    let mut man_tor_y = None;
    let mut man_tor_z = None;
    let mut man_delta_mass = None;

    parse_block!(input, comment, {
        "MAN_EPOCH_START" => man_epoch_start: kv_calendar_epoch,
        "MAN_DURATION" => man_duration: kv_from_kvn,
        "MAN_REF_FRAME" => man_ref_frame: kv_string,
        "MAN_TOR_X" => man_tor_x: kv_from_kvn,
        "MAN_TOR_Y" => man_tor_y: kv_from_kvn,
        "MAN_TOR_Z" => man_tor_z: kv_from_kvn,
        "MAN_DELTA_MASS" => man_delta_mass: kv_from_kvn,
    }, |i: &mut &str| at_block_end("MAN", i));

    expect_block_end("MAN").parse_next(input)?;

    Ok(AttManeuverState {
        comment,
        man_epoch_start: require_field(input, "MAN", "MAN_EPOCH_START", man_epoch_start)?,
        man_duration: require_field(input, "MAN", "MAN_DURATION", man_duration)?,
        man_ref_frame: require_field(input, "MAN", "MAN_REF_FRAME", man_ref_frame)?,
        man_tor_x: require_field(input, "MAN", "MAN_TOR_X", man_tor_x)?,
        man_tor_y: require_field(input, "MAN", "MAN_TOR_Y", man_tor_y)?,
        man_tor_z: require_field(input, "MAN", "MAN_TOR_Z", man_tor_z)?,
        man_delta_mass,
    })
}

//----------------------------------------------------------------------
// APM Segment & Main Parser
//----------------------------------------------------------------------

pub fn apm_data(input: &mut &str) -> KvnResult<ApmData> {
    // APM Data Section parsing

    // First, comments are allowed before EPOCH
    let mut comment = Vec::new();
    comment.extend(collect_comments.parse_next(input)?);

    // EPOCH is mandatory and usually the first key in data section
    let epoch = expect_unitless_key("EPOCH").parse_next(input)?;
    let epoch = crate::types::CalendarEpoch::from_str(epoch)
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;

    let mut quaternion_state = Vec::new();
    let mut euler_angle_state = Vec::new();
    let mut angular_velocity = Vec::new();
    let mut spin = Vec::new();
    let mut inertia = Vec::new();
    let mut maneuver_parameters = Vec::new();

    // Logical blocks can appear in any order.
    loop {
        let checkpoint = input.checkpoint();
        let _ = skip_empty_lines.parse_next(input);

        // Check for recognized blocks using lookahead or just trying parsers
        // We use at_block_start to check which one it is.
        if at_block_start("QUAT", input) {
            quaternion_state.push(self::quaternion_state.parse_next(input)?);
        } else if at_block_start("EULER", input) {
            euler_angle_state.push(self::euler_angle_state.parse_next(input)?);
        } else if at_block_start("ANGVEL", input) {
            angular_velocity.push(self::ang_vel_state.parse_next(input)?);
        } else if at_block_start("SPIN", input) {
            spin.push(self::spin_state.parse_next(input)?);
        } else if at_block_start("INERTIA", input) {
            inertia.push(self::inertia_state.parse_next(input)?);
        } else if at_block_start("MAN", input) {
            maneuver_parameters.push(self::maneuver_parameters.parse_next(input)?);
        } else {
            // Unknown block or end of stream.
            break;
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(ApmData {
        comment,
        epoch,
        quaternion_state,
        euler_angle_state,
        angular_velocity,
        spin,
        inertia,
        maneuver_parameters,
    })
}

pub fn apm_segment(input: &mut &str) -> KvnResult<ApmSegment> {
    let metadata = apm_metadata.parse_next(input)?;
    let _ = skip_empty_lines.parse_next(input);
    let data = apm_data.parse_next(input)?;

    Ok(ApmSegment { metadata, data })
}

pub fn parse_apm(input: &mut &str) -> KvnResult<Apm> {
    let version = apm_version.parse_next(input)?;
    let header = adm_header.parse_next(input)?;

    let _ = skip_empty_lines.parse_next(input);
    let segment = apm_segment.parse_next(input)?;

    Ok(Apm {
        header,
        body: ApmBody { segment },
        id: Some("CCSDS_APM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Apm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_apm.parse_next(input)
    }
}

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    fn top_rank(key: &str) -> Option<u16> {
        Some(match key {
            "CCSDS_APM_VERS" => 0,
            "CLASSIFICATION" => 1,
            "CREATION_DATE" => 2,
            "ORIGINATOR" => 3,
            "MESSAGE_ID" => 4,
            "OBJECT_NAME" => 10,
            "OBJECT_ID" => 11,
            "CENTER_NAME" => 12,
            "TIME_SYSTEM" => 13,
            "EPOCH" => 20,
            _ => return None,
        })
    }

    fn block_rank(block: &str, key: &str) -> Option<u16> {
        let keys: &[&str] = match block {
            "META" => &["OBJECT_NAME", "OBJECT_ID", "CENTER_NAME", "TIME_SYSTEM"],
            "QUAT" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "Q1",
                "Q2",
                "Q3",
                "QC",
                "Q1_DOT",
                "Q2_DOT",
                "Q3_DOT",
                "QC_DOT",
            ],
            "EULER" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "EULER_ROT_SEQ",
                "ANGLE_1",
                "ANGLE_2",
                "ANGLE_3",
                "ANGLE_1_DOT",
                "ANGLE_2_DOT",
                "ANGLE_3_DOT",
            ],
            "ANGVEL" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "ANGVEL_FRAME",
                "ANGVEL_X",
                "ANGVEL_Y",
                "ANGVEL_Z",
            ],
            "SPIN" => &[
                "REF_FRAME_A",
                "REF_FRAME_B",
                "SPIN_ALPHA",
                "SPIN_DELTA",
                "SPIN_ANGLE",
                "SPIN_ANGLE_VEL",
                "NUTATION",
                "NUTATION_PER",
                "NUTATION_PHASE",
                "MOMENTUM_ALPHA",
                "MOMENTUM_DELTA",
                "NUTATION_VEL",
            ],
            "INERTIA" => &[
                "INERTIA_REF_FRAME",
                "IXX",
                "IYY",
                "IZZ",
                "IXY",
                "IXZ",
                "IYZ",
            ],
            "MAN" => &[
                "MAN_EPOCH_START",
                "MAN_DURATION",
                "MAN_REF_FRAME",
                "MAN_TOR_X",
                "MAN_TOR_Y",
                "MAN_TOR_Z",
                "MAN_DELTA_MASS",
            ],
            _ => return None,
        };
        keys.iter()
            .position(|candidate| *candidate == key)
            .map(|rank| rank as u16)
    }

    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating APM KVN structure"],
            offset,
        }))))
    };
    let mut current_block: Option<&str> = None;
    let mut top_previous = None;
    let mut block_previous = None;
    let mut pending_comment = false;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(line_number, offset, message.into()));
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
            if current_block.is_some() && block_previous.is_some() {
                return fail("COMMENT is not at the beginning of a logical block");
            }
            if current_block.is_none() && !matches!(top_previous, Some(0 | 3 | 4 | 13)) {
                return fail("COMMENT is not at the beginning of a logical block");
            }
            pending_comment = true;
            offset += raw_line.len() + 1;
            continue;
        }

        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if current_block.is_some()
                || !matches!(
                    marker,
                    "META" | "QUAT" | "EULER" | "ANGVEL" | "SPIN" | "INERTIA" | "MAN"
                )
            {
                return fail("unknown or nested APM logical-block start");
            }
            if pending_comment {
                return fail("COMMENT must follow, not precede, a logical-block start");
            }
            if marker == "META" {
                if !matches!(top_previous, Some(3 | 4)) {
                    return fail("META_START is out of order");
                }
            } else if top_previous != Some(20) {
                return fail("attitude logical block must follow EPOCH");
            }
            current_block = Some(marker);
            block_previous = None;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if current_block != Some(marker) {
                return fail("mismatched APM logical-block end");
            }
            if pending_comment {
                return fail("trailing COMMENT has no logical block content");
            }
            if marker == "META" {
                top_previous = Some(13);
            }
            current_block = None;
            block_previous = None;
            offset += raw_line.len() + 1;
            continue;
        }

        if !line.contains('=') {
            return fail("expected an assignment or logical-block delimiter");
        }
        let key = line
            .split_once('=')
            .expect("assignment count checked")
            .0
            .trim();
        if let Some(block) = current_block {
            let rank = block_rank(block, key)
                .ok_or_else(|| invalid(line_number, offset, "unknown APM block keyword".into()))?;
            if block_previous.is_some_and(|previous| rank <= previous) {
                return fail("duplicate or out-of-order APM block keyword");
            }
            block_previous = Some(rank);
        } else {
            let rank = top_rank(key)
                .ok_or_else(|| invalid(line_number, offset, "unknown APM keyword".into()))?;
            if pending_comment {
                let starts_block = match key {
                    "CLASSIFICATION" | "CREATION_DATE" => top_previous == Some(0),
                    "OBJECT_NAME" => matches!(top_previous, Some(3 | 4)),
                    "EPOCH" => top_previous == Some(13),
                    _ => false,
                };
                if !starts_block {
                    return fail("COMMENT is not at the beginning of a logical block");
                }
            }
            if top_previous.is_some_and(|previous| rank <= previous) {
                return fail("duplicate or out-of-order APM keyword");
            }
            top_previous = Some(rank);
        }
        pending_comment = false;
        offset += raw_line.len() + 1;
    }

    if current_block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unterminated APM logical block".into(),
        ));
    }
    if pending_comment {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "trailing COMMENT has no logical block".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Apm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_APM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for ApmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for ApmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_line("META_START");
        self.metadata.write_kvn(writer);
        writer.write_line("META_STOP");
        writer.write_line("");
        self.data.write_kvn(writer);
    }
}

impl ToKvn for ApmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
    }
}

impl ToKvn for ApmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        for block in &self.quaternion_state {
            writer.write_line("QUAT_START");
            block.write_kvn(writer);
            writer.write_line("QUAT_STOP");
            writer.write_line("");
        }
        for block in &self.euler_angle_state {
            writer.write_line("EULER_START");
            block.write_kvn(writer);
            writer.write_line("EULER_STOP");
            writer.write_line("");
        }
        for block in &self.angular_velocity {
            writer.write_line("ANGVEL_START");
            block.write_kvn(writer);
            writer.write_line("ANGVEL_STOP");
            writer.write_line("");
        }
        for block in &self.spin {
            writer.write_line("SPIN_START");
            block.write_kvn(writer);
            writer.write_line("SPIN_STOP");
            writer.write_line("");
        }
        for block in &self.inertia {
            writer.write_line("INERTIA_START");
            block.write_kvn(writer);
            writer.write_line("INERTIA_STOP");
            writer.write_line("");
        }
        for man in &self.maneuver_parameters {
            writer.write_line("MAN_START");
            man.write_kvn(writer);
            writer.write_line("MAN_STOP");
            writer.write_line("");
        }
    }
}
