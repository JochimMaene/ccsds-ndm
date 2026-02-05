// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters};
use crate::error::{EnumParseError, Result, ValidationError};
use crate::kvn::parser::ParseKvn;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn, Validate};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

//----------------------------------------------------------------------
// OMM Specific Units
//----------------------------------------------------------------------

// 1/ER (Inverse Earth Radii) for BSTAR
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum InvErUnits {
    #[serde(rename = "1/ER")]
    #[default]
    InvEr,
}
impl std::fmt::Display for InvErUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1/ER")
    }
}
impl FromStr for InvErUnits {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1/ER" => Ok(InvErUnits::InvEr),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "1/ER",
            }),
        }
    }
}
pub type BStar = UnitValue<f64, InvErUnits>;

// rev/day for MEAN_MOTION
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDayUnits {
    #[serde(rename = "rev/day")]
    #[default]
    RevPerDay,
    #[serde(rename = "REV/DAY")]
    RevPerDayUpper,
}
impl std::fmt::Display for RevPerDayUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDayUnits::RevPerDay => write!(f, "rev/day"),
            RevPerDayUnits::RevPerDayUpper => write!(f, "REV/DAY"),
        }
    }
}
impl FromStr for RevPerDayUnits {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day" => Ok(RevPerDayUnits::RevPerDay),
            "REV/DAY" => Ok(RevPerDayUnits::RevPerDayUpper),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day or REV/DAY",
            }),
        }
    }
}
pub type MeanMotion = UnitValue<f64, RevPerDayUnits>;

// rev/day**2 for MEAN_MOTION_DOT
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDay2Units {
    #[serde(rename = "rev/day**2")]
    #[default]
    RevPerDay2,
    #[serde(rename = "REV/DAY**2")]
    RevPerDay2Upper,
}
impl std::fmt::Display for RevPerDay2Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDay2Units::RevPerDay2 => write!(f, "rev/day**2"),
            RevPerDay2Units::RevPerDay2Upper => write!(f, "REV/DAY**2"),
        }
    }
}
impl FromStr for RevPerDay2Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**2" => Ok(RevPerDay2Units::RevPerDay2),
            "REV/DAY**2" => Ok(RevPerDay2Units::RevPerDay2Upper),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day**2 or REV/DAY**2",
            }),
        }
    }
}
pub type MeanMotionDot = UnitValue<f64, RevPerDay2Units>;

// rev/day**3 for MEAN_MOTION_DDOT
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum RevPerDay3Units {
    #[serde(rename = "rev/day**3")]
    #[default]
    RevPerDay3,
    #[serde(rename = "REV/DAY**3")]
    RevPerDay3Upper,
}
impl std::fmt::Display for RevPerDay3Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RevPerDay3Units::RevPerDay3 => write!(f, "rev/day**3"),
            RevPerDay3Units::RevPerDay3Upper => write!(f, "REV/DAY**3"),
        }
    }
}
impl FromStr for RevPerDay3Units {
    type Err = EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rev/day**3" => Ok(RevPerDay3Units::RevPerDay3),
            "REV/DAY**3" => Ok(RevPerDay3Units::RevPerDay3Upper),
            _ => Err(EnumParseError {
                field: "unit",
                value: s.to_string(),
                expected: "rev/day**3 or REV/DAY**3",
            }),
        }
    }
}
pub type MeanMotionDDot = UnitValue<f64, RevPerDay3Units>;

//----------------------------------------------------------------------
// Root OMM Structure
//----------------------------------------------------------------------

/// Orbit Mean-Elements Message (OMM).
///
/// The OMM contains the orbital characteristics of a single object at a specified epoch,
/// expressed in mean Keplerian elements: mean motion, eccentricity, inclination, right
/// ascension of ascending node, argument of perigee, and mean anomaly.
///
/// These elements are adequate for providing the initial mean state of analytical and
/// semi-analytical orbit models (e.g., SGP4). The OMM includes keywords and values that may
/// be used to generate canonical NORAD Two Line Element (TLE) sets to accommodate the needs
/// of heritage users.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "omm")]
pub struct Omm {
    pub header: OdmHeader,
    pub body: OmmBody,
    #[serde(rename = "@id")]
    #[builder(required, default = Some("CCSDS_OMM_VERS".to_string()))]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    #[builder(default = "3.0".to_string(), into)]
    pub version: String,
}

/// Optional overrides when constructing a minimal OMM from TLE lines.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TleToOmmOptions {
    /// Spacecraft name to use in generated metadata. Defaults to `UNKNOWN`.
    pub object_name: Option<String>,
    /// Object identifier to use in generated metadata.
    /// Defaults to the international designator derived from TLE line 1.
    pub object_id: Option<String>,
    /// Originator value for the generated ODM header. Defaults to `UNKNOWN`.
    pub originator: Option<String>,
    /// Optional message ID for the generated ODM header.
    pub message_id: Option<String>,
    /// Optional creation date override for the generated ODM header.
    /// Defaults to the parsed TLE epoch.
    pub creation_date: Option<Epoch>,
}

impl crate::traits::Validate for Omm {
    fn validate(&self) -> Result<()> {
        crate::versioning::validate_root(
            crate::validation::MessageKind::Omm,
            &self.id,
            &self.version,
        )?;
        self.header.validate()?;
        self.body.validate()
    }
}

impl Ndm for Omm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let omm = Self::from_kvn_str(kvn)?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Omm, &omm)?;
        Ok(omm)
    }

    fn to_xml(&self) -> Result<String> {
        self.validate()?;
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let omm: Self = crate::xml::from_str_with_context(xml, "OMM")?;
        crate::validation::validate_with_mode(crate::validation::MessageKind::Omm, &omm)?;
        Ok(omm)
    }
}

impl Omm {
    /// Validates the OMM against CCSDS constraints that cannot be checked during parsing.
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;
        self.body.segment.validate()
    }

    /// Generate canonical NORAD TLE line 1 and line 2 from this OMM.
    ///
    /// This method requires:
    /// - `MEAN_ELEMENT_THEORY` to be `SGP` or `SGP4`
    /// - presence of `TLE_PARAMETERS`
    /// - presence of launch designator fields in `OBJECT_ID` (`YYYY-NNNPPP`)
    /// - strict field-width compliance for NORAD fixed-width line formats
    pub fn to_tle_lines(&self) -> Result<(String, String)> {
        self.validate()?;

        let metadata = &self.body.segment.metadata;
        let data = &self.body.segment.data;
        let tle = data
            .tle_parameters
            .as_ref()
            .ok_or(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("OMM Data"),
                field: Cow::Borrowed("TLE_PARAMETERS"),
                line: None,
            })?;

        let theory = metadata.mean_element_theory.trim();
        if !matches!(theory, "SGP" | "SGP4") {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("MEAN_ELEMENT_THEORY"),
                value: theory.to_string(),
                expected: Cow::Borrowed("SGP or SGP4"),
                line: None,
            }
            .into());
        }

        if metadata.time_system.trim().to_uppercase() != "UTC" {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("TIME_SYSTEM"),
                value: metadata.time_system.clone(),
                expected: Cow::Borrowed("UTC"),
                line: None,
            }
            .into());
        }

        let launch = parse_object_id_launch_designator(&metadata.object_id)?;
        let norad_cat_id = require_tle_field(tle.norad_cat_id, "NORAD_CAT_ID")?;
        if norad_cat_id > 99_999 {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("NORAD_CAT_ID"),
                value: norad_cat_id.to_string(),
                expected: Cow::Borrowed("[0, 99999]"),
                line: None,
            }
            .into());
        }

        let classification =
            tle.classification_type
                .as_ref()
                .ok_or(ValidationError::MissingRequiredField {
                    block: Cow::Borrowed("TLE Parameters"),
                    field: Cow::Borrowed("CLASSIFICATION_TYPE"),
                    line: None,
                })?;
        let classification_char = parse_classification_char(classification)?;

        let ephemeris_type = require_tle_field(tle.ephemeris_type, "EPHEMERIS_TYPE")?;
        if !(0..=9).contains(&ephemeris_type) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPHEMERIS_TYPE"),
                value: ephemeris_type.to_string(),
                expected: Cow::Borrowed("[0, 9]"),
                line: None,
            }
            .into());
        }

        let element_set_no = require_tle_field(tle.element_set_no, "ELEMENT_SET_NO")?;
        let rev_at_epoch = require_tle_field(tle.rev_at_epoch, "REV_AT_EPOCH")?;
        if rev_at_epoch > 99_999 {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("REV_AT_EPOCH"),
                value: rev_at_epoch.to_string(),
                expected: Cow::Borrowed("[0, 99999]"),
                line: None,
            }
            .into());
        }

        let mean_motion = data.mean_elements.mean_motion.as_ref().ok_or(
            ValidationError::MissingRequiredField {
                block: Cow::Borrowed("Mean Elements"),
                field: Cow::Borrowed("MEAN_MOTION"),
                line: None,
            },
        )?;

        let mean_motion_ddot =
            tle.mean_motion_ddot
                .as_ref()
                .ok_or(ValidationError::MissingRequiredField {
                    block: Cow::Borrowed("TLE Parameters"),
                    field: Cow::Borrowed("MEAN_MOTION_DDOT"),
                    line: None,
                })?;

        let bstar = tle
            .bstar
            .as_ref()
            .ok_or(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("TLE Parameters"),
                field: Cow::Borrowed("BSTAR"),
                line: None,
            })?;

        let (epoch_year_2, epoch_day_field) =
            format_tle_epoch_components(data.mean_elements.epoch.as_str())?;
        let mean_motion_dot_field = format_tle_dot_term(tle.mean_motion_dot.value)?;
        let mean_motion_ddot_field = format_tle_assumed_decimal(mean_motion_ddot.value)?;
        let bstar_field = format_tle_assumed_decimal(bstar.value)?;

        let inclination = data.mean_elements.inclination.angle.value;
        let raan = data.mean_elements.ra_of_asc_node.value;
        let arg_pericenter = data.mean_elements.arg_of_pericenter.value;
        let mean_anomaly = data.mean_elements.mean_anomaly.value;
        let eccentricity = data.mean_elements.eccentricity.value;

        validate_angle_for_tle("INCLINATION", inclination, false)?;
        validate_angle_for_tle("RA_OF_ASC_NODE", raan, true)?;
        validate_angle_for_tle("ARG_OF_PERICENTER", arg_pericenter, true)?;
        validate_angle_for_tle("MEAN_ANOMALY", mean_anomaly, true)?;
        if !(0.0..1.0).contains(&eccentricity) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("ECCENTRICITY"),
                value: eccentricity.to_string(),
                expected: Cow::Borrowed("[0.0, 1.0)"),
                line: None,
            }
            .into());
        }
        if !(0.0..100.0).contains(&mean_motion.value) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("MEAN_MOTION"),
                value: mean_motion.value.to_string(),
                expected: Cow::Borrowed("[0.0, 100.0)"),
                line: None,
            }
            .into());
        }

        let ecc_scaled = (eccentricity * 1.0e7).round();
        if !(0.0..10_000_000.0).contains(&ecc_scaled) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("ECCENTRICITY"),
                value: eccentricity.to_string(),
                expected: Cow::Borrowed("encodable as 7 TLE digits"),
                line: None,
            }
            .into());
        }

        let mut line1_no_checksum = format!(
            "1 {:05}{} {:02}{:03}{:<3} {:02}{} {} {} {} {} {:>4}",
            norad_cat_id,
            classification_char,
            launch.launch_year % 100,
            launch.launch_number,
            launch.launch_piece,
            epoch_year_2,
            epoch_day_field,
            mean_motion_dot_field,
            mean_motion_ddot_field,
            bstar_field,
            ephemeris_type,
            element_set_no.value
        );
        line1_no_checksum = normalize_tle_line_len(line1_no_checksum, "line 1")?;
        let checksum1 = tle_checksum(&line1_no_checksum);
        let line1 = format!("{}{}", line1_no_checksum, checksum1);

        let mut line2_no_checksum = format!(
            "2 {:05} {:8.4} {:8.4} {:07} {:8.4} {:8.4} {:11.8}{:5}",
            norad_cat_id,
            inclination,
            raan,
            ecc_scaled as u32,
            arg_pericenter,
            mean_anomaly,
            mean_motion.value,
            rev_at_epoch
        );
        line2_no_checksum = normalize_tle_line_len(line2_no_checksum, "line 2")?;
        let checksum2 = tle_checksum(&line2_no_checksum);
        let line2 = format!("{}{}", line2_no_checksum, checksum2);

        Ok((line1, line2))
    }

    /// Parse canonical NORAD TLE line 1/2 into a minimal OMM.
    pub fn from_tle_lines(line1: &str, line2: &str) -> Result<Self> {
        Self::from_tle_lines_with_options(line1, line2, &TleToOmmOptions::default())
    }

    /// Parse canonical NORAD TLE line 1/2 into a minimal OMM with metadata/header overrides.
    pub fn from_tle_lines_with_options(
        line1: &str,
        line2: &str,
        options: &TleToOmmOptions,
    ) -> Result<Self> {
        let line1 = normalize_tle_input_line(line1, "line 1")?;
        let line2 = normalize_tle_input_line(line2, "line 2")?;
        validate_tle_checksum(&line1, "line 1")?;
        validate_tle_checksum(&line2, "line 2")?;

        let l1 = line1.as_bytes();
        ensure_tle_line_structure(&line1, '1')?;
        ensure_tle_line_structure(&line2, '2')?;

        let norad_cat_id_l1 = parse_u32_strict(&line1[2..7], "NORAD_CAT_ID")?;
        let norad_cat_id_l2 = parse_u32_strict(&line2[2..7], "NORAD_CAT_ID")?;
        if norad_cat_id_l1 != norad_cat_id_l2 {
            return Err(ValidationError::Conflict {
                fields: vec![
                    Cow::Borrowed("NORAD_CAT_ID (line1)"),
                    Cow::Borrowed("NORAD_CAT_ID (line2)"),
                ],
                line: None,
            }
            .into());
        }

        let classification_type = (l1[7] as char).to_string();
        parse_classification_char(&classification_type)?;

        let launch_year_2 = parse_u32_strict(&line1[9..11], "LAUNCH_YEAR")?;
        let launch_number = parse_u32_strict(&line1[11..14], "LAUNCH_NUMBER")?;
        let launch_piece = line1[14..17].trim_end().to_string();
        if !launch_piece.is_empty()
            && (!launch_piece.chars().all(|c| c.is_ascii_alphanumeric()) || launch_piece.len() > 3)
        {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("LAUNCH_PIECE"),
                value: launch_piece,
                expected: Cow::Borrowed("1..=3 ASCII alphanumeric characters"),
                line: None,
            }
            .into());
        }

        let epoch_year_2 = parse_u32_strict(&line1[18..20], "EPOCH_YEAR")?;
        let epoch = parse_tle_epoch_field(epoch_year_2, &line1[20..32])?;

        let mean_motion_dot = parse_tle_dot_term(&line1[33..43], "MEAN_MOTION_DOT")?;
        let mean_motion_ddot = parse_tle_assumed_decimal(&line1[44..52], "MEAN_MOTION_DDOT")?;
        let bstar = parse_tle_assumed_decimal(&line1[53..61], "BSTAR")?;
        let ephemeris_type = parse_u32_trimmed(&line1[62..63], "EPHEMERIS_TYPE")?;
        if ephemeris_type > 9 {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPHEMERIS_TYPE"),
                value: ephemeris_type.to_string(),
                expected: Cow::Borrowed("[0, 9]"),
                line: None,
            }
            .into());
        }
        let element_set_no = parse_u32_trimmed(&line1[64..68], "ELEMENT_SET_NO")?;

        let inclination = parse_f64_trimmed(&line2[8..16], "INCLINATION")?;
        let ra_of_asc_node = parse_f64_trimmed(&line2[17..25], "RA_OF_ASC_NODE")?;
        let eccentricity_digits = parse_u32_strict(&line2[26..33], "ECCENTRICITY")?;
        let arg_of_pericenter = parse_f64_trimmed(&line2[34..42], "ARG_OF_PERICENTER")?;
        let mean_anomaly = parse_f64_trimmed(&line2[43..51], "MEAN_ANOMALY")?;
        let mean_motion = parse_f64_trimmed(&line2[52..63], "MEAN_MOTION")?;
        let rev_at_epoch = parse_u32_trimmed(&line2[63..68], "REV_AT_EPOCH")?;

        let derived_object_id =
            format_launch_designator_object_id(launch_year_2, launch_number, &line1[14..17]);

        let object_name = options
            .object_name
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let object_id = options.object_id.clone().unwrap_or(derived_object_id);
        let originator = options
            .originator
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let creation_date = options.creation_date.unwrap_or(epoch);

        Ok(Self {
            header: OdmHeader {
                comment: Vec::new(),
                classification: None,
                creation_date,
                originator,
                message_id: options.message_id.clone(),
            },
            body: OmmBody {
                segment: OmmSegment {
                    metadata: OmmMetadata {
                        comment: Vec::new(),
                        object_name,
                        object_id,
                        center_name: "EARTH".to_string(),
                        ref_frame: "TEME".to_string(),
                        ref_frame_epoch: None,
                        time_system: "UTC".to_string(),
                        mean_element_theory: "SGP4".to_string(),
                    },
                    data: OmmData {
                        comment: Vec::new(),
                        mean_elements: MeanElements {
                            comment: Vec::new(),
                            epoch,
                            semi_major_axis: None,
                            mean_motion: Some(MeanMotion::new(mean_motion, None)),
                            eccentricity: NonNegativeDouble::new(
                                eccentricity_digits as f64 / 1.0e7,
                            )?,
                            inclination: Inclination::new(inclination, None)?,
                            ra_of_asc_node: Angle::new(ra_of_asc_node, None)?,
                            arg_of_pericenter: Angle::new(arg_of_pericenter, None)?,
                            mean_anomaly: Angle::new(mean_anomaly, None)?,
                            gm: None,
                        },
                        spacecraft_parameters: None,
                        tle_parameters: Some(TleParameters {
                            comment: Vec::new(),
                            ephemeris_type: Some(ephemeris_type as i32),
                            classification_type: Some(classification_type),
                            norad_cat_id: Some(norad_cat_id_l1),
                            element_set_no: Some(ElementSetNo::new(element_set_no)?),
                            rev_at_epoch: Some(rev_at_epoch),
                            bstar: Some(BStar::new(bstar, None)),
                            bterm: None,
                            mean_motion_dot: MeanMotionDot::new(mean_motion_dot, None),
                            mean_motion_ddot: Some(MeanMotionDDot::new(mean_motion_ddot, None)),
                            agom: None,
                        }),
                        covariance_matrix: None,
                        user_defined_parameters: None,
                    },
                },
            },
            id: Some("CCSDS_OMM_VERS".to_string()),
            version: "2.0".to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LaunchDesignator {
    launch_year: u32,
    launch_number: u32,
    launch_piece: String,
}

const NS_PER_DAY: i128 = 86_400_000_000_000;

fn require_tle_field<T: Copy>(value: Option<T>, field: &'static str) -> Result<T> {
    value
        .ok_or(ValidationError::MissingRequiredField {
            block: Cow::Borrowed("TLE Parameters"),
            field: Cow::Borrowed(field),
            line: None,
        })
        .map_err(Into::into)
}

fn parse_classification_char(value: &str) -> Result<char> {
    let trimmed = value.trim();
    if trimmed.len() != 1 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("CLASSIFICATION_TYPE"),
            value: value.to_string(),
            expected: Cow::Borrowed("single ASCII letter"),
            line: None,
        }
        .into());
    }
    let c = trimmed.chars().next().expect("len checked");
    if !c.is_ascii_alphabetic() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("CLASSIFICATION_TYPE"),
            value: value.to_string(),
            expected: Cow::Borrowed("single ASCII letter"),
            line: None,
        }
        .into());
    }
    Ok(c.to_ascii_uppercase())
}

fn validate_angle_for_tle(name: &'static str, value: f64, allow_wrap: bool) -> Result<()> {
    let in_range = if allow_wrap {
        (0.0..360.0).contains(&value)
    } else {
        (0.0..=180.0).contains(&value)
    };
    if !in_range {
        let expected = if allow_wrap {
            "[0.0, 360.0)"
        } else {
            "[0.0, 180.0]"
        };
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed(name),
            value: value.to_string(),
            expected: Cow::Borrowed(expected),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn parse_object_id_launch_designator(object_id: &str) -> Result<LaunchDesignator> {
    let id = object_id.trim();
    let (year_str, rest) = id
        .split_once('-')
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNNPPP"),
            line: None,
        })?;

    if year_str.len() != 4 || !year_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNNPPP (4-digit year)"),
            line: None,
        }
        .into());
    }
    if rest.len() < 4 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNNPPP"),
            line: None,
        }
        .into());
    }

    let launch_number_str = &rest[..3];
    let launch_piece_str = &rest[3..];
    if !launch_number_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNNPPP (3-digit launch number)"),
            line: None,
        }
        .into());
    }
    if launch_piece_str.is_empty()
        || launch_piece_str.len() > 3
        || !launch_piece_str.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("OBJECT_ID"),
            value: id.to_string(),
            expected: Cow::Borrowed("YYYY-NNNPPP (piece is 1..=3 ASCII alphanumeric chars)"),
            line: None,
        }
        .into());
    }

    let launch_year = year_str.parse::<u32>()?;
    let launch_number = launch_number_str.parse::<u32>()?;
    if launch_number > 999 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("OBJECT_ID launch number"),
            value: launch_number.to_string(),
            expected: Cow::Borrowed("[0, 999]"),
            line: None,
        }
        .into());
    }

    Ok(LaunchDesignator {
        launch_year,
        launch_number,
        launch_piece: launch_piece_str.to_ascii_uppercase(),
    })
}

fn normalize_tle_line_len(line: String, label: &'static str) -> Result<String> {
    if line.len() != 68 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line,
            expected: Cow::Borrowed("line content must be exactly 68 chars before checksum"),
            line: None,
        }
        .into());
    }
    Ok(line)
}

fn normalize_tle_input_line(line: &str, label: &'static str) -> Result<String> {
    let trimmed = line.trim_end_matches(['\r', '\n']);
    if trimmed.len() != 69 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: trimmed.to_string(),
            expected: Cow::Borrowed("exactly 69 characters including checksum"),
            line: None,
        }
        .into());
    }
    Ok(trimmed.to_string())
}

fn tle_checksum(line_without_checksum: &str) -> u32 {
    line_without_checksum
        .as_bytes()
        .iter()
        .map(|b| match *b {
            b'0'..=b'9' => (b - b'0') as u32,
            b'-' => 1,
            _ => 0,
        })
        .sum::<u32>()
        % 10
}

fn validate_tle_checksum(line: &str, label: &'static str) -> Result<()> {
    let expected = line
        .as_bytes()
        .last()
        .and_then(|b| match *b {
            b'0'..=b'9' => Some((b - b'0') as u32),
            _ => None,
        })
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line.to_string(),
            expected: Cow::Borrowed("checksum digit in column 69"),
            line: None,
        })?;

    let actual = tle_checksum(&line[..68]);
    if actual != expected {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(label),
            value: line.to_string(),
            expected: Cow::Owned(format!("valid checksum {}", actual)),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn ensure_tle_line_structure(line: &str, line_number: char) -> Result<()> {
    let b = line.as_bytes();
    if b[0] as char != line_number {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE line number"),
            value: line.to_string(),
            expected: Cow::Owned(format!("{}", line_number)),
            line: None,
        }
        .into());
    }
    if b[1] != b' ' {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE spacing"),
            value: line.to_string(),
            expected: Cow::Borrowed("space in column 2"),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn parse_u32_strict(s: &str, field: &'static str) -> Result<u32> {
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("ASCII digits"),
            line: None,
        }
        .into());
    }
    Ok(s.parse::<u32>()?)
}

fn parse_u32_trimmed(s: &str, field: &'static str) -> Result<u32> {
    let t = s.trim();
    if t.is_empty() || !t.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("right-aligned unsigned integer"),
            line: None,
        }
        .into());
    }
    Ok(t.parse::<u32>()?)
}

fn parse_f64_trimmed(s: &str, field: &'static str) -> Result<f64> {
    let t = s.trim();
    if t.is_empty() {
        return Err(ValidationError::MissingRequiredField {
            block: Cow::Borrowed("TLE"),
            field: Cow::Borrowed(field),
            line: None,
        }
        .into());
    }
    t.parse::<f64>()
        .map_err(|_| ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: s.to_string(),
            expected: Cow::Borrowed("floating-point value"),
            line: None,
        })
        .map_err(Into::into)
}

fn format_tle_dot_term(value: f64) -> Result<String> {
    if !value.is_finite() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("MEAN_MOTION_DOT"),
            value: value.to_string(),
            expected: Cow::Borrowed("finite number"),
            line: None,
        }
        .into());
    }
    let sign = if value.is_sign_negative() { '-' } else { ' ' };
    let scaled = (value.abs() * 1.0e8).round();
    if !(0.0..100_000_000.0).contains(&scaled) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("MEAN_MOTION_DOT"),
            value: value.to_string(),
            expected: Cow::Borrowed("encodable as s.dddddddd"),
            line: None,
        }
        .into());
    }
    Ok(format!("{}.{}", sign, format!("{:08}", scaled as u64)))
}

fn parse_tle_dot_term(field: &str, name: &'static str) -> Result<f64> {
    let b = field.as_bytes();
    if b.len() != 10 || b[1] != b'.' {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("s.dddddddd"),
            line: None,
        }
        .into());
    }
    if !field[2..].chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("s.dddddddd"),
            line: None,
        }
        .into());
    }
    let sign = match b[0] as char {
        '-' => -1.0,
        ' ' | '+' => 1.0,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("leading sign/space"),
                line: None,
            }
            .into());
        }
    };
    let digits = parse_u32_strict(&field[2..], name)?;
    Ok(sign * (digits as f64) / 1.0e8)
}

fn format_tle_assumed_decimal(value: f64) -> Result<String> {
    if !value.is_finite() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("TLE assumed-decimal field"),
            value: value.to_string(),
            expected: Cow::Borrowed("finite number"),
            line: None,
        }
        .into());
    }

    if value == 0.0 {
        return Ok(" 00000-0".to_string());
    }

    let sign_char = if value.is_sign_negative() { '-' } else { ' ' };
    let abs = value.abs();
    let exponent = abs.log10().floor() as i32 + 1;
    let mantissa_raw = abs / 10f64.powi(exponent);
    let mut mantissa = (mantissa_raw * 1.0e5).round() as i32;
    let mut exp = exponent;

    if mantissa == 100_000 {
        mantissa = 10_000;
        exp += 1;
    }
    if !(10_000..=99_999).contains(&mantissa) || !(-9..=9).contains(&exp) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("TLE assumed-decimal field"),
            value: value.to_string(),
            expected: Cow::Borrowed("encodable as sXXXXXsY"),
            line: None,
        }
        .into());
    }

    let exp_sign = if exp < 0 { '-' } else { '+' };
    let exp_digit = exp.unsigned_abs();
    Ok(format!(
        "{}{:05}{}{}",
        sign_char, mantissa, exp_sign, exp_digit
    ))
}

fn parse_tle_assumed_decimal(field: &str, name: &'static str) -> Result<f64> {
    if field.len() != 8 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("sXXXXXsY"),
            line: None,
        }
        .into());
    }
    let b = field.as_bytes();
    let sign = match b[0] as char {
        '-' => -1.0,
        ' ' | '+' => 1.0,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("leading sign/space"),
                line: None,
            }
            .into());
        }
    };
    let mantissa_str = &field[1..6];
    if !mantissa_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed(name),
            value: field.to_string(),
            expected: Cow::Borrowed("5 mantissa digits"),
            line: None,
        }
        .into());
    }
    let exp_sign = match b[6] as char {
        '+' | ' ' => 1_i32,
        '-' => -1_i32,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("exponent sign"),
                line: None,
            }
            .into());
        }
    };
    let exp_digit = match b[7] {
        b'0'..=b'9' => (b[7] - b'0') as i32,
        _ => {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed(name),
                value: field.to_string(),
                expected: Cow::Borrowed("exponent digit"),
                line: None,
            }
            .into());
        }
    };
    let mantissa = mantissa_str.parse::<u32>()? as f64;
    let exponent = exp_sign * exp_digit;
    Ok(sign * (mantissa / 1.0e5) * 10f64.powi(exponent))
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: u32) -> Option<u32> {
    let d = match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => return None,
    };
    Some(d)
}

fn month_day_to_doy(year: i32, month: u32, day: u32) -> Option<u32> {
    let mut doy = 0_u32;
    for m in 1..month {
        doy += days_in_month(year, m)?;
    }
    if day == 0 || day > days_in_month(year, month)? {
        return None;
    }
    Some(doy + day)
}

fn doy_to_month_day(year: i32, mut doy: u32) -> Option<(u32, u32)> {
    if doy == 0 || doy > days_in_year(year) {
        return None;
    }
    for month in 1..=12 {
        let dim = days_in_month(year, month)?;
        if doy <= dim {
            return Some((month, doy));
        }
        doy -= dim;
    }
    None
}

fn parse_om_epoch_to_utc_ydns(epoch: &str) -> Result<(i32, u32, i128)> {
    let (date_part, time_part_full) =
        epoch
            .split_once('T')
            .ok_or_else(|| ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: epoch.to_string(),
                expected: Cow::Borrowed("YYYY-MM-DDThh:mm:ss(.fffffff)[Z|(+|-)hh:mm]"),
                line: None,
            })?;

    let (time_part, tz_offset_minutes) = parse_time_and_tz(time_part_full)?;
    let (mut year, mut doy) = parse_epoch_date_to_year_doy(date_part)?;
    let ns_of_day = parse_time_to_ns_of_day(time_part)?;

    let mut utc_ns_of_day = ns_of_day - (tz_offset_minutes as i128) * 60 * 1_000_000_000;
    while utc_ns_of_day < 0 {
        utc_ns_of_day += NS_PER_DAY;
        if doy == 1 {
            year -= 1;
            doy = days_in_year(year);
        } else {
            doy -= 1;
        }
    }
    while utc_ns_of_day >= NS_PER_DAY {
        utc_ns_of_day -= NS_PER_DAY;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    Ok((year, doy, utc_ns_of_day))
}

fn parse_time_and_tz(value: &str) -> Result<(&str, i32)> {
    if let Some(stripped) = value.strip_suffix('Z') {
        return Ok((stripped, 0));
    }

    if value.len() >= 6 {
        let sign_index = value.len() - 6;
        let tz = &value[sign_index..];
        let tzb = tz.as_bytes();
        if (tzb[0] == b'+' || tzb[0] == b'-') && tzb[3] == b':' {
            let hh = parse_u32_strict(&tz[1..3], "EPOCH timezone hour")?;
            let mm = parse_u32_strict(&tz[4..6], "EPOCH timezone minute")?;
            if hh > 23 || mm > 59 {
                return Err(ValidationError::OutOfRange {
                    name: Cow::Borrowed("EPOCH timezone"),
                    value: tz.to_string(),
                    expected: Cow::Borrowed("hh in [00,23], mm in [00,59]"),
                    line: None,
                }
                .into());
            }
            let sign = if tzb[0] == b'-' { -1 } else { 1 };
            let offset = sign * ((hh as i32) * 60 + (mm as i32));
            return Ok((&value[..sign_index], offset));
        }
    }

    Ok((value, 0))
}

fn parse_epoch_date_to_year_doy(date: &str) -> Result<(i32, u32)> {
    let bytes = date.as_bytes();
    if bytes.is_empty() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("non-empty date part"),
            line: None,
        }
        .into());
    }

    let mut idx = 0usize;
    if bytes[0] == b'+' || bytes[0] == b'-' {
        idx = 1;
    }
    let first_dash_rel = date[idx..]
        .find('-')
        .ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("date separator '-'"),
            line: None,
        })?;
    let first_dash = idx + first_dash_rel;
    let year = date[..first_dash]
        .parse::<i32>()
        .map_err(|_| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: date.to_string(),
            expected: Cow::Borrowed("valid year"),
            line: None,
        })?;
    let tail = &date[first_dash + 1..];

    if let Some((month_str, day_str)) = tail.split_once('-') {
        let month = parse_u32_strict(month_str, "EPOCH month")?;
        let day = parse_u32_strict(day_str, "EPOCH day")?;
        let doy =
            month_day_to_doy(year, month, day).ok_or_else(|| ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: date.to_string(),
                expected: Cow::Borrowed("valid calendar date"),
                line: None,
            })?;
        Ok((year, doy))
    } else {
        let doy = parse_u32_strict(tail, "EPOCH day-of-year")?;
        if doy == 0 || doy > days_in_year(year) {
            return Err(ValidationError::OutOfRange {
                name: Cow::Borrowed("EPOCH day-of-year"),
                value: doy.to_string(),
                expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
                line: None,
            }
            .into());
        }
        Ok((year, doy))
    }
}

fn parse_time_to_ns_of_day(time: &str) -> Result<i128> {
    let mut parts = time.split(':');
    let hh = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    let mm = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    let ss = parts.next().ok_or_else(|| ValidationError::InvalidValue {
        field: Cow::Borrowed("EPOCH"),
        value: time.to_string(),
        expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
        line: None,
    })?;
    if parts.next().is_some() {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: time.to_string(),
            expected: Cow::Borrowed("hh:mm:ss(.fraction)"),
            line: None,
        }
        .into());
    }

    let hour = parse_u32_strict(hh, "EPOCH hour")?;
    let minute = parse_u32_strict(mm, "EPOCH minute")?;
    if hour > 23 || minute > 59 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH hour/minute"),
            value: format!("{}:{}", hour, minute),
            expected: Cow::Borrowed("hour [0,23], minute [0,59]"),
            line: None,
        }
        .into());
    }

    let (sec_str, frac_str) = match ss.split_once('.') {
        Some((s, f)) => (s, Some(f)),
        None => (ss, None),
    };
    let mut second = parse_u32_strict(sec_str, "EPOCH second")?;
    if second > 60 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH second"),
            value: second.to_string(),
            expected: Cow::Borrowed("[0, 60]"),
            line: None,
        }
        .into());
    }

    let mut nanos = 0_u32;
    if let Some(frac) = frac_str {
        if !frac.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidValue {
                field: Cow::Borrowed("EPOCH"),
                value: time.to_string(),
                expected: Cow::Borrowed("fractional seconds must be digits"),
                line: None,
            }
            .into());
        }
        if frac.len() <= 9 {
            let mut padded = frac.to_string();
            while padded.len() < 9 {
                padded.push('0');
            }
            nanos = padded.parse::<u32>()?;
        } else {
            let head = &frac[..9];
            let mut rounded = head.parse::<u32>()?;
            let next_digit = frac.as_bytes()[9];
            if next_digit >= b'5' {
                rounded += 1;
            }
            if rounded == 1_000_000_000 {
                rounded = 0;
                second += 1;
            }
            nanos = rounded;
        }
    }

    if second == 60 && nanos > 0 {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: time.to_string(),
            expected: Cow::Borrowed("leap second must not carry fractional part"),
            line: None,
        }
        .into());
    }
    if second > 60 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH second"),
            value: second.to_string(),
            expected: Cow::Borrowed("[0, 60]"),
            line: None,
        }
        .into());
    }
    if second == 60 {
        second = 59;
        nanos = 999_999_999;
    }

    Ok((((hour * 3600 + minute * 60 + second) as i128) * 1_000_000_000) + nanos as i128)
}

fn format_tle_epoch_components(epoch: &str) -> Result<(u32, String)> {
    let (mut year, mut doy, utc_ns_of_day) = parse_om_epoch_to_utc_ydns(epoch)?;
    if year < 0 || year > 9999 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH year"),
            value: year.to_string(),
            expected: Cow::Borrowed("[0, 9999]"),
            line: None,
        }
        .into());
    }

    let mut frac_scaled =
        ((utc_ns_of_day as i128 * 100_000_000 + (NS_PER_DAY / 2)) / NS_PER_DAY) as u32;
    if frac_scaled == 100_000_000 {
        frac_scaled = 0;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    if doy == 0 || doy > days_in_year(year) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH day-of-year"),
            value: doy.to_string(),
            expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
            line: None,
        }
        .into());
    }

    Ok((
        (year as u32) % 100,
        format!("{:03}.{:08}", doy, frac_scaled),
    ))
}

fn parse_tle_epoch_year(two_digit_year: u32) -> i32 {
    if two_digit_year >= 57 {
        1900 + two_digit_year as i32
    } else {
        2000 + two_digit_year as i32
    }
}

fn parse_tle_epoch_field(epoch_year_2: u32, epoch_day_field: &str) -> Result<Epoch> {
    if epoch_day_field.len() != 12 || &epoch_day_field[3..4] != "." {
        return Err(ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: epoch_day_field.to_string(),
            expected: Cow::Borrowed("DDD.dddddddd"),
            line: None,
        }
        .into());
    }
    let mut year = parse_tle_epoch_year(epoch_year_2);
    let mut doy = parse_u32_strict(&epoch_day_field[..3], "EPOCH day-of-year")?;
    if doy == 0 || doy > days_in_year(year) {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH day-of-year"),
            value: doy.to_string(),
            expected: Cow::Owned(format!("[1, {}]", days_in_year(year))),
            line: None,
        }
        .into());
    }
    let frac = parse_u32_strict(&epoch_day_field[4..], "EPOCH fraction")?;
    if frac >= 100_000_000 {
        return Err(ValidationError::OutOfRange {
            name: Cow::Borrowed("EPOCH fraction"),
            value: frac.to_string(),
            expected: Cow::Borrowed("[0, 99999999]"),
            line: None,
        }
        .into());
    }

    let mut ns_of_day = ((frac as i128) * NS_PER_DAY + 50_000_000) / 100_000_000;
    if ns_of_day >= NS_PER_DAY {
        ns_of_day -= NS_PER_DAY;
        if doy == days_in_year(year) {
            year += 1;
            doy = 1;
        } else {
            doy += 1;
        }
    }

    let (month, day) =
        doy_to_month_day(year, doy).ok_or_else(|| ValidationError::InvalidValue {
            field: Cow::Borrowed("EPOCH"),
            value: epoch_day_field.to_string(),
            expected: Cow::Borrowed("valid day of year"),
            line: None,
        })?;

    let hour = (ns_of_day / 3_600_000_000_000) as u32;
    ns_of_day %= 3_600_000_000_000;
    let minute = (ns_of_day / 60_000_000_000) as u32;
    ns_of_day %= 60_000_000_000;
    let second = (ns_of_day / 1_000_000_000) as u32;
    let nanos = (ns_of_day % 1_000_000_000) as u32;

    let mut epoch = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    );
    if nanos > 0 {
        let mut frac_ns = format!("{:09}", nanos);
        while frac_ns.ends_with('0') {
            frac_ns.pop();
        }
        epoch.push('.');
        epoch.push_str(&frac_ns);
    }

    Epoch::new(&epoch).map_err(Into::into)
}

fn format_launch_designator_object_id(
    launch_year_2: u32,
    launch_number: u32,
    launch_piece_field: &str,
) -> String {
    let launch_year = parse_tle_epoch_year(launch_year_2);
    let piece = launch_piece_field.trim();
    if piece.is_empty() {
        format!("{:04}-{:03}", launch_year, launch_number)
    } else {
        format!("{:04}-{:03}{}", launch_year, launch_number, piece)
    }
}

impl ToKvn for Omm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_OMM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

//----------------------------------------------------------------------
// Body & Segment
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OmmBody {
    #[serde(rename = "segment")]
    pub segment: OmmSegment,
}

impl crate::traits::Validate for OmmBody {
    fn validate(&self) -> Result<()> {
        self.segment.validate()
    }
}

impl ToKvn for OmmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
pub struct OmmSegment {
    pub metadata: OmmMetadata,
    pub data: OmmData,
}

impl ToKvn for OmmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl crate::traits::Validate for OmmSegment {
    fn validate(&self) -> Result<()> {
        self.metadata.validate()?;
        self.data.validate_with_metadata(&self.metadata)
    }
}

//----------------------------------------------------------------------
// Metadata
//----------------------------------------------------------------------

/// Metadata for the OMM.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmMetadata {
    /// Comments (allowed at the beginning of the OMM Metadata). (See 7.8 for formatting rules.)
    ///
    /// **Examples**: This is a comment
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Spacecraft name for which mean element orbit state data is provided. While there is no
    /// CCSDS-based restriction on the value for this keyword, it is recommended to use names
    /// from the UN Office of Outer Space Affairs designator index (reference `[3]`, which include
    /// Object name and international designator of the participant). If OBJECT_NAME is not
    /// listed in reference `[3]` or the content is either unknown or cannot be disclosed, the
    /// value should be set to UNKNOWN.
    ///
    /// **Examples**: Telkom 2, Spaceway 2, INMARSAT 4-F2, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub object_name: String,
    /// Object identifier of the object for which mean element orbit state data is provided.
    /// While there is no CCSDS-based restriction on the value for this keyword, it is
    /// recommended to use the international spacecraft designator as published in the UN Office
    /// of Outer Space Affairs designator index (reference `[3]`). Recommended values have the
    /// format YYYY-NNNP{PP}, where: YYYY = Year of launch. NNN = Three-digit serial number of
    /// launch in year YYYY (with leading zeros). P{PP} = At least one capital letter for the
    /// identification of the part brought into space by the launch. If the asset is not listed
    /// in reference `[3]`, the UN Office of Outer Space Affairs designator index format is not
    /// used, or the content is either unknown or cannot be disclosed, the value should be set
    /// to UNKNOWN.
    ///
    /// **Examples**: 2005-046A, 2005-046B, 2003-022A, UNKNOWN
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub object_id: String,
    /// Origin of the OMM reference frame, which shall be a natural solar system body (planets,
    /// asteroids, comets, and natural satellites), including any planet barycenter or the solar
    /// system barycenter. Natural bodies shall be selected from the accepted set of values
    /// indicated in annex B, subsection B2.
    ///
    /// **Examples**: EARTH, MARS, MOON
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub center_name: String,
    /// Reference frame in which the Keplerian element data are given. Use of values other than
    /// those in 3.2.3.3 should be documented in an ICD. NOTE—NORAD Two Line Element Sets and
    /// corresponding Simplified General Perturbations (SGP) orbit propagator ephemeris outputs
    /// are explicitly defined to be in the True Equator Mean Equinox of Date (TEME of Date)
    /// reference frame. Therefore, TEME of date shall be used for OMMs based on NORAD Two Line
    /// Element sets, rather than the almost imperceptibly different TEME of Epoch (see
    /// reference `[H2]` or `[H3]` for further details).
    ///
    /// **Examples**: ICRF, ITRF2000, EME2000, TEME
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub ref_frame: String,
    /// Epoch of reference frame, if not intrinsic to the definition of the reference frame.
    /// (See 7.5.10 for formatting rules.)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ref_frame_epoch: Option<Epoch>,
    /// Time system used for Keplerian elements and covariance data. Use of values other than
    /// those in 3.2.3.2 should be documented in an ICD.
    ///
    /// **Examples**: UTC
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub time_system: String,
    /// Description of the Mean Element Theory. Indicates the proper method to employ to
    /// propagate the state.
    ///
    /// **Examples**: SGP, SGP4, SGP4-XP, DSST, USM
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.3.
    #[builder(into)]
    pub mean_element_theory: String,
}

impl crate::traits::Validate for OmmMetadata {
    fn validate(&self) -> Result<()> {
        if self.object_id.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "OBJECT_ID".into(),
                line: None,
            }
            .into());
        }
        if self.time_system.trim().is_empty() {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM Metadata".into(),
                field: "TIME_SYSTEM".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl ToKvn for OmmMetadata {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("OBJECT_NAME", &self.object_name);
        writer.write_pair("OBJECT_ID", &self.object_id);
        writer.write_pair("CENTER_NAME", &self.center_name);
        writer.write_pair("REF_FRAME", &self.ref_frame);
        if let Some(v) = &self.ref_frame_epoch {
            writer.write_pair("REF_FRAME_EPOCH", v);
        }
        writer.write_pair("TIME_SYSTEM", &self.time_system);
        writer.write_pair("MEAN_ELEMENT_THEORY", &self.mean_element_theory);
    }
}

//----------------------------------------------------------------------
// Data
//----------------------------------------------------------------------

/// OMM Data section.
///
/// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OmmData {
    /// Comments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Mean Keplerian Elements in the Specified Reference Frame.
    #[serde(rename = "meanElements")]
    pub mean_elements: MeanElements,
    /// Spacecraft Parameters.
    #[serde(
        rename = "spacecraftParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spacecraft_parameters: Option<SpacecraftParameters>,
    /// TLE Related Parameters (Only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
    #[serde(
        rename = "tleParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tle_parameters: Option<TleParameters>,
    /// Position/Velocity Covariance Matrix (6x6 Lower Triangular Form).
    #[serde(
        rename = "covarianceMatrix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub covariance_matrix: Option<OpmCovarianceMatrix>,
    /// User-Defined Parameters.
    #[serde(
        rename = "userDefinedParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_parameters: Option<UserDefined>,
}

impl ToKvn for OmmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        // Mean Elements
        self.mean_elements.write_kvn(writer);

        // Spacecraft Params
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.mass {
                writer.write_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_measure("SOLAR_RAD_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_pair("SOLAR_RAD_COEFF", v);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_measure("DRAG_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_pair("DRAG_COEFF", v);
            }
        }

        // TLE Params
        if let Some(tle) = &self.tle_parameters {
            tle.write_kvn(writer);
        }

        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }

        // User Defined
        if let Some(ud) = &self.user_defined_parameters {
            writer.write_comments(&ud.comment);
            for p in &ud.user_defined {
                writer.write_user_defined(&p.parameter, &p.value);
            }
        }
    }
}

impl crate::traits::Validate for OmmData {
    fn validate(&self) -> Result<()> {
        self.mean_elements.validate()
    }
}

impl OmmData {
    pub fn validate_with_metadata(&self, metadata: &OmmMetadata) -> Result<()> {
        let theory = metadata.mean_element_theory.as_str();

        self.validate()?;

        // 1. Validate TLE Parameters presence based on theory
        match theory {
            "SGP" | "SGP4" | "PPT3" | "SGP4-XP" => {
                let tle =
                    self.tle_parameters
                        .as_ref()
                        .ok_or(ValidationError::MissingRequiredField {
                            block: Cow::Borrowed("OMM Data"),
                            field: Cow::Borrowed("TLE_PARAMETERS"),
                            line: None,
                        })?;
                tle.validate(theory)?;
            }
            _ => {
                // Not strictly required for other theories
            }
        }

        // 2. Validate Mean Motion vs Semi Major Axis
        // If SGP/SGP4, MEAN_MOTION is preferred/required.
        if matches!(theory, "SGP" | "SGP4") && self.mean_elements.mean_motion.is_none() {
            return Err(ValidationError::MissingRequiredField {
                block: Cow::Borrowed("Mean Elements"),
                field: Cow::Borrowed("MEAN_MOTION"),
                line: None,
            }
            .into());
        }

        Ok(())
    }
}

//----------------------------------------------------------------------
// Mean Elements
//----------------------------------------------------------------------

/// Mean Keplerian Elements in the Specified Reference Frame.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MeanElements {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Epoch of Mean Keplerian elements (see 7.5.10 for formatting rules)
    ///
    /// **Examples**: 2001-11-06T11:17:33, 2002-204T15:56:23Z
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub epoch: Epoch,
    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// **Examples**: 28594.4
    ///
    /// **Units**: km or rev/day
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semi_major_axis: Option<Distance>,
    /// Semi-major axis in kilometers (preferred), or, if MEAN_ELEMENT_THEORY = SGP/SGP4, the
    /// Keplerian Mean motion in revolutions per day
    ///
    /// **Examples**: 1.491325
    ///
    /// **Units**: km or rev/day
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_motion: Option<MeanMotion>,
    /// Eccentricity
    ///
    /// **Examples**: 0.7303
    ///
    /// **Units**: n/a
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub eccentricity: NonNegativeDouble,
    /// Inclination
    ///
    /// **Examples**: 63.4
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub inclination: Inclination,
    /// Right ascension of ascending node
    ///
    /// **Examples**: 345.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub ra_of_asc_node: Angle,
    /// Argument of pericenter
    ///
    /// **Examples**: 270.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub arg_of_pericenter: Angle,
    /// Mean anomaly
    ///
    /// **Examples**: 130.0
    ///
    /// **Units**: deg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub mean_anomaly: Angle,
    /// Gravitational Coefficient (Gravitational Constant × Central Mass)
    ///
    /// **Examples**: 398600.44
    ///
    /// **Units**: km³/s²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub gm: Option<Gm>,
}

impl crate::traits::Validate for MeanElements {
    fn validate(&self) -> Result<()> {
        match (self.semi_major_axis.is_some(), self.mean_motion.is_some()) {
            (true, false) | (false, true) => Ok(()),
            _ => Err(ValidationError::Generic {
                message: Cow::Borrowed(
                    "Mean Elements must have exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION",
                ),
                line: None,
            }
            .into()),
        }
    }
}

impl ToKvn for MeanElements {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("EPOCH", self.epoch);
        if let Some(v) = &self.semi_major_axis {
            writer.write_measure("SEMI_MAJOR_AXIS", v);
        }
        if let Some(v) = &self.mean_motion {
            writer.write_measure("MEAN_MOTION", v);
        }
        writer.write_pair("ECCENTRICITY", self.eccentricity);
        writer.write_measure("INCLINATION", &self.inclination.to_unit_value());
        writer.write_measure("RA_OF_ASC_NODE", &self.ra_of_asc_node.to_unit_value());
        writer.write_measure("ARG_OF_PERICENTER", &self.arg_of_pericenter.to_unit_value());
        writer.write_measure("MEAN_ANOMALY", &self.mean_anomaly.to_unit_value());
        if let Some(v) = &self.gm {
            writer.write_measure("GM", &UnitValue::new(v.value, v.units.clone()));
        }
    }
}

//----------------------------------------------------------------------
// Spacecraft Parameters
//----------------------------------------------------------------------

//----------------------------------------------------------------------
// TLE Parameters
//----------------------------------------------------------------------

/// TLE Related Parameters (This section is only required if MEAN_ELEMENT_THEORY=SGP/SGP4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TleParameters {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comment: Vec<String>,
    /// Ephemeris type. Default value = 0. (See 4.2.4.7.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub ephemeris_type: Option<i32>,
    /// Classification type. Default value = U. (See 4.2.4.7.)
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    #[builder(into)]
    pub classification_type: Option<String>,
    /// NORAD Catalog Number (‘Satellite Number’) an integer of up to nine digits. This keyword
    /// is only required if MEAN_ELEMENT_THEORY=SGP/SGP4.
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub norad_cat_id: Option<u32>,
    /// Element set number for this satellite. Normally incremented sequentially but may be out
    /// of sync if it is generated from a backup source. Used to distinguish different TLEs,
    /// and therefore only meaningful if TLE-based data is being exchanged (i.e.,
    /// MEAN_ELEMENT_THEORY = SGP/SGP4).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub element_set_no: Option<ElementSetNo>,
    /// Revolution Number
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub rev_at_epoch: Option<u32>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4 (BSTAR = drag parameter for SGP4).
    ///
    /// **Units**: 1/[Earth radii]
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub bstar: Option<BStar>,
    /// Drag-like ballistic coefficient, required for SGP4 and SGP4-XP mean element models:
    /// MEAN_ELEMENT_THEORY= SGP4-XP (BTERM ballistic coefficient CDA/m, where CD = drag
    /// coefficient, A = average cross-sectional area, m = mass. Example values for BTERM =
    /// 0.02 (rocket body), 0.0015 (payload); average value spanning 20,000 catalog objects =
    /// 0.0286.
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub bterm: Option<M2kg>,
    /// First Time Derivative of the Mean Motion (i.e., a drag term, required when
    /// MEAN_ELEMENT_THEORY = SGP or PPT3). (See 4.2.4.7 for important details).
    ///
    /// **Units**: rev/day²
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    pub mean_motion_dot: MeanMotionDot,
    /// Second Time Derivative of Mean Motion (i.e., a drag term). (See 4.2.4.7 for important
    /// details). Required when MEAN_ELEMENT_THEORY= SGP or PPT3.
    ///
    /// **Units**: rev/day³
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub mean_motion_ddot: Option<MeanMotionDDot>,
    /// Solar radiation pressure coefficient AY/m, where y = reflectivity, A = average
    /// cross-sectional area, m = mass. Example values AGOM = 0.01 (rocket body) and 0.001
    /// (payload); average value spanning 20,000 catalog objects = 0.0143 m2/kg. Required
    /// when MEAN_ELEMENT_THEORY= SGP4-XP.
    ///
    /// **Units**: m²/kg
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 4.2.4.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
    pub agom: Option<M2kg>,
}

impl ToKvn for TleParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        if let Some(v) = self.ephemeris_type {
            writer.write_pair("EPHEMERIS_TYPE", v);
        }
        if let Some(v) = &self.classification_type {
            writer.write_pair("CLASSIFICATION_TYPE", v);
        }
        if let Some(v) = self.norad_cat_id {
            writer.write_pair("NORAD_CAT_ID", v);
        }
        if let Some(v) = self.element_set_no {
            writer.write_pair("ELEMENT_SET_NO", v);
        }
        if let Some(v) = self.rev_at_epoch {
            writer.write_pair("REV_AT_EPOCH", v);
        }
        if let Some(v) = &self.bstar {
            writer.write_measure("BSTAR", v);
        }
        if let Some(v) = &self.bterm {
            writer.write_measure("BTERM", v);
        }
        writer.write_measure("MEAN_MOTION_DOT", &self.mean_motion_dot);
        if let Some(v) = &self.mean_motion_ddot {
            writer.write_measure("MEAN_MOTION_DDOT", v);
        }
        if let Some(v) = &self.agom {
            writer.write_measure("AGOM", v);
        }
    }
}

impl TleParameters {
    pub fn validate(&self, theory: &str) -> Result<()> {
        match theory {
            "SGP" | "PPT3" => {
                if self.mean_motion_ddot.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("MEAN_MOTION_DDOT"),
                        line: None,
                    }
                    .into());
                }
            }
            "SGP4" => {
                if self.bstar.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("BSTAR"),
                        line: None,
                    }
                    .into());
                }
            }
            "SGP4-XP" => {
                if self.bterm.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("BTERM"),
                        line: None,
                    }
                    .into());
                }
                if self.agom.is_none() {
                    return Err(ValidationError::MissingRequiredField {
                        block: Cow::Borrowed("TLE Parameters"),
                        field: Cow::Borrowed("AGOM"),
                        line: None,
                    }
                    .into());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // OMM Roundtrip Tests (Kitchen Sink)
    // =========================================================================

    #[test]
    fn full_optional_fields_roundtrip() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
COMMENT Header Comment 1
COMMENT Header Comment 2
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = SATELLITE
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
GM = 398600.4418 [km**3/s**2]
MASS = 1500.0 [kg]
SOLAR_RAD_AREA = 20.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 15.0 [m**2]
DRAG_COEFF = 2.2
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 99999
ELEMENT_SET_NO = 123
REV_AT_EPOCH = 500
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
COV_REF_FRAME = TEME
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.1
CZ_Y = 0.1
CZ_Z = 1.0
CX_DOT_X = 0.1
CX_DOT_Y = 0.1
CX_DOT_Z = 0.1
CX_DOT_X_DOT = 1.0
CY_DOT_X = 0.1
CY_DOT_Y = 0.1
CY_DOT_Z = 0.1
CY_DOT_X_DOT = 0.1
CY_DOT_Y_DOT = 1.0
CZ_DOT_X = 0.1
CZ_DOT_Y = 0.1
CZ_DOT_Z = 0.1
CZ_DOT_X_DOT = 0.1
CZ_DOT_Y_DOT = 0.1
CZ_DOT_Z_DOT = 1.0
USER_DEFINED_FOO = BAR
USER_DEFINED_BAZ = QUX
"#;
        let omm = Omm::from_kvn(kvn).expect("Failed to parse kitchen sink OMM");

        // Verify some fields
        assert_eq!(omm.header.message_id, Some("MSG-001".to_string()));
        assert_eq!(omm.header.comment.len(), 2);

        let me = &omm.body.segment.data.mean_elements;
        assert_eq!(me.gm.as_ref().unwrap().value, 398600.4418);

        let sp = omm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .unwrap();
        assert_eq!(sp.mass.as_ref().unwrap().value, 1500.0);

        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(99999));

        let binding = omm
            .body
            .segment
            .data
            .user_defined_parameters
            .as_ref()
            .unwrap();
        let ud = &binding.user_defined;
        assert_eq!(ud.len(), 2);
        assert_eq!(ud[0].parameter, "FOO");
        assert_eq!(ud[0].value, "BAR");

        // Roundtrip
        let kvn_out = omm.to_kvn().expect("Failed to serialize OMM");
        let omm2 = Omm::from_kvn(&kvn_out).expect("Failed to re-parse OMM");

        assert_eq!(omm, omm2);
    }

    #[test]
    fn test_parse_xml_omm_g10() {
        let xml = include_str!("../../../data/xml/omm_g10.xml");
        let omm = Omm::from_xml(xml).expect("Failed to parse omm_g10.xml");

        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES-9");
        assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");

        let me = &omm.body.segment.data.mean_elements;
        assert!(me.mean_motion.is_some());

        // Has covariance
        assert!(omm.body.segment.data.covariance_matrix.is_some());

        // Has TLE parameters
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(23581));
    }

    #[test]
    fn test_roundtrip_kvn_minimal() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm1 = Omm::from_kvn(kvn).expect("First parse failed");
        let kvn2 = omm1.to_kvn().expect("Serialization failed");
        let omm2 = Omm::from_kvn(&kvn2).expect("Second parse failed");

        assert_eq!(omm1.version, omm2.version);
        assert_eq!(omm1.header.originator, omm2.header.originator);
        assert_eq!(
            omm1.body.segment.metadata.object_name,
            omm2.body.segment.metadata.object_name
        );
        assert_eq!(
            omm1.body.segment.data.mean_elements.eccentricity,
            omm2.body.segment.data.mean_elements.eccentricity
        );
    }

    #[test]
    fn test_omm_validation_missing_mandatory_metadata() {
        // Missing OBJECT_NAME
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  EPHEMERIS_TYPE = 0
  CLASSIFICATION_TYPE = U
  NORAD_CAT_ID = 99999
  ELEMENT_SET_NO = 123
  REV_AT_EPOCH = 500
  BSTAR = 0.0001 [1/ER]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        // OBJECT_NAME is mandatory in the struct builder
        // The parser usually fails if a required field is missing for the builder
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_reqs() {
        // Case 1: SGP4 theory but missing TLE Parameters block
        let kvn_no_tle = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        let res = Omm::from_kvn(kvn_no_tle);
        assert!(res.is_err());
        // Check for specific error if possible, but strict error checking might be brittle
        // Expecting ValidationError::MissingRequiredField for TLE_PARAMETERS

        // Case 2: SGP4 theory but using SEMI_MAJOR_AXIS instead of MEAN_MOTION
        let kvn_sma = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  BSTAR = 0.0001 [1/ER]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
"#;
        // Validation logic should flag missing MEAN_MOTION for SGP4
        assert!(Omm::from_kvn(kvn_sma).is_err());

        // Case 3: SGP4 theory but missing BSTAR in TLE parameters
        let kvn_no_bstar = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
"#;
        assert!(Omm::from_kvn(kvn_no_bstar).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_xp_reqs() {
        // SGP4-XP requires AGOM and BTERM
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  BTERM = 0.01 [m**2/kg]
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  # Missing AGOM
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_mean_elements_choice() {
        // Missing both SMA and Mean Motion
        let kvn_none = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn_none).is_err());

        // Both SMA and Mean Motion present
        let kvn_both = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn_both).is_err());
    }

    #[test]
    fn test_omm_validation_negative_values() {
        // Negative Eccentricity
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = -0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_units_parsing() {
        use std::str::FromStr;
        assert!(InvErUnits::from_str("1/ER").is_ok());
        assert!(InvErUnits::from_str("INVALID").is_err());

        assert!(RevPerDayUnits::from_str("rev/day").is_ok());
        assert!(RevPerDayUnits::from_str("REV/DAY").is_ok());
        assert!(RevPerDayUnits::from_str("INVALID").is_err());

        assert!(RevPerDay2Units::from_str("rev/day**2").is_ok());
        assert!(RevPerDay2Units::from_str("REV/DAY**2").is_ok());
        assert!(RevPerDay2Units::from_str("INVALID").is_err());

        assert!(RevPerDay3Units::from_str("rev/day**3").is_ok());
        assert!(RevPerDay3Units::from_str("REV/DAY**3").is_ok());
        assert!(RevPerDay3Units::from_str("INVALID").is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp_ppt3_reqs() {
        // SGP/PPT3 requires MEAN_MOTION_DDOT
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.0 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 10.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 10.0 [deg]
MEAN_ANOMALY = 10.0 [deg]
TLE_PARAMETERS =
  MEAN_MOTION_DOT = 0.0 [rev/day**2]
  # Missing MEAN_MOTION_DDOT
"#;
        assert!(Omm::from_kvn(kvn).is_err());
    }

    #[test]
    fn test_omm_validation_theory_sgp4_xp_additional_reqs() {
        // SGP4-XP requires BTERM and AGOM
        let data = OmmData::builder()
            .mean_elements(
                MeanElements::builder()
                    .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .mean_motion(MeanMotion::new(15.0, None))
                    .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                    .inclination(Inclination::new(10.0, None).unwrap())
                    .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                    .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                    .mean_anomaly(Angle::new(10.0, None).unwrap())
                    .build(),
            )
            .tle_parameters(
                TleParameters::builder()
                    .mean_motion_dot(MeanMotionDot::new(0.0, None))
                    .build(),
            )
            .build();

        let mut segment = OmmSegment::builder()
            .metadata(
                OmmMetadata::builder()
                    .object_name("SAT")
                    .object_id("1")
                    .center_name("EARTH")
                    .ref_frame("TEME")
                    .time_system("UTC")
                    .mean_element_theory("SGP4-XP")
                    .build(),
            )
            .data(data.clone())
            .build();

        // Missing BTERM
        assert!(segment.validate().is_err());

        segment.data.tle_parameters.as_mut().unwrap().bterm = Some(M2kg::new(0.01, None));
        // Missing AGOM
        assert!(segment.validate().is_err());

        segment.data.tle_parameters.as_mut().unwrap().agom = Some(M2kg::new(1.0, None));
        assert!(segment.validate().is_ok());
    }

    #[test]
    fn test_omm_serialization_gaps() {
        let mut tle = TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build();
        tle.bterm = Some(M2kg::new(0.01, None));
        tle.agom = Some(M2kg::new(1.0, None));

        let omm = Omm::builder()
            .version("3.0")
            .header(
                OdmHeader::builder()
                    .creation_date(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .originator("ME")
                    .build(),
            )
            .body(
                OmmBody::builder()
                    .segment(
                        OmmSegment::builder()
                            .metadata(
                                OmmMetadata::builder()
                                    .object_name("SAT")
                                    .object_id("1")
                                    .center_name("EARTH")
                                    .ref_frame("TEME")
                                    .time_system("UTC")
                                    .mean_element_theory("SGP4-XP")
                                    .build(),
                            )
                            .data(
                                OmmData::builder()
                                    .mean_elements(
                                        MeanElements::builder()
                                            .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                                            .mean_motion(MeanMotion::new(15.0, None))
                                            .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                                            .inclination(Inclination::new(10.0, None).unwrap())
                                            .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                                            .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                                            .mean_anomaly(Angle::new(10.0, None).unwrap())
                                            .build(),
                                    )
                                    .tle_parameters(tle)
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();

        let kvn = omm.to_kvn().unwrap();
        assert!(kvn.contains("BTERM"));
        assert!(kvn.contains("0.01"));
        assert!(kvn.contains("AGOM"));
        assert!(kvn.contains("1"));
    }

    #[test]
    fn test_omm_validation_theory_gaps() {
        // SGP missing MEAN_MOTION
        let meta = OmmMetadata::builder()
            .object_name("SAT")
            .object_id("1")
            .center_name("EARTH")
            .ref_frame("TEME")
            .time_system("UTC")
            .mean_element_theory("SGP")
            .build();
        let mut data = OmmData::builder()
            .mean_elements(
                MeanElements::builder()
                    .epoch(Epoch::new("2023-01-01T00:00:00").unwrap())
                    .semi_major_axis(Distance::new(7000.0, None))
                    .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                    .inclination(Inclination::new(10.0, None).unwrap())
                    .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                    .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                    .mean_anomaly(Angle::new(10.0, None).unwrap())
                    .build(),
            )
            .tle_parameters(
                TleParameters::builder()
                    .mean_motion_dot(MeanMotionDot::new(0.0, None))
                    .build(),
            )
            .build();

        let segment = OmmSegment::builder()
            .metadata(meta)
            .data(data.clone())
            .build();
        assert!(segment.validate().is_err()); // Missing MEAN_MOTION for SGP

        // TleParameters SGP/PPT3 missing mean_motion_ddot
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP")
            .is_err());
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("PPT3")
            .is_err());

        // TleParameters SGP4 missing bstar
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP4")
            .is_err());

        // TleParameters SGP4-XP missing bterm/agom
        assert!(data
            .tle_parameters
            .as_ref()
            .unwrap()
            .validate("SGP4-XP")
            .is_err());
        let mut tle_xp = data.tle_parameters.clone().unwrap();
        tle_xp.bterm = Some(M2kg::new(0.01, None));
        assert!(tle_xp.validate("SGP4-XP").is_err()); // Missing AGOM

        // MeanElements both SMA and MeanMotion
        data.mean_elements.mean_motion = Some(MeanMotion::new(15.0, None));
        assert!(data.mean_elements.validate().is_err());

        // MeanElements neither SMA nor MeanMotion
        data.mean_elements.semi_major_axis = None;
        data.mean_elements.mean_motion = None;
        assert!(data.mean_elements.validate().is_err());

        // Unknown theory in TleParameters
        let tle = TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build();
        assert!(tle.validate("UNKNOWN").is_ok());
    }

    #[test]
    fn test_to_tle_lines_iss_example() {
        let kvn = r#"CCSDS_OMM_VERS = 2.0
CREATION_DATE = 2020-12-13T17:26:09
ORIGINATOR = 18 SPCS
OBJECT_NAME = ISS (ZARYA)
OBJECT_ID = 1998-067A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2020-12-13T16:36:04.502592
MEAN_MOTION = 15.49181153 [rev/day]
ECCENTRICITY = 0.00017790
INCLINATION = 51.6444 [deg]
RA_OF_ASC_NODE = 180.2777 [deg]
ARG_OF_PERICENTER = 128.5985 [deg]
MEAN_ANOMALY = 350.1361 [deg]
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 25544
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 25984
BSTAR = 0.00002412400000 [1/ER]
MEAN_MOTION_DOT = 0.00000888 [rev/day**2]
MEAN_MOTION_DDOT = 0.0000000000000 [rev/day**3]
"#;

        let omm = Omm::from_kvn(kvn).expect("failed to parse ISS OMM sample");
        let (line1, line2) = omm.to_tle_lines().expect("failed to generate TLE lines");
        assert_eq!(
            line1,
            "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
        );
        assert_eq!(
            line2,
            "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"
        );
    }

    #[test]
    fn test_from_tle_lines_iss_example() {
        let line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995";
        let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";

        let omm = Omm::from_tle_lines(line1, line2).expect("failed to parse TLE lines");
        assert_eq!(omm.body.segment.metadata.object_id, "1998-067A");
        assert_eq!(omm.body.segment.metadata.object_name, "UNKNOWN");
        assert_eq!(omm.body.segment.metadata.center_name, "EARTH");
        assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");
        assert_eq!(omm.body.segment.metadata.time_system, "UTC");
        assert_eq!(omm.body.segment.metadata.mean_element_theory, "SGP4");

        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert_eq!(tle.norad_cat_id, Some(25544));
        assert_eq!(tle.classification_type.as_deref(), Some("U"));
        assert_eq!(tle.ephemeris_type, Some(0));
        assert_eq!(tle.element_set_no.as_ref().map(|v| v.value), Some(999));
        assert_eq!(tle.rev_at_epoch, Some(25984));
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.bstar.is_some());
        omm.validate().expect("generated OMM should validate");
    }

    #[test]
    fn test_tle_roundtrip_with_options() {
        let line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995";
        let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";
        let options = TleToOmmOptions {
            object_name: Some("ISS (ZARYA)".to_string()),
            object_id: None,
            originator: Some("18 SPCS".to_string()),
            message_id: None,
            creation_date: None,
        };

        let omm = Omm::from_tle_lines_with_options(line1, line2, &options)
            .expect("failed to parse TLE lines with options");
        let (line1_out, line2_out) = omm.to_tle_lines().expect("failed to regenerate TLE");
        assert_eq!(line1_out, line1);
        assert_eq!(line2_out, line2);
    }

    #[test]
    fn test_to_tle_lines_requires_parseable_object_id() {
        let kvn = r#"CCSDS_OMM_VERS = 2.0
CREATION_DATE = 2020-12-13T17:26:09
ORIGINATOR = 18 SPCS
OBJECT_NAME = ISS (ZARYA)
OBJECT_ID = UNKNOWN
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2020-12-13T16:36:04.502592
MEAN_MOTION = 15.49181153 [rev/day]
ECCENTRICITY = 0.00017790
INCLINATION = 51.6444 [deg]
RA_OF_ASC_NODE = 180.2777 [deg]
ARG_OF_PERICENTER = 128.5985 [deg]
MEAN_ANOMALY = 350.1361 [deg]
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 25544
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 25984
BSTAR = 0.00002412400000 [1/ER]
MEAN_MOTION_DOT = 0.00000888 [rev/day**2]
MEAN_MOTION_DDOT = 0.0000000000000 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("failed to parse OMM");
        assert!(omm.to_tle_lines().is_err());
    }

    #[test]
    fn test_rev_per_day_units_display_all() {
        assert_eq!(format!("{}", RevPerDayUnits::RevPerDayUpper), "REV/DAY");
        assert_eq!(
            format!("{}", RevPerDay2Units::RevPerDay2Upper),
            "REV/DAY**2"
        );
        assert_eq!(
            format!("{}", RevPerDay3Units::RevPerDay3Upper),
            "REV/DAY**3"
        );
    }
}
