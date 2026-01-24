// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::error::{Result, ValidationError};
use crate::kvn::ser::KvnWriter;
use crate::kvn::parser::ParseKvn;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root AEM Structure
//----------------------------------------------------------------------

/// Attitude Ephemeris Message (AEM).
///
/// **CCSDS Reference**: 504.0-B-2, Section 4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "aem")]
pub struct Aem {
    pub header: AdmHeader,
    pub body: AemBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Aem {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let aem = Self::from_kvn_str(kvn)?;
        aem.validate()?;
        Ok(aem)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let aem: Self = crate::xml::from_str_with_context(xml, "AEM")?;
        aem.validate()?;
        Ok(aem)
    }
}

impl Aem {
    pub fn validate(&self) -> Result<()> {
        // Validation logic can be added here
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AemMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub object_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_name: Option<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    pub time_system: String,
    pub start_time: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    pub stop_time: Epoch,
    pub attitude_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub euler_rot_seq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_frame: Option<String>, // XSD says ANGVEL_FRAME but in KVN it might be RATE_FRAME? Book says RATE_FRAME is allowed. Check XSD/Book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<std::num::NonZeroU32>,
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
        if let Some(v) = &self.rate_frame {
            writer.write_pair("RATE_FRAME", v); // Need to verify if it is RATE_FRAME or ANGVEL_FRAME in KVN
        }
        if let Some(v) = &self.interpolation_method {
            writer.write_pair("INTERPOLATION_METHOD", v);
        }
        if let Some(v) = self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AemData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(rename = "attitudeState")]
    pub attitude_states: Vec<AttitudeState>,
}

impl ToKvn for AemData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        for state in &self.attitude_states {
            state.write_kvn(writer);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AttitudeState {
    pub epoch: Epoch,
    pub values: Vec<f64>, // Generic storage for now, easier for parsed data lines
}

impl ToKvn for AttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // Manual formatting of data line
        let mut line = self.epoch.to_string();
        for val in &self.values {
            line.push_str(&format!(" {}", val));
        }
        writer.write_line(&line);
    }
}
