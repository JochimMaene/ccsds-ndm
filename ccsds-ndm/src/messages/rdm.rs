// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{
    AtmosphericReentryParameters, GroundImpactParameters, OdParameters, OpmCovarianceMatrix,
    RdmSpacecraftParameters, StateVector,
};
use crate::error::Result;
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::types::{ControlledType, Epoch, ObjectDescription, PositionRequired, YesNo};
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------------
// Root RDM Structure
//----------------------------------------------------------------------

///
/// A message format for use in exchanging spacecraft re-entry information.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "rdm")]
pub struct Rdm {
    pub header: RdmHeader,
    pub body: RdmBody,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Ndm for Rdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        writer.write_pair("CCSDS_RDM_VERS", &self.version);
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
// Header
//----------------------------------------------------------------------

/// The RDM Header provides information about the message.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmHeader {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// File creation date and time in UTC.
    ///
    /// Examples: 2001-11-06T11:17:33, 2002-204T15:56:23
    pub creation_date: Epoch,
    /// Creating agency or entity.
    ///
    /// Examples: DLR, ESA
    pub originator: String,
    /// ID that uniquely identifies a message from a given originator.
    ///
    /// Examples: 201113719185, ESA20190101-3345
    pub message_id: String,
}

impl ToKvn for RdmHeader {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("CREATION_DATE", self.creation_date);
        writer.write_pair("ORIGINATOR", &self.originator);
        writer.write_pair("MESSAGE_ID", &self.message_id);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The RDM Body consists of a single segment.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RdmBody {
    pub segment: Box<RdmSegment>,
}

impl ToKvn for RdmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RdmSegment {
    /// The metadata for this RDM segment.
    pub metadata: RdmMetadata,
    /// The data for this RDM segment.
    pub data: RdmData,
}

impl ToKvn for RdmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// The RDM Metadata provides information about the re-entry event.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmMetadata {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// The name of the object.
    ///
    /// Examples: FENGYUN 1C, UARS, Tiangong-1
    pub object_name: String,
    /// The international designator of the object.
    ///
    /// Examples: 1999-025A, 1991-063B, 2011-053A
    pub international_designator: String,
    /// The catalog name for the object.
    ///
    /// Examples: SATCAT, SPCS, MCN
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    /// The object designator in the catalog.
    ///
    /// Examples: 25730, 21574, 37820
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_designator: Option<String>,
    /// The type of the object.
    ///
    /// Examples: PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, OTHER
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectDescription>,
    /// The owner of the object.
    ///
    /// Examples: China, USA, France
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_owner: Option<String>,
    /// The operator of the object.
    ///
    /// Examples: EUMETSAT, SES
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_operator: Option<String>,
    /// Whether the re-entry is controlled or not.
    ///
    /// Examples: YES, NO, UNKNOWN
    pub controlled_reentry: ControlledType,
    /// The celestial body the object is orbiting.
    ///
    /// Examples: EARTH, MOON, MARS
    pub center_name: String,
    /// The time system used for the message.
    ///
    /// Examples: UTC, TAI, TDB
    pub time_system: String,
    /// The reference epoch for the message.
    ///
    /// Examples: 2018-04-22T09:00:00.00
    pub epoch_tzero: Epoch,
    /// The reference frame of the state vector and covariance matrix.
    ///
    /// Examples: EME2000, GCRF, ICRF, ITRF2000, TDR
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame: Option<String>,
    /// The epoch of the reference frame.
    ///
    /// Examples: 2000-01-01T00:00:00.000
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_frame_epoch: Option<Epoch>,
    /// The name of the ephemeris used.
    ///
    /// Examples: DE430, JPLEPH.405
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeris_name: Option<String>,
    /// The gravity model used.
    ///
    /// Examples: EGM-96, JGM-3
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravity_model: Option<String>,
    /// The atmospheric model used.
    ///
    /// Examples: Jacchia 70, MSIS-86
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atmospheric_model: Option<String>,
    /// The solar flux and geomagnetic activity data used.
    ///
    /// Examples: F10.7_MEAN_81_CYCLE, SCHATTEN_ADJUSTED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_flux_prediction: Option<String>,
    /// The n-body perturbations used.
    ///
    /// Examples: MOON, SUN, JUPITER
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_body_perturbations: Option<String>,
    /// Whether solar radiation pressure was used.
    ///
    /// Examples: YES, NO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solar_rad_pressure: Option<String>,
    /// The Earth tides model used.
    ///
    /// Examples: ERS, IERS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earth_tides: Option<String>,
    /// Whether there was any intrack thrust.
    ///
    /// Examples: YES, NO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrack_thrust: Option<YesNo>,
    /// The source of the drag parameters.
    ///
    /// Examples: OD, DATABASE, DEFAULT
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_parameters_source: Option<String>,
    /// The altitude at which the drag parameters were estimated.
    ///
    /// Units: km
    ///
    /// Examples: 200.0 [km]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_parameters_altitude: Option<PositionRequired>,
    /// The method used to compute re-entry uncertainty.
    ///
    /// Examples: MONTE-CARLO, ANALYTICAL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_uncertainty_method: Option<String>,
    /// The method used to model the object’s disintegration.
    ///
    /// Examples: MASS-LOSS, BREAK-UP, NONE
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_disintegration: Option<String>,
    /// The method used to compute impact uncertainty.
    ///
    /// Examples: MONTE-CARLO, ANALYTICAL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_uncertainty_method: Option<String>,
    /// The ID of the previous message for this object.
    ///
    /// Examples: ESA/20180421-007
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_id: Option<String>,
    /// The epoch of the previous message for this object.
    ///
    /// Examples: 2018-04-21T09:00:00.00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_message_epoch: Option<Epoch>,
    /// The epoch of the next message for this object.
    ///
    /// Examples: 2018-04-23T09:00:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_message_epoch: Option<Epoch>,
}

impl ToKvn for RdmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("INTERNATIONAL_DESIGNATOR", &self.international_designator);
        if let Some(v) = &self.catalog_name {
            writer.write_pair("CATALOG_NAME", v);
        }
        if let Some(v) = &self.object_designator {
            writer.write_pair("OBJECT_DESIGNATOR", v);
        }
        if let Some(v) = &self.object_type {
            writer.write_pair("OBJECT_TYPE", v);
        }
        if let Some(v) = &self.object_owner {
            writer.write_pair("OBJECT_OWNER", v);
        }
        if let Some(v) = &self.object_operator {
            writer.write_pair("OBJECT_OPERATOR", v);
        }
        writer.write_pair("CONTROLLED_REENTRY", &self.controlled_reentry);
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("EPOCH_TZERO", self.epoch_tzero);
        if let Some(v) = &self.ref_frame {
            writer.write_pair("REF_FRAME", v);
        }
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        if let Some(v) = &self.ephemeris_name {
            writer.write_pair("EPHEMERIS_NAME", v);
        }
        if let Some(v) = &self.gravity_model {
            writer.write_pair("GRAVITY_MODEL", v);
        }
        if let Some(v) = &self.atmospheric_model {
            writer.write_pair("ATMOSPHERIC_MODEL", v);
        }
        if let Some(v) = &self.solar_flux_prediction {
            writer.write_pair("SOLAR_FLUX_PREDICTION", v);
        }
        if let Some(v) = &self.n_body_perturbations {
            writer.write_pair("N_BODY_PERTURBATIONS", v);
        }
        if let Some(v) = &self.solar_rad_pressure {
            writer.write_pair("SOLAR_RAD_PRESSURE", v);
        }
        if let Some(v) = &self.earth_tides {
            writer.write_pair("EARTH_TIDES", v);
        }
        if let Some(v) = &self.intrack_thrust {
            writer.write_pair("INTRACK_THRUST", v);
        }
        if let Some(v) = &self.drag_parameters_source {
            writer.write_pair("DRAG_PARAMETERS_SOURCE", v);
        }
        if let Some(v) = &self.drag_parameters_altitude {
            writer.write_pair("DRAG_PARAMETERS_ALTITUDE", v);
        }
        if let Some(v) = &self.reentry_uncertainty_method {
            writer.write_pair("REENTRY_UNCERTAINTY_METHOD", v);
        }
        if let Some(v) = &self.reentry_disintegration {
            writer.write_pair("REENTRY_DISINTEGRATION", v);
        }
        if let Some(v) = &self.impact_uncertainty_method {
            writer.write_pair("IMPACT_UNCERTAINTY_METHOD", v);
        }
        if let Some(v) = &self.previous_message_id {
            writer.write_pair("PREVIOUS_MESSAGE_ID", v);
        }
        if let Some(v) = &self.previous_message_epoch {
            writer.write_pair("PREVIOUS_MESSAGE_EPOCH", v);
        }
        if let Some(v) = &self.next_message_epoch {
            writer.write_pair("NEXT_MESSAGE_EPOCH", v);
        }
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// The RDM Data section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RdmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// Atmospheric re-entry parameters.
    #[serde(rename = "atmosphericReentryParameters")]
    pub atmospheric_reentry_parameters: AtmosphericReentryParameters,
    /// Ground impact parameters.
    #[serde(
        rename = "groundImpactParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ground_impact_parameters: Option<GroundImpactParameters>,
    /// State vector.
    #[serde(
        rename = "stateVector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_vector: Option<StateVector>,
    /// Position/velocity covariance matrix.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
    /// Spacecraft parameters.
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<RdmSpacecraftParameters>,
    /// Orbit determination parameters.
    #[serde(
        rename = "odParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub od_parameters: Option<OdParameters>,
    /// User defined parameters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_defined_parameters: Vec<(String, String)>,
}

impl ToKvn for RdmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // No DATA_START
        writer.write_comments(&self.comment);
        // Atmospheric (mandatory)
        let a = &self.atmospheric_reentry_parameters;
        writer.write_pair("ORBIT_LIFETIME", &a.orbit_lifetime);
        writer.write_pair("REENTRY_ALTITUDE", &a.reentry_altitude);
        if let Some(v) = &a.orbit_lifetime_window_start {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_START", v);
        }
        if let Some(v) = &a.orbit_lifetime_window_end {
            writer.write_pair("ORBIT_LIFETIME_WINDOW_END", v);
        }
        if let Some(v) = &a.nominal_reentry_epoch {
            writer.write_pair("NOMINAL_REENTRY_EPOCH", v);
        }
        if let Some(v) = &a.reentry_window_start {
            writer.write_pair("REENTRY_WINDOW_START", v);
        }
        if let Some(v) = &a.reentry_window_end {
            writer.write_pair("REENTRY_WINDOW_END", v);
        }
        if let Some(v) = &a.orbit_lifetime_confidence_level {
            writer.write_pair("ORBIT_LIFETIME_CONFIDENCE_LEVEL", v);
        }

        // Ground impact (optional)
        if let Some(g) = &self.ground_impact_parameters {
            if let Some(v) = &g.probability_of_impact {
                writer.write_pair("PROBABILITY_OF_IMPACT", v);
            }
            if let Some(v) = &g.probability_of_burn_up {
                writer.write_pair("PROBABILITY_OF_BURN_UP", v);
            }
            if let Some(v) = &g.probability_of_break_up {
                writer.write_pair("PROBABILITY_OF_BREAK_UP", v);
            }
            if let Some(v) = &g.probability_of_land_impact {
                writer.write_pair("PROBABILITY_OF_LAND_IMPACT", v);
            }
            if let Some(v) = &g.probability_of_casualty {
                writer.write_pair("PROBABILITY_OF_CASUALTY", v);
            }
            if let Some(v) = &g.nominal_impact_epoch {
                writer.write_pair("NOMINAL_IMPACT_EPOCH", v);
            }
            if let Some(v) = &g.impact_window_start {
                writer.write_pair("IMPACT_WINDOW_START", v);
            }
            if let Some(v) = &g.impact_window_end {
                writer.write_pair("IMPACT_WINDOW_END", v);
            }
            if let Some(v) = &g.impact_ref_frame {
                writer.write_pair("IMPACT_REF_FRAME", v);
            }
            if let Some(v) = &g.nominal_impact_lon {
                writer.write_pair("NOMINAL_IMPACT_LON", v);
            }
            if let Some(v) = &g.nominal_impact_lat {
                writer.write_pair("NOMINAL_IMPACT_LAT", v);
            }
            if let Some(v) = &g.nominal_impact_alt {
                writer.write_pair("NOMINAL_IMPACT_ALT", v);
            }
            if let Some(v) = &g.impact_1_confidence {
                writer.write_pair("IMPACT_1_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_1_start_lon {
                writer.write_pair("IMPACT_1_START_LON", v);
            }
            if let Some(v) = &g.impact_1_start_lat {
                writer.write_pair("IMPACT_1_START_LAT", v);
            }
            if let Some(v) = &g.impact_1_stop_lon {
                writer.write_pair("IMPACT_1_STOP_LON", v);
            }
            if let Some(v) = &g.impact_1_stop_lat {
                writer.write_pair("IMPACT_1_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_1_cross_track {
                writer.write_pair("IMPACT_1_CROSS_TRACK", v);
            }
            if let Some(v) = &g.impact_2_confidence {
                writer.write_pair("IMPACT_2_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_2_start_lon {
                writer.write_pair("IMPACT_2_START_LON", v);
            }
            if let Some(v) = &g.impact_2_start_lat {
                writer.write_pair("IMPACT_2_START_LAT", v);
            }
            if let Some(v) = &g.impact_2_stop_lon {
                writer.write_pair("IMPACT_2_STOP_LON", v);
            }
            if let Some(v) = &g.impact_2_stop_lat {
                writer.write_pair("IMPACT_2_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_2_cross_track {
                writer.write_pair("IMPACT_2_CROSS_TRACK", v);
            }
            if let Some(v) = &g.impact_3_confidence {
                writer.write_pair("IMPACT_3_CONFIDENCE", v);
            }
            if let Some(v) = &g.impact_3_start_lon {
                writer.write_pair("IMPACT_3_START_LON", v);
            }
            if let Some(v) = &g.impact_3_start_lat {
                writer.write_pair("IMPACT_3_START_LAT", v);
            }
            if let Some(v) = &g.impact_3_stop_lon {
                writer.write_pair("IMPACT_3_STOP_LON", v);
            }
            if let Some(v) = &g.impact_3_stop_lat {
                writer.write_pair("IMPACT_3_STOP_LAT", v);
            }
            if let Some(v) = &g.impact_3_cross_track {
                writer.write_pair("IMPACT_3_CROSS_TRACK", v);
            }
        }

        // Optional blocks: write when present
        if let Some(sv) = &self.state_vector {
            sv.write_kvn(writer);
        }
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }
        if let Some(sp) = &self.spacecraft_parameters {
            // Write minimal known fields
            if let Some(v) = &sp.wet_mass {
                writer.write_pair("WET_MASS", v);
            }
            if let Some(v) = &sp.dry_mass {
                writer.write_pair("DRY_MASS", v);
            }
            if let Some(v) = &sp.hazardous_substances {
                writer.write_pair("HAZARDOUS_SUBSTANCES", v);
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_pair("SOLAR_RAD_AREA", v);
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_pair("SOLAR_RAD_COEFF", v);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_pair("DRAG_AREA", v);
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_pair("DRAG_COEFF", v);
            }
            if let Some(v) = &sp.rcs {
                writer.write_pair("RCS", v);
            }
            if let Some(v) = &sp.ballistic_coeff {
                writer.write_pair("BALLISTIC_COEFF", v);
            }
            if let Some(v) = &sp.thrust_acceleration {
                writer.write_pair("THRUST_ACCELERATION", v);
            }
        }
        if let Some(od) = &self.od_parameters {
            if let Some(v) = &od.time_lastob_start {
                writer.write_pair("TIME_LASTOB_START", v);
            }
            if let Some(v) = &od.time_lastob_end {
                writer.write_pair("TIME_LASTOB_END", v);
            }
            if let Some(v) = &od.recommended_od_span {
                writer.write_pair("RECOMMENDED_OD_SPAN", v);
            }
            if let Some(v) = &od.actual_od_span {
                writer.write_pair("ACTUAL_OD_SPAN", v);
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
                writer.write_pair("RESIDUALS_ACCEPTED", v);
            }
            if let Some(v) = &od.weighted_rms {
                writer.write_pair("WEIGHTED_RMS", v);
            }
        }

        for (k, v) in &self.user_defined_parameters {
            writer.write_pair(k, v);
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kitchen_sink_roundtrip() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = COMPREHENSIVE_TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T09:00:00
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2023-01-06T19:45:33
REENTRY_WINDOW_START = 2023-01-06T11:45:33
REENTRY_WINDOW_END = 2023-01-06T22:12:56
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.75
EPOCH = 2023-01-01T09:30:12
X = 4000.000000 [km]
Y = 4000.000000 [km]
Z = 4000.000000 [km]
X_DOT = 7.000000 [km/s]
Y_DOT = 7.000000 [km/s]
Z_DOT = 7.000000 [km/s]
COV_REF_FRAME = RTN
CX_X = 0.10000 [km**2]
CY_X = 0.10000 [km**2]
CY_Y = 0.10000 [km**2]
CZ_X = 0.10000 [km**2]
CZ_Y = 0.10000 [km**2]
CZ_Z = 0.10000 [km**2]
CX_DOT_X = 0.02000 [km**2/s]
CX_DOT_Y = 0.02000 [km**2/s]
CX_DOT_Z = 0.02000 [km**2/s]
CX_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_X = 0.02000 [km**2/s]
CY_DOT_Y = 0.02000 [km**2/s]
CY_DOT_Z = 0.02000 [km**2/s]
CY_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
CZ_DOT_X = 0.02000 [km**2/s]
CZ_DOT_Y = 0.02000 [km**2/s]
CZ_DOT_Z = 0.02000 [km**2/s]
CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
WET_MASS = 3582 [kg]
DRAG_AREA = 23.3565 [m**2]
DRAG_COEFF = 2.2634
ACTUAL_OD_SPAN = 3.4554 [d]
TRACKS_AVAILABLE = 18
TRACKS_USED = 17
USER_DEFINED_TEST = VALUE
"#;
        let rdm = Rdm::from_kvn(kvn).expect("parse kvn");
        let generated = rdm.to_kvn().expect("generate kvn");
        let rdm2 = Rdm::from_kvn(&generated).expect("parse generated kvn");

        assert_eq!(rdm.header, rdm2.header);
        assert_eq!(
            rdm.body.segment.metadata.object_name,
            rdm2.body.segment.metadata.object_name
        );
        assert_eq!(
            rdm.body.segment.data.user_defined_parameters,
            rdm2.body.segment.data.user_defined_parameters
        );
    }

    /// Parse official RDM XML example C-3 (minimal)
    #[test]
    fn test_xsd_rdm_sample_c3_xml() {
        let xml = std::fs::read_to_string("../data/xml/rdm_c3.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.version, "1.0");
        assert_eq!(rdm.header.originator, "ESA");
        assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    }

    /// Parse official RDM XML example C-4 (comprehensive)
    #[test]
    fn test_xsd_rdm_sample_c4_xml() {
        let xml = std::fs::read_to_string("../data/xml/rdm_c4.xml").unwrap();
        let rdm = Rdm::from_xml(&xml).unwrap();
        assert_eq!(rdm.header.message_id, "ESA/20180422-001");
        assert!(rdm.body.segment.data.ground_impact_parameters.is_some());
        assert!(rdm.body.segment.data.state_vector.is_some());
        assert!(rdm.body.segment.data.covariance_matrix.is_some());
        assert!(rdm.body.segment.data.spacecraft_parameters.is_some());
        assert!(rdm.body.segment.data.od_parameters.is_some());
    }

    #[test]
    fn test_rdm_xml_roundtrip_minimal() {
        let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
        let rdm = Rdm::from_kvn(kvn).unwrap();
        let xml = rdm.to_xml().unwrap();
        assert!(xml.contains("<rdm"));
        assert!(xml.contains("OBJECT_NAME"));
        let rdm2 = Rdm::from_xml(&xml).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.object_name,
            rdm2.body.segment.metadata.object_name
        );
    }
}
