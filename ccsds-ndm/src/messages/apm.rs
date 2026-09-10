// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

mod kvn;
mod xml;

use crate::common::{
    AdmHeader, AngVelState, AttManeuverState, EulerAngleState, InertiaState, QuaternionState,
    SpinState,
};

use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::traits::Ndm;
#[cfg(test)]
use crate::traits::Validate;
use crate::types::*;
use serde::{Deserialize, Serialize};

/// Attitude Parameter Message (APM).
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message
/// is suited to interagency exchanges that involve automated interaction and/or human
/// interaction, and/or human interaction, and do not require high-fidelity dynamic modeling.
///
/// The APM requires the use of a propagation technique to determine the attitude state at
/// times different from the specified epoch.
///
/// **CCSDS Reference**: 504.0-B-2, Section 3.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "apm")]
pub struct Apm {
    pub header: AdmHeader,
    pub body: ApmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_APM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "2.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Apm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Apm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Apm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        kvn::validate_kvn_syntax(kvn)?;
        let apm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&apm)?;
        Ok(apm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::validate_document_root(xml, b"apm", "APM")?;
        xml::validate_xml_sequences(xml)?;
        let apm: Self = crate::xml::from_str_with_context(xml, "APM")?;
        crate::traits::Validate::validate(&apm)?;
        Ok(apm)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct ApmBody {
    // XSD says minOccurs=1 maxOccurs=1 for APM segment!
    #[serde(rename = "segment")]
    pub segment: ApmSegment,
}

impl crate::traits::Validate for ApmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct ApmSegment {
    pub metadata: ApmMetadata,
    pub data: ApmData,
}

impl crate::traits::Validate for ApmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()
    }
}

impl ApmSegment {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

/// APM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct ApmMetadata {
    /// Comments (allowed only at the beginning of the APM Metadata before OBJECT_NAME). Each
    /// comment line shall begin with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which the attitude state is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference [ADM-2], which include object
    /// name and international designator). When OBJECT_NAME is not known or cannot be disclosed,
    /// the value should be set to UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1, MARS PATHFINDER, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Spacecraft identifier of the object corresponding to the attitude data to be given. While
    /// there is no CCSDS-based restriction on the value for this keyword, it is recommended to use
    /// international designators from the UN Office of Outer Space Affairs (reference [ADM-2]).
    /// Recommended values have the format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three
    /// digit serial number of launch in year YYYY (with leading zeros). P{PP} = At least one
    /// letter for the identification of the part brought into space by the launch. In cases in
    /// which the asset is not listed in reference [ADM-2], the UN Office of Outer Space Affairs
    /// designator index format is not used, or the content cannot be disclosed, the value should
    /// be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH, BARYCENTER, MOON
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Time system used for attitude and maneuver data. The set of allowed values is described in
    /// annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[builder(into)]
    pub time_system: String,
}

impl crate::traits::Validate for ApmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

/// APM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct ApmData {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub epoch: CalendarEpoch,
    /// Attitude quaternion. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "quaternionState",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub quaternion_state: Vec<QuaternionState>,
    /// Euler angle elements. All mandatory elements of the logical block are to be provided if the
    /// block is present. (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "eulerAngleState",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub euler_angle_state: Vec<EulerAngleState>,
    /// Angular velocity vector. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "angularVelocity",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub angular_velocity: Vec<AngVelState>,
    /// Spin. All mandatory elements are to be provided if the block is present. (See annex F for
    /// conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "spin", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub spin: Vec<SpinState>,
    /// Inertia. All mandatory elements are to be provided if the block is present. (See annex F
    /// for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "inertia", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub inertia: Vec<InertiaState>,
    /// Maneuver Parameters. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(
        rename = "maneuverParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub maneuver_parameters: Vec<AttManeuverState>,
}

impl crate::traits::Validate for ApmData {
    fn validate(&self) -> Result<()> {
        if self.quaternion_state.is_empty()
            && self.euler_angle_state.is_empty()
            && self.angular_velocity.is_empty()
            && self.spin.is_empty()
            && self.inertia.is_empty()
            && self.maneuver_parameters.is_empty()
        {
            return Err(ValidationError::MissingRequiredField {
                block: "APM Data".into(),
                field: "At least one logical block".into(),
                line: None,
            }
            .into());
        }
        for block in &self.quaternion_state {
            block.quaternion.validate()?;
        }
        for block in &self.spin {
            block.validate()?;
        }
        // These three repeated blocks previously had no validator reached from any root, so an
        // out-of-range Euler angle produced XML the reference schema rejects on `maxExclusive`.
        for block in &self.euler_angle_state {
            crate::traits::Validate::validate(block)?;
        }
        for block in &self.angular_velocity {
            crate::traits::Validate::validate(block)?;
        }
        for block in &self.inertia {
            crate::traits::Validate::validate(block)?;
        }
        Ok(())
    }
}

impl ApmData {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_apm_kvn() -> String {
        r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2002-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
OBJECT_ID = 1996-062A
TIME_SYSTEM = UTC
META_STOP
EPOCH = 2002-11-04T17:22:31
QUAT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
Q1 = 0.5
Q2 = 0.5
Q3 = 0.5
QC = 0.5
QUAT_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_apm_success() {
        let kvn = sample_apm_kvn();
        let apm = Apm::from_kvn(&kvn).expect("APM parse failed");

        assert_eq!(apm.version, "2.0");
        assert_eq!(
            apm.body.segment.metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
        assert_eq!(apm.body.segment.data.quaternion_state[0].quaternion.q1, 0.5);
    }

    /// The two rules that reject an otherwise well-formed APM, as mutations of a shipped fixture.
    ///
    /// `apm_g2.kvn` is the baseline because its `OBJECT_NAME` is not preceded by comments:
    /// dropping that line from `apm_g1.kvn` leaves a `COMMENT` at the head of the metadata and the
    /// message is refused for comment placement instead, which is not the rule under test.
    #[test]
    fn apm_rejects_missing_object_name_or_all_data_blocks() {
        const FIXTURE: &str = include_str!("../../data/kvn/apm_g2.kvn");
        Apm::from_kvn(FIXTURE).expect("baseline fixture must satisfy both rules");

        let attitude_at = FIXTURE.find("EULER_START").unwrap();
        for (mutated, expected) in [
            (
                FIXTURE[..attitude_at].to_owned(),
                "Missing required field: At least one logical block in block APM Data",
            ),
            (
                FIXTURE.replace("OBJECT_NAME = GOES-P\n", ""),
                "Missing required field: OBJECT_NAME in block APM Metadata",
            ),
        ] {
            let error = Apm::from_kvn(&mutated).expect_err("mutation accepted");
            assert!(
                error.to_string().contains(expected),
                "diagnostic did not name {expected}: {error}"
            );
        }
    }

    #[test]
    fn test_apm_multiple_blocks() {
        let kvn = r#"CCSDS_APM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT1
OBJECT_ID = 999
TIME_SYSTEM = UTC
EPOCH = 2023-01-01T00:00:00
QUAT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
Q1 = 0
Q2 = 0
Q3 = 0
QC = 1
QUAT_STOP
EULER_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
EULER_ROT_SEQ = XYZ
ANGLE_1 = 10 [deg]
ANGLE_2 = 20 [deg]
ANGLE_3 = 30 [deg]
EULER_STOP
"#;
        let apm = Apm::from_kvn(kvn).unwrap();
        assert_eq!(apm.body.segment.data.quaternion_state.len(), 1);
        assert_eq!(apm.body.segment.data.euler_angle_state.len(), 1);
    }
    #[test]
    fn test_apm_validation_single_blocks() {
        // Test that having just one block is sufficient
        let mut apm = Apm::from_kvn(&sample_apm_kvn()).unwrap();

        // Clear all blocks
        apm.body.segment.data.quaternion_state.clear();
        let error = apm
            .validate()
            .expect_err("a data section with no block was accepted");
        assert!(
            error.to_string().contains("At least one logical block"),
            "unexpected diagnostic: {error}"
        );

        // Add just Inertia
        apm.body.segment.data.inertia.push(InertiaState {
            comment: vec![],
            inertia_ref_frame: "SC_BODY".to_string(),
            ixx: crate::types::Moment::new(1.0, None),
            iyy: crate::types::Moment::new(2.0, None),
            izz: crate::types::Moment::new(3.0, None),
            ixy: crate::types::Moment::new(0.0, None),
            ixz: crate::types::Moment::new(0.0, None),
            iyz: crate::types::Moment::new(0.0, None),
        });
        assert!(apm.validate().is_ok());

        // Clear and add just Angular Velocity
        apm.body.segment.data.inertia.clear();
        apm.body.segment.data.angular_velocity.push(AngVelState {
            comment: vec![],
            ref_frame_a: "GCRF".to_string(),
            ref_frame_b: "SC_BODY".to_string(),
            angvel_frame: crate::types::AngVelFrameType("SC_BODY".to_string()),
            angvel_x: crate::types::AngleRate::new(0.1, None),
            angvel_y: crate::types::AngleRate::new(0.1, None),
            angvel_z: crate::types::AngleRate::new(0.1, None),
        });
        assert!(apm.validate().is_ok());
    }
}
