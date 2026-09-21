// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters, StateVector};
use crate::error::{Result, ValidationError};
use crate::traits::{Ndm, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod kvn;
mod xml;

//----------------------------------------------------------------------
// Root OPM Structure
//----------------------------------------------------------------------

/// Orbit Parameter Message (OPM).
///
/// Orbit information may be exchanged between two participants by sending a state vector (see
/// reference \[H1\]) for a specified epoch using an OPM. The message recipient must have an orbit
/// propagator available that is able to propagate the OPM state vector to compute the orbit at other
/// desired epochs. For this propagation, additional ancillary information (spacecraft properties
/// such as mass, area, and maneuver planning data, if applicable) may be included with the message.
///
/// **CCSDS Reference**: 502.0-B-3, Section 3.1.1.
#[derive(Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "opm")]
pub struct Opm {
    pub header: OdmHeader,
    pub body: OpmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_OPM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "3.0".to_string(), into)]
    pub version: String,
}

impl crate::traits::Validate for Opm {
    fn validate(&self) -> Result<()> {
        crate::validation::validate_at_field_path(
            crate::versioning::validate_root(
                crate::validation::MessageKind::Opm,
                &self.id,
                &self.version,
            ),
            "",
        )?;
        crate::versioning::validate_opm_edition(self)?;
        self.header.validate()?;
        self.body.validate()?;
        Ok(())
    }
}

impl Ndm for Opm {
    fn to_kvn(&self) -> Result<String> {
        crate::generation::to_kvn_string(self)
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        Self::from_kvn_strict(kvn)
    }

    fn to_xml(&self) -> Result<String> {
        crate::generation::to_xml_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        Self::from_xml_strict(xml)
    }

    fn write_kvn_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_kvn_to(self, output)
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        crate::generation::write_xml_to(self, output)
    }
}

/// A diagnostic field path that is only materialized when an error is reported.
///
/// Repeated blocks need their index in the path, which cannot be a `&'static str`; resolving
/// lazily keeps the validation passes allocation-free while they are finding nothing wrong.
#[derive(Clone, Copy)]
enum FieldPath {
    Fixed(&'static str),
    /// A field of `maneuver_parameters[index]`.
    Maneuver(usize, &'static str),
    /// The `maneuver_parameters[index]` block itself.
    ManeuverBlock(usize),
}

impl FieldPath {
    fn resolve(self) -> Cow<'static, str> {
        match self {
            Self::Fixed(path) => Cow::Borrowed(path),
            Self::Maneuver(index, field) => Cow::Owned(format!(
                "body.segment.data.maneuver_parameters[{index}].{field}"
            )),
            Self::ManeuverBlock(index) => {
                Cow::Owned(format!("body.segment.data.maneuver_parameters[{index}]"))
            }
        }
    }
}

impl From<&'static str> for FieldPath {
    fn from(path: &'static str) -> Self {
        Self::Fixed(path)
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

/// The body of the OPM, containing a single segment.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OpmBody {
    #[serde(rename = "segment")]
    pub segment: OpmSegment,
}

impl crate::traits::Validate for OpmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }
}

/// A single segment of the OPM.
///
/// Contains metadata and data sections.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(deny_unknown_fields)]
pub struct OpmSegment {
    pub metadata: OpmMetadata,
    pub data: OpmData,
}

impl crate::traits::Validate for OpmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate()
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// OPM Metadata Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct OpmMetadata {
    /// Comments (allowed at the beginning of the OPM Metadata). (See 7.8 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which orbit state data is provided. While there is no CCSDS-based
    /// restriction on the value for this keyword, it is recommended to use names from the UN
    /// Office of Outer Space Affairs designator index (reference ``[3]``, which include Object name
    /// and international designator of the participant). If OBJECT_NAME is not listed in reference
    /// `[3]` or the content is either unknown or cannot be disclosed, the value should be set to
    /// UNKNOWN.
    ///
    /// **Examples**: EUTELSAT W1 MARS PATHFINDER STS 106 NEAR UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Object identifier of the object for which orbit state data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use the
    /// international spacecraft designator as published in the UN Office of Outer Space Affairs
    /// designator index (reference ``[3]``). Recommended values have the format YYYY-NNNP{PP}, where:
    /// YYYY = Year of launch. NNN = Three-digit serial number of launch in year YYYY (with leading
    /// zeros). P{PP} = At least one capital letter for the identification of the part brought into
    /// space by the launch. If the asset is not listed in reference ``[3]``, the UN Office of Outer
    /// Space Affairs designator index format is not used, or the content is either unknown or
    /// cannot be disclosed, the value should be set to UNKNOWN.
    ///
    /// **Examples**: 2000-052A 1996-068A 2000-053A 1996-008A UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Origin of the OPM reference frame, which shall be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. Natural bodies shall be selected from the accepted set of values
    /// indicated in annex B, subsection B2.
    ///
    /// **Examples**: EARTH EARTH BARYCENTER MOON SOLAR SYSTEM BARYCENTER SUN JUPITER BARYCENTER
    /// STS 106 EROS
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[builder(into)]
    pub center_name: String,
    /// Reference frame in which the state vector and optional Keplerian element data are given.
    /// Use of values other than those in 3.2.3.3 should be documented in an ICD.
    ///
    /// **Examples**: ICRF EME2000 ITRF2000 TEME
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[builder(into)]
    pub ref_frame: String,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame. (See
    /// 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_epoch: Option<CalendarEpoch>,
    /// Time system used for state vector, maneuver, and covariance data. Use of values other than
    /// those in 3.2.3.2 should be documented in an ICD.
    ///
    /// **Examples**: UTC, TAI, TT, GPS, TDB, TCB
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.3.
    #[builder(into)]
    pub time_system: String,
}

impl crate::traits::Validate for OpmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OPM Metadata".into(),
                field: "OBJECT_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OPM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.center_name.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OPM Metadata".into(),
                field: "CENTER_NAME".into(),
                line: None,
            }
            .into());
        }
        if self.ref_frame.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OPM Metadata".into(),
                field: "REF_FRAME".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OPM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// OPM Data Section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct OpmData {
    /// Comments (see 7.8 for formatting rules).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,

    /// State vector components (position and velocity).
    #[serde(rename = "stateVector")]
    pub state_vector: StateVector,

    /// Osculating Keplerian elements.
    #[serde(
        rename = "keplerianElements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub keplerian_elements: Option<KeplerianElements>,

    /// Spacecraft physical parameters (mass, area, coefficients).
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<SpacecraftParameters>,

    /// Position/velocity covariance matrix.
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,

    /// Maneuver parameters.
    #[serde(
        rename = "maneuverParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub maneuver_parameters: Vec<ManeuverParameters>,

    /// User-defined parameters.
    #[serde(
        rename = "userDefinedParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_parameters: Option<UserDefined>,
}

impl Validate for OpmData {
    fn validate(&self) -> Result<()> {
        crate::validation::validate_at_field_path(
            self.state_vector.validate(),
            "body.segment.data.state_vector",
        )?;
        if let Some(ke) = &self.keplerian_elements {
            crate::validation::validate_at_field_path(
                ke.validate(),
                "body.segment.data.keplerian_elements",
            )?;
        }
        if let Some(parameters) = &self.spacecraft_parameters {
            crate::validation::validate_at_field_path(
                parameters.validate(),
                "body.segment.data.spacecraft_parameters",
            )?;
        }
        if let Some(covariance) = &self.covariance_matrix {
            crate::validation::validate_at_field_path(
                covariance.validate(),
                "body.segment.data.covariance_matrix",
            )?;
        }
        for (index, maneuver) in self.maneuver_parameters.iter().enumerate() {
            match maneuver.validate() {
                Ok(()) => {}
                // Only the failing maneuver needs its indexed path built.
                error => crate::validation::validate_at_field_path(
                    error,
                    FieldPath::ManeuverBlock(index).resolve(),
                )?,
            }
        }
        if !self.maneuver_parameters.is_empty()
            && self
                .spacecraft_parameters
                .as_ref()
                .and_then(|sp| sp.mass.as_ref())
                .is_none()
        {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("Spacecraft Parameters"),
                field: Cow::Borrowed("MASS"),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Keplerian Elements
//----------------------------------------------------------------------

/// Osculating Keplerian Elements in the Specified Reference Frame (none or all parameters of
/// this block must be given).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct KeplerianElements {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Semi-major axis
    ///
    /// **Units**: km
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub semi_major_axis: Distance,
    /// Eccentricity
    ///
    /// **Units**: n/a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub eccentricity: NonNegativeDouble,
    /// Inclination
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub inclination: Inclination,
    /// Right ascension of ascending node
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub ra_of_asc_node: Angle,
    /// Argument of pericenter
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub arg_of_pericenter: Angle,
    /// True anomaly or mean anomaly
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub true_anomaly: Option<Angle>,
    /// True anomaly or mean anomaly
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub mean_anomaly: Option<Angle>,
    /// Gravitational Coefficient (Gravitational Constant × Central Mass)
    ///
    /// **Units**: km³/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub gm: Gm,
}

impl crate::traits::Validate for KeplerianElements {
    fn validate(&self) -> Result<()> {
        let semi_major_axis = self.semi_major_axis.value;
        if !semi_major_axis.is_finite() {
            return Err(ValidationError::InvalidValue {
                field: "SEMI_MAJOR_AXIS".into(),
                value: semi_major_axis.to_string(),
                expected: "a finite number".into(),
                line: None,
            }
            .into());
        }
        let eccentricity = self.eccentricity.value;
        if !eccentricity.is_finite() {
            return Err(ValidationError::InvalidValue {
                field: "ECCENTRICITY".into(),
                value: eccentricity.to_string(),
                expected: "a finite number".into(),
                line: None,
            }
            .into());
        }
        if eccentricity < 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "ECCENTRICITY".into(),
                value: eccentricity.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        let inclination = self.inclination.angle.value;
        if !inclination.is_finite() {
            return Err(ValidationError::InvalidValue {
                field: "INCLINATION".into(),
                value: inclination.to_string(),
                expected: "a finite number".into(),
                line: None,
            }
            .into());
        }
        if !(0.0..=180.0).contains(&inclination) {
            return Err(ValidationError::OutOfRange {
                name: "INCLINATION".into(),
                value: inclination.to_string(),
                expected: "[0, 180]".into(),
                line: None,
            }
            .into());
        }
        for (field, angle) in [
            ("RA_OF_ASC_NODE", Some(&self.ra_of_asc_node)),
            ("ARG_OF_PERICENTER", Some(&self.arg_of_pericenter)),
            ("TRUE_ANOMALY", self.true_anomaly.as_ref()),
            ("MEAN_ANOMALY", self.mean_anomaly.as_ref()),
        ] {
            let Some(angle) = angle else { continue };
            if !angle.value.is_finite() {
                return Err(ValidationError::InvalidValue {
                    field: field.into(),
                    value: angle.value.to_string(),
                    expected: "a finite number".into(),
                    line: None,
                }
                .into());
            }
            if !(-360.0..360.0).contains(&angle.value) {
                return Err(ValidationError::OutOfRange {
                    name: field.into(),
                    value: angle.value.to_string(),
                    expected: "[-360, 360)".into(),
                    line: None,
                }
                .into());
            }
        }
        Gm::validate_value(self.gm.value, "GM")?;
        if self.true_anomaly.is_some() == self.mean_anomaly.is_some() {
            let selected = [
                ("TRUE_ANOMALY", self.true_anomaly.is_some()),
                ("MEAN_ANOMALY", self.mean_anomaly.is_some()),
            ]
            .into_iter()
            .filter(|(_, present)| *present)
            .map(|(name, _)| Cow::Borrowed(name))
            .collect();
            return Err(ValidationError::InvalidChoice {
                fields: vec![Cow::Borrowed("TRUE_ANOMALY"), Cow::Borrowed("MEAN_ANOMALY")],
                selected,
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Maneuver Parameters
//----------------------------------------------------------------------

/// Maneuver Parameters (Repeat for each maneuver).
///
/// References:
/// - CCSDS 502.0-B-3, Section 3.2.4 (OPM Data Section)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub struct ManeuverParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of ignition (see 7.5.10 for formatting rules)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub man_epoch_ignition: CalendarEpoch,
    /// Maneuver duration (If = 0, impulsive maneuver)
    ///
    /// **Units**: s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub man_duration: Duration,
    /// Mass change during maneuver (value is < 0)
    ///
    /// **Units**: kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    ///
    /// The applicable XML schema uses `deltamassTypeZ`, so zero is allowed.
    pub man_delta_mass: DeltaMassZ,
    /// Reference frame in which the velocity increment vector data are given. The user must
    /// select from the accepted set of values indicated in 3.2.4.11.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    #[builder(into)]
    pub man_ref_frame: String,
    /// 1st component of the velocity increment
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub man_dv_1: Velocity,
    /// 2nd component of the velocity increment
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub man_dv_2: Velocity,
    /// 3rd component of the velocity increment
    ///
    /// **Units**: km/s
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 3.2.4.
    pub man_dv_3: Velocity,
}

impl Validate for ManeuverParameters {
    fn validate(&self) -> Result<()> {
        if self.man_epoch_ignition.is_empty() {
            return Err(ValidationError::missing_required(
                "Maneuver Parameters",
                "MAN_EPOCH_IGNITION",
            )
            .into());
        }
        if self.man_ref_frame.trim().is_empty() {
            return Err(
                ValidationError::missing_required("Maneuver Parameters", "MAN_REF_FRAME").into(),
            );
        }
        let duration = self.man_duration.value;
        if !duration.is_finite() {
            return Err(ValidationError::InvalidValue {
                field: "MAN_DURATION".into(),
                value: duration.to_string(),
                expected: "a finite number".into(),
                line: None,
            }
            .into());
        }
        if duration < 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "MAN_DURATION".into(),
                value: duration.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        // `durationType` declares `timeUnits`, which permits only `s`. `TimeUnits` is shared with
        // messages that also allow days, so accept the permitted spelling rather than excluding
        // today's other variants: a new variant must not silently become valid here.
        if !matches!(self.man_duration.units, None | Some(TimeUnits::Seconds)) {
            return Err(ValidationError::InvalidValue {
                field: "MAN_DURATION units".into(),
                value: self
                    .man_duration
                    .units
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                expected: "s or omitted".into(),
                line: None,
            }
            .into());
        }
        let delta_mass = self.man_delta_mass.value;
        if !delta_mass.is_finite() {
            return Err(ValidationError::InvalidValue {
                field: "MAN_DELTA_MASS".into(),
                value: delta_mass.to_string(),
                expected: "a finite number".into(),
                line: None,
            }
            .into());
        }
        if delta_mass > 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "MAN_DELTA_MASS".into(),
                value: delta_mass.to_string(),
                expected: "<= 0".into(),
                line: None,
            }
            .into());
        }
        for (field, value) in [
            ("MAN_DV_1", self.man_dv_1.value),
            ("MAN_DV_2", self.man_dv_2.value),
            ("MAN_DV_3", self.man_dv_3.value),
        ] {
            if !value.is_finite() {
                return Err(ValidationError::InvalidValue {
                    field: field.into(),
                    value: value.to_string(),
                    expected: "a finite number".into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }
}
