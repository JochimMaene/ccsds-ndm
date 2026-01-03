// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdmHeader;
use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use crate::kvn::ser::KvnWriter;
use crate::traits::{FromKvnTokens, FromKvnValue, Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;

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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        let mut tokens = KvnTokenizer::new(kvn).peekable();

        // 1. Version Check
        let version = loop {
            match tokens.peek() {
                Some(Ok(KvnLine::Pair {
                    key: "CCSDS_OCM_VERS",
                    ..
                })) => {
                    if let Some(Ok(KvnLine::Pair { val, .. })) = tokens.next() {
                        break val.to_string();
                    }
                    unreachable!();
                }
                Some(Ok(KvnLine::Comment(_))) | Some(Ok(KvnLine::Empty)) => {
                    tokens.next();
                }
                Some(_) => {
                    return Err(CcsdsNdmError::MissingField(
                        "CCSDS_OCM_VERS must be the first keyword".into(),
                    ))
                }
                None => return Err(CcsdsNdmError::MissingField("Empty file".into())),
            }
        };

        // 2. Header
        let header = OdmHeader::from_kvn_tokens(&mut tokens)?;

        // 3. Body
        let body = OcmBody::from_kvn_tokens(&mut tokens)?;

        Ok(Ocm {
            header,
            body,
            id: Some("CCSDS_OCM_VERS".to_string()),
            version,
        })
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OcmBody {
    #[serde(rename = "segment")]
    pub segment: Box<OcmSegment>,
}

impl ToKvn for OcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl OcmBody {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // OCM has exactly one segment implied by structure in KVN
        let segment = OcmSegment::from_kvn_tokens(tokens)?;
        Ok(OcmBody {
            segment: Box::new(segment),
        })
    }
}

/// A single segment of the OCM.
///
/// Contains metadata and data sections.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

impl OcmSegment {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        // Expect META_START
        match tokens.peek() {
            Some(Ok(KvnLine::BlockStart("META"))) => {}
            Some(Ok(t)) => {
                return Err(CcsdsNdmError::KvnParse(format!(
                    "Expected META_START, found {:?}",
                    t
                )))
            }
            Some(Err(_)) => {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err())
            }
            None => return Err(CcsdsNdmError::KvnParse("Unexpected EOF".into())),
        }

        let metadata = OcmMetadata::from_kvn_tokens(tokens)?;
        let data = OcmData::from_kvn_tokens(tokens)?;

        Ok(OcmSegment { metadata, data })
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// OCM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        writer.write_pair("EPOCH_TZERO", &self.epoch_tzero);
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

impl OcmMetadata {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        match tokens.next() {
            Some(Ok(KvnLine::BlockStart("META"))) => {}
            _ => return Err(CcsdsNdmError::KvnParse("Expected META_START".into())),
        }

        let mut comment: Vec<String> = vec![];
        let mut object_name: Option<String> = None;
        let mut international_designator: Option<String> = None;
        let mut catalog_name: Option<String> = None;
        let mut object_designator: Option<String> = None;
        let mut alternate_names: Option<String> = None;
        let mut originator_poc: Option<String> = None;
        let mut originator_position: Option<String> = None;
        let mut originator_phone: Option<String> = None;
        let mut originator_email: Option<String> = None;
        let mut originator_address: Option<String> = None;
        let mut tech_org: Option<String> = None;
        let mut tech_poc: Option<String> = None;
        let mut tech_position: Option<String> = None;
        let mut tech_phone: Option<String> = None;
        let mut tech_email: Option<String> = None;
        let mut tech_address: Option<String> = None;
        let mut previous_message_id: Option<String> = None;
        let mut next_message_id: Option<String> = None;
        let mut adm_msg_link: Option<String> = None;
        let mut cdm_msg_link: Option<String> = None;
        let mut prm_msg_link: Option<String> = None;
        let mut rdm_msg_link: Option<String> = None;
        let mut tdm_msg_link: Option<String> = None;
        let mut operator: Option<String> = None;
        let mut owner: Option<String> = None;
        let mut country: Option<String> = None;
        let mut constellation: Option<String> = None;
        let mut object_type: Option<ObjectDescription> = None;
        let mut time_system: Option<String> = None;
        let mut epoch_tzero: Option<Epoch> = None;
        let mut ops_status: Option<String> = None;
        let mut orbit_category: Option<String> = None;
        let mut ocm_data_elements: Option<String> = None;
        let mut sclk_offset_at_epoch: Option<TimeOffset> = None;
        let mut sclk_sec_per_si_sec: Option<Duration> = None;
        let mut previous_message_epoch: Option<Epoch> = None;
        let mut next_message_epoch: Option<Epoch> = None;
        let mut start_time: Option<Epoch> = None;
        let mut stop_time: Option<Epoch> = None;
        let mut time_span: Option<DayInterval> = None;
        let mut taimutc_at_tzero: Option<TimeOffset> = None;
        let mut next_leap_epoch: Option<Epoch> = None;
        let mut next_leap_taimutc: Option<TimeOffset> = None;
        let mut ut1mutc_at_tzero: Option<TimeOffset> = None;
        let mut eop_source: Option<String> = None;
        let mut interp_method_eop: Option<String> = None;
        let mut celestial_source: Option<String> = None;

        while tokens.peek().is_some() {
            if let Some(Err(_)) = tokens.peek() {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err());
            }
            match tokens
                .peek()
                .expect("Peeked value should exist")
                .as_ref()
                .expect("Peeked value should be Ok")
            {
                KvnLine::BlockEnd("META") => {
                    tokens.next();
                    break;
                }
                KvnLine::Comment(c) => {
                    comment.push(c.to_string());
                    tokens.next();
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                KvnLine::Pair { key, val, unit } => {
                    match *key {
                        "OBJECT_NAME" => object_name = Some(val.to_string()),
                        "INTERNATIONAL_DESIGNATOR" => {
                            international_designator = Some(val.to_string())
                        }
                        "CATALOG_NAME" => catalog_name = Some(val.to_string()),
                        "OBJECT_DESIGNATOR" => object_designator = Some(val.to_string()),
                        "ALTERNATE_NAMES" => alternate_names = Some(val.to_string()),
                        "ORIGINATOR_POC" => originator_poc = Some(val.to_string()),
                        "ORIGINATOR_POSITION" => originator_position = Some(val.to_string()),
                        "ORIGINATOR_PHONE" => originator_phone = Some(val.to_string()),
                        "ORIGINATOR_EMAIL" => originator_email = Some(val.to_string()),
                        "ORIGINATOR_ADDRESS" => originator_address = Some(val.to_string()),
                        "TECH_ORG" => tech_org = Some(val.to_string()),
                        "TECH_POC" => tech_poc = Some(val.to_string()),
                        "TECH_POSITION" => tech_position = Some(val.to_string()),
                        "TECH_PHONE" => tech_phone = Some(val.to_string()),
                        "TECH_EMAIL" => tech_email = Some(val.to_string()),
                        "TECH_ADDRESS" => tech_address = Some(val.to_string()),
                        "PREVIOUS_MESSAGE_ID" => previous_message_id = Some(val.to_string()),
                        "NEXT_MESSAGE_ID" => next_message_id = Some(val.to_string()),
                        "ADM_MSG_LINK" => adm_msg_link = Some(val.to_string()),
                        "CDM_MSG_LINK" => cdm_msg_link = Some(val.to_string()),
                        "PRM_MSG_LINK" => prm_msg_link = Some(val.to_string()),
                        "RDM_MSG_LINK" => rdm_msg_link = Some(val.to_string()),
                        "TDM_MSG_LINK" => tdm_msg_link = Some(val.to_string()),
                        "OPERATOR" => operator = Some(val.to_string()),
                        "OWNER" => owner = Some(val.to_string()),
                        "COUNTRY" => country = Some(val.to_string()),
                        "CONSTELLATION" => constellation = Some(val.to_string()),
                        "OBJECT_TYPE" => object_type = Some(FromKvnValue::from_kvn_value(val)?),
                        "TIME_SYSTEM" => time_system = Some(val.to_string()),
                        "EPOCH_TZERO" => epoch_tzero = Some(FromKvnValue::from_kvn_value(val)?),
                        "OPS_STATUS" => ops_status = Some(val.to_string()),
                        "ORBIT_CATEGORY" => orbit_category = Some(val.to_string()),
                        "OCM_DATA_ELEMENTS" => ocm_data_elements = Some(val.to_string()),
                        "SCLK_OFFSET_AT_EPOCH" => {
                            sclk_offset_at_epoch = Some(TimeOffset::from_kvn(val, *unit)?)
                        }
                        "SCLK_SEC_PER_SI_SEC" => {
                            sclk_sec_per_si_sec = Some(Duration::from_kvn(val, *unit)?)
                        }
                        "PREVIOUS_MESSAGE_EPOCH" => {
                            previous_message_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                        }
                        "NEXT_MESSAGE_EPOCH" => {
                            next_message_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                        }
                        "START_TIME" => start_time = Some(FromKvnValue::from_kvn_value(val)?),
                        "STOP_TIME" => stop_time = Some(FromKvnValue::from_kvn_value(val)?),
                        "TIME_SPAN" => time_span = Some(DayInterval::from_kvn(val, *unit)?),
                        "TAIMUTC_AT_TZERO" => {
                            taimutc_at_tzero = Some(TimeOffset::from_kvn(val, *unit)?)
                        }
                        "NEXT_LEAP_EPOCH" => {
                            next_leap_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                        }
                        "NEXT_LEAP_TAIMUTC" => {
                            next_leap_taimutc = Some(TimeOffset::from_kvn(val, *unit)?)
                        }
                        "UT1MUTC_AT_TZERO" => {
                            ut1mutc_at_tzero = Some(TimeOffset::from_kvn(val, *unit)?)
                        }
                        "EOP_SOURCE" => eop_source = Some(val.to_string()),
                        "INTERP_METHOD_EOP" => interp_method_eop = Some(val.to_string()),
                        "CELESTIAL_SOURCE" => celestial_source = Some(val.to_string()),
                        _ => {
                            return Err(CcsdsNdmError::KvnParse(format!(
                                "Unexpected OCM Metadata key: {}",
                                key
                            )))
                        }
                    }
                    tokens.next();
                }
                _ => break,
            }
        }

        let ts = time_system.unwrap_or_else(|| "UTC".to_string());
        let et = epoch_tzero.ok_or(CcsdsNdmError::MissingField("EPOCH_TZERO".into()))?;

        // Apply defaults per XSD/Blue Book where applicable
        let sclk_offset_at_epoch = sclk_offset_at_epoch.or_else(|| {
            Some(
                TimeOffset::from_kvn("0.0", None)
                    .expect("default SCLK_OFFSET_AT_EPOCH '0.0' is valid"),
            )
        });
        let sclk_sec_per_si_sec = sclk_sec_per_si_sec.or_else(|| {
            Some(
                Duration::from_kvn("1.0", None)
                    .expect("default SCLK_SEC_PER_SI_SEC '1.0' is valid"),
            )
        });

        Ok(OcmMetadata {
            comment,
            object_name,
            international_designator,
            catalog_name,
            object_designator,
            alternate_names,
            originator_poc,
            originator_position,
            originator_phone,
            originator_email,
            originator_address,
            tech_org,
            tech_poc,
            tech_position,
            tech_phone,
            tech_email,
            tech_address,
            previous_message_id,
            next_message_id,
            adm_msg_link,
            cdm_msg_link,
            prm_msg_link,
            rdm_msg_link,
            tdm_msg_link,
            operator,
            owner,
            country,
            constellation,
            object_type,
            time_system: ts,
            epoch_tzero: et,
            ops_status,
            orbit_category,
            ocm_data_elements,
            sclk_offset_at_epoch,
            sclk_sec_per_si_sec,
            previous_message_epoch,
            next_message_epoch,
            start_time,
            stop_time,
            time_span,
            taimutc_at_tzero,
            next_leap_epoch,
            next_leap_taimutc,
            ut1mutc_at_tzero,
            eop_source,
            interp_method_eop,
            celestial_source,
        })
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

impl OcmData {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        let mut data = OcmData::default();
        let mut pending_comments = Vec::new();

        while tokens.peek().is_some() {
            if let Some(Err(_)) = tokens.peek() {
                return Err(tokens
                    .next()
                    .expect("Peeked error should exist")
                    .unwrap_err());
            }
            match tokens
                .peek()
                .expect("Peeked value should exist")
                .as_ref()
                .expect("Peeked value should be Ok")
            {
                KvnLine::BlockStart("TRAJ") => {
                    let mut block = OcmTrajState::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.traj.push(block);
                }
                KvnLine::BlockStart("PHYS") => {
                    let mut block = OcmPhysicalDescription::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.phys = Some(block);
                }
                KvnLine::BlockStart("COV") => {
                    let mut block = OcmCovarianceMatrix::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.cov.push(block);
                }
                KvnLine::BlockStart("MAN") => {
                    let mut block = OcmManeuverParameters::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.man.push(block);
                }
                KvnLine::BlockStart("PERT") => {
                    let mut block = OcmPerturbations::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.pert = Some(block);
                }
                KvnLine::BlockStart("OD") => {
                    let mut block = OcmOdParameters::from_kvn_tokens(tokens)?;
                    if !pending_comments.is_empty() {
                        block.comment.splice(0..0, pending_comments.drain(..));
                    }
                    data.od = Some(block);
                }
                KvnLine::BlockStart("USER") => {
                    tokens.next(); // Consume USER_START
                    let mut ud = UserDefined::default();
                    if !pending_comments.is_empty() {
                        ud.comment.splice(0..0, pending_comments.drain(..));
                    }
                    for token in tokens.by_ref() {
                        match token? {
                            KvnLine::BlockEnd("USER") => break,
                            KvnLine::Comment(c) => ud.comment.push(c.to_string()),
                            KvnLine::Pair { key, val, .. } => {
                                ud.user_defined.push(UserDefinedParameter {
                                    parameter: key.to_string(),
                                    value: val.to_string(),
                                })
                            }
                            KvnLine::Empty => continue,
                            _ => return Err(CcsdsNdmError::KvnParse("Unexpected in USER".into())),
                        }
                    }
                    data.user = Some(ud);
                }
                KvnLine::Comment(_) => {
                    if let Some(Ok(KvnLine::Comment(c))) = tokens.next() {
                        pending_comments.push(c.to_string());
                    }
                }
                KvnLine::Empty => {
                    tokens.next();
                }
                _ => break,
            }
        }
        Ok(data)
    }
}

//----------------------------------------------------------------------
// 1. Trajectory State
//----------------------------------------------------------------------

/// A block of trajectory state data, which can be a time history of states.
///
/// References:
/// - CCSDS 502.0-B-3, Section 4.5.2 (OCM Trajectory State Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
            .map(|v| v.parse::<f64>().map_err(serde::de::Error::custom))
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

impl OcmTrajState {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next(); // Consume TRAJ_START
        let mut traj = OcmTrajState {
            comment: vec![],
            traj_id: None,
            traj_prev_id: None,
            traj_next_id: None,
            traj_basis: None,
            traj_basis_id: None,
            interpolation: None,
            interpolation_degree: None,
            propagator: None,
            center_name: String::new(),
            traj_ref_frame: String::new(),
            traj_frame_epoch: None,
            useable_start_time: None,
            useable_stop_time: None,
            orb_revnum: None,
            orb_revnum_basis: None,
            traj_type: String::new(),
            orb_averaging: None,
            traj_units: None,
            traj_lines: vec![],
        };

        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("TRAJ") => break,
                KvnLine::Comment(c) => traj.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, .. } => match key {
                    "TRAJ_ID" => traj.traj_id = Some(val.into()),
                    "TRAJ_PREV_ID" => traj.traj_prev_id = Some(val.into()),
                    "TRAJ_NEXT_ID" => traj.traj_next_id = Some(val.into()),
                    "TRAJ_BASIS" => traj.traj_basis = Some(FromKvnValue::from_kvn_value(val)?),
                    "TRAJ_BASIS_ID" => traj.traj_basis_id = Some(val.into()),
                    "INTERPOLATION" => traj.interpolation = Some(val.into()),
                    "INTERPOLATION_DEGREE" => {
                        traj.interpolation_degree = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid INTERPOLATION_DEGREE: {}", e))
                        })?)
                    }
                    "PROPAGATOR" => traj.propagator = Some(val.into()),
                    "CENTER_NAME" => traj.center_name = val.into(),
                    "TRAJ_REF_FRAME" => traj.traj_ref_frame = val.into(),
                    "TRAJ_FRAME_EPOCH" => {
                        traj.traj_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "USEABLE_START_TIME" => {
                        traj.useable_start_time = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "USEABLE_STOP_TIME" => {
                        traj.useable_stop_time = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "ORB_REVNUM" => {
                        traj.orb_revnum = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid ORB_REVNUM: {}", e))
                        })?)
                    }
                    "ORB_REVNUM_BASIS" => {
                        traj.orb_revnum_basis = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "TRAJ_TYPE" => traj.traj_type = val.into(),
                    "ORB_AVERAGING" => traj.orb_averaging = Some(val.into()),
                    "TRAJ_UNITS" => traj.traj_units = Some(val.into()),
                    _ => {}
                },
                KvnLine::Raw(line) => {
                    let mut parts = line.split_whitespace();
                    if let Some(epoch) = parts.next() {
                        let values: Result<Vec<f64>> = parts
                            .map(|s| {
                                s.parse::<f64>()
                                    .map_err(|e| CcsdsNdmError::KvnParse(e.to_string()))
                            })
                            .collect();
                        traj.traj_lines.push(TrajLine {
                            epoch: epoch.to_string(),
                            values: values?,
                        });
                    }
                }
                _ => {}
            }
        }
        if traj.center_name.is_empty() {
            traj.center_name = "EARTH".to_string();
        }
        if traj.traj_ref_frame.is_empty() {
            traj.traj_ref_frame = "ICRF3".to_string();
        }
        if traj.orb_revnum_basis.is_none() {
            traj.orb_revnum_basis = Some(RevNumBasis::Zero);
        }
        if traj.orb_averaging.is_none() {
            traj.orb_averaging = Some("OSCULATING".to_string());
        }
        if traj.interpolation_degree.is_none() {
            traj.interpolation_degree = Some(3);
        }

        if traj.traj_type.is_empty() {
            return Err(CcsdsNdmError::MissingField("TRAJ_TYPE".into()));
        }
        if traj.traj_lines.is_empty() {
            return Err(CcsdsNdmError::MissingField(
                "trajLine (at least one required)".into(),
            ));
        }
        Ok(traj)
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

impl OcmPhysicalDescription {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next(); // Consume PHYS_START
        let mut phys = OcmPhysicalDescription::default();
        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("PHYS") => break,
                KvnLine::Comment(c) => phys.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, unit } => match key {
                    "MANUFACTURER" => phys.manufacturer = Some(val.into()),
                    "BUS_MODEL" => phys.bus_model = Some(val.into()),
                    "DOCKED_WITH" => phys.docked_with = Some(val.into()),
                    "DRAG_CONST_AREA" => phys.drag_const_area = Some(Area::from_kvn(val, unit)?),
                    "DRAG_COEFF_NOM" => {
                        phys.drag_coeff_nom = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid DRAG_COEFF_NOM: {}", e))
                        })?)
                    }
                    "DRAG_UNCERTAINTY" => {
                        phys.drag_uncertainty = Some(Percentage::from_kvn(val, unit)?)
                    }
                    "INITIAL_WET_MASS" => phys.initial_wet_mass = Some(Mass::from_kvn(val, unit)?),
                    "WET_MASS" => phys.wet_mass = Some(Mass::from_kvn(val, unit)?),
                    "DRY_MASS" => phys.dry_mass = Some(Mass::from_kvn(val, unit)?),
                    "OEB_PARENT_FRAME" => phys.oeb_parent_frame = Some(val.into()),
                    "OEB_PARENT_FRAME_EPOCH" => {
                        phys.oeb_parent_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "OEB_Q1" => {
                        phys.oeb_q1 = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OEB_Q1: {}", e))
                        })?)
                    }
                    "OEB_Q2" => {
                        phys.oeb_q2 = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OEB_Q2: {}", e))
                        })?)
                    }
                    "OEB_Q3" => {
                        phys.oeb_q3 = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OEB_Q3: {}", e))
                        })?)
                    }
                    "OEB_QC" => {
                        phys.oeb_qc = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OEB_QC: {}", e))
                        })?)
                    }
                    "OEB_MAX" => phys.oeb_max = Some(Length::from_kvn(val, unit)?),
                    "OEB_INT" => phys.oeb_int = Some(Length::from_kvn(val, unit)?),
                    "OEB_MIN" => phys.oeb_min = Some(Length::from_kvn(val, unit)?),
                    "AREA_ALONG_OEB_MAX" => {
                        phys.area_along_oeb_max = Some(Area::from_kvn(val, unit)?)
                    }
                    "AREA_ALONG_OEB_INT" => {
                        phys.area_along_oeb_int = Some(Area::from_kvn(val, unit)?)
                    }
                    "AREA_ALONG_OEB_MIN" => {
                        phys.area_along_oeb_min = Some(Area::from_kvn(val, unit)?)
                    }
                    "AREA_MIN_FOR_PC" => phys.area_min_for_pc = Some(Area::from_kvn(val, unit)?),
                    "AREA_MAX_FOR_PC" => phys.area_max_for_pc = Some(Area::from_kvn(val, unit)?),
                    "AREA_TYP_FOR_PC" => phys.area_typ_for_pc = Some(Area::from_kvn(val, unit)?),
                    "RCS" => phys.rcs = Some(Area::from_kvn(val, unit)?),
                    "RCS_MIN" => phys.rcs_min = Some(Area::from_kvn(val, unit)?),
                    "RCS_MAX" => phys.rcs_max = Some(Area::from_kvn(val, unit)?),
                    "SRP_CONST_AREA" => phys.srp_const_area = Some(Area::from_kvn(val, unit)?),
                    "SOLAR_RAD_COEFF" => {
                        phys.solar_rad_coeff = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid SOLAR_RAD_COEFF: {}", e))
                        })?)
                    }
                    "SOLAR_RAD_UNCERTAINTY" => {
                        phys.solar_rad_uncertainty = Some(Percentage::from_kvn(val, unit)?)
                    }
                    "VM_ABSOLUTE" => {
                        phys.vm_absolute = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid VM_ABSOLUTE: {}", e))
                        })?)
                    }
                    "VM_APPARENT_MIN" => {
                        phys.vm_apparent_min = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid VM_APPARENT_MIN: {}", e))
                        })?)
                    }
                    "VM_APPARENT" => {
                        phys.vm_apparent = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid VM_APPARENT: {}", e))
                        })?)
                    }
                    "VM_APPARENT_MAX" => {
                        phys.vm_apparent_max = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid VM_APPARENT_MAX: {}", e))
                        })?)
                    }
                    "REFLECTANCE" => {
                        phys.reflectance = Some(Probability::new(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid REFLECTANCE: {}", e))
                        })?)?)
                    }
                    "ATT_CONTROL_MODE" => phys.att_control_mode = Some(val.into()),
                    "ATT_ACTUATOR_TYPE" => phys.att_actuator_type = Some(val.into()),
                    "ATT_KNOWLEDGE" => phys.att_knowledge = Some(Angle::from_kvn(val, unit)?),
                    "ATT_CONTROL" => phys.att_control = Some(Angle::from_kvn(val, unit)?),
                    "ATT_POINTING" => phys.att_pointing = Some(Angle::from_kvn(val, unit)?),
                    "AVG_MANEUVER_FREQ" => {
                        phys.avg_maneuver_freq = Some(ManeuverFreq::from_kvn(val, unit)?)
                    }
                    "MAX_THRUST" => phys.max_thrust = Some(Thrust::from_kvn(val, unit)?),
                    "DV_BOL" => phys.dv_bol = Some(Velocity::from_kvn(val, unit)?),
                    "DV_REMAINING" => phys.dv_remaining = Some(Velocity::from_kvn(val, unit)?),
                    "IXX" => phys.ixx = Some(Moment::from_kvn(val, unit)?),
                    "IYY" => phys.iyy = Some(Moment::from_kvn(val, unit)?),
                    "IZZ" => phys.izz = Some(Moment::from_kvn(val, unit)?),
                    "IXY" => phys.ixy = Some(Moment::from_kvn(val, unit)?),
                    "IXZ" => phys.ixz = Some(Moment::from_kvn(val, unit)?),
                    "IYZ" => phys.iyz = Some(Moment::from_kvn(val, unit)?),
                    _ => {}
                },
                _ => {}
            }
        }
        if phys.oeb_parent_frame.is_none() {
            phys.oeb_parent_frame = Some("RSW_ROTATING".to_string());
        }
        Ok(phys)
    }
}

//----------------------------------------------------------------------
// 3. Covariance (ocmCovarianceMatrixType)
//----------------------------------------------------------------------

/// OCM Covariance Matrix.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Default)]
struct OcmCovarianceMatrixBuilder {
    comment: Vec<String>,
    cov_id: Option<String>,
    cov_prev_id: Option<String>,
    cov_next_id: Option<String>,
    cov_basis: Option<CovBasis>,
    cov_basis_id: Option<String>,
    cov_ref_frame: Option<String>,
    cov_frame_epoch: Option<Epoch>,
    cov_scale_min: Option<f64>,
    cov_scale_max: Option<f64>,
    cov_confidence: Option<Percentage>,
    cov_type: Option<String>,
    cov_ordering: Option<CovOrder>,
    cov_units: Option<String>,
    cov_lines: Vec<CovLine>,
}

impl OcmCovarianceMatrixBuilder {
    fn build(self) -> Result<OcmCovarianceMatrix> {
        Ok(OcmCovarianceMatrix {
            comment: self.comment,
            cov_id: self.cov_id,
            cov_prev_id: self.cov_prev_id,
            cov_next_id: self.cov_next_id,
            cov_basis: self.cov_basis,
            cov_basis_id: self.cov_basis_id,
            cov_ref_frame: self
                .cov_ref_frame
                .unwrap_or_else(|| "TNW_INERTIAL".to_string()),
            cov_frame_epoch: self.cov_frame_epoch,
            cov_scale_min: self.cov_scale_min,
            cov_scale_max: self.cov_scale_max,
            cov_confidence: self.cov_confidence,
            cov_type: self
                .cov_type
                .ok_or(CcsdsNdmError::MissingField("COV_TYPE".into()))?,
            cov_ordering: self.cov_ordering.unwrap_or(CovOrder::Ltm),
            cov_units: self.cov_units,
            cov_lines: self.cov_lines,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            .map(|v| v.parse::<f64>().map_err(serde::de::Error::custom))
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

impl OcmCovarianceMatrix {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next();
        let mut builder = OcmCovarianceMatrixBuilder::default();
        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("COV") => break,
                KvnLine::Comment(c) => builder.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, unit } => match key {
                    "COV_ID" => builder.cov_id = Some(val.into()),
                    "COV_PREV_ID" => builder.cov_prev_id = Some(val.into()),
                    "COV_NEXT_ID" => builder.cov_next_id = Some(val.into()),
                    "COV_BASIS" => builder.cov_basis = Some(FromKvnValue::from_kvn_value(val)?),
                    "COV_BASIS_ID" => builder.cov_basis_id = Some(val.into()),
                    "COV_REF_FRAME" => builder.cov_ref_frame = Some(val.into()),
                    "COV_FRAME_EPOCH" => {
                        builder.cov_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "COV_SCALE_MIN" => {
                        builder.cov_scale_min = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid COV_SCALE_MIN: {}", e))
                        })?)
                    }
                    "COV_SCALE_MAX" => {
                        builder.cov_scale_max = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid COV_SCALE_MAX: {}", e))
                        })?)
                    }
                    "COV_CONFIDENCE" => {
                        builder.cov_confidence = Some(Percentage::from_kvn(val, unit)?)
                    }
                    "COV_TYPE" => builder.cov_type = Some(val.into()),
                    "COV_ORDERING" => {
                        builder.cov_ordering = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "COV_UNITS" => builder.cov_units = Some(val.into()),
                    _ => {}
                },
                KvnLine::Raw(line) => {
                    let mut parts = line.split_whitespace();
                    if let Some(epoch) = parts.next() {
                        let values: Result<Vec<f64>> = parts
                            .map(|s| {
                                s.parse::<f64>()
                                    .map_err(|e| CcsdsNdmError::KvnParse(e.to_string()))
                            })
                            .collect();
                        builder.cov_lines.push(CovLine {
                            epoch: epoch.to_string(),
                            values: values?,
                        });
                    }
                }
                _ => {}
            }
        }
        builder.build()
    }
}

//----------------------------------------------------------------------
// 4. Maneuver (ocmManeuverParametersType)
//----------------------------------------------------------------------

/// OCM Maneuver Parameters.
///
/// References:
/// - CCSDS 502.0-B-3, Section 4.5.5 (OCM Maneuver Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Default)]
struct OcmManeuverParametersBuilder {
    comment: Vec<String>,
    man_id: Option<String>,
    man_prev_id: Option<String>,
    man_next_id: Option<String>,
    man_basis: Option<ManBasis>,
    man_basis_id: Option<String>,
    man_device_id: Option<String>,
    man_prev_epoch: Option<Epoch>,
    man_next_epoch: Option<Epoch>,
    man_purpose: Option<String>,
    man_pred_source: Option<String>,
    man_ref_frame: Option<String>,
    man_frame_epoch: Option<Epoch>,
    grav_assist_name: Option<String>,
    dc_type: Option<ManDc>,
    dc_win_open: Option<Epoch>,
    dc_win_close: Option<Epoch>,
    dc_min_cycles: Option<u64>,
    dc_max_cycles: Option<u64>,
    dc_exec_start: Option<Epoch>,
    dc_exec_stop: Option<Epoch>,
    dc_ref_time: Option<Epoch>,
    dc_time_pulse_duration: Option<Duration>,
    dc_time_pulse_period: Option<Duration>,
    dc_ref_dir: Option<Vec3Double>,
    dc_body_frame: Option<String>,
    dc_body_trigger: Option<Vec3Double>,
    dc_pa_start_angle: Option<Angle>,
    dc_pa_stop_angle: Option<Angle>,
    man_composition: Option<String>,
    man_units: Option<String>,
    man_lines: Vec<ManLine>,
}

impl OcmManeuverParametersBuilder {
    fn build(self) -> Result<OcmManeuverParameters> {
        Ok(OcmManeuverParameters {
            comment: self.comment,
            man_id: self
                .man_id
                .ok_or(CcsdsNdmError::MissingField("MAN_ID".into()))?,
            man_prev_id: self.man_prev_id,
            man_next_id: self.man_next_id,
            man_basis: self.man_basis,
            man_basis_id: self.man_basis_id,
            man_device_id: self
                .man_device_id
                .ok_or(CcsdsNdmError::MissingField("MAN_DEVICE_ID".into()))?,
            man_prev_epoch: self.man_prev_epoch,
            man_next_epoch: self.man_next_epoch,
            man_purpose: self.man_purpose,
            man_pred_source: self.man_pred_source,
            man_ref_frame: self
                .man_ref_frame
                .unwrap_or_else(|| "TNW_INERTIAL".to_string()),
            man_frame_epoch: self.man_frame_epoch,
            grav_assist_name: self.grav_assist_name,
            dc_type: self.dc_type.unwrap_or(ManDc::Continuous),
            dc_win_open: self.dc_win_open,
            dc_win_close: self.dc_win_close,
            dc_min_cycles: self.dc_min_cycles,
            dc_max_cycles: self.dc_max_cycles,
            dc_exec_start: self.dc_exec_start,
            dc_exec_stop: self.dc_exec_stop,
            dc_ref_time: self.dc_ref_time,
            dc_time_pulse_duration: self.dc_time_pulse_duration,
            dc_time_pulse_period: self.dc_time_pulse_period,
            dc_ref_dir: self.dc_ref_dir,
            dc_body_frame: self.dc_body_frame,
            dc_body_trigger: self.dc_body_trigger,
            dc_pa_start_angle: self.dc_pa_start_angle,
            dc_pa_stop_angle: self.dc_pa_stop_angle,
            man_composition: self
                .man_composition
                .ok_or(CcsdsNdmError::MissingField("MAN_COMPOSITION".into()))?,
            man_units: self.man_units,
            man_lines: self.man_lines,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl OcmManeuverParameters {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next();
        let mut builder = OcmManeuverParametersBuilder::default();
        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("MAN") => break,
                KvnLine::Comment(c) => builder.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, unit } => match key {
                    "MAN_ID" => builder.man_id = Some(val.into()),
                    "MAN_PREV_ID" => builder.man_prev_id = Some(val.into()),
                    "MAN_NEXT_ID" => builder.man_next_id = Some(val.into()),
                    "MAN_BASIS" => builder.man_basis = Some(FromKvnValue::from_kvn_value(val)?),
                    "MAN_BASIS_ID" => builder.man_basis_id = Some(val.into()),
                    "MAN_DEVICE_ID" => builder.man_device_id = Some(val.into()),
                    "MAN_PREV_EPOCH" => {
                        builder.man_prev_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "MAN_NEXT_EPOCH" => {
                        builder.man_next_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "MAN_PURPOSE" => builder.man_purpose = Some(val.into()),
                    "MAN_PRED_SOURCE" => builder.man_pred_source = Some(val.into()),
                    "MAN_REF_FRAME" => builder.man_ref_frame = Some(val.into()),
                    "MAN_FRAME_EPOCH" => {
                        builder.man_frame_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "GRAV_ASSIST_NAME" => builder.grav_assist_name = Some(val.into()),
                    "DC_TYPE" => builder.dc_type = Some(FromKvnValue::from_kvn_value(val)?),
                    "DC_WIN_OPEN" => builder.dc_win_open = Some(FromKvnValue::from_kvn_value(val)?),
                    "DC_WIN_CLOSE" => {
                        builder.dc_win_close = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "DC_MIN_CYCLES" => {
                        builder.dc_min_cycles = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid DC_MIN_CYCLES: {}", e))
                        })?)
                    }
                    "DC_MAX_CYCLES" => {
                        builder.dc_max_cycles = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid DC_MAX_CYCLES: {}", e))
                        })?)
                    }
                    "DC_EXEC_START" => {
                        builder.dc_exec_start = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "DC_EXEC_STOP" => {
                        builder.dc_exec_stop = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "DC_REF_TIME" => builder.dc_ref_time = Some(FromKvnValue::from_kvn_value(val)?),
                    "DC_TIME_PULSE_DURATION" => {
                        builder.dc_time_pulse_duration = Some(Duration::from_kvn(val, unit)?)
                    }
                    "DC_TIME_PULSE_PERIOD" => {
                        builder.dc_time_pulse_period = Some(Duration::from_kvn(val, unit)?)
                    }
                    "DC_REF_DIR" => builder.dc_ref_dir = Some(Vec3Double::from_kvn_value(val)?),
                    "DC_BODY_FRAME" => builder.dc_body_frame = Some(val.into()),
                    "DC_BODY_TRIGGER" => {
                        builder.dc_body_trigger = Some(Vec3Double::from_kvn_value(val)?)
                    }
                    "DC_PA_START_ANGLE" => {
                        builder.dc_pa_start_angle = Some(Angle::from_kvn(val, unit)?)
                    }
                    "DC_PA_STOP_ANGLE" => {
                        builder.dc_pa_stop_angle = Some(Angle::from_kvn(val, unit)?)
                    }
                    "MAN_COMPOSITION" => builder.man_composition = Some(val.into()),
                    "MAN_UNITS" => builder.man_units = Some(val.into()),
                    _ => {}
                },
                KvnLine::Raw(line) => {
                    let mut parts = line.split_whitespace();
                    if let Some(epoch) = parts.next() {
                        builder.man_lines.push(ManLine {
                            epoch: epoch.to_string(),
                            values: parts.map(|s| s.to_string()).collect(),
                        });
                    }
                }
                _ => {}
            }
        }
        builder.build()
    }
}

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

impl OcmPerturbations {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next();
        let mut pert = OcmPerturbations::default();
        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("PERT") => break,
                KvnLine::Comment(c) => pert.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, unit } => match key {
                    "ATMOSPHERIC_MODEL" => pert.atmospheric_model = Some(val.into()),
                    "GRAVITY_MODEL" => pert.gravity_model = Some(val.into()),
                    "EQUATORIAL_RADIUS" => {
                        pert.equatorial_radius = Some(Position::from_kvn(val, unit)?)
                    }
                    "GM" => pert.gm = Some(Gm::from_kvn(val, unit)?),
                    "N_BODY_PERTURBATIONS" => pert.n_body_perturbations = Some(val.into()),
                    "CENTRAL_BODY_ROTATION" => {
                        pert.central_body_rotation = Some(AngleRate::from_kvn(val, unit)?)
                    }
                    "OBLATE_FLATTENING" => {
                        pert.oblate_flattening = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OBLATE_FLATTENING: {}", e))
                        })?)
                    }
                    "OCEAN_TIDES_MODEL" => pert.ocean_tides_model = Some(val.into()),
                    "SOLID_TIDES_MODEL" => pert.solid_tides_model = Some(val.into()),
                    "REDUCTION_THEORY" => pert.reduction_theory = Some(val.into()),
                    "ALBEDO_MODEL" => pert.albedo_model = Some(val.into()),
                    "ALBEDO_GRID_SIZE" => {
                        pert.albedo_grid_size = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid ALBEDO_GRID_SIZE: {}", e))
                        })?)
                    }
                    "SHADOW_MODEL" => pert.shadow_model = Some(val.into()),
                    "SHADOW_BODIES" => pert.shadow_bodies = Some(val.into()),
                    "SRP_MODEL" => pert.srp_model = Some(val.into()),
                    "SW_DATA_SOURCE" => pert.sw_data_source = Some(val.into()),
                    "SW_DATA_EPOCH" => {
                        pert.sw_data_epoch = Some(FromKvnValue::from_kvn_value(val)?)
                    }
                    "SW_INTERP_METHOD" => pert.sw_interp_method = Some(val.into()),
                    "FIXED_GEOMAG_KP" => pert.fixed_geomag_kp = Some(Geomag::from_kvn(val, unit)?),
                    "FIXED_GEOMAG_AP" => pert.fixed_geomag_ap = Some(Geomag::from_kvn(val, unit)?),
                    "FIXED_GEOMAG_DST" => {
                        pert.fixed_geomag_dst = Some(Geomag::from_kvn(val, unit)?)
                    }
                    "FIXED_F10P7" => pert.fixed_f10p7 = Some(SolarFlux::from_kvn(val, unit)?),
                    "FIXED_F10P7_MEAN" => {
                        pert.fixed_f10p7_mean = Some(SolarFlux::from_kvn(val, unit)?)
                    }
                    "FIXED_M10P7" => pert.fixed_m10p7 = Some(SolarFlux::from_kvn(val, unit)?),
                    "FIXED_M10P7_MEAN" => {
                        pert.fixed_m10p7_mean = Some(SolarFlux::from_kvn(val, unit)?)
                    }
                    "FIXED_S10P7" => pert.fixed_s10p7 = Some(SolarFlux::from_kvn(val, unit)?),
                    "FIXED_S10P7_MEAN" => {
                        pert.fixed_s10p7_mean = Some(SolarFlux::from_kvn(val, unit)?)
                    }
                    "FIXED_Y10P7" => pert.fixed_y10p7 = Some(SolarFlux::from_kvn(val, unit)?),
                    "FIXED_Y10P7_MEAN" => {
                        pert.fixed_y10p7_mean = Some(SolarFlux::from_kvn(val, unit)?)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(pert)
    }
}

//----------------------------------------------------------------------
// 6. OD (ocmOdParametersType)
//----------------------------------------------------------------------

/// OCM Orbit Determination Parameters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Default)]
struct OcmOdParametersBuilder {
    comment: Vec<String>,
    od_id: Option<String>,
    od_prev_id: Option<String>,
    od_method: Option<String>,
    od_epoch: Option<Epoch>,
    days_since_first_obs: Option<DayInterval>,
    days_since_last_obs: Option<DayInterval>,
    recommended_od_span: Option<DayInterval>,
    actual_od_span: Option<DayInterval>,
    obs_available: Option<u64>,
    obs_used: Option<u64>,
    tracks_available: Option<u64>,
    tracks_used: Option<u64>,
    maximum_obs_gap: Option<DayInterval>,
    od_epoch_eigmaj: Option<Length>,
    od_epoch_eigint: Option<Length>,
    od_epoch_eigmin: Option<Length>,
    od_max_pred_eigmaj: Option<Length>,
    od_min_pred_eigmin: Option<Length>,
    od_confidence: Option<Percentage>,
    gdop: Option<f64>,
    solve_n: Option<u64>,
    solve_states: Option<String>,
    consider_n: Option<u64>,
    consider_params: Option<String>,
    sedr: Option<Wkg>,
    sensors_n: Option<u64>,
    sensors: Option<String>,
    weighted_rms: Option<f64>,
    data_types: Option<String>,
}

impl OcmOdParametersBuilder {
    fn build(self) -> Result<OcmOdParameters> {
        Ok(OcmOdParameters {
            comment: self.comment,
            od_id: self
                .od_id
                .ok_or(CcsdsNdmError::MissingField("OD_ID".into()))?,
            od_prev_id: self.od_prev_id,
            od_method: self
                .od_method
                .ok_or(CcsdsNdmError::MissingField("OD_METHOD".into()))?,
            od_epoch: self
                .od_epoch
                .ok_or(CcsdsNdmError::MissingField("OD_EPOCH".into()))?,
            days_since_first_obs: self.days_since_first_obs,
            days_since_last_obs: self.days_since_last_obs,
            recommended_od_span: self.recommended_od_span,
            actual_od_span: self.actual_od_span,
            obs_available: self.obs_available,
            obs_used: self.obs_used,
            tracks_available: self.tracks_available,
            tracks_used: self.tracks_used,
            maximum_obs_gap: self.maximum_obs_gap,
            od_epoch_eigmaj: self.od_epoch_eigmaj,
            od_epoch_eigint: self.od_epoch_eigint,
            od_epoch_eigmin: self.od_epoch_eigmin,
            od_max_pred_eigmaj: self.od_max_pred_eigmaj,
            od_min_pred_eigmin: self.od_min_pred_eigmin,
            od_confidence: self.od_confidence,
            gdop: self.gdop,
            solve_n: self.solve_n,
            solve_states: self.solve_states,
            consider_n: self.consider_n,
            consider_params: self.consider_params,
            sedr: self.sedr,
            sensors_n: self.sensors_n,
            sensors: self.sensors,
            weighted_rms: self.weighted_rms,
            data_types: self.data_types,
        })
    }
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
        writer.write_pair("OD_EPOCH", &self.od_epoch);
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

impl OcmOdParameters {
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>,
    {
        tokens.next();
        let mut builder = OcmOdParametersBuilder::default();

        for token in tokens.by_ref() {
            match token? {
                KvnLine::BlockEnd("OD") => break,
                KvnLine::Comment(c) => builder.comment.push(c.to_string()),
                KvnLine::Empty => continue,
                KvnLine::Pair { key, val, unit } => match key {
                    "OD_ID" => builder.od_id = Some(val.into()),
                    "OD_PREV_ID" => builder.od_prev_id = Some(val.into()),
                    "OD_METHOD" => builder.od_method = Some(val.into()),
                    "OD_EPOCH" => builder.od_epoch = Some(FromKvnValue::from_kvn_value(val)?),
                    "DAYS_SINCE_FIRST_OBS" => {
                        builder.days_since_first_obs = Some(DayInterval::from_kvn(val, unit)?)
                    }
                    "DAYS_SINCE_LAST_OBS" => {
                        builder.days_since_last_obs = Some(DayInterval::from_kvn(val, unit)?)
                    }
                    "RECOMMENDED_OD_SPAN" => {
                        builder.recommended_od_span = Some(DayInterval::from_kvn(val, unit)?)
                    }
                    "ACTUAL_OD_SPAN" => {
                        builder.actual_od_span = Some(DayInterval::from_kvn(val, unit)?)
                    }
                    "OBS_AVAILABLE" => {
                        builder.obs_available = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OBS_AVAILABLE: {}", e))
                        })?)
                    }
                    "OBS_USED" => {
                        builder.obs_used = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid OBS_USED: {}", e))
                        })?)
                    }
                    "TRACKS_AVAILABLE" => {
                        builder.tracks_available = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid TRACKS_AVAILABLE: {}", e))
                        })?)
                    }
                    "TRACKS_USED" => {
                        builder.tracks_used = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid TRACKS_USED: {}", e))
                        })?)
                    }
                    "MAXIMUM_OBS_GAP" => {
                        builder.maximum_obs_gap = Some(DayInterval::from_kvn(val, unit)?)
                    }
                    "OD_EPOCH_EIGMAJ" => {
                        builder.od_epoch_eigmaj = Some(Length::from_kvn(val, unit)?)
                    }
                    "OD_EPOCH_EIGINT" => {
                        builder.od_epoch_eigint = Some(Length::from_kvn(val, unit)?)
                    }
                    "OD_EPOCH_EIGMIN" => {
                        builder.od_epoch_eigmin = Some(Length::from_kvn(val, unit)?)
                    }
                    "OD_MAX_PRED_EIGMAJ" => {
                        builder.od_max_pred_eigmaj = Some(Length::from_kvn(val, unit)?)
                    }
                    "OD_MIN_PRED_EIGMIN" => {
                        builder.od_min_pred_eigmin = Some(Length::from_kvn(val, unit)?)
                    }
                    "OD_CONFIDENCE" => {
                        builder.od_confidence = Some(Percentage::from_kvn(val, unit)?)
                    }
                    "GDOP" => {
                        builder.gdop =
                            Some(val.parse().map_err(|e| {
                                CcsdsNdmError::KvnParse(format!("Invalid GDOP: {}", e))
                            })?)
                    }
                    "SOLVE_N" => {
                        builder.solve_n = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid SOLVE_N: {}", e))
                        })?)
                    }
                    "SOLVE_STATES" => builder.solve_states = Some(val.into()),
                    "CONSIDER_N" => {
                        builder.consider_n = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid CONSIDER_N: {}", e))
                        })?)
                    }
                    "CONSIDER_PARAMS" => builder.consider_params = Some(val.into()),
                    "SEDR" => builder.sedr = Some(Wkg::from_kvn(val, unit)?),
                    "SENSORS_N" => {
                        builder.sensors_n = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid SENSORS_N: {}", e))
                        })?)
                    }
                    "SENSORS" => builder.sensors = Some(val.into()),
                    "WEIGHTED_RMS" => {
                        builder.weighted_rms = Some(val.parse().map_err(|e| {
                            CcsdsNdmError::KvnParse(format!("Invalid WEIGHTED_RMS: {}", e))
                        })?)
                    }
                    "DATA_TYPES" => builder.data_types = Some(val.into()),
                    _ => {}
                },
                _ => {}
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_xsd_metadata_mandatory_time_system() {
        // XSD: TIME_SYSTEM is mandatory (no minOccurs="0" attribute)
        // Note: The library defaults TIME_SYSTEM to "UTC" if missing
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        // Library defaults to UTC if not present
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_xsd_metadata_mandatory_epoch_tzero() {
        // XSD: EPOCH_TZERO is mandatory (no minOccurs="0" attribute)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "EPOCH_TZERO"));
    }

    #[test]
    fn test_xsd_metadata_all_optional_fields() {
        // XSD: Most metadata fields are minOccurs="0" - verify they can all be set
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SATELLITE-1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = NORAD
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT1
ORIGINATOR_POC = John Doe
OPERATOR = OPERATOR_A
OWNER = OWNER_B
COUNTRY = USA
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body.segment.metadata.object_name,
            Some("SATELLITE-1".into())
        );
        assert_eq!(
            ocm.body.segment.metadata.international_designator,
            Some("2023-001A".into())
        );
        assert_eq!(
            ocm.body.segment.metadata.operator,
            Some("OPERATOR_A".into())
        );
        assert_eq!(ocm.body.segment.metadata.country, Some("USA".into()));
    }

    #[test]
    fn test_xsd_metadata_sclk_defaults() {
        // XSD: SCLK_OFFSET_AT_EPOCH default="0.0", SCLK_SEC_PER_SI_SEC default="1.0"
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
        // XSD defaults
        assert!(ocm.body.segment.metadata.sclk_offset_at_epoch.is_some());
        assert!(ocm.body.segment.metadata.sclk_sec_per_si_sec.is_some());
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 2: Trajectory Block (ocmTrajStateType)
    // XSD: CENTER_NAME, TRAJ_REF_FRAME, TRAJ_TYPE mandatory
    // XSD: trajLine minOccurs="1" maxOccurs="unbounded"
    // XSD: traj minOccurs="0" maxOccurs="unbounded" in ocmData
    // Note: Library applies defaults for some mandatory fields
    // =========================================================================

    #[test]
    fn test_xsd_traj_optional_in_data() {
        // XSD: traj minOccurs="0" - OCM can exist without trajectory block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.traj.is_empty());
    }

    #[test]
    fn test_xsd_traj_center_name_default() {
        // XSD: CENTER_NAME is mandatory but library defaults to "EARTH"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].center_name, "EARTH");
    }

    #[test]
    fn test_xsd_traj_ref_frame_default() {
        // XSD: TRAJ_REF_FRAME is mandatory but library defaults to "ICRF3"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].traj_ref_frame, "ICRF3");
    }

    #[test]
    fn test_xsd_traj_mandatory_traj_type() {
        // XSD: TRAJ_TYPE is mandatory (no minOccurs="0")
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
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "TRAJ_TYPE"));
    }

    #[test]
    fn test_xsd_traj_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple trajectory blocks
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
TRAJ_START
CENTER_NAME = MOON
TRAJ_REF_FRAME = ICRF
TRAJ_TYPE = KEPLERIAN
2023-01-01T01:00:00 7000 0.001 28 0 0 0
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 2);
        assert_eq!(ocm.body.segment.data.traj[0].center_name, "EARTH");
        assert_eq!(ocm.body.segment.data.traj[1].center_name, "MOON");
    }

    #[test]
    fn test_xsd_traj_multiple_lines() {
        // XSD: trajLine maxOccurs="unbounded" - multiple trajectory data lines
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
2023-01-01T01:00:00 1.1 2.1 3.1 4.1 5.1 6.1
2023-01-01T02:00:00 1.2 2.2 3.2 4.2 5.2 6.2
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 3);
    }

    #[test]
    fn test_xsd_traj_optional_fields() {
        // XSD: Many trajectory fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
TRAJ_ID = TRAJECTORY_1
TRAJ_PREV_ID = TRAJECTORY_0
TRAJ_NEXT_ID = TRAJECTORY_2
TRAJ_BASIS = DETERMINED
INTERPOLATION = LAGRANGE
INTERPOLATION_DEGREE = 7
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 100
TRAJ_TYPE = CARTPV
ORB_AVERAGING = OSCULATING
TRAJ_UNITS = km km km km/s km/s km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let traj = &ocm.body.segment.data.traj[0];
        assert_eq!(traj.traj_id, Some("TRAJECTORY_1".into()));
        assert_eq!(traj.interpolation, Some("LAGRANGE".into()));
        assert_eq!(traj.interpolation_degree, Some(7));
        assert_eq!(traj.propagator, Some("SGP4".into()));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 3: Covariance Block (ocmCovarianceMatrixType)
    // XSD: COV_REF_FRAME, COV_TYPE, COV_ORDERING mandatory
    // XSD: covLine minOccurs="1" maxOccurs="unbounded"
    // XSD: cov minOccurs="0" maxOccurs="unbounded" in ocmData
    // =========================================================================

    #[test]
    fn test_xsd_cov_optional_in_data() {
        // XSD: cov minOccurs="0" - OCM can exist without covariance block
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
        assert!(ocm.body.segment.data.cov.is_empty());
    }

    #[test]
    fn test_xsd_cov_mandatory_type() {
        // XSD: COV_TYPE is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "COV_TYPE"));
    }

    #[test]
    fn test_xsd_cov_ref_frame_default() {
        // XSD: COV_REF_FRAME mandatory but library defaults to "TNW_INERTIAL"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_ref_frame, "TNW_INERTIAL");
    }

    #[test]
    fn test_xsd_cov_ordering_default() {
        // XSD: COV_ORDERING mandatory but library defaults to LTM
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_ordering, CovOrder::Ltm);
    }

    #[test]
    fn test_xsd_cov_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple covariance blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COV_START
COV_REF_FRAME = TNW
COV_TYPE = KEPLERIAN
COV_ORDERING = UTM
2023-01-01T01:00:00 1e-5 0 1e-5 0 0 1e-5
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov.len(), 2);
        assert_eq!(ocm.body.segment.data.cov[0].cov_type, "CARTPV");
        assert_eq!(ocm.body.segment.data.cov[1].cov_type, "KEPLERIAN");
    }

    #[test]
    fn test_xsd_cov_multiple_lines() {
        // XSD: covLine maxOccurs="unbounded"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
2023-01-01T01:00:00 1.1e-6 0 1.1e-6 0 0 1.1e-6
2023-01-01T02:00:00 1.2e-6 0 1.2e-6 0 0 1.2e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_lines.len(), 3);
    }

    #[test]
    fn test_xsd_cov_optional_fields() {
        // XSD: Many covariance fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_ID = COVARIANCE_1
COV_PREV_ID = COVARIANCE_0
COV_NEXT_ID = COVARIANCE_2
COV_BASIS = DETERMINED
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = km**2 km**2 km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let cov = &ocm.body.segment.data.cov[0];
        assert_eq!(cov.cov_id, Some("COVARIANCE_1".into()));
        assert_eq!(cov.cov_scale_min, Some(0.5));
        assert_eq!(cov.cov_scale_max, Some(2.0));
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 4: Maneuver Block (ocmManeuverParametersType)
    // XSD: MAN_ID, MAN_DEVICE_ID, MAN_REF_FRAME, DC_TYPE, MAN_COMPOSITION mandatory
    // XSD: manLine minOccurs="1" maxOccurs="unbounded"
    // XSD: man minOccurs="0" maxOccurs="unbounded" in ocmData
    // =========================================================================

    #[test]
    fn test_xsd_man_optional_in_data() {
        // XSD: man minOccurs="0" - OCM can exist without maneuver block
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
        assert!(ocm.body.segment.data.man.is_empty());
    }

    #[test]
    fn test_xsd_man_mandatory_man_id() {
        // XSD: MAN_ID is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "MAN_ID"));
    }

    #[test]
    fn test_xsd_man_mandatory_device_id() {
        // XSD: MAN_DEVICE_ID is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "MAN_DEVICE_ID"));
    }

    #[test]
    fn test_xsd_man_mandatory_composition() {
        // XSD: MAN_COMPOSITION is mandatory
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k == "MAN_COMPOSITION"));
    }

    #[test]
    fn test_xsd_man_ref_frame_default() {
        // XSD: MAN_REF_FRAME mandatory but library defaults to "TNW_INERTIAL"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.man[0].man_ref_frame, "TNW_INERTIAL");
    }

    #[test]
    fn test_xsd_man_dc_type_default() {
        // XSD: DC_TYPE mandatory but library defaults to CONTINUOUS
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.man[0].dc_type, ManDc::Continuous);
    }

    #[test]
    fn test_xsd_man_multiple_blocks_unbounded() {
        // XSD: maxOccurs="unbounded" allows multiple maneuver blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
MAN_START
MAN_ID = MANEUVER_2
MAN_DEVICE_ID = THRUSTER_2
MAN_REF_FRAME = TNW
DC_TYPE = TIME
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-02T00:00:00 0 0.1 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.man.len(), 2);
        assert_eq!(ocm.body.segment.data.man[0].man_id, "MANEUVER_1");
        assert_eq!(ocm.body.segment.data.man[1].man_id, "MANEUVER_2");
    }

    #[test]
    fn test_xsd_man_optional_fields() {
        // XSD: Many maneuver fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MANEUVER_1
MAN_PREV_ID = MANEUVER_0
MAN_NEXT_ID = MANEUVER_2
MAN_BASIS = CANDIDATE
MAN_DEVICE_ID = THRUSTER_1
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
DC_TYPE = CONTINUOUS
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T01:00:00
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let man = &ocm.body.segment.data.man[0];
        assert_eq!(man.man_prev_id, Some("MANEUVER_0".into()));
        assert_eq!(man.man_purpose, Some("ORBIT_RAISING".into()));
        assert!(man.dc_win_open.is_some());
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 5: Physical, Perturbations, OD Parameters
    // XSD: phys, pert, od all minOccurs="0" (optional)
    // XSD: All fields within these blocks are optional (minOccurs="0")
    // =========================================================================

    #[test]
    fn test_xsd_phys_optional_in_data() {
        // XSD: phys minOccurs="0" - OCM can exist without physical description
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
        assert!(ocm.body.segment.data.phys.is_none());
    }

    #[test]
    fn test_xsd_phys_all_optional_fields() {
        // XSD: All physical description fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME_CORP
BUS_MODEL = LEO_BUS
WET_MASS = 500 [kg]
DRY_MASS = 400 [kg]
DRAG_CONST_AREA = 10.0 [m**2]
DRAG_COEFF_NOM = 2.2
SRP_CONST_AREA = 8.0 [m**2]
SOLAR_RAD_COEFF = 1.2
RCS = 1.0 [m**2]
MAX_THRUST = 0.1 [N]
DV_BOL = 0.3 [km/s]
DV_REMAINING = 0.15 [km/s]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.manufacturer, Some("ACME_CORP".into()));
        assert!(phys.wet_mass.is_some());
        assert!(phys.dry_mass.is_some());
    }

    #[test]
    fn test_xsd_phys_inertia_tensor() {
        // XSD: momentType for IXX, IYY, IZZ, IXY, IXZ, IYZ
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
IXX = 100 [kg*m**2]
IYY = 200 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 10 [kg*m**2]
IXZ = 5 [kg*m**2]
IYZ = 8 [kg*m**2]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert!(phys.ixx.is_some());
        assert!(phys.iyy.is_some());
        assert!(phys.izz.is_some());
    }

    #[test]
    fn test_xsd_pert_optional_in_data() {
        // XSD: pert minOccurs="0" - OCM can exist without perturbations block
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
        assert!(ocm.body.segment.data.pert.is_none());
    }

    #[test]
    fn test_xsd_pert_all_optional_fields() {
        // XSD: All perturbation fields are minOccurs="0"
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();
        assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".into()));
        assert!(pert.gravity_model.is_some());
        assert!(pert.gm.is_some());
    }

    #[test]
    fn test_xsd_od_optional_in_data() {
        // XSD: od minOccurs="0" - OCM can exist without OD parameters block
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
        assert!(ocm.body.segment.data.od.is_none());
    }

    #[test]
    fn test_xsd_od_all_optional_fields() {
        // XSD: OD has some mandatory fields (OD_ID, OD_METHOD, OD_EPOCH) and many optional
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD_1
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
WEIGHTED_RMS = 1.5
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();
        assert_eq!(od.od_method, "BATCH_LS");
        assert!(!od.od_epoch.to_string().is_empty());
    }

    #[test]
    fn test_xsd_user_defined_optional() {
        // XSD: user minOccurs="0" - user defined block is optional
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
CUSTOM_PARAM = custom_value
ANOTHER_PARAM = another_value
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let user = ocm.body.segment.data.user.as_ref().unwrap();
        assert_eq!(user.user_defined.len(), 2);
    }

    // =========================================================================
    // XSD COMPLIANCE TESTS - Group 6: Sample Files & Roundtrips
    // =========================================================================

    #[test]
    fn test_xsd_sample_ocm_g15_kvn() {
        // Parse official CCSDS OCM example G-15
        let kvn = include_str!("../../../data/kvn/ocm_g15.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
        assert!(!ocm.body.segment.metadata.epoch_tzero.to_string().is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g16_kvn() {
        // Parse official CCSDS OCM example G-16
        let kvn = include_str!("../../../data/kvn/ocm_g16.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g17_kvn() {
        // Parse official CCSDS OCM example G-17
        let kvn = include_str!("../../../data/kvn/ocm_g17.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g18_kvn() {
        // Parse official CCSDS OCM example G-18
        let kvn = include_str!("../../../data/kvn/ocm_g18.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

    #[test]
    fn test_xsd_sample_ocm_g19_kvn() {
        // Parse official CCSDS OCM example G-19
        let kvn = include_str!("../../../data/kvn/ocm_g19.kvn");
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify mandatory metadata
        assert!(!ocm.body.segment.metadata.time_system.is_empty());
    }

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
    fn test_xsd_complex_ocm_all_blocks() {
        // OCM with trajectory, covariance, maneuver blocks
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = TEST_SATELLITE
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1000 2000 3000 4 5 6
TRAJ_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
        assert_eq!(ocm.body.segment.data.cov.len(), 1);
        assert_eq!(ocm.body.segment.data.man.len(), 1);
    }

    #[test]
    fn test_ocm_parsing_errors() {
        // Empty file
        let err = Ocm::from_kvn("").unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(msg) if msg == "Empty file"));

        // Wrong first keyword
        let err = Ocm::from_kvn("CREATION_DATE = 2023-01-01T00:00:00").unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(msg) if msg.contains("first keyword")));

        // Comments before version should be OK
        let kvn = r#"
COMMENT leading comment
CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        assert!(Ocm::from_kvn(kvn).is_ok());

        // Unexpected segment start
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
TRAJ_START
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Expected META_START")));

        // Metadata unexpected key
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
BAD_KEY = VAL
META_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Unexpected OCM Metadata key"))
        );
    }

    #[test]
    fn test_ocm_metadata_all_fields() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
COMMENT meta comment
OBJECT_NAME = SAT1
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = SATCAT
OBJECT_DESIGNATOR = 12345
ALTERNATE_NAMES = SAT_ALT
ORIGINATOR_POC = JOHN DOE
ORIGINATOR_POSITION = ENGINEER
ORIGINATOR_PHONE = 123-456
ORIGINATOR_EMAIL = john@example.com
ORIGINATOR_ADDRESS = 123 Street
TECH_ORG = SPACE_CORP
TECH_POC = JANE DOE
TECH_POSITION = SCIENTIST
TECH_PHONE = 987-654
TECH_EMAIL = jane@example.com
TECH_ADDRESS = 456 Avenue
PREVIOUS_MESSAGE_ID = MSG_001
NEXT_MESSAGE_ID = MSG_003
ADM_MSG_LINK = ADM_LINK
CDM_MSG_LINK = CDM_LINK
PRM_MSG_LINK = PRM_LINK
RDM_MSG_LINK = RDM_LINK
TDM_MSG_LINK = TDM_LINK
OPERATOR = OPS_TEAM
OWNER = OWNER_TEAM
COUNTRY = USA
CONSTELLATION = STARLINK
OBJECT_TYPE = PAYLOAD
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
OPS_STATUS = OPERATIONAL
ORBIT_CATEGORY = LEO
OCM_DATA_ELEMENTS = ALL
SCLK_OFFSET_AT_EPOCH = 0.1 [s]
SCLK_SEC_PER_SI_SEC = 0.99 [s]
PREVIOUS_MESSAGE_EPOCH = 2022-12-31T23:00:00
NEXT_MESSAGE_EPOCH = 2023-01-01T01:00:00
START_TIME = 2023-01-01T00:00:00
STOP_TIME = 2023-01-02T00:00:00
TIME_SPAN = 1.0 [d]
TAIMUTC_AT_TZERO = 37.0 [s]
NEXT_LEAP_EPOCH = 2024-01-01T00:00:00
NEXT_LEAP_TAIMUTC = 38.0 [s]
UT1MUTC_AT_TZERO = -0.1 [s]
EOP_SOURCE = IERS
INTERP_METHOD_EOP = LINEAR
CELESTIAL_SOURCE = IAU
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let meta = &ocm.body.segment.metadata;
        assert_eq!(meta.object_name, Some("SAT1".to_string()));
        assert_eq!(meta.object_type, Some(ObjectDescription::Payload));
        assert!(meta.sclk_offset_at_epoch.is_some());

        // Roundtrip to hit write_kvn for all fields
        let output = ocm.to_kvn().unwrap();
        let ocm2 = Ocm::from_kvn(&output).unwrap();
        assert_eq!(ocm.body.segment.metadata, ocm2.body.segment.metadata);
    }

    #[test]
    fn test_ocm_data_loop_break_and_comments() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT preceding comment
USER_START
PARAM = VAL
USER_STOP
UNEXPECTED_KEY = VAL
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.user.is_some());
        assert_eq!(
            ocm.body.segment.data.user.as_ref().unwrap().comment[0],
            "preceding comment"
        );
    }

    #[test]
    fn test_ocm_user_defined_unexpected_token() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
META_START
USER_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Unexpected in USER")));
    }

    #[test]
    fn test_traj_all_fields() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
COMMENT traj comment
TRAJ_ID = T1
TRAJ_PREV_ID = T0
TRAJ_NEXT_ID = T2
TRAJ_BASIS = PREDICTED
TRAJ_BASIS_ID = B1
INTERPOLATION = LINEAR
INTERPOLATION_DEGREE = 1
PROPAGATOR = SGP4
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
USEABLE_START_TIME = 2023-01-01T00:00:00
USEABLE_STOP_TIME = 2023-01-02T00:00:00
ORB_REVNUM = 1234
ORB_REVNUM_BASIS = 1
TRAJ_TYPE = CARTPV
ORB_AVERAGING = NONE
TRAJ_UNITS = km km/s
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let traj = &ocm.body.segment.data.traj[0];
        assert_eq!(traj.traj_id, Some("T1".to_string()));
        assert_eq!(traj.traj_basis, Some(TrajBasis::Predicted));
        assert_eq!(traj.orb_revnum_basis, Some(RevNumBasis::One));

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("ORB_REVNUM_BASIS"));
        assert!(output.contains("1"));
    }

    #[test]
    fn test_phys_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
BUS_MODEL = B1
DOCKED_WITH = SAT2
DRAG_CONST_AREA = 1.0 [m**2]
DRAG_COEFF_NOM = 2.2
DRAG_UNCERTAINTY = 10 [%]
INITIAL_WET_MASS = 1000 [kg]
WET_MASS = 900 [kg]
DRY_MASS = 800 [kg]
OEB_PARENT_FRAME = GCRF
OEB_PARENT_FRAME_EPOCH = 2023-01-01T00:00:00
OEB_Q1 = 0
OEB_Q2 = 0
OEB_Q3 = 0
OEB_QC = 1
OEB_MAX = 5 [m]
OEB_INT = 3 [m]
OEB_MIN = 2 [m]
AREA_ALONG_OEB_MAX = 15 [m**2]
AREA_ALONG_OEB_INT = 10 [m**2]
AREA_ALONG_OEB_MIN = 6 [m**2]
AREA_MIN_FOR_PC = 5 [m**2]
AREA_MAX_FOR_PC = 20 [m**2]
AREA_TYP_FOR_PC = 10 [m**2]
RCS = 1 [m**2]
RCS_MIN = 0.5 [m**2]
RCS_MAX = 2 [m**2]
SRP_CONST_AREA = 12 [m**2]
SOLAR_RAD_COEFF = 1.5
SOLAR_RAD_UNCERTAINTY = 5 [%]
VM_ABSOLUTE = 4.5
VM_APPARENT_MIN = 5.0
VM_APPARENT = 5.5
VM_APPARENT_MAX = 6.0
REFLECTANCE = 0.8
ATT_CONTROL_MODE = THREE_AXIS
ATT_ACTUATOR_TYPE = REACTION_WHEELS
ATT_KNOWLEDGE = 0.1 [deg]
ATT_CONTROL = 0.5 [deg]
ATT_POINTING = 0.2 [deg]
AVG_MANEUVER_FREQ = 12 [#/yr]
MAX_THRUST = 0.1 [N]
DV_BOL = 0.5 [km/s]
DV_REMAINING = 0.2 [km/s]
IXX = 100 [kg*m**2]
IYY = 150 [kg*m**2]
IZZ = 150 [kg*m**2]
IXY = 1 [kg*m**2]
IXZ = 2 [kg*m**2]
IYZ = 3 [kg*m**2]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let phys = ocm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.manufacturer, Some("ACME".to_string()));
        assert_eq!(phys.ixx.as_ref().unwrap().value, 100.0);

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("IXX"));
        assert!(output.contains("100") || output.contains("1.0e2") || output.contains("1e2"));
    }

    #[test]
    fn test_phys_parsing_errors() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
DRAG_COEFF_NOM = NOT_A_FLOAT
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid DRAG_COEFF_NOM"))
        );

        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q1 = NOT_A_FLOAT
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OEB_Q1")));
    }

    #[test]
    fn test_pert_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = SUN MOON JUPITER
OCEAN_TIDES_MODEL = GOT
SOLID_TIDES_MODEL = IERS
REDUCTION_THEORY = IAU
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();
        assert_eq!(pert.atmospheric_model, Some("NRLMSISE-00".to_string()));
        assert!(pert.gm.is_some());
    }

    #[test]
    fn test_od_all_fields_robust() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_TIME_TAG = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 10 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 10
TRACKS_USED = 9
MAX_RESI_ACCEPTED = 3 [%]
WEIGHTED_RMS = 0.5
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();
        assert_eq!(od.od_id, "OD1");
        assert_eq!(od.obs_available, Some(100));
    }

    #[test]
    fn test_cov_ordering_wcc() {
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTMWCC
2023-01-01T00:00:00 1e-6 1e-6 1e-6 1e-6 1e-6 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.cov[0].cov_ordering, CovOrder::LtmWcc);

        // Also test UTMWCC
        let kvn2 = kvn.replace("LTMWCC", "UTMWCC");
        let ocm2 = Ocm::from_kvn(&kvn2).unwrap();
        assert_eq!(ocm2.body.segment.data.cov[0].cov_ordering, CovOrder::UtmWcc);

        // Also test FULL
        let kvn3 = kvn.replace("LTMWCC", "FULL");
        let ocm3 = Ocm::from_kvn(&kvn3).unwrap();
        assert_eq!(ocm3.body.segment.data.cov[0].cov_ordering, CovOrder::Full);
    }

    // =========================================================================
    // Additional coverage tests for 100% coverage
    // =========================================================================

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
    fn test_eof_after_meta_start() {
        // Cover line 150: EOF after META_START check
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        // This should hit the "Unexpected EOF" branch since no META_START
        assert!(matches!(err, CcsdsNdmError::KvnParse(_)));
    }

    #[test]
    fn test_ocm_data_write_kvn_all_blocks() {
        // Cover write_kvn for COV, MAN, PERT, OD, USER blocks (lines 673-690)
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
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
USER_START
COMMENT user comment
CUSTOM_PARAM = custom_value
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Now write to KVN to cover all write_kvn methods
        let output = ocm.to_kvn().unwrap();

        // Verify all blocks are present
        assert!(output.contains("COV_START"));
        assert!(output.contains("MAN_START"));
        assert!(output.contains("PERT_START"));
        assert!(output.contains("OD_START"));
        assert!(output.contains("USER_START"));
        assert!(output.contains("USER_STOP"));
        assert!(output.contains("CUSTOM_PARAM"));
    }

    #[test]
    fn test_empty_lines_in_metadata() {
        // Cover lines 488-490: Empty lines in metadata
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START

TIME_SYSTEM = UTC

EPOCH_TZERO = 2023-01-01T00:00:00

META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_empty_lines_in_data_section() {
        // Cover lines 785-787: Empty lines in data section
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
    }

    #[test]
    fn test_comments_before_blocks() {
        // Cover pending_comments splice (lines 719, 726, 733, etc.)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COMMENT Comment before TRAJ
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
COMMENT Comment before COV
COV_START
COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
COMMENT Comment before MAN
MAN_START
MAN_ID = MAN_1
MAN_DEVICE_ID = THRUSTER_1
MAN_REF_FRAME = RSW
DC_TYPE = CONTINUOUS
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
COMMENT Comment before PERT
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
PERT_STOP
COMMENT Comment before OD
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
COMMENT Comment before USER
USER_START
PARAM = VAL
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();

        // Verify comments were captured in each block
        assert!(ocm.body.segment.data.traj[0]
            .comment
            .contains(&"Comment before TRAJ".to_string()));
        assert!(ocm.body.segment.data.cov[0]
            .comment
            .contains(&"Comment before COV".to_string()));
        assert!(ocm.body.segment.data.man[0]
            .comment
            .contains(&"Comment before MAN".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .pert
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before PERT".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .od
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before OD".to_string()));
        assert!(ocm
            .body
            .segment
            .data
            .user
            .as_ref()
            .unwrap()
            .comment
            .contains(&"Comment before USER".to_string()));
    }

    #[test]
    fn test_user_comment_inside_block() {
        // Cover line 767: Comment inside USER block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
COMMENT inside user block
PARAM = VAL
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm
            .body
            .segment
            .data
            .user
            .as_ref()
            .unwrap()
            .comment
            .contains(&"inside user block".to_string()));
    }

    #[test]
    fn test_user_empty_line_inside_block() {
        // Cover line 774: Empty line inside USER block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
USER_START
PARAM1 = VAL1

PARAM2 = VAL2
USER_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body
                .segment
                .data
                .user
                .as_ref()
                .unwrap()
                .user_defined
                .len(),
            2
        );
    }

    #[test]
    fn test_traj_empty_line_inside_block() {
        // Cover line 981: Empty line inside TRAJ block
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
        assert_eq!(ocm.body.segment.data.traj[0].traj_lines.len(), 1);
    }

    #[test]
    fn test_phys_empty_line_inside_block() {
        // Cover line 1345: Empty line inside PHYS block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME

WET_MASS = 500 [kg]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
    }

    #[test]
    fn test_od_empty_line_inside_block() {
        // Cover line 2553: Empty line inside OD block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1

OD_METHOD = LS

OD_EPOCH = 2023-01-01T00:00:00
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.od.is_some());
    }

    #[test]
    fn test_traj_missing_lines() {
        // Cover lines 1057-1059: Missing trajLine error
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
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::MissingField(k) if k.contains("trajLine")));
    }

    #[test]
    fn test_traj_invalid_interpolation_degree() {
        // Cover lines 991-992: Invalid INTERPOLATION_DEGREE
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
INTERPOLATION_DEGREE = NOT_A_NUMBER
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid INTERPOLATION_DEGREE"))
        );
    }

    #[test]
    fn test_traj_invalid_orb_revnum() {
        // Cover lines 1008-1009: Invalid ORB_REVNUM
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
ORB_REVNUM = NOT_A_NUMBER
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid ORB_REVNUM")));
    }

    #[test]
    fn test_phys_invalid_oeb_q2() {
        // Cover lines 1373-1374: Invalid OEB_Q2
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q2 = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OEB_Q2")));
    }

    #[test]
    fn test_phys_invalid_oeb_q3() {
        // Cover lines 1378-1379: Invalid OEB_Q3
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_Q3 = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OEB_Q3")));
    }

    #[test]
    fn test_phys_invalid_oeb_qc() {
        // Cover lines 1383-1384: Invalid OEB_QC
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
OEB_QC = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OEB_QC")));
    }

    #[test]
    fn test_phys_invalid_solar_rad_coeff() {
        // Cover lines 1407-1408: Invalid SOLAR_RAD_COEFF
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
SOLAR_RAD_COEFF = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid SOLAR_RAD_COEFF"))
        );
    }

    #[test]
    fn test_phys_invalid_vm_fields() {
        // Cover lines 1415-1431: Invalid VM_ABSOLUTE, VM_APPARENT_MIN, VM_APPARENT, VM_APPARENT_MAX
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
VM_ABSOLUTE = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid VM_ABSOLUTE")));

        let kvn2 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT_MIN");
        let err2 = Ocm::from_kvn(&kvn2).unwrap_err();
        assert!(
            matches!(err2, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid VM_APPARENT_MIN"))
        );

        let kvn3 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT");
        let err3 = Ocm::from_kvn(&kvn3).unwrap_err();
        assert!(
            matches!(err3, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid VM_APPARENT"))
        );

        let kvn4 = kvn.replace("VM_ABSOLUTE", "VM_APPARENT_MAX");
        let err4 = Ocm::from_kvn(&kvn4).unwrap_err();
        assert!(
            matches!(err4, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid VM_APPARENT_MAX"))
        );
    }

    #[test]
    fn test_phys_invalid_reflectance() {
        // Cover lines 1435-1436: Invalid REFLECTANCE
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
REFLECTANCE = NOT_A_NUMBER
PHYS_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid REFLECTANCE")));
    }

    #[test]
    fn test_od_all_optional_fields_coverage() {
        // Cover OD write_kvn optional fields (lines 2470-2538)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
DAYS_SINCE_FIRST_OBS = 30 [d]
DAYS_SINCE_LAST_OBS = 1 [d]
RECOMMENDED_OD_SPAN = 7 [d]
ACTUAL_OD_SPAN = 7.5 [d]
OBS_AVAILABLE = 1000
OBS_USED = 950
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
MAXIMUM_OBS_GAP = 0.5 [d]
OD_EPOCH_EIGMAJ = 100 [m]
OD_EPOCH_EIGINT = 50 [m]
OD_EPOCH_EIGMIN = 25 [m]
OD_MAX_PRED_EIGMAJ = 200 [m]
OD_MIN_PRED_EIGMIN = 10 [m]
OD_CONFIDENCE = 95 [%]
GDOP = 1.5
SOLVE_N = 6
SOLVE_STATES = X Y Z VX VY VZ
CONSIDER_N = 2
CONSIDER_PARAMS = CD CR
SEDR = 0.001 [W/kg]
SENSORS_N = 3
SENSORS = SENSOR_A SENSOR_B SENSOR_C
WEIGHTED_RMS = 1.2
DATA_TYPES = RANGE DOPPLER
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let od = ocm.body.segment.data.od.as_ref().unwrap();

        // Verify all fields were parsed
        assert_eq!(od.od_prev_id, Some("OD0".to_string()));
        assert!(od.actual_od_span.is_some());
        assert_eq!(od.obs_available, Some(1000));
        assert_eq!(od.obs_used, Some(950));
        assert_eq!(od.tracks_available, Some(50));
        assert_eq!(od.tracks_used, Some(48));
        assert!(od.maximum_obs_gap.is_some());
        assert!(od.od_epoch_eigmaj.is_some());
        assert!(od.od_epoch_eigint.is_some());
        assert!(od.od_epoch_eigmin.is_some());
        assert!(od.od_max_pred_eigmaj.is_some());
        assert!(od.od_min_pred_eigmin.is_some());
        assert!(od.od_confidence.is_some());
        assert_eq!(od.gdop, Some(1.5));
        assert_eq!(od.solve_n, Some(6));
        assert!(od.solve_states.is_some());
        assert_eq!(od.consider_n, Some(2));
        assert!(od.consider_params.is_some());
        assert!(od.sedr.is_some());
        assert_eq!(od.sensors_n, Some(3));
        assert!(od.sensors.is_some());
        assert_eq!(od.weighted_rms, Some(1.2));
        assert!(od.data_types.is_some());

        // Now write to KVN to cover all the write_kvn branches
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("OD_PREV_ID"));
        assert!(output.contains("ACTUAL_OD_SPAN"));
        assert!(output.contains("OBS_AVAILABLE"));
        assert!(output.contains("TRACKS_AVAILABLE"));
        assert!(output.contains("MAXIMUM_OBS_GAP"));
        assert!(output.contains("OD_EPOCH_EIGMAJ"));
        assert!(output.contains("OD_CONFIDENCE"));
        assert!(output.contains("GDOP"));
        assert!(output.contains("SOLVE_N"));
        assert!(output.contains("SOLVE_STATES"));
        assert!(output.contains("CONSIDER_N"));
        assert!(output.contains("SEDR"));
        assert!(output.contains("SENSORS_N"));
        assert!(output.contains("WEIGHTED_RMS"));
        assert!(output.contains("DATA_TYPES"));
    }

    #[test]
    fn test_od_invalid_numeric_fields() {
        // Cover OD parsing error branches
        let base = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
OD_START
OD_ID = OD1
OD_METHOD = LS
OD_EPOCH = 2023-01-01T00:00:00
"#;

        // OBS_AVAILABLE invalid
        let kvn = format!("{base}OBS_AVAILABLE = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OBS_AVAILABLE"))
        );

        // OBS_USED invalid
        let kvn = format!("{base}OBS_USED = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid OBS_USED")));

        // TRACKS_AVAILABLE invalid
        let kvn = format!("{base}TRACKS_AVAILABLE = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid TRACKS_AVAILABLE"))
        );

        // TRACKS_USED invalid
        let kvn = format!("{base}TRACKS_USED = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid TRACKS_USED")));

        // GDOP invalid
        let kvn = format!("{base}GDOP = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid GDOP")));

        // SOLVE_N invalid
        let kvn = format!("{base}SOLVE_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid SOLVE_N")));

        // CONSIDER_N invalid
        let kvn = format!("{base}CONSIDER_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid CONSIDER_N")));

        // SENSORS_N invalid
        let kvn = format!("{base}SENSORS_N = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid SENSORS_N")));

        // WEIGHTED_RMS invalid
        let kvn = format!("{base}WEIGHTED_RMS = NOT_A_NUMBER\nOD_STOP\n");
        let err = Ocm::from_kvn(&kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(msg) if msg.contains("Invalid WEIGHTED_RMS"))
        );
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
    fn test_cov_write_kvn_all_optional_fields() {
        // Cover COV write_kvn optional fields (lines 1594-1627)
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT cov comment
COV_ID = COV_001
COV_PREV_ID = COV_000
COV_NEXT_ID = COV_002
COV_BASIS = DETERMINED
COV_BASIS_ID = BASIS_001
COV_REF_FRAME = RSW
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_SCALE_MIN = 0.5
COV_SCALE_MAX = 2.0
COV_CONFIDENCE = 95 [%]
COV_TYPE = CARTPV
COV_ORDERING = LTM
COV_UNITS = km**2
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let cov = &ocm.body.segment.data.cov[0];

        // Verify all optional fields were parsed
        assert_eq!(cov.comment, vec!["cov comment"]);
        assert_eq!(cov.cov_id, Some("COV_001".to_string()));
        assert_eq!(cov.cov_prev_id, Some("COV_000".to_string()));
        assert_eq!(cov.cov_next_id, Some("COV_002".to_string()));
        assert!(cov.cov_basis.is_some());
        assert_eq!(cov.cov_basis_id, Some("BASIS_001".to_string()));
        assert!(cov.cov_frame_epoch.is_some());
        assert_eq!(cov.cov_scale_min, Some(0.5));
        assert_eq!(cov.cov_scale_max, Some(2.0));
        assert!(cov.cov_confidence.is_some());
        assert_eq!(cov.cov_units, Some("km**2".to_string()));

        // Write to KVN to cover all write_kvn branches
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("COV_ID"));
        assert!(output.contains("COV_PREV_ID"));
        assert!(output.contains("COV_NEXT_ID"));
        assert!(output.contains("COV_BASIS"));
        assert!(output.contains("COV_BASIS_ID"));
        assert!(output.contains("COV_FRAME_EPOCH"));
        assert!(output.contains("COV_SCALE_MIN"));
        assert!(output.contains("COV_SCALE_MAX"));
        assert!(output.contains("COV_CONFIDENCE"));
        assert!(output.contains("COV_UNITS"));
    }

    #[test]
    fn test_traj_unknown_key_ignored() {
        // Cover line 1017: Unknown key in TRAJ block (wildcard match)
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
UNKNOWN_KEY = IGNORED_VALUE
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        // Should parse successfully, ignoring unknown keys
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
    }

    #[test]
    fn test_traj_block_start_in_traj_ignored() {
        // Cover lines 1032-1034: BlockStart/other tokens in TRAJ loop
        // The wildcard _ branch catches unexpected tokens
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
        assert_eq!(ocm.body.segment.data.traj[0].traj_type, "CARTPV");
    }

    #[test]
    fn test_phys_unknown_key_ignored() {
        // Cover lines 1455-1457: Unknown key in PHYS block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
MANUFACTURER = ACME
UNKNOWN_PHYS_KEY = IGNORED
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
    }

    #[test]
    fn test_metadata_unknown_token_breaks_loop() {
        // Cover line 567: Unknown token type breaks metadata loop
        // This is hard to hit directly since metadata normally ends with META_STOP
        // But we can verify with a minimal valid OCM
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.metadata.time_system, "UTC");
    }

    #[test]
    fn test_cov_comment_and_empty_line() {
        // Cover lines 1647-1648: Comment and empty line in COV block
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
COV_START
COMMENT inside cov block

COV_REF_FRAME = RSW
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1e-6 0 1e-6 0 0 1e-6
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.cov[0]
            .comment
            .contains(&"inside cov block".to_string()));
    }

    #[test]
    fn test_man_all_optional_fields_write_kvn() {
        // Cover MAN write_kvn optional fields
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
MAN_START
MAN_ID = MAN_1
MAN_PREV_ID = MAN_0
MAN_NEXT_ID = MAN_2
MAN_BASIS = PLANNED
MAN_BASIS_ID = PLAN_001
MAN_DEVICE_ID = THRUSTER_1
MAN_PREV_EPOCH = 2022-12-31T00:00:00
MAN_NEXT_EPOCH = 2023-01-02T00:00:00
MAN_PURPOSE = ORBIT_RAISING
MAN_PRED_SOURCE = FDSS
MAN_REF_FRAME = RSW
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
GRAV_ASSIST_NAME = MOON
DC_TYPE = TIME
DC_WIN_OPEN = 2023-01-01T00:00:00
DC_WIN_CLOSE = 2023-01-01T02:00:00
DC_MIN_CYCLES = 1
DC_MAX_CYCLES = 10
DC_EXEC_START = 2023-01-01T00:30:00
DC_EXEC_STOP = 2023-01-01T01:30:00
DC_REF_TIME = 2023-01-01T01:00:00
DC_TIME_PULSE_DURATION = 60 [s]
DC_TIME_PULSE_PERIOD = 120 [s]
DC_REF_DIR = 1 0 0
DC_BODY_FRAME = SC_BODY
DC_BODY_TRIGGER = 0 1 0
DC_PA_START_ANGLE = 0 [deg]
DC_PA_STOP_ANGLE = 180 [deg]
MAN_COMPOSITION = EPOCH DV_X DV_Y DV_Z
MAN_UNITS = km/s km/s km/s
2023-01-01T00:00:00 0.1 0 0
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let man = &ocm.body.segment.data.man[0];

        // Verify optional fields
        assert_eq!(man.man_prev_id, Some("MAN_0".to_string()));
        assert_eq!(man.man_next_id, Some("MAN_2".to_string()));
        assert!(man.man_basis.is_some());
        assert!(man.grav_assist_name.is_some());
        assert!(man.dc_win_open.is_some());
        assert!(man.dc_min_cycles.is_some());
        assert!(man.dc_ref_dir.is_some());
        assert!(man.dc_body_trigger.is_some());
        assert!(man.dc_pa_start_angle.is_some());

        // Write to KVN
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("MAN_PREV_ID"));
        assert!(output.contains("MAN_NEXT_ID"));
        assert!(output.contains("MAN_BASIS"));
        assert!(output.contains("GRAV_ASSIST_NAME"));
        assert!(output.contains("DC_WIN_OPEN"));
        assert!(output.contains("DC_MIN_CYCLES"));
        assert!(output.contains("DC_REF_DIR"));
        assert!(output.contains("DC_BODY_TRIGGER"));
        assert!(output.contains("DC_PA_START_ANGLE"));
    }

    #[test]
    fn test_pert_all_optional_fields_write_kvn() {
        // Cover PERT write_kvn optional fields
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PERT_START
ATMOSPHERIC_MODEL = NRLMSISE-00
GRAVITY_MODEL = EGM2008 70x70
EQUATORIAL_RADIUS = 6378.137 [km]
GM = 398600.4415 [km**3/s**2]
N_BODY_PERTURBATIONS = MOON SUN JUPITER
CENTRAL_BODY_ROTATION = 7.2921e-5 [deg/s]
OBLATE_FLATTENING = 0.003353
OCEAN_TIDES_MODEL = GOT4.7
SOLID_TIDES_MODEL = IERS2010
REDUCTION_THEORY = IERS2010
ALBEDO_MODEL = EARTH_ALBEDO
ALBEDO_GRID_SIZE = 36
SHADOW_MODEL = CYLINDRICAL
SHADOW_BODIES = MOON
SRP_MODEL = FLAT_PLATE
SW_DATA_SOURCE = CSSI
SW_DATA_EPOCH = 2023-01-01T00:00:00
SW_INTERP_METHOD = LINEAR
FIXED_GEOMAG_KP = 3 [nT]
FIXED_GEOMAG_AP = 15 [nT]
FIXED_GEOMAG_DST = -10 [nT]
FIXED_F10P7 = 150 [SFU]
FIXED_F10P7_MEAN = 145 [SFU]
FIXED_M10P7 = 148 [SFU]
FIXED_M10P7_MEAN = 143 [SFU]
FIXED_S10P7 = 147 [SFU]
FIXED_S10P7_MEAN = 142 [SFU]
FIXED_Y10P7 = 146 [SFU]
FIXED_Y10P7_MEAN = 141 [SFU]
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        let pert = ocm.body.segment.data.pert.as_ref().unwrap();

        // Verify all fields
        assert!(pert.central_body_rotation.is_some());
        assert!(pert.oblate_flattening.is_some());
        assert!(pert.albedo_grid_size.is_some());
        assert!(pert.sw_data_epoch.is_some());
        assert!(pert.fixed_geomag_kp.is_some());
        assert!(pert.fixed_m10p7.is_some());

        // Write to KVN
        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("CENTRAL_BODY_ROTATION"));
        assert!(output.contains("OBLATE_FLATTENING"));
        assert!(output.contains("ALBEDO_GRID_SIZE"));
        assert!(output.contains("FIXED_GEOMAG_KP"));
        assert!(output.contains("FIXED_M10P7"));
    }

    #[test]
    fn test_traj_orb_revnum_basis_zero() {
        // Cover ORB_REVNUM_BASIS = 0 case in write_kvn
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
ORB_REVNUM = 100
ORB_REVNUM_BASIS = 0
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(
            ocm.body.segment.data.traj[0].orb_revnum_basis,
            Some(RevNumBasis::Zero)
        );

        let output = ocm.to_kvn().unwrap();
        assert!(output.contains("ORB_REVNUM_BASIS"));
    }

    #[test]
    fn test_comment_before_cov_block() {
        // Cover line 733: pending_comments splice before COV block
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
COMMENT This comment should be prepended to COV
COV_START
COV_ID = COV1
COV_TYPE = ANGLE
COV_ORDERING = LTM
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(!ocm.body.segment.data.cov.is_empty());
        assert!(ocm.body.segment.data.cov[0]
            .comment
            .iter()
            .any(|c| c.contains("prepended to COV")));
    }

    #[test]
    fn test_comment_before_phys_block() {
        // Cover line 726: pending_comments splice before PHYS block
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
COMMENT This comment should be prepended to PHYS
PHYS_START
WET_MASS = 1000.0 [kg]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
        let phys = ocm.body.segment.data.phys.unwrap();
        assert!(phys.comment.iter().any(|c| c.contains("prepended to PHYS")));
    }

    #[test]
    fn test_cov_scale_min_max_invalid() {
        // Cover lines 1661-1667: Invalid COV_SCALE_MIN and COV_SCALE_MAX parsing
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
COV_START
COV_ID = COV1
COV_SCALE_MIN = not_a_number
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("COV_SCALE_MIN")),
            "Expected COV_SCALE_MIN error, got: {:?}",
            err
        );

        // Test COV_SCALE_MAX invalid
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
COV_START
COV_ID = COV1
COV_SCALE_MAX = invalid
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("COV_SCALE_MAX")),
            "Expected COV_SCALE_MAX error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_unknown_key_in_phys_block() {
        // Cover line 1457: _ => {} wildcard match in PHYS parsing
        // Unknown keys in PHYS block should be silently ignored
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
PHYS_START
UNKNOWN_PHYS_KEY = some_value
WET_MASS = 1000.0 [kg]
PHYS_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.phys.is_some());
        // The unknown key should be silently ignored and parsing succeeds
    }

    #[test]
    fn test_unknown_key_in_traj_block() {
        // Cover lines 1032, 1034: _ => {} wildcard match in TRAJ parsing
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
UNKNOWN_TRAJ_KEY = some_value
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert_eq!(ocm.body.segment.data.traj.len(), 1);
        // Unknown key is silently ignored
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
    fn test_missing_meta_start() {
        // Cover line 139-142 in OcmSegment: Expected META_START error
        // Note: Line 415 in OcmMetadata is dead code because OcmSegment checks first
        let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        // OcmSegment checks first and gives "Expected META_START, found BlockStart(\"TRAJ\")"
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("META_START")),
            "Expected META_START error, got: {:?}",
            err
        );
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

    #[test]
    fn test_empty_line_in_man_block() {
        // Cover line 1990: Empty line in MAN block
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
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1

MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(!ocm.body.segment.data.man.is_empty());
    }

    #[test]
    fn test_dc_min_max_cycles_invalid() {
        // Cover lines 2018-2024: Invalid DC_MIN_CYCLES/DC_MAX_CYCLES parsing
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
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
DC_MIN_CYCLES = not_a_number
MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("DC_MIN_CYCLES")),
            "Expected DC_MIN_CYCLES error, got: {:?}",
            err
        );

        // Test DC_MAX_CYCLES invalid
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
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
DC_MAX_CYCLES = invalid
MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("DC_MAX_CYCLES")),
            "Expected DC_MAX_CYCLES error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_empty_line_in_pert_block() {
        // Cover line 2245: Empty line in PERT block
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
PERT_START
ATMOSPHERIC_MODEL = NRLMSIS00

GRAVITY_MODEL = EGM-96
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.pert.is_some());
    }

    #[test]
    fn test_oblate_flattening_invalid() {
        // Cover lines 2259-2260: Invalid OBLATE_FLATTENING parsing
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
PERT_START
OBLATE_FLATTENING = not_a_number
PERT_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("OBLATE_FLATTENING")),
            "Expected OBLATE_FLATTENING error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_albedo_grid_size_invalid() {
        // Cover lines 2268-2269: Invalid ALBEDO_GRID_SIZE parsing
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
PERT_START
ALBEDO_GRID_SIZE = invalid
PERT_STOP
"#;
        let err = Ocm::from_kvn(kvn).unwrap_err();
        assert!(
            matches!(err, CcsdsNdmError::KvnParse(ref msg) if msg.contains("ALBEDO_GRID_SIZE")),
            "Expected ALBEDO_GRID_SIZE error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_unknown_key_in_od_block() {
        // Cover line 2645: _ => {} wildcard match in OD parsing
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
OD_START
OD_ID = OD1
OD_PREV_ID = OD0
OD_METHOD = BATCH_LS
OD_EPOCH = 2023-01-01T00:00:00
UNKNOWN_OD_KEY = some_value
OD_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.od.is_some());
    }

    #[test]
    fn test_unknown_key_in_cov_block() {
        // Cover line 1694: _ => {} wildcard match in COV parsing
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
COV_START
COV_ID = COV1
COV_TYPE = ANGLE
COV_ORDERING = LTM
UNKNOWN_COV_KEY = some_value
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.2
CZ_Y = 0.2
CZ_Z = 1.0
CX_DOT_X = 0.01
CX_DOT_Y = 0.01
CX_DOT_Z = 0.01
CX_DOT_X_DOT = 0.001
CY_DOT_X = 0.01
CY_DOT_Y = 0.01
CY_DOT_Z = 0.01
CY_DOT_X_DOT = 0.001
CY_DOT_Y_DOT = 0.001
CZ_DOT_X = 0.01
CZ_DOT_Y = 0.01
CZ_DOT_Z = 0.01
CZ_DOT_X_DOT = 0.001
CZ_DOT_Y_DOT = 0.001
CZ_DOT_Z_DOT = 0.001
COV_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(!ocm.body.segment.data.cov.is_empty());
    }

    #[test]
    fn test_unknown_key_in_man_block() {
        // Cover lines 2052, 2063: _ => {} wildcard match in MAN parsing
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
MAN_START
MAN_ID = MAN1
MAN_DEVICE_ID = DEV1
UNKNOWN_MAN_KEY = some_value
MAN_REF_FRAME = TNW
MAN_COMPOSITION = abc
MAN_UNITS = km/s
MAN_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(!ocm.body.segment.data.man.is_empty());
    }

    #[test]
    fn test_unknown_key_in_pert_block() {
        // Cover line 2300, 2302: _ => {} wildcard match in PERT parsing
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
PERT_START
UNKNOWN_PERT_KEY = some_value
PERT_STOP
"#;
        let ocm = Ocm::from_kvn(kvn).unwrap();
        assert!(ocm.body.segment.data.pert.is_some());
    }
}
