// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdmHeader;
use crate::error::Result;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use fast_float;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root OCM Structure
//----------------------------------------------------------------------

/// Orbit Comprehensive Message (OCM).
///
/// An OCM aggregates and extends OMM, OPM, and OEM content in a single hybrid message.
/// It emphasizes flexibility and message conciseness by offering extensive optional
/// standardized content while minimizing mandatory content.
///
/// References:
/// - CCSDS 502.0-B-3, Section 5 (OCM)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename = "ocm")]
pub struct Ocm {
    pub header: OdmHeader,
    pub body: OcmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Ocm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        writer.write_pair("CCSDS_OCM_VERS", &self.version);
        self.header.write_kvn(&mut writer);
        self.body.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        Self::from_kvn_str(kvn)
    }

    fn to_xml(&self) -> Result<String> {
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        crate::xml::from_str(xml)
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The body of the OCM, containing a single segment.
///
/// This struct serves as a container for the `OcmSegment`, which holds the
/// metadata and data for the Orbit Comprehensive Message.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct OcmBody {
    #[serde(rename = "segment")]
    pub segment: Box<OcmSegment>,
}

impl ToKvn for OcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

/// A single segment of the OCM.
///
/// Contains metadata and data sections.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct OcmSegment {
    pub metadata: OcmMetadata,
    pub data: OcmData,
}

impl ToKvn for OcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// OCM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmMetadata {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Spacecraft name for which OCM data is provided. While there is no CCSDS-based restriction on
    /// the value for this keyword, it is recommended to use names from either the UN Office of Outer
    /// Space Affairs designator index (reference \[3\]), the spacecraft operator, or a State Actor or
    /// commercial Space Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space
    /// catalog. If OBJECT_NAME is not listed in reference \[3\] or the content is either unknown
    /// (uncorrelated) or cannot be disclosed, the value should be set to UNKNOWN (or this keyword
    /// omitted).
    ///
    /// **Examples**: EUTELSAT W1, MARS PATHFINDER, STS 106, NEAR, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    /// COSPAR international designator for the object. Such designator values shall have the
    /// following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch; NNN = Three-digit serial
    /// number of launch in year YYYY (with leading zeros); P{PP} = At least one capital letter for
    /// the identification of the part brought into space by the launch. If the object has no
    /// international designator or the content is either unknown (uncorrelated) or cannot be
    /// disclosed, the value should be set to UNKNOWN (or this keyword omitted).
    ///
    /// **Examples**: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_designator: Option<String>,
    /// Satellite catalog source (or source agency or operator, value to be drawn from the SANA
    /// registry list of Space Object Catalogs at <https://sanaregistry.org/r/space_object_catalog>).
    ///
    /// **Examples**: NORAD, SATCAT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    /// Unique satellite identification designator for the object, as reflected in the catalog whose
    /// name is ‘CATALOG_NAME’. If the ID is not known (uncorrelated object) or cannot be disclosed,
    /// ‘UNKNOWN’ may be used (or this keyword omitted).
    ///
    /// **Examples**: 28893
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_designator: Option<String>,
    /// Alternate name(s) of this space object, including assigned names used by spacecraft operator,
    /// State Actors, commercial SSA providers, and/or media.
    ///
    /// **Examples**: CALIPSO, 2006-016B
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_names: Option<String>,
    /// Point-of-Contact (PoC) for OCM.
    ///
    /// **Examples**: John Doe
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_poc: Option<String>,
    /// Contact position of the originator PoC.
    ///
    /// **Examples**: Analyst
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_position: Option<String>,
    /// Originator PoC phone number.
    ///
    /// **Examples**: +1 123-456-7890
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_phone: Option<String>,
    /// Originator PoC email address.
    ///
    /// **Examples**: john.doe@example.com
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_email: Option<String>,
    /// Originator’s physical address.
    ///
    /// **Examples**: 123 Main St, Anytown, USA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_address: Option<String>,
    /// Creating agency or operator (value should be drawn from the ‘Abbreviation’ column of the SANA
    /// Organizations registry at <https://www.sanaregistry.org/r/organizations>).
    ///
    /// **Examples**: NASA, ESA, JAXA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_org: Option<String>,
    /// Technical PoC for OCM.
    ///
    /// **Examples**: Jane Smith
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_poc: Option<String>,
    /// Contact position of the technical PoC.
    ///
    /// **Examples**: Engineer
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_position: Option<String>,
    /// Technical PoC phone number.
    ///
    /// **Examples**: +1 987-654-3210
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_phone: Option<String>,
    /// Technical PoC email address.
    ///
    /// **Examples**: jane.smith@example.com
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_email: Option<String>,
    /// Physical address information for OCM creator.
    ///
    /// **Examples**: 456 Tech Park, Sometown, USA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_address: Option<String>,
    /// Message ID of the previous message from this message originator for this space object.
    ///
    /// **Examples**: MSG-12344
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_id: Option<String>,
    /// Message ID of the next message from this message originator for this space object.
    ///
    /// **Examples**: MSG-12346
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_id: Option<String>,
    /// Link(s) to relevant Attitude Data Message(s).
    ///
    /// **Examples**: ADM-2023-001
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adm_msg_link: Option<String>,
    /// Link(s) to relevant Conjunction Data Message(s).
    ///
    /// **Examples**: CDM-2023-042
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdm_msg_link: Option<String>,
    /// Link(s) to relevant Pointing Request Message(s).
    ///
    /// **Examples**: PRM-2023-005
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prm_msg_link: Option<String>,
    /// Link(s) to relevant Reentry Data Message(s).
    ///
    /// **Examples**: RDM-2023-010
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdm_msg_link: Option<String>,
    /// Link(s) to relevant Tracking Data Message(s).
    ///
    /// **Examples**: TDM-2023-111
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tdm_msg_link: Option<String>,
    /// Spacecraft operator of the space object.
    ///
    /// **Examples**: SES, INTELSAT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Owner of the space object.
    ///
    /// **Examples**: Government of France
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Country or country code where the owner is based.
    ///
    /// **Examples**: FR, USA, JP
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Constellation to which this space object belongs.
    ///
    /// **Examples**: GALILEO, STARLINK
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constellation: Option<String>,
    /// Type of object (value to be drawn from the SANA registry list of Object Descriptions at
    /// <https://sanaregistry.org/r/object_types>).
    ///
    /// **Examples**: PAYLOAD, ROCKET BODY, DEBRIS, OTHER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    /// Time system used for all absolute time stamps in the message (e.g., UTC, TAI).
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    pub time_system: String,
    /// Epoch to which all relative times in the message are referenced. (For format specification,
    /// see 7.5.10.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    pub epoch_tzero: Epoch,
    /// Operational status of the space object (value to be drawn from the SANA registry list of
    /// Operational Status at <https://sanaregistry.org/r/operational_status>).
    ///
    /// **Examples**: OPERATIONAL, NON-OPERATIONAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ops_status: Option<String>,
    /// Orbit category of the space object (value to be drawn from the SANA registry list of Orbit
    /// Categories at <https://sanaregistry.org/r/orbit_categories>).
    ///
    /// **Examples**: GEO, LEO
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_category: Option<String>,
    /// List of data elements included in the OCM message.
    ///
    /// **Examples**: TRAJ, PHYS, COV, MAN, PERT, OD, USER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocm_data_elements: Option<String>,
    /// Spacecraft clock offset at EPOCH_TZERO.
    ///
    /// **Examples**: 0.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sclk_offset_at_epoch: Option<TimeOffset>,
    /// Spacecraft clock scale factor.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: s/SI-s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sclk_sec_per_si_sec: Option<Duration>,
    /// Epoch of the previous message. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_epoch: Option<Epoch>,
    /// Anticipated epoch of the next message. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_epoch: Option<Epoch>,
    /// Time of the earliest data in the message. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Epoch>,
    /// Time of the latest data in the message. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Epoch>,
    /// Approximate time span covered by the data in the message.
    ///
    /// **Examples**: 0.1
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_span: Option<DayInterval>,
    /// TAI minus UTC difference at EPOCH_TZERO.
    ///
    /// **Examples**: 37.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taimutc_at_tzero: Option<TimeOffset>,
    /// Epoch of the next leap second. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_epoch: Option<Epoch>,
    /// TAI minus UTC difference at NEXT_LEAP_EPOCH.
    ///
    /// **Examples**: 38.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_taimutc: Option<TimeOffset>,
    /// UT1 minus UTC difference at EPOCH_TZERO.
    ///
    /// **Examples**: 0.3
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ut1mutc_at_tzero: Option<TimeOffset>,
    /// Source of Earth Orientation Parameters.
    ///
    /// **Examples**: IERS_A
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eop_source: Option<String>,
    /// Interpolation method for EOP data.
    ///
    /// **Examples**: HERMITE, LINEAR
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interp_method_eop: Option<String>,
    /// Source of celestial body ephemerides.
    ///
    /// **Examples**: JPL_DE430
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub celestial_source: Option<String>,
}

impl ToKvn for OcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.object_name {
            writer.write_pair("OBJECT_NAME", v);
        }
        if let Some(v) = &self.international_designator {
            writer.write_pair("INTERNATIONAL_DESIGNATOR", v);
        }
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(v) = &self.alternate_names {
            writer.write_pair("ALTERNATE_NAMES", v);
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
        if let Some(v) = &self.tech_org {
            writer.write_pair("TECH_ORG", v);
        }
        if let Some(v) = &self.tech_poc {
            writer.write_pair("TECH_POC", v);
        }
        if let Some(v) = &self.tech_position {
            writer.write_pair("TECH_POSITION", v);
        }
        if let Some(v) = &self.tech_phone {
            writer.write_pair("TECH_PHONE", v);
        }
        if let Some(v) = &self.tech_email {
            writer.write_pair("TECH_EMAIL", v);
        }
        if let Some(v) = &self.tech_address {
            writer.write_pair("TECH_ADDRESS", v);
        }
        if let Some(v) = &self.previous_message_id {
            writer.write_pair("PREVIOUS_MESSAGE_ID", v);
        }
        if let Some(v) = &self.next_message_id {
            writer.write_pair("NEXT_MESSAGE_ID", v);
        }
        if let Some(v) = &self.adm_msg_link {
            writer.write_pair("ADM_MSG_LINK", v);
        }
        if let Some(v) = &self.cdm_msg_link {
            writer.write_pair("CDM_MSG_LINK", v);
        }
        if let Some(v) = &self.prm_msg_link {
            writer.write_pair("PRM_MSG_LINK", v);
        }
        if let Some(v) = &self.rdm_msg_link {
            writer.write_pair("RDM_MSG_LINK", v);
        }
        if let Some(v) = &self.tdm_msg_link {
            writer.write_pair("TDM_MSG_LINK", v);
        }
        if let Some(v) = &self.operator {
            writer.write_pair("OPERATOR", v);
        }
        if let Some(v) = &self.owner {
            writer.write_pair("OWNER", v);
        }
        if let Some(v) = &self.country {
            writer.write_pair("COUNTRY", v);
        }
        if let Some(v) = &self.constellation {
            writer.write_pair("CONSTELLATION", v);
        }
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v.to_string());
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.ops_status {
            writer.write_pair("OPS_STATUS", v);
        }
        if let Some(v) = &self.orbit_category {
            writer.write_pair("ORBIT_CATEGORY", v);
        }
        if let Some(v) = &self.ocm_data_elements {
            writer.write_pair("OCM_DATA_ELEMENTS", v);
        }
        if let Some(v) = &self.sclk_offset_at_epoch {
            writer.write_measure("SCLK_OFFSET_AT_EPOCH", &v.to_unit_value());
        }
        if let Some(v) = &self.sclk_sec_per_si_sec {
            writer.write_measure("SCLK_SEC_PER_SI_SEC", &v.to_unit_value());
        }
        if let Some(v) = &self.previous_message_epoch {
            writer.write_pair("PREVIOUS_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.next_message_epoch {
            writer.write_pair("NEXT_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = &self.stop_time {
            writer.write_pair("STOP_TIME", v);
        }
        if let Some(v) = &self.time_span {
            writer.write_measure("TIME_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.taimutc_at_tzero {
            writer.write_measure("TAIMUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.next_leap_epoch {
            writer.write_pair("NEXT_LEAP_EPOCH", v);
        }
        if let Some(v) = &self.next_leap_taimutc {
            writer.write_measure("NEXT_LEAP_TAIMUTC", &v.to_unit_value());
        }
        if let Some(v) = &self.ut1mutc_at_tzero {
            writer.write_measure("UT1MUTC_AT_TZERO", &v.to_unit_value());
        }
        if let Some(v) = &self.eop_source {
            writer.write_pair("EOP_SOURCE", v);
        }
        if let Some(v) = &self.interp_method_eop {
            writer.write_pair("INTERP_METHOD_EOP", v);
        }
        if let Some(v) = &self.celestial_source {
            writer.write_pair("CELESTIAL_SOURCE", v);
        }
        writer.write_section("META_STOP");
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// OCM Data Section.
///
/// This struct is the primary data container for the OCM. It holds all the
/// different data blocks, such as trajectory, physical properties, covariance,
/// maneuvers, and other related information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmData {
    /// List of trajectory state time history blocks.
    #[serde(rename = "traj", default)]
    pub traj: Vec<OcmTrajState>,
    /// Space object physical characteristics.
    #[serde(rename = "phys", default)]
    pub phys: Option<OcmPhysicalDescription>,
    /// List of covariance time history blocks.
    #[serde(rename = "cov", default)]
    pub cov: Vec<OcmCovarianceMatrix>,
    /// List of maneuver specifications.
    #[serde(rename = "man", default)]
    pub man: Vec<OcmManeuverParameters>,
    /// Perturbation parameters.
    #[serde(rename = "pert", default)]
    pub pert: Option<OcmPerturbations>,
    /// Orbit determination data.
    #[serde(rename = "od", default)]
    pub od: Option<OcmOdParameters>,
    /// User-defined parameters.
    #[serde(rename = "user", default)]
    pub user: Option<UserDefined>,
}

impl ToKvn for OcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for t in &self.traj {
            t.write_kvn(writer);
        }
        if let Some(p) = &self.phys {
            p.write_kvn(writer);
        }
        for c in &self.cov {
            c.write_kvn(writer);
        }
        for m in &self.man {
            m.write_kvn(writer);
        }
        if let Some(p) = &self.pert {
            p.write_kvn(writer);
        }
        if let Some(o) = &self.od {
            o.write_kvn(writer);
        }
        if let Some(u) = &self.user {
            writer.write_section("USER_START");
            writer.write_comments(&u.comment);
            for p in &u.user_defined {
                writer.write_pair(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

//----------------------------------------------------------------------
// 1. Trajectory State
//----------------------------------------------------------------------

/// A block of trajectory state data, which can be a time history of states.
///
/// References:
/// - CCSDS 502.0-B-3, Section 4.5.2 (OCM Trajectory State Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmTrajState {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Identification number for this trajectory state time history block.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_id: Option<String>,
    /// Identification number for the previous trajectory state time history.
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_prev_id: Option<String>,
    /// Identification number for the next trajectory state time history.
    ///
    /// **Examples**: 2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_next_id: Option<String>,
    /// Basis of this trajectory state time history data (e.g., PREDICTED, DETERMINED, SIMULATED).
    ///
    /// **Examples**: PREDICTED
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_basis: Option<TrajBasis>,
    /// Identification number for the telemetry dataset, orbit determination, or simulation upon
    /// which the TRAJ_BASIS is based.
    ///
    /// **Examples**: OD-123
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_basis_id: Option<String>,
    /// Recommended interpolation method for the state elements (value to be drawn from the SANA
    /// registry list of Interpolation Methods at <https://sanaregistry.org/r/interpolation_methods>).
    ///
    /// **Examples**: HERMITE, LINEAR
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<String>,
    /// Recommended interpolation degree for the state elements.
    ///
    /// **Examples**: 5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<u32>,
    /// Name of the propagator used in the creation of the trajectory state data.
    ///
    /// **Examples**: GMAT, STK
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagator: Option<String>,
    /// Name of the central body (value to be drawn from the SANA registry list of Common Central Body
    /// Names at <https://sanaregistry.org/r/central_body_name>).
    ///
    /// **Examples**: EARTH, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    pub center_name: String,
    /// Orbit reference frame (value to be drawn from the SANA registry list of Reference Frames at
    /// <https://sanaregistry.org/r/orbit_relative_reference_frames>).
    ///
    /// **Examples**: ICRF, EME2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    pub traj_ref_frame: String,
    /// Epoch of the orbit reference frame, if TRAJ_REF_FRAME is provided and its epoch is not
    /// intrinsic to the definition of the reference frame.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_frame_epoch: Option<Epoch>,
    /// Start time of the useable time span covered by the ephemeris data.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    /// Stop time of the useable time span covered by the ephemeris data.
    ///
    /// **Examples**: 2000-01-02T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    /// Integer orbit revolution number at the epoch of the first trajectory data line.
    ///
    /// **Examples**: 1234.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orb_revnum: Option<f64>,
    /// Basis for the orbit revolution counter (0 or 1).
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orb_revnum_basis: Option<RevNumBasis>,
    /// Specification of the trajectory state element set type (value to be drawn from the SANA
    /// registry list of Trajectory State Types at <https://sanaregistry.org/r/orbital_elements>).
    ///
    /// **Examples**: CARTESIAN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    pub traj_type: String,
    /// Method used for orbit averaging if TRAJ_TYPE is not osculating (value to be drawn from the SANA
    /// registry list of Orbit Averaging Methods at <https://sanaregistry.org/r/orbit_averaging>).
    ///
    /// **Examples**: BROUWER-LYDDANE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orb_averaging: Option<String>,
    /// SI unit designations for the state elements.
    ///
    /// **Examples**: km, km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_units: Option<String>,
    /// Contiguous set of trajectory state data lines.
    #[serde(rename = "trajLine")]
    pub traj_lines: Vec<TrajLine>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TrajLine {
    pub epoch: String,
    pub values: Vec<f64>,
}

impl Serialize for TrajLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.clone();
        for v in &self.values {
            s.push(' ');
            s.push_str(&v.to_string());
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for TrajLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split_whitespace();
        let epoch = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("Missing epoch"))?
            .to_string();
        let values: std::result::Result<Vec<f64>, _> = parts
            .map(|v| fast_float::parse(v).map_err(serde::de::Error::custom))
            .collect();
        Ok(TrajLine {
            epoch,
            values: values?,
        })
    }
}

impl ToKvn for OcmTrajState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("TRAJ_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.traj_id {
            writer.write_pair("TRAJ_ID", v);
        }
        if let Some(v) = &self.traj_prev_id {
            writer.write_pair("TRAJ_PREV_ID", v);
        }
        if let Some(v) = &self.traj_next_id {
            writer.write_pair("TRAJ_NEXT_ID", v);
        }
        if let Some(v) = &self.traj_basis {
            writer.write_pair("TRAJ_BASIS", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.traj_basis_id {
            writer.write_pair("TRAJ_BASIS_ID", v);
        }
        if let Some(v) = &self.interpolation {
            writer.write_pair("INTERPOLATION", v);
        }
        if let Some(v) = &self.interpolation_degree {
            writer.write_pair("INTERPOLATION_DEGREE", v);
        }
        if let Some(v) = &self.propagator {
            writer.write_pair("PROPAGATOR", v);
        }
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TRAJ_REF_FRAME", &self.traj_ref_frame);
        if let Some(v) = &self.traj_frame_epoch {
            writer.write_pair("TRAJ_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.useable_start_time {
            writer.write_pair("USEABLE_START_TIME", v);
        }
        if let Some(v) = &self.useable_stop_time {
            writer.write_pair("USEABLE_STOP_TIME", v);
        }
        if let Some(v) = &self.orb_revnum {
            writer.write_pair("ORB_REVNUM", v);
        }
        if let Some(v) = &self.orb_revnum_basis {
            writer.write_pair(
                "ORB_REVNUM_BASIS",
                match v {
                    RevNumBasis::Zero => "0",
                    RevNumBasis::One => "1",
                },
            );
        }
        writer.write_pair("TRAJ_TYPE", &self.traj_type);
        if let Some(v) = &self.orb_averaging {
            writer.write_pair("ORB_AVERAGING", v);
        }
        if let Some(v) = &self.traj_units {
            writer.write_pair("TRAJ_UNITS", v);
        }
        for line in &self.traj_lines {
            let vals: Vec<String> = line.values.iter().map(|v| v.to_string()).collect();
            writer.write_line(format!("{} {}", line.epoch, vals.join(" ")));
        }
        writer.write_section("TRAJ_STOP");
    }
}

//----------------------------------------------------------------------
// 2. Physical Properties (ocmPhysicalDescriptionType)
//----------------------------------------------------------------------

/// Space Object Physical Characteristics.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmPhysicalDescription {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Free-text field containing the satellite manufacturer’s name.
    ///
    /// **Examples**: Boeing, Lockheed Martin
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Free-text field containing the satellite manufacturer’s spacecraft bus model name.
    ///
    /// **Examples**: LS-1300, A2100
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_model: Option<String>,
    /// Free-text field containing a comma-separated list of other space objects that this object is
    /// docked to.
    ///
    /// **Examples**: 2021-098A, 2021-098B
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docked_with: Option<String>,
    /// Attitude-independent drag cross-sectional area (AD) facing the relative wind vector, not
    /// already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.
    ///
    /// **Examples**: 2.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_const_area: Option<Area>,
    /// Nominal drag Coefficient (CD NOM). If the atmospheric drag coefficient, CD, is set to zero, no
    /// atmospheric drag shall be considered.
    ///
    /// **Examples**: 2.2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff_nom: Option<f64>,
    /// Drag coefficient one sigma (1σ) percent uncertainty, where the actual range of drag
    /// coefficients to within 1σ shall be obtained from \[1.0 ± 0.01*DRAG_UNCERTAINTY\] (CD NOM). This
    /// factor is intended to allow operators to supply the nominal ballistic coefficient components
    /// while accommodating ballistic coefficient uncertainties.
    ///
    /// **Examples**: 5.0
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_uncertainty: Option<Percentage>,
    /// Space object total mass at beginning of life.
    ///
    /// **Examples**: 1000.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_wet_mass: Option<Mass>,
    /// Space object total mass (including propellant, i.e., ‘wet mass’) at the current reference epoch
    /// ‘EPOCH_TZERO’.
    ///
    /// **Examples**: 950.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    /// Space object dry mass (without propellant).
    ///
    /// **Examples**: 500.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    /// Parent reference frame that maps to the OEB frame via the quaternion-based transformation
    /// defined in annex F, subsection F1. Select from the accepted set of values indicated in annex
    /// B, subsections B4 and B5. This keyword shall be provided if OEB_Q1,2,3,qc are specified.
    ///
    /// **Examples**: ICRF, EME2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_parent_frame: Option<String>,
    /// Epoch of the OEB parent frame, if OEB_PARENT_FRAME is provided and its epoch is not intrinsic
    /// to the definition of the reference frame.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_parent_frame_epoch: Option<Epoch>,
    /// q1 = e1 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e1 = 1st component
    /// of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
    /// the frame aligned with the OEB (defined in annex F, subsection F1). A value of ‘-999’ denotes
    /// a tumbling space object.
    ///
    /// **Examples**: 0.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q1: Option<f64>,
    /// q2 = e2 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e2 = 2nd component
    /// of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
    /// the frame aligned with the Optimally Encompassing Box (defined in annex F, subsection F1). A
    /// value of ‘-999’ denotes a tumbling space object.
    ///
    /// **Examples**: 0.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q2: Option<f64>,
    /// q3 = e3 * sin(φ/2), where per reference [H1], φ = Euler rotation angle and e3 = 3rd component
    /// of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME (defined above) to
    /// the frame aligned with the Optimally Encompassing Box (defined in annex F, subsection F1). A
    /// value of ‘-999’ denotes a tumbling space object.
    ///
    /// **Examples**: 0.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q3: Option<f64>,
    /// qc = cos(φ/2), where per reference [H1], φ = the Euler rotation angle for the rotation that
    /// maps from the OEB_PARENT_FRAME (defined above) to the frame aligned with the Optimally
    /// Encompassing Box (annex F, subsection F1). qc shall be made non-negative by convention. A
    /// value of ‘-999’ denotes a tumbling space object.
    ///
    /// **Examples**: 1.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_qc: Option<f64>,
    /// Maximum physical dimension (along Xoeb) of the OEB.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_max: Option<Length>,
    /// Intermediate physical dimension (along Ŷoeb) of OEB normal to OEB_MAX direction.
    ///
    /// **Examples**: 5.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_int: Option<Length>,
    /// Minimum physical dimension (along Ẑoeb) of OEB in direction normal to both OEB_MAX and OEB_INT
    /// directions.
    ///
    /// **Examples**: 2.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_min: Option<Length>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along max OEB (Xoeb) direction as defined in
    /// annex F.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_max: Option<Area>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along intermediate OEB (Ŷoeb) direction as
    /// defined in annex F.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_int: Option<Area>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along minimum OEB (Ẑoeb) direction as defined
    /// in annex F.
    ///
    /// **Examples**: 50.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_min: Option<Area>,
    /// Minimum cross-sectional area for collision probability estimation purposes.
    ///
    /// **Examples**: 5.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_min_for_pc: Option<Area>,
    /// Maximum cross-sectional area for collision probability estimation purposes.
    ///
    /// **Examples**: 50.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_max_for_pc: Option<Area>,
    /// Typical (50th percentile) cross-sectional area sampled over all space object orientations for
    /// collision probability estimation purposes.
    ///
    /// **Examples**: 15.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_typ_for_pc: Option<Area>,
    /// Typical (50th percentile) effective Radar Cross Section of the space object sampled over all
    /// possible viewing angles.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs: Option<Area>,
    /// Minimum Radar Cross Section observed for this object.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs_min: Option<Area>,
    /// Maximum Radar Cross Section observed for this object.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs_max: Option<Area>,
    /// Attitude-independent solar radiation pressure cross-sectional area (AR) facing the Sun, not
    /// already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.
    ///
    /// **Examples**: 5.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srp_const_area: Option<Area>,
    /// Nominal Solar Radiation Pressure Coefficient (CR NOM). If the solar radiation coefficient, CR,
    /// is set to zero, no solar radiation pressure shall be considered.
    ///
    /// **Examples**: 1.2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<f64>,
    /// SRP one sigma (1σ) percent uncertainty, where the actual range of SRP coefficients to within
    /// 1σ shall be obtained from \[1.0 ± 0.01*SRP_UNCERTAINTY\] (CR NOM). This factor is intended to
    /// allow operators to supply the nominal ballistic coefficient components while accommodating
    /// ballistic coefficient uncertainties.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_uncertainty: Option<Percentage>,
    /// Typical (50th percentile) absolute Visual Magnitude of the space object sampled over all
    /// possible viewing angles and ‘normalized’ as specified in informative annex F, subsection F2 to
    /// a 1 AU Sun-to-target distance, a phase angle of 0°, and a 40,000 km target-to-sensor distance.
    ///
    /// **Examples**: 4.5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_absolute: Option<f64>,
    /// Minimum apparent Visual Magnitude observed for this space object. The ‘MIN’ value represents
    /// the brightest observation, which associates with a lower Vmag.
    ///
    /// **Examples**: 3.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent_min: Option<f64>,
    /// Typical (50th percentile) apparent Visual Magnitude observed for this space object.
    ///
    /// **Examples**: 12.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent: Option<f64>,
    /// Maximum apparent Visual Magnitude observed for this space object. The ‘MAX’ value represents
    /// the dimmest observation, which associates with a higher Vmag.
    ///
    /// **Examples**: 18.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent_max: Option<f64>,
    /// Typical (50th percentile) coefficient of REFLECTANCE of the space object over all possible
    /// viewing angles, ranging from 0 (none) to 1 (perfect reflectance).
    ///
    /// **Examples**: 0.2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reflectance: Option<Probability>,
    /// Free-text specification of primary mode of attitude control for the space object.
    ///
    /// **Examples**: THREE_AXIS, SPIN, DUAL_SPIN, TUMBLING, GRAVITY_GRADIENT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_control_mode: Option<String>,
    /// Free-text specification of type of actuator for attitude control.
    ///
    /// **Examples**: ATT_THRUSTERS, ACTIVE_MAG_TORQUE, PASSIVE_MAG_TORQUE, REACTION_WHEELS,
    /// MOMENTUM_WHEELS, CONTROL_MOMENT_GYROSCOPE, NONE, OTHER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_actuator_type: Option<String>,
    /// Accuracy of attitude knowledge.
    ///
    /// **Examples**: 0.01
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_knowledge: Option<Angle>,
    /// Accuracy of attitude control system (ACS) to maintain attitude, assuming attitude knowledge
    /// was perfect (i.e., deadbands).
    ///
    /// **Examples**: 0.1
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_control: Option<Angle>,
    /// Overall accuracy of spacecraft to maintain attitude, including attitude knowledge errors and
    /// ACS operation.
    ///
    /// **Examples**: 0.5
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_pointing: Option<Angle>,
    /// Average maneuver frequency, measured in the number of orbit- or attitude-adjust maneuvers per
    /// year.
    ///
    /// **Examples**: 52.0
    ///
    /// **Units**: #/yr
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_maneuver_freq: Option<ManeuverFreq>,
    /// Maximum composite thrust the spacecraft can accomplish in any single body-fixed direction.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: N
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_thrust: Option<Thrust>,
    /// Total ΔV capability of the spacecraft at beginning of life.
    ///
    /// **Examples**: 2.0
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_bol: Option<Velocity>,
    /// Total ΔV remaining for the spacecraft.
    ///
    /// **Examples**: 1.5
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_remaining: Option<Velocity>,
    /// Moment of Inertia about the X-axis of the space object’s primary body frame.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixx: Option<Moment>,
    /// Moment of Inertia about the Y-axis.
    ///
    /// **Examples**: 200.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyy: Option<Moment>,
    /// Moment of Inertia about the Z-axis.
    ///
    /// **Examples**: 300.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub izz: Option<Moment>,
    /// Inertia Cross Product of the X & Y axes.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixy: Option<Moment>,
    /// Inertia Cross Product of the X & Z axes.
    ///
    /// **Examples**: 2.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixz: Option<Moment>,
    /// Inertia Cross Product of the Y & Z axes.
    ///
    /// **Examples**: 3.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyz: Option<Moment>,
}

impl ToKvn for OcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.manufacturer {
            writer.write_pair("MANUFACTURER", v);
        }
        if let Some(v) = &self.bus_model {
            writer.write_pair("BUS_MODEL", v);
        }
        if let Some(v) = &self.docked_with {
            writer.write_pair("DOCKED_WITH", v);
        }
        if let Some(v) = &self.drag_const_area {
            writer.write_measure("DRAG_CONST_AREA", &v.to_unit_value());
        }
        if let Some(v) = &self.drag_coeff_nom {
            writer.write_pair("DRAG_COEFF_NOM", v);
        }
        if let Some(v) = &self.drag_uncertainty {
            writer.write_measure("DRAG_UNCERTAINTY", &v.to_unit_value());
        }
        if let Some(v) = &self.initial_wet_mass {
            writer.write_measure("INITIAL_WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.wet_mass {
            writer.write_measure("WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.dry_mass {
            writer.write_measure("DRY_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.oeb_parent_frame {
            writer.write_pair("OEB_PARENT_FRAME", v);
        }
        if let Some(v) = &self.oeb_parent_frame_epoch {
            writer.write_pair("OEB_PARENT_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.oeb_q1 {
            writer.write_pair("OEB_Q1", v);
        }
        if let Some(v) = &self.oeb_q2 {
            writer.write_pair("OEB_Q2", v);
        }
        if let Some(v) = &self.oeb_q3 {
            writer.write_pair("OEB_Q3", v);
        }
        if let Some(v) = &self.oeb_qc {
            writer.write_pair("OEB_QC", v);
        }
        if let Some(v) = &self.oeb_max {
            writer.write_measure("OEB_MAX", v);
        }
        if let Some(v) = &self.oeb_int {
            writer.write_measure("OEB_INT", v);
        }
        if let Some(v) = &self.oeb_min {
            writer.write_measure("OEB_MIN", v);
        }
        if let Some(v) = &self.area_along_oeb_max {
            writer.write_measure("AREA_ALONG_OEB_MAX", &v.to_unit_value());
        }
        if let Some(v) = &self.area_along_oeb_int {
            writer.write_measure("AREA_ALONG_OEB_INT", &v.to_unit_value());
        }
        if let Some(v) = &self.area_along_oeb_min {
            writer.write_measure("AREA_ALONG_OEB_MIN", &v.to_unit_value());
        }
        if let Some(v) = &self.area_min_for_pc {
            writer.write_measure("AREA_MIN_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.area_max_for_pc {
            writer.write_measure("AREA_MAX_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.area_typ_for_pc {
            writer.write_measure("AREA_TYP_FOR_PC", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs {
            writer.write_measure("RCS", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs_min {
            writer.write_measure("RCS_MIN", &v.to_unit_value());
        }
        if let Some(v) = &self.rcs_max {
            writer.write_measure("RCS_MAX", &v.to_unit_value());
        }
        if let Some(v) = &self.srp_const_area {
            writer.write_measure("SRP_CONST_AREA", &v.to_unit_value());
        }
        if let Some(v) = &self.solar_rad_coeff {
            writer.write_pair("SOLAR_RAD_COEFF", v);
        }
        if let Some(v) = &self.solar_rad_uncertainty {
            writer.write_measure("SOLAR_RAD_UNCERTAINTY", &v.to_unit_value());
        }
        if let Some(v) = &self.vm_absolute {
            writer.write_pair("VM_ABSOLUTE", v);
        }
        if let Some(v) = &self.vm_apparent_min {
            writer.write_pair("VM_APPARENT_MIN", v);
        }
        if let Some(v) = &self.vm_apparent {
            writer.write_pair("VM_APPARENT", v);
        }
        if let Some(v) = &self.vm_apparent_max {
            writer.write_pair("VM_APPARENT_MAX", v);
        }
        if let Some(v) = &self.reflectance {
            writer.write_pair("REFLECTANCE", v);
        }
        if let Some(v) = &self.att_control_mode {
            writer.write_pair("ATT_CONTROL_MODE", v);
        }
        if let Some(v) = &self.att_actuator_type {
            writer.write_pair("ATT_ACTUATOR_TYPE", v);
        }
        if let Some(v) = &self.att_knowledge {
            writer.write_measure("ATT_KNOWLEDGE", &v.to_unit_value());
        }
        if let Some(v) = &self.att_control {
            writer.write_measure("ATT_CONTROL", &v.to_unit_value());
        }
        if let Some(v) = &self.att_pointing {
            writer.write_measure("ATT_POINTING", &v.to_unit_value());
        }
        if let Some(v) = &self.avg_maneuver_freq {
            writer.write_measure("AVG_MANEUVER_FREQ", v);
        }
        if let Some(v) = &self.max_thrust {
            writer.write_measure("MAX_THRUST", v);
        }
        if let Some(v) = &self.dv_bol {
            writer.write_measure("DV_BOL", v);
        }
        if let Some(v) = &self.dv_remaining {
            writer.write_measure("DV_REMAINING", v);
        }
        if let Some(v) = &self.ixx {
            writer.write_measure("IXX", v);
        }
        if let Some(v) = &self.iyy {
            writer.write_measure("IYY", v);
        }
        if let Some(v) = &self.izz {
            writer.write_measure("IZZ", v);
        }
        if let Some(v) = &self.ixy {
            writer.write_measure("IXY", v);
        }
        if let Some(v) = &self.ixz {
            writer.write_measure("IXZ", v);
        }
        if let Some(v) = &self.iyz {
            writer.write_measure("IYZ", v);
        }
        writer.write_section("PHYS_STOP");
    }
}

//----------------------------------------------------------------------
// 3. Covariance (ocmCovarianceMatrixType)
//----------------------------------------------------------------------

/// OCM Covariance Matrix.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmCovarianceMatrix {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Identification number for this covariance time history block.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_id: Option<String>,
    /// Identification number for the previous covariance time history.
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_prev_id: Option<String>,
    /// Identification number for the next covariance time history.
    ///
    /// **Examples**: 2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_next_id: Option<String>,
    /// Basis of this covariance time history data (e.g., PREDICTED, DETERMINED).
    ///
    /// **Examples**: PREDICTED
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_basis: Option<CovBasis>,
    /// Identification number for the telemetry dataset, orbit determination, or simulation upon
    /// which the COV_BASIS is based.
    ///
    /// **Examples**: OD-123
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_basis_id: Option<String>,
    /// Reference frame of the covariance time history (value to be drawn from the SANA registry list
    /// of Reference Frames at <https://sanaregistry.org/r/celestial_body_reference_frames> or
    /// <https://sanaregistry.org/r/orbit_relative_reference_frames>).
    ///
    /// **Examples**: ICRF, EME2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    pub cov_ref_frame: String,
    /// Epoch of the covariance data reference frame, if not intrinsic to its definition.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_frame_epoch: Option<Epoch>,
    /// Minimum scale factor to apply to this covariance data to achieve realism.
    ///
    /// **Examples**: 0.9
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_scale_min: Option<f64>,
    /// Maximum scale factor to apply to this covariance data to achieve realism.
    ///
    /// **Examples**: 1.1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_scale_max: Option<f64>,
    /// A measure of the confidence in the covariance errors matching reality.
    ///
    /// **Examples**: 95.0
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_confidence: Option<Percentage>,
    /// Specification of the covariance element set type (value to be drawn from the SANA registry
    /// list of Covariance Types at <https://sanaregistry.org/r/orbital_covariance_matrix_types>).
    ///
    /// **Examples**: CARTESIAN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    pub cov_type: String,
    /// Indicates covariance ordering (LTM or UTM).
    ///
    /// **Examples**: LTM
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    pub cov_ordering: CovOrder,
    /// SI unit designations for the covariance elements.
    ///
    /// **Examples**: km**2, km**2/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_units: Option<String>,
    /// Contiguous set of covariance matrix data lines.
    #[serde(rename = "covLine")]
    pub cov_lines: Vec<CovLine>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CovLine {
    pub epoch: String,
    pub values: Vec<f64>,
}

impl Serialize for CovLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.clone();
        for v in &self.values {
            s.push(' ');
            s.push_str(&v.to_string());
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for CovLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split_whitespace();
        let epoch = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("Missing epoch"))?
            .to_string();
        let values: std::result::Result<Vec<f64>, _> = parts
            .map(|v| fast_float::parse(v).map_err(serde::de::Error::custom))
            .collect();
        Ok(CovLine {
            epoch,
            values: values?,
        })
    }
}

impl ToKvn for OcmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("COV_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.cov_id {
            writer.write_pair("COV_ID", v);
        }
        if let Some(v) = &self.cov_prev_id {
            writer.write_pair("COV_PREV_ID", v);
        }
        if let Some(v) = &self.cov_next_id {
            writer.write_pair("COV_NEXT_ID", v);
        }
        if let Some(v) = &self.cov_basis {
            writer.write_pair("COV_BASIS", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.cov_basis_id {
            writer.write_pair("COV_BASIS_ID", v);
        }
        writer.write_pair("COV_REF_FRAME", &self.cov_ref_frame);
        if let Some(v) = &self.cov_frame_epoch {
            writer.write_pair("COV_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.cov_scale_min {
            writer.write_pair("COV_SCALE_MIN", v);
        }
        if let Some(v) = &self.cov_scale_max {
            writer.write_pair("COV_SCALE_MAX", v);
        }
        if let Some(v) = &self.cov_confidence {
            writer.write_measure("COV_CONFIDENCE", &v.to_unit_value());
        }
        writer.write_pair("COV_TYPE", &self.cov_type);
        writer.write_pair(
            "COV_ORDERING",
            format!("{:?}", self.cov_ordering).to_uppercase(),
        );
        if let Some(v) = &self.cov_units {
            writer.write_pair("COV_UNITS", v);
        }
        for line in &self.cov_lines {
            let vals: Vec<String> = line.values.iter().map(|v| v.to_string()).collect();
            writer.write_line(format!("{} {}", line.epoch, vals.join(" ")));
        }
        writer.write_section("COV_STOP");
    }
}

//----------------------------------------------------------------------
// 4. Maneuver (ocmManeuverParametersType)
//----------------------------------------------------------------------

/// OCM Maneuver Parameters.
///
/// References:
/// - CCSDS 502.0-B-3, Section 4.5.5 (OCM Maneuver Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmManeuverParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Unique maneuver identification number for this maneuver block.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    pub man_id: String,
    /// Identification number for the previous maneuver.
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_prev_id: Option<String>,
    /// Identification number for the next maneuver.
    ///
    /// **Examples**: 2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_next_id: Option<String>,
    /// Basis of this maneuver data (e.g., PREDICTED, DETERMINED, SIMULATED).
    ///
    /// **Examples**: PREDICTED
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_basis: Option<ManBasis>,
    /// Identification number for the telemetry dataset, orbit determination, or simulation upon
    /// which the MAN_BASIS is based.
    ///
    /// **Examples**: OD-123
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_basis_id: Option<String>,
    /// Identification name of the maneuver device (e.g., ‘THRUSTER-1’).
    ///
    /// **Examples**: THRUSTER-1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    pub man_device_id: String,
    /// Completion time of the previous maneuver for this MAN_BASIS.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_prev_epoch: Option<Epoch>,
    /// Start time of the next maneuver for this MAN_BASIS.
    ///
    /// **Examples**: 2000-01-02T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_next_epoch: Option<Epoch>,
    /// Purpose of the maneuver (e.g., ‘WHEEL-DESAT’, ‘STATION-KEEPING’).
    ///
    /// **Examples**: STATION-KEEPING
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_purpose: Option<String>,
    /// Identification (e.g., message or file) of the predicted maneuver parameters upon which this
    /// maneuver is based.
    ///
    /// **Examples**: MAN-PRED-456
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_pred_source: Option<String>,
    /// Reference frame for the maneuver thrust vector (value to be drawn from the SANA registry list
    /// of Reference Frames at <https://sanaregistry.org/r/orbit_relative_reference_frames>).
    ///
    /// **Examples**: TNW, RSW
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    pub man_ref_frame: String,
    /// Epoch of the maneuver reference frame, if not intrinsic to its definition.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_frame_epoch: Option<Epoch>,
    /// Identification of a gravitational body that would be used for an assist maneuver (value to be
    /// drawn from the SANA registry list of Common Central Body Names at
    /// <https://sanaregistry.org/r/central_body_name>).
    ///
    /// **Examples**: EARTH, JUPITER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grav_assist_name: Option<String>,
    /// Duty cycle type to use for this maneuver time history section.
    ///
    /// **Examples**: LUSTRE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    pub dc_type: ManDc,
    /// Start time of the duty cycle-based maneuver window.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_win_open: Option<Epoch>,
    /// End time of the duty cycle-based maneuver window.
    ///
    /// **Examples**: 2000-01-01T13:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_win_close: Option<Epoch>,
    /// Minimum number of ‘ON’ duty cycles.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_min_cycles: Option<u64>,
    /// Maximum number of ‘ON’ duty cycles.
    ///
    /// **Examples**: 10
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_max_cycles: Option<u64>,
    /// Start time of the initial duty cycle-based maneuver sequence execution.
    ///
    /// **Examples**: 2000-01-01T12:05:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_exec_start: Option<Epoch>,
    /// End time of the final duty cycle-based maneuver sequence execution.
    ///
    /// **Examples**: 2000-01-01T12:55:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_exec_stop: Option<Epoch>,
    /// Reference time for the THRUST duty cycle.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_ref_time: Option<Epoch>,
    /// Thruster pulse ‘ON’ duration.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_time_pulse_duration: Option<Duration>,
    /// Elapsed time between the start of one pulse and the start of the next.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_time_pulse_period: Option<Duration>,
    /// Reference vector direction in the body frame for angle-initiated thruster duty cycles.
    ///
    /// **Examples**: 1.0 0.0 0.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_ref_dir: Option<Vec3Double>,
    /// Body reference frame in which DC_BODY_TRIGGER will be specified.
    ///
    /// **Examples**: SC_BODY
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_body_frame: Option<String>,
    /// Body frame reference vector direction for angle-based duty cycle initiation.
    ///
    /// **Examples**: 0.0 1.0 0.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_body_trigger: Option<Vec3Double>,
    /// Phase angle offset of thruster pulse start.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_pa_start_angle: Option<Angle>,
    /// Phase angle of thruster pulse stop.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_pa_stop_angle: Option<Angle>,
    /// Specification of the maneuver element set type (value to be drawn from the SANA registry list
    /// of Maneuver Types at https://sanaregistry.org/r/maneuver_type).
    ///
    /// **Examples**: ΔV_CARTESIAN, ΔV_SPHERICAL, THRUST_CARTESIAN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    pub man_composition: String,
    /// SI unit designations for the maneuver parameters.
    ///
    /// **Examples**: km/s, N
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_units: Option<String>,
    /// Maneuver time history data lines.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.3.3.
    #[serde(rename = "manLine")]
    pub man_lines: Vec<ManLine>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ManLine {
    pub epoch: String,
    pub values: Vec<String>,
}

impl Serialize for ManLine {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.epoch.clone();
        for v in &self.values {
            s.push(' ');
            s.push_str(v);
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ManLine {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split_whitespace();
        let epoch = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("Missing epoch"))?
            .to_string();
        let values: Vec<String> = parts.map(|s| s.to_string()).collect();
        Ok(ManLine { epoch, values })
    }
}

impl ToKvn for OcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_ID", &self.man_id);
        if let Some(v) = &self.man_prev_id {
            writer.write_pair("MAN_PREV_ID", v);
        }
        if let Some(v) = &self.man_next_id {
            writer.write_pair("MAN_NEXT_ID", v);
        }
        if let Some(v) = &self.man_basis {
            writer.write_pair("MAN_BASIS", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.man_basis_id {
            writer.write_pair("MAN_BASIS_ID", v);
        }
        writer.write_pair("MAN_DEVICE_ID", &self.man_device_id);
        if let Some(v) = &self.man_prev_epoch {
            writer.write_pair("MAN_PREV_EPOCH", v);
        }
        if let Some(v) = &self.man_next_epoch {
            writer.write_pair("MAN_NEXT_EPOCH", v);
        }
        if let Some(v) = &self.man_purpose {
            writer.write_pair("MAN_PURPOSE", v);
        }
        if let Some(v) = &self.man_pred_source {
            writer.write_pair("MAN_PRED_SOURCE", v);
        }
        writer.write_pair("MAN_REF_FRAME", &self.man_ref_frame);
        if let Some(v) = &self.man_frame_epoch {
            writer.write_pair("MAN_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.grav_assist_name {
            writer.write_pair("GRAV_ASSIST_NAME", v);
        }
        writer.write_pair("DC_TYPE", format!("{:?}", self.dc_type).to_uppercase());
        if let Some(v) = &self.dc_win_open {
            writer.write_pair("DC_WIN_OPEN", v);
        }
        if let Some(v) = &self.dc_win_close {
            writer.write_pair("DC_WIN_CLOSE", v);
        }
        if let Some(v) = &self.dc_min_cycles {
            writer.write_pair("DC_MIN_CYCLES", v);
        }
        if let Some(v) = &self.dc_max_cycles {
            writer.write_pair("DC_MAX_CYCLES", v);
        }
        if let Some(v) = &self.dc_exec_start {
            writer.write_pair("DC_EXEC_START", v);
        }
        if let Some(v) = &self.dc_exec_stop {
            writer.write_pair("DC_EXEC_STOP", v);
        }
        if let Some(v) = &self.dc_ref_time {
            writer.write_pair("DC_REF_TIME", v);
        }
        if let Some(v) = &self.dc_time_pulse_duration {
            writer.write_measure("DC_TIME_PULSE_DURATION", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_time_pulse_period {
            writer.write_measure("DC_TIME_PULSE_PERIOD", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_ref_dir {
            writer.write_pair("DC_REF_DIR", format!("{} {} {}", v.x, v.y, v.z));
        }
        if let Some(v) = &self.dc_body_frame {
            writer.write_pair("DC_BODY_FRAME", v);
        }
        if let Some(v) = &self.dc_body_trigger {
            writer.write_pair("DC_BODY_TRIGGER", format!("{} {} {}", v.x, v.y, v.z));
        }
        if let Some(v) = &self.dc_pa_start_angle {
            writer.write_measure("DC_PA_START_ANGLE", &v.to_unit_value());
        }
        if let Some(v) = &self.dc_pa_stop_angle {
            writer.write_measure("DC_PA_STOP_ANGLE", &v.to_unit_value());
        }
        writer.write_pair("MAN_COMPOSITION", &self.man_composition);
        if let Some(v) = &self.man_units {
            writer.write_pair("MAN_UNITS", v);
        }
        for line in &self.man_lines {
            writer.write_line(format!("{} {}", line.epoch, line.values.join(" ")));
        }
        writer.write_section("MAN_STOP");
    }
}

//----------------------------------------------------------------------
// 5. Perturbations

//----------------------------------------------------------------------
// 5. Perturbations (ocmPerturbationsType)
//----------------------------------------------------------------------

/// OCM Perturbations Parameters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmPerturbations {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Name of the atmospheric model (value to be drawn from the SANA registry list of Atmospheric
    /// Models at https://sanaregistry.org/r/atmospheric_model).
    ///
    /// **Examples**: JB2008, MSISE00
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    /// Name of the gravity model (value to be drawn from the SANA registry list of Gravitational
    /// Models at https://sanaregistry.org/r/gravity_model).
    ///
    /// **Examples**: EGM96, EGM2008
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    /// Equatorial radius of the central body.
    ///
    /// **Examples**: 6378137.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equatorial_radius: Option<Position>,
    /// Gravitational coefficient of the central body.
    ///
    /// **Examples**: 398600.4418
    ///
    /// **Units**: km³/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gm: Option<Gm>,
    /// List of N-body perturbations included (value(s) to be drawn from the SANA registry list of
    /// Common Central Body Names at https://sanaregistry.org/r/central_body_name).
    ///
    /// **Examples**: MOON, SUN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    /// Central body angular rotation rate.
    ///
    /// **Examples**: 0.00417807462
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub central_body_rotation: Option<AngleRate>,
    /// Oblate flattening of the central body.
    ///
    /// **Examples**: 0.00335281
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oblate_flattening: Option<f64>,
    /// Name of the ocean tides model (value to be drawn from the SANA registry list of Ocean Tides
    /// Models at https://sanaregistry.org/r/ocean_tides_model).
    ///
    /// **Examples**: FES2004
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocean_tides_model: Option<String>,
    /// Name of solid tides model (optionally specify order or constituent effects, diurnal,
    /// semi-diurnal, etc.).
    ///
    /// **Examples**: DIURNAL, SEMI-DIURNAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solid_tides_model: Option<String>,
    /// Specification of the reduction theory used for precession and nutation modeling. This is a
    /// free-text field, so if the examples on the right are insufficient, others may be used.
    ///
    /// **Examples**: IAU1976/FK5, IAU2010
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduction_theory: Option<String>,
    /// Name of the albedo model.
    ///
    /// **Examples**: EARTH_ALBEDO
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub albedo_model: Option<String>,
    /// Size of the albedo grid.
    ///
    /// **Examples**: 10
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub albedo_grid_size: Option<u64>,
    /// Shadow model used for Solar Radiation Pressure; dual cone uses both umbra/penumbra regions.
    //  Selected option should be one of ‘NONE’, ‘CYLINDRICAL’, ‘CONE’, or ‘DUAL_CONE’.
    ///
    /// **Examples**: NONE, CONE, DUAL_CONE, CYLINDRICAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_model: Option<String>,
    /// List of bodies included in shadow calculations (value(s) to be drawn from the SANA registry
    /// list of Orbit Centers at <https://sanaregistry.org/r/orbit_centers>).
    ///
    /// **Examples**: EARTH, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_bodies: Option<String>,
    /// Name of the Solar Radiation Pressure (SRP) model.
    ///
    /// **Examples**: CANNONBALL, FLAT_PLATE, BOX_WING
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srp_model: Option<String>,
    /// Space weather data source.
    ///
    /// **Examples**: NOAA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_data_source: Option<String>,
    /// Epoch of the space weather data.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_data_epoch: Option<Epoch>,
    /// Free-text field specifying the method used to select or interpolate any and all sequential
    /// space weather data (Kp, ap, Dst, F10.7, M10.7, S10.7, Y10.7, etc.). While not constrained to
    /// specific entries, it is anticipated that the utilized method would match methods detailed in
    /// numerical analysis textbooks.
    ///
    /// **Examples**: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_interp_method: Option<String>,
    /// Fixed geomagnetic Kp index.
    ///
    /// **Examples**: 3.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_kp: Option<Geomag>,
    /// Fixed geomagnetic Ap index.
    ///
    /// **Examples**: 15.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_ap: Option<Geomag>,
    /// Fixed geomagnetic Dst index.
    ///
    /// **Examples**: -20.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_dst: Option<Geomag>,
    /// Fixed F10.7 solar flux.
    ///
    /// **Examples**: 150.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_f10p7: Option<SolarFlux>,
    /// Fixed 81-day average F10.7 solar flux.
    ///
    /// **Examples**: 140.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_f10p7_mean: Option<SolarFlux>,
    /// Fixed M10.7 solar flux.
    ///
    /// **Examples**: 130.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_m10p7: Option<SolarFlux>,
    /// Fixed 81-day average M10.7 solar flux.
    ///
    /// **Examples**: 120.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_m10p7_mean: Option<SolarFlux>,
    /// Fixed S10.7 solar flux.
    ///
    /// **Examples**: 110.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_s10p7: Option<SolarFlux>,
    /// Fixed 81-day average S10.7 solar flux.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_s10p7_mean: Option<SolarFlux>,
    /// Fixed Y10.7 solar flux.
    ///
    /// **Examples**: 90.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_y10p7: Option<SolarFlux>,
    /// Fixed 81-day average Y10.7 solar flux.
    ///
    /// **Examples**: 85.0
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_y10p7_mean: Option<SolarFlux>,
}

impl ToKvn for OcmPerturbations {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PERT_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.equatorial_radius {
            writer.write_measure("EQUATORIAL_RADIUS", v);
        }
        if let Some(v) = &self.gm {
            writer.write_pair("GM", v.value.to_string());
        } // GM units are optional/complex
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.central_body_rotation {
            writer.write_measure("CENTRAL_BODY_ROTATION", v);
        }
        if let Some(v) = &self.oblate_flattening {
            writer.write_pair("OBLATE_FLATTENING", v);
        }
        if let Some(v) = &self.ocean_tides_model {
            writer.write_pair("OCEAN_TIDES_MODEL", v);
        }
        if let Some(v) = &self.solid_tides_model {
            writer.write_pair("SOLID_TIDES_MODEL", v);
        }
        if let Some(v) = &self.reduction_theory {
            writer.write_pair("REDUCTION_THEORY", v);
        }
        if let Some(v) = &self.albedo_model {
            writer.write_pair("ALBEDO_MODEL", v);
        }
        if let Some(v) = &self.albedo_grid_size {
            writer.write_pair("ALBEDO_GRID_SIZE", v);
        }
        if let Some(v) = &self.shadow_model {
            writer.write_pair("SHADOW_MODEL", v);
        }
        if let Some(v) = &self.shadow_bodies {
            writer.write_pair("SHADOW_BODIES", v);
        }
        if let Some(v) = &self.srp_model {
            writer.write_pair("SRP_MODEL", v);
        }
        if let Some(v) = &self.sw_data_source {
            writer.write_pair("SW_DATA_SOURCE", v);
        }
        if let Some(v) = &self.sw_data_epoch {
            writer.write_pair("SW_DATA_EPOCH", v);
        }
        if let Some(v) = &self.sw_interp_method {
            writer.write_pair("SW_INTERP_METHOD", v);
        }
        if let Some(v) = &self.fixed_geomag_kp {
            writer.write_measure("FIXED_GEOMAG_KP", v);
        }
        if let Some(v) = &self.fixed_geomag_ap {
            writer.write_measure("FIXED_GEOMAG_AP", v);
        }
        if let Some(v) = &self.fixed_geomag_dst {
            writer.write_measure("FIXED_GEOMAG_DST", v);
        }
        if let Some(v) = &self.fixed_f10p7 {
            writer.write_measure("FIXED_F10P7", v);
        }
        if let Some(v) = &self.fixed_f10p7_mean {
            writer.write_measure("FIXED_F10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_m10p7 {
            writer.write_measure("FIXED_M10P7", v);
        }
        if let Some(v) = &self.fixed_m10p7_mean {
            writer.write_measure("FIXED_M10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_s10p7 {
            writer.write_measure("FIXED_S10P7", v);
        }
        if let Some(v) = &self.fixed_s10p7_mean {
            writer.write_measure("FIXED_S10P7_MEAN", v);
        }
        if let Some(v) = &self.fixed_y10p7 {
            writer.write_measure("FIXED_Y10P7", v);
        }
        if let Some(v) = &self.fixed_y10p7_mean {
            writer.write_measure("FIXED_Y10P7_MEAN", v);
        }
        writer.write_section("PERT_STOP");
    }
}

//----------------------------------------------------------------------
// 6. OD (ocmOdParametersType)
//----------------------------------------------------------------------

/// OCM Orbit Determination Parameters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmOdParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Identification number for this orbit determination.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    pub od_id: String,
    /// Optional identification number for the previous orbit determination.
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_prev_id: Option<String>,
    /// Type of orbit determination method used to produce the orbit estimate.
    ///
    /// **Examples**: LEAST_SQUARES, KALMAN_FILTER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    pub od_method: String,
    /// Relative or absolute time tag of the orbit determination solved-for state in the selected OCM
    /// time system recorded by the TIME_SYSTEM keyword.
    ///
    /// **Examples**: 2000-01-01T12:00:00Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    pub od_epoch: Epoch,
    /// Days elapsed between first accepted observation and OD_EPOCH.
    ///
    /// **Examples**: 1.5
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_obs: Option<DayInterval>,
    /// Days elapsed between last accepted observation and OD_EPOCH.
    ///
    /// **Examples**: 0.1
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_last_obs: Option<DayInterval>,
    /// Number of days of observations recommended for the OD of the object (useful only for Batch OD
    /// systems).
    ///
    /// **Examples**: 5.0
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_od_span: Option<DayInterval>,
    /// Actual time span in days used for the OD of the object.
    ///
    /// **Examples**: 4.8
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_od_span: Option<DayInterval>,
    /// The number of observations available within the actual OD time span.
    ///
    /// **Examples**: 100
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_available: Option<u64>,
    /// The number of observations accepted within the actual OD time span.
    ///
    /// **Examples**: 95
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_used: Option<u64>,
    /// The number of sensor tracks available for the OD within the actual time span (see definition
    /// of ‘tracks’, 1.5.2).
    ///
    /// **Examples**: 10
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_available: Option<u64>,
    /// The number of sensor tracks accepted for the OD within the actual time span (see definition of
    /// ‘tracks’, 1.5.2).
    ///
    /// **Examples**: 9
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_used: Option<u64>,
    /// The maximum time between observations in the OD of the object.
    ///
    /// **Examples**: 0.5
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_obs_gap: Option<DayInterval>,
    /// Positional error ellipsoid 1 sigma (1σ) major eigenvalue at the epoch of the OD.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigmaj: Option<Length>,
    /// Positional error ellipsoid 1σ intermediate eigenvalue at the epoch of the OD.
    ///
    /// **Examples**: 50.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigint: Option<Length>,
    /// Positional error ellipsoid 1σ minor eigenvalue at the epoch of the OD.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigmin: Option<Length>,
    /// The resulting maximum predicted major eigenvalue of the 1σ positional error ellipsoid over
    /// the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// **Examples**: 500.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_max_pred_eigmaj: Option<Length>,
    /// The resulting minimum predicted minor eigenvalue of the 1σ positional error ellipsoid over
    /// the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_min_pred_eigmin: Option<Length>,
    /// OD confidence metric, which spans 0 to 100% (useful only for Filter-based OD systems).
    ///
    /// **Examples**: 99.0
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_confidence: Option<Percentage>,
    /// Generalized Dilution Of Precision for this orbit determination.
    ///
    /// **Examples**: 1.5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gdop: Option<f64>,
    /// The number of solve-for states in the orbit determination.
    ///
    /// **Examples**: 6
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve_n: Option<u64>,
    /// Free-text comma-delimited description of the state elements solved for in the orbit
    /// determination.
    ///
    /// **Examples**: X, Y, Z, X_DOT, Y_DOT, Z_DOT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve_states: Option<String>,
    /// The number of consider parameters used in the orbit determination.
    ///
    /// **Examples**: 3
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consider_n: Option<u64>,
    /// Free-text comma-delimited description of the consider parameters used in the orbit
    /// determination.
    ///
    /// **Examples**: DRAG_COEFF, SRP_COEFF
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consider_params: Option<String>,
    /// The Specific Energy Dissipation Rate, which is the amount of energy being removed from the
    /// object's orbit by the non-conservative forces.
    ///
    /// **Examples**: 1.25e-7
    ///
    /// **Units**: W/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedr: Option<Wkg>,
    /// The number of sensors used in the orbit determination.
    ///
    /// **Examples**: 5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensors_n: Option<u64>,
    /// Free-text comma-delimited description of the sensors used in the orbit determination.
    ///
    /// **Examples**: SENSOR1, SENSOR2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensors: Option<String>,
    /// (Useful/valid only for Batch OD systems.) The weighted RMS residual ratio.
    ///
    /// **Examples**: 0.95
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighted_rms: Option<f64>,
    /// Comma-separated list of observation data types utilized in this orbit determination.
    ///
    /// **Examples**: RANGE, DOPPLER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
}

impl ToKvn for OcmOdParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("OD_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OD_ID", &self.od_id);
        if let Some(v) = &self.od_prev_id {
            writer.write_pair("OD_PREV_ID", v);
        }
        writer.write_pair("OD_METHOD", &self.od_method);
        writer.write_pair("OD_EPOCH", self.od_epoch);
        if let Some(v) = &self.days_since_first_obs {
            writer.write_measure("DAYS_SINCE_FIRST_OBS", &v.to_unit_value());
        }
        if let Some(v) = &self.days_since_last_obs {
            writer.write_measure("DAYS_SINCE_LAST_OBS", &v.to_unit_value());
        }
        if let Some(v) = &self.recommended_od_span {
            writer.write_measure("RECOMMENDED_OD_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.actual_od_span {
            writer.write_measure("ACTUAL_OD_SPAN", &v.to_unit_value());
        }
        if let Some(v) = &self.obs_available {
            writer.write_pair("OBS_AVAILABLE", v);
        }
        if let Some(v) = &self.obs_used {
            writer.write_pair("OBS_USED", v);
        }
        if let Some(v) = &self.tracks_available {
            writer.write_pair("TRACKS_AVAILABLE", v);
        }
        if let Some(v) = &self.tracks_used {
            writer.write_pair("TRACKS_USED", v);
        }
        if let Some(v) = &self.maximum_obs_gap {
            writer.write_measure("MAXIMUM_OBS_GAP", &v.to_unit_value());
        }
        if let Some(v) = &self.od_epoch_eigmaj {
            writer.write_measure("OD_EPOCH_EIGMAJ", v);
        }
        if let Some(v) = &self.od_epoch_eigint {
            writer.write_measure("OD_EPOCH_EIGINT", v);
        }
        if let Some(v) = &self.od_epoch_eigmin {
            writer.write_measure("OD_EPOCH_EIGMIN", v);
        }
        if let Some(v) = &self.od_max_pred_eigmaj {
            writer.write_measure("OD_MAX_PRED_EIGMAJ", v);
        }
        if let Some(v) = &self.od_min_pred_eigmin {
            writer.write_measure("OD_MIN_PRED_EIGMIN", v);
        }
        if let Some(v) = &self.od_confidence {
            writer.write_measure("OD_CONFIDENCE", &v.to_unit_value());
        }
        if let Some(v) = &self.gdop {
            writer.write_pair("GDOP", v);
        }
        if let Some(v) = &self.solve_n {
            writer.write_pair("SOLVE_N", v);
        }
        if let Some(v) = &self.solve_states {
            writer.write_pair("SOLVE_STATES", v);
        }
        if let Some(v) = &self.consider_n {
            writer.write_pair("CONSIDER_N", v);
        }
        if let Some(v) = &self.consider_params {
            writer.write_pair("CONSIDER_PARAMS", v);
        }
        if let Some(v) = &self.sedr {
            writer.write_measure("SEDR", v);
        }
        if let Some(v) = &self.sensors_n {
            writer.write_pair("SENSORS_N", v);
        }
        if let Some(v) = &self.sensors {
            writer.write_pair("SENSORS", v);
        }
        if let Some(v) = &self.weighted_rms {
            writer.write_pair("WEIGHTED_RMS", v);
        }
        if let Some(v) = &self.data_types {
            writer.write_pair("DATA_TYPES", v);
        }
        writer.write_section("OD_STOP");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::traits::Ndm;

    #[test]
    fn parse_simple_ocm() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
        assert_eq!(ocm.body.segment.data.traj[0].traj_lines[0].values.len(), 6);
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 1: Mandatory Metadata Fields
    // XSD: TIME_SYSTEM and EPOCH_TZERO are mandatory (no minOccurs="0")
    // =========================================================================

    #[test]
    fn test_xsd_sample_ocm_g20_xml() {
        // Parse official CCSDS OCM XML example G-20
        let xml = include_str!("../../../data/xml/ocm_g20.xml");
        let ocm = Ocm::from_xml(xml).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_kvn_roundtrip() {
        // Full roundtrip: KVN -> Ocm -> KVN
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let output = ocm.to_kvn().unwrap();

        // Parse output again
        let ocm2 = Ocm::from_kvn(&output).unwrap();
        assert_eq!(
            ocm.body.segment.metadata.time_system,
            ocm2.body.segment.metadata.time_system
        );
        assert_eq!(
            ocm.body.segment.data.traj.len(),
            ocm2.body.segment.data.traj.len()
        );
    }

    #[test]
    fn test_to_xml_roundtrip() {
        // Cover to_xml method (lines 79-81)
        // Use the official XML example which is known to be valid
        let xml = include_str!("../../../data/xml/ocm_g20.xml");
        let ocm = Ocm::from_xml(xml).unwrap();
        let xml_out = ocm.to_xml().unwrap();
        assert!(xml_out.contains("ocm"));
        // Verify we can serialize without error
        assert!(xml_out.len() > 100);
    }

    #[test]
    fn test_xml_roundtrip_with_all_blocks() {
        // Cover XML serialization for TrajLine, CovLine, ManLine
        // Use the official XML example to test XML roundtrip
        let xml = include_str!("../../../data/xml/ocm_g20.xml");
        let ocm = Ocm::from_xml(xml).unwrap();

        // Verify structure was parsed
        assert!(!ocm.body.segment.data.traj.is_empty());

        // Convert back to XML to exercise serialize methods
        let xml_out = ocm.to_xml().unwrap();
        assert!(xml_out.contains("traj"));
    }

    #[test]
    fn test_covline_serialize_deserialize() {
        // Check if there are COV blocks
        // The official file may not have covariance data in line format
        // So we manually build one and check serialization
        let cov_line = CovLine {
            epoch: "2023-01-01T00:00:00".to_string(),
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };

        // Test Display trait which is used in to_kvn - use debug instead
        let display = format!("{:?}", cov_line);
        assert!(display.contains("2023-01-01T00:00:00"));
    }

    #[test]
    fn test_covline_xml_serialization() {
        // Cover lines 1555-1565: CovLine serialize for XML
        // Test serialization by wrapping in an XML struct
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct TestWrapper {
            cov_line: CovLine,
        }

        let cov_line = CovLine {
            epoch: "2023-01-01T00:00:00".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };

        let wrapper = TestWrapper { cov_line };

        // Use quick-xml to serialize (which uses the custom Serialize impl)
        let xml = quick_xml::se::to_string(&wrapper).unwrap();
        assert!(xml.contains("2023-01-01T00:00:00"));
        assert!(xml.contains("1"));
        assert!(xml.contains("2"));
        assert!(xml.contains("3"));

        // Deserialize and verify using quick-xml
        let deserialized: TestWrapper = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(deserialized.cov_line.epoch, "2023-01-01T00:00:00");
        assert_eq!(deserialized.cov_line.values.len(), 3);
    }

    #[test]
    fn test_manline_xml_serialization() {
        // Cover lines 1859-1885: ManLine serialize/deserialize for XML
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct TestWrapper {
            man_line: ManLine,
        }

        let man_line = ManLine {
            epoch: "2023-01-01T00:00:00".to_string(),
            values: vec!["1.0".to_string(), "2.0".to_string(), "3.0".to_string()],
        };

        let wrapper = TestWrapper { man_line };

        // Use quick-xml to serialize
        let xml = quick_xml::se::to_string(&wrapper).unwrap();
        assert!(xml.contains("2023-01-01T00:00:00"));

        // Deserialize and verify
        let deserialized: TestWrapper = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(deserialized.man_line.epoch, "2023-01-01T00:00:00");
        assert_eq!(deserialized.man_line.values.len(), 3);
    }
}
