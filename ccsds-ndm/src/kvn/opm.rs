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

use crate::kvn::parser::*;
use crate::messages::opm::{
    KeplerianElements, ManeuverParameters, Opm, OpmBody, OpmData, OpmMetadata, OpmSegment,
};
use crate::parse_block;
use winnow::combinator::peek;
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
    if value != "3.0" && value != "2.0" {
        return Err(cut_err(input, "3.0 or 2.0"));
    }
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
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_epoch,
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

    if true_anomaly.is_some() && mean_anomaly.is_some() {
        return Err(cut_err(
            input,
            "Cannot have both TRUE_ANOMALY and MEAN_ANOMALY",
        ));
    }

    if true_anomaly.is_none() && mean_anomaly.is_none() {
        return Err(cut_err(
            input,
            "Either TRUE_ANOMALY or MEAN_ANOMALY must be present in Keplerian Elements",
        ));
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
    let mut comment = Vec::new();
    let mut man_epoch_ignition = None;
    let mut man_duration = None;
    let mut man_delta_mass = None;
    let mut man_ref_frame = None;
    let mut man_dv_1 = None;
    let mut man_dv_2 = None;
    let mut man_dv_3 = None;

    parse_block!(input, comment, {
        "MAN_EPOCH_IGNITION" => man_epoch_ignition: kv_epoch,
        "MAN_DURATION" => man_duration: kv_from_kvn,
        "MAN_DELTA_MASS" => man_delta_mass: kv_from_kvn,
        "MAN_REF_FRAME" => man_ref_frame: kv_string,
        "MAN_DV_1" => man_dv_1: kv_from_kvn,
        "MAN_DV_2" => man_dv_2: kv_from_kvn,
        "MAN_DV_3" => man_dv_3: kv_from_kvn,
    }, |i: &mut &str| man_epoch_ignition.is_some() && peek(key_token).parse_next(i).map(|k| k == "MAN_EPOCH_IGNITION").unwrap_or(false));

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
            Ok(None) => break,
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

    if !maneuver_parameters.is_empty() {
        let has_mass = spacecraft_parameters
            .as_ref()
            .and_then(|sp| sp.mass.as_ref())
            .is_some();
        if !has_mass {
            return Err(cut_err(
                input,
                "MASS must be provided if maneuvers are specified",
            ));
        }
    }

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

        // Keplerian: both TRUE_ANOMALY and MEAN_ANOMALY
        let mut kvn_kep_both = "SEMI_MAJOR_AXIS = 7000.0\nECCENTRICITY = 0.0\nINCLINATION = 0.0\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nTRUE_ANOMALY = 0.0\nMEAN_ANOMALY = 0.0\nGM = 398600.44\n";
        assert!(keplerian_elements.parse_next(&mut kvn_kep_both).is_err());

        // Keplerian: neither TRUE_ANOMALY nor MEAN_ANOMALY
        let mut kvn_kep_none = "SEMI_MAJOR_AXIS = 7000.0\nECCENTRICITY = 0.0\nINCLINATION = 0.0\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nGM = 398600.44\n";
        assert!(keplerian_elements.parse_next(&mut kvn_kep_none).is_err());

        // Maneuver without MASS
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
        assert!(Opm::from_kvn_str(kvn_man_no_mass).is_err());

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
