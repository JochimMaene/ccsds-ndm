// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdmHeader;
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::*;
use fast_float;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//----------------------------------------------------------------------
// Root OCM Structure
//----------------------------------------------------------------------

/// Orbit Comprehensive Message (OCM).
///
/// An OCM specifies position and velocity of either a single object or an en masse parent/child
/// deployment scenario stemming from a single object. The OCM aggregates and extends OPM, OEM,
/// and OMM content in a single comprehensive hybrid message.
///
/// Key features:
/// - Support for single object or parent/child deployment scenarios.
/// - Aggregation of OPM, OMM, and OEM content.
/// - Extensive optional content including physical properties, covariance, maneuvers, and
///   perturbations.
/// - Well-suited for exchanges involving automated interaction and large object catalogs.
///
/// **CCSDS Reference**: 502.0-B-3, Section 6.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "ocm")]
pub struct Ocm {
    pub header: OdmHeader,
    pub body: OcmBody,
    #[serde(rename = "@id")]
    #[builder(into)]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(into)]
    pub version: String,
}

impl crate::traits::Validate for Ocm {
    fn validate(&self) -> Result<()> {
        Ocm::validate(self)
    }
}

impl Ndm for Ocm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let ocm = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Ocm, &ocm)?;
        Ok(ocm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let ocm: Self = crate::xml::from_str_with_context(xml, "OCM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Ocm, &ocm)?;
        Ok(ocm)
    }
}

impl Ocm {
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.segment.validate(&self.header)
    }
}

impl ToKvn for Ocm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_pair("CCSDS_OCM_VERS", &self.version);
        self.header.write_kvn(writer);
        self.body.write_kvn(writer);
    }
}

impl OcmSegment {
    pub fn validate(&self, _header: &OdmHeader) -> Result<()> {
        self.data.validate(&self.metadata)
    }
}

impl OcmData {
    pub fn validate(&self, _metadata: &OcmMetadata) -> Result<()> {
        for traj in &self.traj {
            traj.validate()?;
        }
        if let Some(phys) = &self.phys {
            phys.validate()?;
        }
        for cov in &self.cov {
            cov.validate()?;
        }
        for man in &self.man {
            man.validate()?;
        }
        if let Some(pert) = &self.pert {
            pert.validate()?;
        }
        if let Some(od) = &self.od {
            od.validate()?;
        }
        OcmTrajState::validate_all(&self.traj)?;
        Ok(())
    }
}

impl OcmPhysicalDescription {
    fn validate(&self) -> Result<()> {
        if let Some(v) = self.drag_coeff_nom {
            if v <= 0.0 {
                return Err(ValidationError::OutOfRange {
                    name: "DRAG_COEFF_NOM".into(),
                    value: v.to_string(),
                    expected: "> 0".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

impl OcmTrajState {
    fn validate(&self) -> Result<()> {
        if self.traj_lines.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("TRAJ"),
                field: Cow::Borrowed("trajLine"),
                line: None,
            }
            .into());
        }
        if let Some(rev) = self.orb_revnum {
            if rev < 0.0 {
                return Err(ValidationError::Generic {
                    message: Cow::Borrowed("ORB_REVNUM must be non-negative"),
                    line: None,
                }
                .into());
            }
        }

        let traj_type = self.traj_type.trim().to_uppercase();

        if let Some(units) = &self.traj_units {
            let unit_list = parse_units_list(units);
            if let Some(expected_units) = expected_traj_units(&traj_type) {
                if unit_list.len() == expected_units.len() {
                    for (unit, expected) in unit_list.iter().zip(expected_units.iter()) {
                        if !unit_matches_expected(unit, expected) {
                            return Err(ValidationError::InvalidValue {
                                field: Cow::Borrowed("TRAJ_UNITS"),
                                value: units.clone(),
                                expected: format!("expected {} units", traj_type).into(),
                                line: None,
                            }
                            .into());
                        }
                    }
                }
            } else if traj_type == "DELAUNAY" {
                let has_angle_unit = unit_list.iter().any(|u| {
                    let u = u.to_ascii_lowercase();
                    u.contains("rad") || u.contains("deg") || u.contains("rev")
                });
                if !has_angle_unit {
                    return Err(ValidationError::InvalidValue {
                        field: Cow::Borrowed("TRAJ_UNITS"),
                        value: units.clone(),
                        expected: "expected angular units for DELAUNAY".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }
        Ok(())
    }

    // Example of cross-block validation (not strictly required by spec but good practice)
    fn validate_all(_trajs: &[OcmTrajState]) -> Result<()> {
        // Could check for overlapping time spans or duplicate IDs
        Ok(())
    }
}

impl OcmCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if self.cov_lines.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("COV"),
                field: Cow::Borrowed("covLine"),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl OcmManeuverParameters {
    fn validate(&self) -> Result<()> {
        if self.man_lines.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("MAN"),
                field: Cow::Borrowed("manLine"),
                line: None,
            }
            .into());
        }
        let tokens = parse_csv_list(&self.man_composition);
        let time_tags = tokens.iter().filter(|t| is_man_time_tag(t)).count();
        let expected = tokens.len().saturating_sub(time_tags);
        if expected > 0 {
            for line in &self.man_lines {
                if line.values.len() < expected {
                    return Err(ValidationError::InvalidValue {
                        field: Cow::Borrowed("MAN_COMPOSITION"),
                        value: format!("{} ({} values)", self.man_composition, line.values.len()),
                        expected: format!("{} values per line", expected).into(),
                        line: None,
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}

fn expected_traj_units(traj_type: &str) -> Option<Vec<&'static str>> {
    match traj_type {
        "CARTP" => Some(vec!["km", "km", "km"]),
        "CARTPV" => Some(vec!["km", "km", "km", "km/s", "km/s", "km/s"]),
        "CARTPVA" => Some(vec![
            "km", "km", "km", "km/s", "km/s", "km/s", "km/s**2", "km/s**2", "km/s**2",
        ]),
        _ => None,
    }
}

fn unit_matches_expected(unit: &str, expected: &str) -> bool {
    let u = unit.trim().to_ascii_lowercase();
    match expected {
        "km" => u == "km",
        "km/s" => u == "km/s",
        "km/s**2" => u == "km/s**2" || u == "km/s^2",
        _ => false,
    }
}

fn parse_units_list(units: &str) -> Vec<String> {
    let trimmed = units.trim().trim_start_matches('[').trim_end_matches(']');
    trimmed
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

fn parse_csv_list(value: &str) -> Vec<String> {
    value
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn is_man_time_tag(value: &str) -> bool {
    let v = value.trim().to_ascii_uppercase();
    matches!(v.as_str(), "EPOCH" | "TIME_ABSOLUTE" | "TIME_RELATIVE")
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The body of the OCM, containing a single segment.
///
/// This struct serves as a container for the `OcmSegment`, which holds the
/// metadata and data for the Orbit Comprehensive Message.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmMetadata {
    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Metadata section; see 7.8 for comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the name of the object. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from either
    /// the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
    /// Object name and international designator of the participant), the spacecraft operator,
    /// or a State Actor or commercial Space Situational Awareness (SSA) provider maintaining
    /// the ‘CATALOG_NAME’ space catalog. If OBJECT_NAME is not listed in reference `[3]` or the
    /// content is either unknown (uncorrelated) or cannot be disclosed, the value should be
    /// set to UNKNOWN (or this keyword omitted).
    ///
    /// **Examples**: SPOT-7, ENVISAT, IRIDIUM NEXT-8, INTELSAT G-15, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub object_name: Option<String>,
    /// Free-text field containing an international designator for the object as assigned by
    /// the UN Committee on Space Research (COSPAR). Such designator values shall have the
    /// following COSPAR format: YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit
    /// serial number of launch in year YYYY (with leading zeros). P{PP} = At least one capital
    /// letter for the identification of the part brought into space by the launch. If the
    /// object has no international designator or the content is either unknown (uncorrelated)
    /// or cannot be disclosed, the value should be set to UNKNOWN (or this keyword omitted).
    /// NOTE—The international designator was typically specified by 'OBJECT_ID' in the OPM,
    /// OMM, and OEM.
    ///
    /// **Examples**: 2000-052A, 1996-068A, 2000-053A, 1996-008A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub international_designator: Option<String>,
    /// Free-text field containing the satellite catalog source (or source agency or operator,
    /// value to be drawn from the SANA registry list of Space Object Catalogs at
    /// <https://sanaregistry.org/r/space_object_catalog>, or alternatively, from the list of
    /// organizations listed in the 'Abbreviation' column of the SANA Organizations registry at
    /// <https://www.sanaregistry.org/r/organizations>) from which 'OBJECT_DESIGNATOR' was
    /// obtained.
    ///
    /// **Examples**: CSPOC, RFSA, ESA, COMSPOC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub catalog_name: Option<String>,
    /// Free-text field specification of the unique satellite identification designator for the
    /// object, as reflected in the catalog whose name is 'CATALOG_NAME'. If the ID is not known
    /// (uncorrelated object) or cannot be disclosed, 'UNKNOWN' may be used (or this keyword
    /// omitted).
    ///
    /// **Examples**: 22444, 18SPCS 18571, 2147483648_04ae[...]d84c, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub object_designator: Option<String>,
    /// Free-text comma-delimited field containing alternate name(s) of this space object,
    /// including assigned names used by spacecraft operator, State Actors, commercial SSA
    /// providers, and/or media.
    ///
    /// **Examples**: SV08, IN8
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub alternate_names: Option<String>,
    /// Free-text field containing originator or programmatic Point-of-Contact (POC) for OCM.
    ///
    /// **Examples**: Mr. Rodgers
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_poc: Option<String>,
    /// Free-text field containing contact position of the originator PoC.
    ///
    /// **Examples**: Flight Dynamics, Mission Design Lead
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_position: Option<String>,
    /// Free-text field containing originator PoC phone number.
    ///
    /// **Examples**: +12345678901
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_phone: Option<String>,
    /// Free-text field containing originator PoC email address.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_email: Option<String>,
    /// Free-text field containing originator's physical address information for OCM creator
    /// (suggest comma-delimited address lines).
    ///
    /// **Examples**: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub originator_address: Option<String>,
    /// Free-text field containing the creating agency or operator (value should be drawn from
    /// the 'Abbreviation' column of the SANA Organizations registry at
    /// <https://www.sanaregistry.org/r/organizations>).
    ///
    /// **Examples**: NASA, ESA, JAXA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_org: Option<String>,
    /// Free-text field containing technical PoC for OCM.
    ///
    /// **Examples**: Maxwell Smart
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_poc: Option<String>,
    /// Free-text field containing contact position of the technical PoC.
    ///
    /// **Examples**: Flight Dynamics, Mission Design Lead
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_position: Option<String>,
    /// Free-text field containing technical PoC phone number.
    ///
    /// **Examples**: +49615130312
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_phone: Option<String>,
    /// Free-text field containing technical PoC email address.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.ORG
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_email: Option<String>,
    /// Free-text field containing technical PoC physical address information for OCM creator
    /// (suggest comma-delimited address lines).
    ///
    /// **Examples**: 5040 Spaceflight Ave., Cocoa Beach, FL, USA, 12345
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tech_address: Option<String>,
    /// Free-text field containing an ID that uniquely identifies the previous message from
    /// this message originator for this space object. The format and content of the message
    /// identifier value are at the discretion of the originator. NOTE—One may provide the
    /// previous message ID without supplying the 'PREVIOUS_MESSAGE_EPOCH' keyword, and vice
    /// versa.
    ///
    /// **Examples**: OCM 201113719184, ABC-12_33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub previous_message_id: Option<String>,
    /// Free-text field containing an ID that uniquely identifies the next message from this
    /// message originator for this space object. The format and content of the message
    /// identifier value are at the discretion of the originator. NOTE—One may provide the next
    /// message ID without supplying the ‘NEXT_MESSAGE_EPOCH' keyword, and vice versa.
    ///
    /// **Examples**: OCM 201113719186, ABC-12_35
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub next_message_id: Option<String>,
    /// Free-text field containing a unique identifier of Attitude Data Message (ADM)
    /// (reference `[10]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// **Examples**: ADM_MSG_35132.txt, ADM_ID_0572
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub adm_msg_link: Option<String>,
    /// Free-text field containing a unique identifier of Conjunction Data Message (CDM)
    /// (reference `[14]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// **Examples**: CDM_MSG_35132.txt, CDM_ID_8257
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cdm_msg_link: Option<String>,
    /// Free-text field containing a unique identifier of Pointing Request Message (PRM)
    /// (reference `[13]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// **Examples**: PRM_MSG_35132.txt, PRM_ID_6897
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub prm_msg_link: Option<String>,
    /// Free-text field containing a unique identifier of Reentry Data Message (RDM)
    /// (reference `[12]`) that are linked (relevant) to this Orbit Data Message.
    ///
    /// **Examples**: RDM_MSG_35132.txt, RDM_ID_1839
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub rdm_msg_link: Option<String>,
    /// Free-text string containing a comma-separated list of file name(s) and/or associated
    /// identification number(s) of Tracking Data Message (TDM) (reference `[9]`) observations
    /// upon which this OD is based.
    ///
    /// **Examples**: TDM_MSG_37.txt, TDM_835, TDM_836
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub tdm_msg_link: Option<String>,
    /// Free-text field containing the operator of the space object.
    ///
    /// **Examples**: INTELSAT
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub operator: Option<String>,
    /// Free-text field containing the owner of the space object.
    ///
    /// **Examples**: SIRIUS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub owner: Option<String>,
    /// Free-text field containing the name of the country, country code, or country
    /// abbreviation where the space object owner is based.
    ///
    /// **Examples**: US, SPAIN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub country: Option<String>,
    /// Free-text field containing the name of the constellation to which this space object
    /// belongs.
    ///
    /// **Examples**: SPIRE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub constellation: Option<String>,
    /// Specification of the type of object. Select from the accepted set of values indicated
    /// in annex B, subsection B11.
    ///
    /// **Examples**: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    /// Time system for all absolute time stamps in this OCM including EPOCH_TZERO. Select from
    /// the accepted set of values indicated in annex B, subsection B3. This field is used by
    /// all OCM data blocks. If the SCLK timescale is selected, then 'EPOCH_TZERO' shall be
    /// interpreted as the spacecraft clock epoch and both SCLK_OFFSET_AT_EPOCH and
    /// SCLK_SEC_PER_SI_SEC shall be supplied.
    ///
    /// **Examples**: UTC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[builder(into)]
    pub time_system: String,
    /// Default epoch to which all relative times are referenced in data blocks (for format
    /// specification, see 7.5.10). The time scale of EPOCH_TZERO is controlled via the
    /// ‘TIME_SYSTEM' keyword, with the exception that for the SCLK timescale, EPOCH_TZERO
    /// shall be interpreted as being in the UTC timescale. This field is used by all OCM data
    /// blocks.
    ///
    /// **Examples**: 2001-11-06T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    pub epoch_tzero: Epoch,
    /// Specification of the operational status of the space object. Select from the accepted
    /// set of values indicated in annex B, subsection B12.
    ///
    /// **Examples**: OPERATIONAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ops_status: Option<String>,
    /// Specification of the type of orbit. Select from the accepted set of values indicated in
    /// annex B, subsection B14.
    ///
    /// **Examples**: GEO, LEO
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub orbit_category: Option<String>,
    /// Comma-delimited list of elements of information data blocks included in this message.
    /// The order shall be the same as the order of the data blocks in the message. Values shall
    /// be confined to the following list: ORB, PHYS, COV, MAN, PERT, OD, and USER. If the OCM
    /// contains multiple ORB, COV, or MAN data blocks (as allowed by table 6-1), the
    /// corresponding ORB, COV, or MAN entry shall be duplicated to match.
    ///
    /// **Examples**: ORB, ORB, PHYS, COV, MAN, MAN, PERT, OD, USER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ocm_data_elements: Option<String>,
    /// Defines the number of spacecraft clock counts existing at EPOCH_TZERO. This is only
    /// used if the SCLK timescale is employed by the user.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sclk_offset_at_epoch: Option<TimeOffset>,
    /// Defines the current number of clock seconds occurring during one SI second. It should be
    /// noted that this clock rate may vary with time and is the current approximate value.
    /// This is only used if the SCLK timescale is employed by the user.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sclk_sec_per_si_sec: Option<Duration>,
    /// Creation epoch of the previous message from this originator for this space object. (For
    /// format specification, see 7.5.10.) NOTE—One may provide the previous message epoch
    /// without supplying the PREVIOUS_MESSAGE_ID, and vice versa.
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_epoch: Option<Epoch>,
    /// Anticipated (or actual) epoch of the next message from this originator for this space
    /// object. (For format specification, see 7.5.10.) NOTE—One may provide the next message
    /// epoch without supplying the NEXT_MESSAGE_ID, and vice versa.
    ///
    /// **Examples**: 2001-11-07T11:17:33
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_epoch: Option<Epoch>,
    /// Time of the earliest data contained in the OCM, specified as either a relative or
    /// absolute time tag.
    ///
    /// **Examples**: 2001-11-06T00:00:00
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Epoch>,
    /// Time of the latest data contained in the OCM, specified as either a relative or absolute
    /// time tag.
    ///
    /// **Examples**: 2001-11-08T00:00:00
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Epoch>,
    /// Span of time that the OCM covers, measured in days. TIME_SPAN is defined as
    /// (STOP_TIME-START_TIME), measured in days, irrespective of whether START_TIME or
    /// STOP_TIME are provided by the message creator.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_span: Option<DayInterval>,
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
    /// 1958) as modeled by the message originator at epoch 'EPOCH_TZERO'.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taimutc_at_tzero: Option<TimeOffset>,
    /// Epoch of next leap second, specified as an absolute time tag.
    ///
    /// **Examples**: 2016-12-31T23:59:60
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_epoch: Option<Epoch>,
    /// Difference (TAI – UTC) in seconds (i.e., total number of leap seconds elapsed since
    /// 1958) incorporated by the message originator at epoch 'NEXT_LEAP_EPOCH'. This keyword
    /// should be provided if NEXT_LEAP_EPOCH is supplied.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_leap_taimutc: Option<TimeOffset>,
    /// Difference (UT1 – UTC) in seconds, as modeled by the originator at epoch 'EPOCH_TZERO'.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ut1mutc_at_tzero: Option<TimeOffset>,
    /// Free-text field specifying the source and version of the message originator's Earth
    /// Orientation Parameters (EOP) used in the creation of this message, including leap
    /// seconds, TAI – UT1, etc.
    ///
    /// **Examples**: CELESTRAK_20201028
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub eop_source: Option<String>,
    /// Free-text field specifying the method used to select or interpolate sequential EOP data.
    ///
    /// **Examples**: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub interp_method_eop: Option<String>,
    /// Free-text field specifying the source and version of the message originator's celestial
    /// body (e.g., Sun/Earth/Planetary) ephemeris data used in the creation of this message.
    ///
    /// **Examples**: JPL_DE_FILES
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmData {
    /// List of trajectory state time history blocks.
    #[serde(rename = "traj", default)]
    #[builder(default)]
    pub traj: Vec<OcmTrajState>,
    /// Space object physical characteristics.
    #[serde(rename = "phys", default, skip_serializing_if = "Option::is_none")]
    pub phys: Option<OcmPhysicalDescription>,
    /// List of covariance time history blocks.
    #[serde(rename = "cov", default)]
    #[builder(default)]
    pub cov: Vec<OcmCovarianceMatrix>,
    /// List of maneuver specifications.
    #[serde(rename = "man", default)]
    #[builder(default)]
    pub man: Vec<OcmManeuverParameters>,
    /// Perturbation parameters.
    #[serde(rename = "pert", default, skip_serializing_if = "Option::is_none")]
    pub pert: Option<OcmPerturbations>,
    /// Orbit determination data.
    #[serde(rename = "od", default, skip_serializing_if = "Option::is_none")]
    pub od: Option<OcmOdParameters>,
    /// User-defined parameters.
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
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
                writer.write_user_defined(&p.parameter, &p.value);
            }
            writer.write_section("USER_STOP");
        }
    }
}

//----------------------------------------------------------------------
// 1. Trajectory State
//----------------------------------------------------------------------

/// A block of trajectory state data, which can be a time history of states.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmTrajState {
    /// Comments (a contiguous set of one or more comment lines may be provided in the
    /// Trajectory State Time History section only immediately after the TRAJ_START keyword;
    /// see 7.8 for comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the identification number for this trajectory state time
    /// history block.
    ///
    /// **Examples**: TRAJ_20160402_XYZ
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub traj_id: Option<String>,
    /// Free-text field containing the identification number for the previous trajectory state
    /// time history, contained either within this message or presented in a previous OCM.
    /// NOTE—If this message is not part of a sequence of orbit time histories or if this
    /// trajectory state time history is the first in a sequence of orbit time histories, then
    /// TRAJ_PREV_ID should be excluded from this message.
    ///
    /// **Examples**: ORB20160305A
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub traj_prev_id: Option<String>,
    /// Free-text field containing the identification number for the next trajectory state
    /// time history, contained either within this message, or presented in a future OCM.
    /// NOTE—If this message is not part of a sequence of orbit time histories or if this
    /// trajectory state time history is the last in a sequence of orbit time histories, then
    /// TRAJ_NEXT_ID should be excluded from this message.
    ///
    /// **Examples**: ORB20160305C
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub traj_next_id: Option<String>,
    /// The basis of this trajectory state time history data. This is a free-text field with the
    /// following suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from
    /// observation-based orbit determination, reconstruction, and/or calibration. For
    /// definitive OD performed onboard spacecraft whose solutions have been telemetered to the
    /// ground for inclusion in an OCM, the TRAJ_BASIS shall be DETERMINED. c) 'TELEMETRY' when
    /// the trajectory states are read directly from telemetry, for example, based on inertial
    /// navigation systems or GNSS data. d) 'SIMULATED' for generic simulations, future mission
    /// design studies, and optimization studies. e) 'OTHER' for other bases of this data.
    ///
    /// **Examples**: PREDICTED
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_basis: Option<TrajBasis>,
    /// Free-text field containing the identification number for the telemetry dataset, orbit
    /// determination, navigation solution, or simulation upon which this trajectory state time
    /// history block is based. When a matching orbit determination block accompanies this
    /// trajectory state time history, the TRAJ_BASIS_ID should match the corresponding OD_ID
    /// (see table 6-11).
    ///
    /// **Examples**: OD_5910
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub traj_basis_id: Option<String>,
    /// This keyword may be used to specify the recommended interpolation method for ephemeris
    /// data in the immediately following set of ephemeris lines. PROPAGATE indicates that orbit
    /// propagation is the preferred method to obtain states at intermediate times, via either
    /// a midpoint-switching or endpoint switching approach.
    ///
    /// **Examples**: HERMITE, LINEAR, LAGRANGE, PROPAGATE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub interpolation: Option<String>,
    /// Recommended interpolation degree for ephemeris data in the immediately following set of
    /// ephemeris lines. Must be an integer value. This keyword must be provided if the
    /// 'INTERPOLATION' keyword is used and set to anything other than PROPAGATE.
    ///
    /// **Examples**: 5, 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<u32>,
    /// Free-text field containing the name of the orbit propagator used to create this
    /// trajectory state time history.
    ///
    /// **Examples**: HPOP, SP, SGP4
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub propagator: Option<String>,
    /// Origin of the orbit reference frame, which may be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter, or another reference frame center (such as a spacecraft, formation
    /// flying reference 'chief' spacecraft, etc.). Natural bodies shall be selected from the
    /// accepted set of values indicated in annex B, subsection B2. For spacecraft, it is
    /// recommended to use either the 'OBJECT_NAME' or 'INTERNATIONAL_DESIGNATOR' of the
    /// participant as catalogued in the UN Office of Outer Space Affairs designator index
    /// (reference `[3]`). Alternately, the 'OBJECT_DESIGNATOR' may be used. For other reference
    /// frame origins, this field is a free-text descriptor which may draw upon other naming
    /// conventions and sources.
    ///
    /// **Examples**: EARTH, MOON, ISS, EROS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[builder(into)]
    pub center_name: String,
    /// Reference frame of the trajectory state time history. Select from the accepted set of
    /// values indicated in annex B, subsection B4.
    ///
    /// **Examples**: ICRF3, J2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[builder(into)]
    pub traj_ref_frame: String,
    /// Epoch of the orbit data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traj_frame_epoch: Option<Epoch>,
    /// Start time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
    /// allows the message creator to introduce fictitious (but numerically smooth) data nodes
    /// following the actual data time history to support interpolation methods requiring more
    /// than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
    /// keyword and introduction of fictitious node points are optional and may not be necessary.
    /// 2. If this keyword is not supplied, then all data shall be assumed to be valid.
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_start_time: Option<Epoch>,
    /// Stop time of USEABLE time span covered by ephemeris data immediately following this
    /// metadata block. (For format specification, see 7.5.10.) NOTES 1. This optional keyword
    /// allows the message creator to introduce fictitious (but numerically smooth) data nodes
    /// following the actual data time history to support interpolation methods requiring more
    /// than two nodes (e.g., pure higher-order Lagrange interpolation methods). The use of this
    /// keyword and introduction of fictitious node points are optional and may not be necessary.
    /// 2. If this keyword is not supplied, then all data shall be assumed to be valid.
    ///
    /// **Examples**: 1996-12-18T14:28:15.1172, 1996-277T07:22:54
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub useable_stop_time: Option<Epoch>,
    /// The integer orbit revolution number associated with the first trajectory state in this
    /// trajectory state time history block. NOTE—The first ascending node crossing that occurs
    /// AFTER launch or deployment is designated to be the beginning of orbit revolution number
    /// = one ('1').
    ///
    /// **Examples**: 1500, 30007
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orb_revnum: Option<f64>,
    /// Specifies the message creator's basis for their orbit revolution counter, with '0',
    /// designating that the first launch or deployment trajectory state corresponds to a
    /// revolution number of 0.XXXX, where XXXX represents the fraction of an orbit revolution
    /// measured from the equatorial plane, and orbit revolution 1.0 begins at the very next
    /// (subsequent) ascending node passage; '1', designating that the first launch or
    /// deployment trajectory state corresponds to a revolution number of 1.XXXX, and orbit
    /// revolution 2.0 begins at the very next ascending node passage. This keyword shall be
    /// provided if ORB_REVNUM is specified.
    ///
    /// **Examples**: 0, 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orb_revnum_basis: Option<RevNumBasis>,
    /// Specifies the trajectory state type; selected per annex B, subsection B7.
    ///
    /// **Examples**: CARTP, CARTPV
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[builder(into)]
    pub traj_type: String,
    /// If orbital elements are provided, specifies whether those elements are osculating
    /// elements or mean elements, and if mean elements, which mean element definition is
    /// employed. The intent of this field is to allow the user to correctly interpret how to
    /// use the provided orbit elements and know how to use them operationally. This field is
    /// not required if one of the orbital element types selected by the "TRAJ_TYPE" keyword is
    /// Cartesian (e.g., CARTP, CARTPV, or CARTPVA) or spherical elements (e.g., LDBARV, ADBARV,
    /// or GEODETIC). Values should be selected from the accepted set indicated in annex B,
    /// subsection B13. If an alternate single- or double-averaging formulation other than that
    /// provided is used, the user may name it as mutually agreed upon by message exchange
    /// participants.
    ///
    /// **Examples**: OSCULATING, BROUWER, KOZAI
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub orb_averaging: Option<String>,
    /// A comma-delimited set of SI unit designations for each element of the trajectory state
    /// time history following the trajectory state time tag solely for informational purposes,
    /// provided as a free-text field enclosed in square brackets. When provided, each
    /// trajectory state element shall have a corresponding units entry, with non-dimensional
    /// values (such as orbit eccentricity) denoted by 'n/a'. NOTE—The listing of units via the
    /// TRAJ_UNITS keyword does not override the mandatory units specified for the selected
    /// TRAJ_TYPE (links to the relevant SANA registries provided in annex B, subsection B7).
    ///
    /// **Examples**: [km,km,km,km/s,km/s,km/s], [km,n/a,deg, deg, deg, deg]
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub traj_units: Option<String>,
    /// Contiguous set of trajectory state data lines.
    #[serde(rename = "trajLine")]
    #[builder(default)]
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
            writer.write_pair("TRAJ_BASIS", v.to_string());
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmPhysicalDescription {
    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM Space
    /// Object Physical Characteristics only immediately after the PHYS_START keyword; see 7.8
    /// for comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Free-text field containing the satellite manufacturer's name.
    ///
    /// **Examples**: BOEING
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Free-text field containing the satellite manufacturer's spacecraft bus model name.
    ///
    /// **Examples**: 702
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_model: Option<String>,
    /// Free-text field containing a comma-separated list of other space objects that this
    /// object is docked to.
    ///
    /// **Examples**: ISS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docked_with: Option<String>,
    /// Attitude-independent drag cross-sectional area (AD) facing the relative wind vector,
    /// not already incorporated into the attitude-dependent 'AREA_ALONG_OEB' parameters.
    ///
    /// **Examples**: 2.5
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_const_area: Option<Area>,
    /// Nominal drag Coefficient (CD Nom). If the atmospheric drag coefficient, CD, is set to
    /// zero, no atmospheric drag shall be considered.
    ///
    /// **Examples**: 2.2
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_coeff_nom: Option<f64>,
    /// Drag coefficient one sigma (1σ) percent uncertainty, where the actual range of drag
    /// coefficients to within 1σ shall be obtained from [1.0 ± DRAG_UNCERTAINTY/100.0] * (CD
    /// Nom). This factor is intended to allow operators to supply the nominal ballistic
    /// coefficient components while accommodating ballistic coefficient uncertainties.
    ///
    /// **Examples**: 10.0
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_uncertainty: Option<Percentage>,
    /// Space object total mass at beginning of life.
    ///
    /// **Examples**: 500
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_wet_mass: Option<Mass>,
    /// Space object total mass (including propellant, i.e., 'wet mass') at the current
    /// reference epoch 'EPOCH_TZERO'.
    ///
    /// **Examples**: 472.3
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wet_mass: Option<Mass>,
    /// Space object dry mass (without propellant).
    ///
    /// **Examples**: 300
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_mass: Option<Mass>,
    /// Parent reference frame that maps to the OEB frame via the quaternion-based
    /// transformation defined in annex F, subsection F1. Select from the accepted set of
    /// values indicated in B, subsections B4 and B5. This keyword shall be provided if
    /// OEB_Q1,2,3,4 are specified.
    ///
    /// **Examples**: ITRF1997
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_parent_frame: Option<String>,
    /// Epoch of the OEB parent frame, if OEB_PARENT_FRAME is provided and its epoch is not
    /// intrinsic to the definition of the reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_parent_frame_epoch: Option<Epoch>,
    /// q1 = e1 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e1 = 1st
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the OEB (defined in annex F, subsection F1).
    /// A value of '-999' denotes a tumbling space object.
    ///
    /// **Examples**: -0.575131822
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q1: Option<f64>,
    /// q2 = e2 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e2 = 2nd
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
    /// annex F, subsection F1). A value of '-999' denotes a tumbling space object.
    ///
    /// **Examples**: -0.280510532
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q2: Option<f64>,
    /// q3 = e3 * sin(φ/2), where per reference `[H1]`, φ = Euler rotation angle and e3 = 3rd
    /// component of Euler rotation axis for the rotation that maps from the OEB_PARENT_FRAME
    /// (defined above) to the frame aligned with the Optimally Encompassing Box (defined in
    /// annex F, subsection F1). A value of '-999' denotes a tumbling space object.
    ///
    /// **Examples**: -0.195634856
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_q3: Option<f64>,
    /// qc = cos(φ/2), where per reference `[H1]`, φ = the Euler rotation angle for the rotation
    /// that maps from the OEB_PARENT_FRAME (defined above) to the frame aligned with the
    /// Optimally Encompassing Box (annex F, subsection F1). qc shall be made non-negative by
    /// convention. A value of '-999' denotes a tumbling space object.
    ///
    /// **Examples**: 0.743144825
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_qc: Option<f64>,
    /// Maximum physical dimension (along Xoeb) of the OEB.
    ///
    /// **Examples**: 1
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_max: Option<Length>,
    /// Intermediate physical dimension (along Ŷoeb) of OEB normal to OEB_MAX direction.
    ///
    /// **Examples**: 0.5
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_int: Option<Length>,
    /// Minimum physical dimension (along Ẑoeb) of OEB in direction normal to both OEB_MAX and
    /// OEB_INT directions.
    ///
    /// **Examples**: 0.3
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oeb_min: Option<Length>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along max OEB (Xoeb) direction as
    /// defined in annex F.
    ///
    /// **Examples**: 0.15
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_max: Option<Area>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along intermediate OEB (Ŷoeb) direction
    /// as defined in annex F.
    ///
    /// **Examples**: 0.3
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_int: Option<Area>,
    /// Attitude-dependent cross-sectional area of space object (not already included in
    /// DRAG_CONST_AREA and SRP_CONST_AREA) when viewed along minimum OEB (Ẑoeb) direction as
    /// defined in annex F.
    ///
    /// **Examples**: 0.5
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_along_oeb_min: Option<Area>,
    /// Minimum cross-sectional area for collision probability estimation purposes.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_min_for_pc: Option<Area>,
    /// Maximum cross-sectional area for collision probability estimation purposes.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_max_for_pc: Option<Area>,
    /// Typical (50th percentile) cross-sectional area sampled over all space object
    /// orientations for collision probability estimation purposes.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_typ_for_pc: Option<Area>,
    /// Typical (50th percentile) effective Radar Cross Section of the space object sampled
    /// over all possible viewing angles.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs: Option<Area>,
    /// Minimum Radar Cross Section observed for this object.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs_min: Option<Area>,
    /// Maximum Radar Cross Section observed for this object.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcs_max: Option<Area>,
    /// Attitude-independent solar radiation pressure cross-sectional area (AR) facing the Sun,
    /// not already incorporated into the attitude-dependent ‘AREA_ALONG_OEB’ parameters.
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srp_const_area: Option<Area>,
    /// Nominal Solar Radiation Pressure Coefficient (CR NOM). If the solar radiation
    /// coefficient, CR, is set to zero, no solar radiation pressure shall be considered.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_coeff: Option<f64>,
    /// SRP one sigma (1σ) percent uncertainty, where the actual range of SRP coefficients to
    /// within 1σ shall be obtained from [1.0 ± 0.01*SRP_UNCERTAINTY] (CR NOM). This factor is
    /// intended to allow operators to supply the nominal ballistic coefficient components
    /// while accommodating ballistic coefficient uncertainties.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_uncertainty: Option<Percentage>,
    /// Typical (50th percentile) Visual Magnitude of the space object sampled over all
    /// possible viewing angles and sampled over all possible viewing angles and ‘normalized’
    /// as specified in informative annex F, subsection F2 to a 1 AU Sun-to-target distance,
    /// a phase angle of 0°, and a 40,000 km target-to-sensor distance (equivalent of GEO
    /// satellite tracked at 15.6° above local horizon).
    ///
    /// **Examples**: 15.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_absolute: Option<f64>,
    /// Minimum apparent Visual Magnitude observed for this space object.
    ///
    /// **Examples**: 19.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent_min: Option<f64>,
    /// Typical (50th percentile) apparent Visual Magnitude observed for this space object.
    ///
    /// **Examples**: 15.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent: Option<f64>,
    /// Maximum apparent Visual Magnitude observed for this space object. NOTE—The 'MAX' value
    /// represents the brightest observation, which associates with a lower Vmag.
    ///
    /// **Examples**: 16.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_apparent_max: Option<f64>,
    /// Typical (50th percentile) coefficient of REFLECTANCE of the space object over all
    /// possible viewing angles, ranging from 0 (none) to 1 (perfect reflectance).
    ///
    /// **Examples**: 0.7
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reflectance: Option<Probability>,
    /// Free-text specification of primary mode of attitude control for the space object.
    /// Suggested examples include: THREE_AXIS, SPIN, DUAL_SPIN, TUMBLING, GRAVITY_GRADIENT
    ///
    /// **Examples**: SPIN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_control_mode: Option<String>,
    /// Free-text specification of type of actuator for attitude control. Suggested examples
    /// include: ATT_THRUSTERS, ACTIVE_MAG_TORQUE, PASSIVE_MAG_TORQUE, REACTION_WHEELS,
    /// MOMENTUM_WHEELS, CONTROL_MOMENT_GYROSCOPE, NONE, OTHER
    ///
    /// **Examples**: ATT_THRUSTERS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_actuator_type: Option<String>,
    /// Accuracy of attitude knowledge.
    ///
    /// **Examples**: 0.3
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_knowledge: Option<Angle>,
    /// Accuracy of attitude control system (ACS) to maintain attitude, assuming attitude
    /// knowledge was perfect (i.e., deadbands).
    ///
    /// **Examples**: 2.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_control: Option<Angle>,
    /// Overall accuracy of spacecraft to maintain attitude, including attitude knowledge
    /// errors and ACS operation.
    ///
    /// **Examples**: 2.3
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub att_pointing: Option<Angle>,
    /// Average maneuver frequency, measured in the number of orbit- or attitude-adjust
    /// maneuvers per year.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: #/yr
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_maneuver_freq: Option<ManeuverFreq>,
    /// Maximum composite thrust the spacecraft can accomplish in any single body-fixed
    /// direction.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: N
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_thrust: Option<Thrust>,
    /// Total ΔV capability of the spacecraft at beginning of life.
    ///
    /// **Examples**: 1.0
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_bol: Option<Velocity>,
    /// Total ΔV remaining for the spacecraft.
    ///
    /// **Examples**: 0.2
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_remaining: Option<Velocity>,
    /// Moment of Inertia about the X-axis of the space object's primary body frame (e.g.,
    /// SC_Body_1) (see reference `[H1]`).
    ///
    /// **Examples**: 1000.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixx: Option<Moment>,
    /// Moment of Inertia about the Y-axis.
    ///
    /// **Examples**: 800.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iyy: Option<Moment>,
    /// Moment of Inertia about the Z-axis.
    ///
    /// **Examples**: 400.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub izz: Option<Moment>,
    /// Inertia Cross Product of the X & Y axes.
    ///
    /// **Examples**: 20.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixy: Option<Moment>,
    /// Inertia Cross Product of the X & Z axes.
    ///
    /// **Examples**: 40.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ixz: Option<Moment>,
    /// Inertia Cross Product of the Y & Z axes.
    ///
    /// **Examples**: 60.0
    ///
    /// **Units**: kg·m²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.6.
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmCovarianceMatrix {
    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// covariance time history section only immediately after the COV_START keyword; see 7.8
    /// for comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the identification number for this covariance time history
    /// block.
    ///
    /// **Examples**: COV_20160402_XYZ
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_id: Option<String>,
    /// Free-text field containing the identification number for the previous covariance time
    /// history, contained either within this message or presented in a previous OCM. NOTE—If
    /// this message is not part of a sequence of covariance time histories or if this
    /// covariance time history is the first in a sequence of covariance time histories, then
    /// COV_PREV_ID should be excluded from this message.
    ///
    /// **Examples**: COV_20160305a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_prev_id: Option<String>,
    /// Free-text field containing the identification number for the next covariance time
    /// history, contained either within this message, or presented in a future OCM. NOTE—If
    /// this message is not part of a sequence of covariance time histories or if this
    /// covariance time history is the last in a sequence of covariance time histories, then
    /// COV_NEXT_ID should be excluded from this message.
    ///
    /// **Examples**: COV_20160305C
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_next_id: Option<String>,
    /// Basis of this covariance time history data. This is free-text field with the following
    /// suggested values: a) 'PREDICTED'. b) 'DETERMINED' when estimated from observation-based
    /// orbit determination, reconstruction and/or calibration. For definitive OD performed
    /// onboard whose solutions have been telemetered to the ground for inclusion in an OCM,
    /// the COV_BASIS shall be considered to be DETERMINED. c) EMPIRICAL (for empirically
    /// determined such as overlap analyses). d) SIMULATED for simulation-based (including
    /// Monte Carlo) estimations, future mission design studies, and optimization studies. e)
    /// 'OTHER' for other bases of this data.
    ///
    /// **Examples**: PREDICTED, EMPIRICAL, DETERMINED, SIMULATED, OTHER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_basis: Option<CovBasis>,
    /// Free-text field containing the identification number for the orbit determination,
    /// navigation solution, or simulation upon which this covariance time history block is
    /// based. When a matching orbit determination block accompanies this covariance time
    /// history, the COV_BASIS_ID should match the corresponding OD_ID (see table 6-11).
    ///
    /// **Examples**: OD_5910
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_basis_id: Option<String>,
    /// Reference frame of the covariance time history. Select from the accepted set of values
    /// indicated in annex B, subsection B4 and B5.
    ///
    /// **Examples**: TNW_INERTIA, J2000
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[builder(into)]
    pub cov_ref_frame: String,
    /// Epoch of the covariance data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_frame_epoch: Option<Epoch>,
    /// Minimum scale factor to apply to this covariance data to achieve realism.
    ///
    /// **Examples**: 0.5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_scale_min: Option<f64>,
    /// Maximum scale factor to apply to this covariance data to achieve realism.
    ///
    /// **Examples**: 5.0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_scale_max: Option<f64>,
    /// A measure of the confidence in the covariance errors matching reality, as characterized
    /// via a Wald test, a Chi-squared test, the log of likelihood, or a numerical
    /// representation per mutual agreement.
    ///
    /// **Examples**: 50
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_confidence: Option<Percentage>,
    /// Indicates covariance composition. Select from annex B, subsections B7 and B8.
    ///
    /// **Examples**: CARTP, CARTPV, ADBARV
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[builder(into)]
    pub cov_type: String,
    /// Indicates covariance ordering as being either LTM, UTM, Full covariance, LTM covariance
    /// with cross-correlation information provided in upper triangle off-diagonal terms
    /// (LTMWCC), or UTM covariance with cross-correlation information provided in lower
    /// triangle off-diagonal terms (UTMWCC).
    ///
    /// **Examples**: LTM, UTM, FULL, LTMWCC, UTMWCC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[builder(default)]
    pub cov_ordering: CovOrder,
    /// A comma-delimited set of SI unit designations for each element of the covariance time
    /// history following the covariance time tag, solely for informational purposes, provided
    /// as a free-text field enclosed in square brackets. When provided, these units
    /// designations shall correspond to the units of the standard deviations (or square roots)
    /// of each of the covariance matrix diagonal elements (or variances), respectively, and
    /// all diagonal elements shall have a corresponding units entry, with non-dimensional
    /// values (such as dispersion in orbit eccentricity) denoted by 'n/a'. NOTE—The listing of
    /// units via the COV_UNITS keyword does not override the mandatory units specified for the
    /// selected COV_TYPE (links to the relevant SANA registries provided in annex B,
    /// subsections B7 and B8).
    ///
    /// **Examples**: [km,km,km,km/s,km/s,km/s]
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.7.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub cov_units: Option<String>,
    /// Contiguous set of covariance matrix data lines.
    #[serde(rename = "covLine")]
    #[builder(default)]
    pub cov_lines: Vec<CovLine>,
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
            writer.write_pair("COV_BASIS", v.to_string());
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
        writer.write_pair("COV_ORDERING", self.cov_ordering.to_string());
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmManeuverParameters {
    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Maneuver Specification only immediately after the MAN_START keyword; see 7.8 for
    /// comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Free-text field containing the unique maneuver identification number for this maneuver.
    /// All supplied maneuver 'constituents' within the same MAN_BASIS and MAN_REF_FRAME
    /// categories shall be added together to represent the total composite maneuver
    /// description.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
    pub man_id: String,
    /// Free-text field containing the identification number of the previous maneuver for this
    /// MAN_BASIS, contained either within this message, or presented in a previous OCM. If
    /// this message is not part of a sequence of maneuver messages or if this maneuver is the
    /// first in a sequence of maneuvers, then MAN_PREV_ID should be excluded from this
    /// message.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub man_prev_id: Option<String>,
    /// Free-text field containing the identification number of the next maneuver for this
    /// MAN_BASIS, contained either within this message, or presented in a future OCM. If this
    /// message is not part of a sequence of maneuver messages or if this maneuver is the last
    /// in a sequence of maneuvers, then MAN_NEXT_ID should be excluded from this message.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_next_id: Option<String>,
    /// Basis of this maneuver time history data, which shall be selected from one of the
    /// following values: 'CANDIDATE' for a proposed operational or a hypothetical (i.e.,
    /// mission design and optimization studies) future maneuver, 'PLANNED' for a currently
    /// planned future maneuver, 'ANTICIPATED' for a non-cooperative future maneuver that is
    /// anticipated (i.e., likely) to occur (e.g., based upon patterns-of-life analysis),
    /// 'TELEMETRY' when the maneuver is determined directly from telemetry (e.g., based on
    /// inertial navigation systems or accelerometers), 'DETERMINED' when a past maneuver is
    /// estimated from observation-based orbit determination reconstruction and/or
    /// calibration, 'SIMULATED' for generic maneuver simulations, future mission design
    /// studies, and optimization studies, 'OTHER' for other bases of this data.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_basis: Option<ManBasis>,
    /// Free-text field containing the identification number for the orbit determination,
    /// navigation solution, or simulation upon which this maneuver time history block is
    /// based. Where a matching orbit determination block accompanies this maneuver time
    /// history, the MAN_BASIS_ID should match the corresponding OD_ID (see table 6-11).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_basis_id: Option<String>,
    /// Free-text field containing the maneuver device identifier used for this maneuver. 'ALL'
    /// indicates that this maneuver represents the summed acceleration, velocity increment,
    /// or thrust imparted by any/all thrusters utilized in the maneuver.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
    pub man_device_id: String,
    /// Identifies the completion time of the previous maneuver for this MAN_BASIS.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_prev_epoch: Option<Epoch>,
    /// Identifies the start time of the next maneuver for this MAN_BASIS.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_next_epoch: Option<Epoch>,
    /// A free-text field used to specify the intention(s) of the maneuver. Multiple maneuver
    /// purposes can be provided as a comma-delimited list.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub man_purpose: Option<String>,
    /// For future maneuvers, specifies the source of the orbit and/or attitude state(s) upon
    /// which the maneuver is based. While there is no CCSDS-based restriction on the value for
    /// this free-text keyword, it is suggested to consider using TRAJ_ID and OD_ID keywords
    /// as described in tables 6-4 and 6-11, respectively, or a combination thereof.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub man_pred_source: Option<String>,
    /// Reference frame in which all maneuver vector direction data is provided in this
    /// maneuver data block. Select from the accepted set of values indicated in annex B,
    /// subsections B4 and B5. The reference frame must be the same for all data elements
    /// within a given maneuver time history block.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
    pub man_ref_frame: String,
    /// Epoch of the maneuver data reference frame, if not intrinsic to the definition of the
    /// reference frame. (See 7.5.10 for formatting rules.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man_frame_epoch: Option<Epoch>,
    /// Origin of maneuver gravitational assist body, which may be a natural solar system body
    /// (planets, asteroids, comets, and natural satellites), including any planet barycenter
    /// or the solar system barycenter. (See annex B, subsection B2, for acceptable
    /// GRAV_ASSIST_NAME values and the procedure to propose new values.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub grav_assist_name: Option<String>,
    /// Duty cycle type to use for this maneuver time history section: CONTINUOUS denotes
    /// full/continuous thrust `<default>`; TIME denotes a time-based duty cycle driven by time
    /// past a reference time and the duty cycle ON and OFF durations; TIME_AND_ANGLE denotes a
    /// duty cycle driven by the phasing/clocking of a space object body frame 'trigger'
    /// direction past a reference direction.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(default)]
    pub dc_type: ManDc,
    /// Start time of the duty cycle-based maneuver window that occurs on or prior to the
    /// actual maneuver execution start time. For example, this may identify the time at which
    /// the satellite is first placed into a special duty-cycle-based maneuver mode. This
    /// keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_win_open: Option<Epoch>,
    /// End time of the duty cycle-based maneuver window that occurs on or after the actual
    /// maneuver execution end time. For example, this may identify the time at which the
    /// satellite is taken out of a special duty-cycle-based maneuver mode. This keyword shall
    /// be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_win_close: Option<Epoch>,
    /// Minimum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
    /// even if DC_TYPE = 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_min_cycles: Option<u64>,
    /// Maximum number of 'ON' duty cycles (may override DC_EXEC_STOP). This value is optional
    /// even if DC_TYPE = 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_max_cycles: Option<u64>,
    /// Start time of the initial duty cycle-based maneuver sequence execution. DC_EXEC_START
    /// is defined to occur on or prior to the first maneuver 'ON' portion within the duty
    /// cycle sequence. DC_EXEC_START must be scheduled to occur coincident with or after
    /// DC_WIN_OPEN. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_exec_start: Option<Epoch>,
    /// End time of the final duty cycle-based maneuver sequence execution. DC_EXEC_STOP
    /// typically occurs on or after the end of the final maneuver 'ON' portion within the duty
    /// cycle sequence. DC_EXEC_STOP must be scheduled to occur coincident with or prior to
    /// DC_WIN_CLOSE. This keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_exec_stop: Option<Epoch>,
    /// Reference time for the THRUST duty cycle, specified as either time in seconds (relative
    /// to EPOCH_TZERO), or as an absolute '`<epoch>`' (see 7.5.10 for formatting rules).
    /// NOTE—Depending upon EPOCH_TZERO, DC_REF_TIME relative times may be negative. This
    /// keyword shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_ref_time: Option<Epoch>,
    /// Thruster pulse 'ON' duration, initiated at first satisfaction of the burn 'ON' time
    /// constraint or upon completion of the previous DC_TIME_PULSE_PERIOD cycle. This keyword
    /// shall be set if DC_TYPE ≠ 'CONTINUOUS'.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_time_pulse_duration: Option<Duration>,
    /// Elapsed time between the start of one pulse and the start of the next. Must be greater
    /// than or equal to DC_TIME_PULSE_DURATION. This keyword shall be set if DC_TYPE ≠
    /// 'CONTINUOUS'.
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_time_pulse_period: Option<Duration>,
    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the reference
    /// vector direction in the 'MAN_REF_FRAME' reference frame at which, when mapped into the
    /// space object's spin plane (normal to the spin axis), the duty cycle is triggered (see
    /// DC_PA_START_ANGLE for phasing). This (tripartite, or three-element vector) value shall
    /// be provided if DC_TYPE = 'TIME_AND_ANGLE'. This reference direction does not represent
    /// the duty cycle midpoint.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_ref_dir: Option<Vec3Double>,
    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body
    /// reference frame in which DC_BODY_TRIGGER will be specified. Select from the accepted
    /// set of values indicated in annex B, subsection B6. This keyword shall be set if
    /// DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub dc_body_frame: Option<String>,
    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the body frame
    /// reference vector direction in the 'DC_BODY_FRAME' reference frame at which, when its
    /// projection onto the spin plane crosses the corresponding projection of DC_REF_DIR onto
    /// the spin plane, this angle-based duty cycle is initiated (see DC_PA_START_ANGLE for
    /// phasing). This tripartite value shall be provided if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_body_trigger: Option<Vec3Double>,
    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
    /// offset of thruster pulse start, measured with respect to the occurrence of a
    /// DC_BODY_TRIGGER crossing of the DC_REF_DIR direction when both are projected into the
    /// spin plane (normal to the body spin axis). This phase angle offset can be positive or
    /// negative to allow the duty cycle to begin prior to the next crossing of the
    /// DC_REF_DIR. As this angular direction is to be used in a modulo sense, there is no
    /// requirement for the magnitude of the phase angle offset to be less than 360 degrees.
    /// This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_pa_start_angle: Option<Angle>,
    /// For phase angle thruster duty cycles (DC_TYPE=TIME_AND_ANGLE); specifies the phase angle
    /// of thruster pulse stop, measured with respect to the DC_BODY_TRIGGER crossing of the
    /// DC_REF_DIR direction when both are projected into the spin plane. This phase angle
    /// offset can be positive or negative to allow the duty cycle to end after to the next
    /// crossing of the DC_REF_DIR. As this angular direction is to be used in a modulo sense,
    /// there is no requirement for the magnitude of the phase angle offset to be less than
    /// 360 degrees. This keyword shall be set if DC_TYPE = 'TIME_AND_ANGLE'.
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_pa_stop_angle: Option<Angle>,
    /// The comma-delimited ordered set of maneuver elements of information contained on every
    /// maneuver time history line, with values selected from table 6-8. Within this maneuver
    /// data section, the maneuver composition shall include only one TIME specification
    /// (TIME_ABSOLUTE or TIME_RELATIVE).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
    pub man_composition: String,
    /// A comma-delimited set of SI unit designations for each and every element of the
    /// maneuver time history following the maneuver time tag(s), solely for informational
    /// purposes, provided as a free-text field enclosed in square brackets. When MAN_UNITS is
    /// provided, all elements of MAN_COMPOSITION AFTER the maneuver time tag(s) must have a
    /// corresponding units entry; percentages shall be denoted by '%', and control switches,
    /// non-dimensional values, and text strings shall be labelled as 'n/a'. NOTE—The listing
    /// of units via the MAN_UNITS keyword does not override the mandatory units for the
    /// selected MAN_COMPOSITION, as specified in table 6-8 or table 6-9.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub man_units: Option<String>,
    /// Maneuver time history data lines.
    #[serde(rename = "manLine")]
    #[builder(default)]
    pub man_lines: Vec<ManLine>,
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
            writer.write_pair("MAN_BASIS", v.to_string());
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
        writer.write_pair("DC_TYPE", self.dc_type.to_string());
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmPerturbations {
    /// Comments (a contiguous set of one or more comment lines may be provided in the OCM
    /// Perturbations Specification only immediately after the PERT_START keyword; see 7.8 for
    /// comment formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Name of atmosphere model, which shall be selected from the accepted set of values
    /// indicated in annex B, subsection B9.
    ///
    /// **Examples**: MSISE90, NRLMSIS00, J70, J71, JROBERTS, DTM, JB2008
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    /// The gravity model (selected from the accepted set of gravity model names indicated in
    /// annex B, subsection B10), followed by the degree (D) and order (O) of the applied
    /// spherical harmonic coefficients used in the simulation. NOTE—Specifying a zero value
    /// for 'order' (e.g., 2D 0O) denotes zonals (J2 ... JD).
    ///
    /// **Examples**: EGM-96: 36D 36O, WGS-84: 8D 0O, GGM-01: 36D 36O, TEG-4: 36D 36O
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    /// Oblate spheroid equatorial radius of the central body used in the message, if
    /// different from the gravity model.
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equatorial_radius: Option<Position>,
    /// Gravitational coefficient of attracting body (Gravitational Constant × Central Mass),
    /// if different from the gravity model.
    ///
    /// **Units**: km³/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gm: Option<Gm>,
    /// One OR MORE (N-body) gravitational perturbations bodies used. Values, listed serially
    /// in comma-delimited fashion, denote a natural solar or extra-solar system body (stars,
    /// planets, asteroids, comets, and natural satellites). NOTE—Only those entries specified
    /// under CENTER_NAME in annex B, subsection B2 are acceptable values.
    ///
    /// **Examples**: MOON, SUN, JUPITER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    /// Central body angular rotation rate, measured about the major principal axis of the
    /// inertia tensor of the central body, relating inertial, and central-body-fixed
    /// reference frames. NOTE—The rotation axis may be slightly offset from the inertial
    /// frame Z-axis definition.
    ///
    /// **Units**: deg/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub central_body_rotation: Option<AngleRate>,
    /// Central body's oblate spheroid oblateness for the polar-symmetric oblate central body
    /// model (e.g., for the Earth, it is approximately 1.0/298.257223563).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oblate_flattening: Option<f64>,
    /// Name of ocean tides model (optionally specify order or constituent effects, diurnal,
    /// semi-diurnal, etc.). This is a free-text field, so if the examples on the right are
    /// insufficient, others may be used.
    ///
    /// **Examples**: DIURNAL, SEMI-DIURNAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocean_tides_model: Option<String>,
    /// Name of solid tides model (optionally specify order or constituent effects, diurnal,
    /// semi-diurnal, etc.).
    ///
    /// **Examples**: DIURNAL, SEMI-DIURNAL
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solid_tides_model: Option<String>,
    /// Specification of the reduction theory used for precession and nutation modeling. This
    /// is a free-text field, so if the examples on the right are insufficient, others may be
    /// used.
    ///
    /// **Examples**: IAU1976/FK5, IAU2010, IERS1996
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduction_theory: Option<String>,
    /// Name of the albedo model.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub albedo_model: Option<String>,
    /// Size of the albedo grid.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub albedo_grid_size: Option<u64>,
    /// Shadow model used for Solar Radiation Pressure; dual cone uses both umbra/penumbra
    /// regions. Selected option should be one of ‘NONE’, ‘CYLINDRICAL’, ‘CONE’, or
    /// ‘DUAL_CONE’.
    ///
    /// **Examples**: NONE, CYLINDRICAL, CONE, DUAL_CONE
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_model: Option<String>,
    /// List of bodies included in shadow calculations (value(s) to be drawn from the SANA
    /// registry list of Orbit Centers at <https://sanaregistry.org/r/orbit_centers>).
    ///
    /// **Examples**: EARTH, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_bodies: Option<String>,
    /// Name of the Solar Radiation Pressure (SRP) model.
    ///
    /// **Examples**: CANNONBALL, FLAT_PLATE, BOX_WING
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srp_model: Option<String>,
    /// Space weather data source.
    ///
    /// **Examples**: NOAA
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_data_source: Option<String>,
    /// Epoch of the space weather data.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_data_epoch: Option<Epoch>,
    /// Free-text field specifying the method used to select or interpolate any and all
    /// sequential space weather data (Kp, ap, Dst, F10.7, M10.7, S10.7, Y10.7, etc.). While
    /// not constrained to specific entries, it is anticipated that the utilized method would
    /// match methods detailed in numerical analysis textbooks.
    ///
    /// **Examples**: PRECEDING_VALUE, NEAREST_NEIGHBOR, LINEAR, LAGRANGE_ORDER_5
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_interp_method: Option<String>,
    /// Fixed geomagnetic Kp index.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_kp: Option<Geomag>,
    /// Fixed geomagnetic Ap index.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_ap: Option<Geomag>,
    /// Fixed geomagnetic Dst index.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_geomag_dst: Option<Geomag>,
    /// Fixed F10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_f10p7: Option<SolarFlux>,
    /// Fixed 81-day average F10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_f10p7_mean: Option<SolarFlux>,
    /// Fixed M10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_m10p7: Option<SolarFlux>,
    /// Fixed 81-day average M10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_m10p7_mean: Option<SolarFlux>,
    /// Fixed S10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_s10p7: Option<SolarFlux>,
    /// Fixed 81-day average S10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_s10p7_mean: Option<SolarFlux>,
    /// Fixed Y10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_y10p7: Option<SolarFlux>,
    /// Fixed 81-day average Y10.7 solar flux.
    ///
    /// **Units**: SFU
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OcmOdParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Identification number for this orbit determination.
    ///
    /// **Examples**: 1
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
    pub od_id: String,
    /// Optional identification number for the previous orbit determination.
    ///
    /// **Examples**: 0
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub od_prev_id: Option<String>,
    /// Type of orbit determination method used to produce the orbit estimate.
    ///
    /// **Examples**: LEAST_SQUARES, KALMAN_FILTER
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.8.
    #[builder(into)]
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
    /// Days elapsed between first accepted observation and OD_EPOCH. NOTE—May be positive or
    /// negative.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_obs: Option<DayInterval>,
    /// Days elapsed between last accepted observation and OD_EPOCH. NOTE—May be positive or
    /// negative.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_last_obs: Option<DayInterval>,
    /// Number of days of observations recommended for the OD of the object (useful only for
    /// Batch OD systems).
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_od_span: Option<DayInterval>,
    /// Actual time span in days used for the OD of the object. NOTE—Should equal
    /// (DAYS_SINCE_FIRST_OBS - DAYS_SINCE_LAST_OBS).
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_od_span: Option<DayInterval>,
    /// The number of observations available within the actual OD time span.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_available: Option<u64>,
    /// The number of observations accepted within the actual OD time span.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obs_used: Option<u64>,
    /// The number of sensor tracks available for the OD within the actual time span (see
    /// definition of 'tracks', 1.5.2).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_available: Option<u64>,
    /// The number of sensor tracks accepted for the OD within the actual time span (see
    /// definition of 'tracks', 1.5.2).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks_used: Option<u64>,
    /// The maximum time between observations in the OD of the object.
    ///
    /// **Units**: d
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_obs_gap: Option<DayInterval>,
    /// Positional error ellipsoid 1σ major eigenvalue at the epoch of the OD.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigmaj: Option<Length>,
    /// Positional error ellipsoid 1σ intermediate eigenvalue at the epoch of the OD.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigint: Option<Length>,
    /// Positional error ellipsoid 1σ minor eigenvalue at the epoch of the OD.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_epoch_eigmin: Option<Length>,
    /// The resulting maximum predicted major eigenvalue of the 1σ positional error ellipsoid
    /// over the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_max_pred_eigmaj: Option<Length>,
    /// The resulting minimum predicted minor eigenvalue of the 1σ positional error ellipsoid
    /// over the entire TIME_SPAN of the OCM, stemming from this OD.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_min_pred_eigmin: Option<Length>,
    /// OD confidence metric, which spans 0 to 100% (useful only for Filter-based OD systems).
    /// The OD confidence metric shall be as mutually defined by message exchange
    /// participants.
    ///
    /// **Units**: %
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub od_confidence: Option<Percentage>,
    /// Generalized Dilution Of Precision for this orbit determination, based on the
    /// observability grammian as defined in references `[H15]` and `[H16]` and expressed in
    /// informative annex F, subsection F4. GDOP provides a rating metric of the observability
    /// of the element set from the OD. Alternate GDOP formations may be used as mutually
    /// defined by message exchange participants.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gdop: Option<f64>,
    /// The number of solve-for states in the orbit determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve_n: Option<u64>,
    /// Free-text comma-delimited description of the state elements solved for in the orbit
    /// determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve_states: Option<String>,
    /// The number of consider parameters used in the orbit determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consider_n: Option<u64>,
    /// Free-text comma-delimited description of the consider parameters used in the orbit
    /// determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consider_params: Option<String>,
    /// The Specific Energy Dissipation Rate, which is the amount of energy being removed from
    /// the object's orbit by the non-conservative forces. This value is an average
    /// calculated during the OD. (See annex F, subsection F7 for definition.)
    ///
    /// **Units**: W/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedr: Option<Wkg>,
    /// The number of sensors used in the orbit determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensors_n: Option<u64>,
    /// Free-text comma-delimited description of the sensors used in the orbit determination.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensors: Option<String>,
    /// (Useful/valid only for Batch OD systems.) The weighted RMS residual ratio, defined as:
    /// .. math:: \text{Weighted RMS} = \sqrt{\frac{\sum_{i=1}^{N} w_i(y_i - \hat{y}_i)^2}{N}}
    /// Where yi is the ith observation measurement, ŷi is the current estimate of yi, wi =
    /// 1/σi² is the weight (sigma) associated with the measurement at the ith time and N is
    /// the number of observations. This is a value that can generally identify the quality of
    /// the most recent vector update and is used by the analyst in evaluating the OD process.
    /// A value of 1.00 is ideal.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighted_rms: Option<NonNegativeDouble>,
    /// Comma-separated list of observation data types utilized in this orbit determination.
    /// Although this is a free-text field, it is recommended at a minimum to use data type
    /// descriptor(s) as provided in table 3-5 of the TDM standard (reference `[9]`) (excluding
    /// the DATA_START, DATA_STOP, and COMMENT keywords). Additional descriptors/detail is
    /// encouraged if the descriptors of table 3-5 are not sufficiently clear; for example, one
    /// could replace ANGLE_1 and ANGLE_2 with RADEC (e.g., from a telescope), AZEL (e.g., from
    /// a ground radar), RANGE (whether from radar or laser ranging), etc.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.10.
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
            writer.write_measure("SEDR", &v.to_unit_value());
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

impl OcmPerturbations {
    fn validate(&self) -> Result<()> {
        if let Some(v) = self.oblate_flattening {
            if v <= 0.0 {
                return Err(ValidationError::OutOfRange {
                    name: "OBLATE_FLATTENING".into(),
                    value: v.to_string(),
                    expected: "> 0".into(),
                    line: None,
                }
                .into());
            }
        }
        if let Some(v) = self.albedo_grid_size {
            if v == 0 {
                return Err(ValidationError::OutOfRange {
                    name: "ALBEDO_GRID_SIZE".into(),
                    value: v.to_string(),
                    expected: ">= 1".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

impl OcmOdParameters {
    fn validate(&self) -> Result<()> {
        for (name, val) in [
            ("OBS_AVAILABLE", self.obs_available),
            ("OBS_USED", self.obs_used),
            ("TRACKS_AVAILABLE", self.tracks_available),
            ("TRACKS_USED", self.tracks_used),
            ("SOLVE_N", self.solve_n),
            ("CONSIDER_N", self.consider_n),
            ("SENSORS_N", self.sensors_n),
        ] {
            if let Some(v) = val {
                if v == 0 {
                    return Err(ValidationError::OutOfRange {
                        name: name.into(),
                        value: v.to_string(),
                        expected: ">= 1".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }

        for (name, val) in [
            ("GDOP", self.gdop),
            ("WEIGHTED_RMS", self.weighted_rms.map(|v| v.value)),
        ] {
            if let Some(v) = val {
                if v < 0.0 {
                    return Err(ValidationError::OutOfRange {
                        name: name.into(),
                        value: v.to_string(),
                        expected: ">= 0".into(),
                        line: None,
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::traits::Ndm;

    #[test]
    fn test_ocm_validation_traj_lines() {
        let mut ocm = Ocm::builder()
            .header(
                OdmHeader::builder()
                    .originator("TEST")
                    .creation_date("2000-01-01T00:00:00".parse().unwrap())
                    .build(),
            )
            .body(
                OcmBody::builder()
                    .segment(Box::new(
                        OcmSegment::builder()
                            .metadata(
                                OcmMetadata::builder()
                                    .time_system("UTC")
                                    .epoch_tzero("2000-01-01T00:00:00".parse().unwrap())
                                    .build(),
                            )
                            .data(OcmData::default())
                            .build(),
                    ))
                    .build(),
            )
            .version("3.0")
            .build();

        let traj = OcmTrajState::builder()
            .center_name("EARTH")
            .traj_ref_frame("GCRF")
            .traj_type("CARTPV")
            .build();
        // Missing lines
        ocm.body.segment.data.traj.push(traj);
        assert!(ocm.validate().is_err());

        // Fix it
        ocm.body.segment.data.traj[0].traj_lines.push(TrajLine {
            epoch: "2000-01-01T00:00:00".to_string(),
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        });
        assert!(ocm.validate().is_ok());
    }

    #[test]
    fn test_ocm_validation_orb_revnum() {
        let mut traj = OcmTrajState::builder()
            .center_name("EARTH")
            .traj_ref_frame("GCRF")
            .traj_type("CARTPV")
            .build();
        traj.traj_lines.push(TrajLine {
            epoch: "2000-01-01T00:00:00".to_string(),
            values: vec![1.0],
        });
        traj.orb_revnum = Some(-1.0);

        assert!(traj.validate().is_err());

        traj.orb_revnum = Some(0.0);
        assert!(traj.validate().is_ok());
    }

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

    #[test]
    fn test_ocm_validation_gaps() {
        // 1. DRAG_COEFF_NOM <= 0.0
        let mut phys = OcmPhysicalDescription::default();
        phys.drag_coeff_nom = Some(-1.0);
        assert!(phys.validate().is_err());
        phys.drag_coeff_nom = Some(0.0);
        assert!(phys.validate().is_err());

        // 2. ORB_REVNUM < 0.0
        let mut traj = OcmTrajState::builder()
            .center_name("EARTH")
            .traj_ref_frame("GCRF")
            .traj_type("OSCULATING")
            .build();
        traj.orb_revnum = Some(-1.0);
        // Also needs at least one line to pass first check
        traj.traj_lines.push(TrajLine {
            epoch: "2023-01-01T00:00:00".to_string(),
            values: vec![0.0],
        });
        assert!(traj.validate().is_err());

        // 3. OcmTrajState::traj_lines empty
        traj.traj_lines.clear();
        assert!(traj.validate().is_err());

        // 4. OcmCovarianceMatrix::cov_lines empty
        let cov = OcmCovarianceMatrix::builder()
            .cov_ref_frame("GCRF")
            .cov_type("SYMMETRIC")
            .build();
        assert!(cov.validate().is_err());

        // 5. OcmManeuverParameters::man_lines empty
        let man = OcmManeuverParameters::builder()
            .man_id("MAN1")
            .man_device_id("DEV1")
            .man_ref_frame("GCRF")
            .man_composition("CHEMICAL")
            .build();
        assert!(man.validate().is_err());

        // 6. RevNumBasis::One coverage
        let mut traj = OcmTrajState::builder()
            .center_name("EARTH")
            .traj_ref_frame("GCRF")
            .traj_type("OSCULATING")
            .build();
        traj.orb_revnum_basis = Some(RevNumBasis::One);
        traj.traj_lines.push(TrajLine {
            epoch: "2023-01-01T00:00:00".to_string(),
            values: vec![0.0],
        });
        let mut writer = crate::kvn::ser::KvnWriter::new();
        use crate::traits::ToKvn;
        traj.write_kvn(&mut writer);
        let kvn = writer.finish();
        assert!(kvn.contains("ORB_REVNUM_BASIS"));
        assert!(kvn.contains("= 1"));
    }
}
