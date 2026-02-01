// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OMM (Orbit Mean-Elements Message).
//!
//! This module implements KVN parsing for OMM using winnow parser combinators.

use crate::kvn::parser::*;
use crate::messages::omm::{
    MeanElements, Omm, OmmBody, OmmData, OmmMetadata, OmmSegment, TleParameters,
};
use crate::parse_block;
use winnow::prelude::*;

//----------------------------------------------------------------------
// OMM Version Parser
//----------------------------------------------------------------------

pub fn omm_version(input: &mut &str) -> KvnResult<String> {
    ws.parse_next(input)?;
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OMM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OMM Metadata Parser
//----------------------------------------------------------------------

pub fn omm_metadata(input: &mut &str) -> KvnResult<OmmMetadata> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;
    let mut mean_element_theory = None;

    parse_block!(input, comment, {
        "OBJECT_NAME" => object_name: kv_string,
        "OBJECT_ID" => object_id: kv_string,
        "CENTER_NAME" => center_name: kv_string,
        "REF_FRAME" => ref_frame: kv_string,
        "REF_FRAME_EPOCH" => ref_frame_epoch: kv_epoch,
        "TIME_SYSTEM" => time_system: kv_string,
        "MEAN_ELEMENT_THEORY" => mean_element_theory: kv_string,
    }, |_| false);

    Ok(OmmMetadata {
        comment,
        object_name: object_name
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "OBJECT_NAME"))?,
        object_id: object_id
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "OBJECT_ID"))?,
        center_name: center_name
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "CENTER_NAME"))?,
        ref_frame: ref_frame
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "REF_FRAME"))?,
        ref_frame_epoch,
        time_system: time_system
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "TIME_SYSTEM"))?,
        mean_element_theory: mean_element_theory
            .ok_or_else(|| missing_field_err(input, "OMM Metadata", "MEAN_ELEMENT_THEORY"))?,
    })
}

//----------------------------------------------------------------------
// Mean Elements Parser
//----------------------------------------------------------------------

pub fn mean_elements(input: &mut &str) -> KvnResult<(Vec<String>, MeanElements)> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut epoch = None;
    let mut semi_major_axis = None;
    let mut mean_motion = None;
    let mut eccentricity = None;
    let mut inclination = None;
    let mut ra_of_asc_node = None;
    let mut arg_of_pericenter = None;
    let mut mean_anomaly = None;
    let mut gm = None;

    parse_block!(input, comment, {
        "EPOCH" => epoch: kv_epoch,
        "SEMI_MAJOR_AXIS" => semi_major_axis: kv_from_kvn,
        "MEAN_MOTION" => mean_motion: kv_from_kvn,
        "ECCENTRICITY" => eccentricity: kv_from_kvn,
        "INCLINATION" => inclination: kv_from_kvn,
        "RA_OF_ASC_NODE" => ra_of_asc_node: kv_from_kvn,
        "ARG_OF_PERICENTER" => arg_of_pericenter: kv_from_kvn,
        "MEAN_ANOMALY" => mean_anomaly: kv_from_kvn,
        "GM" => gm: kv_from_kvn,
    }, |_| false);

    if semi_major_axis.is_some() && mean_motion.is_some() {
        return Err(cut_err(
            input,
            "Cannot have both SEMI_MAJOR_AXIS and MEAN_MOTION",
        ));
    }

    if semi_major_axis.is_none() && mean_motion.is_none() {
        return Err(cut_err(
            input,
            "Either SEMI_MAJOR_AXIS or MEAN_MOTION must be present",
        ));
    }

    Ok((
        comment,
        MeanElements {
            comment: Vec::new(), // comment is returned as part of the tuple for OmmData
            epoch: epoch.ok_or_else(|| missing_field_err(input, "Mean Elements", "EPOCH"))?,
            semi_major_axis,
            mean_motion,
            eccentricity: eccentricity
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "ECCENTRICITY"))?,
            inclination: inclination
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "INCLINATION"))?,
            ra_of_asc_node: ra_of_asc_node
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "RA_OF_ASC_NODE"))?,
            arg_of_pericenter: arg_of_pericenter
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "ARG_OF_PERICENTER"))?,
            mean_anomaly: mean_anomaly
                .ok_or_else(|| missing_field_err(input, "Mean Elements", "MEAN_ANOMALY"))?,
            gm,
        },
    ))
}

//----------------------------------------------------------------------
// TLE Parameters Parser
//----------------------------------------------------------------------

pub fn tle_parameters(input: &mut &str) -> KvnResult<Option<TleParameters>> {
    ws.parse_next(input)?;
    let mut comment = Vec::new();
    let mut ephemeris_type = None;
    let mut classification_type = None;
    let mut norad_cat_id = None;
    let mut element_set_no = None;
    let mut rev_at_epoch = None;
    let mut bstar = None;
    let mut bterm = None;
    let mut mean_motion_dot = None;
    let mut mean_motion_ddot = None;
    let mut agom = None;

    parse_block!(input, comment, {
        "EPHEMERIS_TYPE" => ephemeris_type: kv_i32,
        "CLASSIFICATION_TYPE" => classification_type: kv_string,
        "NORAD_CAT_ID" => norad_cat_id: kv_u32,
        "ELEMENT_SET_NO" => val: kv_u32 => { element_set_no = Some(val.into()); },
        "REV_AT_EPOCH" => rev_at_epoch: kv_u32,
        "BSTAR" => bstar: kv_from_kvn,
        "BTERM" => bterm: kv_from_kvn,
        "MEAN_MOTION_DOT" => mean_motion_dot: kv_from_kvn,
        "MEAN_MOTION_DDOT" => mean_motion_ddot: kv_from_kvn,
        "AGOM" => agom: kv_from_kvn,
    }, |_| false);

    if ephemeris_type.is_none()
        && classification_type.is_none()
        && norad_cat_id.is_none()
        && element_set_no.is_none()
        && rev_at_epoch.is_none()
        && bstar.is_none()
        && bterm.is_none()
        && mean_motion_dot.is_none()
        && mean_motion_ddot.is_none()
        && agom.is_none()
    {
        return Ok(None);
    }

    if bstar.is_some() && bterm.is_some() {
        return Err(cut_err(input, "Cannot have both BSTAR and BTERM"));
    }

    if mean_motion_ddot.is_some() && agom.is_some() {
        return Err(cut_err(input, "Cannot have both MEAN_MOTION_DDOT and AGOM"));
    }

    Ok(Some(TleParameters {
        comment,
        ephemeris_type,
        classification_type,
        norad_cat_id,
        element_set_no,
        rev_at_epoch,
        bstar,
        bterm,
        mean_motion_dot: mean_motion_dot
            .ok_or_else(|| missing_field_err(input, "TLE Parameters", "MEAN_MOTION_DOT"))?,
        mean_motion_ddot,
        agom,
    }))
}

//----------------------------------------------------------------------
// OMM Data Parser
//----------------------------------------------------------------------

pub fn omm_data(input: &mut &str) -> KvnResult<OmmData> {
    let (me_comment, mean_elements) = mean_elements.parse_next(input)?;

    // Spacecraft parameters
    let spacecraft_parameters = spacecraft_parameters.parse_next(input)?;

    // TLE parameters
    let tle_parameters = tle_parameters.parse_next(input)?;

    // Covariance matrix
    let covariance_matrix = covariance_matrix.parse_next(input)?;

    // User defined
    let user_defined_parameters = user_defined_parameters.parse_next(input)?;

    Ok(OmmData {
        comment: me_comment,
        mean_elements,
        spacecraft_parameters,
        tle_parameters,
        covariance_matrix,
        user_defined_parameters,
    })
}

//----------------------------------------------------------------------
// Complete OMM Parser
//----------------------------------------------------------------------

pub fn parse_omm(input: &mut &str) -> KvnResult<Omm> {
    let version = omm_version.parse_next(input)?;
    let header = odm_header.parse_next(input)?;
    let metadata = omm_metadata.parse_next(input)?;
    let data = omm_data.parse_next(input)?;

    Ok(Omm {
        header,
        body: OmmBody {
            segment: OmmSegment { metadata, data },
        },
        id: Some("CCSDS_OMM_VERS".to_string()),
        version,
    })
}

impl ParseKvn for Omm {
    fn parse_kvn(input: &mut &str) -> KvnResult<Self> {
        parse_omm.parse_next(input)
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Ndm;

    const MINIMAL_OMM: &str = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
BSTAR = 0.0001 [1/ER]
"#;

    #[test]
    fn test_parse_minimal_omm() {
        let result = Omm::from_kvn_str(MINIMAL_OMM);
        assert!(
            result.is_ok(),
            "Failed to parse minimal OMM: {:?}",
            result.err()
        );

        let omm = result.unwrap();
        assert_eq!(omm.version, "3.0");
        assert_eq!(omm.header.originator, "JAXA");
        assert_eq!(omm.body.segment.metadata.object_name, "GOES 9");
    }

    #[test]
    fn test_parse_full_omm() {
        let full_omm = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-06-28T11:59:28
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
SEMI_MAJOR_AXIS = 42164.0 [km]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
GM = 398600.4415 [km**3/s**2]
MASS = 1000 [kg]
SOLAR_RAD_AREA = 10 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 5 [m**2]
DRAG_COEFF = 2.2
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 1234
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
BSTAR = 0.0001 [1/ER]
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
"#;
        let result = Omm::from_kvn_str(full_omm);
        assert!(
            result.is_ok(),
            "Failed to parse full OMM: {:?}",
            result.err()
        );
        let omm = result.unwrap();
        assert!(omm.body.segment.metadata.ref_frame_epoch.is_some());
        assert!(omm.body.segment.data.spacecraft_parameters.is_some());
        assert!(omm.body.segment.data.tle_parameters.is_some());
        assert!(omm.body.segment.data.covariance_matrix.is_some());
        assert!(omm.body.segment.data.user_defined_parameters.is_some());
    }

    #[test]
    fn test_omm_tle_no_cov() {
        let tle_omm = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2022-11-06T09:23:57
ORIGINATOR = JAXA
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2000-06-28T11:59:28.000000
MEAN_MOTION = 1.00273272 [rev/day]
ECCENTRICITY = 0.00050130
INCLINATION = 3.053900 [deg]
RA_OF_ASC_NODE = 81.793900 [deg]
ARG_OF_PERICENTER = 249.236300 [deg]
MEAN_ANOMALY = 150.160200 [deg]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
BSTAR = 0.0001 [1/ER]
AGOM = 0.0001 [m**2/kg]
"#;
        let result = Omm::from_kvn_str(tle_omm);
        assert!(
            result.is_ok(),
            "Failed to parse TLE OMM: {:?}",
            result.err()
        );
        let omm = result.unwrap();
        assert!(omm.body.segment.data.tle_parameters.is_some());
        assert!(omm
            .body
            .segment
            .data
            .tle_parameters
            .as_ref()
            .unwrap()
            .agom
            .is_some());
    }

    #[test]
    fn test_omm_errors() {
        // Unknown metadata key
        let mut input = "OBJECT_NAME = GOES 9\nUNKNOWN_KEY = VAL\n";
        assert!(omm_metadata.parse_next(&mut input).is_err());

        // Exhaustive missing mandatory fields in metadata
        let mandatory_meta = [
            "OBJECT_NAME = GOES 9\n",
            "OBJECT_ID = 1\n",
            "CENTER_NAME = EARTH\n",
            "REF_FRAME = TEME\n",
            "TIME_SYSTEM = UTC\n",
            "MEAN_ELEMENT_THEORY = SGP4\n",
        ];
        for i in 0..mandatory_meta.len() {
            let mut input_str = String::new();
            for (j, item) in mandatory_meta.iter().enumerate() {
                if i != j {
                    input_str.push_str(item);
                }
            }
            let mut input = input_str.as_str();
            assert!(
                omm_metadata.parse_next(&mut input).is_err(),
                "Should fail without {}",
                mandatory_meta[i]
            );
        }

        // Both SEMI_MAJOR_AXIS and MEAN_MOTION
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nMEAN_MOTION = 1.0\nECCENTRICITY = 0.1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // Neither SEMI_MAJOR_AXIS nor MEAN_MOTION
        let mut input = "EPOCH = 2000-06-28T11:59:28\nECCENTRICITY = 0.1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // Negative eccentricity
        let mut input =
            "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = -0.1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // Exhaustive missing mandatory fields in mean elements
        let mandatory_me = [
            "EPOCH = 2000-06-28T11:59:28\n",
            "ECCENTRICITY = 0.1\n",
            "INCLINATION = 0\n",
            "RA_OF_ASC_NODE = 0\n",
            "ARG_OF_PERICENTER = 0\n",
            "MEAN_ANOMALY = 0\n",
        ];
        for i in 0..mandatory_me.len() {
            let mut input_str = String::from("SEMI_MAJOR_AXIS = 42164\n");
            for (j, item) in mandatory_me.iter().enumerate() {
                if i != j {
                    input_str.push_str(item);
                }
            }
            let mut input = input_str.as_str();
            assert!(
                mean_elements.parse_next(&mut input).is_err(),
                "Should fail without {}",
                mandatory_me[i]
            );
        }

        // TLE: both BSTAR and BTERM
        let mut input =
            "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nBSTAR = 0.0001\nBTERM = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: neither BSTAR nor BTERM (Now allowed in parser, but this input has both DDOT and AGOM)
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_err()); // Error due to DDOT + AGOM

        // TLE: independent checks for format

        // TLE: missing MEAN_MOTION_DOT (Mandatory in parser now)
        let mut input = "BSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: both MEAN_MOTION_DDOT and AGOM (Still Error - mutex)
        let mut input =
            "MEAN_MOTION_DOT = 0.000001\nBSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: neither MEAN_MOTION_DDOT nor AGOM (Allowed in parser)
        let mut input = "MEAN_MOTION_DOT = 0.000001\nBSTAR = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        // TLE: invalid ELEMENT_SET_NO
        // Now allowed in parser (no range check)
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nBSTAR = 0.0001\nELEMENT_SET_NO = 10000\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        // Invalid units for coverage (Existing tests...)
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nMEAN_MOTION = 1.0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input =
            "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = INVALID\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\nGM = 398600 [INVALID]\n";
        assert!(mean_elements.parse_next(&mut input).is_err());
        let mut input = "EPOCH = 2000-06-28T11:59:28\nSEMI_MAJOR_AXIS = 42164\nECCENTRICITY = 0.1\nINCLINATION = 0\nRA_OF_ASC_NODE = 0\nARG_OF_PERICENTER = 0\nMEAN_ANOMALY = 0\nGM = -1\n";
        assert!(mean_elements.parse_next(&mut input).is_err());

        // TLE invalid formats
        let mut input = "EPHEMERIS_TYPE = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "NORAD_CAT_ID = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "ELEMENT_SET_NO = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "REV_AT_EPOCH = INVALID\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "BSTAR = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "BTERM = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "MEAN_MOTION_DOT = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "MEAN_MOTION_DDOT = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());
        let mut input = "AGOM = 0 [INVALID]\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // Extra error coverage
        let mut input = "REV_AT_EPOCH = 1\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        let mut input = "MEAN_MOTION_DOT = 0\nBSTAR = 0\nREV_AT_EPOCH = 1\nMEAN_MOTION_DDOT = 0\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        let mut input = "OBJECT_NAME = GOES 9\nREF_FRAME_EPOCH = INVALID\n";
        assert!(omm_metadata.parse_next(&mut input).is_err());
    }

    #[test]
    fn test_omm_validation() {
        // Construct a parsed OMM-like structure or just parse minimal incomplete one and validate
        // SGP4 Theory requires BSTAR
        let kvn_sgp4_missing_bstar = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
COMMENT No TLE Parameters at all!
"#;
        let res = Omm::from_kvn(kvn_sgp4_missing_bstar);
        assert!(res.is_err());
        let err = res.err().unwrap();
        if !err.is_validation_error() {
            panic!("Expected validation error, got: {:?}", err);
        }
        assert!(err.is_validation_error());

        // SGP4 with params but missing BSTAR
        let kvn_sgp4_missing_bstar_field = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
COMMENT TLE Params present but partial
MEAN_MOTION_DOT = 0.001 [rev/day**2]
"#;
        let res = Omm::from_kvn(kvn_sgp4_missing_bstar_field);
        assert!(res.is_err());
        // Should error about missing BSTAR
        assert!(format!("{}", res.err().unwrap()).contains("BSTAR"));

        // Valid SGP4
        let kvn_sgp4_valid = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001
MEAN_MOTION_DOT = 0.0001
"#;
        assert!(Omm::from_kvn(kvn_sgp4_valid).is_ok());
    }

    // =========================================================================
    // Migrated Tests from messages/omm.rs
    // =========================================================================

    #[test]
    fn parse_omm_with_covariance_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
CX_X = 1.0 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 0.01 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 0.01 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 0.01 [km**2/s**2]
"#;
        let omm = Omm::from_kvn(kvn).expect("OMM Covariance parse failed");
        assert!(omm.body.segment.data.covariance_matrix.is_some());
    }

    #[test]
    fn test_mean_elements_choice_semi_major_axis_only_moved() {
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
        let omm = Omm::from_kvn(kvn).expect("Should parse with SEMI_MAJOR_AXIS");
        assert!(omm
            .body
            .segment
            .data
            .mean_elements
            .semi_major_axis
            .is_some());
    }

    #[test]
    fn test_mean_elements_choice_mean_motion_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION");
        assert!(omm.body.segment.data.mean_elements.mean_motion.is_some());
    }

    #[test]
    fn test_tle_choice_bstar_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BSTAR");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bstar.is_some());
        assert!(tle.bterm.is_none());
    }

    #[test]
    fn test_tle_choice_bterm_only_moved() {
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
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with BTERM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.bterm.is_some());
        assert!(tle.bstar.is_none());
    }

    #[test]
    fn test_tle_choice_mean_motion_ddot_only_moved() {
        let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION_DDOT");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.mean_motion_ddot.is_some());
        assert!(tle.agom.is_none());
    }

    #[test]
    fn test_tle_choice_agom_only_moved() {
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
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
        let omm = Omm::from_kvn(kvn).expect("Should parse with AGOM");
        let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
        assert!(tle.agom.is_some());
        assert!(tle.mean_motion_ddot.is_none());
    }
}
