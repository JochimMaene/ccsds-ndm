// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AdmHeader, AngVelState, AttManeuverState, EulerAngleState, InertiaState, QuaternionState, SpinState,
};

use crate::error::{Result, ValidationError};
use crate::kvn::ser::KvnWriter;
use crate::kvn::parser::ParseKvn;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root APM Structure
//----------------------------------------------------------------------

/// Attitude Parameter Message (APM).
///
/// An APM specifies the attitude state of a single object at a specified epoch. This message is
/// suited to interagency exchanges that (1) involve automated interaction and/or human
/// interaction, and (2) do not require high-fidelity dynamic modeling.
///
/// **CCSDS Reference**: 504.0-B-2, Section 3.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "apm")]
pub struct Apm {
    pub header: AdmHeader,
    pub body: ApmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Apm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let apm = Self::from_kvn_str(kvn)?;
        apm.validate()?;
        Ok(apm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let apm: Self = crate::xml::from_str_with_context(xml, "APM")?;
        apm.validate()?;
        Ok(apm)
    }
}

impl Apm {
    pub fn validate(&self) -> Result<()> {
        // Validation logic can be added here
        // E.g. check at least one logical block is present in segment
        self.body.segment.validate()?;
        Ok(())
    }
}

impl ToKvn for Apm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_APM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ApmBody {
    // XSD says minOccurs=1 maxOccurs=1 for APM segment!
    #[serde(rename = "segment")]
    pub segment: ApmSegment,
}

impl ToKvn for ApmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ApmSegment {
    pub metadata: ApmMetadata,
    pub data: ApmData,
}

impl ApmSegment {
    pub fn validate(&self) -> Result<()> {
        self.data.validate()
    }
}

impl ToKvn for ApmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_line("META_START");
        self.metadata.write_kvn(writer);
        writer.write_line("META_STOP");
        writer.write_line("");
        // APM Data in KVN doesn't have "DATA_START"/"DATA_STOP" wrapper around the whole thing?
        // Wait, APM structure in KVN:
        // META_START ... META_STOP
        // QUAT_START ... QUAT_STOP
        // EULER_START ... EULER_STOP
        // etc.
        // It does NOT have a single DATA_START block wrapping everything usually.
        // Let's check CCSDS 504.0-B-2 Section 3.
        // "The APM Data Section shall follow the APM Metadata Section."
        // Structure:
        // Header
        // Metadata
        // Data (composed of logical blocks)
        self.data.write_kvn(writer);
    }
}

/// APM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApmMetadata {
    /// Comments (allowed only at the beginning of the APM Metadata before OBJECT_NAME). Each
    /// comment line shall begin with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub object_id: String,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH, BARYCENTER, MOON
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_name: Option<String>,
    /// Time system used for attitude and maneuver data. The set of allowed values is described in
    /// annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.3.
    pub time_system: String,
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

/// APM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApmData {
    /// One or more comment line(s). Each comment line shall begin with this keyword.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Epoch of the attitude elements and optional logical blocks.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    pub epoch: Epoch,
    /// Attitude quaternion. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "quaternionState", default, skip_serializing_if = "Vec::is_empty")]
    pub quaternion_state: Vec<QuaternionState>,
    /// Euler angle elements. All mandatory elements of the logical block are to be provided if the
    /// block is present. (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "eulerAngleState", default, skip_serializing_if = "Vec::is_empty")]
    pub euler_angle_state: Vec<EulerAngleState>,
    /// Angular velocity vector. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "angularVelocity", default, skip_serializing_if = "Vec::is_empty")]
    pub angular_velocity: Vec<AngVelState>,
    /// Spin. All mandatory elements are to be provided if the block is present. (See annex F for
    /// conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "spin", default, skip_serializing_if = "Vec::is_empty")]
    pub spin: Vec<SpinState>,
    /// Inertia. All mandatory elements are to be provided if the block is present. (See annex F
    /// for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "inertia", default, skip_serializing_if = "Vec::is_empty")]
    pub inertia: Vec<InertiaState>,
    /// Maneuver Parameters. All mandatory elements are to be provided if the block is present.
    /// (See annex F for conventions and further detail.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 3.2.4.
    #[serde(rename = "maneuverParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub maneuver_parameters: Vec<AttManeuverState>,
}

impl ApmData {
    pub fn validate(&self) -> Result<()> {
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
             }.into());
        }
        Ok(())
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
