// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OPM (Orbit Parameter Message).
//!
//! This module implements KVN parsing for OPM using winnow parser combinators.
//! The parsing follows the CCSDS 502.0-B-3 specification structure:
//!
//! ```text
//! OPM
//! ├── Version (CCSDS_OPM_VERS)
//! ├── Header (OdmHeader)
//! │   ├── COMMENT* (optional, multiple)
//! │   ├── CLASSIFICATION (optional)
//! │   ├── CREATION_DATE (required)
//! │   ├── ORIGINATOR (required)
//! │   └── MESSAGE_ID (optional)
//! └── Body (OpmBody)
//!     └── Segment (OpmSegment)
//!         ├── Metadata (OpmMetadata)
//!         │   ├── COMMENT* (optional)
//!         │   ├── OBJECT_NAME (required)
//!         │   ├── OBJECT_ID (required)
//!         │   ├── CENTER_NAME (required)
//!         │   ├── REF_FRAME (required)
//!         │   ├── REF_FRAME_EPOCH (optional)
//!         │   └── TIME_SYSTEM (required)
//!         └── Data (OpmData)
//!             ├── COMMENT* (optional)
//!             ├── StateVector (required)
//!             ├── KeplerianElements (optional)
//!             ├── SpacecraftParameters (optional)
//!             ├── CovarianceMatrix (optional)
//!             ├── ManeuverParameters* (optional, multiple)
//!             └── UserDefinedParameters (optional)
//! ```

use crate::common::SpacecraftParameters;
use crate::kvn::parser::*;
use crate::messages::opm::{
    KeplerianElements, ManeuverParameters, Opm, OpmBody, OpmData, OpmMetadata, OpmSegment,
};
use crate::types::*;
use std::str::FromStr;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helper: Check if key belongs to OPM Data section
//----------------------------------------------------------------------

fn is_opm_data_key(key: &str) -> bool {
    matches!(
        key,
        "EPOCH"
            | "X"
            | "Y"
            | "Z"
            | "X_DOT"
            | "Y_DOT"
            | "Z_DOT"
            | "SEMI_MAJOR_AXIS"
            | "ECCENTRICITY"
            | "INCLINATION"
            | "RA_OF_ASC_NODE"
            | "ARG_OF_PERICENTER"
            | "TRUE_ANOMALY"
            | "MEAN_ANOMALY"
            | "GM"
            | "MASS"
            | "SOLAR_RAD_AREA"
            | "SOLAR_RAD_COEFF"
            | "DRAG_AREA"
            | "DRAG_COEFF"
            | "COV_REF_FRAME"
            | "CX_X"
            | "CY_X"
            | "CY_Y"
            | "CZ_X"
            | "CZ_Y"
            | "CZ_Z"
            | "CX_DOT_X"
            | "CX_DOT_Y"
            | "CX_DOT_Z"
            | "CX_DOT_X_DOT"
            | "CY_DOT_X"
            | "CY_DOT_Y"
            | "CY_DOT_Z"
            | "CY_DOT_X_DOT"
            | "CY_DOT_Y_DOT"
            | "CZ_DOT_X"
            | "CZ_DOT_Y"
            | "CZ_DOT_Z"
            | "CZ_DOT_X_DOT"
            | "CZ_DOT_Y_DOT"
            | "CZ_DOT_Z_DOT"
            | "MAN_EPOCH_IGNITION"
            | "MAN_DURATION"
            | "MAN_DELTA_MASS"
            | "MAN_REF_FRAME"
            | "MAN_DV_1"
            | "MAN_DV_2"
            | "MAN_DV_3"
    ) || key.starts_with("USER_DEFINED_")
}

//----------------------------------------------------------------------
// OPM Version Parser
//----------------------------------------------------------------------

/// Parses the OPM version line: `CCSDS_OPM_VERS = 3.0`
pub fn opm_version(input: &mut &str) -> ModalResult<String> {
    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_OPM_VERS").parse_next(input)?;
    if value != "3.0" && value != "2.0" {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("3.0 or 2.0")),
        )));
    }
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OPM Metadata Parser
//----------------------------------------------------------------------

/// Parses the OPM metadata section.
pub fn opm_metadata(input: &mut &str) -> ModalResult<OpmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;

    loop {
        // Collect any comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        // Check what's next
        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_opm_data_key(key) => {
                // We've reached the data section
                break;
            }
            Some(_key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "OBJECT_NAME" => object_name = Some(v.to_string()),
                    "OBJECT_ID" => object_id = Some(v.to_string()),
                    "CENTER_NAME" => center_name = Some(v.to_string()),
                    "REF_FRAME" => ref_frame = Some(v.to_string()),
                    "REF_FRAME_EPOCH" => {
                        ref_frame_epoch =
                            Some(Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid Epoch"))?);
                    }
                    "TIME_SYSTEM" => time_system = Some(v.to_string()),
                    _ => {
                        return Err(cut_err(input, "Unexpected key or invalid format"));
                    }
                }
            }
            None => break,
        }
    }

    Ok(OpmMetadata {
        comment,
        object_name: object_name.ok_or_else(|| cut_err(input, "Missing required value"))?,
        object_id: object_id.ok_or_else(|| cut_err(input, "Missing required value"))?,
        center_name: center_name.ok_or_else(|| cut_err(input, "Missing required value"))?,
        ref_frame: ref_frame.ok_or_else(|| cut_err(input, "Missing required value"))?,
        ref_frame_epoch,
        time_system: time_system.ok_or_else(|| cut_err(input, "Missing required value"))?,
    })
}

//----------------------------------------------------------------------
// Keplerian Elements Parser
//----------------------------------------------------------------------

/// Checks if current key is a Keplerian element key.
fn is_keplerian_key(key: &str) -> bool {
    matches!(
        key,
        "SEMI_MAJOR_AXIS"
            | "ECCENTRICITY"
            | "INCLINATION"
            | "RA_OF_ASC_NODE"
            | "ARG_OF_PERICENTER"
            | "TRUE_ANOMALY"
            | "MEAN_ANOMALY"
            | "GM"
    )
}

/// Parses the optional Keplerian elements section.
pub fn keplerian_elements(input: &mut &str) -> ModalResult<Option<KeplerianElements>> {
    let mut comment = Vec::new();
    let mut semi_major_axis = None;
    let mut eccentricity = None;
    let mut inclination = None;
    let mut ra_of_asc_node = None;
    let mut arg_of_pericenter = None;
    let mut true_anomaly = None;
    let mut mean_anomaly = None;
    let mut gm = None;

    // Check if we have any Keplerian keys
    let next_key = peek_key(input)?;
    if !matches!(next_key, Some(k) if is_keplerian_key(k)) {
        // Also check for comments before Keplerian section
        let comments = collect_comments.parse_next(input)?;
        if !comments.is_empty() {
            let next_key = peek_key(input)?;
            if !matches!(next_key, Some(k) if is_keplerian_key(k)) {
                // No Keplerian section, but we consumed comments - this is fine
                // The comments might belong to spacecraft params
                return Ok(None);
            }
            comment.extend(comments);
        } else {
            return Ok(None);
        }
    }

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if is_keplerian_key(k) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "SEMI_MAJOR_AXIS" => {
                        semi_major_axis = Some(
                            Distance::from_kvn(val, unit.or(Some("km")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "ECCENTRICITY" => {
                        eccentricity =
                            Some(parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    "INCLINATION" => {
                        let angle = Angle::from_kvn(val, unit)
                            .map_err(|_| cut_err(input, "Invalid value"))?;
                        inclination = Some(
                            Inclination::new(angle.value, angle.units)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "RA_OF_ASC_NODE" => {
                        ra_of_asc_node = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "ARG_OF_PERICENTER" => {
                        arg_of_pericenter = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "TRUE_ANOMALY" => {
                        true_anomaly = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MEAN_ANOMALY" => {
                        mean_anomaly = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "GM" => {
                        gm = Some(
                            Gm::from_kvn(val, unit.or(Some("km**3/s**2")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    // If we have any Keplerian data, build the struct
    if semi_major_axis.is_some() || eccentricity.is_some() {
        Ok(Some(KeplerianElements {
            comment,
            semi_major_axis: semi_major_axis
                .ok_or_else(|| cut_err(input, "Missing required value"))?,
            eccentricity: eccentricity.ok_or_else(|| cut_err(input, "Missing required value"))?,
            inclination: inclination.ok_or_else(|| cut_err(input, "Missing required value"))?,
            ra_of_asc_node: ra_of_asc_node
                .ok_or_else(|| cut_err(input, "Missing required value"))?,
            arg_of_pericenter: arg_of_pericenter
                .ok_or_else(|| cut_err(input, "Missing required value"))?,
            true_anomaly,
            mean_anomaly,
            gm: gm.ok_or_else(|| cut_err(input, "Missing required value"))?,
        }))
    } else {
        Ok(None)
    }
}

//----------------------------------------------------------------------
// Spacecraft Parameters Parser
//----------------------------------------------------------------------

fn is_spacecraft_key(key: &str) -> bool {
    matches!(
        key,
        "MASS" | "SOLAR_RAD_AREA" | "SOLAR_RAD_COEFF" | "DRAG_AREA" | "DRAG_COEFF"
    )
}

/// Parses the optional spacecraft parameters section.
pub fn spacecraft_parameters(input: &mut &str) -> ModalResult<Option<SpacecraftParameters>> {
    let mut comment = Vec::new();
    let mut mass = None;
    let mut solar_rad_area = None;
    let mut solar_rad_coeff = None;
    let mut drag_area = None;
    let mut drag_coeff = None;

    // Check if we have any spacecraft keys
    let next_key = peek_key(input)?;
    if !matches!(next_key, Some(k) if is_spacecraft_key(k)) {
        let comments = collect_comments.parse_next(input)?;
        if !comments.is_empty() {
            let next_key = peek_key(input)?;
            if !matches!(next_key, Some(k) if is_spacecraft_key(k)) {
                return Ok(None);
            }
            comment.extend(comments);
        } else {
            return Ok(None);
        }
    }

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if is_spacecraft_key(k) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "MASS" => {
                        mass = Some(
                            Mass::from_kvn(val, unit.or(Some("kg")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "SOLAR_RAD_AREA" => {
                        solar_rad_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "SOLAR_RAD_COEFF" => {
                        solar_rad_coeff =
                            Some(parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    "DRAG_AREA" => {
                        drag_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "DRAG_COEFF" => {
                        drag_coeff =
                            Some(parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    // If we have any spacecraft data, build the struct
    if mass.is_some() || solar_rad_area.is_some() || drag_area.is_some() {
        Ok(Some(SpacecraftParameters {
            comment,
            mass,
            solar_rad_area,
            solar_rad_coeff,
            drag_area,
            drag_coeff,
        }))
    } else {
        Ok(None)
    }
}

//----------------------------------------------------------------------
// Maneuver Parameters Parser
//----------------------------------------------------------------------

fn is_maneuver_key(key: &str) -> bool {
    matches!(
        key,
        "MAN_EPOCH_IGNITION"
            | "MAN_DURATION"
            | "MAN_DELTA_MASS"
            | "MAN_REF_FRAME"
            | "MAN_DV_1"
            | "MAN_DV_2"
            | "MAN_DV_3"
    )
}

/// Parses a single maneuver parameter block.
pub fn maneuver_parameters(input: &mut &str) -> ModalResult<Option<ManeuverParameters>> {
    let mut comment = Vec::new();
    let mut man_epoch_ignition = None;
    let mut man_duration = None;
    let mut man_delta_mass = None;
    let mut man_ref_frame = None;
    let mut man_dv_1 = None;
    let mut man_dv_2 = None;
    let mut man_dv_3 = None;

    // Check if we have maneuver keys
    let next_key = peek_key(input)?;
    if !matches!(next_key, Some(k) if is_maneuver_key(k)) {
        let comments = collect_comments.parse_next(input)?;
        if !comments.is_empty() {
            let next_key = peek_key(input)?;
            if !matches!(next_key, Some(k) if is_maneuver_key(k)) {
                return Ok(None);
            }
            comment.extend(comments);
        } else {
            return Ok(None);
        }
    }

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some("MAN_EPOCH_IGNITION") if man_epoch_ignition.is_some() => {
                // New maneuver starting - return current one
                break;
            }
            Some(k) if is_maneuver_key(k) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "MAN_EPOCH_IGNITION" => {
                        man_epoch_ignition = Some(
                            Epoch::from_str(val).map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MAN_DURATION" => {
                        man_duration = Some(
                            Duration::from_kvn(val, unit.or(Some("s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MAN_DELTA_MASS" => {
                        let value = parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?;
                        let units = unit.and_then(|u| u.parse::<MassUnits>().ok());
                        man_delta_mass = Some(
                            DeltaMassZ::new(value, units)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MAN_REF_FRAME" => {
                        man_ref_frame = Some(val.to_string());
                    }
                    "MAN_DV_1" => {
                        man_dv_1 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MAN_DV_2" => {
                        man_dv_2 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "MAN_DV_3" => {
                        man_dv_3 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    // If we have maneuver data, build the struct
    if man_epoch_ignition.is_some() {
        Ok(Some(ManeuverParameters {
            comment,
            man_epoch_ignition: man_epoch_ignition
                .ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_duration: man_duration.ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_delta_mass: man_delta_mass
                .ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_ref_frame: man_ref_frame.ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_dv_1: man_dv_1.ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_dv_2: man_dv_2.ok_or_else(|| cut_err(input, "Missing required value"))?,
            man_dv_3: man_dv_3.ok_or_else(|| cut_err(input, "Missing required value"))?,
        }))
    } else {
        Ok(None)
    }
}

/// Parses all maneuver parameter blocks.
pub fn all_maneuvers(input: &mut &str) -> ModalResult<Vec<ManeuverParameters>> {
    let mut maneuvers = Vec::new();

    loop {
        match maneuver_parameters.parse_next(input) {
            Ok(Some(man)) => maneuvers.push(man),
            Ok(None) => break,
            Err(e) => return Err(e), // Propagate errors
        }
    }

    Ok(maneuvers)
}

//----------------------------------------------------------------------
// User Defined Parameters Parser
//----------------------------------------------------------------------

/// Parses user-defined parameters.
pub fn user_defined_parameters(input: &mut &str) -> ModalResult<Option<UserDefined>> {
    let mut comment = Vec::new();
    let mut params = Vec::new();

    loop {
        let comments = collect_comments.parse_next(input)?;

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if k.starts_with("USER_DEFINED_") => {
                comment.extend(comments);
                let (key, val, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                params.push(UserDefinedParameter {
                    parameter: key.to_string(),
                    value: val.to_string(),
                });
            }
            _ => break,
        }
    }

    if params.is_empty() {
        Ok(None)
    } else {
        Ok(Some(UserDefined {
            comment,
            user_defined: params,
        }))
    }
}

//----------------------------------------------------------------------
// OPM Data Parser
//----------------------------------------------------------------------

/// Parses the complete OPM data section.
pub fn opm_data(input: &mut &str) -> ModalResult<OpmData> {
    // Parse state vector (required)
    let (sv_comment, state_vector) = state_vector.parse_next(input)?;

    // Parse optional sections in order
    let keplerian_elements = keplerian_elements.parse_next(input)?;
    let spacecraft_parameters = spacecraft_parameters.parse_next(input)?;
    let covariance_matrix = covariance_matrix.parse_next(input)?;
    let maneuver_parameters = all_maneuvers.parse_next(input)?;
    let user_defined_parameters = user_defined_parameters.parse_next(input)?;

    Ok(OpmData {
        comment: sv_comment,
        state_vector,
        keplerian_elements,
        spacecraft_parameters,
        covariance_matrix,
        maneuver_parameters,
        user_defined_parameters,
    })
}

//----------------------------------------------------------------------
// Complete OPM Parser
//----------------------------------------------------------------------

/// Parses a complete OPM message.
pub fn parse_opm(input: &mut &str) -> ModalResult<Opm> {
    // 1. Version
    let version = opm_version.parse_next(input)?;

    // 2. Header
    let header = odm_header.parse_next(input)?;

    // 3. Metadata
    let metadata = opm_metadata.parse_next(input)?;

    // 4. Data
    let data = opm_data.parse_next(input)?;

    Ok(Opm {
        header,
        body: OpmBody {
            segment: OpmSegment { metadata, data },
        },
        id: Some("CCSDS_OPM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Opm {
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
        parse_opm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_OPM: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = OSPREY 5
OBJECT_ID = 1998-999A
CENTER_NAME = EARTH
REF_FRAME = ITRF2000
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514
Y = 1239.647
Z = -717.490
X_DOT = -0.873160
Y_DOT = 8.740420
Z_DOT = -4.191076
"#;

    const OPM_WITH_UNITS: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = OSPREY 5
OBJECT_ID = 1998-999A
CENTER_NAME = EARTH
REF_FRAME = ITRF2000
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514 [km]
Y = 1239.647 [km]
Z = -717.490 [km]
X_DOT = -0.873160 [km/s]
Y_DOT = 8.740420 [km/s]
Z_DOT = -4.191076 [km/s]
"#;

    #[test]
    fn test_parse_minimal_opm() {
        let result = Opm::from_kvn_str(MINIMAL_OPM);
        assert!(
            result.is_ok(),
            "Failed to parse minimal OPM: {:?}",
            result.err()
        );

        let opm = result.unwrap();
        assert_eq!(opm.version, "3.0");
        assert_eq!(opm.header.originator, "JAXA");
        assert_eq!(opm.body.segment.metadata.object_name, "OSPREY 5");
        assert_eq!(opm.body.segment.metadata.object_id, "1998-999A");
    }

    #[test]
    fn test_parse_opm_with_units() {
        let result = Opm::from_kvn_str(OPM_WITH_UNITS);
        assert!(
            result.is_ok(),
            "Failed to parse OPM with units: {:?}",
            result.err()
        );

        let opm = result.unwrap();
        assert_eq!(opm.body.segment.data.state_vector.x.value, 6503.514);
    }

    #[test]
    fn test_parse_opm_version() {
        let mut input = "CCSDS_OPM_VERS = 3.0\n";
        let version = opm_version.parse_next(&mut input).unwrap();
        assert_eq!(version, "3.0");
    }

    #[test]
    fn test_parse_odm_header() {
        let mut input =
            "CREATION_DATE = 2022-11-06T09:23:57\nORIGINATOR = JAXA\nOBJECT_NAME = TEST\n";
        let header = odm_header.parse_next(&mut input).unwrap();
        assert_eq!(header.originator, "JAXA");
        assert_eq!(header.creation_date.as_str(), "2022-11-06T09:23:57");
    }

    #[test]
    fn test_parse_opm_metadata() {
        let input_str = "OBJECT_NAME = SAT1\nOBJECT_ID = 2023-001A\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nEPOCH = 2023-01-01T00:00:00\n";
        let mut input = input_str;
        let metadata = opm_metadata.parse_next(&mut input).unwrap();
        assert_eq!(metadata.object_name, "SAT1");
        assert_eq!(metadata.object_id, "2023-001A");
    }

    #[test]
    fn test_parse_opm_with_spacecraft_params() {
        const OPM_WITH_SC: &str = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = OSPREY 5
OBJECT_ID = 1998-999A
CENTER_NAME = EARTH
REF_FRAME = ITRF2000
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514
Y = 1239.647
Z = -717.490
X_DOT = -0.873160
Y_DOT = 8.740420
Z_DOT = -4.191076
MASS = 3000.0
SOLAR_RAD_AREA = 18.77
SOLAR_RAD_COEFF = 1.0
DRAG_AREA = 18.77
DRAG_COEFF = 2.5
"#;

        let result = Opm::from_kvn_str(OPM_WITH_SC);
        assert!(
            result.is_ok(),
            "Failed to parse OPM with spacecraft params: {:?}",
            result.err()
        );

        let opm = result.unwrap();
        let sc = opm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .expect("Should have spacecraft params");
        assert_eq!(sc.mass.as_ref().unwrap().value, 3000.0);
        assert_eq!(*sc.drag_coeff.as_ref().unwrap(), 2.5);
    }

    #[test]
    fn test_opm_errors() {
        // Version error
        assert!(opm_version
            .parse_next(&mut "CCSDS_OPM_VERS = BAD\n")
            .is_err());

        // Metadata errors
        let mut kvn_meta_err = "OBJECT_NAME = SAT\nUNKNOWN_KEY = VAL\n";
        assert!(opm_metadata.parse_next(&mut kvn_meta_err).is_err());

        let mut kvn_epoch_err = "REF_FRAME_EPOCH = INVALID\n";
        assert!(opm_metadata.parse_next(&mut kvn_epoch_err).is_err());

        // Keplerian errors
        let mut kvn_kep_err = "SEMI_MAJOR_AXIS = 7000.0\n"; // Missing others
        assert!(keplerian_elements.parse_next(&mut kvn_kep_err).is_err());

        let mut input = "SEMI_MAJOR_AXIS = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "ECCENTRICITY = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "INCLINATION = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "INCLINATION = 190.0\n"; // Out of range
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "RA_OF_ASC_NODE = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "ARG_OF_PERICENTER = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "TRUE_ANOMALY = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "MEAN_ANOMALY = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        let mut input = "GM = BAD\n";
        assert!(keplerian_elements.parse_next(&mut input).is_err());

        // Spacecraft errors
        let mut input = "MASS = BAD\n";
        assert!(spacecraft_parameters.parse_next(&mut input).is_err());
        let mut input = "SOLAR_RAD_AREA = BAD\n";
        assert!(spacecraft_parameters.parse_next(&mut input).is_err());
        let mut input = "SOLAR_RAD_COEFF = BAD\n";
        assert!(spacecraft_parameters.parse_next(&mut input).is_err());
        let mut input = "DRAG_AREA = BAD\n";
        assert!(spacecraft_parameters.parse_next(&mut input).is_err());
        let mut input = "DRAG_COEFF = BAD\n";
        assert!(spacecraft_parameters.parse_next(&mut input).is_err());

        // Maneuver errors
        let mut input = "MAN_EPOCH_IGNITION = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DURATION = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DELTA_MASS = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DV_1 = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DV_2 = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DV_3 = BAD\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());

        // Incomplete maneuver
        let mut input = "MAN_EPOCH_IGNITION = 2023-01-01T00:00:00\n";
        assert!(maneuver_parameters.parse_next(&mut input).is_err());

        // Trailing data error
        let kvn_trailing = format!("{}EXTRA = DATA\n", MINIMAL_OPM);
        assert!(Opm::from_kvn_str(&kvn_trailing).is_err());
    }

    #[test]
    fn test_opm_optional_comments() {
        let mut input = "COMMENT kep comment\nSEMI_MAJOR_AXIS = 7000.0\nECCENTRICITY = 0.0\nINCLINATION = 0.0\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nTRUE_ANOMALY = 0.0\nGM = 398600.44\n";
        let kep = keplerian_elements.parse_next(&mut input).unwrap().unwrap();
        assert_eq!(kep.comment, vec!["kep comment"]);

        let mut input = "COMMENT sc comment\nMASS = 1000.0\n";
        let sc = spacecraft_parameters
            .parse_next(&mut input)
            .unwrap()
            .unwrap();
        assert_eq!(sc.comment, vec!["sc comment"]);

        let mut input = "COMMENT man comment\nMAN_EPOCH_IGNITION = 2023-01-01T00:00:00\nMAN_DURATION = 0.0\nMAN_DELTA_MASS = 0.0\nMAN_REF_FRAME = TNW\nMAN_DV_1 = 0.0\nMAN_DV_2 = 0.0\nMAN_DV_3 = 0.0\n";
        let man = maneuver_parameters.parse_next(&mut input).unwrap().unwrap();
        assert_eq!(man.comment, vec!["man comment"]);
    }

    #[test]
    fn test_opm_optional_empty() {
        let mut input = "COMMENT only comment\nNOT_A_KEY = VAL\n";
        assert!(keplerian_elements.parse_next(&mut input).unwrap().is_none());

        let mut input = "COMMENT only comment\nNOT_A_KEY = VAL\n";
        assert!(spacecraft_parameters
            .parse_next(&mut input)
            .unwrap()
            .is_none());

        let mut input = "COMMENT only comment\nNOT_A_KEY = VAL\n";
        assert!(maneuver_parameters
            .parse_next(&mut input)
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_opm_user_defined() {
        let mut input = "COMMENT user comment\nUSER_DEFINED_FOO = BAR\nUSER_DEFINED_BAZ = QUX\n";
        let ud = user_defined_parameters
            .parse_next(&mut input)
            .unwrap()
            .unwrap();
        assert_eq!(ud.comment, vec!["user comment"]);
        assert_eq!(ud.user_defined.len(), 2);
        assert_eq!(ud.user_defined[0].parameter, "USER_DEFINED_FOO");
    }

    #[test]
    fn test_opm_data_loop() {
        // Test multiple maneuvers
        let mut input = r#"EPOCH = 2023-01-01T00:00:00
X = 1000
Y = 2000
Z = 3000
X_DOT = 1
Y_DOT = 2
Z_DOT = 3
MAN_EPOCH_IGNITION = 2023-01-01T01:00:00
MAN_DURATION = 10
MAN_DELTA_MASS = -1
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1
MAN_DV_2 = 0.2
MAN_DV_3 = 0.3
MAN_EPOCH_IGNITION = 2023-01-01T02:00:00
MAN_DURATION = 20
MAN_DELTA_MASS = -2
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.4
MAN_DV_2 = 0.5
MAN_DV_3 = 0.6
"#;
        let data = opm_data.parse_next(&mut input).unwrap();
        assert_eq!(data.maneuver_parameters.len(), 2);
    }
}
