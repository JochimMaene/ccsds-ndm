// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use crate::kvn::parser::KvnResult;
use winnow::error::ParserError;

//----------------------------------------------------------------------
// Root ACM Structure
//----------------------------------------------------------------------

/// Attitude Comprehensive Message (ACM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename = "acm")]
pub struct Acm {
    pub header: AdmHeader,
    pub body: AcmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Acm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let acm = Self::from_kvn_str(kvn)?;
        acm.validate()?;
        Ok(acm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let acm: Self = crate::xml::from_str_with_context(xml, "ACM")?;
        acm.validate()?;
        Ok(acm)
    }
}

impl Acm {
    pub fn validate(&self) -> Result<()> {
        self.body.segment.validate(&self.header)
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AcmBody {
    #[serde(rename = "segment")]
    pub segment: Box<AcmSegment>,
}

impl ToKvn for AcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AcmSegment {
    pub metadata: AcmMetadata,
    pub data: AcmData,
}

impl AcmSegment {
    pub fn validate(&self, _header: &AdmHeader) -> Result<()> {
        self.data.validate(&self.metadata)
    }
}

impl ToKvn for AcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub object_name: String,
    pub international_designator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_designator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_names: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_poc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_org: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_poc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub odm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdm_msg_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constellation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    pub time_system: String,
    pub epoch_tzero: Epoch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taimutc_at_tzero: Option<TimeOffset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_taimutc: Option<TimeOffset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ut1mutc_at_tzero: Option<TimeOffset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eop_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interp_method_eop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub celestial_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acm_data_elements: Option<String>,
}

impl ToKvn for AcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.catalog_name { writer.write_pair("CATALOG_NAME", v); }
        if let Some(v) = &self.object_designator { writer.write_pair("OBJECT_DESIGNATOR", v); }
        if let Some(v) = &self.alternate_names { writer.write_pair("ALTERNATE_NAMES", v); }
        if let Some(v) = &self.originator_poc { writer.write_pair("ORIGINATOR_POC", v); }
        if let Some(v) = &self.originator_position { writer.write_pair("ORIGINATOR_POSITION", v); }
        if let Some(v) = &self.originator_phone { writer.write_pair("ORIGINATOR_PHONE", v); }
        if let Some(v) = &self.originator_email { writer.write_pair("ORIGINATOR_EMAIL", v); }
        if let Some(v) = &self.originator_address { writer.write_pair("ORIGINATOR_ADDRESS", v); }
        if let Some(v) = &self.tech_org { writer.write_pair("TECH_ORG", v); }
        if let Some(v) = &self.tech_poc { writer.write_pair("TECH_POC", v); }
        if let Some(v) = &self.tech_position { writer.write_pair("TECH_POSITION", v); }
        if let Some(v) = &self.tech_phone { writer.write_pair("TECH_PHONE", v); }
        if let Some(v) = &self.tech_email { writer.write_pair("TECH_EMAIL", v); }
        if let Some(v) = &self.tech_address { writer.write_pair("TECH_ADDRESS", v); }
        if let Some(v) = &self.previous_message_id { writer.write_pair("PREVIOUS_MESSAGE_ID", v); }
        if let Some(v) = &self.next_message_id { writer.write_pair("NEXT_MESSAGE_ID", v); }
        if let Some(v) = &self.adm_msg_link { writer.write_pair("ADM_MSG_LINK", v); }
        if let Some(v) = &self.odm_msg_link { writer.write_pair("ODM_MSG_LINK", v); }
        if let Some(v) = &self.cdm_msg_link { writer.write_pair("CDM_MSG_LINK", v); }
        if let Some(v) = &self.prm_msg_link { writer.write_pair("PRM_MSG_LINK", v); }
        if let Some(v) = &self.rdm_msg_link { writer.write_pair("RDM_MSG_LINK", v); }
        if let Some(v) = &self.tdm_msg_link { writer.write_pair("TDM_MSG_LINK", v); }
        if let Some(v) = &self.operator { writer.write_pair("OPERATOR", v); }
        if let Some(v) = &self.owner { writer.write_pair("OWNER", v); }
        if let Some(v) = &self.country { writer.write_pair("COUNTRY", v); }
        if let Some(v) = &self.constellation { writer.write_pair("CONSTELLATION", v); }
        if let Some(v) = &self.object_type { writer.write_pair("OBJECT_TYPE", v); }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.taimutc_at_tzero { writer.write_measure("TAIMUTC_AT_TZERO", &v.to_unit_value()); }
        if let Some(v) = &self.next_leap_epoch { writer.write_pair("NEXT_LEAP_EPOCH", v); }
        if let Some(v) = &self.next_leap_taimutc { writer.write_measure("NEXT_LEAP_TAIMUTC", &v.to_unit_value()); }
        if let Some(v) = &self.ut1mutc_at_tzero { writer.write_measure("UT1MUTC_AT_TZERO", &v.to_unit_value()); }
        if let Some(v) = &self.eop_source { writer.write_pair("EOP_SOURCE", v); }
        if let Some(v) = &self.interp_method_eop { writer.write_pair("INTERP_METHOD_EOP", v); }
        if let Some(v) = &self.celestial_source { writer.write_pair("CELESTIAL_SOURCE", v); }
        if let Some(v) = &self.acm_data_elements { writer.write_pair("ACM_DATA_ELEMENTS", v); }
        writer.write_section("META_STOP");
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmData {
    #[serde(rename = "att", default)]
    pub att: Vec<AcmAttitudeState>,
    #[serde(rename = "phys", default)]
    pub phys: Option<AcmPhysicalDescription>,
    #[serde(rename = "cov", default)]
    pub cov: Vec<AcmCovarianceMatrix>,
    #[serde(rename = "man", default)]
    pub man: Vec<AcmManeuverParameters>,
    #[serde(rename = "ad", default)]
    pub ad: Option<AcmAttitudeDetermination>,
    #[serde(rename = "user_defined", default)]
    pub user_defined: Option<UserDefined>,
}

impl AcmData {
    fn validate(&self, _metadata: &AcmMetadata) -> Result<()> {
        Ok(())
    }
}

impl ToKvn for AcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for att in &self.att { att.write_kvn(writer); }
        if let Some(phys) = &self.phys { phys.write_kvn(writer); }
        for cov in &self.cov { cov.write_kvn(writer); }
        for man in &self.man { man.write_kvn(writer); }
        if let Some(ad) = &self.ad { ad.write_kvn(writer); }
        if let Some(user) = &self.user_defined { 
            for p in &user.user_defined {
                writer.write_pair(&p.parameter, &p.value);
            }
        }
    }
}

//----------------------------------------------------------------------
// Attitude State Block (ATT)
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ref_frame_a: String,
    pub ref_frame_b: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attitude_dir: Option<String>,
    pub number_states: u32,
    pub att_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub euler_rot_seq: Option<RotSeq>,
    #[serde(rename = "attLine", default)]
    pub att_lines: Vec<AttLine>,
}

impl ToKvn for AcmAttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("ATT_START");
        writer.write_comments(&self.comment);
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        if let Some(v) = &self.attitude_dir { writer.write_pair("ATTITUDE_DIR", v); }
        writer.write_pair("NUMBER_STATES", self.number_states);
        writer.write_pair("ATT_TYPE", &self.att_type);
        if let Some(v) = &self.rate_type { writer.write_pair("RATE_TYPE", v); }
        if let Some(v) = &self.euler_rot_seq { writer.write_pair("EULER_ROT_SEQ", v); }
        for line in &self.att_lines {
            writer.write_line(&line.to_string());
        }
        writer.write_section("ATT_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AttLine {
    #[serde(rename = "$value")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for AttLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 { write!(f, " ")?; }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Physical Description Block (PHYS)
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmPhysicalDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_of_pressure: Option<Vector3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_rad_coeff: Option<NonNegativeDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_drag_coeff: Option<NonNegativeDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sa_area: Option<Area>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sa_rad_coeff: Option<NonNegativeDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sa_drag_coeff: Option<NonNegativeDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inertia_ref_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixx: Option<Moment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyy: Option<Moment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub izz: Option<Moment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixy: Option<Moment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixz: Option<Moment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyz: Option<Moment>,
}

impl ToKvn for AcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.wet_mass { writer.write_measure("WET_MASS", &v.to_unit_value()); }
        if let Some(v) = &self.dry_mass { writer.write_measure("DRY_MASS", &v.to_unit_value()); }
        if let Some(v) = &self.center_of_pressure {
            writer.write_pair("CP_X", v.elements[0]);
            writer.write_pair("CP_Y", v.elements[1]);
            writer.write_pair("CP_Z", v.elements[2]);
        }
        if let Some(v) = &self.bus_area { writer.write_measure("BUS_AREA", &v.to_unit_value()); }
        if let Some(v) = &self.bus_rad_coeff { writer.write_pair("BUS_RAD_COEFF", v.value); }
        if let Some(v) = &self.bus_drag_coeff { writer.write_pair("BUS_DRAG_COEFF", v.value); }
        if let Some(v) = &self.sa_area { writer.write_measure("SA_AREA", &v.to_unit_value()); }
        if let Some(v) = &self.sa_rad_coeff { writer.write_pair("SA_RAD_COEFF", v.value); }
        if let Some(v) = &self.sa_drag_coeff { writer.write_pair("SA_DRAG_COEFF", v.value); }
        if let Some(v) = &self.inertia_ref_frame { writer.write_pair("INERTIA_REF_FRAME", v); }
        if let Some(v) = &self.ixx { writer.write_measure("IXX", v); }
        if let Some(v) = &self.iyy { writer.write_measure("IYY", v); }
        if let Some(v) = &self.izz { writer.write_measure("IZZ", v); }
        if let Some(v) = &self.ixy { writer.write_measure("IXY", v); }
        if let Some(v) = &self.ixz { writer.write_measure("IXZ", v); }
        if let Some(v) = &self.iyz { writer.write_measure("IYZ", v); }
        writer.write_section("PHYS_STOP");
    }
}

//----------------------------------------------------------------------
// Covariance Block (COV)
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmCovarianceMatrix {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub cov_basis: String,
    pub cov_ref_frame: String,
    pub cov_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_confidence: Option<f64>,
    #[serde(rename = "covLine", default)]
    pub cov_lines: Vec<CovLine>,
}

impl ToKvn for AcmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COV_START");
        writer.write_comments(&self.comment);
        writer.write_pair("COV_BASIS", &self.cov_basis);
        writer.write_pair("COV_REF_FRAME", &self.cov_ref_frame);
        writer.write_pair("COV_TYPE", &self.cov_type);
        if let Some(v) = self.cov_confidence { writer.write_pair("COV_CONFIDENCE", v); }
        for line in &self.cov_lines {
            writer.write_line(&line.to_string());
        }
        writer.write_section("COV_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct CovLine {
    #[serde(rename = "$value")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for CovLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 { write!(f, " ")?; }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Maneuver Block (MAN)
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmManeuverParameters {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub man_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_prev_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_purpose: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_begin_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_end_time: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_duration: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actuator_used: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_momentum: Option<Vector3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_mom_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_torque: Option<Vector3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_tor_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_delta_mass: Option<Mass>,
}

impl ToKvn for AcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_ID", &self.man_id);
        if let Some(v) = &self.man_prev_id { writer.write_pair("MAN_PREV_ID", v); }
        if let Some(v) = &self.man_purpose { writer.write_pair("MAN_PURPOSE", v); }
        if let Some(v) = &self.man_begin_time { writer.write_pair("MAN_BEGIN_TIME", v); }
        if let Some(v) = &self.man_end_time { writer.write_pair("MAN_END_TIME", v); }
        if let Some(v) = &self.man_duration { writer.write_measure("MAN_DURATION", &v.to_unit_value()); }
        if let Some(v) = &self.actuator_used { writer.write_pair("ACTUATOR_USED", v); }
        if let Some(v) = &self.target_momentum {
            writer.write_pair("TARGET_MOM_X", v.elements[0]);
            writer.write_pair("TARGET_MOM_Y", v.elements[1]);
            writer.write_pair("TARGET_MOM_Z", v.elements[2]);
        }
        if let Some(v) = &self.target_mom_frame { writer.write_pair("TARGET_MOM_FRAME", v); }
        if let Some(v) = &self.man_torque {
            writer.write_pair("MAN_TOR_X", v.elements[0]);
            writer.write_pair("MAN_TOR_Y", v.elements[1]);
            writer.write_pair("MAN_TOR_Z", v.elements[2]);
        }
        if let Some(v) = &self.man_tor_frame { writer.write_pair("MAN_TOR_FRAME", v); }
        if let Some(v) = &self.man_delta_mass { writer.write_measure("MAN_DELTA_MASS", &v.to_unit_value()); }
        writer.write_section("MAN_STOP");
    }
}

//----------------------------------------------------------------------
// Attitude Determination Block (AD)
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeDetermination {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub ad_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_prev_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attitude_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attitude_states: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_epoch: Option<Epoch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_a: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_b: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attitude_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_states: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_u: Option<AngleRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_v: Option<AngleRate>,
    #[serde(rename = "sensor", default)]
    pub sensors: Vec<AcmSensor>,
}

impl ToKvn for AcmAttitudeDetermination {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("AD_START");
        writer.write_comments(&self.comment);
        writer.write_pair("AD_ID", &self.ad_id);
        if let Some(v) = &self.ad_prev_id { writer.write_pair("AD_PREV_ID", v); }
        if let Some(v) = &self.ad_method { writer.write_pair("AD_METHOD", v); }
        if let Some(v) = &self.attitude_source { writer.write_pair("ATTITUDE_SOURCE", v); }
        if let Some(v) = &self.attitude_states { writer.write_pair("ATTITUDE_STATES", v); }
        if let Some(v) = &self.ad_epoch { writer.write_pair("AD_EPOCH", v); }
        if let Some(v) = &self.ref_frame_a { writer.write_pair("REF_FRAME_A", v); }
        if let Some(v) = &self.ref_frame_b { writer.write_pair("REF_FRAME_B", v); }
        if let Some(v) = &self.attitude_type { writer.write_pair("ATTITUDE_TYPE", v); }
        if let Some(v) = &self.rate_states { writer.write_pair("RATE_STATES", v); }
        if let Some(v) = &self.sigma_u { writer.write_measure("SIGMA_U", v); }
        if let Some(v) = &self.sigma_v { writer.write_measure("SIGMA_V", v); }
        for sensor in &self.sensors { sensor.write_kvn(writer); }
        writer.write_section("AD_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmSensor {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    pub sensor_number: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_used: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_noise_stddev: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_frequency: Option<f64>,
}

impl ToKvn for AcmSensor {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("SENSOR_START");
        writer.write_comments(&self.comment);
        writer.write_pair("SENSOR_NUMBER", self.sensor_number);
        if let Some(v) = &self.sensor_used { writer.write_pair("SENSOR_USED", v); }
        if let Some(v) = self.sensor_noise_stddev { writer.write_pair("SENSOR_NOISE_STDDEV", v); }
        if let Some(v) = self.sensor_frequency { writer.write_pair("SENSOR_FREQUENCY", v); }
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
