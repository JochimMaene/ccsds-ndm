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
use winnow::error::{ContextError, ErrMode};
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
                        ref_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TIME_SYSTEM" => time_system = Some(v.to_string()),
                    _ => {
                        return Err(ErrMode::Cut(ContextError::new()));
                    }
                }
            }
            None => break,
        }
    }

    Ok(OpmMetadata {
        comment,
        object_name: object_name.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        object_id: object_id.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        center_name: center_name.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        ref_frame: ref_frame.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
        ref_frame_epoch,
        time_system: time_system.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
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
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ECCENTRICITY" => {
                        eccentricity =
                            Some(parse_f64(val).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "INCLINATION" => {
                        let angle = Angle::from_kvn(val, unit)
                            .map_err(|_| ErrMode::Cut(ContextError::new()))?;
                        inclination = Some(
                            Inclination::new(angle.value, angle.units)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "RA_OF_ASC_NODE" => {
                        ra_of_asc_node = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "ARG_OF_PERICENTER" => {
                        arg_of_pericenter = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "TRUE_ANOMALY" => {
                        true_anomaly = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MEAN_ANOMALY" => {
                        mean_anomaly = Some(
                            Angle::from_kvn(val, unit)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "GM" => {
                        gm = Some(
                            Gm::from_kvn(val, unit.or(Some("km**3/s**2")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
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
            semi_major_axis: semi_major_axis.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            eccentricity: eccentricity.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            inclination: inclination.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            ra_of_asc_node: ra_of_asc_node.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            arg_of_pericenter: arg_of_pericenter
                .ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            true_anomaly,
            mean_anomaly,
            gm: gm.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
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
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SOLAR_RAD_AREA" => {
                        solar_rad_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "SOLAR_RAD_COEFF" => {
                        solar_rad_coeff =
                            Some(parse_f64(val).map_err(|_| ErrMode::Cut(ContextError::new()))?);
                    }
                    "DRAG_AREA" => {
                        drag_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "DRAG_COEFF" => {
                        drag_coeff =
                            Some(parse_f64(val).map_err(|_| ErrMode::Cut(ContextError::new()))?);
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
                            Epoch::from_str(val).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_DURATION" => {
                        man_duration = Some(
                            Duration::from_kvn(val, unit.or(Some("s")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_DELTA_MASS" => {
                        let value =
                            parse_f64(val).map_err(|_| ErrMode::Cut(ContextError::new()))?;
                        let units = unit.and_then(|u| u.parse::<MassUnits>().ok());
                        man_delta_mass = Some(
                            DeltaMassZ::new(value, units)
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_REF_FRAME" => {
                        man_ref_frame = Some(val.to_string());
                    }
                    "MAN_DV_1" => {
                        man_dv_1 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_DV_2" => {
                        man_dv_2 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
                    }
                    "MAN_DV_3" => {
                        man_dv_3 = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| ErrMode::Cut(ContextError::new()))?,
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
                .ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_duration: man_duration.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_delta_mass: man_delta_mass.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_ref_frame: man_ref_frame.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_dv_1: man_dv_1.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_dv_2: man_dv_2.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
            man_dv_3: man_dv_3.ok_or_else(|| ErrMode::Cut(ContextError::new()))?,
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
}
