// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for AEM (Attitude Ephemeris Message).

use super::{Aem, AemBody, AemData, AemMetadata, AemSegment};
use crate::common::AemAttitudeState;
use crate::error::{CcsdsNdmError, FormatError, InternalParserError, KvnParseError, Result};
use crate::kvn::parser::*;
use crate::kvn::ser::KvnWriter;
use crate::parse_block;
use crate::traits::ToKvn;
use crate::types::{AttitudeTypeType, InterpolationDegree};
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
        "START_TIME" => start_time: kv_calendar_epoch,
        "USEABLE_START_TIME" => useable_start_time: kv_calendar_epoch,
        "USEABLE_STOP_TIME" => useable_stop_time: kv_calendar_epoch,
        "STOP_TIME" => stop_time: kv_calendar_epoch,
        "ATTITUDE_TYPE" => attitude_type: kv_enum,
        "EULER_ROT_SEQ" => euler_rot_seq: kv_enum,
        "RATE_FRAME" => rate_frame: kv_string, // CCSDS 504.0-B-2 says RATE_FRAME in KVN
        "INTERPOLATION_METHOD" => interpolation_method: kv_string,
        "INTERPOLATION_DEGREE" => interpolation_degree: kv_u32,
    }, |i: &mut &str| at_block_end("META", i));

    expect_block_end("META").parse_next(input)?;

    Ok(AemMetadata {
        comment,
        object_name: require_field(input, "AEM Metadata", "OBJECT_NAME", object_name)?,
        object_id: require_field(input, "AEM Metadata", "OBJECT_ID", object_id)?,
        center_name,
        ref_frame_a: require_field(input, "AEM Metadata", "REF_FRAME_A", ref_frame_a)?,
        ref_frame_b: require_field(input, "AEM Metadata", "REF_FRAME_B", ref_frame_b)?,
        time_system: require_field(input, "AEM Metadata", "TIME_SYSTEM", time_system)?,
        start_time: require_field(input, "AEM Metadata", "START_TIME", start_time)?,
        useable_start_time,
        useable_stop_time,
        stop_time: require_field(input, "AEM Metadata", "STOP_TIME", stop_time)?,
        attitude_type: require_field(input, "AEM Metadata", "ATTITUDE_TYPE", attitude_type)?,
        euler_rot_seq,
        angvel_frame: rate_frame,
        interpolation_method,
        interpolation_degree: interpolation_degree
            .map(|degree| {
                std::num::NonZeroU32::new(degree)
                    .map(InterpolationDegree)
                    .ok_or_else(|| cut_err(input, "positive integer"))
            })
            .transpose()?,
    })
}

//----------------------------------------------------------------------
// AEM Data Parser
//----------------------------------------------------------------------

/// Parses a single data line.
/// Parses a single data line based on ATTITUDE_TYPE.
fn attitude_state_line(
    input: &mut &str,
    attitude_type: &AttitudeTypeType,
) -> KvnResult<AemAttitudeState> {
    let record_start = input.checkpoint();
    let line = terminated(raw_line, opt_line_ending).parse_next(input)?;
    let mut parts = line.split_whitespace();

    let epoch_str = parts.next().ok_or_else(|| {
        ErrMode::Cut(InternalParserError::from_input(input).add_context(
            input,
            &input.checkpoint(),
            winnow::error::StrContext::Label("Epoch in data line"),
        ))
    })?;

    let epoch = crate::types::CalendarEpoch::from_str(epoch_str)
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))?;

    let expected_values = attitude_type.value_count();
    let mut values = [0.0; 8];
    let mut value_count = 0usize;
    for s in parts {
        if !valid_ccsds_number(s) {
            return Err(ErrMode::Cut(
                InternalParserError::from_input(input).add_context(
                    input,
                    &input.checkpoint(),
                    winnow::error::StrContext::Label("CCSDS number in data line"),
                ),
            ));
        }
        let val = fast_float::parse(s).map_err(|_| {
            ErrMode::Cut(InternalParserError::from_input(input).add_context(
                input,
                &input.checkpoint(),
                winnow::error::StrContext::Label("Float value in data line"),
            ))
        })?;
        if value_count == values.len() {
            return Err(ErrMode::Cut(InternalParserError::from_input(input)));
        }
        values[value_count] = val;
        value_count += 1;
    }
    if value_count != expected_values {
        return Err(ErrMode::Cut(InternalParserError::from_input(input)));
    }

    let state = AemAttitudeState::from_values(epoch, &values[..value_count], attitude_type)
        .ok_or_else(|| ErrMode::Cut(InternalParserError::from_input(input)))?;
    if let Err(error) = state.validate_values() {
        input.reset(&record_start);
        return Err(ErrMode::Cut(InternalParserError::from_external_error(
            input, error,
        )));
    }
    Ok(state)
}

/// Parses the AEM data section.
pub fn aem_data(input: &mut &str, attitude_type: &AttitudeTypeType) -> KvnResult<AemData> {
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
        attitude_states.push(state);

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

pub(super) fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const HEADER: &[&str] = &[
        "CLASSIFICATION",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
    ];
    const META: &[&str] = &[
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "TIME_SYSTEM",
        "START_TIME",
        "USEABLE_START_TIME",
        "USEABLE_STOP_TIME",
        "STOP_TIME",
        "ATTITUDE_TYPE",
        "EULER_ROT_SEQ",
        "RATE_FRAME",
        "INTERPOLATION_METHOD",
        "INTERPOLATION_DEGREE",
    ];
    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating AEM KVN structure"],
            offset,
        }))))
    };
    let mut block = None;
    let mut last_block = None;
    let mut previous_key = None;
    let mut top_rank = None;
    let mut block_has_content = false;
    let mut offset = 0usize;
    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
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
            if block.is_none() && top_rank != Some(0) {
                return fail("AEM header COMMENT must immediately follow the version record");
            }
            if block_has_content && !(block.is_none() && top_rank == Some(0)) {
                return fail("COMMENT is not at the beginning of an AEM logical block");
            }
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if block.is_some() || !matches!(marker, "META" | "DATA") {
                return fail("unknown or nested AEM marked block");
            }
            if (marker == "META" && matches!(last_block, Some("META")))
                || (marker == "DATA" && last_block != Some("META"))
            {
                return fail("out-of-order AEM marked block");
            }
            block = Some(marker);
            previous_key = None;
            block_has_content = false;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if block != Some(marker) {
                return fail("mismatched AEM marked block end");
            }
            block = None;
            last_block = Some(marker);
            previous_key = None;
            block_has_content = false;
            offset += raw_line.len() + 1;
            continue;
        }
        match block {
            Some("DATA") => {
                if line.contains('=') {
                    return fail("assignment in AEM attitude-state history");
                }
                block_has_content = true;
            }
            Some("META") => {
                if !line.contains('=') {
                    return fail("expected one AEM metadata assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = META
                    .iter()
                    .position(|candidate| *candidate == key)
                    .ok_or_else(|| {
                        invalid(number, offset, "unknown AEM metadata keyword".into())
                    })?;
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order AEM metadata keyword");
                }
                previous_key = Some(rank);
                block_has_content = true;
            }
            None => {
                if last_block.is_some() {
                    return fail("content outside an AEM marked block");
                }
                if !line.contains('=') {
                    return fail("expected one AEM header assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = if key == "CCSDS_AEM_VERS" {
                    0
                } else {
                    HEADER
                        .iter()
                        .position(|candidate| *candidate == key)
                        .map(|rank| rank + 1)
                        .ok_or_else(|| {
                            invalid(number, offset, "unknown AEM header keyword".into())
                        })?
                };
                if top_rank.is_none() && rank != 0 {
                    return fail("CCSDS_AEM_VERS must be the first record");
                }
                if top_rank.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order AEM header keyword");
                }
                top_rank = Some(rank);
                block_has_content = true;
            }
            _ => unreachable!(),
        }
        offset += raw_line.len() + 1;
    }
    if block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unclosed AEM marked block".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Aem {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_AEM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl ToKvn for AemBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for seg in &self.segment {
            seg.write_kvn(writer);
        }
    }
}

impl ToKvn for AemSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_line("META_START");
        self.metadata.write_kvn(writer);
        writer.write_line("META_STOP");
        writer.write_line("");
        writer.write_line("DATA_START");
        self.data.write_kvn(writer);
        writer.write_line("DATA_STOP");
        writer.write_line("");
    }
}

impl ToKvn for AemMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("START_TIME", self.start_time);
        if let Some(v) = self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        writer.write_pair("STOP_TIME", self.stop_time);
        writer.write_pair("ATTITUDE_TYPE", &self.attitude_type);
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.angvel_frame {
            writer.write_pair("RATE_FRAME", v);
        }
        if let Some(v) = &self.interpolation_method {
            writer.write_pair("INTERPOLATION_METHOD", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

impl ToKvn for AemData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        for state in &self.attitude_states {
            state.write_kvn(writer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CcsdsNdmError, FormatError, ValidationError};
    use crate::traits::Ndm;

    fn sample_aem_header() -> String {
        r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
"#
        .to_string()
    }

    fn sample_aem_meta() -> String {
        r#"META_START
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
"#
        .to_string()
    }

    #[test]
    fn test_parse_aem_minimal() {
        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\nDATA_STOP\n",
            sample_aem_header(),
            sample_aem_meta()
        );
        let aem = Aem::from_kvn(&input).unwrap();
        assert_eq!(aem.version, "2.0");
        assert_eq!(aem.header.originator, "NASA/JPL");
        assert_eq!(aem.body.segment.len(), 1);
    }

    #[test]
    fn test_parse_aem_version_error() {
        let input = r#"CCSDS_AEM_VERS = 3.0
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
DATA_STOP
"#;
        let err = Aem::from_kvn(input).unwrap_err();
        match err {
            CcsdsNdmError::UnsupportedInputVersion { version, .. } => {
                assert_eq!(version, "3.0");
            }
            _ => panic!("Expected unsupported input version, got {:?}", err),
        }
    }

    #[test]
    fn test_aem_missing_mandatory_metadata() {
        // Missing OBJECT_NAME
        let input = r#"CCSDS_AEM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_ID = SAT1
CENTER_NAME = EARTH
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        let err = Aem::from_kvn(input).unwrap_err();
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

    fn parse_attitude_state(attitude_type: &str, values: &str) -> AemAttitudeState {
        let mut meta = sample_aem_meta().replace("QUATERNION", attitude_type);
        let mut extra = String::new();
        if attitude_type.starts_with("EULER") {
            extra.push_str("EULER_ROT_SEQ = ZYX\n");
        }
        if attitude_type.contains("ANGVEL") {
            extra.push_str("RATE_FRAME = SC_BODY_1\n");
        }
        if !extra.is_empty() {
            meta = meta.replace("META_STOP", &format!("{extra}META_STOP"));
        }
        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 {values}\nDATA_STOP\n",
            sample_aem_header(),
            meta
        );
        Aem::from_kvn(&input).unwrap().body.segment[0]
            .data
            .attitude_states[0]
            .clone()
    }

    // ADM table 4-4: compare every named component in its KVN column order.
    fn parsed_type_and_values(state: &AemAttitudeState) -> (AttitudeTypeType, Vec<f64>) {
        match state {
            AemAttitudeState::QuaternionEphemeris(value) => (
                AttitudeTypeType::Quaternion,
                vec![
                    value.quaternion.q1,
                    value.quaternion.q2,
                    value.quaternion.q3,
                    value.quaternion.qc,
                ],
            ),
            AemAttitudeState::QuaternionDerivative(value) => (
                AttitudeTypeType::QuaternionDerivative,
                vec![
                    value.quaternion.q1,
                    value.quaternion.q2,
                    value.quaternion.q3,
                    value.quaternion.qc,
                    value.quaternion_dot.q1_dot.value,
                    value.quaternion_dot.q2_dot.value,
                    value.quaternion_dot.q3_dot.value,
                    value.quaternion_dot.qc_dot.value,
                ],
            ),
            AemAttitudeState::QuaternionAngVel(value) => (
                AttitudeTypeType::QuaternionAngVel,
                vec![
                    value.quaternion.q1,
                    value.quaternion.q2,
                    value.quaternion.q3,
                    value.quaternion.qc,
                    value.ang_vel.angvel_x.value,
                    value.ang_vel.angvel_y.value,
                    value.ang_vel.angvel_z.value,
                ],
            ),
            AemAttitudeState::EulerAngle(value) => (
                AttitudeTypeType::EulerAngle,
                vec![
                    value.angle_1.value,
                    value.angle_2.value,
                    value.angle_3.value,
                ],
            ),
            AemAttitudeState::EulerAngleDerivative(value) => (
                AttitudeTypeType::EulerAngleDerivative,
                vec![
                    value.angle_1.value,
                    value.angle_2.value,
                    value.angle_3.value,
                    value.angle_1_dot.value,
                    value.angle_2_dot.value,
                    value.angle_3_dot.value,
                ],
            ),
            AemAttitudeState::EulerAngleAngVel(value) => (
                AttitudeTypeType::EulerAngleAngVel,
                vec![
                    value.angle_1.value,
                    value.angle_2.value,
                    value.angle_3.value,
                    value.angvel_x.value,
                    value.angvel_y.value,
                    value.angvel_z.value,
                ],
            ),
            AemAttitudeState::Spin(value) => (
                AttitudeTypeType::Spin,
                vec![
                    value.spin_alpha.value,
                    value.spin_delta.value,
                    value.spin_angle.value,
                    value.spin_angle_vel.value,
                ],
            ),
            AemAttitudeState::SpinNutation(value) => (
                AttitudeTypeType::SpinNutation,
                vec![
                    value.spin_alpha.value,
                    value.spin_delta.value,
                    value.spin_angle.value,
                    value.spin_angle_vel.value,
                    value.nutation.value,
                    value.nutation_per.value,
                    value.nutation_phase.value,
                ],
            ),
            AemAttitudeState::SpinNutationMom(value) => (
                AttitudeTypeType::SpinNutationMom,
                vec![
                    value.spin_alpha.value,
                    value.spin_delta.value,
                    value.spin_angle.value,
                    value.spin_angle_vel.value,
                    value.momentum_alpha.value,
                    value.momentum_delta.value,
                    value.nutation_vel.value,
                ],
            ),
        }
    }

    #[test]
    fn test_parse_aem_attitude_types() {
        let cases = [
            (
                "QUATERNION",
                "0.0 0.36 0.48 0.8",
                AttitudeTypeType::Quaternion,
            ),
            (
                "QUATERNION/DERIVATIVE",
                "0.0 0.36 0.48 0.8 0.5 0.6 0.7 0.9",
                AttitudeTypeType::QuaternionDerivative,
            ),
            (
                "QUATERNION/ANGVEL",
                "0.0 0.36 0.48 0.8 0.01 0.02 0.03",
                AttitudeTypeType::QuaternionAngVel,
            ),
            (
                "EULER_ANGLE",
                "10.0 20.0 30.0",
                AttitudeTypeType::EulerAngle,
            ),
            (
                "EULER_ANGLE/DERIVATIVE",
                "10.0 20.0 30.0 0.1 0.2 0.3",
                AttitudeTypeType::EulerAngleDerivative,
            ),
            (
                "EULER_ANGLE/ANGVEL",
                "10.0 20.0 30.0 0.1 0.2 0.3",
                AttitudeTypeType::EulerAngleAngVel,
            ),
            ("SPIN", "10.0 20.0 30.0 0.1", AttitudeTypeType::Spin),
            (
                "SPIN/NUTATION",
                "10.0 20.0 30.0 0.1 5.0 100.0 45.0",
                AttitudeTypeType::SpinNutation,
            ),
            (
                "SPIN/NUTATION_MOM",
                "10.0 20.0 30.0 0.1 5.0 6.0 0.05",
                AttitudeTypeType::SpinNutationMom,
            ),
        ];

        for (attitude_type, values, expected_type) in cases {
            let state = parse_attitude_state(attitude_type, values);
            let (actual_type, actual_values) = parsed_type_and_values(&state);
            let expected_values: Vec<f64> = values
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();
            assert_eq!(actual_type, expected_type);
            assert_eq!(
                actual_values, expected_values,
                "{attitude_type} column order"
            );
        }
    }

    #[test]
    fn test_parse_aem_quaternion_angvel_requires_angvel_frame() {
        let qr_meta = sample_aem_meta().replace("QUATERNION", "QUATERNION/ANGVEL");
        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.1 0.2 0.3 0.4 0.01 0.02 0.03\nDATA_STOP\n",
            sample_aem_header(),
            qr_meta
        );
        let err = Aem::from_kvn(&input).unwrap_err();
        match err {
            CcsdsNdmError::Validation(boxed_err) => match *boxed_err {
                ValidationError::MissingRequiredField { field, .. } => {
                    assert_eq!(field, "ANGVEL_FRAME (required for ANGVEL types)");
                }
                _ => panic!("Expected missing field error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Validation error, got {:?}", err),
        }
    }

    #[test]
    fn test_parse_aem_invalid_lines() {
        let cases = [
            ("wrong quaternion columns", sample_aem_meta(), "0.1 0.2 0.3"),
            (
                "out-of-range normalized quaternion",
                sample_aem_meta(),
                "1.0001 0 0 0",
            ),
            (
                "wrong derivative columns",
                sample_aem_meta().replace("QUATERNION", "QUATERNION/DERIVATIVE"),
                "0.1 0.2 0.3 0.4",
            ),
            (
                "wrong Euler columns",
                sample_aem_meta()
                    .replace("QUATERNION", "EULER_ANGLE")
                    .replace("META_STOP", "EULER_ROT_SEQ = ZYX\nMETA_STOP"),
                "10.0 20.0",
            ),
            (
                "invalid epoch",
                sample_aem_meta(),
                "NOT_A_DATE 0.1 0.2 0.3 0.4",
            ),
            (
                "invalid float",
                sample_aem_meta(),
                "2002-11-04T17:22:31 0.1 NOT_A_NUMBER 0.3 0.4",
            ),
        ];

        for (label, metadata, values) in cases {
            let input = format!(
                "{}{}\nDATA_START\n{values}\nDATA_STOP\n",
                sample_aem_header(),
                metadata
            );
            assert!(Aem::from_kvn(&input).is_err(), "accepted {label}");
        }

        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.1 0.2 0.3 0.4\n",
            sample_aem_header(),
            sample_aem_meta()
        );
        assert!(
            Aem::from_kvn(&input).is_err(),
            "accepted an unterminated block"
        );
    }

    #[test]
    fn test_aem_value_error_reports_record_line() {
        let bad_record = "2002-11-04T17:23:00 1.5 0 0 0";
        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\n{bad_record}\nDATA_STOP\n",
            sample_aem_header(),
            sample_aem_meta()
        );
        let expected_line = input[..input.find(bad_record).unwrap()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1;

        let CcsdsNdmError::Validation(error) = Aem::from_kvn(&input).unwrap_err() else {
            panic!("expected validation error");
        };
        let ValidationError::OutOfRange { name, line, .. } = *error else {
            panic!("expected quaternion component range error");
        };
        assert_eq!(name, "Q1");
        assert_eq!(line, Some(expected_line));
    }

    #[test]
    fn test_parse_aem_multiple_segments() {
        let seg1 = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.5 0.5 0.5 0.5\nDATA_STOP\n",
            "",
            sample_aem_meta()
        );
        let seg2 = format!(
            "{}\nDATA_START\n2002-11-04T17:23:00 0.5 0.5 0.5 0.5\nDATA_STOP\n",
            sample_aem_meta()
        ); // Re-use meta for simplicity

        let input = format!("{}{}{}", sample_aem_header(), seg1, seg2);
        let aem = Aem::from_kvn(&input).unwrap();
        assert_eq!(aem.body.segment.len(), 2);
    }

    #[test]
    fn test_parse_aem_comments() {
        let input = r#"CCSDS_AEM_VERS = 2.0
COMMENT Header comment
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST

META_START
COMMENT Meta comment
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP

DATA_START
COMMENT Data comment
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        let aem = Aem::from_kvn(input).unwrap();
        assert!(aem.header.comment.contains(&"Header comment".to_string()));
        assert!(aem.body.segment[0]
            .metadata
            .comment
            .contains(&"Meta comment".to_string()));
        assert!(aem.body.segment[0]
            .data
            .comment
            .contains(&"Data comment".to_string()));
    }
    #[test]
    fn test_parse_aem_rate_frame_alias() {
        let meta = sample_aem_meta()
            .replace("QUATERNION", "QUATERNION/ANGVEL")
            .replace("META_STOP", "RATE_FRAME = SC_BODY_1\nMETA_STOP");

        // QUATERNION/ANGVEL needs 7 columns
        let input = format!(
            "{}{}\nDATA_START\n2002-11-04T17:22:31 0.0 0.0 0.0 1.0 0.01 0.02 0.03\nDATA_STOP\n",
            sample_aem_header(),
            meta
        );

        let aem = Aem::from_kvn(&input).unwrap();
        // Check that proper parsing happened (RATE_FRAME maps to angvel_frame)
        assert_eq!(
            aem.body.segment[0].metadata.angvel_frame.as_deref(),
            Some("SC_BODY_1")
        );
    }

    #[test]
    fn test_aem_no_segments() {
        // Valid header but no body segments
        let input = sample_aem_header().to_string();
        let err = Aem::from_kvn(&input).unwrap_err();
        match err {
            CcsdsNdmError::Format(boxed_err) => match *boxed_err {
                FormatError::Kvn(e) => {
                    assert!(format!("{:?}", e).contains("At least one segment required"));
                }
                _ => panic!("Expected Kvn format error, got {:?}", boxed_err),
            },
            _ => panic!("Expected Format error, got {:?}", err),
        }
    }
}
