// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;

use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result, ValidationError};
use crate::kvn::parser::KvnResult;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::SensorNoise;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::io::Write;

//----------------------------------------------------------------------
// Root ACM Structure
//----------------------------------------------------------------------

/// Attitude Comprehensive Message (ACM).
///
/// An ACM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The ACM aggregates and extends APM and AEM content in a single
/// comprehensive hybrid message.
///
/// Capabilities include:
/// - Optional rate data elements
/// - Optional spacecraft physical properties
/// - Optional covariance elements
/// - Optional maneuver parameters
/// - Optional estimator information
///
/// **CCSDS Reference**: 504.0-B-2, Section 5.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "acm")]
pub struct Acm {
    pub header: AdmHeader,
    pub body: AcmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_ACM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "2.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Acm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Acm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Acm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        validate_kvn_syntax(kvn)?;
        let acm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&acm)?;
        Ok(acm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"acm", "ACM")?;
        validate_xml_sequences(xml)?;
        let acm: Self = crate::xml::from_str_with_context(xml, "ACM")?;
        crate::traits::Validate::validate(&acm)?;
        Ok(acm)
    }
}

impl Acm {
    pub(crate) fn validate_xml_representability(&self) -> Result<()> {
        if self
            .body
            .segment
            .data
            .ad
            .iter()
            .flat_map(|determination| &determination.sensors)
            .any(|sensor| !sensor.comment.is_empty())
        {
            return Err(ValidationError::InvalidValue {
                field: "SENSOR COMMENT".into(),
                value: "present".into(),
                expected: "omitted; ACM 2.0 XML sensorData has no COMMENT element".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        let invalid_number = || {
            CcsdsNdmError::Validation(Box::new(ValidationError::Generic {
                message: Cow::Borrowed("ACM KVN numbers must be representable CCSDS numbers"),
                line: None,
            }))
        };
        let check_number = |value: f64| {
            if crate::kvn::ser::OdmFloat::is_valid(value) {
                Ok(())
            } else {
                Err(invalid_number())
            }
        };
        let check_values = |values: &[f64]| {
            if values
                .iter()
                .all(|value| crate::kvn::ser::OdmFloat::is_valid(*value))
            {
                Ok(())
            } else {
                Err(invalid_number())
            }
        };
        let check_text = |value: &str| -> Result<()> {
            if value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                Ok(())
            } else {
                Err(ValidationError::Generic {
                    message: Cow::Borrowed(
                        "ACM KVN free text must contain printable ASCII without line breaks",
                    ),
                    line: None,
                }
                .into())
            }
        };

        for value in self
            .header
            .comment
            .iter()
            .chain(self.body.segment.metadata.comment.iter())
        {
            check_text(value)?;
        }
        for value in [
            self.header.classification.as_ref(),
            Some(&self.header.originator),
            self.header.message_id.as_ref(),
            Some(&self.body.segment.metadata.object_name),
            self.body.segment.metadata.international_designator.as_ref(),
            self.body.segment.metadata.catalog_name.as_ref(),
            self.body.segment.metadata.object_designator.as_ref(),
            self.body.segment.metadata.originator_poc.as_ref(),
            self.body.segment.metadata.originator_position.as_ref(),
            self.body.segment.metadata.originator_phone.as_ref(),
            self.body.segment.metadata.originator_email.as_ref(),
            self.body.segment.metadata.originator_address.as_ref(),
            self.body.segment.metadata.odm_msg_link.as_ref(),
            self.body.segment.metadata.center_name.as_ref(),
            Some(&self.body.segment.metadata.time_system),
            self.body.segment.metadata.acm_data_elements.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            check_text(value)?;
        }
        if let Some(value) = &self.body.segment.metadata.taimutc_at_tzero {
            check_number(value.value)?;
        }
        if let Some(value) = &self.body.segment.metadata.next_leap_taimutc {
            check_number(value.value)?;
        }

        let data = &self.body.segment.data;
        for attitude in &data.att {
            for value in attitude.comment.iter().chain(
                [
                    attitude.att_id.as_ref(),
                    attitude.att_prev_id.as_ref(),
                    attitude.att_basis_id.as_ref(),
                    Some(&attitude.ref_frame_a),
                    Some(&attitude.ref_frame_b),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for line in &attitude.att_lines {
                check_values(&line.values)?;
            }
        }
        if let Some(physical) = &data.phys {
            for value in physical.comment.iter().chain(
                [
                    physical.cp_ref_frame.as_ref(),
                    physical.inertia_ref_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for value in [
                physical.drag_coeff,
                physical.wet_mass.as_ref().map(|value| value.value),
                physical.dry_mass.as_ref().map(|value| value.value),
                physical.ixx.as_ref().map(|value| value.value),
                physical.iyy.as_ref().map(|value| value.value),
                physical.izz.as_ref().map(|value| value.value),
                physical.ixy.as_ref().map(|value| value.value),
                physical.ixz.as_ref().map(|value| value.value),
                physical.iyz.as_ref().map(|value| value.value),
            ]
            .into_iter()
            .flatten()
            {
                check_number(value)?;
            }
            if let Some(value) = &physical.cp {
                check_values(&value.elements)?;
            }
        }
        for covariance in &data.cov {
            for value in covariance.comment.iter().chain(
                [
                    covariance.cov_id.as_ref(),
                    covariance.cov_prev_id.as_ref(),
                    covariance.cov_basis_id.as_ref(),
                    covariance.cov_ref_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for line in &covariance.cov_lines {
                check_values(&line.values)?;
            }
        }
        for maneuver in &data.man {
            for value in maneuver.comment.iter().chain(
                [
                    maneuver.man_id.as_ref(),
                    maneuver.man_prev_id.as_ref(),
                    maneuver.man_purpose.as_ref(),
                    maneuver.actuator_used.as_ref(),
                    maneuver.target_mom_frame.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            if let Some(value) = &maneuver.man_duration {
                check_number(value.value)?;
            }
            if let Some(value) = &maneuver.target_momentum {
                check_values(&value.elements)?;
            }
            if let Some(value) = &maneuver.target_attitude {
                check_values(&value.values)?;
            }
            if let Some(value) = &maneuver.target_spinrate {
                check_number(value.value)?;
            }
        }
        if let Some(determination) = &data.ad {
            for value in determination.comment.iter().chain(
                [
                    determination.ad_id.as_ref(),
                    determination.ad_prev_id.as_ref(),
                    determination.ad_method.as_ref(),
                    determination.attitude_source.as_ref(),
                    determination.ref_frame_a.as_ref(),
                    determination.ref_frame_b.as_ref(),
                ]
                .into_iter()
                .flatten(),
            ) {
                check_text(value)?;
            }
            for value in [
                determination.sigma_u.as_ref().map(|value| value.value),
                determination.sigma_v.as_ref().map(|value| value.value),
                determination
                    .rate_process_noise_stddev
                    .as_ref()
                    .map(|value| value.value),
            ]
            .into_iter()
            .flatten()
            {
                check_number(value)?;
            }
            for sensor in &determination.sensors {
                for value in sensor.comment.iter().chain(sensor.sensor_used.iter()) {
                    check_text(value)?;
                }
                if let Some(value) = &sensor.sensor_noise_stddev {
                    check_values(&value.values)?;
                }
                if let Some(value) = &sensor.sensor_frequency {
                    check_number(value.value)?;
                }
            }
        }
        if let Some(user) = &data.user {
            for value in user.comment.iter().chain(
                user.user_defined
                    .iter()
                    .flat_map(|parameter| [&parameter.parameter, &parameter.value]),
            ) {
                check_text(value)?;
            }
        }

        let mut sink = AcmKvnLexicalSink::default();
        let mut writer = KvnWriter::from_io(&mut sink);
        self.write_kvn(&mut writer);
        writer.finish_io()
    }
}

#[derive(Default)]
struct AcmKvnLexicalSink {
    line_len: usize,
}

impl Write for AcmKvnLexicalSink {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        for byte in buffer {
            if *byte == b'\n' {
                if self.line_len > 254 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "generated ACM KVN record exceeds 254 characters",
                    ));
                }
                self.line_len = 0;
            } else {
                if !(b' '..=b'~').contains(byte) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "generated ACM KVN contains non-printable or non-ASCII content",
                    ));
                }
                self.line_len += 1;
            }
        }
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;
    crate::xml::validate_element_sequences(
        xml,
        "ACM",
        |parent, child| {
            let children = acm_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable = matches!(
                child,
                b"COMMENT"
                    | b"att"
                    | b"cov"
                    | b"man"
                    | b"attLine"
                    | b"covLine"
                    | b"sensorData"
                    | b"USER_DEFINED"
            );
            // `userDefinedType` wraps its children in a repeating sequence, so a COMMENT may
            // open a new iteration after a USER_DEFINED.
            if parent == b"user" {
                return Some(XmlSequenceRule::restarting(rank, repeatable));
            }
            Some(XmlSequenceRule::new(rank, repeatable))
        },
        |element, attribute| {
            (attribute == b"parameter" && element == b"USER_DEFINED")
                || (attribute == b"units"
                    && matches!(
                        element,
                        b"TAIMUTC_AT_TZERO"
                            | b"NEXT_LEAP_TAIMUTC"
                            | b"WET_MASS"
                            | b"DRY_MASS"
                            | b"CP"
                            | b"IXX"
                            | b"IYY"
                            | b"IZZ"
                            | b"IXY"
                            | b"IXZ"
                            | b"IYZ"
                            | b"MAN_DURATION"
                            | b"TARGET_MOMENTUM"
                            | b"TARGET_SPINRATE"
                            | b"SIGMA_U"
                            | b"SIGMA_V"
                            | b"RATE_PROCESS_NOISE_STDDEV"
                            | b"SENSOR_NOISE_STDDEV"
                            | b"SENSOR_FREQUENCY"
                    ))
        },
    )
}

fn acm_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"acm" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CLASSIFICATION",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"segment"],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT_NAME",
            b"INTERNATIONAL_DESIGNATOR",
            b"CATALOG_NAME",
            b"OBJECT_DESIGNATOR",
            b"ORIGINATOR_POC",
            b"ORIGINATOR_POSITION",
            b"ORIGINATOR_PHONE",
            b"ORIGINATOR_EMAIL",
            b"ORIGINATOR_ADDRESS",
            b"ODM_MSG_LINK",
            b"CENTER_NAME",
            b"TIME_SYSTEM",
            b"EPOCH_TZERO",
            b"ACM_DATA_ELEMENTS",
            b"START_TIME",
            b"STOP_TIME",
            b"TAIMUTC_AT_TZERO",
            b"NEXT_LEAP_EPOCH",
            b"NEXT_LEAP_TAIMUTC",
        ],
        b"data" => &[b"att", b"phys", b"cov", b"man", b"ad", b"user"],
        b"att" => &[
            b"COMMENT",
            b"ATT_ID",
            b"ATT_PREV_ID",
            b"ATT_BASIS",
            b"ATT_BASIS_ID",
            b"REF_FRAME_A",
            b"REF_FRAME_B",
            b"NUMBER_STATES",
            b"ATT_TYPE",
            b"EULER_ROT_SEQ",
            b"RATE_TYPE",
            b"attLine",
        ],
        b"phys" => &[
            b"COMMENT",
            b"DRAG_COEFF",
            b"WET_MASS",
            b"DRY_MASS",
            b"CP_REF_FRAME",
            b"CP",
            b"INERTIA_REF_FRAME",
            b"IXX",
            b"IYY",
            b"IZZ",
            b"IXY",
            b"IXZ",
            b"IYZ",
        ],
        b"cov" => &[
            b"COMMENT",
            b"COV_ID",
            b"COV_PREV_ID",
            b"COV_BASIS",
            b"COV_BASIS_ID",
            b"COV_REF_FRAME",
            b"COV_TYPE",
            b"covLine",
        ],
        b"man" => &[
            b"COMMENT",
            b"MAN_ID",
            b"MAN_PREV_ID",
            b"MAN_PURPOSE",
            b"MAN_BEGIN_TIME",
            b"MAN_END_TIME",
            b"MAN_DURATION",
            b"ACTUATOR_USED",
            b"TARGET_MOMENTUM",
            b"TARGET_MOM_FRAME",
            b"TARGET_ATTITUDE",
            b"TARGET_SPINRATE",
        ],
        b"ad" => &[
            b"COMMENT",
            b"AD_ID",
            b"AD_PREV_ID",
            b"AD_METHOD",
            b"ATTITUDE_SOURCE",
            b"NUMBER_STATES",
            b"ATTITUDE_STATES",
            b"EULER_ROT_SEQ",
            b"COV_TYPE",
            b"REF_FRAME_A",
            b"REF_FRAME_B",
            b"RATE_STATES",
            b"SIGMA_U",
            b"SIGMA_V",
            b"RATE_PROCESS_NOISE_STDDEV",
            b"sensorData",
        ],
        b"sensorData" => &[
            b"SENSOR_NUMBER",
            b"SENSOR_USED",
            b"NUMBER_SENSOR_NOISE_COVARIANCE",
            b"SENSOR_NOISE_STDDEV",
            b"SENSOR_FREQUENCY",
        ],
        b"user" => &[b"COMMENT", b"USER_DEFINED"],
        _ => return None,
    })
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    const HEADER: &[&str] = &[
        "CLASSIFICATION",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
    ];
    const META: &[&str] = &[
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "CATALOG_NAME",
        "OBJECT_DESIGNATOR",
        "ORIGINATOR_POC",
        "ORIGINATOR_POSITION",
        "ORIGINATOR_PHONE",
        "ORIGINATOR_EMAIL",
        "ORIGINATOR_ADDRESS",
        "ODM_MSG_LINK",
        "CENTER_NAME",
        "TIME_SYSTEM",
        "EPOCH_TZERO",
        "ACM_DATA_ELEMENTS",
        "START_TIME",
        "STOP_TIME",
        "TAIMUTC_AT_TZERO",
        "NEXT_LEAP_EPOCH",
        "NEXT_LEAP_TAIMUTC",
    ];
    const ATT: &[&str] = &[
        "ATT_ID",
        "ATT_PREV_ID",
        "ATT_BASIS",
        "ATT_BASIS_ID",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "NUMBER_STATES",
        "ATT_TYPE",
        "EULER_ROT_SEQ",
        "RATE_TYPE",
    ];
    const PHYS: &[&str] = &[
        "DRAG_COEFF",
        "WET_MASS",
        "DRY_MASS",
        "CP_REF_FRAME",
        "CP",
        "INERTIA_REF_FRAME",
        "IXX",
        "IYY",
        "IZZ",
        "IXY",
        "IXZ",
        "IYZ",
    ];
    const COV: &[&str] = &[
        "COV_ID",
        "COV_PREV_ID",
        "COV_BASIS",
        "COV_BASIS_ID",
        "COV_REF_FRAME",
        "COV_TYPE",
    ];
    const MAN: &[&str] = &[
        "MAN_ID",
        "MAN_PREV_ID",
        "MAN_PURPOSE",
        "MAN_BEGIN_TIME",
        "MAN_END_TIME",
        "MAN_DURATION",
        "ACTUATOR_USED",
        "TARGET_MOMENTUM",
        "TARGET_MOM_FRAME",
        "TARGET_ATTITUDE",
        "TARGET_SPINRATE",
    ];
    const AD: &[&str] = &[
        "AD_ID",
        "AD_PREV_ID",
        "AD_METHOD",
        "ATTITUDE_SOURCE",
        "NUMBER_STATES",
        "ATTITUDE_STATES",
        "EULER_ROT_SEQ",
        "COV_TYPE",
        "REF_FRAME_A",
        "REF_FRAME_B",
        "RATE_STATES",
        "SIGMA_U",
        "SIGMA_V",
        "RATE_PROCESS_NOISE_STDDEV",
    ];
    const SENSOR: &[&str] = &[
        "SENSOR_NUMBER",
        "SENSOR_USED",
        "NUMBER_SENSOR_NOISE_COVARIANCE",
        "SENSOR_NOISE_STDDEV",
        "SENSOR_FREQUENCY",
    ];

    let invalid = |line: usize, offset: usize, message: String| {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message,
            contexts: vec!["while validating ACM KVN structure"],
            offset,
        }))))
    };
    let block_keys = |block: &str| -> Option<&[&str]> {
        Some(match block {
            "META" => META,
            "ATT" => ATT,
            "PHYS" => PHYS,
            "COV" => COV,
            "MAN" => MAN,
            "AD" => AD,
            "SENSOR" => SENSOR,
            "USER" => return None,
            _ => return None,
        })
    };
    let outer_rank = |block: &str| match block {
        "META" => Some(0usize),
        "ATT" => Some(1),
        "PHYS" => Some(2),
        "COV" => Some(3),
        "MAN" => Some(4),
        "AD" => Some(5),
        "USER" => Some(6),
        _ => None,
    };

    let mut block: Option<&str> = None;
    let mut previous_key = None;
    let mut top_rank = None;
    let mut last_outer_rank = None;
    let mut seen_nonrepeatable = 0u8;
    let mut block_has_content = false;
    let mut history_started = false;
    let mut ad_sensor_started = false;
    let mut offset = 0usize;

    for (index, raw_line) in kvn.split('\n').enumerate() {
        let number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let fail = |message: &str| Err(invalid(number, offset, message.into()));
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
            if block.is_none() && top_rank != Some(0) {
                return fail("ACM header COMMENT must immediately follow the version record");
            }
            if block_has_content && block.is_some() {
                return fail("COMMENT is not at the beginning of an ACM logical block");
            }
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_START").filter(|_| !line.contains('=')) {
            if marker == "SENSOR" {
                if block != Some("AD") || !ad_sensor_started && history_started {
                    return fail("SENSOR block must be nested directly in AD");
                }
                block = Some("SENSOR");
                previous_key = None;
                block_has_content = false;
                history_started = false;
                ad_sensor_started = true;
                offset += raw_line.len() + 1;
                continue;
            }
            if block.is_some() {
                return fail("unknown or nested ACM marked block");
            }
            let rank = outer_rank(marker)
                .ok_or_else(|| invalid(number, offset, "unknown ACM marked block".into()))?;
            if rank == 0 && last_outer_rank.is_some() {
                return fail("duplicate ACM metadata block");
            }
            if last_outer_rank.is_some_and(|previous| rank < previous) {
                return fail("out-of-order ACM marked block");
            }
            if !matches!(marker, "ATT" | "COV" | "MAN") {
                let bit = 1u8 << rank;
                if seen_nonrepeatable & bit != 0 {
                    return fail("duplicate non-repeatable ACM marked block");
                }
                seen_nonrepeatable |= bit;
            }
            if rank != 0 && last_outer_rank.is_none() {
                return fail("META must be the first ACM marked block");
            }
            last_outer_rank = Some(rank);
            block = Some(marker);
            previous_key = None;
            block_has_content = false;
            history_started = false;
            ad_sensor_started = false;
            offset += raw_line.len() + 1;
            continue;
        }
        if let Some(marker) = line.strip_suffix("_STOP").filter(|_| !line.contains('=')) {
            if marker == "SENSOR" {
                if block != Some("SENSOR") {
                    return fail("mismatched ACM SENSOR block end");
                }
                block = Some("AD");
                previous_key = None;
                block_has_content = true;
                history_started = false;
                offset += raw_line.len() + 1;
                continue;
            }
            if block != Some(marker) {
                return fail("mismatched ACM marked block end");
            }
            block = None;
            previous_key = None;
            block_has_content = false;
            history_started = false;
            offset += raw_line.len() + 1;
            continue;
        }

        match block {
            None => {
                if last_outer_rank.is_some() {
                    return fail("content outside an ACM marked block");
                }
                if !line.contains('=') {
                    return fail("expected one ACM header assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let rank = if key == "CCSDS_ACM_VERS" {
                    0
                } else {
                    HEADER
                        .iter()
                        .position(|candidate| *candidate == key)
                        .map(|rank| rank + 1)
                        .ok_or_else(|| {
                            invalid(number, offset, "unknown ACM header keyword".into())
                        })?
                };
                if top_rank.is_none() && rank != 0 {
                    return fail("CCSDS_ACM_VERS must be the first record");
                }
                if top_rank.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order ACM header keyword");
                }
                top_rank = Some(rank);
                block_has_content = true;
            }
            Some("ATT" | "COV") if !line.contains('=') => {
                history_started = true;
                block_has_content = true;
            }
            Some("USER") => {
                if !line.contains('=')
                    || !line
                        .split_once('=')
                        .unwrap()
                        .0
                        .trim()
                        .starts_with("USER_DEFINED_")
                {
                    return fail("invalid ACM user-defined assignment");
                }
                block_has_content = true;
            }
            Some(current) => {
                if history_started {
                    return fail("assignment after ACM history data");
                }
                if current == "AD" && ad_sensor_started {
                    return fail("AD assignment after SENSOR block");
                }
                if !line.contains('=') {
                    return fail("expected one ACM block assignment");
                }
                let key = line.split_once('=').unwrap().0.trim();
                let keys = block_keys(current)
                    .ok_or_else(|| invalid(number, offset, "unknown ACM marked block".into()))?;
                let rank = keys
                    .iter()
                    .position(|candidate| *candidate == key)
                    .ok_or_else(|| invalid(number, offset, "unknown ACM keyword".into()))?;
                if previous_key.is_some_and(|previous| rank <= previous) {
                    return fail("duplicate or out-of-order ACM keyword");
                }
                previous_key = Some(rank);
                block_has_content = true;
            }
        }
        offset += raw_line.len() + 1;
    }
    if block.is_some() {
        return Err(invalid(
            kvn.lines().count().max(1),
            kvn.len(),
            "unclosed ACM marked block".into(),
        ));
    }
    Ok(())
}

impl ToKvn for Acm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_ACM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AcmBody {
    #[serde(rename = "segment")]
    pub segment: Box<AcmSegment>,
}

impl crate::traits::Validate for AcmBody {
    fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self.segment.as_ref())
    }
}

impl ToKvn for AcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AcmSegment {
    pub metadata: AcmMetadata,
    pub data: AcmData,
}

impl crate::traits::Validate for AcmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate_with_metadata(&self.metadata)
    }
}

impl AcmSegment {
    pub fn validate(&self, _header: &AdmHeader) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

impl ToKvn for AcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

/// ACM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct AcmMetadata {
    /// Comments (allowed only at the beginning of the ACM Metadata). Each comment line shall begin
    /// with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the name of the object. There is no CCSDS-based restriction on
    /// the value for this keyword, but it is recommended to use names from either the UN Office of
    /// Outer Space Affairs designator index (reference `[2]`), which include Object name and
    /// international designator), the spacecraft operator, or a State Actor or commercial Space
    /// Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space catalog. If the
    /// object name is not known (uncorrelated object), ‘UNKNOWN’ may be used (or this keyword
    /// omitted).
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[builder(into)]
    pub object_name: String,
    /// Free text field containing an international designator for the object as assigned by the UN
    /// Committee on Space Research (COSPAR) and the US National Space Science Data Center (NSSDC).
    /// Such designator values have the following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year
    /// of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
    /// P{PP} = At least one capital letter for the identification of the part brought into space
    /// by the launch. In cases in which the object has no international designator, the value
    /// UNKNOWN may be used. NOTE – The international designator is typically specified by
    /// ‘OBJECT_ID’ in the APM and AEM.
    ///
    /// **Examples**: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub international_designator: Option<String>,
    /// Free text field containing the satellite catalog source or the source agency or operator
    /// abbreviated name (see annex B, subsection B1).
    ///
    /// **Examples**: CSPOC, RFSA, ESA, COMSPOC
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub catalog_name: Option<String>,
    /// Free text field specification of the unique satellite identification designator for the
    /// object, as reflected in the catalog whose name is ‘CATALOG_NAME’. If the ID is not known,
    /// ‘UNKNOWN’ may be used (or this keyword omitted).
    ///
    /// **Examples**: 22444, 18SPCS 18571, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub object_designator: Option<String>,
    /// Free text field containing Programmatic or Technical Point-of-Contact (POC) for ACM.
    ///
    /// **Examples**: Ms. Rodgers
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub originator_poc: Option<String>,
    /// Free text field containing contact position of the PoC.
    ///
    /// **Examples**: GNC Engineer, ACS Design Lead
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub originator_position: Option<String>,
    /// Free text field containing PoC phone number.
    ///
    /// **Examples**: +49615130312
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub originator_phone: Option<String>,
    /// Free-text field containing originator PoC email address.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub originator_email: Option<String>,
    /// Free text field containing Technical PoC information for ACM creator (suggest email,
    /// website, or physical address, etc.).
    ///
    /// **Examples**: JANE.DOE@SOMEWHERE.NET
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub originator_address: Option<String>,
    /// Free text field containing a unique identifier of Orbit Data Message(s) that are linked
    /// (relevant) to this Attitude Data Message.
    ///
    /// **Examples**: ODM_MSG_12345.txt, ORB_ID_0123
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub odm_msg_link: Option<String>,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH BARYCENTER, MOON
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Time system used for metadata, attitude data, covariance data. The set of allowed values is
    /// described in annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[builder(into)]
    pub time_system: String,
    /// Epoch from which all ACM relative times are referenced. (For format specification, see
    /// 6.8.9.) The time scale for EPOCH_TZERO is the one specified by ‘TIME_SYSTEM’ keyword in the
    /// Metadata section.
    ///
    /// **Examples**: 2016-11-10T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    pub epoch_tzero: CalendarEpoch,
    /// Comma-delimited list of elements of information data blocks included in this message. The
    /// order shall be the same as the order of the data blocks in the message. Values shall be
    /// confined to the following list: ATT, PHYS, COV, MAN, AD, USER. If the ACM contains multiple
    /// ATT, COV, MAN data blocks (as allowed by table 5-1), the corresponding ATT, COV, MAN entry
    /// shall be duplicated to match.
    ///
    /// **Examples**: ATT, AD, USER; ATT, ATT, PHYS; ATT, COV, AD
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub acm_data_elements: Option<String>,
    /// Time of the earliest data contained in the ACM, specified as either a relative or absolute
    /// time tag.
    ///
    /// **Examples**: 100.0, 2016-11-10T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub start_time: Option<Epoch>,
    /// Time of the latest data contained in the ACM, specified as either a relative or absolute
    /// time tag.
    ///
    /// **Examples**: 1500.0, 2016-11-11T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub stop_time: Option<Epoch>,
    /// Difference (TAI – UTC) in seconds (i.e., total # leap seconds elapsed since 1958) as modeled
    /// by the message originator at epoch ‘EPOCH_TZERO’.
    ///
    /// **Examples**: 36
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub taimutc_at_tzero: Option<TimeOffset>,
    /// Epoch of next leap second, specified as an absolute time tag.
    ///
    /// **Examples**: 2017-01-01T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub next_leap_epoch: Option<CalendarEpoch>,
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since 1958)
    /// incorporated by the message originator at epoch ‘NEXT_LEAP_EPOCH’. This keyword should be
    /// provided if NEXT_LEAP_EPOCH is supplied.
    ///
    /// **Examples**: 37
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub next_leap_taimutc: Option<TimeOffset>,
}

impl AcmMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        if self.epoch_tzero.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Metadata".into(),
                field: "EPOCH_TZERO".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl crate::traits::Validate for AcmMetadata {
    fn validate(&self) -> Result<()> {
        self.validate()
    }
}

impl ToKvn for AcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        if let Some(v) = &self.international_designator {
            writer.write_pair("INTERNATIONAL_DESIGNATOR", v);
        }
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
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
        if let Some(v) = &self.odm_msg_link {
            writer.write_pair("ODM_MSG_LINK", v);
        }
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.acm_data_elements {
            writer.write_pair("ACM_DATA_ELEMENTS", v);
        }
        if let Some(v) = self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        if let Some(v) = &self.taimutc_at_tzero {
            writer.write_odm_float_measure("TAIMUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.next_leap_epoch {
            writer.write_pair("NEXT_LEAP_EPOCH", v);
        }
        if let Some(v) = &self.next_leap_taimutc {
            writer.write_odm_float_measure("NEXT_LEAP_TAIMUTC", &v.to_unit_value());
        }
        writer.write_section("META_STOP");
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// ACM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct AcmData {
    /// One or more optional attitude state time histories (each consisting of one or more attitude
    /// states).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(rename = "att", default)]
    #[builder(default)]
    pub att: Vec<AcmAttitudeState>,
    /// A single space object physical characteristics section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(rename = "phys", default, skip_serializing_if = "Option::is_none")]
    pub phys: Option<AcmPhysicalDescription>,
    /// One or more optional covariance time histories (each consisting of one or more covariance
    /// matrix diagonals).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(rename = "cov", default)]
    #[builder(default)]
    pub cov: Vec<AcmCovarianceMatrix>,
    /// One or more optional maneuver specification section(s).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(rename = "man", default)]
    #[builder(default)]
    pub man: Vec<AcmManeuverParameters>,
    /// A single attitude determination Data section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(rename = "ad", default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<AcmAttitudeDetermination>,
    /// A single user-defined Data section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.10.
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserDefined>,
}

impl crate::traits::Validate for AcmData {
    fn validate(&self) -> Result<()> {
        for att in &self.att {
            att.validate()?;
        }
        if let Some(phys) = &self.phys {
            phys.validate()?;
        }
        for cov in &self.cov {
            cov.validate()?;
        }
        if let Some(ad) = &self.ad {
            ad.validate()?;
        }
        for man in &self.man {
            man.validate()?;
        }
        Ok(())
    }
}

impl AcmData {
    pub fn validate_with_metadata(&self, _metadata: &AcmMetadata) -> Result<()> {
        crate::traits::Validate::validate(self)
    }

    pub fn validate(&self, metadata: &AcmMetadata) -> Result<()> {
        self.validate_with_metadata(metadata)
    }
}

impl ToKvn for AcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for att in &self.att {
            att.write_kvn(writer);
        }
        if let Some(phys) = &self.phys {
            phys.write_kvn(writer);
        }
        for cov in &self.cov {
            cov.write_kvn(writer);
        }
        for man in &self.man {
            man.write_kvn(writer);
        }
        if let Some(ad) = &self.ad {
            ad.write_kvn(writer);
        }
        if let Some(user) = &self.user {
            writer.write_section("USER_START");
            writer.write_comments(&user.comment);
            for p in &user.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

//----------------------------------------------------------------------
// Attitude State Block (ATT)
//----------------------------------------------------------------------

/// ACM Data: Attitude State Time History Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeState {
    /// Comments allowed only immediately after the ATT_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string containing the identification number for this
    /// attitude state time history.
    ///
    /// **Examples**: ATT_20160402_XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub att_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous attitude time history block. NOTE: If the message is not part of a sequence of
    /// attitude time histories or if this attitude time history is the first in a sequence of
    /// attitude time histories, then ATT_PREV_ID should be excluded from this message.
    ///
    /// **Examples**: ATT_20160401_XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub att_prev_id: Option<String>,
    /// Basis of this attitude state time history data.
    ///
    /// **Examples**: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub att_basis: Option<AttBasisType>,
    /// Free-text field containing the identification number for the telemetry dataset, attitude
    /// determination, or simulation upon which this attitude state time history block is based.
    /// When a matching attitude determination block accompanies this attitude state time history,
    /// the ATT_BASIS_ID should match the corresponding AD_ID (see table 5-8).
    ///
    /// **Examples**: AD 1985
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub att_basis_id: Option<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: J2000
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Number of data states included. States to be included are attitude states and optional rate
    /// states.
    ///
    /// **Examples**: 3, 4, 7
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    pub number_states: u32,
    /// Type of attitude data, selected per annex B, subsection B4. Attitude data must always be
    /// listed before rate data. The units that shall be used are given in annex B, subsection B4.
    ///
    /// **Examples**: QUATERNION, EULER_ANGLES, DCM
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub att_type: AcmAttitudeType,
    /// Type of rate data, selected per annex B, subsection B4. If rate data are included,
    /// NUMBER_STATES must be at least 6 to include both attitude and rate data. The units that
    /// shall be used are given in annex B, subsection B4. If the value is ANGVEL, the reference
    /// frame used shall be REF_FRAME_B.
    ///
    /// **Examples**: ANGVEL, GYRO_BIAS, Q_DOT, NONE
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub rate_type: Option<AttRateType>,
    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation. This keyword is applicable only if ATT_TYPE specifies
    /// the use of Euler angles.
    ///
    /// **Examples**: ZXZ, XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub euler_rot_seq: Option<RotSeq>,
    /// Data lines that consist of attitude data followed by rate data. (For the data units, see
    /// above [ATT_TYPE and RATE_TYPE keywords]).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(rename = "attLine", default)]
    #[builder(default)]
    pub att_lines: Vec<AttLine>,
}

impl AcmAttitudeState {
    fn validate(&self) -> Result<()> {
        if self.ref_frame_a.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude State".into(),
                field: "REF_FRAME_A".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame_b.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude State".into(),
                field: "REF_FRAME_B".into(),
                line: None,
            }
            .into());
        }
        if self.number_states == 0 {
            return Err(ValidationError::OutOfRange {
                name: "NUMBER_STATES".into(),
                value: self.number_states.to_string(),
                expected: "positive integer".into(),
                line: None,
            }
            .into());
        }
        if self.att_lines.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude State".into(),
                field: "attLine".into(),
                line: None,
            }
            .into());
        }
        let expected = self.number_states as usize + 1;
        if let Some(line) = self
            .att_lines
            .iter()
            .find(|line| line.values.len() != expected)
        {
            return Err(ValidationError::InvalidValue {
                field: "attLine".into(),
                value: line.values.len().to_string(),
                expected: format!("relative time plus {} attitude states", self.number_states)
                    .into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmAttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("ATT_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.att_id {
            writer.write_pair("ATT_ID", v);
        }
        if let Some(v) = &self.att_prev_id {
            writer.write_pair("ATT_PREV_ID", v);
        }
        if let Some(v) = &self.att_basis {
            writer.write_pair("ATT_BASIS", v);
        }
        if let Some(v) = &self.att_basis_id {
            writer.write_pair("ATT_BASIS_ID", v);
        }
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("NUMBER_STATES", self.number_states);
        writer.write_pair("ATT_TYPE", &self.att_type);
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.rate_type {
            writer.write_pair("RATE_TYPE", v);
        }
        for line in &self.att_lines {
            writer.write_numeric_record(&line.values);
        }
        writer.write_section("ATT_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AttLine {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for AttLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Physical Description Block (PHYS)
//----------------------------------------------------------------------

/// ACM Data: Space Object Physical Characteristics Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmPhysicalDescription {
    /// Comments allowed only immediately after the PHYS_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Drag coefficient.
    ///
    /// **Examples**: 2
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub drag_coeff: Option<f64>,
    /// Space object total mass at the reference epoch ‘EPOCH_TZERO’.
    ///
    /// **Examples**: 750.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub wet_mass: Option<Mass>,
    /// Space object dry mass (without propellant).
    ///
    /// **Examples**: 500.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub dry_mass: Option<Mass>,
    /// Coordinate system for the center of pressure vector. The set of allowed values is described
    /// in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub cp_ref_frame: Option<String>,
    /// CP_REF_FRAME shall be present if CP is present. Vector location of spacecraft center of
    /// pressure for determining solar pressure torque, measured from the spacecraft center of
    /// mass. The coordinate frame is defined by CP_REF_FRAME. CP contains 3 elements, one for each
    /// axis represented in CP_REF_FRAME.
    ///
    /// **Examples**: 0.02 0.01 0.2
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cp: Option<Vector3>,
    /// Coordinate system for the inertia tensor. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub inertia_ref_frame: Option<String>,
    /// Moment of Inertia about the X axis of the spacecraft body frame defined by
    /// INERTIA_REF_FRAME.
    ///
    /// **Examples**: 1000.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ixx: Option<Moment>,
    /// Moment of Inertia about the Y axis.
    ///
    /// **Examples**: 800.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub iyy: Option<Moment>,
    /// Moment of Inertia about the Z axis.
    ///
    /// **Examples**: 400.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub izz: Option<Moment>,
    /// Inertia Cross Product of the X & Y axes.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ixy: Option<Moment>,
    /// Inertia Cross Product of the X & Z axes.
    ///
    /// **Examples**: 40.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ixz: Option<Moment>,
    /// Inertia Cross Product of the Y & Z axes.
    ///
    /// **Examples**: 60.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub iyz: Option<Moment>,
}

impl AcmPhysicalDescription {
    fn validate(&self) -> Result<()> {
        if self.cp_ref_frame.is_some() && self.cp.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Physical Description".into(),
                field: "CP".into(),
                line: None,
            }
            .into());
        }
        if self.cp.is_some() && self.cp_ref_frame.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Physical Description".into(),
                field: "CP_REF_FRAME".into(),
                line: None,
            }
            .into());
        }
        if let Some(cp) = &self.cp {
            if cp.elements.len() != 3 {
                return Err(ValidationError::InvalidValue {
                    field: "CP".into(),
                    value: cp.elements.len().to_string(),
                    expected: "exactly 3 vector elements".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

impl ToKvn for AcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = self.drag_coeff {
            writer.write_odm_float_pair("DRAG_COEFF", v);
        }
        if let Some(v) = &self.wet_mass {
            writer.write_odm_float_measure("WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.dry_mass {
            writer.write_odm_float_measure("DRY_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.cp_ref_frame {
            writer.write_pair("CP_REF_FRAME", v);
        }
        if let Some(v) = &self.cp {
            writer.write_numeric_vector("CP", &v.elements, v.units.as_ref());
        }
        if let Some(v) = &self.inertia_ref_frame {
            writer.write_pair("INERTIA_REF_FRAME", v);
        }
        if let Some(v) = &self.ixx {
            writer.write_odm_float_measure("IXX", v);
        }
        if let Some(v) = &self.iyy {
            writer.write_odm_float_measure("IYY", v);
        }
        if let Some(v) = &self.izz {
            writer.write_odm_float_measure("IZZ", v);
        }
        if let Some(v) = &self.ixy {
            writer.write_odm_float_measure("IXY", v);
        }
        if let Some(v) = &self.ixz {
            writer.write_odm_float_measure("IXZ", v);
        }
        if let Some(v) = &self.iyz {
            writer.write_odm_float_measure("IYZ", v);
        }
        writer.write_section("PHYS_STOP");
    }
}

//----------------------------------------------------------------------
// Covariance Block (COV)
//----------------------------------------------------------------------

/// ACM Data: Covariance Time History Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmCovarianceMatrix {
    /// Comments allowed only immediately after the COV_START keyword.
    ///
    /// **Examples**: THIS is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Basis of this covariance time history data.
    ///
    /// **Examples**: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cov_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cov_prev_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cov_basis: Option<AttBasisType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cov_basis_id: Option<String>,
    /// Reference frame of the covariance time history. The full set of values is enumerated in
    /// annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub cov_ref_frame: Option<String>,
    /// Indicates covariance composition. Select from annex B, subsection B6.
    ///
    /// **Examples**: ANGLE, ANGLE_GYROBIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    pub cov_type: AcmCovarianceLineType,
    /// Optional confidence level of the covariance matrix.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(skip)]
    pub cov_confidence: Option<f64>,
    /// Covariance data lines (diagonal terms only). (For the data units, see annex B, subsection
    /// B6.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(rename = "covLine", default)]
    pub cov_lines: Vec<CovLine>,
}

impl AcmCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if self.cov_confidence.is_some() {
            return Err(ValidationError::InvalidValue {
                field: "COV_CONFIDENCE".into(),
                value: "present".into(),
                expected: "omitted; ACM 2.0 does not define COV_CONFIDENCE".into(),
                line: None,
            }
            .into());
        }
        if self.cov_lines.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Covariance".into(),
                field: "covLine".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COV_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.cov_id {
            writer.write_pair("COV_ID", v);
        }
        if let Some(v) = &self.cov_prev_id {
            writer.write_pair("COV_PREV_ID", v);
        }
        if let Some(v) = &self.cov_basis {
            writer.write_pair("COV_BASIS", v);
        }
        if let Some(v) = &self.cov_basis_id {
            writer.write_pair("COV_BASIS_ID", v);
        }
        if let Some(v) = &self.cov_ref_frame {
            writer.write_pair("COV_REF_FRAME", v);
        }
        writer.write_pair("COV_TYPE", &self.cov_type);
        for line in &self.cov_lines {
            writer.write_numeric_record(&line.values);
        }
        writer.write_section("COV_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct CovLine {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for CovLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Maneuver Block (MAN)
//----------------------------------------------------------------------

/// ACM Data: Maneuver Specification Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmManeuverParameters {
    /// Comments allowed only immediately after the MAN_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string containing the identification number for this
    /// maneuver.
    ///
    /// **Examples**: DH2018172
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub man_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous maneuver block. If the message is not part of a sequence of maneuvers or if this
    /// maneuver is the first in a sequence of maneuvers, then MAN_PREV_ID should be excluded from
    /// this message.
    ///
    /// **Examples**: DH2018171
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub man_prev_id: Option<String>,
    /// The user may specify the intention(s) of the maneuver. Multiple maneuver purposes may be
    /// provided as a comma-delimited list. While there is no CCSDS-based restriction on the value
    /// for this keyword, it is suggested to use: Attitude adjust (ATT_ADJUST); Momentum
    /// desaturation (MOM_DESAT); Pointing Request Message (PRM_ID_xxxx); Science objective
    /// (SCI_OBJ); Spin rate adjust (SPIN_RATE_ADJUST).
    ///
    /// **Examples**: ATT_ADJUST
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub man_purpose: Option<String>,
    /// Start time of actual maneuver, measured as a relative time with respect to EPOCH_TZERO.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub man_begin_time: Option<RelativeTime>,
    /// End time of actual maneuver, measured as a relative time with respect to EPOCH_TZERO.
    ///
    /// **Examples**: 120.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub man_end_time: Option<RelativeTime>,
    /// Maneuver duration.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub man_duration: Option<Duration>,
    /// Actuator used for the maneuver.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub actuator_used: Option<String>,
    /// Target angular momentum vector.
    ///
    /// **Units**: N*m*s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub target_momentum: Option<TargetMomentum>,
    /// Coordinate system for the target momentum vector.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub target_mom_frame: Option<String>,
    /// Target attitude (e.g., quaternion).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub target_attitude: Option<Vec4Double>,
    /// Target spin rate.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub target_spinrate: Option<AngleRate>,
}

impl AcmManeuverParameters {
    fn validate(&self) -> Result<()> {
        if self.man_purpose.as_deref().unwrap_or("").trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "MAN_PURPOSE".into(),
                line: None,
            }
            .into());
        }
        if self.man_begin_time.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "MAN_BEGIN_TIME".into(),
                line: None,
            }
            .into());
        }
        if self.man_end_time.is_some() && self.man_duration.is_some() {
            return Err(ValidationError::Conflict {
                fields: vec!["MAN_END_TIME".into(), "MAN_DURATION".into()],
                line: None,
            }
            .into());
        }
        if self.man_end_time.is_none() && self.man_duration.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "MAN_END_TIME or MAN_DURATION".into(),
                line: None,
            }
            .into());
        }
        if self.target_momentum.is_some() && self.target_mom_frame.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "TARGET_MOM_FRAME".into(),
                line: None,
            }
            .into());
        }
        if self.target_momentum.is_none() && self.target_mom_frame.is_some() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "TARGET_MOMENTUM".into(),
                line: None,
            }
            .into());
        }
        if let Some(momentum) = &self.target_momentum {
            if momentum.elements.len() != 3 {
                return Err(ValidationError::InvalidValue {
                    field: "TARGET_MOMENTUM".into(),
                    value: momentum.elements.len().to_string(),
                    expected: "exactly 3 vector elements".into(),
                    line: None,
                }
                .into());
            }
        }
        if let Some(att) = &self.target_attitude {
            if att.values.len() != 4 {
                return Err(ValidationError::InvalidValue {
                    field: "TARGET_ATTITUDE".into(),
                    value: format!("{:?}", att.values),
                    expected: "4 values".into(),
                    line: None,
                }
                .into());
            }
        }
        let has_target_momentum = self.target_momentum.is_some();
        let has_target_attitude = self.target_attitude.is_some();
        let has_target_spinrate = self.target_spinrate.is_some();
        let choice_count = usize::from(has_target_momentum)
            + usize::from(has_target_attitude)
            + usize::from(has_target_spinrate);
        if choice_count > 1 {
            return Err(ValidationError::Conflict {
                fields: vec![
                    "TARGET_MOMENTUM".into(),
                    "TARGET_ATTITUDE".into(),
                    "TARGET_SPINRATE".into(),
                ],
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.man_id {
            writer.write_pair("MAN_ID", v);
        }
        if let Some(v) = &self.man_prev_id {
            writer.write_pair("MAN_PREV_ID", v);
        }
        if let Some(v) = &self.man_purpose {
            writer.write_pair("MAN_PURPOSE", v);
        }
        if let Some(v) = &self.man_begin_time {
            writer.write_pair("MAN_BEGIN_TIME", v);
        }
        if let Some(v) = &self.man_end_time {
            writer.write_pair("MAN_END_TIME", v);
        }
        if let Some(v) = &self.man_duration {
            writer.write_odm_float_measure("MAN_DURATION", &v.to_unit_value());
        }
        if let Some(v) = &self.actuator_used {
            writer.write_pair("ACTUATOR_USED", v);
        }
        if let Some(v) = &self.target_momentum {
            writer.write_numeric_vector("TARGET_MOMENTUM", &v.elements, v.units.as_ref());
        }
        if let Some(v) = &self.target_mom_frame {
            writer.write_pair("TARGET_MOM_FRAME", v);
        }
        if let Some(v) = &self.target_attitude {
            writer.write_numeric_vector("TARGET_ATTITUDE", &v.values, None::<&&str>);
        }
        if let Some(v) = &self.target_spinrate {
            writer.write_odm_float_measure("TARGET_SPINRATE", v);
        }
        writer.write_section("MAN_STOP");
    }
}

//----------------------------------------------------------------------
// Attitude Determination Block (AD)
//----------------------------------------------------------------------

/// ACM Data: Attitude Determination Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeDetermination {
    /// Comments allowed only immediately after the AD_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string for this attitude determination.
    ///
    /// **Examples**: AD_20190101
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub ad_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous attitude determination block. NOTE: If the message is not part of a sequence of
    /// attitude determination blocks or if this attitude determination block is the first in a
    /// sequence of attitude determination blocks, then AD_PREV_ID should be excluded from this
    /// message.
    ///
    /// **Examples**: AD_20190100
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub ad_prev_id: Option<String>,
    /// Type of attitude determination method used. (For further description, see annex B,
    /// subsection B5.)
    ///
    /// **Examples**: EKF, TRIAD, BATCH
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub ad_method: Option<String>,
    /// Source of attitude estimate, whether from a ground based estimator or onboard estimator.
    ///
    /// **Examples**: GND, OBC
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub attitude_source: Option<String>,
    /// Number of states if EKF, BATCH, or FILTER SMOOTHER is specified.
    ///
    /// **Examples**: 3, 6, 7
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub number_states: Option<u32>,
    /// Type of attitude states if EKF, BATCH, or FILTER SMOOTHER is specified.
    ///
    /// **Examples**: QUATERNION
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub attitude_states: Option<AcmAttitudeType>,
    /// Euler rotation sequence when the estimator attitude states use Euler angles.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub euler_rot_seq: Option<RotSeq>,
    /// Indicates covariance composition. Select from annex B, subsection B6.
    ///
    /// **Examples**: ANGLE, ANGLE_GYROBIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub cov_type: Option<AcmCovarianceLineType>,
    /// Epoch of the attitude determination.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(skip)]
    pub ad_epoch: Option<Epoch>,
    /// Name of the reference frame that defines the starting point of the transformation described
    /// by the attitude state in the estimator. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: J2000
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_a: Option<String>,
    /// Name of the reference frame that defines the ending point of the transformation described
    /// by the attitude state in the estimator. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_b: Option<String>,
    /// Type of attitude data, selected per annex B, subsection B4. Attitude states must always be
    /// listed before rate states.
    ///
    /// **Examples**: QUATERNION
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(skip)]
    pub attitude_type: Option<String>,
    /// Type of rate state included in the estimator. If rate states are included, attitude_states
    /// must be at least 6 to include both attitude states and rate states.
    ///
    /// **Examples**: ANGVEL, GYRO_BIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub rate_states: Option<AttRateType>,
    /// Rate random walk if RATE_STATES=GYRO_BIAS.
    ///
    /// **Examples**: 3.7e-7
    ///
    /// **Units**: deg/s^1.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sigma_u: Option<SigmaU>,
    /// Angle random walk if RATE_STATES=GYRO_BIAS.
    ///
    /// **Examples**: 1.3e-5
    ///
    /// **Units**: deg/s^0.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sigma_v: Option<SigmaV>,
    /// Process noise standard deviation if RATE_STATES=ANG_VEL.
    ///
    /// **Examples**: 5.1E-06
    ///
    /// **Units**: deg/s^1.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub rate_process_noise_stddev: Option<SigmaU>,
    /// Sensor data blocks.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(rename = "sensorData", default)]
    pub sensors: Vec<AcmSensor>,
}

impl AcmAttitudeDetermination {
    fn validate(&self) -> Result<()> {
        if self.ad_epoch.is_some() {
            return Err(ValidationError::InvalidValue {
                field: "AD_EPOCH".into(),
                value: "present".into(),
                expected: "omitted; ACM 2.0 does not define AD_EPOCH".into(),
                line: None,
            }
            .into());
        }
        if self.attitude_type.is_some() {
            return Err(ValidationError::InvalidValue {
                field: "ATTITUDE_TYPE".into(),
                value: "present".into(),
                expected: "omitted; use ATTITUDE_STATES in ACM 2.0".into(),
                line: None,
            }
            .into());
        }
        if self.attitude_states.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude Determination".into(),
                field: "ATTITUDE_STATES".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame_a.as_deref().unwrap_or("").trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude Determination".into(),
                field: "REF_FRAME_A".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame_b.as_deref().unwrap_or("").trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Attitude Determination".into(),
                field: "REF_FRAME_B".into(),
                line: None,
            }
            .into());
        }
        if self.number_states == Some(0) {
            return Err(ValidationError::OutOfRange {
                name: "NUMBER_STATES".into(),
                value: "0".into(),
                expected: "positive integer".into(),
                line: None,
            }
            .into());
        }
        if self.sensors.is_empty() {
            return Ok(());
        }

        for sensor in &self.sensors {
            if sensor.sensor_number == Some(0) {
                return Err(ValidationError::OutOfRange {
                    name: "SENSOR_NUMBER".into(),
                    value: "0".into(),
                    expected: "positive integer".into(),
                    line: None,
                }
                .into());
            }
            if sensor.number_sensor_noise_covariance == Some(0) {
                return Err(ValidationError::OutOfRange {
                    name: "NUMBER_SENSOR_NOISE_COVARIANCE".into(),
                    value: "0".into(),
                    expected: "positive integer".into(),
                    line: None,
                }
                .into());
            }
            if let Some(frequency) = &sensor.sensor_frequency {
                if !frequency.value.is_finite() || frequency.value <= 0.0 {
                    return Err(ValidationError::OutOfRange {
                        name: "SENSOR_FREQUENCY".into(),
                        value: frequency.value.to_string(),
                        expected: "finite value > 0".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }

        let mut numbers: Vec<u32> = self
            .sensors
            .iter()
            .filter_map(|s| s.sensor_number)
            .collect();
        numbers.sort_unstable();

        for window in numbers.windows(2) {
            if let [prev, next] = *window {
                if next == prev {
                    return Err(ValidationError::InvalidValue {
                        field: "SENSOR_NUMBER".into(),
                        value: format!("{:?}", numbers),
                        expected: "unique values".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }

        Ok(())
    }
}

impl ToKvn for AcmAttitudeDetermination {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("AD_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.ad_id {
            writer.write_pair("AD_ID", v);
        }
        if let Some(v) = &self.ad_prev_id {
            writer.write_pair("AD_PREV_ID", v);
        }
        if let Some(v) = &self.ad_method {
            writer.write_pair("AD_METHOD", v);
        }
        if let Some(v) = &self.attitude_source {
            writer.write_pair("ATTITUDE_SOURCE", v);
        }
        if let Some(v) = &self.number_states {
            writer.write_pair("NUMBER_STATES", v);
        }
        if let Some(v) = &self.attitude_states {
            writer.write_pair("ATTITUDE_STATES", v);
        }
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        if let Some(v) = &self.cov_type {
            writer.write_pair("COV_TYPE", v);
        }
        if let Some(v) = &self.ref_frame_a {
            writer.write_pair("REF_FRAME_A", v);
        }
        if let Some(v) = &self.ref_frame_b {
            writer.write_pair("REF_FRAME_B", v);
        }
        if let Some(v) = &self.rate_states {
            writer.write_pair("RATE_STATES", v);
        }
        if let Some(v) = &self.sigma_u {
            writer.write_odm_float_measure("SIGMA_U", v);
        }
        if let Some(v) = &self.sigma_v {
            writer.write_odm_float_measure("SIGMA_V", v);
        }
        if let Some(v) = &self.rate_process_noise_stddev {
            writer.write_odm_float_measure("RATE_PROCESS_NOISE_STDDEV", v);
        }
        for sensor in &self.sensors {
            sensor.write_kvn(writer);
        }
        writer.write_section("AD_STOP");
    }
}

/// ACM Data: Sensor Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmSensor {
    /// Comments allowed only immediately after the SENSOR_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Sensor number. Multiple sensors may be included, with each having a unique, ascending
    /// number.
    ///
    /// **Examples**: 1, 2, 3
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sensor_number: Option<u32>,
    /// Type of sensor used in estimation.
    ///
    /// **Examples**: AST, DSS, GYRO
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sensor_used: Option<String>,
    /// Number of elements in the sensor-noise covariance representation.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub number_sensor_noise_covariance: Option<u32>,
    /// Standard deviation of sensor noise.
    ///
    /// **Examples**: 0.0097 0.0097
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sensor_noise_stddev: Option<SensorNoise>,
    /// Frequency of sensor data.
    ///
    /// **Examples**: 5
    ///
    /// **Units**: Hz
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub sensor_frequency: Option<Frequency>,
}

impl ToKvn for AcmSensor {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("SENSOR_START");
        writer.write_comments(&self.comment);
        if let Some(v) = self.sensor_number {
            writer.write_pair("SENSOR_NUMBER", v);
        }
        if let Some(v) = &self.sensor_used {
            writer.write_pair("SENSOR_USED", v);
        }
        if let Some(v) = self.number_sensor_noise_covariance {
            writer.write_pair("NUMBER_SENSOR_NOISE_COVARIANCE", v);
        }
        if let Some(v) = &self.sensor_noise_stddev {
            writer.write_numeric_vector("SENSOR_NOISE_STDDEV", &v.values, v.units.as_ref());
        }
        if let Some(v) = &self.sensor_frequency {
            writer.write_odm_float_measure("SENSOR_FREQUENCY", &v.to_unit_value());
        }
        writer.write_section("SENSOR_STOP");
    }
}

//----------------------------------------------------------------------
// KVN Parsing
//----------------------------------------------------------------------

impl ParseKvn for Acm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        crate::kvn::acm::parse_acm(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_acm_kvn() -> String {
        r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0.5 0.5 0.5 0.5
ATT_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_acm_success() {
        let kvn = sample_acm_kvn();
        let acm = Acm::from_kvn(&kvn).expect("ACM parse failed");

        assert_eq!(acm.version, "2.0");
        assert_eq!(
            acm.body.segment.metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
        assert_eq!(acm.body.segment.data.att.len(), 1);
        assert_eq!(acm.body.segment.data.att[0].att_lines.len(), 1);
    }

    #[test]
    fn test_acm_multiple_att_blocks() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = INSTRUMENT
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
"#;
        let acm = Acm::from_kvn(kvn).unwrap();
        assert_eq!(acm.body.segment.data.att.len(), 2);
    }

    #[test]
    fn test_acm_missing_mandatory_metadata() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
NUMBER_STATES = 4
ATT_TYPE = QUATERNION
0.0 0 0 0 1
ATT_STOP
"#;
        // Missing OBJECT_NAME
        assert!(Acm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_acm_physical_block() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
WET_MASS = 1000 [kg]
DRY_MASS = 500 [kg]
PHYS_STOP
"#;
        let acm = Acm::from_kvn(kvn).unwrap();
        let phys = acm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1000.0);
    }

    #[test]
    fn test_acm_sensor_xml_roundtrip() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
AD_START
ATTITUDE_STATES = QUATERNION
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY_1
SENSOR_START
SENSOR_NUMBER = 1
SENSOR_USED = AST
SENSOR_STOP
AD_STOP
"#;

        let acm = Acm::from_kvn(kvn).expect("failed to parse ACM KVN");
        let xml = acm.to_xml().expect("failed to serialize ACM XML");
        let parsed = Acm::from_xml(&xml).expect("failed to parse ACM XML");
        assert_eq!(
            parsed.body.segment.data.ad.as_ref().unwrap().sensors.len(),
            1
        );
        assert_eq!(
            parsed.body.segment.data.ad.as_ref().unwrap().sensors[0].sensor_number,
            Some(1)
        );
    }
}
