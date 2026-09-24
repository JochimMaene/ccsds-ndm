// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

mod kvn;
mod xml;

use crate::common::OdParameters;
use crate::error::{Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::traits::{Ndm, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//----------------------------------------------------------------------
// Root CDM Structure
//----------------------------------------------------------------------

/// Conjunction Data Message (CDM).
///
/// The CDM contains information about a single conjunction between a primary object (Object1)
/// and a secondary object (Object2). It allows satellite operators to evaluate the risk of
/// collision and plan avoidance maneuvers.
///
/// The message includes:
/// - Positions and velocities of both objects at Time of Closest Approach (TCA).
/// - Covariance matrices for both objects at TCA.
/// - Relative position and velocity of Object2 with respect to Object1.
/// - Metadata describing how the data was determined (orbit determination settings).
///
/// **CCSDS Reference**: 508.0-B-1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "cdm")]
pub struct Cdm {
    pub header: CdmHeader,
    pub body: CdmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_CDM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "1.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Cdm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Cdm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Cdm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        kvn::validate_kvn_syntax(kvn)?;
        let cdm = Self::from_kvn_str(kvn)?;
        crate::traits::Validate::validate(&cdm)?;
        Ok(cdm)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        xml::validate_xml_sequences(xml)?;
        let cdm: Self = crate::xml::from_str_with_context(xml, "CDM")?;
        crate::traits::Validate::validate(&cdm)?;
        Ok(cdm)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

//----------------------------------------------------------------------
// Header
//----------------------------------------------------------------------

/// Represents the `cdmHeader` complex type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmHeader {
    /// Comments (allowed in the CDM Header only immediately after the CDM version number).
    /// (See 6.3.4 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Message creation date/time in Coordinated Universal Time (UTC). (See 6.3.2.6 for
    /// formatting rules.)
    ///
    /// **Examples**: 2010-03-12T22:31:12.000, 2010-071T22:31:12.000
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    pub creation_date: CalendarEpoch,
    /// Creating agency or owner/operator. Value should be the 'Abbreviation' value from the
    /// SANA 'Organizations' registry (<https://sanaregistry.org/r/organizations>) for an
    /// organization that has the Role of 'Conjunction Data Message Originator'. (See 5.2.9
    /// for formatting rules.)
    ///
    /// **Examples**: JSPOC, ESA SST, CAESAR, JPL, SDC
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[builder(into)]
    pub originator: String,
    /// Spacecraft name(s) for which the CDM is provided.
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub message_for: Option<String>,
    /// ID that uniquely identifies a message from a given originator. The format and content
    /// of the message identifier value are at the discretion of the originator. (See 5.2.9
    /// for formatting rules.)
    ///
    /// **Examples**: 201113719185, ABC-12_34
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.2.
    #[builder(into)]
    pub message_id: String,
}

impl Cdm {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

impl crate::traits::Validate for CdmHeader {
    fn validate(&self) -> Result<()> {
        crate::validation::epoch_precision(&[("CREATION_DATE", &self.creation_date)])?;
        if self.creation_date.is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "CREATION_DATE".into(),
                line: None,
            }
            .into());
        }
        if self.originator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "ORIGINATOR".into(),
                line: None,
            }
            .into());
        }
        if self.message_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Header".into(),
                field: "MESSAGE_ID".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Body
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct CdmBody {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: RelativeMetadataData,
    #[serde(rename = "segment")]
    pub segments: Vec<CdmSegment>,
}

impl crate::traits::Validate for CdmBody {
    fn validate(&self) -> Result<()> {
        if self.segments.len() != 2 {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed("CDM Body must have exactly 2 segments"),
                line: None,
            }
            .into());
        }
        let object1_count = self
            .segments
            .iter()
            .filter(|s| s.metadata.object == CdmObjectType::Object1)
            .count();
        let object2_count = self
            .segments
            .iter()
            .filter(|s| s.metadata.object == CdmObjectType::Object2)
            .count();
        if object1_count != 1 || object2_count != 1 {
            return Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "CDM Body segments must contain exactly one OBJECT1 and one OBJECT2",
                ),
                line: None,
            }
            .into());
        }
        self.relative_metadata_data.validate()?;
        for segment in &self.segments {
            segment.validate()?;
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Relative Metadata/Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeMetadataData {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// The date and time in UTC of the closest approach. (See 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    pub tca: CalendarEpoch,
    /// The norm of the relative position vector. It indicates how close the two objects are at
    /// TCA. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    pub miss_distance: Length,
    /// The norm of the relative velocity vector. It indicates how fast the two objects are
    /// moving relative to each other at TCA. Data type = double.
    ///
    /// **Units**: m/s
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_speed: Option<Dv>,
    /// The relative position and velocity of Object2 with respect to Object1.
    #[serde(
        rename = "relativeStateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_state_vector: Option<RelativeStateVector>,
    /// The start time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_screen_period: Option<CalendarEpoch>,
    /// The stop time in UTC of the screening period for the conjunction assessment. (See
    /// 6.3.2.6 for formatting rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_screen_period: Option<CalendarEpoch>,
    /// Name of the Object1 centered reference frame in which the screening volume data are
    /// given. Available options are RTN and Transverse, Velocity, and Normal (TVN). (See annex
    /// E for definition.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_frame: Option<ScreenVolumeFrameType>,
    /// Shape of the screening volume: ELLIPSOID or BOX.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_shape: Option<ScreenVolumeShapeType>,

    /// The R or T (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_x: Option<Length>,
    /// The T or V (depending on if RTN or TVN is selected) component size of the screening
    /// volume in the SCREEN_VOLUME_FRAME. Data type = double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_y: Option<Length>,
    /// The N component size of the screening volume in the SCREEN_VOLUME_FRAME. Data type =
    /// double.
    ///
    /// **Units**: m
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_z: Option<Length>,
    /// The time in UTC when Object2 enters the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_entry_time: Option<CalendarEpoch>,
    /// The time in UTC when Object2 exits the screening volume. (See 6.3.2.6 for formatting
    /// rules.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_exit_time: Option<CalendarEpoch>,
    /// The probability (denoted 'p' where 0.0<=p<=1.0), that Object1 and Object2 will collide.
    /// Data type = double.
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_probability: Option<Probability>,
    /// The method that was used to calculate the collision probability. (See annex E for
    /// definition.)
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub collision_probability_method: Option<String>,
}

impl crate::traits::Validate for RelativeMetadataData {
    fn validate(&self) -> Result<()> {
        crate::validation::epoch_precision(&[
            ("TCA", &self.tca),
            ("START_SCREEN_PERIOD", &self.start_screen_period),
            ("STOP_SCREEN_PERIOD", &self.stop_screen_period),
            ("SCREEN_ENTRY_TIME", &self.screen_entry_time),
            ("SCREEN_EXIT_TIME", &self.screen_exit_time),
        ])?;
        // These are plain doubles in the CDM schema, so only finiteness constrains them. The KVN
        // writer already refused non-finite numbers; `validate` and the XML writer did not, and
        // an infinity reached the output as the lexical `inf`, which `xsd:double` rejects.
        let mut numbers: Vec<(&'static str, f64)> =
            vec![("MISS_DISTANCE", self.miss_distance.value)];
        if let Some(value) = &self.relative_speed {
            numbers.push(("RELATIVE_SPEED", value.value));
        }
        if let Some(vector) = &self.relative_state_vector {
            numbers.extend([
                ("RELATIVE_POSITION_R", vector.relative_position_r.value),
                ("RELATIVE_POSITION_T", vector.relative_position_t.value),
                ("RELATIVE_POSITION_N", vector.relative_position_n.value),
                ("RELATIVE_VELOCITY_R", vector.relative_velocity_r.value),
                ("RELATIVE_VELOCITY_T", vector.relative_velocity_t.value),
                ("RELATIVE_VELOCITY_N", vector.relative_velocity_n.value),
            ]);
        }
        for (field, value) in [
            ("SCREEN_VOLUME_X", &self.screen_volume_x),
            ("SCREEN_VOLUME_Y", &self.screen_volume_y),
            ("SCREEN_VOLUME_Z", &self.screen_volume_z),
        ] {
            if let Some(value) = value {
                numbers.push((field, value.value));
            }
        }
        for (field, value) in numbers {
            require_finite(field, value)?;
        }
        if let Some(prob) = &self.collision_probability {
            if !(0.0..=1.0).contains(&prob.value) {
                return Err(crate::error::ValidationError::OutOfRange {
                    name: "COLLISION_PROBABILITY".into(),
                    value: prob.value.to_string(),
                    expected: "0.0 <= p <= 1.0".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeStateVector {
    pub relative_position_r: Length,
    pub relative_position_t: Length,
    pub relative_position_n: Length,
    pub relative_velocity_r: Dv,
    pub relative_velocity_t: Dv,
    pub relative_velocity_n: Dv,
}

//----------------------------------------------------------------------
// Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct CdmSegment {
    pub metadata: CdmMetadata,
    pub data: CdmData,
}

impl crate::traits::Validate for CdmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmMetadata {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// The object to which the metadata and data apply (Object1 or Object2).
    ///
    /// **Examples**: OBJECT1, OBJECT2
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub object: CdmObjectType,
    /// The satellite catalog designator for the object. (See 5.2.9 for formatting rules.)
    ///
    /// **Examples**: 12345
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub object_designator: String,
    /// The satellite catalog used for the object. Value should be taken from the SANA
    /// 'Conjunction Data Message CATALOG_NAME' registry
    /// (<https://sanaregistry.org/r/cdm_catalog>). (See 5.2.9 for formatting rules.)
    ///
    /// **Examples**: SATCAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub catalog_name: String,
    /// Spacecraft name for the object.
    ///
    /// **Examples**: SPOT, ENVISAT, IRIDIUM, INTELSAT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub object_name: String,
    /// The full international designator for the object. Values shall have the format
    /// YYYY-NNNP{PP}, where: YYYY = year of launch; NNN = three-digit serial number of launch
    /// (with leading zeros); P{PP} = At least one capital letter for the identification of the
    /// part brought into space by the launch. In cases where the object has no international
    /// designator, the value UNKNOWN should be used. (See 5.2.9 for further formatting rules.)
    ///
    /// **Examples**: 2002-021A, UNKNOWN
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub international_designator: String,
    /// The object type.
    ///
    /// **Examples**: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    /// Contact position of the owner/operator of the object.
    ///
    /// **Examples**: ORBITAL SAFETY ANALYST (OSA), NETWORK CONTROLLER
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub operator_contact_position: Option<String>,
    /// Contact organization of the object.
    ///
    /// **Examples**: EUMETSAT, ESA, INTELSAT, IRIDIUM
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub operator_organization: Option<String>,
    /// Phone number of the contact position or organization for the object.
    ///
    /// **Examples**: +49615130312
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub operator_phone: Option<String>,
    /// Email address of the contact position or organization of the object.
    ///
    /// **Examples**: JOHN.DOE@SOMEWHERE.NET
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub operator_email: Option<String>,
    /// Unique name of the external ephemeris file used for the object or NONE. This is used to
    /// indicate whether an external (i.e., Owner/Operator [O/O] provided) ephemeris file was
    /// used to calculate the CA. If 'NONE' is specified, then the output of the most current
    /// Orbit Determination (OD) of the CDM originator was used in the CA.
    ///
    /// **Examples**: EPHEMERIS SATELLITE A, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[builder(into)]
    pub ephemeris_name: String,
    /// Method used to calculate the covariance during the OD that produced the state vector, or
    /// whether an arbitrary, non-calculated default value was used. Caution should be used
    /// when using the default value for calculating collision probability.
    ///
    /// **Examples**: CALCULATED, DEFAULT
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub covariance_method: CovarianceMethodType,
    /// The maneuver capacity of the object. (See 1.4.3.1 for definition of 'N/A'.)
    ///
    /// **Examples**: YES, NO, N/A
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub maneuverable: ManeuverableType,
    /// The central body about which Object1 and Object2 orbit. If not specified, the center is
    /// assumed to be Earth.
    ///
    /// **Examples**: EARTH, SUN, MOON, MARS
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub orbit_center: Option<String>,
    /// Name of the reference frame in which the state vector data are given. Value must be
    /// selected from the list of values to the right (see reference `[F1]`) and be the same for
    /// both Object1 and Object2.
    ///
    /// **Examples**: GCRF, EME2000, ITRF
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    pub ref_frame: ReferenceFrameType,
    /// The gravity model used for the OD of the object. (See annex E under GRAVITY_MODEL for
    /// definition).
    ///
    /// **Examples**: EGM-96: 36D 360, WGS-84_GEOID: 24D 240, JGM-2: 41D 410
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub gravity_model: Option<String>,
    /// The atmospheric density model used for the OD of the object. If 'NONE' is specified,
    /// then no atmospheric model was used.
    ///
    /// **Examples**: JACCHIA 70, MSIS, JACCHIA 70 DCA, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub atmospheric_model: Option<String>,
    /// The N-body gravitational perturbations used for the OD of the object. If 'NONE' is
    /// specified, then no third-body gravitational perturbations were used.
    ///
    /// **Examples**: MOON, SUN, JUPITER, NONE
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub n_body_perturbations: Option<String>,
    /// Indication of whether solar radiation pressure perturbations were used for the OD of the
    /// object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_pressure: Option<YesNo>,
    /// Indication of whether solid Earth and ocean tides were used for the OD of the object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earth_tides: Option<YesNo>,
    /// Indication of whether in-track thrust modeling was used for the OD of the object.
    ///
    /// **Examples**: YES, NO
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrack_thrust: Option<YesNo>,
}

impl crate::traits::Validate for CdmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_designator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "OBJECT_DESIGNATOR".into(),
                line: None,
            }
            .into());
        }
        if self.catalog_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "CATALOG_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.international_designator.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "INTERNATIONAL_DESIGNATOR".into(),
                line: None,
            }
            .into());
        }
        if self.ephemeris_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Metadata".into(),
                field: "EPHEMERIS_NAME".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl CdmMetadata {
    pub fn validate(&self) -> Result<()> {
        crate::traits::Validate::validate(self)
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct CdmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Orbit Determination Parameters.
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    /// Additional Parameters.
    #[serde(
        rename = "additionalParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parameters: Option<AdditionalParameters>,
    /// State Vector.
    #[serde(rename = "stateVector")]
    pub state_vector: CdmStateVector,
    /// Covariance Matrix.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<CdmCovarianceMatrix>,
}

impl crate::traits::Validate for CdmData {
    fn validate(&self) -> Result<()> {
        if self.covariance_matrix.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: "CDM Data".into(),
                field: "covarianceMatrix".into(),
                line: None,
            }
            .into());
        }
        if let Some(parameters) = &self.od_parameters {
            parameters.validate()?;
        }
        if let Some(parameters) = &self.additional_parameters {
            parameters.validate()?;
        }
        self.state_vector.validate()?;
        if let Some(covariance_matrix) = &self.covariance_matrix {
            covariance_matrix.validate()?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdditionalParameters {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,

    /// The actual area of the object. (See annex E for definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_pc: Option<Area>,
    /// The effective area of the object exposed to atmospheric drag. (See annex E for
    /// definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_drg: Option<Area>,

    /// The effective area of the object exposed to solar radiation pressure. (See annex E for
    /// definition.)
    ///
    /// **Units**: m²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_srp: Option<Area>,

    /// The mass of the object.
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Mass>,

    /// The object's CD•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd_area_over_mass: Option<M2kgRequired>,

    /// The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cr_area_over_mass: Option<M2kgRequired>,

    /// The object's acceleration due to in-track thrust used to propagate the state vector and
    /// covariance to TCA. (See annex E for definition.)
    ///
    /// **Units**: m/s²
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrust_acceleration: Option<Ms2>,

    /// The amount of energy being removed from the object's orbit by atmospheric drag. This
    /// value is an average calculated during the OD.
    ///
    /// **Units**: W/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sedr: Option<Wkg>,
}

impl Validate for AdditionalParameters {
    fn validate(&self) -> Result<()> {
        // NDM/XML 4.0: areaType, massType, m2kgType and wkgType extend
        // nonNegativeDouble; only thrust acceleration is a signed double.
        for (field, value, nonnegative) in [
            ("AREA_PC", self.area_pc.as_ref().map(|v| v.value), true),
            ("AREA_DRG", self.area_drg.as_ref().map(|v| v.value), true),
            ("AREA_SRP", self.area_srp.as_ref().map(|v| v.value), true),
            ("MASS", self.mass.as_ref().map(|v| v.value), true),
            (
                "CD_AREA_OVER_MASS",
                self.cd_area_over_mass.as_ref().map(|v| v.value),
                true,
            ),
            (
                "CR_AREA_OVER_MASS",
                self.cr_area_over_mass.as_ref().map(|v| v.value),
                true,
            ),
            (
                "THRUST_ACCELERATION",
                self.thrust_acceleration.as_ref().map(|v| v.value),
                false,
            ),
            ("SEDR", self.sedr.as_ref().map(|v| v.value), true),
        ] {
            let Some(value) = value else { continue };
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
            if nonnegative && value < 0.0 {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: value.to_string(),
                    expected: ">= 0".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmStateVector {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Object Position Vector X component.
    ///
    /// Units: km
    pub x: PositionRequired,
    /// Object Position Vector Y component.
    ///
    /// Units: km
    pub y: PositionRequired,
    /// Object Position Vector Z component.
    ///
    /// Units: km
    pub z: PositionRequired,
    /// Object Velocity Vector X component.
    ///
    /// Units: km/s
    pub x_dot: VelocityRequired,
    /// Object Velocity Vector Y component.
    ///
    /// Units: km/s
    pub y_dot: VelocityRequired,
    /// Object Velocity Vector Z component.
    ///
    /// Units: km/s
    pub z_dot: VelocityRequired,
}

fn non_finite_error(field: &'static str, value: f64) -> Option<ValidationError> {
    (!value.is_finite()).then(|| ValidationError::InvalidValue {
        field: field.into(),
        value: value.to_string(),
        expected: "a finite number".into(),
        line: None,
    })
}

impl Validate for CdmStateVector {
    fn validate(&self) -> Result<()> {
        for (field, value) in self.values() {
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
        }
        Ok(())
    }
}

impl CdmStateVector {
    fn values(&self) -> impl Iterator<Item = (&'static str, f64)> {
        [
            ("X", self.x.value),
            ("Y", self.y.value),
            ("Z", self.z.value),
            ("X_DOT", self.x_dot.value),
            ("Y_DOT", self.y_dot.value),
            ("Z_DOT", self.z_dot.value),
        ]
        .into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmCovarianceMatrix {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Object covariance matrix `[1,1]`.
    ///
    /// Units: m²
    pub cr_r: M2,
    /// Object covariance matrix `[2,1]`.
    ///
    /// Units: m²
    pub ct_r: M2,
    /// Object covariance matrix `[2,2]`.
    ///
    /// Units: m²
    pub ct_t: M2,
    /// Object covariance matrix `[3,1]`.
    ///
    /// Units: m²
    pub cn_r: M2,
    /// Object covariance matrix `[3,2]`.
    ///
    /// Units: m²
    pub cn_t: M2,
    /// Object covariance matrix `[3,3]`.
    ///
    /// Units: m²
    pub cn_n: M2,
    /// Object covariance matrix `[4,1]`.
    ///
    /// Units: m²/s
    pub crdot_r: M2s,
    /// Object covariance matrix `[4,2]`.
    ///
    /// Units: m²/s
    pub crdot_t: M2s,
    /// Object covariance matrix `[4,3]`.
    ///
    /// Units: m²/s
    pub crdot_n: M2s,
    /// Object covariance matrix `[4,4]`.
    ///
    /// Units: m²/s²
    pub crdot_rdot: M2s2,
    /// Object covariance matrix `[5,1]`.
    ///
    /// Units: m²/s
    pub ctdot_r: M2s,
    /// Object covariance matrix `[5,2]`.
    ///
    /// Units: m²/s
    pub ctdot_t: M2s,
    /// Object covariance matrix `[5,3]`.
    ///
    /// Units: m²/s
    pub ctdot_n: M2s,
    /// Object covariance matrix `[5,4]`.
    ///
    /// Units: m²/s²
    pub ctdot_rdot: M2s2,
    /// Object covariance matrix `[5,5]`.
    ///
    /// Units: m²/s²
    pub ctdot_tdot: M2s2,
    /// Object covariance matrix `[6,1]`.
    ///
    /// Units: m²/s
    pub cndot_r: M2s,
    /// Object covariance matrix `[6,2]`.
    ///
    /// Units: m²/s
    pub cndot_t: M2s,
    /// Object covariance matrix `[6,3]`.
    ///
    /// Units: m²/s
    pub cndot_n: M2s,
    /// Object covariance matrix `[6,4]`.
    ///
    /// Units: m²/s²
    pub cndot_rdot: M2s2,
    /// Object covariance matrix `[6,5]`.
    ///
    /// Units: m²/s²
    pub cndot_tdot: M2s2,
    /// Object covariance matrix `[6,6]`.
    ///
    /// Units: m²/s²
    pub cndot_ndot: M2s2,

    /// Object covariance matrix `[7,1]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_r: Option<M3kg>,
    /// Object covariance matrix `[7,2]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_t: Option<M3kg>,
    /// Object covariance matrix `[7,3]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_n: Option<M3kg>,
    /// Object covariance matrix `[7,4]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_rdot: Option<M3kgs>,
    /// Object covariance matrix `[7,5]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_tdot: Option<M3kgs>,
    /// Object covariance matrix `[7,6]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_ndot: Option<M3kgs>,
    /// Object covariance matrix `[7,7]`.
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_drg: Option<M4kg2>,

    /// Object covariance matrix `[8,1]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_r: Option<M3kg>,
    /// Object covariance matrix `[8,2]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_t: Option<M3kg>,
    /// Object covariance matrix `[8,3]`.
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_n: Option<M3kg>,
    /// Object covariance matrix `[8,4]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_rdot: Option<M3kgs>,
    /// Object covariance matrix `[8,5]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_tdot: Option<M3kgs>,
    /// Object covariance matrix `[8,6]`.
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_ndot: Option<M3kgs>,
    /// Object covariance matrix `[8,7]`.
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_drg: Option<M4kg2>,
    /// Object covariance matrix `[8,8]`.
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_srp: Option<M4kg2>,

    /// Object covariance matrix `[9,1]`.
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_r: Option<M2s2>,
    /// Object covariance matrix `[9,2]`.
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_t: Option<M2s2>,
    /// Object covariance matrix `[9,3]`.
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_n: Option<M2s2>,
    /// Object covariance matrix `[9,4]`.
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_rdot: Option<M2s3>,
    /// Object covariance matrix `[9,5]`.
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_tdot: Option<M2s3>,
    /// Object covariance matrix `[9,6]`.
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_ndot: Option<M2s3>,
    /// Object covariance matrix `[9,7]`.
    ///
    /// Units: m³/(kg*s²)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_drg: Option<M3kgs2>,
    /// Object covariance matrix `[9,8]`.
    ///
    /// Units: m³/(kg*s²)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_srp: Option<M3kgs2>,
    /// Object covariance matrix `[9,9]`.
    ///
    /// Units: m²/s⁴
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_thr: Option<M2s4>,
}

impl Validate for CdmCovarianceMatrix {
    fn validate(&self) -> Result<()> {
        if let Some(error) = self.optional_row_errors().into_iter().next() {
            return Err(error.into());
        }
        for (field, value) in self.values() {
            if let Some(error) = non_finite_error(field, value) {
                return Err(error.into());
            }
        }
        Ok(())
    }
}

impl CdmCovarianceMatrix {
    fn optional_row_errors(&self) -> Vec<ValidationError> {
        let row_7 = [
            ("CDRG_R", self.cdrg_r.is_some()),
            ("CDRG_T", self.cdrg_t.is_some()),
            ("CDRG_N", self.cdrg_n.is_some()),
            ("CDRG_RDOT", self.cdrg_rdot.is_some()),
            ("CDRG_TDOT", self.cdrg_tdot.is_some()),
            ("CDRG_NDOT", self.cdrg_ndot.is_some()),
            ("CDRG_DRG", self.cdrg_drg.is_some()),
        ];
        let row_8 = [
            ("CSRP_R", self.csrp_r.is_some()),
            ("CSRP_T", self.csrp_t.is_some()),
            ("CSRP_N", self.csrp_n.is_some()),
            ("CSRP_RDOT", self.csrp_rdot.is_some()),
            ("CSRP_TDOT", self.csrp_tdot.is_some()),
            ("CSRP_NDOT", self.csrp_ndot.is_some()),
            ("CSRP_DRG", self.csrp_drg.is_some()),
            ("CSRP_SRP", self.csrp_srp.is_some()),
        ];
        let row_9 = [
            ("CTHR_R", self.cthr_r.is_some()),
            ("CTHR_T", self.cthr_t.is_some()),
            ("CTHR_N", self.cthr_n.is_some()),
            ("CTHR_RDOT", self.cthr_rdot.is_some()),
            ("CTHR_TDOT", self.cthr_tdot.is_some()),
            ("CTHR_NDOT", self.cthr_ndot.is_some()),
            ("CTHR_DRG", self.cthr_drg.is_some()),
            ("CTHR_SRP", self.cthr_srp.is_some()),
            ("CTHR_THR", self.cthr_thr.is_some()),
        ];
        let rows: [(&str, &[(&str, bool)]); 3] = [
            ("CDM Covariance Matrix row 7", &row_7),
            ("CDM Covariance Matrix row 8", &row_8),
            ("CDM Covariance Matrix row 9", &row_9),
        ];
        let Some(last_present_row) = rows
            .iter()
            .rposition(|(_, fields)| fields.iter().any(|(_, present)| *present))
        else {
            return Vec::new();
        };

        rows.into_iter()
            .take(last_present_row + 1)
            .flat_map(|(block, fields)| {
                fields
                    .iter()
                    .filter(|(_, present)| !present)
                    .map(move |(field, _)| ValidationError::MissingRequiredField {
                        block: block.into(),
                        field: (*field).into(),
                        line: None,
                    })
            })
            .collect()
    }

    fn values(&self) -> impl Iterator<Item = (&'static str, f64)> + '_ {
        let required = [
            ("CR_R", self.cr_r.value),
            ("CT_R", self.ct_r.value),
            ("CT_T", self.ct_t.value),
            ("CN_R", self.cn_r.value),
            ("CN_T", self.cn_t.value),
            ("CN_N", self.cn_n.value),
            ("CRDOT_R", self.crdot_r.value),
            ("CRDOT_T", self.crdot_t.value),
            ("CRDOT_N", self.crdot_n.value),
            ("CRDOT_RDOT", self.crdot_rdot.value),
            ("CTDOT_R", self.ctdot_r.value),
            ("CTDOT_T", self.ctdot_t.value),
            ("CTDOT_N", self.ctdot_n.value),
            ("CTDOT_RDOT", self.ctdot_rdot.value),
            ("CTDOT_TDOT", self.ctdot_tdot.value),
            ("CNDOT_R", self.cndot_r.value),
            ("CNDOT_T", self.cndot_t.value),
            ("CNDOT_N", self.cndot_n.value),
            ("CNDOT_RDOT", self.cndot_rdot.value),
            ("CNDOT_TDOT", self.cndot_tdot.value),
            ("CNDOT_NDOT", self.cndot_ndot.value),
        ];
        let optional = [
            ("CDRG_R", self.cdrg_r.as_ref().map(|value| value.value)),
            ("CDRG_T", self.cdrg_t.as_ref().map(|value| value.value)),
            ("CDRG_N", self.cdrg_n.as_ref().map(|value| value.value)),
            (
                "CDRG_RDOT",
                self.cdrg_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CDRG_TDOT",
                self.cdrg_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CDRG_NDOT",
                self.cdrg_ndot.as_ref().map(|value| value.value),
            ),
            ("CDRG_DRG", self.cdrg_drg.as_ref().map(|value| value.value)),
            ("CSRP_R", self.csrp_r.as_ref().map(|value| value.value)),
            ("CSRP_T", self.csrp_t.as_ref().map(|value| value.value)),
            ("CSRP_N", self.csrp_n.as_ref().map(|value| value.value)),
            (
                "CSRP_RDOT",
                self.csrp_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CSRP_TDOT",
                self.csrp_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CSRP_NDOT",
                self.csrp_ndot.as_ref().map(|value| value.value),
            ),
            ("CSRP_DRG", self.csrp_drg.as_ref().map(|value| value.value)),
            ("CSRP_SRP", self.csrp_srp.as_ref().map(|value| value.value)),
            ("CTHR_R", self.cthr_r.as_ref().map(|value| value.value)),
            ("CTHR_T", self.cthr_t.as_ref().map(|value| value.value)),
            ("CTHR_N", self.cthr_n.as_ref().map(|value| value.value)),
            (
                "CTHR_RDOT",
                self.cthr_rdot.as_ref().map(|value| value.value),
            ),
            (
                "CTHR_TDOT",
                self.cthr_tdot.as_ref().map(|value| value.value),
            ),
            (
                "CTHR_NDOT",
                self.cthr_ndot.as_ref().map(|value| value.value),
            ),
            ("CTHR_DRG", self.cthr_drg.as_ref().map(|value| value.value)),
            ("CTHR_SRP", self.cthr_srp.as_ref().map(|value| value.value)),
            ("CTHR_THR", self.cthr_thr.as_ref().map(|value| value.value)),
        ];

        required.into_iter().chain(
            optional
                .into_iter()
                .filter_map(|(field, value)| value.map(|value| (field, value))),
        )
    }
}
