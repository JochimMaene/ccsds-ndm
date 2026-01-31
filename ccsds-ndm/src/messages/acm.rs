// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::AdmHeader;

use crate::error::{Result, ValidationError};
use crate::kvn::parser::KvnResult;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::SensorNoise;
use crate::types::*;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root ACM Structure
//----------------------------------------------------------------------

/// Attitude Comprehensive Message (ACM).
///
/// An ACM specifies the attitude state of a single object at multiple epochs, contained within a
/// specified time range. The ACM aggregates and extends APM and AEM content in a single
/// comprehensive hybrid message.
///
/// Capabilities include:
/// - Optional rate data elements
/// - Optional spacecraft physical properties
/// - Optional covariance elements
/// - Optional maneuver parameters
/// - Optional estimator information
///
/// **CCSDS Reference**: 504.0-B-2, Section 5.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "acm")]
pub struct Acm {
    pub header: AdmHeader,
    pub body: AcmBody,
    #[serde(rename = "@id")]
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
}

impl crate::traits::Validate for Acm {
    fn validate(&self) -> Result<()> {
        Acm::validate(self)
    }
}

impl Ndm for Acm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let acm = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Acm, &acm)?;
        Ok(acm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let acm: Self = crate::xml::from_str_with_context(xml, "ACM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Acm, &acm)?;
        Ok(acm)
    }
}

impl Acm {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AcmBody {
    #[serde(rename = "segment")]
    pub segment: Box<AcmSegment>,
}

impl ToKvn for AcmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct AcmSegment {
    pub metadata: AcmMetadata,
    pub data: AcmData,
}

impl AcmSegment {
    pub fn validate(&self, _header: &AdmHeader) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate(&self.metadata)
    }
}

impl ToKvn for AcmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

/// ACM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmMetadata {
    /// Comments (allowed only at the beginning of the ACM Metadata). Each comment line shall begin
    /// with this keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the name of the object. There is no CCSDS-based restriction on
    /// the value for this keyword, but it is recommended to use names from either the UN Office of
    /// Outer Space Affairs designator index (reference `[2]`), which include Object name and
    /// international designator), the spacecraft operator, or a State Actor or commercial Space
    /// Situational Awareness (SSA) provider maintaining the ‘CATALOG_NAME’ space catalog. If the
    /// object name is not known (uncorrelated object), ‘UNKNOWN’ may be used (or this keyword
    /// omitted).
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[builder(into)]
    pub object_name: String,
    /// Free text field containing an international designator for the object as assigned by the UN
    /// Committee on Space Research (COSPAR) and the US National Space Science Data Center (NSSDC).
    /// Such designator values have the following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year
    /// of launch. NNN = Three-digit serial number of launch in year YYYY (with leading zeros).
    /// P{PP} = At least one capital letter for the identification of the part brought into space
    /// by the launch. In cases in which the object has no international designator, the value
    /// UNKNOWN may be used. NOTE – The international designator is typically specified by
    /// ‘OBJECT_ID’ in the APM and AEM.
    ///
    /// **Examples**: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub international_designator: Option<String>,
    /// Free text field containing the satellite catalog source or the source agency or operator
    /// abbreviated name (see annex B, subsection B1).
    ///
    /// **Examples**: CSPOC, RFSA, ESA, COMSPOC
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub catalog_name: Option<String>,
    /// Free text field specification of the unique satellite identification designator for the
    /// object, as reflected in the catalog whose name is ‘CATALOG_NAME’. If the ID is not known,
    /// ‘UNKNOWN’ may be used (or this keyword omitted).
    ///
    /// **Examples**: 22444, 18SPCS 18571, UNKNOWN
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub object_designator: Option<String>,
    /// Free text field containing Programmatic or Technical Point-of-Contact (POC) for ACM.
    ///
    /// **Examples**: Ms. Rodgers
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_poc: Option<String>,
    /// Free text field containing contact position of the PoC.
    ///
    /// **Examples**: GNC Engineer, ACS Design Lead
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_position: Option<String>,
    /// Free text field containing PoC phone number.
    ///
    /// **Examples**: +49615130312
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_phone: Option<String>,
    /// Free-text field containing originator PoC email address.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_email: Option<String>,
    /// Free text field containing Technical PoC information for ACM creator (suggest email,
    /// website, or physical address, etc.).
    ///
    /// **Examples**: JANE.DOE@SOMEWHERE.NET
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_address: Option<String>,
    /// Free text field containing a unique identifier of Orbit Data Message(s) that are linked
    /// (relevant) to this Attitude Data Message.
    ///
    /// **Examples**: ODM_MSG_12345.txt, ORB_ID_0123
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub odm_msg_link: Option<String>,
    /// Celestial body orbited by the object, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. The set of allowed values is described in annex B, subsection B8.
    ///
    /// **Examples**: EARTH BARYCENTER, MOON
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub center_name: Option<String>,
    /// Time system used for metadata, attitude data, covariance data. The set of allowed values is
    /// described in annex B, subsection B2.
    ///
    /// **Examples**: UTC, TAI
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[builder(into)]
    pub time_system: String,
    /// Epoch from which all ACM relative times are referenced. (For format specification, see
    /// 6.8.9.) The time scale for EPOCH_TZERO is the one specified by ‘TIME_SYSTEM’ keyword in the
    /// Metadata section.
    ///
    /// **Examples**: 2016-11-10T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    pub epoch_tzero: Epoch,
    /// Comma-delimited list of elements of information data blocks included in this message. The
    /// order shall be the same as the order of the data blocks in the message. Values shall be
    /// confined to the following list: ATT, PHYS, COV, MAN, AD, USER. If the ACM contains multiple
    /// ATT, COV, MAN data blocks (as allowed by table 5-1), the corresponding ATT, COV, MAN entry
    /// shall be duplicated to match.
    ///
    /// **Examples**: ATT, AD, USER; ATT, ATT, PHYS; ATT, COV, AD
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub acm_data_elements: Option<String>,
    /// Time of the earliest data contained in the ACM, specified as either a relative or absolute
    /// time tag.
    ///
    /// **Examples**: 100.0, 2016-11-10T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Epoch>,
    /// Time of the latest data contained in the ACM, specified as either a relative or absolute
    /// time tag.
    ///
    /// **Examples**: 1500.0, 2016-11-11T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Epoch>,
    /// Difference (TAI – UTC) in seconds (i.e., total # leap seconds elapsed since 1958) as modeled
    /// by the message originator at epoch ‘EPOCH_TZERO’.
    ///
    /// **Examples**: 36
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taimutc_at_tzero: Option<TimeOffset>,
    /// Epoch of next leap second, specified as an absolute time tag.
    ///
    /// **Examples**: 2017-01-01T00:00:00
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_epoch: Option<Epoch>,
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since 1958)
    /// incorporated by the message originator at epoch ‘NEXT_LEAP_EPOCH’. This keyword should be
    /// provided if NEXT_LEAP_EPOCH is supplied.
    ///
    /// **Examples**: 37
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_taimutc: Option<TimeOffset>,
}

impl AcmMetadata {
    pub fn validate(&self) -> Result<()> {
        if self.object_name.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("META_START");
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        if let Some(v) = &self.international_designator {
            writer.write_pair("INTERNATIONAL_DESIGNATOR", v);
        }
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
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
        if let Some(v) = &self.odm_msg_link {
            writer.write_pair("ODM_MSG_LINK", v);
        }
        if let Some(v) = &self.center_name {
            writer.write_pair("CENTER_NAME", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.acm_data_elements {
            writer.write_pair("ACM_DATA_ELEMENTS", v);
        }
        if let Some(v) = self.start_time {
            writer.write_pair("START_TIME", v);
        }
        if let Some(v) = self.stop_time {
            writer.write_pair("STOP_TIME", v);
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
        writer.write_section("META_STOP");
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// ACM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmData {
    /// One or more optional attitude state time histories (each consisting of one or more attitude
    /// states).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(rename = "att", default)]
    #[builder(default)]
    pub att: Vec<AcmAttitudeState>,
    /// A single space object physical characteristics section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(rename = "phys", default)]
    pub phys: Option<AcmPhysicalDescription>,
    /// One or more optional covariance time histories (each consisting of one or more covariance
    /// matrix diagonals).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(rename = "cov", default)]
    #[builder(default)]
    pub cov: Vec<AcmCovarianceMatrix>,
    /// One or more optional maneuver specification section(s).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(rename = "man", default)]
    #[builder(default)]
    pub man: Vec<AcmManeuverParameters>,
    /// A single attitude determination Data section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(rename = "ad", default)]
    pub ad: Option<AcmAttitudeDetermination>,
    /// A single user-defined Data section.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.10.
    #[serde(rename = "user", default)]
    pub user: Option<UserDefined>,
}

impl AcmData {
    fn validate(&self, _metadata: &AcmMetadata) -> Result<()> {
        if let Some(phys) = &self.phys {
            phys.validate()?;
        }
        if let Some(ad) = &self.ad {
            ad.validate()?;
        }
        for man in &self.man {
            man.validate()?;
        }
        Ok(())
    }
}

impl ToKvn for AcmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        for att in &self.att {
            att.write_kvn(writer);
        }
        if let Some(phys) = &self.phys {
            phys.write_kvn(writer);
        }
        for cov in &self.cov {
            cov.write_kvn(writer);
        }
        for man in &self.man {
            man.write_kvn(writer);
        }
        if let Some(ad) = &self.ad {
            ad.write_kvn(writer);
        }
        if let Some(user) = &self.user {
            writer.write_section("USER_START");
            writer.write_comments(&user.comment);
            for p in &user.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

//----------------------------------------------------------------------
// Attitude State Block (ATT)
//----------------------------------------------------------------------

/// ACM Data: Attitude State Time History Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeState {
    /// Comments allowed only immediately after the ATT_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string containing the identification number for this
    /// attitude state time history.
    ///
    /// **Examples**: ATT_20160402_XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub att_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous attitude time history block. NOTE: If the message is not part of a sequence of
    /// attitude time histories or if this attitude time history is the first in a sequence of
    /// attitude time histories, then ATT_PREV_ID should be excluded from this message.
    ///
    /// **Examples**: ATT_20160401_XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub att_prev_id: Option<String>,
    /// Basis of this attitude state time history data.
    ///
    /// **Examples**: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_basis: Option<AttBasis>,
    /// Free-text field containing the identification number for the telemetry dataset, attitude
    /// determination, or simulation upon which this attitude state time history block is based.
    /// When a matching attitude determination block accompanies this attitude state time history,
    /// the ATT_BASIS_ID should match the corresponding AD_ID (see table 5-8).
    ///
    /// **Examples**: AD 1985
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub att_basis_id: Option<String>,
    /// Name of the reference frame that defines the starting point of the transformation. The set
    /// of allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: J2000
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub ref_frame_a: String,
    /// Name of the reference frame that defines the end point of the transformation. The set of
    /// allowed values is described in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub ref_frame_b: String,
    /// Number of data states included. States to be included are attitude states and optional rate
    /// states.
    ///
    /// **Examples**: 3, 4, 7
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    pub number_states: u32,
    /// Type of attitude data, selected per annex B, subsection B4. Attitude data must always be
    /// listed before rate data. The units that shall be used are given in annex B, subsection B4.
    ///
    /// **Examples**: QUATERNION, EULER_ANGLES, DCM
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[builder(into)]
    pub att_type: String,
    /// Type of rate data, selected per annex B, subsection B4. If rate data are included,
    /// NUMBER_STATES must be at least 6 to include both attitude and rate data. The units that
    /// shall be used are given in annex B, subsection B4. If the value is ANGVEL, the reference
    /// frame used shall be REF_FRAME_B.
    ///
    /// **Examples**: ANGVEL, GYRO_BIAS, Q_DOT, NONE
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub rate_type: Option<String>,
    /// Rotation sequence that defines the REF_FRAME_A to REF_FRAME_B transformation. The order of
    /// the transformation is from left to right, where the leftmost letter (X, Y, or Z) represents
    /// the rotation axis of the first rotation, the second letter (X, Y, or Z) represents the
    /// rotation axis of the second rotation, and the third letter (X, Y, or Z) represents the
    /// rotation axis of the third rotation. This keyword is applicable only if ATT_TYPE specifies
    /// the use of Euler angles.
    ///
    /// **Examples**: ZXZ, XYZ
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub euler_rot_seq: Option<RotSeq>,
    /// Data lines that consist of attitude data followed by rate data. (For the data units, see
    /// above [ATT_TYPE and RATE_TYPE keywords]).
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.5.
    #[serde(rename = "attLine", default)]
    #[builder(default)]
    pub att_lines: Vec<AttLine>,
}

impl ToKvn for AcmAttitudeState {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("ATT_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.att_id {
            writer.write_pair("ATT_ID", v);
        }
        if let Some(v) = &self.att_prev_id {
            writer.write_pair("ATT_PREV_ID", v);
        }
        if let Some(v) = &self.att_basis {
            writer.write_pair("ATT_BASIS", v);
        }
        if let Some(v) = &self.att_basis_id {
            writer.write_pair("ATT_BASIS_ID", v);
        }
        writer.write_pair("REF_FRAME_A", &self.ref_frame_a);
        writer.write_pair("REF_FRAME_B", &self.ref_frame_b);
        writer.write_pair("NUMBER_STATES", self.number_states);
        writer.write_pair("ATT_TYPE", &self.att_type);
        if let Some(v) = &self.rate_type {
            writer.write_pair("RATE_TYPE", v);
        }
        if let Some(v) = &self.euler_rot_seq {
            writer.write_pair("EULER_ROT_SEQ", v);
        }
        for line in &self.att_lines {
            writer.write_line(line.to_string());
        }
        writer.write_section("ATT_STOP");
    }
}

// Add AttBasis enum here if not in common
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttBasis {
    #[serde(rename = "PREDICTED")]
    Predicted,
    #[serde(rename = "DETERMINED_GND")]
    DeterminedGnd,
    #[serde(rename = "DETERMINED_OBC")]
    DeterminedObc,
    #[serde(rename = "SIMULATED")]
    Simulated,
}

impl std::fmt::Display for AttBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Predicted => write!(f, "PREDICTED"),
            Self::DeterminedGnd => write!(f, "DETERMINED_GND"),
            Self::DeterminedObc => write!(f, "DETERMINED_OBC"),
            Self::Simulated => write!(f, "SIMULATED"),
        }
    }
}

impl std::str::FromStr for AttBasis {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PREDICTED" => Ok(Self::Predicted),
            "DETERMINED_GND" => Ok(Self::DeterminedGnd),
            "DETERMINED_OBC" => Ok(Self::DeterminedObc),
            "SIMULATED" => Ok(Self::Simulated),
            _ => Err(crate::error::EnumParseError {
                field: "ATT_BASIS",
                value: s.to_string(),
                expected: "PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED",
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AttLine {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for AttLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Physical Description Block (PHYS)
//----------------------------------------------------------------------

/// ACM Data: Space Object Physical Characteristics Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmPhysicalDescription {
    /// Comments allowed only immediately after the PHYS_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Drag coefficient.
    ///
    /// **Examples**: 2
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff: Option<f64>,
    /// Space object total mass at the reference epoch ‘EPOCH_TZERO’.
    ///
    /// **Examples**: 750.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    /// Space object dry mass (without propellant).
    ///
    /// **Examples**: 500.0
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    /// Coordinate system for the center of pressure vector. The set of allowed values is described
    /// in annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cp_ref_frame: Option<String>,
    /// CP_REF_FRAME shall be present if CP is present. Vector location of spacecraft center of
    /// pressure for determining solar pressure torque, measured from the spacecraft center of
    /// mass. The coordinate frame is defined by CP_REF_FRAME. CP contains 3 elements, one for each
    /// axis represented in CP_REF_FRAME.
    ///
    /// **Examples**: 0.02 0.01 0.2
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cp: Option<Vector3>,
    /// Coordinate system for the inertia tensor. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub inertia_ref_frame: Option<String>,
    /// Moment of Inertia about the X axis of the spacecraft body frame defined by
    /// INERTIA_REF_FRAME.
    ///
    /// **Examples**: 1000.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixx: Option<Moment>,
    /// Moment of Inertia about the Y axis.
    ///
    /// **Examples**: 800.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyy: Option<Moment>,
    /// Moment of Inertia about the Z axis.
    ///
    /// **Examples**: 400.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub izz: Option<Moment>,
    /// Inertia Cross Product of the X & Y axes.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixy: Option<Moment>,
    /// Inertia Cross Product of the X & Z axes.
    ///
    /// **Examples**: 40.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixz: Option<Moment>,
    /// Inertia Cross Product of the Y & Z axes.
    ///
    /// **Examples**: 60.0
    ///
    /// **Units**: kg*m²
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyz: Option<Moment>,
}

impl AcmPhysicalDescription {
    fn validate(&self) -> Result<()> {
        if self.cp_ref_frame.is_some() && self.cp.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Physical Description".into(),
                field: "CP".into(),
                line: None,
            }
            .into());
        }
        if self.cp.is_some() && self.cp_ref_frame.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Physical Description".into(),
                field: "CP_REF_FRAME".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmPhysicalDescription {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("PHYS_START");
        writer.write_comments(&self.comment);
        if let Some(v) = self.drag_coeff {
            writer.write_pair("DRAG_COEFF", v);
        }
        if let Some(v) = &self.wet_mass {
            writer.write_measure("WET_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.dry_mass {
            writer.write_measure("DRY_MASS", &v.to_unit_value());
        }
        if let Some(v) = &self.cp_ref_frame {
            writer.write_pair("CP_REF_FRAME", v);
        }
        if let Some(v) = &self.cp {
            writer.write_pair("CP_X", v.elements[0]);
            writer.write_pair("CP_Y", v.elements[1]);
            writer.write_pair("CP_Z", v.elements[2]);
        }
        if let Some(v) = &self.inertia_ref_frame {
            writer.write_pair("INERTIA_REF_FRAME", v);
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
// Covariance Block (COV)
//----------------------------------------------------------------------

/// ACM Data: Covariance Time History Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmCovarianceMatrix {
    /// Comments allowed only immediately after the COV_START keyword.
    ///
    /// **Examples**: THIS is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Basis of this covariance time history data.
    ///
    /// **Examples**: PREDICTED, DETERMINED_GND, DETERMINED_OBC, SIMULATED
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    pub cov_basis: String,
    /// Reference frame of the covariance time history. The full set of values is enumerated in
    /// annex B, subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    pub cov_ref_frame: String,
    /// Indicates covariance composition. Select from annex B, subsection B6.
    ///
    /// **Examples**: ANGLE, ANGLE_GYROBIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    pub cov_type: String,
    /// Optional confidence level of the covariance matrix.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_confidence: Option<f64>,
    /// Covariance data lines (diagonal terms only). (For the data units, see annex B, subsection
    /// B6.)
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
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
        if let Some(v) = self.cov_confidence {
            writer.write_pair("COV_CONFIDENCE", v);
        }
        for line in &self.cov_lines {
            writer.write_line(line.to_string());
        }
        writer.write_section("COV_STOP");
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct CovLine {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl std::fmt::Display for CovLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Maneuver Block (MAN)
//----------------------------------------------------------------------

/// ACM Data: Maneuver Specification Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmManeuverParameters {
    /// Comments allowed only immediately after the MAN_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string containing the identification number for this
    /// maneuver.
    ///
    /// **Examples**: DH2018172
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub man_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous maneuver block. If the message is not part of a sequence of maneuvers or if this
    /// maneuver is the first in a sequence of maneuvers, then MAN_PREV_ID should be excluded from
    /// this message.
    ///
    /// **Examples**: DH2018171
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_prev_id: Option<String>,
    /// The user may specify the intention(s) of the maneuver. Multiple maneuver purposes may be
    /// provided as a comma-delimited list. While there is no CCSDS-based restriction on the value
    /// for this keyword, it is suggested to use: Attitude adjust (ATT_ADJUST); Momentum
    /// desaturation (MOM_DESAT); Pointing Request Message (PRM_ID_xxxx); Science objective
    /// (SCI_OBJ); Spin rate adjust (SPIN_RATE_ADJUST).
    ///
    /// **Examples**: ATT_ADJUST
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_purpose: Option<String>,
    /// Start time of actual maneuver, measured as a relative time with respect to EPOCH_TZERO.
    ///
    /// **Examples**: 100.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_begin_time: Option<Epoch>,
    /// End time of actual maneuver, measured as a relative time with respect to EPOCH_TZERO.
    ///
    /// **Examples**: 120.0
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_end_time: Option<Epoch>,
    /// Maneuver duration.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_duration: Option<Duration>,
    /// Actuator used for the maneuver.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actuator_used: Option<String>,
    /// Target angular momentum vector.
    ///
    /// **Units**: N*m*s
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_momentum: Option<TargetMomentum>,
    /// Coordinate system for the target momentum vector.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_mom_frame: Option<String>,
}

impl AcmManeuverParameters {
    fn validate(&self) -> Result<()> {
        if self.man_end_time.is_some() && self.man_duration.is_some() {
            return Err(ValidationError::Conflict {
                fields: vec!["MAN_END_TIME".into(), "MAN_DURATION".into()],
                line: None,
            }
            .into());
        }
        if self.target_momentum.is_some() && self.target_mom_frame.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "TARGET_MOM_FRAME".into(),
                line: None,
            }
            .into());
        }
        if self.target_momentum.is_none() && self.target_mom_frame.is_some() {
            return Err(ValidationError::MissingRequiredField {
                block: "ACM Maneuver".into(),
                field: "TARGET_MOMENTUM".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for AcmManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("MAN_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.man_id {
            writer.write_pair("MAN_ID", v);
        }
        if let Some(v) = &self.man_prev_id {
            writer.write_pair("MAN_PREV_ID", v);
        }
        if let Some(v) = &self.man_purpose {
            writer.write_pair("MAN_PURPOSE", v);
        }
        if let Some(v) = &self.man_begin_time {
            writer.write_pair("MAN_BEGIN_TIME", v);
        }
        if let Some(v) = &self.man_end_time {
            writer.write_pair("MAN_END_TIME", v);
        }
        if let Some(v) = &self.man_duration {
            writer.write_measure("MAN_DURATION", &v.to_unit_value());
        }
        if let Some(v) = &self.actuator_used {
            writer.write_pair("ACTUATOR_USED", v);
        }
        if let Some(v) = &self.target_momentum {
            writer.write_pair("TARGET_MOM_X", v.elements[0]);
            writer.write_pair("TARGET_MOM_Y", v.elements[1]);
            writer.write_pair("TARGET_MOM_Z", v.elements[2]);
        }
        if let Some(v) = &self.target_mom_frame {
            writer.write_pair("TARGET_MOM_FRAME", v);
        }
        // if let Some(v) = &self.target_attitude ...
        writer.write_section("MAN_STOP");
    }
}

//----------------------------------------------------------------------
// Attitude Determination Block (AD)
//----------------------------------------------------------------------

/// ACM Data: Attitude Determination Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmAttitudeDetermination {
    /// Comments allowed only immediately after the AD_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Optional alphanumeric free-text string for this attitude determination.
    ///
    /// **Examples**: AD_20190101
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ad_id: Option<String>,
    /// Optional alphanumeric free-text string containing the identification number for the
    /// previous attitude determination block. NOTE: If the message is not part of a sequence of
    /// attitude determination blocks or if this attitude determination block is the first in a
    /// sequence of attitude determination blocks, then AD_PREV_ID should be excluded from this
    /// message.
    ///
    /// **Examples**: AD_20190100
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ad_prev_id: Option<String>,
    /// Type of attitude determination method used. (For further description, see annex B,
    /// subsection B5.)
    ///
    /// **Examples**: EKF, TRIAD, BATCH
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ad_method: Option<String>,
    /// Source of attitude estimate, whether from a ground based estimator or onboard estimator.
    ///
    /// **Examples**: GND, OBC
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub attitude_source: Option<String>,
    /// Number of states if EKF, BATCH, or FILTER SMOOTHER is specified.
    ///
    /// **Examples**: 3, 6, 7
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_states: Option<u32>,
    /// Type of attitude states if EKF, BATCH, or FILTER SMOOTHER is specified.
    ///
    /// **Examples**: QUATERNION
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub attitude_states: Option<String>,
    /// Indicates covariance composition. Select from annex B, subsection B6.
    ///
    /// **Examples**: ANGLE, ANGLE_GYROBIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_type: Option<String>,
    /// Epoch of the attitude determination.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_epoch: Option<Epoch>,
    /// Name of the reference frame that defines the starting point of the transformation described
    /// by the attitude state in the estimator. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: J2000
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_a: Option<String>,
    /// Name of the reference frame that defines the ending point of the transformation described
    /// by the attitude state in the estimator. The set of allowed values is described in annex B,
    /// subsection B3.
    ///
    /// **Examples**: SC_BODY_1
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_b: Option<String>,
    /// Type of attitude data, selected per annex B, subsection B4. Attitude states must always be
    /// listed before rate states.
    ///
    /// **Examples**: QUATERNION
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attitude_type: Option<String>,
    /// Type of rate state included in the estimator. If rate states are included, attitude_states
    /// must be at least 6 to include both attitude states and rate states.
    ///
    /// **Examples**: ANGVEL, GYRO_BIAS
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_states: Option<String>,
    /// Rate random walk if RATE_STATES=GYRO_BIAS.
    ///
    /// **Examples**: 3.7e-7
    ///
    /// **Units**: deg/s^1.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_u: Option<SigmaU>,
    /// Angle random walk if RATE_STATES=GYRO_BIAS.
    ///
    /// **Examples**: 1.3e-5
    ///
    /// **Units**: deg/s^0.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_v: Option<SigmaV>,
    /// Process noise standard deviation if RATE_STATES=ANG_VEL.
    ///
    /// **Examples**: 5.1E-06
    ///
    /// **Units**: deg/s^1.5
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_process_noise_stddev: Option<SigmaU>,
    /// Sensor data blocks.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(rename = "sensor", default)]
    pub sensors: Vec<AcmSensor>,
}

impl AcmAttitudeDetermination {
    fn validate(&self) -> Result<()> {
        if self.sensors.is_empty() {
            return Ok(());
        }

        let mut numbers: Vec<u32> = self.sensors.iter().map(|s| s.sensor_number).collect();
        numbers.sort_unstable();

        for window in numbers.windows(2) {
            if let [prev, next] = *window {
                if next == prev {
                    return Err(ValidationError::InvalidValue {
                        field: "SENSOR_NUMBER".into(),
                        value: format!("{:?}", numbers),
                        expected: "unique values".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }

        Ok(())
    }
}

impl ToKvn for AcmAttitudeDetermination {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("AD_START");
        writer.write_comments(&self.comment);
        if let Some(v) = &self.ad_id {
            writer.write_pair("AD_ID", v);
        }
        if let Some(v) = &self.ad_prev_id {
            writer.write_pair("AD_PREV_ID", v);
        }
        if let Some(v) = &self.ad_method {
            writer.write_pair("AD_METHOD", v);
        }
        if let Some(v) = &self.attitude_source {
            writer.write_pair("ATTITUDE_SOURCE", v);
        }
        if let Some(v) = &self.number_states {
            writer.write_pair("NUMBER_STATES", v);
        }
        if let Some(v) = &self.attitude_states {
            writer.write_pair("ATTITUDE_STATES", v);
        }
        if let Some(v) = &self.cov_type {
            writer.write_pair("COV_TYPE", v);
        }
        if let Some(v) = &self.ad_epoch {
            writer.write_pair("AD_EPOCH", v);
        }
        if let Some(v) = &self.ref_frame_a {
            writer.write_pair("REF_FRAME_A", v);
        }
        if let Some(v) = &self.ref_frame_b {
            writer.write_pair("REF_FRAME_B", v);
        }
        if let Some(v) = &self.attitude_type {
            writer.write_pair("ATTITUDE_TYPE", v);
        }
        if let Some(v) = &self.rate_states {
            writer.write_pair("RATE_STATES", v);
        }
        if let Some(v) = &self.sigma_u {
            writer.write_measure("SIGMA_U", v);
        }
        if let Some(v) = &self.sigma_v {
            writer.write_measure("SIGMA_V", v);
        }
        if let Some(v) = &self.rate_process_noise_stddev {
            writer.write_measure("RATE_PROCESS_NOISE_STDDEV", v);
        }
        for sensor in &self.sensors {
            sensor.write_kvn(writer);
        }
        writer.write_section("AD_STOP");
    }
}

/// ACM Data: Sensor Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AcmSensor {
    /// Comments allowed only immediately after the SENSOR_START keyword.
    ///
    /// **Examples**: This is a comment.
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Sensor number. Multiple sensors may be included, with each having a unique, ascending
    /// number.
    ///
    /// **Examples**: 1, 2, 3
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    pub sensor_number: u32,
    /// Type of sensor used in estimation.
    ///
    /// **Examples**: AST, DSS, GYRO
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_used: Option<String>,
    /// Standard deviation of sensor noise.
    ///
    /// **Examples**: 0.0097 0.0097
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_noise_stddev: Option<SensorNoise>,
    /// Frequency of sensor data.
    ///
    /// **Examples**: 5
    ///
    /// **Units**: Hz
    ///
    /// **CCSDS Reference**: 504.0-B-2, Section 5.3.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor_frequency: Option<f64>,
}

impl ToKvn for AcmSensor {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_section("SENSOR_START");
        writer.write_comments(&self.comment);
        writer.write_pair("SENSOR_NUMBER", self.sensor_number);
        if let Some(v) = &self.sensor_used {
            writer.write_pair("SENSOR_USED", v);
        }
        if let Some(v) = &self.sensor_noise_stddev {
            let val_str = v
                .values
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(u) = &v.units {
                writer.write_pair("SENSOR_NOISE_STDDEV", format!("{} [{}]", val_str, u));
            } else {
                writer.write_pair("SENSOR_NOISE_STDDEV", val_str);
            }
        }
        if let Some(v) = self.sensor_frequency {
            writer.write_pair("SENSOR_FREQUENCY", v);
        }
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

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_acm_kvn() -> String {
        r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2022-11-04T17:22:31
ORIGINATOR = NASA/JPL
META_START
OBJECT_NAME = MARS GLOBAL SURVEYOR
TIME_SYSTEM = UTC
EPOCH_TZERO = 2002-11-04T17:22:31
META_STOP
ATT_START
REF_FRAME_A = EME2000
REF_FRAME_B = SC_BODY_1
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0.5 0.5 0.5 0.5
ATT_STOP
"#
        .to_string()
    }

    #[test]
    fn parse_acm_success() {
        let kvn = sample_acm_kvn();
        let acm = Acm::from_kvn(&kvn).expect("ACM parse failed");

        assert_eq!(acm.version, "2.0");
        assert_eq!(
            acm.body.segment.metadata.object_name,
            "MARS GLOBAL SURVEYOR"
        );
        assert_eq!(acm.body.segment.data.att.len(), 1);
        assert_eq!(acm.body.segment.data.att[0].att_lines.len(), 1);
    }

    #[test]
    fn test_acm_multiple_att_blocks() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0 0 0 1
ATT_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = INSTRUMENT
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0 0 0 1
ATT_STOP
"#;
        let acm = Acm::from_kvn(kvn).unwrap();
        assert_eq!(acm.body.segment.data.att.len(), 2);
    }

    #[test]
    fn test_acm_missing_mandatory_metadata() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
ATT_START
REF_FRAME_A = GCRF
REF_FRAME_B = SC_BODY
ATT_TYPE = QUATERNION
NUMBER_STATES = 4
0.0 0 0 0 1
ATT_STOP
"#;
        // Missing OBJECT_NAME
        assert!(Acm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_acm_physical_block() {
        let kvn = r#"CCSDS_ACM_VERS = 2.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
OBJECT_NAME = SAT1
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
PHYS_START
WET_MASS = 1000 [kg]
DRY_MASS = 500 [kg]
PHYS_STOP
"#;
        let acm = Acm::from_kvn(kvn).unwrap();
        let phys = acm.body.segment.data.phys.as_ref().unwrap();
        assert_eq!(phys.wet_mass.as_ref().unwrap().value, 1000.0);
    }
}
