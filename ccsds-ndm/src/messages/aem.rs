// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

mod kvn;
mod xml;

use crate::common::{AdmHeader, AemAttitudeState};
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::traits::Ndm;
#[cfg(test)]
use crate::traits::Validate;
use crate::types::*;
use serde::{Deserialize, Serialize};

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
    #[builder(required, default = Some("CCSDS_AEM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "2.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Aem {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Aem,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Aem {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let normalized = crate::kvn::normalize_line_endings(kvn);
        kvn::validate_kvn_syntax(&normalized)?;
        let aem = Self::from_kvn_str(&normalized)?;
        crate::traits::Validate::validate(&aem)?;
        Ok(aem)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        Self::from_xml_with_options(xml, &crate::options::ParseOptions::default())
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

impl crate::traits::Validate for AemBody {
    fn validate(&self) -> Result<()> {
        if self.segment.is_empty() {
            return Err(ValidationError::missing_required(
                "AEM Body",
                "segment (at least one required)",
            )
            .into());
        }
        for segment in &self.segment {
            segment.validate()?;
        }
        if let Some(error) = self.first_cross_segment_error() {
            return Err(error.into());
        }
        Ok(())
    }
}

impl AemBody {
    fn first_cross_segment_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        if let Some(error) = self.first_identity_error() {
            return Some(error);
        }

        for (index, pair) in self.segment.windows(2).enumerate() {
            let (Some(previous), Some(current)) = (
                pair[0].metadata.useable_stop_time,
                pair[1].metadata.useable_start_time,
            ) else {
                continue;
            };
            if previous.into_epoch().cmp_same_branch(&current.into_epoch())
                == Some(Ordering::Greater)
            {
                return Some(
                    ValidationError::InvalidValue {
                        field: "USEABLE_START_TIME".into(),
                        value: current.to_string(),
                        expected: "no earlier than the preceding segment's USEABLE_STOP_TIME"
                            .into(),
                        line: None,
                    }
                    .at_path(format!(
                        "segment[{}].metadata.useable_start_time",
                        index + 1
                    )),
                );
            }
        }
        None
    }

    /// Enforce the single-object rule from 504.0-B-2 section 2.3.1. Figure G-4 uses different
    /// casing for one object's name, while OBJECT_ID remains exact. TIME_SYSTEM is only fixed
    /// within each segment by section 4.2.4.8.2.
    fn first_identity_error(&self) -> Option<ValidationError> {
        let first = self.segment.first()?;
        let object_name = &first.metadata.object_name;
        let object_id = &first.metadata.object_id;
        for (index, segment) in self.segment.iter().enumerate().skip(1) {
            if !segment
                .metadata
                .object_name
                .eq_ignore_ascii_case(object_name)
                || segment.metadata.object_id != *object_id
            {
                return Some(
                    ValidationError::InvalidValue {
                        field: "OBJECT_NAME/OBJECT_ID".into(),
                        value: format!(
                            "{}/{}",
                            segment.metadata.object_name, segment.metadata.object_id
                        ),
                        expected: format!(
                            "one object throughout the AEM (expected {object_name}/{object_id})"
                        )
                        .into(),
                        line: None,
                    }
                    .at_path(format!("segment[{index}].metadata.object_id")),
                );
            }
        }
        None
    }
}

impl crate::traits::Validate for AemSegment {
    fn validate(&self) -> Result<()> {
        self.validate_inner()
    }
}

impl AemSegment {
    pub fn validate(&self) -> Result<()> {
        self.validate_inner()
    }

    fn validate_inner(&self) -> Result<()> {
        self.metadata.validate_inner()?;
        self.data.validate_structure()?;
        self.data
            .validate_attitude_type(&self.metadata.attitude_type)?;
        match self.first_timeline_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn first_timeline_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        let start = self.metadata.start_time.into_epoch();
        let stop = self.metadata.stop_time.into_epoch();
        let mut previous: Option<crate::types::Epoch> = None;
        for (index, state) in self.data.attitude_states.iter().enumerate() {
            let epoch = state.epoch();
            let current = epoch.into_epoch();
            if start.cmp_same_branch(&current) == Some(Ordering::Greater)
                || current.cmp_same_branch(&stop) == Some(Ordering::Greater)
            {
                return Some(
                    ValidationError::OutOfRange {
                        name: "attitudeState EPOCH".into(),
                        value: epoch.to_string(),
                        expected: format!(
                            "within START_TIME {} and STOP_TIME {}",
                            self.metadata.start_time, self.metadata.stop_time
                        )
                        .into(),
                        line: None,
                    }
                    .at_path(format!("data.attitude_states[{index}].epoch")),
                );
            }
            if matches!(
                previous,
                Some(prior) if prior.cmp_same_branch(&current) != Some(Ordering::Less)
            ) {
                return Some(
                    ValidationError::InvalidValue {
                        field: "attitudeState EPOCH".into(),
                        value: epoch.to_string(),
                        expected: "strictly increasing, non-repeated attitude time tags".into(),
                        line: None,
                    }
                    .at_path(format!("data.attitude_states[{index}].epoch")),
                );
            }
            previous = Some(current);
        }
        None
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AemBody {
    #[serde(rename = "segment")]
    pub segment: Vec<AemSegment>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct AemSegment {
    pub metadata: AemMetadata,
    pub data: AemData,
}

/// AEM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
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
    pub start_time: CalendarEpoch,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub useable_start_time: Option<CalendarEpoch>,
    /// Optional stop of USEABLE time span covered by attitude ephemeris data immediately following
    /// this metadata block. (See also USEABLE_START_TIME.)
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub useable_stop_time: Option<CalendarEpoch>,
    /// End of TOTAL time span covered by the attitude ephemeris data immediately following this
    /// metadata block.
    ///
    /// **Examples**: 1996-12-18T14:28:15.11
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    pub stop_time: CalendarEpoch,
    /// The type of information contained in the data lines. This keyword must have a value from the
    /// set specified at the right. (See table 4-4 for details of the data contained in each line.)
    ///
    /// **Examples**: QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE,
    /// EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, SPIN/NUTATION_MOM
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[builder(into)]
    pub attitude_type: AttitudeTypeType,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub euler_rot_seq: Option<RotSeq>,
    /// The frame of reference in which angular velocity data are specified. The set of allowed
    /// values is described in annex B, subsection B3. This keyword is applicable only if
    /// ATTITUDE_TYPE specifies the use of angular velocities in conjunction with either
    /// quaternions or Euler angles.
    ///
    /// **Examples**: ICRF, SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub angvel_frame: Option<String>,
    /// Recommended interpolation method for attitude ephemeris data in the block immediately
    /// following this metadata block.
    ///
    /// **Examples**: LINEAR, HERMITE, LAGRANGE
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub interpolation_method: Option<String>,
    /// Recommended interpolation degree for attitude ephemeris data in the block immediately
    /// following this metadata block. It must be an integer value. This keyword must be used if
    /// the ‘INTERPOLATION_METHOD’ keyword is used.
    ///
    /// **Examples**: 1, 5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub interpolation_degree: Option<InterpolationDegree>,
}

impl AemMetadata {
    pub fn validate(&self) -> Result<()> {
        self.validate_inner()
    }

    fn validate_inner(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "OBJECT_NAME").into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "OBJECT_ID").into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "TIME_SYSTEM").into());
        }
        if self.ref_frame_a.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "REF_FRAME_A").into());
        }
        if self.ref_frame_b.trim().is_empty() {
            return Err(ValidationError::missing_required("AEM Metadata", "REF_FRAME_B").into());
        }
        // Validation Rule: INTERPOLATION_DEGREE is required if INTERPOLATION_METHOD is used
        if self.interpolation_method.is_some() && self.interpolation_degree.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "INTERPOLATION_DEGREE (required when INTERPOLATION_METHOD is present)",
            )
            .into());
        }

        let requires_euler_rot_seq = matches!(
            self.attitude_type,
            AttitudeTypeType::EulerAngle
                | AttitudeTypeType::EulerAngleDerivative
                | AttitudeTypeType::EulerAngleAngVel
        );
        let requires_angvel_frame = matches!(
            self.attitude_type,
            AttitudeTypeType::QuaternionAngVel | AttitudeTypeType::EulerAngleAngVel
        );

        // Validation Rule: EULER_ROT_SEQ is required if ATTITUDE_TYPE includes EULER_ANGLE
        if requires_euler_rot_seq && self.euler_rot_seq.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "EULER_ROT_SEQ (required for EULER_ANGLE types)",
            )
            .into());
        }

        // Validation Rule: ANGVEL_FRAME is required if ATTITUDE_TYPE includes ANGVEL
        if requires_angvel_frame && self.angvel_frame.is_none() {
            return Err(ValidationError::missing_required(
                "AEM Metadata",
                "ANGVEL_FRAME (required for ANGVEL types)",
            )
            .into());
        }

        match self.first_time_span_error() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    fn first_time_span_error(&self) -> Option<ValidationError> {
        use std::cmp::Ordering;

        let start = self.start_time.into_epoch();
        let stop = self.stop_time.into_epoch();
        if start.cmp_same_branch(&stop) == Some(Ordering::Greater) {
            return Some(ValidationError::InvalidValue {
                field: "START_TIME/STOP_TIME".into(),
                value: format!("{} > {}", self.start_time, self.stop_time),
                expected: "START_TIME no later than STOP_TIME".into(),
                line: None,
            });
        }
        for (field, value) in [
            ("USEABLE_START_TIME", self.useable_start_time),
            ("USEABLE_STOP_TIME", self.useable_stop_time),
        ] {
            if let Some(value) = value {
                let value_epoch = value.into_epoch();
                if start.cmp_same_branch(&value_epoch) == Some(Ordering::Greater)
                    || value_epoch.cmp_same_branch(&stop) == Some(Ordering::Greater)
                {
                    return Some(ValidationError::OutOfRange {
                        name: field.into(),
                        value: value.to_string(),
                        expected: "within the total START_TIME/STOP_TIME span".into(),
                        line: None,
                    });
                }
            }
        }
        if let (Some(useable_start), Some(useable_stop)) =
            (self.useable_start_time, self.useable_stop_time)
        {
            if useable_start
                .into_epoch()
                .cmp_same_branch(&useable_stop.into_epoch())
                == Some(Ordering::Greater)
            {
                return Some(ValidationError::InvalidValue {
                    field: "USEABLE_START_TIME/USEABLE_STOP_TIME".into(),
                    value: format!("{useable_start} > {useable_stop}"),
                    expected: "USEABLE_START_TIME no later than USEABLE_STOP_TIME".into(),
                    line: None,
                });
            }
        }
        None
    }
}

impl crate::traits::Validate for AemMetadata {
    fn validate(&self) -> Result<()> {
        self.validate_inner()
    }
}

/// AEM Data Section.
#[derive(Debug, PartialEq, Clone, bon::Builder)]
pub struct AemData {
    /// Comments allowed only at the beginning of the Data section. Each comment line shall begin
    /// with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[builder(default)]
    pub comment: Vec<String>,
    /// Attitude ephemeris data lines.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 4.2.4.
    #[builder(default)]
    pub attitude_states: Vec<AemAttitudeState>,
}

impl AemAttitudeState {
    fn epoch(&self) -> &CalendarEpoch {
        match self {
            Self::QuaternionEphemeris(v) => &v.epoch,
            Self::QuaternionDerivative(v) => &v.epoch,
            Self::QuaternionAngVel(v) => &v.epoch,
            Self::EulerAngle(v) => &v.epoch,
            Self::EulerAngleDerivative(v) => &v.epoch,
            Self::EulerAngleAngVel(v) => &v.epoch,
            Self::Spin(v) => &v.epoch,
            Self::SpinNutation(v) => &v.epoch,
            Self::SpinNutationMom(v) => &v.epoch,
        }
    }

    fn matches_type(&self, attitude_type: &AttitudeTypeType) -> bool {
        matches!(
            (self, attitude_type),
            (Self::QuaternionEphemeris(_), AttitudeTypeType::Quaternion)
                | (
                    Self::QuaternionDerivative(_),
                    AttitudeTypeType::QuaternionDerivative
                )
                | (
                    Self::QuaternionAngVel(_),
                    AttitudeTypeType::QuaternionAngVel
                )
                | (Self::EulerAngle(_), AttitudeTypeType::EulerAngle)
                | (
                    Self::EulerAngleDerivative(_),
                    AttitudeTypeType::EulerAngleDerivative
                )
                | (
                    Self::EulerAngleAngVel(_),
                    AttitudeTypeType::EulerAngleAngVel
                )
                | (Self::Spin(_), AttitudeTypeType::Spin)
                | (Self::SpinNutation(_), AttitudeTypeType::SpinNutation)
                | (Self::SpinNutationMom(_), AttitudeTypeType::SpinNutationMom)
        )
    }

    fn validate_state(&self) -> Result<()> {
        let angles = |values: &[(&'static str, f64)]| -> Result<()> {
            for (name, value) in values {
                Angle::validate_value(*value, name)?;
            }
            Ok(())
        };
        let finite = |values: &[(&'static str, f64)]| -> Result<()> {
            for (name, value) in values {
                if !value.is_finite() {
                    return Err(ValidationError::InvalidValue {
                        field: (*name).into(),
                        value: value.to_string(),
                        expected: "a finite number".into(),
                        line: None,
                    }
                    .into());
                }
            }
            Ok(())
        };

        match self {
            Self::QuaternionEphemeris(value) => {
                crate::traits::Validate::validate(&value.quaternion)
            }
            Self::QuaternionDerivative(value) => {
                crate::traits::Validate::validate(&value.quaternion)?;
                finite(&[
                    ("Q1_DOT", value.quaternion_dot.q1_dot.value),
                    ("Q2_DOT", value.quaternion_dot.q2_dot.value),
                    ("Q3_DOT", value.quaternion_dot.q3_dot.value),
                    ("QC_DOT", value.quaternion_dot.qc_dot.value),
                ])
            }
            Self::QuaternionAngVel(value) => {
                crate::traits::Validate::validate(&value.quaternion)?;
                finite(&[
                    ("ANGVEL_X", value.ang_vel.angvel_x.value),
                    ("ANGVEL_Y", value.ang_vel.angvel_y.value),
                    ("ANGVEL_Z", value.ang_vel.angvel_z.value),
                ])
            }
            Self::EulerAngle(value) => angles(&[
                ("ANGLE_1", value.angle_1.value),
                ("ANGLE_2", value.angle_2.value),
                ("ANGLE_3", value.angle_3.value),
            ]),
            Self::EulerAngleDerivative(value) => {
                angles(&[
                    ("ANGLE_1", value.angle_1.value),
                    ("ANGLE_2", value.angle_2.value),
                    ("ANGLE_3", value.angle_3.value),
                ])?;
                finite(&[
                    ("ANGLE_1_DOT", value.angle_1_dot.value),
                    ("ANGLE_2_DOT", value.angle_2_dot.value),
                    ("ANGLE_3_DOT", value.angle_3_dot.value),
                ])
            }
            Self::EulerAngleAngVel(value) => {
                angles(&[
                    ("ANGLE_1", value.angle_1.value),
                    ("ANGLE_2", value.angle_2.value),
                    ("ANGLE_3", value.angle_3.value),
                ])?;
                finite(&[
                    ("ANGVEL_X", value.angvel_x.value),
                    ("ANGVEL_Y", value.angvel_y.value),
                    ("ANGVEL_Z", value.angvel_z.value),
                ])
            }
            Self::Spin(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                ])?;
                finite(&[("SPIN_ANGLE_VEL", value.spin_angle_vel.value)])
            }
            Self::SpinNutation(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                    ("NUTATION", value.nutation.value),
                    ("NUTATION_PHASE", value.nutation_phase.value),
                ])?;
                finite(&[("SPIN_ANGLE_VEL", value.spin_angle_vel.value)])?;
                Duration::validate_value(value.nutation_per.value, "NUTATION_PER")
            }
            Self::SpinNutationMom(value) => {
                angles(&[
                    ("SPIN_ALPHA", value.spin_alpha.value),
                    ("SPIN_DELTA", value.spin_delta.value),
                    ("SPIN_ANGLE", value.spin_angle.value),
                    ("MOMENTUM_ALPHA", value.momentum_alpha.value),
                    ("MOMENTUM_DELTA", value.momentum_delta.value),
                ])?;
                finite(&[
                    ("SPIN_ANGLE_VEL", value.spin_angle_vel.value),
                    ("NUTATION_VEL", value.nutation_vel.value),
                ])
            }
        }
    }
}

impl crate::traits::Validate for AemData {
    fn validate(&self) -> Result<()> {
        self.validate_structure()
    }
}

impl AemData {
    fn validate_structure(&self) -> Result<()> {
        if self.attitude_states.is_empty() {
            return Err(ValidationError::missing_required(
                "AEM Data",
                "attitudeState (at least one required)",
            )
            .into());
        }
        for state in &self.attitude_states {
            state.validate_state()?;
        }
        Ok(())
    }

    fn validate_attitude_type(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
        for (idx, state) in self.attitude_states.iter().enumerate() {
            if !state.matches_type(attitude_type) {
                return Err(ValidationError::generic(format!(
                    "Data line {} expected {} data",
                    idx + 1,
                    attitude_type
                ))
                .into());
            }
        }
        Ok(())
    }

    pub fn validate(&self, attitude_type: &AttitudeTypeType) -> Result<()> {
        self.validate_structure()?;
        self.validate_attitude_type(attitude_type)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aem_data_validation_mismatches() {
        use crate::common::*;

        let valid_q = AemAttitudeState::QuaternionEphemeris(QuaternionEphemeris {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            quaternion: Quaternion::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        });

        let valid_euler = AemAttitudeState::EulerAngle(EulerAngle {
            epoch: "2023-01-01T00:00:00".parse().unwrap(),
            angle_1: Angle::new(10.0, None).unwrap(),
            angle_2: Angle::new(20.0, None).unwrap(),
            angle_3: Angle::new(30.0, None).unwrap(),
        });

        // Type mismatch: Expects QUATERNION, gets EULER_ANGLE
        let data = AemData {
            comment: vec![],
            attitude_states: vec![valid_euler.clone()],
        };
        assert!(data.validate(&AttitudeTypeType::Quaternion).is_err());

        // Type mismatch: Expects EULER_ANGLE, gets QUATERNION
        let data_q = AemData {
            comment: vec![],
            attitude_states: vec![valid_q.clone()],
        };
        assert!(data_q.validate(&AttitudeTypeType::EulerAngle).is_err());

        // Check all other variants against a wrong type declaration
        let cases = vec![
            AttitudeTypeType::QuaternionDerivative,
            AttitudeTypeType::QuaternionAngVel,
            AttitudeTypeType::EulerAngleDerivative,
            AttitudeTypeType::EulerAngleAngVel,
            AttitudeTypeType::Spin,
            AttitudeTypeType::SpinNutation,
            AttitudeTypeType::SpinNutationMom,
        ];

        for attitude_type in cases {
            let d = AemData {
                comment: vec![],
                attitude_states: vec![valid_q.clone()],
            };
            assert!(
                d.validate(&attitude_type).is_err(),
                "Expected error for type {}",
                attitude_type
            );
        }
    }

    #[test]
    fn test_aem_data_requires_attitude_state() {
        let data = AemData {
            comment: vec![],
            attitude_states: vec![],
        };
        assert!(crate::traits::Validate::validate(&data).is_err());
    }

    #[test]
    fn test_aem_body_requires_segment() {
        let body = AemBody { segment: vec![] };
        assert!(body.validate().is_err());
    }
}
