// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::OdParameters;
use crate::error::Result;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::*;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root CDM Structure
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "cdm")]
pub struct Cdm {
    pub header: CdmHeader,
    pub body: CdmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Cdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        // 1. Header
        writer.write_pair("CCSDS_CDM_VERS", &self.version);
        self.header.write_kvn(&mut writer);

        // 2. Body
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
// Header
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmHeader {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Message creation date/time in Coordinated Universal Time (UTC).
    ///
    /// Examples: 2010-03-12T22:31:12.000, 2010-071T22:31:12.000
    pub creation_date: Epoch,
    /// Creating agency or owner/operator.
    ///
    /// Examples: JSPOC, ESA SST, CAESAR, JPL, SDC
    pub originator: String,
    /// Spacecraft name(s) for which the CDM is provided.
    ///
    /// Examples: SPOT, ENVISAT, IRIDIUM, INTELSAT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_for: Option<String>,
    /// ID that uniquely identifies a message from a given originator.
    ///
    /// Examples: 201113719185, ABC-12_34
    pub message_id: String,
}

impl ToKvn for CdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", &self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        if let Some(msg_for) = &self.message_for {
            writer.write_pair("MESSAGE_FOR", msg_for);
        }
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

//----------------------------------------------------------------------
// Body
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CdmBody {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: RelativeMetadataData,
    #[serde(rename = "segment")]
    pub segments: Vec<CdmSegment>,
}

impl ToKvn for CdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.relative_metadata_data.write_kvn(writer);
        for segment in &self.segments {
            segment.write_kvn(writer);
        }
    }
}

//----------------------------------------------------------------------
// Relative Metadata/Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeMetadataData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// The date and time in UTC of the closest approach.
    pub tca: Epoch,
    /// The norm of the relative position vector.
    ///
    /// Units: m
    pub miss_distance: Length,
    /// The norm of the relative velocity vector.
    ///
    /// Units: m/s
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_speed: Option<Dv>,
    /// The relative position and velocity of Object2 with respect to Object1.
    #[serde(
        rename = "relativeStateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_state_vector: Option<RelativeStateVector>,
    /// The start time in UTC of the screening period for the conjunction assessment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_screen_period: Option<Epoch>,
    /// The stop time in UTC of the screening period for the conjunction assessment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_screen_period: Option<Epoch>,
    /// Name of the Object1 centered reference frame in which the screening volume data are given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_frame: Option<ScreenVolumeFrameType>,
    /// Shape of the screening volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_shape: Option<ScreenVolumeShapeType>,
    /// The R or T component size of the screening volume.
    ///
    /// Units: m
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_x: Option<Length>,
    /// The T or V component size of the screening volume.
    ///
    /// Units: m
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_y: Option<Length>,
    /// The N component size of the screening volume.
    ///
    /// Units: m
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_volume_z: Option<Length>,
    /// The time in UTC when Object2 enters the screening volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_entry_time: Option<Epoch>,
    /// The time in UTC when Object2 exits the screening volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen_exit_time: Option<Epoch>,
    /// The probability that Object1 and Object2 will collide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_probability: Option<Probability>,
    /// The method that was used to calculate the collision probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_probability_method: Option<String>,
}

impl ToKvn for RelativeMetadataData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("TCA", &self.tca);
        writer.write_measure("MISS_DISTANCE", &self.miss_distance);
        if let Some(v) = &self.relative_speed {
            writer.write_measure("RELATIVE_SPEED", v);
        }
        if let Some(v) = &self.relative_state_vector {
            v.write_kvn(writer);
        }
        if let Some(v) = &self.start_screen_period {
            writer.write_pair("START_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.stop_screen_period {
            writer.write_pair("STOP_SCREEN_PERIOD", v);
        }
        if let Some(v) = &self.screen_volume_frame {
            writer.write_pair("SCREEN_VOLUME_FRAME", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.screen_volume_shape {
            writer.write_pair("SCREEN_VOLUME_SHAPE", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.screen_volume_x {
            writer.write_measure("SCREEN_VOLUME_X", v);
        }
        if let Some(v) = &self.screen_volume_y {
            writer.write_measure("SCREEN_VOLUME_Y", v);
        }
        if let Some(v) = &self.screen_volume_z {
            writer.write_measure("SCREEN_VOLUME_Z", v);
        }
        if let Some(v) = &self.screen_entry_time {
            writer.write_pair("SCREEN_ENTRY_TIME", v);
        }
        if let Some(v) = &self.screen_exit_time {
            writer.write_pair("SCREEN_EXIT_TIME", v);
        }
        if let Some(v) = &self.collision_probability {
            writer.write_pair("COLLISION_PROBABILITY", v.value);
        }
        if let Some(v) = &self.collision_probability_method {
            writer.write_pair("COLLISION_PROBABILITY_METHOD", v);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeStateVector {
    pub relative_position_r: Length,
    pub relative_position_t: Length,
    pub relative_position_n: Length,
    pub relative_velocity_r: Dv,
    pub relative_velocity_t: Dv,
    pub relative_velocity_n: Dv,
}

impl ToKvn for RelativeStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_measure("RELATIVE_POSITION_R", &self.relative_position_r);
        writer.write_measure("RELATIVE_POSITION_T", &self.relative_position_t);
        writer.write_measure("RELATIVE_POSITION_N", &self.relative_position_n);
        writer.write_measure("RELATIVE_VELOCITY_R", &self.relative_velocity_r);
        writer.write_measure("RELATIVE_VELOCITY_T", &self.relative_velocity_t);
        writer.write_measure("RELATIVE_VELOCITY_N", &self.relative_velocity_n);
    }
}

//----------------------------------------------------------------------
// Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CdmSegment {
    pub metadata: CdmMetadata,
    pub data: CdmData,
}

impl ToKvn for CdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmMetadata {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// The object to which the metadata and data apply.
    ///
    /// Examples: OBJECT1, OBJECT2
    pub object: CdmObjectType,
    /// The satellite catalog designator for the object.
    pub object_designator: String,
    /// The satellite catalog used for the object.
    pub catalog_name: String,
    /// Spacecraft name for the object.
    pub object_name: String,
    /// The full international designator for the object.
    pub international_designator: String,
    /// The object type.
    ///
    /// Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    /// Contact position of the owner/operator of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_contact_position: Option<String>,
    /// Contact organization of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_organization: Option<String>,
    /// Phone number of the contact position or organization for the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_phone: Option<String>,
    /// Email address of the contact position or organization of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_email: Option<String>,
    /// Unique name of the external ephemeris file used for the object or NONE.
    pub ephemeris_name: String,
    /// Method used to calculate the covariance.
    pub covariance_method: CovarianceMethodType,
    /// The maneuver capacity of the object.
    pub maneuverable: ManeuverableType,
    /// The central body about which Object1 and Object2 orbit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_center: Option<String>,
    /// Name of the reference frame in which the state vector data are given.
    pub ref_frame: ReferenceFrameType,
    /// The gravity model used for the OD of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    /// The atmospheric density model used for the OD of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    /// The N-body gravitational perturbations used for the OD of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    /// Indication of whether solar radiation pressure perturbations were used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_pressure: Option<YesNo>,
    /// Indication of whether solid Earth and ocean tides were used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earth_tides: Option<YesNo>,
    /// Indication of whether in-track thrust modeling was used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrack_thrust: Option<YesNo>,
}

impl ToKvn for CdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair(
            "OBJECT",
            match self.object {
                CdmObjectType::Object1 => "OBJECT1",
                CdmObjectType::Object2 => "OBJECT2",
                _ => "OBJECT1",
            },
        );
        writer.write_pair("OBJECT_DESIGNATOR", &self.object_designator);
        writer.write_pair("CATALOG_NAME", &self.catalog_name);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.operator_contact_position {
            writer.write_pair("OPERATOR_CONTACT_POSITION", v);
        }
        if let Some(v) = &self.operator_organization {
            writer.write_pair("OPERATOR_ORGANIZATION", v);
        }
        if let Some(v) = &self.operator_phone {
            writer.write_pair("OPERATOR_PHONE", v);
        }
        if let Some(v) = &self.operator_email {
            writer.write_pair("OPERATOR_EMAIL", v);
        }
        writer.write_pair("EPHEMERIS_NAME", &self.ephemeris_name);
        writer.write_pair(
            "COVARIANCE_METHOD",
            format!("{:?}", self.covariance_method).to_uppercase(),
        );
        writer.write_pair(
            "MANEUVERABLE",
            format!("{:?}", self.maneuverable).to_uppercase(),
        );
        if let Some(v) = &self.orbit_center {
            writer.write_pair("ORBIT_CENTER", v);
        }
        writer.write_pair("REF_FRAME", format!("{:?}", self.ref_frame).to_uppercase());
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", format!("{:?}", v).to_uppercase());
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", format!("{:?}", v).to_uppercase());
        }
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "covarianceMatrix")]
    pub covariance_matrix: CdmCovarianceMatrix,
}

impl ToKvn for CdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // OD Parameters
        if let Some(od) = &self.od_parameters {
            writer.write_comments(&od.comment);
            if let Some(v) = &od.time_lastob_start {
                writer.write_pair("TIME_LASTOB_START", v);
            }
            if let Some(v) = &od.time_lastob_end {
                writer.write_pair("TIME_LASTOB_END", v);
            }
            if let Some(v) = &od.recommended_od_span {
                writer.write_measure("RECOMMENDED_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.actual_od_span {
                writer.write_measure("ACTUAL_OD_SPAN", &v.to_unit_value());
            }
            if let Some(v) = &od.obs_available {
                writer.write_pair("OBS_AVAILABLE", v);
            }
            if let Some(v) = &od.obs_used {
                writer.write_pair("OBS_USED", v);
            }
            if let Some(v) = &od.tracks_available {
                writer.write_pair("TRACKS_AVAILABLE", v);
            }
            if let Some(v) = &od.tracks_used {
                writer.write_pair("TRACKS_USED", v);
            }
            if let Some(v) = &od.residuals_accepted {
                writer.write_measure("RESIDUALS_ACCEPTED", &v.to_unit_value());
            }
            if let Some(v) = &od.weighted_rms {
                writer.write_pair("WEIGHTED_RMS", v);
            }
        }
        // Additional Parameters
        if let Some(ap) = &self.additional_parameters {
            writer.write_comments(&ap.comment);
            if let Some(v) = &ap.area_pc {
                writer.write_measure("AREA_PC", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_drg {
                writer.write_measure("AREA_DRG", &v.to_unit_value());
            }
            if let Some(v) = &ap.area_srp {
                writer.write_measure("AREA_SRP", &v.to_unit_value());
            }
            if let Some(v) = &ap.mass {
                writer.write_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &ap.cd_area_over_mass {
                writer.write_measure("CD_AREA_OVER_MASS", v);
            }
            if let Some(v) = &ap.cr_area_over_mass {
                writer.write_measure("CR_AREA_OVER_MASS", v);
            }
            if let Some(v) = &ap.thrust_acceleration {
                writer.write_measure("THRUST_ACCELERATION", v);
            }
            if let Some(v) = &ap.sedr {
                writer.write_measure("SEDR", v);
            }
        }
        // State Vector
        self.state_vector.write_kvn(writer);
        // Covariance
        self.covariance_matrix.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdditionalParameters {
    /// Comments (see 6.3.4 for formatting rules).
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub cd_area_over_mass: Option<M2kg>,

    /// The object's CR•A/m used to propagate the state vector and covariance to TCA. (See
    /// annex E for definition.)
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 508.0-B-1, Section 3.5.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cr_area_over_mass: Option<M2kg>,

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmStateVector {
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

impl ToKvn for CdmStateVector {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_measure("X", &self.x.to_unit_value());
        writer.write_measure("Y", &self.y.to_unit_value());
        writer.write_measure("Z", &self.z.to_unit_value());
        writer.write_measure("X_DOT", &self.x_dot.to_unit_value());
        writer.write_measure("Y_DOT", &self.y_dot.to_unit_value());
        writer.write_measure("Z_DOT", &self.z_dot.to_unit_value());
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CdmCovarianceMatrix {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Object covariance matrix [1,1].
    ///
    /// Units: m²
    pub cr_r: M2,
    /// Object covariance matrix [2,1].
    ///
    /// Units: m²
    pub ct_r: M2,
    /// Object covariance matrix [2,2].
    ///
    /// Units: m²
    pub ct_t: M2,
    /// Object covariance matrix [3,1].
    ///
    /// Units: m²
    pub cn_r: M2,
    /// Object covariance matrix [3,2].
    ///
    /// Units: m²
    pub cn_t: M2,
    /// Object covariance matrix [3,3].
    ///
    /// Units: m²
    pub cn_n: M2,
    /// Object covariance matrix [4,1].
    ///
    /// Units: m²/s
    pub crdot_r: M2s,
    /// Object covariance matrix [4,2].
    ///
    /// Units: m²/s
    pub crdot_t: M2s,
    /// Object covariance matrix [4,3].
    ///
    /// Units: m²/s
    pub crdot_n: M2s,
    /// Object covariance matrix [4,4].
    ///
    /// Units: m²/s²
    pub crdot_rdot: M2s2,
    /// Object covariance matrix [5,1].
    ///
    /// Units: m²/s
    pub ctdot_r: M2s,
    /// Object covariance matrix [5,2].
    ///
    /// Units: m²/s
    pub ctdot_t: M2s,
    /// Object covariance matrix [5,3].
    ///
    /// Units: m²/s
    pub ctdot_n: M2s,
    /// Object covariance matrix [5,4].
    ///
    /// Units: m²/s²
    pub ctdot_rdot: M2s2,
    /// Object covariance matrix [5,5].
    ///
    /// Units: m²/s²
    pub ctdot_tdot: M2s2,
    /// Object covariance matrix [6,1].
    ///
    /// Units: m²/s
    pub cndot_r: M2s,
    /// Object covariance matrix [6,2].
    ///
    /// Units: m²/s
    pub cndot_t: M2s,
    /// Object covariance matrix [6,3].
    ///
    /// Units: m²/s
    pub cndot_n: M2s,
    /// Object covariance matrix [6,4].
    ///
    /// Units: m²/s²
    pub cndot_rdot: M2s2,
    /// Object covariance matrix [6,5].
    ///
    /// Units: m²/s²
    pub cndot_tdot: M2s2,
    /// Object covariance matrix [6,6].
    ///
    /// Units: m²/s²
    pub cndot_ndot: M2s2,

    /// Object covariance matrix [7,1].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_r: Option<M3kg>,
    /// Object covariance matrix [7,2].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_t: Option<M3kg>,
    /// Object covariance matrix [7,3].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_n: Option<M3kg>,
    /// Object covariance matrix [7,4].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_rdot: Option<M3kgs>,
    /// Object covariance matrix [7,5].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_tdot: Option<M3kgs>,
    /// Object covariance matrix [7,6].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_ndot: Option<M3kgs>,
    /// Object covariance matrix [7,7].
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrg_drg: Option<M4kg2>,

    /// Object covariance matrix [8,1].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_r: Option<M3kg>,
    /// Object covariance matrix [8,2].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_t: Option<M3kg>,
    /// Object covariance matrix [8,3].
    ///
    /// Units: m³/kg
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_n: Option<M3kg>,
    /// Object covariance matrix [8,4].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_rdot: Option<M3kgs>,
    /// Object covariance matrix [8,5].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_tdot: Option<M3kgs>,
    /// Object covariance matrix [8,6].
    ///
    /// Units: m³/(kg*s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_ndot: Option<M3kgs>,
    /// Object covariance matrix [8,7].
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_drg: Option<M4kg2>,
    /// Object covariance matrix [8,8].
    ///
    /// Units: m⁴/kg²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrp_srp: Option<M4kg2>,

    /// Object covariance matrix [9,1].
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_r: Option<M2s2>,
    /// Object covariance matrix [9,2].
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_t: Option<M2s2>,
    /// Object covariance matrix [9,3].
    ///
    /// Units: m²/s²
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_n: Option<M2s2>,
    /// Object covariance matrix [9,4].
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_rdot: Option<M2s3>,
    /// Object covariance matrix [9,5].
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_tdot: Option<M2s3>,
    /// Object covariance matrix [9,6].
    ///
    /// Units: m²/s³
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_ndot: Option<M2s3>,
    /// Object covariance matrix [9,7].
    ///
    /// Units: m³/(kg*s²)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_drg: Option<M3kgs2>,
    /// Object covariance matrix [9,8].
    ///
    /// Units: m³/(kg*s²)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_srp: Option<M3kgs2>,
    /// Object covariance matrix [9,9].
    ///
    /// Units: m²/s⁴
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cthr_thr: Option<M2s4>,
}

impl ToKvn for CdmCovarianceMatrix {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Required
        writer.write_measure("CR_R", &self.cr_r);
        writer.write_measure("CT_R", &self.ct_r);
        writer.write_measure("CT_T", &self.ct_t);
        writer.write_measure("CN_R", &self.cn_r);
        writer.write_measure("CN_T", &self.cn_t);
        writer.write_measure("CN_N", &self.cn_n);
        writer.write_measure("CRDOT_R", &self.crdot_r);
        writer.write_measure("CRDOT_T", &self.crdot_t);
        writer.write_measure("CRDOT_N", &self.crdot_n);
        writer.write_measure("CRDOT_RDOT", &self.crdot_rdot);
        writer.write_measure("CTDOT_R", &self.ctdot_r);
        writer.write_measure("CTDOT_T", &self.ctdot_t);
        writer.write_measure("CTDOT_N", &self.ctdot_n);
        writer.write_measure("CTDOT_RDOT", &self.ctdot_rdot);
        writer.write_measure("CTDOT_TDOT", &self.ctdot_tdot);
        writer.write_measure("CNDOT_R", &self.cndot_r);
        writer.write_measure("CNDOT_T", &self.cndot_t);
        writer.write_measure("CNDOT_N", &self.cndot_n);
        writer.write_measure("CNDOT_RDOT", &self.cndot_rdot);
        writer.write_measure("CNDOT_TDOT", &self.cndot_tdot);
        writer.write_measure("CNDOT_NDOT", &self.cndot_ndot);

        // Optionals
        if let Some(v) = &self.cdrg_r {
            writer.write_measure("CDRG_R", v);
        }
        if let Some(v) = &self.cdrg_t {
            writer.write_measure("CDRG_T", v);
        }
        if let Some(v) = &self.cdrg_n {
            writer.write_measure("CDRG_N", v);
        }
        if let Some(v) = &self.cdrg_rdot {
            writer.write_measure("CDRG_RDOT", v);
        }
        if let Some(v) = &self.cdrg_tdot {
            writer.write_measure("CDRG_TDOT", v);
        }
        if let Some(v) = &self.cdrg_ndot {
            writer.write_measure("CDRG_NDOT", v);
        }
        if let Some(v) = &self.cdrg_drg {
            writer.write_measure("CDRG_DRG", v);
        }

        if let Some(v) = &self.csrp_r {
            writer.write_measure("CSRP_R", v);
        }
        if let Some(v) = &self.csrp_t {
            writer.write_measure("CSRP_T", v);
        }
        if let Some(v) = &self.csrp_n {
            writer.write_measure("CSRP_N", v);
        }
        if let Some(v) = &self.csrp_rdot {
            writer.write_measure("CSRP_RDOT", v);
        }
        if let Some(v) = &self.csrp_tdot {
            writer.write_measure("CSRP_TDOT", v);
        }
        if let Some(v) = &self.csrp_ndot {
            writer.write_measure("CSRP_NDOT", v);
        }
        if let Some(v) = &self.csrp_drg {
            writer.write_measure("CSRP_DRG", v);
        }
        if let Some(v) = &self.csrp_srp {
            writer.write_measure("CSRP_SRP", v);
        }

        if let Some(v) = &self.cthr_r {
            writer.write_measure("CTHR_R", v);
        }
        if let Some(v) = &self.cthr_t {
            writer.write_measure("CTHR_T", v);
        }
        if let Some(v) = &self.cthr_n {
            writer.write_measure("CTHR_N", v);
        }
        if let Some(v) = &self.cthr_rdot {
            writer.write_measure("CTHR_RDOT", v);
        }
        if let Some(v) = &self.cthr_tdot {
            writer.write_measure("CTHR_TDOT", v);
        }
        if let Some(v) = &self.cthr_ndot {
            writer.write_measure("CTHR_NDOT", v);
        }
        if let Some(v) = &self.cthr_drg {
            writer.write_measure("CTHR_DRG", v);
        }
        if let Some(v) = &self.cthr_srp {
            writer.write_measure("CTHR_SRP", v);
        }
        if let Some(v) = &self.cthr_thr {
            writer.write_measure("CTHR_THR", v);
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    fn sample_cdm_kvn() -> String {
        let kvn = r#"
CCSDS_CDM_VERS = 1.0
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
COLLISION_PROBABILITY = 0.001
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
REF_FRAME = EME2000

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]

"#;
        kvn.to_string()
    }

    #[test]
    fn kvn_roundtrip() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        let regenerated = cdm.to_kvn().expect("to_kvn");
        // Parse again to ensure structural equality
        let cdm2 = Cdm::from_kvn(&regenerated).expect("re-parse");
        assert_eq!(cdm.header.originator, cdm2.header.originator);
        assert_eq!(cdm.body.segments.len(), cdm2.body.segments.len());
    }

    #[test]
    fn xml_roundtrip() {
        let kvn = sample_cdm_kvn();
        let cdm = Cdm::from_kvn(&kvn).expect("parse");
        let xml = cdm.to_xml().expect("to_xml");
        let cdm2 = Cdm::from_xml(&xml).expect("from_xml");
        assert_eq!(cdm.header.originator, cdm2.header.originator);
        assert_eq!(cdm.body.segments.len(), cdm2.body.segments.len());
    }

    #[test]
    fn full_optional_fields_roundtrip() {
        let kvn = r#"
CCSDS_CDM_VERS = 1.0
COMMENT Header comment
CREATION_DATE = 2025-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_FOR = RECIPIENT
MESSAGE_ID = MSG-001

TCA = 2025-01-02T12:00:00
MISS_DISTANCE = 100.0 [m]
RELATIVE_SPEED = 7.5 [m/s]
RELATIVE_POSITION_R = 10.0 [m]
RELATIVE_POSITION_T = -20.0 [m]
RELATIVE_POSITION_N = 5.0 [m]
RELATIVE_VELOCITY_R = 0.1 [m/s]
RELATIVE_VELOCITY_T = -0.2 [m/s]
RELATIVE_VELOCITY_N = 0.05 [m/s]
START_SCREEN_PERIOD = 2025-01-02T11:00:00
STOP_SCREEN_PERIOD = 2025-01-02T13:00:00
SCREEN_VOLUME_FRAME = RTN
SCREEN_VOLUME_SHAPE = BOX
SCREEN_VOLUME_X = 1000.0 [m]
SCREEN_VOLUME_Y = 2000.0 [m]
SCREEN_VOLUME_Z = 3000.0 [m]
SCREEN_ENTRY_TIME = 2025-01-02T11:30:00
SCREEN_EXIT_TIME = 2025-01-02T12:30:00
COLLISION_PROBABILITY = 0.001
COLLISION_PROBABILITY_METHOD = FOSTER-1992

COMMENT Segment 1
OBJECT = OBJECT1
OBJECT_DESIGNATOR = 00001
CATALOG_NAME = CAT
OBJECT_NAME = OBJ1
INTERNATIONAL_DESIGNATOR = 1998-067A
OBJECT_TYPE = PAYLOAD
OPERATOR_CONTACT_POSITION = POSITION
OPERATOR_ORGANIZATION = ORG
OPERATOR_PHONE = PHONE
OPERATOR_EMAIL = EMAIL
EPHEMERIS_NAME = EPH1
COVARIANCE_METHOD = CALCULATED
MANEUVERABLE = YES
ORBIT_CENTER = EARTH
REF_FRAME = EME2000
GRAVITY_MODEL = EGM-96
ATMOSPHERIC_MODEL = JACCHIA
N_BODY_PERTURBATIONS = MOON,SUN
SOLAR_RAD_PRESSURE = YES
EARTH_TIDES = YES
INTRACK_THRUST = YES

TIME_LASTOB_START = 2025-01-01T00:00:00
TIME_LASTOB_END = 2025-01-02T00:00:00
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.0 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 50
TRACKS_USED = 48
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.23

AREA_PC = 10.0 [m**2]
AREA_DRG = 12.0 [m**2]
AREA_SRP = 15.0 [m**2]
MASS = 1000.0 [kg]
CD_AREA_OVER_MASS = 0.012 [m**2/kg]
CR_AREA_OVER_MASS = 0.015 [m**2/kg]
THRUST_ACCELERATION = 0.001 [m/s**2]
SEDR = 0.05 [W/kg]

X = 1.0 [km]
Y = 2.0 [km]
Z = 3.0 [km]
X_DOT = 0.1 [km/s]
Y_DOT = 0.2 [km/s]
Z_DOT = 0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
CDRG_R = 0.001 [m**3/kg]
CDRG_T = 0.002 [m**3/kg]
CDRG_N = 0.003 [m**3/kg]
CDRG_RDOT = 0.0001 [m**3/(kg*s)]
CDRG_TDOT = 0.0002 [m**3/(kg*s)]
CDRG_NDOT = 0.0003 [m**3/(kg*s)]
CDRG_DRG = 0.00001 [m**4/kg**2]
CSRP_R = 0.001 [m**3/kg]
CSRP_T = 0.002 [m**3/kg]
CSRP_N = 0.003 [m**3/kg]
CSRP_RDOT = 0.0001 [m**3/(kg*s)]
CSRP_TDOT = 0.0002 [m**3/(kg*s)]
CSRP_NDOT = 0.0003 [m**3/(kg*s)]
CSRP_DRG = 0.00001 [m**4/kg**2]
CSRP_SRP = 0.00002 [m**4/kg**2]
CTHR_R = 0.001 [m**2/s**2]
CTHR_T = 0.002 [m**2/s**2]
CTHR_N = 0.003 [m**2/s**2]
CTHR_RDOT = 0.0001 [m**2/s**3]
CTHR_TDOT = 0.0002 [m**2/s**3]
CTHR_NDOT = 0.0003 [m**2/s**3]
CTHR_DRG = 0.00001 [m**3/(kg*s**2)]
CTHR_SRP = 0.00002 [m**3/(kg*s**2)]
CTHR_THR = 0.000001 [m**2/s**4]

OBJECT = OBJECT2
OBJECT_DESIGNATOR = 00002
CATALOG_NAME = CAT
OBJECT_NAME = OBJ2
INTERNATIONAL_DESIGNATOR = 1998-067B
EPHEMERIS_NAME = EPH2
COVARIANCE_METHOD = DEFAULT
MANEUVERABLE = NO
REF_FRAME = EME2000

X = -1.0 [km]
Y = -2.0 [km]
Z = -3.0 [km]
X_DOT = -0.1 [km/s]
Y_DOT = -0.2 [km/s]
Z_DOT = -0.3 [km/s]

CR_R = 1.0 [m**2]
CT_R = 0.0 [m**2]
CT_T = 1.0 [m**2]
CN_R = 0.0 [m**2]
CN_T = 0.0 [m**2]
CN_N = 1.0 [m**2]
CRDOT_R = 0.0 [m**2/s]
CRDOT_T = 0.0 [m**2/s]
CRDOT_N = 0.0 [m**2/s]
CRDOT_RDOT = 1.0 [m**2/s**2]
CTDOT_R = 0.0 [m**2/s]
CTDOT_T = 0.0 [m**2/s]
CTDOT_N = 0.0 [m**2/s]
CTDOT_RDOT = 0.0 [m**2/s**2]
CTDOT_TDOT = 1.0 [m**2/s**2]
CNDOT_R = 0.0 [m**2/s]
CNDOT_T = 0.0 [m**2/s]
CNDOT_N = 0.0 [m**2/s]
CNDOT_RDOT = 0.0 [m**2/s**2]
CNDOT_TDOT = 0.0 [m**2/s**2]
CNDOT_NDOT = 1.0 [m**2/s**2]
"#;
        let cdm = Cdm::from_kvn(kvn).expect("parse full cdm");
        let regenerated = cdm.to_kvn().expect("generate full kvn");
        let cdm2 = Cdm::from_kvn(&regenerated).expect("parse regenerated full cdm");

        // Verify key fields
        assert_eq!(cdm.header.message_for, cdm2.header.message_for);
        assert_eq!(
            cdm.body.relative_metadata_data.screen_volume_frame,
            cdm2.body.relative_metadata_data.screen_volume_frame
        );
        assert!(cdm.body.segments[0].data.od_parameters.is_some());
        assert!(cdm2.body.segments[0].data.od_parameters.is_some());
        assert!(cdm.body.segments[0]
            .data
            .covariance_matrix
            .cthr_thr
            .is_some());
        assert!(cdm2.body.segments[0]
            .data
            .covariance_matrix
            .cthr_thr
            .is_some());
    }
}
