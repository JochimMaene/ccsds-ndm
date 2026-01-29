// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for APM (Attitude Parameter Message).

use crate::common::{
    AngVelState, EulerAngleState, InertiaState, Quaternion, QuaternionDot, QuaternionState,
    SpinState,
};
// But QuaternionState etc are in common.
use crate::common::AttManeuverState;
use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::apm::{Apm, ApmBody, ApmData, ApmMetadata, ApmSegment};
use crate::parse_block;
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

    let (value, _) = expect_key("CCSDS_APM_VERS").parse_next(input)?;
    if value != "1.0" && value != "2.0" {
        return Err(cut_err(input, "1.0 or 2.0"));
    }
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
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "APM Metadata", "OBJECT_NAME"))?,
        object_id: object_id
            .ok_or_else(|| missing_field_err(input, "APM Metadata", "OBJECT_ID"))?,
        center_name,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "APM Metadata", "TIME_SYSTEM"))?,
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
        q1.ok_or_else(|| missing_field_err(input, "QUAT", "Q1"))?,
        q2.ok_or_else(|| missing_field_err(input, "QUAT", "Q2"))?,
        q3.ok_or_else(|| missing_field_err(input, "QUAT", "Q3"))?,
        qc.ok_or_else(|| missing_field_err(input, "QUAT", "QC"))?,
    )
    .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;

    let quaternion_dot =
        if q1_dot.is_some() || q2_dot.is_some() || q3_dot.is_some() || qc_dot.is_some() {
            Some(QuaternionDot {
                q1_dot: q1_dot.ok_or_else(|| missing_field_err(input, "QUAT", "Q1_DOT"))?,
                q2_dot: q2_dot.ok_or_else(|| missing_field_err(input, "QUAT", "Q2_DOT"))?,
                q3_dot: q3_dot.ok_or_else(|| missing_field_err(input, "QUAT", "Q3_DOT"))?,
                qc_dot: qc_dot.ok_or_else(|| missing_field_err(input, "QUAT", "QC_DOT"))?,
            })
        } else {
            None
        };

    Ok(QuaternionState {
        comment,
        ref_frame_a: ref_frame_a.ok_or_else(|| missing_field_err(input, "QUAT", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b.ok_or_else(|| missing_field_err(input, "QUAT", "REF_FRAME_B"))?,
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
        ref_frame_a: ref_frame_a.ok_or_else(|| missing_field_err(input, "EULER", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b.ok_or_else(|| missing_field_err(input, "EULER", "REF_FRAME_B"))?,
        euler_rot_seq: euler_rot_seq
            .ok_or_else(|| missing_field_err(input, "EULER", "EULER_ROT_SEQ"))?,
        angle_1: angle_1.ok_or_else(|| missing_field_err(input, "EULER", "ANGLE_1"))?,
        angle_2: angle_2.ok_or_else(|| missing_field_err(input, "EULER", "ANGLE_2"))?,
        angle_3: angle_3.ok_or_else(|| missing_field_err(input, "EULER", "ANGLE_3"))?,
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
        // Let's assume string for now if enum is problematic.
        // Wait, common.rs defined AngVelFrameType as enum?
        // Step 32 line 194: restriction base string, but empty enumeration? No, empty restriction means "no restrictions"? No, empty restriction block means nothing specified inside?
        // Actually, if it's empty restriction of string, it's just string?
        // Ah, XSD says <xsd:restriction base="xsd:string"></xsd:restriction>.
        // This is valid, effectively string.
        // I should check `common.rs` definition of `AngVelFrameType`.
        // If it's an enum, I parse as enum. If struct wrapper or type alias, parse accordingly.
        // Assuming string based on XSD.
        // But `kv_enum` works if I added `FromStr`.
        "ANGVEL_X" => angvel_x: kv_from_kvn,
        "ANGVEL_Y" => angvel_y: kv_from_kvn,
        "ANGVEL_Z" => angvel_z: kv_from_kvn,
    }, |i: &mut &str| at_block_end("ANGVEL", i));

    expect_block_end("ANGVEL").parse_next(input)?;

    Ok(AngVelState {
        comment,
        ref_frame_a: ref_frame_a
            .ok_or_else(|| missing_field_err(input, "ANGVEL", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b
            .ok_or_else(|| missing_field_err(input, "ANGVEL", "REF_FRAME_B"))?,
        angvel_frame: angvel_frame
            .ok_or_else(|| missing_field_err(input, "ANGVEL", "ANGVEL_FRAME"))?,
        angvel_x: angvel_x.ok_or_else(|| missing_field_err(input, "ANGVEL", "ANGVEL_X"))?,
        angvel_y: angvel_y.ok_or_else(|| missing_field_err(input, "ANGVEL", "ANGVEL_Y"))?,
        angvel_z: angvel_z.ok_or_else(|| missing_field_err(input, "ANGVEL", "ANGVEL_Z"))?,
    })
}
// Note: `AngVelFrameType` in common might be empty enum or similar. I should check.
// If it leads to compilation error, I'll fix it.

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
        ref_frame_a: ref_frame_a.ok_or_else(|| missing_field_err(input, "SPIN", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b.ok_or_else(|| missing_field_err(input, "SPIN", "REF_FRAME_B"))?,
        spin_alpha: spin_alpha.ok_or_else(|| missing_field_err(input, "SPIN", "SPIN_ALPHA"))?,
        spin_delta: spin_delta.ok_or_else(|| missing_field_err(input, "SPIN", "SPIN_DELTA"))?,
        spin_angle: spin_angle.ok_or_else(|| missing_field_err(input, "SPIN", "SPIN_ANGLE"))?,
        spin_angle_vel: spin_angle_vel
            .ok_or_else(|| missing_field_err(input, "SPIN", "SPIN_ANGLE_VEL"))?,
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
        inertia_ref_frame: inertia_ref_frame
            .ok_or_else(|| missing_field_err(input, "INERTIA", "INERTIA_REF_FRAME"))?,
        ixx: ixx.ok_or_else(|| missing_field_err(input, "INERTIA", "IXX"))?,
        iyy: iyy.ok_or_else(|| missing_field_err(input, "INERTIA", "IYY"))?,
        izz: izz.ok_or_else(|| missing_field_err(input, "INERTIA", "IZZ"))?,
        ixy: ixy.ok_or_else(|| missing_field_err(input, "INERTIA", "IXY"))?,
        ixz: ixz.ok_or_else(|| missing_field_err(input, "INERTIA", "IXZ"))?,
        iyz: iyz.ok_or_else(|| missing_field_err(input, "INERTIA", "IYZ"))?,
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
        "MAN_EPOCH_START" => man_epoch_start: kv_epoch,
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
        man_epoch_start: man_epoch_start
            .ok_or_else(|| missing_field_err(input, "MAN", "MAN_EPOCH_START"))?,
        man_duration: man_duration
            .ok_or_else(|| missing_field_err(input, "MAN", "MAN_DURATION"))?,
        man_ref_frame: man_ref_frame
            .ok_or_else(|| missing_field_err(input, "MAN", "MAN_REF_FRAME"))?,
        man_tor_x: man_tor_x.ok_or_else(|| missing_field_err(input, "MAN", "MAN_TOR_X"))?,
        man_tor_y: man_tor_y.ok_or_else(|| missing_field_err(input, "MAN", "MAN_TOR_Y"))?,
        man_tor_z: man_tor_z.ok_or_else(|| missing_field_err(input, "MAN", "MAN_TOR_Z"))?,
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
    let (epoch, _) = expect_key("EPOCH").parse_next(input)?;
    let epoch = crate::types::Epoch::from_str(epoch)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, FormatError, ValidationError};
    use crate::traits::Ndm;

    fn sample_apm_header() -> String {
        r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
        .to_string()
    }

    fn sample_apm_meta() -> String {
        r#"META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
"#
        .to_string()
    }

    #[test]
    fn test_parse_apm_minimal() {
        let input = format!("{}{}\nEPOCH = 2022-11-04T17:22:31\nQUAT_START\nREF_FRAME_A = EME2000\nREF_FRAME_B = SC_BODY_1\nQ1 = 0.5\nQ2 = 0.5\nQ3 = 0.5\nQC = 0.5\nQUAT_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        assert_eq!(apm.version, "2.0");
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
    }

    #[test]
    fn test_parse_apm_version_error() {
        let input =
            "CCSDS_APM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\n";
        let err = Apm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::Format(boxed_err) => match *boxed_err {
                FormatError::Kvn(e) => {
                    assert!(format!("{:?}", e).contains("1.0 or 2.0"));
                }
                _ => panic!("Expected Kvn format error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Format error, got {:?}", err),
        }
    }

    #[test]
    fn test_apm_missing_mandatory_metadata() {
        let input = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
"#;
        let err = Apm::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
                ValidationError::MissingRequiredField { field, .. } => {
                    assert_eq!(field, "OBJECT_NAME");
                }
                _ => panic!("Expected missing field error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Validation error, got {:?}", err),
        }
    }

    #[test]
    fn test_apm_quaternion_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nQUAT_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nQ1 = 0.1\nQ2 = 0.2\nQ3 = 0.3\nQC = 0.4\nQ1_DOT = 0.01\nQ2_DOT = 0.02\nQ3_DOT = 0.03\nQC_DOT = 0.04\nQUAT_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let q = &apm.body.segment.data.quaternion_state[0];
        assert_eq!(q.quaternion.q1, 0.1);
        assert_eq!(q.quaternion_dot.as_ref().unwrap().q1_dot.value, 0.01);
    }

    #[test]
    fn test_apm_euler_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nEULER_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nEULER_ROT_SEQ = ZYX\nANGLE_1 = 10\nANGLE_2 = 20\nANGLE_3 = 30\nEULER_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let e = &apm.body.segment.data.euler_angle_state[0];
        assert_eq!(e.angle_1.value, 10.0);
        // Note: RotSeq 321 is not valid for the enum unless we fixed it or mapped it.
        // We learned from ACM/AEM it expects ZYX etc. But RotSeq implementation in types.rs might support digits if I checked carefully?
        // Wait, checking types.rs again. `RotSeq` impl `FromStr` matches "XYX", "XYZ" etc.
        // So "321" will fail. I should use "ZYX".
    }

    #[test]
    fn test_apm_euler_block_corrected() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nEULER_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nEULER_ROT_SEQ = ZYX\nANGLE_1 = 10\nANGLE_2 = 20\nANGLE_3 = 30\nEULER_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let e = &apm.body.segment.data.euler_angle_state[0];
        assert_eq!(e.euler_rot_seq, crate::types::RotSeq::ZYX);
    }

    #[test]
    fn test_apm_spin_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nSPIN_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nSPIN_ALPHA = 10\nSPIN_DELTA = 20\nSPIN_ANGLE = 30\nSPIN_ANGLE_VEL = 0.1\nSPIN_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let s = &apm.body.segment.data.spin[0];
        assert_eq!(s.spin_alpha.value, 10.0);
    }

    #[test]
    fn test_apm_inertia_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nINERTIA_START\nINERTIA_REF_FRAME = A\nIXX = 100\nIYY = 200\nIZZ = 300\nIXY = 10\nIXZ = 20\nIYZ = 30\nINERTIA_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let i = &apm.body.segment.data.inertia[0];
        assert_eq!(i.ixx.value, 100.0);
    }

    #[test]
    fn test_apm_man_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nMAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let m = &apm.body.segment.data.maneuver_parameters[0];
        assert_eq!(m.man_duration.value, 10.0);
    }

    #[test]
    fn test_apm_multiple_blocks_mixed() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nQUAT_START\nREF_FRAME_A=A\nREF_FRAME_B=B\nQ1=0\nQ2=0\nQ3=0\nQC=1\nQUAT_STOP\nINERTIA_START\nINERTIA_REF_FRAME=A\nIXX=1\nIYY=2\nIZZ=3\nIXY=0\nIXZ=0\nIYZ=0\nINERTIA_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
        assert_eq!(apm.body.segment.data.inertia.len(), 1);
    }

    #[test]
    fn test_apm_validate_at_least_one_block() {
        let input = format!(
            "{}{}\nEPOCH = 2023-01-01T00:00:00\n",
            sample_apm_header(),
            sample_apm_meta()
        );
        let err = Apm::from_kvn(&input).unwrap_err();
        match err {
            CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
                ValidationError::MissingRequiredField { field, .. } => {
                    assert!(field.contains("logical block"));
                }
                _ => panic!("Expected missing logical block error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Validation error, got {:?}", err),
        }
    }
    #[test]
    fn test_parse_apm_angvel_block() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nANGVEL_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nANGVEL_FRAME = B\nANGVEL_X = 1.0\nANGVEL_Y = 2.0\nANGVEL_Z = 3.0\nANGVEL_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let av = &apm.body.segment.data.angular_velocity[0];
        assert_eq!(av.angvel_z.value, 3.0);
        assert_eq!(av.angvel_frame.0, "B");
    }

    #[test]
    fn test_parse_apm_spin_full() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nSPIN_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nSPIN_ALPHA = 10\nSPIN_DELTA = 20\nSPIN_ANGLE = 30\nSPIN_ANGLE_VEL = 0.1\nNUTATION = 5.0\nNUTATION_PER = 100.0\nNUTATION_PHASE = 45.0\nMOMENTUM_ALPHA = 1.0\nMOMENTUM_DELTA = 2.0\nNUTATION_VEL = 0.05\nSPIN_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let s = &apm.body.segment.data.spin[0];
        assert_eq!(s.nutation.as_ref().unwrap().value, 5.0);
        assert_eq!(s.momentum_delta.as_ref().unwrap().value, 2.0);
    }

    #[test]
    fn test_parse_apm_euler_derivatives() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nEULER_START\nREF_FRAME_A = A\nREF_FRAME_B = B\nEULER_ROT_SEQ = ZYX\nANGLE_1 = 10\nANGLE_2 = 20\nANGLE_3 = 30\nANGLE_1_DOT = 0.1\nANGLE_2_DOT = 0.2\nANGLE_3_DOT = 0.3\nEULER_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let e = &apm.body.segment.data.euler_angle_state[0];
        assert_eq!(e.angle_3_dot.as_ref().unwrap().value, 0.3);
    }

    #[test]
    fn test_parse_apm_maneuver_delta_mass() {
        let input = format!("{}{}\nEPOCH = 2023-01-01T00:00:00\nMAN_START\nMAN_EPOCH_START = 2023-01-01T01:00:00\nMAN_DURATION = 10\nMAN_REF_FRAME = A\nMAN_TOR_X = 1\nMAN_TOR_Y = 2\nMAN_TOR_Z = 3\nMAN_DELTA_MASS = -1.5\nMAN_STOP\n",
            sample_apm_header(), sample_apm_meta());
        let apm = Apm::from_kvn(&input).unwrap();
        let m = &apm.body.segment.data.maneuver_parameters[0];
        assert_eq!(m.man_delta_mass.as_ref().unwrap().value, -1.5);
    }
}
