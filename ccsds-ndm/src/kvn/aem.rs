// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for AEM (Attitude Ephemeris Message).

use crate::common::{
    AemAttitudeState, AngVel, EulerAngle, Quaternion, QuaternionAngVel, QuaternionDerivative,
    QuaternionDot, QuaternionEphemeris,
};
use crate::error::InternalParserError;
use crate::kvn::parser::*;
use crate::messages::aem::{Aem, AemBody, AemData, AemMetadata, AemSegment};
use crate::parse_block;
use crate::types::Angle;
use std::str::FromStr;
use winnow::combinator::{peek, terminated};
use winnow::error::{AddContext, ErrMode, FromExternalError};
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// AEM Version Parser
//----------------------------------------------------------------------

/// Parses the AEM version line: `CCSDS_AEM_VERS = 2.0`
pub fn aem_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_AEM_VERS").parse_next(input)?;
    if value != "1.0" && value != "2.0" {
        return Err(cut_err(input, "1.0 or 2.0"));
    }
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// AEM Metadata Parser
//----------------------------------------------------------------------

/// Parses the AEM metadata section.
pub fn aem_metadata(input: &mut &str) -> KvnResult<AemMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame_a = None;
    let mut ref_frame_b = None;
    let mut time_system = None;
    let mut start_time = None;
    let mut useable_start_time = None;
    let mut useable_stop_time = None;
    let mut stop_time = None;
    let mut attitude_type = None;
    let mut euler_rot_seq = None;
    let mut rate_frame = None;
    let mut interpolation_method = None;
    let mut interpolation_degree = None;

    expect_block_start("META").parse_next(input)?;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "REF_FRAME_A" => ref_frame_a: kv_string,
        "REF_FRAME_B" => ref_frame_b: kv_string,
        "TIME_SYSTEM" => time_system: kv_string,
        "START_TIME" => start_time: kv_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_epoch,
        "STOP_TIME" => stop_time: kv_epoch,
        "ATTITUDE_TYPE" => attitude_type: kv_string,
        "EULER_ROT_SEQ" => euler_rot_seq: kv_enum,
        "RATE_FRAME" => rate_frame: kv_string, // CCSDS 504.0-B-2 says RATE_FRAME in KVN
        "ANGVEL_FRAME" => rate_frame: kv_string, // Also support explicit XML tag if used in KVN
        "INTERPOLATION_METHOD" => interpolation_method: kv_string,
        "INTERPOLATION_DEGREE" => interpolation_degree: kv_u32,
    }, |i: &mut &str| at_block_end("META", i));

    expect_block_end("META").parse_next(input)?;

    Ok(AemMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "OBJECT_NAME"))?,
        object_id: object_id
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "OBJECT_ID"))?,
        center_name,
        ref_frame_a: ref_frame_a
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "REF_FRAME_A"))?,
        ref_frame_b: ref_frame_b
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "REF_FRAME_B"))?,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "TIME_SYSTEM"))?,
        start_time: start_time
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "START_TIME"))?,
        useable_start_time,
        useable_stop_time,
        stop_time: stop_time
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "STOP_TIME"))?,
        attitude_type: attitude_type
            .ok_or_else(|| missing_field_err(input, "AEM Metadata", "ATTITUDE_TYPE"))?,
        euler_rot_seq,
        angvel_frame: rate_frame,
        interpolation_method,
        interpolation_degree: interpolation_degree.and_then(std::num::NonZeroU32::new),
    })
}

//----------------------------------------------------------------------
// AEM Data Parser
//----------------------------------------------------------------------

/// Parses a single data line.
/// Parses a single data line based on ATTITUDE_TYPE.
fn attitude_state_line(input: &mut &str, attitude_type: &str) -> KvnResult<AemAttitudeState> {
    let line = terminated(raw_line, opt_line_ending).parse_next(input)?;
    let mut parts = line.split_whitespace();

    let epoch_str = parts.next().ok_or_else(|| {
        ErrMode::Cut(InternalParserError::from_input(input).add_context(
            input,
            &input.checkpoint(),
            winnow::error::StrContext::Label("Epoch in data line"),
        ))
    })?;

    let epoch = crate::types::Epoch::from_str(epoch_str)
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;

    let mut values = Vec::new();
    for s in parts {
        let val = s.parse::<f64>().map_err(|_| {
            ErrMode::Cut(InternalParserError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                winnow::error::StrContext::Label("Float value in data line"),
            ))
        })?;
        values.push(val);
    }

    match attitude_type {
        "QUATERNION" => {
            if values.len() != 4 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            let q = Quaternion::new(values[0], values[1], values[2], values[3])
                .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;
            Ok(AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
                epoch,
                quaternion: q,
            }))
        }
        "QUATERNION/DERIVATIVE" => {
            if values.len() != 8 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            let q = Quaternion::new(values[0], values[1], values[2], values[3])
                .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;
            let q_dot = QuaternionDot {
                q1_dot: crate::types::QuaternionDotComponent::new(values[4], None),
                q2_dot: crate::types::QuaternionDotComponent::new(values[5], None),
                q3_dot: crate::types::QuaternionDotComponent::new(values[6], None),
                qc_dot: crate::types::QuaternionDotComponent::new(values[7], None),
            };
            Ok(AemAttitudeState::QuaternionDerivative(
                QuaternionDerivative {
                    epoch,
                    quaternion: q,
                    quaternion_dot: q_dot,
                },
            ))
        }
        "QUATERNION/RATE" => {
            if values.len() != 7 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            let q = Quaternion::new(values[0], values[1], values[2], values[3])
                .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;
            let ang_vel = AngVel {
                angvel_x: crate::types::AngleRate::new(values[4], None),
                angvel_y: crate::types::AngleRate::new(values[5], None),
                angvel_z: crate::types::AngleRate::new(values[6], None),
            };
            Ok(AemAttitudeState::QuaternionAngVel(QuaternionAngVel {
                epoch,
                quaternion: q,
                ang_vel,
            }))
        }
        "EULER_ANGLE" => {
            if values.len() != 3 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::EulerAngle(EulerAngle {
                epoch,
                angle_1: Angle::new(values[0], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
                angle_2: Angle::new(values[1], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
                angle_3: Angle::new(values[2], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
            }))
        }
        "EULER_ANGLE/DERIVATIVE" => {
            if values.len() != 6 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::EulerAngleDerivative(
                crate::common::EulerAngleDerivative {
                    epoch,
                    angle_1: Angle::new(values[0], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angle_2: Angle::new(values[1], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angle_3: Angle::new(values[2], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angle_1_dot: crate::types::AngleRate::new(values[3], None),
                    angle_2_dot: crate::types::AngleRate::new(values[4], None),
                    angle_3_dot: crate::types::AngleRate::new(values[5], None),
                },
            ))
        }
        "EULER_ANGLE/RATE" => {
            if values.len() != 6 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::EulerAngleAngVel(
                crate::common::EulerAngleAngVel {
                    epoch,
                    angle_1: Angle::new(values[0], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angle_2: Angle::new(values[1], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angle_3: Angle::new(values[2], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    angvel_x: crate::types::AngleRate::new(values[3], None),
                    angvel_y: crate::types::AngleRate::new(values[4], None),
                    angvel_z: crate::types::AngleRate::new(values[5], None),
                },
            ))
        }
        "SPIN" => {
            if values.len() != 4 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::Spin(crate::common::Spin {
                epoch,
                spin_alpha: Angle::new(values[0], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
                spin_delta: Angle::new(values[1], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
                spin_angle: Angle::new(values[2], None).map_err(|e| {
                    ErrMode::Cut(InternalParserError::from_external_error(input, e))
                })?,
                spin_angle_vel: crate::types::AngleRate::new(values[3], None),
            }))
        }
        "SPIN/NUTATION" => {
            if values.len() != 7 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::SpinNutation(
                crate::common::SpinNutation {
                    epoch,
                    spin_alpha: Angle::new(values[0], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_delta: Angle::new(values[1], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_angle: Angle::new(values[2], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_angle_vel: crate::types::AngleRate::new(values[3], None),
                    nutation: Angle::new(values[4], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    nutation_per: crate::types::Duration::new(values[5], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    nutation_phase: Angle::new(values[6], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                },
            ))
        }
        "SPIN/NUTATION_MOM" => {
            if values.len() != 7 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            Ok(AemAttitudeState::SpinNutationMom(
                crate::common::SpinNutationMom {
                    epoch,
                    spin_alpha: Angle::new(values[0], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_delta: Angle::new(values[1], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_angle: Angle::new(values[2], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    spin_angle_vel: crate::types::AngleRate::new(values[3], None),
                    momentum_alpha: Angle::new(values[4], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    momentum_delta: Angle::new(values[5], None).map_err(|e| {
                        ErrMode::Cut(InternalParserError::from_external_error(input, e))
                    })?,
                    nutation_vel: crate::types::AngleRate::new(values[6], None),
                },
            ))
        }
        _ => Err(ErrMode::Cut(InternalParserError::from_input(input))), // Unexpected type
    }
}

/// Parses the AEM data section.
pub fn aem_data(input: &mut &str, attitude_type: &str) -> KvnResult<AemData> {
    expect_block_start("DATA").parse_next(input)?;

    let mut comment = Vec::new();
    comment.extend(collect_comments.parse_next(input)?);

    let mut attitude_states = Vec::new();

    loop {
        if at_block_end("DATA", input) {
            break;
        }
        let start = input.checkpoint();
        if peek((ws, "COMMENT")).parse_next(input).is_ok() {
            comment.extend(collect_comments.parse_next(input)?);
            if input.offset_from(&start) == 0 {
                return Err(ErrMode::Cut(InternalParserError::from_input(input)));
            }
            continue;
        }

        let checkpoint = input.checkpoint();
        let state = attitude_state_line(input, attitude_type)?;
        attitude_states.push(state.into());

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    expect_block_end("DATA").parse_next(input)?;

    Ok(AemData {
        comment,
        attitude_states,
    })
}

//----------------------------------------------------------------------
// AEM Segment Parser
//----------------------------------------------------------------------

pub fn aem_segment(input: &mut &str) -> KvnResult<AemSegment> {
    let metadata = aem_metadata.parse_next(input)?;
    let _ = skip_empty_lines.parse_next(input);
    let data = aem_data(input, &metadata.attitude_type)?;

    Ok(AemSegment { metadata, data })
}

//----------------------------------------------------------------------
// Complete AEM Parser
//----------------------------------------------------------------------

pub fn parse_aem(input: &mut &str) -> KvnResult<Aem> {
    let version = aem_version.parse_next(input)?;
    let header = adm_header.parse_next(input)?;

    let mut segments = Vec::new();
    loop {
        let checkpoint = input.checkpoint();
        let _ = skip_empty_lines.parse_next(input);
        // If we see META_START, it's a new segment
        if at_block_start("META", input) {
            segments.push(aem_segment.parse_next(input)?);
        } else {
            break;
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    if segments.is_empty() {
        return Err(cut_err(input, "At least one segment required"));
    }

    Ok(Aem {
        header,
        body: AemBody { segment: segments },
        id: Some("CCSDS_AEM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Aem {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_aem.parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    #[test]
    fn test_parse_aem_minimal() {
        let input = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL

META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
CENTER_NAME = MARS BARYCENTER
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2002-11-04T17:22:31
STOP_TIME = 2002-11-04T17:25:31
ATTITUDE_TYPE = QUATERNION
META_STOP

DATA_START
2002-11-04T17:22:31 0.5 0.5 0.5 0.5
2002-11-04T17:23:31 0.6 0.4 0.4 0.6
DATA_STOP
"#;
        let aem = Aem::from_kvn(input).unwrap();
        assert_eq!(aem.version, "2.0");
        assert_eq!(aem.header.originator, "NASA/JPL");
        assert_eq!(aem.body.segment.len(), 1);
        let seg = &aem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert_eq!(seg.data.attitude_states.len(), 2);
        match &seg.data.attitude_states[0].content().unwrap() {
            AemAttitudeState::QuaternionEphemeris(q) => {
                assert_eq!(q.quaternion.q1, 0.5);
            }
            _ => panic!("Expected QuaternionEphemeris"),
        }
    }
}
