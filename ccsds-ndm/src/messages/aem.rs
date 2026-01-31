// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//----------------------------------------------------------------------
// Root AEM Structure
//----------------------------------------------------------------------

/// Attitude Ephemeris Message (AEM).
///
/// An AEM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The AEM is suited to interagency exchanges that involve automated
/// interaction and require higher fidelity or higher precision dynamic modeling than is
/// possible with the APM.
///
/// The AEM allows for dynamic modeling of any number of torques (solar pressure, atmospheric
/// torques, magnetics, etc.). It requires the use of an interpolation technique to interpret
/// the attitude state at times different from the tabular epochs.
///
/// **CCSDS Reference**: 504.0-B-2, Section 4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "aem")]
pub struct Aem {
    pub header: AdmHeader,
    pub body: AemBody,
    #[serde(rename = "@id")]
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
}

impl crate::traits::Validate for Aem {
    fn validate(&self) -> Result<()> {
        Aem::validate(self)
    }
}

impl Ndm for Aem {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let aem = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Aem, &aem)?;
        Ok(aem)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let aem: Self = crate::xml::from_str_with_context(xml, "AEM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Aem, &aem)?;
        Ok(aem)
    }
}

impl Aem {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.validate()
    }
}

impl AemBody {
    pub fn validate(&self) -> Result<()> {
        for segment in &self.segment {
            segment.validate()?;
        }
        Ok(())
    }
}

impl AemSegment {
    pub fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate(&self.metadata.attitude_type)?;
        Ok(())
    }
}

impl ToKvn for Aem {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_AEM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AemBody {
    #[serde(rename = "segment")]
    pub segment: Vec<AemSegment>,
}

impl ToKvn for AemBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for seg in &self.segment {
            seg.write_kvn(writer);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AemSegment {
    pub metadata: AemMetadata,
    pub data: AemData,
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

/// AEM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AemMetadata {
    /// Comments allowed only at the beginning of the Metadata section. Each comment line shall
    /// begin with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include Object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// capital letter for the identification of the part brought into space by the launch. In
    /// cases in which the asset is not listed in reference [ADM-2], the UN Office of Outer Space
    /// Affairs designator index format is not used, or the content cannot be disclosed, the value
    /// should be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH, STS-106
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: ICRF, SC_BODY_1, INSTRUMENT_A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1, INSTRUMENT_A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Time system used for both attitude ephemeris data and metadata. The set of allowed values
    /// is described in annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub time_system: String,
    /// Start of TOTAL time span covered by attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    pub start_time: Epoch,
    /// Optional start of USEABLE time span covered by attitude ephemeris data immediately
    /// following this metadata block. To allow for proper interpolation near the beginning/end of
    /// the attitude ephemeris data block, it may be necessary to utilize this keyword with values
    /// within the time span covered by the attitude ephemeris data records as denoted by the
    /// START/STOP_TIME time tags. The USEABLE_START_TIME time tag of a new block of ephemeris data
    /// must be greater than or equal to the USEABLE_STOP_TIME time tag of the previous block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    /// Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
    /// this metadata block. (See also USEABLE_START_TIME.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    /// End of TOTAL time span covered by the attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    pub stop_time: Epoch,
    /// The type of information contained in the data lines. This keyword must have a value from the
    /// set specified at the right. (See table 4-4 for details of the data contained in each line.)
    ///
    /// **Examples**: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
    /// EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub attitude_type: String,
    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation. This keyword is applicable only if ATTITUDE_TYPE
    /// specifies the use of Euler angles.
    ///
    /// **Examples**: ZXZ, XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub euler_rot_seq: Option<RotSeq>,
    /// The frame of reference in which angular velocity data are specified. The set of allowed
    /// values is described in annex B, subsection B3. This keyword is applicable only if
    /// ATTITUDE_TYPE specifies the use of angular velocities in conjunction with either
    /// quaternions or Euler angles.
    ///
    /// **Examples**: ICRF, SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub angvel_frame: Option<String>,
    /// Recommended interpolation method for attitude ephemeris data in the block immediately
    /// following this metadata block.
    ///
    /// **Examples**: LINEAR, HERMITE, LAGRANGE
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub interpolation_method: Option<String>,
    /// Recommended interpolation degree for attitude ephemeris data in the block immediately
    /// following this metadata block. It must be an integer value. This keyword must be used if
    /// the ‘INTERPOLATION_METHOD’ keyword is used.
    ///
    /// **Examples**: 1, 5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<std::num::NonZeroU32>,
}

impl AemMetadata {
    pub fn validate(&self) -> Result<()> {
        // Validation Rule: INTERPOLATION_DEGREE is required if INTERPOLATION_METHOD is used
        if self.interpolation_method.is_some() && self.interpolation_degree.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("AEM Metadata"),
                field: Cow::Borrowed(
                    "INTERPOLATION_DEGREE (required when INTERPOLATION_METHOD is present)",
                ),
                line: None,
            }
            .into());
        }

        // Validation Rule: EULER_ROT_SEQ is required if ATTITUDE_TYPE includes EULER_ANGLE
        if self.attitude_type.contains("EULER_ANGLE") && self.euler_rot_seq.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("AEM Metadata"),
                field: Cow::Borrowed("EULER_ROT_SEQ (required for EULER_ANGLE types)"),
                line: None,
            }
            .into());
        }

        // Validation Rule: ANGVEL_FRAME is required if ATTITUDE_TYPE includes ANGVEL
        if self.attitude_type.contains("ANGVEL") && self.angvel_frame.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("AEM Metadata"),
                field: Cow::Borrowed("ANGVEL_FRAME (required for ANGVEL types)"),
                line: None,
            }
            .into());
        }

        Ok(())
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
            writer.write_pair("ANGVEL_FRAME", v);
        }
        if let Some(v) = &self.interpolation_method {
            writer.write_pair("INTERPOLATION_METHOD", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

/// AEM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AemData {
    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Attitude ephemeris data lines.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[serde(rename = "attitudeState")]
    #[builder(default)]
    pub attitude_states: Vec<AemAttitudeStateWrapper>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AemAttitudeStateWrapper {
    #[serde(
        rename = "quaternionEphemeris",
        skip_serializing_if = "Option::is_none"
    )]
    pub quaternion_ephemeris: Option<crate::common::QuaternionEphemeris>,
    #[serde(
        rename = "quaternionDerivative",
        skip_serializing_if = "Option::is_none"
    )]
    pub quaternion_derivative: Option<crate::common::QuaternionDerivative>,
    #[serde(rename = "quaternionAngVel", skip_serializing_if = "Option::is_none")]
    pub quaternion_ang_vel: Option<crate::common::QuaternionAngVel>,
    #[serde(rename = "eulerAngle", skip_serializing_if = "Option::is_none")]
    pub euler_angle: Option<crate::common::EulerAngle>,
    #[serde(
        rename = "eulerAngleDerivative",
        skip_serializing_if = "Option::is_none"
    )]
    pub euler_angle_derivative: Option<crate::common::EulerAngleDerivative>,
    #[serde(rename = "eulerAngleAngVel", skip_serializing_if = "Option::is_none")]
    pub euler_angle_ang_vel: Option<crate::common::EulerAngleAngVel>,
    #[serde(rename = "spin", skip_serializing_if = "Option::is_none")]
    pub spin: Option<crate::common::Spin>,
    #[serde(rename = "spinNutation", skip_serializing_if = "Option::is_none")]
    pub spin_nutation: Option<crate::common::SpinNutation>,
    #[serde(rename = "spinNutationMom", skip_serializing_if = "Option::is_none")]
    pub spin_nutation_mom: Option<crate::common::SpinNutationMom>,
}

impl From<crate::common::AemAttitudeState> for AemAttitudeStateWrapper {
    fn from(state: crate::common::AemAttitudeState) -> Self {
        let mut wrapper = AemAttitudeStateWrapper {
            quaternion_ephemeris: None,
            quaternion_derivative: None,
            quaternion_ang_vel: None,
            euler_angle: None,
            euler_angle_derivative: None,
            euler_angle_ang_vel: None,
            spin: None,
            spin_nutation: None,
            spin_nutation_mom: None,
        };
        match state {
            crate::common::AemAttitudeState::QuaternionEphemeris(v) => {
                wrapper.quaternion_ephemeris = Some(v)
            }
            crate::common::AemAttitudeState::QuaternionDerivative(v) => {
                wrapper.quaternion_derivative = Some(v)
            }
            crate::common::AemAttitudeState::QuaternionAngVel(v) => {
                wrapper.quaternion_ang_vel = Some(v)
            }
            crate::common::AemAttitudeState::EulerAngle(v) => wrapper.euler_angle = Some(v),
            crate::common::AemAttitudeState::EulerAngleDerivative(v) => {
                wrapper.euler_angle_derivative = Some(v)
            }
            crate::common::AemAttitudeState::EulerAngleAngVel(v) => {
                wrapper.euler_angle_ang_vel = Some(v)
            }
            crate::common::AemAttitudeState::Spin(v) => wrapper.spin = Some(v),
            crate::common::AemAttitudeState::SpinNutation(v) => wrapper.spin_nutation = Some(v),
            crate::common::AemAttitudeState::SpinNutationMom(v) => {
                wrapper.spin_nutation_mom = Some(v)
            }
        }
        wrapper
    }
}

impl AemAttitudeStateWrapper {
    pub fn content(&self) -> Option<crate::common::AemAttitudeState> {
        if let Some(v) = &self.quaternion_ephemeris {
            return Some(crate::common::AemAttitudeState::QuaternionEphemeris(
                v.clone(),
            ));
        }
        if let Some(v) = &self.quaternion_derivative {
            return Some(crate::common::AemAttitudeState::QuaternionDerivative(
                v.clone(),
            ));
        }
        if let Some(v) = &self.quaternion_ang_vel {
            return Some(crate::common::AemAttitudeState::QuaternionAngVel(v.clone()));
        }
        if let Some(v) = &self.euler_angle {
            return Some(crate::common::AemAttitudeState::EulerAngle(v.clone()));
        }
        if let Some(v) = &self.euler_angle_derivative {
            return Some(crate::common::AemAttitudeState::EulerAngleDerivative(
                v.clone(),
            ));
        }
        if let Some(v) = &self.euler_angle_ang_vel {
            return Some(crate::common::AemAttitudeState::EulerAngleAngVel(v.clone()));
        }
        if let Some(v) = &self.spin {
            return Some(crate::common::AemAttitudeState::Spin(v.clone()));
        }
        if let Some(v) = &self.spin_nutation {
            return Some(crate::common::AemAttitudeState::SpinNutation(v.clone()));
        }
        if let Some(v) = &self.spin_nutation_mom {
            return Some(crate::common::AemAttitudeState::SpinNutationMom(v.clone()));
        }
        None
    }
}

impl crate::traits::ToKvn for AemAttitudeStateWrapper {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        if let Some(content) = self.content() {
            content.write_kvn(writer);
        }
    }
}

impl AemData {
    pub fn validate(&self, attitude_type: &str) -> Result<()> {
        for (idx, state) in self.attitude_states.iter().enumerate() {
            match attitude_type {
                "QUATERNION" => {
                    if state.quaternion_ephemeris.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected QUATERNION data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "QUATERNION/DERIVATIVE" => {
                    if state.quaternion_derivative.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected QUATERNION/DERIVATIVE data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "QUATERNION/ANGVEL" | "QUATERNION/RATE" => {
                    if state.quaternion_ang_vel.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected QUATERNION/ANGVEL data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "EULER_ANGLE" => {
                    if state.euler_angle.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected EULER_ANGLE data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "EULER_ANGLE/DERIVATIVE" => {
                    if state.euler_angle_derivative.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected EULER_ANGLE/DERIVATIVE data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "EULER_ANGLE/ANGVEL" | "EULER_ANGLE/RATE" => {
                    if state.euler_angle_ang_vel.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected EULER_ANGLE/ANGVEL data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "SPIN" => {
                    if state.spin.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected SPIN data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "SPIN/NUTATION" => {
                    if state.spin_nutation.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected SPIN/NUTATION data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                "SPIN/NUTATION_MOM" => {
                    if state.spin_nutation_mom.is_none() {
                        return Err(ValidationError::Generic {
                            message: Cow::Owned(format!(
                                "Data line {} expected SPIN/NUTATION_MOM data",
                                idx + 1
                            )),
                            line: None,
                        }
                        .into());
                    }
                }
                _ => {
                    // Unknown type, or maybe user defined.
                    // For now, we strictly validate against known types if possible,
                    // but since the string is open-ended in some contexts, we might warn.
                    // However, XSD lists these as enumerations.
                    return Err(ValidationError::Generic {
                        message: Cow::Owned(format!("Unknown ATTITUDE_TYPE: {}", attitude_type)),
                        line: None,
                    }
                    .into());
                }
            }
        }
        Ok(())
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

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_aem_kvn() -> String {
        r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
TIME_SYSTEM = UTC
START_TIME = 2002-12-18T12:00:00.000
STOP_TIME = 2002-12-18T12:01:00.000
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2002-12-18T12:00:00.000 0.5 0.5 0.5 0.5
2002-12-18T12:01:00.000 0.5 0.5 0.5 0.5
DATA_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_aem_success() {
        let kvn = sample_aem_kvn();
        let aem = Aem::from_kvn(&kvn).expect("AEM parse failed");

        assert_eq!(aem.version, "1.0");
        assert_eq!(aem.body.segment.len(), 1);
        let seg = &aem.body.segment[0];
        assert_eq!(seg.metadata.object_name, "MARS GLOBAL SURVEYOR");
        assert_eq!(seg.data.attitude_states.len(), 2);
    }

    #[test]
    fn test_aem_missing_mandatory_metadata() {
        let kvn = r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        // Missing OBJECT_NAME
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_invalid_time_range() {
        // Technically validation could check if STOP_TIME > START_TIME
        // Our current validate() is empty, but we can still check parsing
        let kvn = sample_aem_kvn();
        let aem = Aem::from_kvn(&kvn).unwrap();
        assert!(
            aem.body.segment[0].metadata.stop_time.as_str()
                > aem.body.segment[0].metadata.start_time.as_str()
        );
    }

    #[test]
    fn test_aem_validation_interpolation_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION
INTERPOLATION_METHOD = HERMITE
# Missing INTERPOLATION_DEGREE
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_validation_euler_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = EULER_ANGLE
# Missing EULER_ROT_SEQ
META_STOP
DATA_START
2023-01-01T00:00:00 10 20 30
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_aem_validation_angvel_reqs() {
        let kvn = r#"CCSDS_AEM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
OBJECT_ID = 999
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
TIME_SYSTEM = UTC
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-01T01:00:00
ATTITUDE_TYPE = QUATERNION/ANGVEL
# Missing ANGVEL_FRAME
META_STOP
DATA_START
2023-01-01T00:00:00 0 0 0 1 0.1 0.1 0.1
DATA_STOP
"#;
        assert!(Aem::from_kvn(kvn).is_err());
    }
    #[test]
    fn test_aem_data_validation_mismatches() {
        use crate::common::*;

        let valid_q = AemAttitudeStateWrapper::from(AemAttitudeState::QuaternionEphemeris(
            QuaternionEphemeris {
                epoch: "2023-01-01T00:00:00".parse().unwrap(),
                quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
            },
        ));

        let valid_euler = AemAttitudeStateWrapper::from(AemAttitudeState::EulerAngle(EulerAngle {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
        }));

        // Type mismatch: Expects QUATERNION, gets EULER_ANGLE
        let data = AemData {
            comment: vec![],
            attitude_states: vec![valid_euler.clone()],
        };
        assert!(data.validate("QUATERNION").is_err());

        // Type mismatch: Expects EULER_ANGLE, gets QUATERNION
        let data_q = AemData {
            comment: vec![],
            attitude_states: vec![valid_q.clone()],
        };
        assert!(data_q.validate("EULER_ANGLE").is_err());

        // Check all other variants against a wrong type declaration
        let cases = vec![
            ("QUATERNION/DERIVATIVE", valid_q.clone()),
            ("QUATERNION/ANGVEL", valid_q.clone()),
            ("EULER_ANGLE/DERIVATIVE", valid_q.clone()),
            ("EULER_ANGLE/ANGVEL", valid_q.clone()),
            ("SPIN", valid_q.clone()),
            ("SPIN/NUTATION", valid_q.clone()),
            ("SPIN/NUTATION_MOM", valid_q.clone()),
        ];

        for (type_str, state) in cases {
            let d = AemData {
                comment: vec![],
                attitude_states: vec![state],
            };
            assert!(
                d.validate(type_str).is_err(),
                "Expected error for type {}",
                type_str
            );
        }
    }
}
