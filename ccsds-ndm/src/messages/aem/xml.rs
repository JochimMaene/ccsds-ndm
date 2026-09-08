// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::{Aem, AemAttitudeState, AemData};
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub(super) fn validate_aem_xml_envelope(
    xml: &str,
    options: &crate::options::ParseOptions,
    source_edition: &mut Option<String>,
) -> Result<()> {
    use crate::xml::XmlSequenceRule;
    crate::xml::validate_standalone_document(
        xml,
        b"aem",
        "AEM",
        options,
        source_edition,
        crate::xml::MessageSchema {
            child_rule: |parent: &[u8], child: &[u8]| {
                let children = aem_xml_children(parent)?;
                let rank = children.iter().position(|candidate| *candidate == child)? as u16;
                let repeatable = matches!(child, b"COMMENT" | b"segment" | b"attitudeState");
                Some(XmlSequenceRule::new(rank, repeatable))
            },
            attribute_allowed: |element: &[u8], attribute: &[u8]| {
                attribute == b"units"
                    && matches!(
                        element,
                        b"Q1_DOT"
                            | b"Q2_DOT"
                            | b"Q3_DOT"
                            | b"QC_DOT"
                            | b"ANGLE_1"
                            | b"ANGLE_2"
                            | b"ANGLE_3"
                            | b"ANGLE_1_DOT"
                            | b"ANGLE_2_DOT"
                            | b"ANGLE_3_DOT"
                            | b"ANGVEL_X"
                            | b"ANGVEL_Y"
                            | b"ANGVEL_Z"
                            | b"SPIN_ALPHA"
                            | b"SPIN_DELTA"
                            | b"SPIN_ANGLE"
                            | b"SPIN_ANGLE_VEL"
                            | b"NUTATION"
                            | b"NUTATION_PER"
                            | b"NUTATION_PHASE"
                            | b"MOMENTUM_ALPHA"
                            | b"MOMENTUM_DELTA"
                            | b"NUTATION_VEL"
                    )
            },
            is_record: |element: &[u8]| element == b"attitudeState",
        },
    )
}

fn aem_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"aem" => &[b"header", b"body"],
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
            b"OBJECT_ID",
            b"CENTER_NAME",
            b"REF_FRAME_A",
            b"REF_FRAME_B",
            b"TIME_SYSTEM",
            b"START_TIME",
            b"USEABLE_START_TIME",
            b"USEABLE_STOP_TIME",
            b"STOP_TIME",
            b"ATTITUDE_TYPE",
            b"EULER_ROT_SEQ",
            b"ANGVEL_FRAME",
            b"INTERPOLATION_METHOD",
            b"INTERPOLATION_DEGREE",
        ],
        b"data" => &[b"COMMENT", b"attitudeState"],
        b"attitudeState" => &[
            b"quaternionEphemeris",
            b"quaternionDerivative",
            b"quaternionAngVel",
            b"eulerAngle",
            b"eulerAngleDerivative",
            b"eulerAngleAngVel",
            b"spin",
            b"spinNutation",
            b"spinNutationMom",
        ],
        b"quaternionEphemeris" => &[b"EPOCH", b"quaternion"],
        b"quaternionDerivative" => &[b"EPOCH", b"quaternion", b"quaternionDot"],
        b"quaternionAngVel" => &[b"EPOCH", b"quaternion", b"angVel"],
        b"quaternion" => &[b"Q1", b"Q2", b"Q3", b"QC"],
        b"quaternionDot" => &[b"Q1_DOT", b"Q2_DOT", b"Q3_DOT", b"QC_DOT"],
        b"angVel" => &[b"ANGVEL_X", b"ANGVEL_Y", b"ANGVEL_Z"],
        b"eulerAngle" => &[b"EPOCH", b"ANGLE_1", b"ANGLE_2", b"ANGLE_3"],
        b"eulerAngleDerivative" => &[
            b"EPOCH",
            b"ANGLE_1",
            b"ANGLE_2",
            b"ANGLE_3",
            b"ANGLE_1_DOT",
            b"ANGLE_2_DOT",
            b"ANGLE_3_DOT",
        ],
        b"eulerAngleAngVel" => &[
            b"EPOCH",
            b"ANGLE_1",
            b"ANGLE_2",
            b"ANGLE_3",
            b"ANGVEL_X",
            b"ANGVEL_Y",
            b"ANGVEL_Z",
        ],
        b"spin" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
        ],
        b"spinNutation" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
            b"NUTATION",
            b"NUTATION_PER",
            b"NUTATION_PHASE",
        ],
        b"spinNutationMom" => &[
            b"EPOCH",
            b"SPIN_ALPHA",
            b"SPIN_DELTA",
            b"SPIN_ANGLE",
            b"SPIN_ANGLE_VEL",
            b"MOMENTUM_ALPHA",
            b"MOMENTUM_DELTA",
            b"NUTATION_VEL",
        ],
        _ => return None,
    })
}

impl Aem {
    pub(crate) fn from_xml_with_options(
        xml: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        let mut source_edition = None;
        validate_aem_xml_envelope(xml, options, &mut source_edition)?;
        let aem: Self = crate::xml::from_str_with_context(xml, "AEM")?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
struct AemDataXml {
    #[serde(default)]
    comment: Vec<String>,
    #[serde(rename = "attitudeState")]
    attitude_states: Vec<AemAttitudeStateXml>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AemAttitudeStateXml {
    #[serde(rename = "quaternionEphemeris", default)]
    quaternion_ephemeris: Option<crate::common::QuaternionEphemeris>,
    #[serde(rename = "quaternionDerivative", default)]
    quaternion_derivative: Option<crate::common::QuaternionDerivative>,
    #[serde(rename = "quaternionAngVel", default)]
    quaternion_ang_vel: Option<crate::common::QuaternionAngVel>,
    #[serde(rename = "eulerAngle", default)]
    euler_angle: Option<crate::common::EulerAngle>,
    #[serde(rename = "eulerAngleDerivative", default)]
    euler_angle_derivative: Option<crate::common::EulerAngleDerivative>,
    #[serde(rename = "eulerAngleAngVel", default)]
    euler_angle_ang_vel: Option<crate::common::EulerAngleAngVel>,
    #[serde(rename = "spin", default)]
    spin: Option<crate::common::Spin>,
    #[serde(rename = "spinNutation", default)]
    spin_nutation: Option<crate::common::SpinNutation>,
    #[serde(rename = "spinNutationMom", default)]
    spin_nutation_mom: Option<crate::common::SpinNutationMom>,
}

impl AemAttitudeStateXml {
    fn into_state(self) -> std::result::Result<AemAttitudeState, &'static str> {
        let count = [
            self.quaternion_ephemeris.is_some(),
            self.quaternion_derivative.is_some(),
            self.quaternion_ang_vel.is_some(),
            self.euler_angle.is_some(),
            self.euler_angle_derivative.is_some(),
            self.euler_angle_ang_vel.is_some(),
            self.spin.is_some(),
            self.spin_nutation.is_some(),
            self.spin_nutation_mom.is_some(),
        ]
        .into_iter()
        .filter(|present| *present)
        .count();
        if count != 1 {
            return Err("attitudeState requires exactly one attitude choice");
        }
        Ok(if let Some(value) = self.quaternion_ephemeris {
            AemAttitudeState::QuaternionEphemeris(value)
        } else if let Some(value) = self.quaternion_derivative {
            AemAttitudeState::QuaternionDerivative(value)
        } else if let Some(value) = self.quaternion_ang_vel {
            AemAttitudeState::QuaternionAngVel(value)
        } else if let Some(value) = self.euler_angle {
            AemAttitudeState::EulerAngle(value)
        } else if let Some(value) = self.euler_angle_derivative {
            AemAttitudeState::EulerAngleDerivative(value)
        } else if let Some(value) = self.euler_angle_ang_vel {
            AemAttitudeState::EulerAngleAngVel(value)
        } else if let Some(value) = self.spin {
            AemAttitudeState::Spin(value)
        } else if let Some(value) = self.spin_nutation {
            AemAttitudeState::SpinNutation(value)
        } else {
            AemAttitudeState::SpinNutationMom(self.spin_nutation_mom.unwrap())
        })
    }
}

impl<'de> Deserialize<'de> for AemData {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        use serde::de::Error;
        let value = AemDataXml::deserialize(deserializer)?;
        Ok(Self {
            comment: value.comment,
            attitude_states: value
                .attitude_states
                .into_iter()
                .map(|state| state.into_state().map_err(D::Error::custom))
                .collect::<std::result::Result<_, _>>()?,
        })
    }
}

impl Serialize for AemData {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AemData", 2)?;
        if !self.comment.is_empty() {
            state.serialize_field("COMMENT", &self.comment)?;
        }
        state.serialize_field(
            "attitudeState",
            &AemAttitudeStatesXml(&self.attitude_states),
        )?;
        state.end()
    }
}

impl Serialize for AemAttitudeStatesXml<'_> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.iter().map(AemAttitudeStateXmlRef))
    }
}

impl Serialize for AemAttitudeStateXmlRef<'_> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("attitudeState", 1)?;
        match self.0 {
            AemAttitudeState::QuaternionEphemeris(value) => {
                state.serialize_field("quaternionEphemeris", value)?
            }
            AemAttitudeState::QuaternionDerivative(value) => {
                state.serialize_field("quaternionDerivative", value)?
            }
            AemAttitudeState::QuaternionAngVel(value) => {
                state.serialize_field("quaternionAngVel", value)?
            }
            AemAttitudeState::EulerAngle(value) => state.serialize_field("eulerAngle", value)?,
            AemAttitudeState::EulerAngleDerivative(value) => {
                state.serialize_field("eulerAngleDerivative", value)?
            }
            AemAttitudeState::EulerAngleAngVel(value) => {
                state.serialize_field("eulerAngleAngVel", value)?
            }
            AemAttitudeState::Spin(value) => state.serialize_field("spin", value)?,
            AemAttitudeState::SpinNutation(value) => {
                state.serialize_field("spinNutation", value)?
            }
            AemAttitudeState::SpinNutationMom(value) => {
                state.serialize_field("spinNutationMom", value)?
            }
        }
        state.end()
    }
}

struct AemAttitudeStatesXml<'a>(&'a [AemAttitudeState]);

struct AemAttitudeStateXmlRef<'a>(&'a AemAttitudeState);
