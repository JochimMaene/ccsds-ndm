// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AdmHeader, AngVelState, EulerAngleState, InertiaState, QuaternionState, SpinState,
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub object_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_name: Option<String>,
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

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApmData {
    // Note: Comments can be inside logical blocks, but also interspersed?
    // Usually comments belong to a block.
    // We model the content as optional fields for each block.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>, // Top level comments in data section?
    #[serde(rename = "quaternionState", default, skip_serializing_if = "Option::is_none")]
    pub quaternion_state: Option<QuaternionState>,
    #[serde(rename = "eulerAngleState", default, skip_serializing_if = "Option::is_none")]
    pub euler_angle_state: Option<EulerAngleState>,
    #[serde(rename = "angVelState", default, skip_serializing_if = "Option::is_none")]
    pub ang_vel_state: Option<AngVelState>, // Note: renamed to standard
    #[serde(rename = "spinState", default, skip_serializing_if = "Option::is_none")]
    pub spin_state: Option<SpinState>,
    #[serde(rename = "inertiaState", default, skip_serializing_if = "Option::is_none")]
    pub inertia_state: Option<InertiaState>,
    #[serde(rename = "maneuverParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub maneuver_parameters: Vec<ManeuverParameters>, 
}

impl ApmData {
    pub fn validate(&self) -> Result<()> {
        if self.quaternion_state.is_none()
            && self.euler_angle_state.is_none()
            && self.ang_vel_state.is_none()
            && self.spin_state.is_none()
            && self.inertia_state.is_none()
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
        if let Some(block) = &self.quaternion_state {
            writer.write_line("QUAT_START");
            block.write_kvn(writer);
            writer.write_line("QUAT_STOP");
            writer.write_line("");
        }
        if let Some(block) = &self.euler_angle_state {
            writer.write_line("EULER_START");
            block.write_kvn(writer);
            writer.write_line("EULER_STOP");
            writer.write_line("");
        }
        if let Some(block) = &self.ang_vel_state {
            writer.write_line("ANGVEL_START");
            block.write_kvn(writer);
            writer.write_line("ANGVEL_STOP");
            writer.write_line("");
        }
        if let Some(block) = &self.spin_state {
            writer.write_line("SPIN_START");
            block.write_kvn(writer);
            writer.write_line("SPIN_STOP");
            writer.write_line("");
        }
        if let Some(block) = &self.inertia_state {
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

impl ToKvn for ManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_EPOCH_START", self.man_epoch_start);
        writer.write_measure("MAN_DURATION", &self.man_duration.to_unit_value());
        writer.write_pair("MAN_REF_FRAME", &self.man_ref_frame);
        writer.write_measure("MAN_TOR_1", &self.man_tor_1);
        writer.write_measure("MAN_TOR_2", &self.man_tor_2);
        writer.write_measure("MAN_TOR_3", &self.man_tor_3);
        if let Some(m) = &self.man_delta_mass {
            writer.write_measure("MAN_DELTA_MASS", &m.to_unit_value());
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ManeuverParameters {
     // TODO: definitions
     #[serde(default, skip_serializing_if = "Vec::is_empty")]
     pub comment: Vec<String>,
     pub man_epoch_start: Epoch,
     pub man_duration: Duration,
     pub man_ref_frame: String,
     pub man_tor_1: Torque,
     pub man_tor_2: Torque,
     pub man_tor_3: Torque,
     #[serde(default, skip_serializing_if = "Option::is_none")]
     pub man_delta_mass: Option<Mass>,
}
