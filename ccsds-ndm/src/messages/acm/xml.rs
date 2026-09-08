// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::Acm;
use crate::error::{Result, ValidationError};

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
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
}
