// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
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

use super::{
    validate_input_size, FieldPath, KeplerianElements, ManeuverParameters, Opm, OpmBody, OpmData,
    OpmMetadata, OpmSegment,
};
use crate::error::{Result, ValidationError};
use crate::kvn::parser::*;
use crate::kvn::ser::{KvnWriter, OdmFloat};
use crate::parse_block;
use crate::traits::ToKvn;
use crate::types::*;
use winnow::prelude::*;
use winnow::stream::Offset;

//----------------------------------------------------------------------
// OPM Version Parser
//----------------------------------------------------------------------

/// Parses the OPM version line: `CCSDS_OPM_VERS = 3.0`
pub fn opm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    // Skip any leading comments/empty lines
    let _ = collect_comments.parse_next(input)?;

    let (value, _) = expect_key("CCSDS_OPM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OPM Metadata Parser
//----------------------------------------------------------------------

/// Parses the OPM metadata section.
pub fn opm_metadata(input: &mut &str) -> KvnResult<OpmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_calendar_epoch,
        "TIME_SYSTEM" => time_system: kv_string,
    }, |_| false);

    Ok(OpmMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "OPM Metadata", "OBJECT_NAME"))?,
        object_id: object_id
            .ok_or_else(|| missing_field_err(input, "OPM Metadata", "OBJECT_ID"))?,
        center_name: center_name
            .ok_or_else(|| missing_field_err(input, "OPM Metadata", "CENTER_NAME"))?,
        ref_frame: ref_frame
            .ok_or_else(|| missing_field_err(input, "OPM Metadata", "REF_FRAME"))?,
        ref_frame_epoch,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "OPM Metadata", "TIME_SYSTEM"))?,
    })
}

//----------------------------------------------------------------------
// Keplerian Elements Parser
//----------------------------------------------------------------------

/// Parses the optional Keplerian elements section.
pub fn keplerian_elements(input: &mut &str) -> KvnResult<Option<KeplerianElements>> {
    let mut comment = Vec::new();
    let mut semi_major_axis = None;
    let mut eccentricity = None;
    let mut inclination = None;
    let mut ra_of_asc_node = None;
    let mut arg_of_pericenter = None;
    let mut true_anomaly = None;
    let mut mean_anomaly = None;
    let mut gm = None;

    parse_block!(input, comment, {
        "SEMI_MAJOR_AXIS" => semi_major_axis: kv_from_kvn,
        "ECCENTRICITY" => eccentricity: kv_from_kvn,
        "INCLINATION" => inclination: kv_from_kvn,
        "RA_OF_ASC_NODE" => ra_of_asc_node: kv_from_kvn,
        "ARG_OF_PERICENTER" => arg_of_pericenter: kv_from_kvn,
        "TRUE_ANOMALY" => true_anomaly: kv_from_kvn,
        "MEAN_ANOMALY" => mean_anomaly: kv_from_kvn,
        "GM" => gm: kv_from_kvn,
    }, |_| false);

    if semi_major_axis.is_none()
        && eccentricity.is_none()
        && inclination.is_none()
        && ra_of_asc_node.is_none()
        && arg_of_pericenter.is_none()
        && true_anomaly.is_none()
        && mean_anomaly.is_none()
        && gm.is_none()
    {
        return Ok(None);
    }

    Ok(Some(KeplerianElements {
        comment,
        semi_major_axis: semi_major_axis
            .ok_or_else(|| missing_field_err(input, "Keplerian Elements", "SEMI_MAJOR_AXIS"))?,
        eccentricity: eccentricity
            .ok_or_else(|| missing_field_err(input, "Keplerian Elements", "ECCENTRICITY"))?,
        inclination: inclination
            .ok_or_else(|| missing_field_err(input, "Keplerian Elements", "INCLINATION"))?,
        ra_of_asc_node: ra_of_asc_node
            .ok_or_else(|| missing_field_err(input, "Keplerian Elements", "RA_OF_ASC_NODE"))?,
        arg_of_pericenter: arg_of_pericenter
            .ok_or_else(|| missing_field_err(input, "Keplerian Elements", "ARG_OF_PERICENTER"))?,
        true_anomaly,
        mean_anomaly,
        gm: gm.ok_or_else(|| missing_field_err(input, "Keplerian Elements", "GM"))?,
    }))
}

//----------------------------------------------------------------------
// Maneuver Parameters Parser
//----------------------------------------------------------------------

/// Parses a single maneuver parameter block.
pub fn maneuver_parameters(input: &mut &str) -> KvnResult<Option<ManeuverParameters>> {
    // ODM §7.8.7 permits comments only at the beginning of a maneuver logical block. Collecting
    // comments inside the assignment loop would steal comments belonging to the next maneuver.
    let comment = collect_comments.parse_next(input)?;
    let mut man_epoch_ignition = None;
    let mut man_duration = None;
    let mut man_delta_mass = None;
    let mut man_ref_frame = None;
    let mut man_dv_1 = None;
    let mut man_dv_2 = None;
    let mut man_dv_3 = None;

    loop {
        let checkpoint = input.checkpoint();
        let key = match key_token.parse_next(input) {
            Ok(key) => key,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };
        if key == "MAN_EPOCH_IGNITION" && man_epoch_ignition.is_some() {
            input.reset(&checkpoint);
            break;
        }
        match key {
            "MAN_EPOCH_IGNITION" => {
                man_epoch_ignition = Some(kv_calendar_epoch.parse_next(input)?);
            }
            "MAN_DURATION" => man_duration = Some(kv_from_kvn.parse_next(input)?),
            "MAN_DELTA_MASS" => man_delta_mass = Some(kv_from_kvn.parse_next(input)?),
            "MAN_REF_FRAME" => man_ref_frame = Some(kv_string.parse_next(input)?),
            "MAN_DV_1" => man_dv_1 = Some(kv_from_kvn.parse_next(input)?),
            "MAN_DV_2" => man_dv_2 = Some(kv_from_kvn.parse_next(input)?),
            "MAN_DV_3" => man_dv_3 = Some(kv_from_kvn.parse_next(input)?),
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }
    }

    if let Some(ignition) = man_epoch_ignition {
        Ok(Some(ManeuverParameters {
            comment,
            man_epoch_ignition: ignition,
            man_duration: man_duration
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_DURATION"))?,
            man_delta_mass: man_delta_mass
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_DELTA_MASS"))?,
            man_ref_frame: man_ref_frame
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_REF_FRAME"))?,
            man_dv_1: man_dv_1
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_DV_1"))?,
            man_dv_2: man_dv_2
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_DV_2"))?,
            man_dv_3: man_dv_3
                .ok_or_else(|| missing_field_err(input, "Maneuver Parameters", "MAN_DV_3"))?,
        }))
    } else if man_duration.is_some()
        || man_delta_mass.is_some()
        || man_ref_frame.is_some()
        || man_dv_1.is_some()
        || man_dv_2.is_some()
        || man_dv_3.is_some()
    {
        Err(missing_field_err(
            input,
            "Maneuver Parameters",
            "MAN_EPOCH_IGNITION",
        ))
    } else {
        Ok(None)
    }
}

/// Parses all maneuver parameter blocks.
pub fn all_maneuvers(input: &mut &str) -> KvnResult<Vec<ManeuverParameters>> {
    let mut maneuvers = Vec::new();

    loop {
        let checkpoint = input.checkpoint();
        match maneuver_parameters.parse_next(input) {
            Ok(Some(man)) => maneuvers.push(man),
            Ok(None) => {
                input.reset(&checkpoint);
                break;
            }
            Err(e) => return Err(e),
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(maneuvers)
}

//----------------------------------------------------------------------
// OPM Data Parser
//----------------------------------------------------------------------

/// Parses the complete OPM data section.
pub fn opm_data(input: &mut &str) -> KvnResult<OpmData> {
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
pub fn parse_opm(input: &mut &str) -> KvnResult<Opm> {
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
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_opm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;
    use crate::types::NonNegativeDouble;

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
    fn opm_epoch_fields_require_calendar_form() {
        let numeric_state_epoch =
            MINIMAL_OPM.replace("EPOCH = 2022-12-18T14:28:15.1172", "EPOCH = 12345.5");
        assert!(Opm::from_kvn_str(&numeric_state_epoch).is_err());

        let with_reference_epoch = MINIMAL_OPM.replace(
            "REF_FRAME = ITRF2000\n",
            "REF_FRAME = ITRF2000\nREF_FRAME_EPOCH = 2000-01-01T12:00:00\n",
        );
        let numeric_reference_epoch = with_reference_epoch.replace(
            "REF_FRAME_EPOCH = 2000-01-01T12:00:00",
            "REF_FRAME_EPOCH = 12345.5",
        );
        assert!(Opm::from_kvn_str(&numeric_reference_epoch).is_err());

        let maneuver = include_str!("../../../data/kvn/opm_g2.kvn");
        let numeric_maneuver_epoch = maneuver.replace(
            "MAN_EPOCH_IGNITION = 2021-06-03T09:00:34.1",
            "MAN_EPOCH_IGNITION = 12345.5",
        );
        assert!(Opm::from_kvn_str(&numeric_maneuver_epoch).is_err());
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
        assert_eq!(
            sc.drag_coeff.as_ref().unwrap(),
            &NonNegativeDouble::new(2.5).unwrap()
        );
    }

    #[test]
    fn test_opm_errors() {
        // Version error
        let bad_version = MINIMAL_OPM.replacen("CCSDS_OPM_VERS = 3.0", "CCSDS_OPM_VERS = BAD", 1);
        let err = Opm::from_kvn(&bad_version).unwrap_err();
        assert_eq!(err.code(), Some("parse.unsupported_input_version"));
        assert_eq!(
            err.diagnostic()
                .and_then(|diagnostic| diagnostic.source_edition),
            Some("BAD")
        );

        // Metadata errors
        let mut kvn_meta_err = "OBJECT_NAME = SAT\nUNKNOWN_KEY = VAL\n";
        assert!(opm_metadata.parse_next(&mut kvn_meta_err).is_err());

        let mut kvn_epoch_err = "REF_FRAME_EPOCH = INVALID\n";
        assert!(opm_metadata.parse_next(&mut kvn_epoch_err).is_err());

        // Keplerian errors
        let mut kvn_kep_err = "SEMI_MAJOR_AXIS = 7000.0\n"; // Missing others
        assert!(keplerian_elements.parse_next(&mut kvn_kep_err).is_err());

        // Keplerian: both TRUE_ANOMALY and MEAN_ANOMALY parses; semantic check is in validation
        let mut kvn_kep_both = "SEMI_MAJOR_AXIS = 7000.0\nECCENTRICITY = 0.0\nINCLINATION = 0.0\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nTRUE_ANOMALY = 0.0\nMEAN_ANOMALY = 0.0\nGM = 398600.44\n";
        let ke_both = keplerian_elements
            .parse_next(&mut kvn_kep_both)
            .unwrap()
            .unwrap();
        assert!(ke_both.true_anomaly.is_some());
        assert!(ke_both.mean_anomaly.is_some());

        // Keplerian: neither TRUE_ANOMALY nor MEAN_ANOMALY parses; semantic check is in validation
        let mut kvn_kep_none = "SEMI_MAJOR_AXIS = 7000.0\nECCENTRICITY = 0.0\nINCLINATION = 0.0\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nGM = 398600.44\n";
        let ke_none = keplerian_elements
            .parse_next(&mut kvn_kep_none)
            .unwrap()
            .unwrap();
        assert!(ke_none.true_anomaly.is_none());
        assert!(ke_none.mean_anomaly.is_none());

        // Maneuver without MASS should parse (mass is optional in OPM spacecraft parameters)
        let kvn_man_no_mass = r#"CCSDS_OPM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = SAT
OBJECT_ID = 1
CENTER_NAME = EARTH
REF_FRAME = GCRF
TIME_SYSTEM = UTC
EPOCH = 2022-12-18T14:28:15.1172
X = 6503.514
Y = 1239.647
Z = -717.490
X_DOT = -0.873160
Y_DOT = 8.740420
Z_DOT = -4.191076
MAN_EPOCH_IGNITION = 2023-01-01T00:00:00
MAN_DURATION = 10.0
MAN_DELTA_MASS = -1.0
MAN_REF_FRAME = RSW
MAN_DV_1 = 0.1
MAN_DV_2 = 0.0
MAN_DV_3 = 0.0
"#;
        let opm = Opm::from_kvn_str(kvn_man_no_mass).expect("maneuver without MASS should parse");
        assert!(opm
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_ref()
            .and_then(|sp| sp.mass.as_ref())
            .is_none());
        assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 1);

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
        assert_eq!(ud.user_defined[0].parameter, "FOO");
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
MASS = 1000
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

    #[test]
    fn test_opm_covariance_matrix() {
        let mut input = r#"EPOCH = 2023-01-01T00:00:00
X = 1000
Y = 2000
Z = 3000
X_DOT = 1
Y_DOT = 2
Z_DOT = 3
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
"#;
        let data = opm_data.parse_next(&mut input).unwrap();
        assert!(data.covariance_matrix.is_some());
        let cov = data.covariance_matrix.unwrap();
        assert_eq!(cov.cx_x.value, 1.0);
    }

    #[test]
    fn test_opm_metadata_missing_fields() {
        // Missing OBJECT_NAME
        let mut input = "OBJECT_ID = 1\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\n";
        assert!(opm_metadata.parse_next(&mut input).is_err());

        // Missing TIME_SYSTEM
        let mut input = "OBJECT_NAME = SAT\nOBJECT_ID = 1\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\n";
        assert!(opm_metadata.parse_next(&mut input).is_err());
    }
}

/// OPM's KVN keyword layout, declared once.
///
/// The ordering fact used to live in three hand-synchronised functions coupled through opaque
/// numbers: a rank table with meaningful gaps, a `comment_starts_block` predicate written in rank
/// literals and ranges such as `17 | 26 | 30..=34 | 61 | 76`, and a repeat predicate written in
/// bare ranks. Adding a keyword to a block meant widening a range in two other places, with
/// nothing but a test to catch a mistake. Rank, block starts and repeats are now derived from this
/// table, so the layout is stated once and the numbers are an implementation detail.
struct OpmKvnBlock {
    keywords: &'static [&'static str],
    /// How many leading keywords may open the block, because everything before them is optional.
    leading_optional: usize,
    /// How many trailing keywords may close the block, because everything after them is optional.
    trailing_optional: usize,
    /// The block may follow itself, as repeated maneuvers and `USER_DEFINED_*` do.
    repeatable: bool,
    /// A repeat may also open a fresh comment run. Maneuvers do; `USER_DEFINED_*` repeats inside
    /// one logical block, so a comment there does not start a new one.
    comment_restarts: bool,
    /// The whole block may be absent. A mandatory block between two others blocks the path, which
    /// is why the header cannot immediately precede the metadata.
    optional: bool,
}

const OPM_KVN_BLOCKS: &[OpmKvnBlock] = &[
    // The version keyword anchors the document and is its own group.
    OpmKvnBlock {
        keywords: &["CCSDS_OPM_VERS"],
        leading_optional: 0,
        trailing_optional: 1,
        repeatable: false,
        comment_restarts: false,
        optional: false,
    },
    // CLASSIFICATION is optional, so either it or CREATION_DATE may open the header content.
    OpmKvnBlock {
        keywords: &[
            "CLASSIFICATION",
            "CREATION_DATE",
            "ORIGINATOR",
            "MESSAGE_ID",
        ],
        leading_optional: 2,
        trailing_optional: 2,
        repeatable: false,
        comment_restarts: false,
        optional: false,
    },
    OpmKvnBlock {
        keywords: &[
            "OBJECT_NAME",
            "OBJECT_ID",
            "CENTER_NAME",
            "REF_FRAME",
            "REF_FRAME_EPOCH",
            "TIME_SYSTEM",
        ],
        leading_optional: 1,
        trailing_optional: 1,
        repeatable: false,
        comment_restarts: false,
        optional: false,
    },
    OpmKvnBlock {
        keywords: &["EPOCH", "X", "Y", "Z", "X_DOT", "Y_DOT", "Z_DOT"],
        leading_optional: 1,
        trailing_optional: 1,
        repeatable: false,
        comment_restarts: false,
        optional: false,
    },
    OpmKvnBlock {
        keywords: &[
            "SEMI_MAJOR_AXIS",
            "ECCENTRICITY",
            "INCLINATION",
            "RA_OF_ASC_NODE",
            "ARG_OF_PERICENTER",
            "TRUE_ANOMALY",
            "GM",
        ],
        leading_optional: 1,
        trailing_optional: 1,
        repeatable: false,
        comment_restarts: false,
        optional: true,
    },
    // Every spacecraft keyword is optional, so any of them may open or close the block.
    OpmKvnBlock {
        keywords: &[
            "MASS",
            "SOLAR_RAD_AREA",
            "SOLAR_RAD_COEFF",
            "DRAG_AREA",
            "DRAG_COEFF",
        ],
        leading_optional: 5,
        trailing_optional: 5,
        repeatable: false,
        comment_restarts: false,
        optional: true,
    },
    OpmKvnBlock {
        keywords: &[
            "COV_REF_FRAME",
            "CX_X",
            "CY_X",
            "CY_Y",
            "CZ_X",
            "CZ_Y",
            "CZ_Z",
            "CX_DOT_X",
            "CX_DOT_Y",
            "CX_DOT_Z",
            "CX_DOT_X_DOT",
            "CY_DOT_X",
            "CY_DOT_Y",
            "CY_DOT_Z",
            "CY_DOT_X_DOT",
            "CY_DOT_Y_DOT",
            "CZ_DOT_X",
            "CZ_DOT_Y",
            "CZ_DOT_Z",
            "CZ_DOT_X_DOT",
            "CZ_DOT_Y_DOT",
            "CZ_DOT_Z_DOT",
        ],
        leading_optional: 2,
        trailing_optional: 1,
        repeatable: false,
        comment_restarts: false,
        optional: true,
    },
    OpmKvnBlock {
        keywords: &[
            "MAN_EPOCH_IGNITION",
            "MAN_DURATION",
            "MAN_DELTA_MASS",
            "MAN_REF_FRAME",
            "MAN_DV_1",
            "MAN_DV_2",
            "MAN_DV_3",
        ],
        leading_optional: 1,
        trailing_optional: 1,
        repeatable: true,
        comment_restarts: true,
        optional: true,
    },
    OpmKvnBlock {
        keywords: &["USER_DEFINED_"],
        leading_optional: 1,
        trailing_optional: 1,
        repeatable: true,
        comment_restarts: false,
        optional: true,
    },
];

/// Ranks are spaced so each block occupies its own span; only the ordering and the grouping into
/// blocks are meaningful, never the specific numbers.
const OPM_BLOCK_STRIDE: u16 = 32;

/// Locates a keyword in the declared layout, resolving the anomaly choice and the
/// `USER_DEFINED_*` prefix to their declared slots.
fn opm_kvn_position(key: &str) -> Option<(usize, usize)> {
    for (block_index, block) in OPM_KVN_BLOCKS.iter().enumerate() {
        if let Some(offset) = block.keywords.iter().position(|candidate| {
            *candidate == key
                || (*candidate == "USER_DEFINED_" && key.starts_with("USER_DEFINED_"))
                || (*candidate == "TRUE_ANOMALY" && key == "MEAN_ANOMALY")
        }) {
            return Some((block_index, offset));
        }
    }
    None
}

fn opm_kvn_rank_derived(key: &str) -> Option<u16> {
    opm_kvn_position(key).map(|(block, offset)| block as u16 * OPM_BLOCK_STRIDE + offset as u16)
}

/// Whether `key` may open its block, i.e. everything before it in the block is optional.
fn opm_opens_block(block: &OpmKvnBlock, offset: usize) -> bool {
    offset < block.leading_optional
}

/// Whether `previous` is a rank that may immediately precede the start of block `index`: any
/// closing keyword of an earlier block, and of the block itself when `self_follow` is set.
fn opm_may_precede(index: usize, previous: u16, self_follow: bool) -> bool {
    OPM_KVN_BLOCKS
        .iter()
        .enumerate()
        .filter(|(other_index, _)| {
            if *other_index == index {
                return self_follow;
            }
            // Only reachable if every block strictly between the two may be absent.
            *other_index < index
                && OPM_KVN_BLOCKS[*other_index + 1..index]
                    .iter()
                    .all(|between| between.optional)
        })
        .any(|(other_index, other)| {
            let closing_from = other.keywords.len().saturating_sub(other.trailing_optional);
            (closing_from..other.keywords.len())
                .any(|offset| previous == other_index as u16 * OPM_BLOCK_STRIDE + offset as u16)
        })
}

fn opm_comment_starts_block_derived(previous: u16, key: &str) -> bool {
    let Some((index, offset)) = opm_kvn_position(key) else {
        return false;
    };
    let block = &OPM_KVN_BLOCKS[index];
    opm_opens_block(block, offset) && opm_may_precede(index, previous, block.comment_restarts)
}

/// Whether a non-increasing step is legal: a repeatable block restarting, or the alternative
/// spelling of a shared-rank keyword choice.
fn opm_allows_non_increasing_derived(
    previous: crate::kvn::strict::Assignment<'_>,
    current: crate::kvn::strict::Assignment<'_>,
) -> bool {
    if current.rank == previous.rank {
        return current.key != previous.key;
    }
    let (Some((current_block, current_offset)), Some((previous_block, _))) = (
        opm_kvn_position(current.key),
        opm_kvn_position(previous.key),
    ) else {
        return false;
    };
    let block = &OPM_KVN_BLOCKS[current_block];
    block.repeatable
        && current_block == previous_block
        && opm_opens_block(block, current_offset)
        && opm_may_precede(current_block, previous.rank, true)
}

fn validate_kvn_syntax(kvn: &str) -> Result<()> {
    crate::kvn::strict::validate_odm_assignments(
        kvn,
        &crate::kvn::strict::OdmAssignmentRules {
            context: "strict OPM KVN",
            message_name: "OPM",
            rank: opm_kvn_rank_derived,
            comment_starts_block: opm_comment_starts_block_derived,
            allows_non_increasing: opm_allows_non_increasing_derived,
        },
    )
}

impl Opm {
    pub(crate) fn from_kvn_with_options(
        kvn: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        let source_edition = kvn.split(['\r', '\n']).find_map(|line| {
            line.split_once('=')
                .filter(|(key, _)| key.trim() == "CCSDS_OPM_VERS")
                .map(|(_, value)| value.trim())
        });
        (|| {
            validate_input_size(kvn, options)?;
            let normalized = crate::kvn::normalize_line_endings(kvn);
            validate_kvn_syntax(&normalized)?;
            let opm = Self::from_kvn_str(&normalized)?;
            crate::traits::Validate::validate(&opm)?;
            Ok(opm)
        })()
        .map_err(|error: crate::error::CcsdsNdmError| {
            error.with_parse_context(
                crate::validation::MessageKind::Opm,
                crate::error::DiagnosticNotation::Kvn,
                kvn,
                source_edition,
            )
        })
    }

    fn validate_kvn_numbers(&self) -> Result<()> {
        fn check(field: &'static str, value: f64, path: impl Into<FieldPath>) -> Result<()> {
            if OdmFloat::is_valid(value) {
                return Ok(());
            }
            Err(crate::validation::unrepresentable_number(
                field,
                value,
                path.into().resolve(),
            ))
        }

        macro_rules! check {
            ($field:literal, $value:expr, $path:literal) => {
                check($field, $value, $path)?
            };
        }

        let data = &self.body.segment.data;
        let state = &data.state_vector;
        check!("X", state.x.value, "body.segment.data.state_vector.x");
        check!("Y", state.y.value, "body.segment.data.state_vector.y");
        check!("Z", state.z.value, "body.segment.data.state_vector.z");
        check!(
            "X_DOT",
            state.x_dot.value,
            "body.segment.data.state_vector.x_dot"
        );
        check!(
            "Y_DOT",
            state.y_dot.value,
            "body.segment.data.state_vector.y_dot"
        );
        check!(
            "Z_DOT",
            state.z_dot.value,
            "body.segment.data.state_vector.z_dot"
        );

        if let Some(elements) = &data.keplerian_elements {
            check!(
                "SEMI_MAJOR_AXIS",
                elements.semi_major_axis.value,
                "body.segment.data.keplerian_elements.semi_major_axis"
            );
            check!(
                "ECCENTRICITY",
                elements.eccentricity.value,
                "body.segment.data.keplerian_elements.eccentricity"
            );
            check!(
                "INCLINATION",
                elements.inclination.angle.value,
                "body.segment.data.keplerian_elements.inclination"
            );
            check!(
                "RA_OF_ASC_NODE",
                elements.ra_of_asc_node.value,
                "body.segment.data.keplerian_elements.ra_of_asc_node"
            );
            check!(
                "ARG_OF_PERICENTER",
                elements.arg_of_pericenter.value,
                "body.segment.data.keplerian_elements.arg_of_pericenter"
            );
            if let Some(value) = &elements.true_anomaly {
                check!(
                    "TRUE_ANOMALY",
                    value.value,
                    "body.segment.data.keplerian_elements.true_anomaly"
                );
            }
            if let Some(value) = &elements.mean_anomaly {
                check!(
                    "MEAN_ANOMALY",
                    value.value,
                    "body.segment.data.keplerian_elements.mean_anomaly"
                );
            }
            check!(
                "GM",
                elements.gm.value,
                "body.segment.data.keplerian_elements.gm"
            );
        }

        if let Some(parameters) = &data.spacecraft_parameters {
            for (field, value, path) in [
                (
                    "MASS",
                    parameters.mass.as_ref().map(|value| value.value),
                    "body.segment.data.spacecraft_parameters.mass",
                ),
                (
                    "SOLAR_RAD_AREA",
                    parameters.solar_rad_area.as_ref().map(|value| value.value),
                    "body.segment.data.spacecraft_parameters.solar_rad_area",
                ),
                (
                    "SOLAR_RAD_COEFF",
                    parameters.solar_rad_coeff.as_ref().map(|value| value.value),
                    "body.segment.data.spacecraft_parameters.solar_rad_coeff",
                ),
                (
                    "DRAG_AREA",
                    parameters.drag_area.as_ref().map(|value| value.value),
                    "body.segment.data.spacecraft_parameters.drag_area",
                ),
                (
                    "DRAG_COEFF",
                    parameters.drag_coeff.as_ref().map(|value| value.value),
                    "body.segment.data.spacecraft_parameters.drag_coeff",
                ),
            ] {
                if let Some(value) = value {
                    check(field, value, path)?;
                }
            }
        }

        if let Some(covariance) = &data.covariance_matrix {
            for (field, value, path) in covariance.kvn_numbers() {
                check(field, value, path)?;
            }
        }

        for (index, maneuver) in data.maneuver_parameters.iter().enumerate() {
            for (field, value, field_path) in [
                ("MAN_DURATION", maneuver.man_duration.value, "man_duration"),
                (
                    "MAN_DELTA_MASS",
                    maneuver.man_delta_mass.value,
                    "man_delta_mass",
                ),
                ("MAN_DV_1", maneuver.man_dv_1.value, "man_dv_1"),
                ("MAN_DV_2", maneuver.man_dv_2.value, "man_dv_2"),
                ("MAN_DV_3", maneuver.man_dv_3.value, "man_dv_3"),
            ] {
                check(field, value, FieldPath::Maneuver(index, field_path))?;
            }
        }
        Ok(())
    }

    fn validate_kvn_text(&self) -> Result<()> {
        fn invalid_text(
            field: &'static str,
            value: &str,
            path: impl Into<FieldPath>,
        ) -> Result<()> {
            if value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
                return Ok(());
            }
            Err(ValidationError::InvalidValue {
                field: field.into(),
                value: value.into(),
                expected: "printable ASCII characters and blanks".into(),
                line: None,
            }
            .at_path(path.into().resolve())
            .into())
        }

        fn pair(
            field: &'static str,
            value: &str,
            path: impl Into<FieldPath>,
            key_len: usize,
        ) -> Result<()> {
            let path = path.into();
            invalid_text(field, value, path)?;
            let line_len = key_len.max(20) + 3 + value.len();
            if line_len <= 254 {
                return Ok(());
            }
            Err(ValidationError::OutOfRange {
                name: field.into(),
                value: line_len.to_string(),
                expected: "a KVN line no longer than 254 characters".into(),
                line: None,
            }
            .at_path(path.resolve())
            .into())
        }

        fn comments(comments: &[String], path: impl Into<FieldPath>) -> Result<()> {
            let path = path.into();
            for comment in comments {
                if let Some(error) = crate::validation::kvn_comment_error(comment) {
                    return Err(error.at_path(path.resolve()).into());
                }
            }
            Ok(())
        }

        comments(&self.header.comment, "header.comment")?;
        if let Some(value) = &self.header.classification {
            pair(
                "CLASSIFICATION",
                value,
                "header.classification",
                "CLASSIFICATION".len(),
            )?;
        }
        pair(
            "ORIGINATOR",
            &self.header.originator,
            "header.originator",
            "ORIGINATOR".len(),
        )?;
        if let Some(value) = &self.header.message_id {
            pair("MESSAGE_ID", value, "header.message_id", "MESSAGE_ID".len())?;
        }

        let segment = &self.body.segment;
        comments(&segment.metadata.comment, "body.segment.metadata.comment")?;
        for (field, value, path) in [
            (
                "OBJECT_NAME",
                segment.metadata.object_name.as_str(),
                "body.segment.metadata.object_name",
            ),
            (
                "OBJECT_ID",
                segment.metadata.object_id.as_str(),
                "body.segment.metadata.object_id",
            ),
            (
                "CENTER_NAME",
                segment.metadata.center_name.as_str(),
                "body.segment.metadata.center_name",
            ),
            (
                "REF_FRAME",
                segment.metadata.ref_frame.as_str(),
                "body.segment.metadata.ref_frame",
            ),
            (
                "TIME_SYSTEM",
                segment.metadata.time_system.as_str(),
                "body.segment.metadata.time_system",
            ),
        ] {
            pair(field, value, path, field.len())?;
        }

        let data = &segment.data;
        comments(&data.comment, "body.segment.data.comment")?;
        comments(
            &data.state_vector.comment,
            "body.segment.data.state_vector.comment",
        )?;
        if let Some(elements) = &data.keplerian_elements {
            comments(
                &elements.comment,
                "body.segment.data.keplerian_elements.comment",
            )?;
        }
        if let Some(parameters) = &data.spacecraft_parameters {
            comments(
                &parameters.comment,
                "body.segment.data.spacecraft_parameters.comment",
            )?;
        }
        if let Some(covariance) = &data.covariance_matrix {
            comments(
                &covariance.comment,
                "body.segment.data.covariance_matrix.comment",
            )?;
            if let Some(value) = &covariance.cov_ref_frame {
                pair(
                    "COV_REF_FRAME",
                    value,
                    "body.segment.data.covariance_matrix.cov_ref_frame",
                    "COV_REF_FRAME".len(),
                )?;
            }
        }
        for (index, maneuver) in data.maneuver_parameters.iter().enumerate() {
            comments(&maneuver.comment, FieldPath::Maneuver(index, "comment"))?;
            pair(
                "MAN_REF_FRAME",
                &maneuver.man_ref_frame,
                FieldPath::Maneuver(index, "man_ref_frame"),
                "MAN_REF_FRAME".len(),
            )?;
        }
        if let Some(user_defined) = &data.user_defined_parameters {
            comments(
                &user_defined.comment,
                "body.segment.data.user_defined_parameters.comment",
            )?;
            for parameter in &user_defined.user_defined {
                let suffix = parameter
                    .parameter
                    .strip_prefix("USER_DEFINED_")
                    .unwrap_or(&parameter.parameter);
                invalid_text(
                    "USER_DEFINED parameter",
                    suffix,
                    "body.segment.data.user_defined_parameters.user_defined.parameter",
                )?;
                if suffix.is_empty()
                    || suffix
                        .bytes()
                        .any(|byte| byte.is_ascii_lowercase() || byte == b' ' || byte == b'=')
                {
                    return Err(ValidationError::InvalidValue {
                        field: "USER_DEFINED parameter".into(),
                        value: parameter.parameter.clone(),
                        expected: "a non-empty uppercase KVN keyword suffix without blanks or ="
                            .into(),
                        line: None,
                    }
                    .at_path("body.segment.data.user_defined_parameters.user_defined.parameter")
                    .into());
                }
                let key_len = "USER_DEFINED_".len() + suffix.len();
                let minimum_line_len = key_len.max(20) + 3;
                if minimum_line_len > 254 {
                    return Err(ValidationError::OutOfRange {
                        name: "USER_DEFINED parameter".into(),
                        value: minimum_line_len.to_string(),
                        expected: "a KVN line no longer than 254 characters".into(),
                        line: None,
                    }
                    .at_path("body.segment.data.user_defined_parameters.user_defined.parameter")
                    .into());
                }
                pair(
                    "USER_DEFINED",
                    &parameter.value,
                    "body.segment.data.user_defined_parameters.user_defined.value",
                    key_len,
                )?;
            }
        }
        Ok(())
    }
}

impl ToKvn for Opm {
    fn validate_kvn(&self) -> Result<()> {
        self.validate_kvn_text()?;
        self.validate_kvn_numbers()
    }

    fn write_kvn(&self, writer: &mut KvnWriter) {
        // 1. Header
        writer.write_pair("CCSDS_OPM_VERS", &self.version);
        self.header.write_kvn(writer);

        // 2. Body
        self.body.write_kvn(writer);
    }
}

impl ToKvn for OpmBody {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.segment.write_kvn(writer);
    }
}

impl ToKvn for OpmSegment {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        self.metadata.write_kvn(writer);
        self.data.write_kvn(writer);
    }
}

impl ToKvn for OpmMetadata {
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
    }
}

impl ToKvn for OpmData {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        self.state_vector.write_kvn(writer);

        // Keplerian Elements
        if let Some(ke) = &self.keplerian_elements {
            ke.write_kvn(writer);
        }

        // Spacecraft Parameters
        if let Some(sp) = &self.spacecraft_parameters {
            writer.write_comments(&sp.comment);
            if let Some(v) = &sp.mass {
                writer.write_odm_float_measure("MASS", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_area {
                writer.write_odm_float_measure("SOLAR_RAD_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.solar_rad_coeff {
                writer.write_odm_float_pair("SOLAR_RAD_COEFF", v.value);
            }
            if let Some(v) = &sp.drag_area {
                writer.write_odm_float_measure("DRAG_AREA", &v.to_unit_value());
            }
            if let Some(v) = &sp.drag_coeff {
                writer.write_odm_float_pair("DRAG_COEFF", v.value);
            }
        }

        // Covariance
        if let Some(cov) = &self.covariance_matrix {
            cov.write_kvn(writer);
        }

        // Maneuvers
        for man in &self.maneuver_parameters {
            man.write_kvn(writer);
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

impl ToKvn for KeplerianElements {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_odm_float_measure("SEMI_MAJOR_AXIS", &self.semi_major_axis);
        writer.write_odm_float_pair("ECCENTRICITY", self.eccentricity.value);
        writer.write_odm_float_measure("INCLINATION", &self.inclination.to_unit_value());
        writer.write_odm_float_measure("RA_OF_ASC_NODE", &self.ra_of_asc_node.to_unit_value());
        writer
            .write_odm_float_measure("ARG_OF_PERICENTER", &self.arg_of_pericenter.to_unit_value());
        if let Some(v) = &self.true_anomaly {
            writer.write_odm_float_measure("TRUE_ANOMALY", &v.to_unit_value());
        }
        if let Some(v) = &self.mean_anomaly {
            writer.write_odm_float_measure("MEAN_ANOMALY", &v.to_unit_value());
        }
        // ODM 7.7.1 requires KVN units to match the keyword table spelling exactly, so the
        // uppercase spelling the XML schema also permits is canonicalized on output.
        let gm_units = self.gm.units.as_ref().map(|_| GmUnits::Km3PerS2);
        writer.write_odm_float_measure("GM", &UnitValue::new(self.gm.value, gm_units));
    }
}

impl ToKvn for ManeuverParameters {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        writer.write_comments(&self.comment);
        writer.write_pair("MAN_EPOCH_IGNITION", self.man_epoch_ignition);
        writer.write_odm_float_measure("MAN_DURATION", &self.man_duration.to_unit_value());
        writer.write_odm_float_measure(
            "MAN_DELTA_MASS",
            &UnitValue::new(self.man_delta_mass.value, self.man_delta_mass.units.clone()),
        );
        writer.write_pair("MAN_REF_FRAME", &self.man_ref_frame);
        writer.write_odm_float_measure("MAN_DV_1", &self.man_dv_1);
        writer.write_odm_float_measure("MAN_DV_2", &self.man_dv_2);
        writer.write_odm_float_measure("MAN_DV_3", &self.man_dv_3);
    }
}

#[cfg(test)]
mod kvn_layout {
    use super::*;

    fn rank(key: &str) -> u16 {
        opm_kvn_rank_derived(key).expect("keyword is in the declared layout")
    }

    fn assignment(key: &'static str) -> crate::kvn::strict::Assignment<'static> {
        crate::kvn::strict::Assignment {
            rank: rank(key),
            key,
        }
    }

    /// The keyword sequence of ODM 502.0-B-3 tables 3-1 through 3-6, in normative order.
    ///
    /// Checking against the book rather than against a frozen copy of the previous implementation
    /// means a future intentional change is measured against the standard, which is the thing that
    /// actually governs.
    const NORMATIVE_ORDER: &[&str] = &[
        "CCSDS_OPM_VERS",
        "CLASSIFICATION",
        "CREATION_DATE",
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_NAME",
        "OBJECT_ID",
        "CENTER_NAME",
        "REF_FRAME",
        "REF_FRAME_EPOCH",
        "TIME_SYSTEM",
        "EPOCH",
        "X",
        "Y",
        "Z",
        "X_DOT",
        "Y_DOT",
        "Z_DOT",
        "SEMI_MAJOR_AXIS",
        "ECCENTRICITY",
        "INCLINATION",
        "RA_OF_ASC_NODE",
        "ARG_OF_PERICENTER",
        "TRUE_ANOMALY",
        "GM",
        "MASS",
        "SOLAR_RAD_AREA",
        "SOLAR_RAD_COEFF",
        "DRAG_AREA",
        "DRAG_COEFF",
        "COV_REF_FRAME",
        "CX_X",
        "CZ_DOT_Z_DOT",
        "MAN_EPOCH_IGNITION",
        "MAN_DV_3",
        "USER_DEFINED_FOO",
    ];

    #[test]
    fn declared_layout_follows_the_normative_keyword_order() {
        for pair in NORMATIVE_ORDER.windows(2) {
            assert!(
                rank(pair[0]) < rank(pair[1]),
                "{} must precede {}",
                pair[0],
                pair[1]
            );
        }
        assert!(opm_kvn_rank_derived("NOT_A_KEYWORD").is_none());
    }

    /// ODM 502.0-B-3 table 3-2: either anomaly spelling may fill the slot, so they share a rank
    /// and only the alternative may follow. Both present is a choice violation reported by
    /// `KeplerianElements::validate`, not an ordering error.
    #[test]
    fn the_anomaly_choice_shares_one_slot() {
        assert_eq!(rank("TRUE_ANOMALY"), rank("MEAN_ANOMALY"));
        assert!(opm_allows_non_increasing_derived(
            assignment("TRUE_ANOMALY"),
            assignment("MEAN_ANOMALY")
        ));
        assert!(!opm_allows_non_increasing_derived(
            assignment("TRUE_ANOMALY"),
            assignment("TRUE_ANOMALY")
        ));
    }

    /// The version keyword anchors the document, so the header content following it is its own
    /// comment group, opened by `CLASSIFICATION` or — since that is optional — `CREATION_DATE`.
    ///
    /// Modelling the header as a single block got this wrong while the layout was being derived.
    #[test]
    fn header_content_opens_a_comment_group_after_the_version_keyword() {
        for key in ["CLASSIFICATION", "CREATION_DATE"] {
            assert!(opm_comment_starts_block_derived(
                rank("CCSDS_OPM_VERS"),
                key
            ));
        }
        assert!(!opm_comment_starts_block_derived(
            rank("CCSDS_OPM_VERS"),
            "ORIGINATOR"
        ));
    }

    /// A mandatory block between two others blocks the path: metadata cannot immediately follow
    /// the version keyword because the header content is required.
    ///
    /// Omitting this fact made the derived rule more permissive than the standard allows.
    #[test]
    fn a_mandatory_block_between_two_others_blocks_the_path() {
        assert!(!opm_comment_starts_block_derived(
            rank("CCSDS_OPM_VERS"),
            "OBJECT_NAME"
        ));
        for closer in ["ORIGINATOR", "MESSAGE_ID"] {
            assert!(opm_comment_starts_block_derived(
                rank(closer),
                "OBJECT_NAME"
            ));
        }
    }

    /// Repetition and comment restart are different facts. A second maneuver block is a new
    /// logical block, so a comment may open it; `USER_DEFINED_*` repeats inside one block, so a
    /// comment there does not start a new one.
    ///
    /// Conflating the two was the second modelling error the derivation uncovered.
    #[test]
    fn repetition_and_comment_restart_are_distinct() {
        assert!(opm_comment_starts_block_derived(
            rank("MAN_DV_3"),
            "MAN_EPOCH_IGNITION"
        ));
        assert!(!opm_comment_starts_block_derived(
            rank("USER_DEFINED_FOO"),
            "USER_DEFINED_FOO"
        ));
        assert!(opm_allows_non_increasing_derived(
            assignment("MAN_DV_3"),
            assignment("MAN_EPOCH_IGNITION")
        ));
        assert!(opm_allows_non_increasing_derived(
            assignment("USER_DEFINED_FOO"),
            assignment("USER_DEFINED_BAR")
        ));
    }

    /// Only a repeatable block may follow itself. Without this, a document could restate a block
    /// that the standard allows exactly once.
    #[test]
    fn a_non_repeatable_block_may_not_restart() {
        assert!(!opm_allows_non_increasing_derived(
            assignment("Z_DOT"),
            assignment("EPOCH")
        ));
        assert!(!opm_allows_non_increasing_derived(
            assignment("DRAG_COEFF"),
            assignment("MASS")
        ));
        assert!(!opm_allows_non_increasing_derived(
            assignment("CZ_DOT_Z_DOT"),
            assignment("COV_REF_FRAME")
        ));
        // A backwards step across blocks is never legal either.
        assert!(!opm_allows_non_increasing_derived(
            assignment("MAN_DV_3"),
            assignment("MASS")
        ));
    }

    /// Optional blocks may be skipped, so a block start may follow the closing keyword of any
    /// earlier block with only optional blocks between them.
    #[test]
    fn optional_blocks_may_be_skipped() {
        for closer in ["Z_DOT", "GM", "DRAG_COEFF", "CZ_DOT_Z_DOT", "MAN_DV_3"] {
            assert!(
                opm_comment_starts_block_derived(rank(closer), "USER_DEFINED_FOO"),
                "USER_DEFINED_* should be reachable after {closer}"
            );
        }
        // Every spacecraft keyword is optional, so any of them may close that block.
        for closer in ["MASS", "SOLAR_RAD_AREA", "DRAG_COEFF"] {
            assert!(opm_comment_starts_block_derived(
                rank(closer),
                "COV_REF_FRAME"
            ));
        }
        // The state vector is mandatory, so nothing before it closes into the keplerian block.
        assert!(!opm_comment_starts_block_derived(
            rank("TIME_SYSTEM"),
            "SEMI_MAJOR_AXIS"
        ));
        assert!(opm_comment_starts_block_derived(
            rank("Z_DOT"),
            "SEMI_MAJOR_AXIS"
        ));
    }
}
