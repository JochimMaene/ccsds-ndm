// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow parsers for OMM (Orbit Mean-Elements Message).
//!
//! This module implements KVN parsing for OMM using winnow parser combinators.

use crate::kvn::parser::*;
use crate::messages::omm::{
    BStar, MeanElements, MeanMotion, MeanMotionDDot, MeanMotionDot, Omm, OmmBody, OmmData,
    OmmMetadata, OmmSegment, TleParameters,
};
use crate::types::*;
use std::str::FromStr;
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Helper: Check if key belongs to OMM Data section
//----------------------------------------------------------------------

fn is_omm_data_key(key: &str) -> bool {
    matches!(
        key,
        "EPOCH"
            | "SEMI_MAJOR_AXIS"
            | "MEAN_MOTION"
            | "ECCENTRICITY"
            | "INCLINATION"
            | "RA_OF_ASC_NODE"
            | "ARG_OF_PERICENTER"
            | "MEAN_ANOMALY"
            | "GM"
            | "MASS"
            | "SOLAR_RAD_AREA"
            | "SOLAR_RAD_COEFF"
            | "DRAG_AREA"
            | "DRAG_COEFF"
            | "EPHEMERIS_TYPE"
            | "CLASSIFICATION_TYPE"
            | "NORAD_CAT_ID"
            | "ELEMENT_SET_NO"
            | "REV_AT_EPOCH"
            | "BSTAR"
            | "BTERM"
            | "MEAN_MOTION_DOT"
            | "MEAN_MOTION_DDOT"
            | "AGOM"
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
    ) || key.starts_with("USER_DEFINED_")
}

//----------------------------------------------------------------------
// OMM Version Parser
//----------------------------------------------------------------------

pub fn omm_version(input: &mut &str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OMM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OMM Metadata Parser
//----------------------------------------------------------------------

pub fn omm_metadata(input: &mut &str) -> ModalResult<OmmMetadata> {
    let mut comment = Vec::new();
    let mut object_name = None;
    let mut object_id = None;
    let mut center_name = None;
    let mut ref_frame = None;
    let mut ref_frame_epoch = None;
    let mut time_system = None;
    let mut mean_element_theory = None;

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_omm_data_key(key) => break,
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
                    "MEAN_ELEMENT_THEORY" => mean_element_theory = Some(v.to_string()),
                    _ => {
                        return Err(ErrMode::Cut(ContextError::new().add_context(
                            input,
                            &input.checkpoint(),
                            StrContext::Label("Unknown OMM Metadata key"),
                        )));
                    }
                }
            }
            None => break,
        }
    }

    Ok(OmmMetadata {
        comment,
        object_name: object_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT_NAME")),
            ))
        })?,
        object_id: object_id.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("OBJECT_ID")),
            ))
        })?,
        center_name: center_name.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("CENTER_NAME")),
            ))
        })?,
        ref_frame: ref_frame.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("REF_FRAME")),
            ))
        })?,
        ref_frame_epoch,
        time_system: time_system.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("TIME_SYSTEM")),
            ))
        })?,
        mean_element_theory: mean_element_theory.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MEAN_ELEMENT_THEORY")),
            ))
        })?,
    })
}

//----------------------------------------------------------------------
// Mean Elements Parser
//----------------------------------------------------------------------

pub fn mean_elements(input: &mut &str) -> ModalResult<(Vec<String>, MeanElements)> {
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

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(
                _k @ ("EPOCH" | "SEMI_MAJOR_AXIS" | "MEAN_MOTION" | "ECCENTRICITY" | "INCLINATION"
                | "RA_OF_ASC_NODE" | "ARG_OF_PERICENTER" | "MEAN_ANOMALY" | "GM"),
            ) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "EPOCH" => {
                        epoch = Some(Epoch::from_str(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "SEMI_MAJOR_AXIS" => {
                        semi_major_axis = Some(Distance::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "MEAN_MOTION" => {
                        mean_motion = Some(MeanMotion::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "ECCENTRICITY" => {
                        eccentricity = Some(parse_f64(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "INCLINATION" => {
                        inclination = Some(Inclination::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "RA_OF_ASC_NODE" => {
                        ra_of_asc_node = Some(Angle::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "ARG_OF_PERICENTER" => {
                        arg_of_pericenter = Some(Angle::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "MEAN_ANOMALY" => {
                        mean_anomaly = Some(Angle::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    "GM" => {
                        let uv = UnitValue::<f64, GmUnits>::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?;
                        gm = Some(Gm::new(uv.value, uv.units).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?);
                    }
                    _ => unreachable!(),
                }
            }
            _ => break,
        }
    }

    if semi_major_axis.is_some() && mean_motion.is_some() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Cannot have both SEMI_MAJOR_AXIS and MEAN_MOTION"),
        )));
    }
    if semi_major_axis.is_none() && mean_motion.is_none() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Either SEMI_MAJOR_AXIS or MEAN_MOTION must be present"),
        )));
    }

    let ecc = eccentricity.ok_or_else(|| {
        ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("ECCENTRICITY")),
        ))
    })?;
    if ecc < 0.0 {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("ECCENTRICITY must be >= 0"),
        )));
    }

    let me = MeanElements {
        comment: Vec::new(),
        epoch: epoch.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("EPOCH")),
            ))
        })?,
        semi_major_axis,
        mean_motion,
        eccentricity: ecc,
        inclination: inclination.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("INCLINATION")),
            ))
        })?,
        ra_of_asc_node: ra_of_asc_node.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("RA_OF_ASC_NODE")),
            ))
        })?,
        arg_of_pericenter: arg_of_pericenter.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("ARG_OF_PERICENTER")),
            ))
        })?,
        mean_anomaly: mean_anomaly.ok_or_else(|| {
            ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description("MEAN_ANOMALY")),
            ))
        })?,
        gm,
    };

    Ok((comment, me))
}

//----------------------------------------------------------------------
// TLE Parameters Parser
//----------------------------------------------------------------------

fn is_tle_key(key: &str) -> bool {
    matches!(
        key,
        "EPHEMERIS_TYPE"
            | "CLASSIFICATION_TYPE"
            | "NORAD_CAT_ID"
            | "ELEMENT_SET_NO"
            | "REV_AT_EPOCH"
            | "BSTAR"
            | "BTERM"
            | "MEAN_MOTION_DOT"
            | "MEAN_MOTION_DDOT"
            | "AGOM"
    )
}

pub fn tle_parameters(input: &mut &str) -> ModalResult<Option<TleParameters>> {
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

    loop {
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if is_tle_key(k) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "EPHEMERIS_TYPE" => {
                        ephemeris_type = Some(parse_i32(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "CLASSIFICATION_TYPE" => classification_type = Some(val.to_string()),
                    "NORAD_CAT_ID" => {
                        norad_cat_id = Some(parse_u32(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "ELEMENT_SET_NO" => {
                        element_set_no = Some(parse_u32(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "REV_AT_EPOCH" => {
                        rev_at_epoch = Some(parse_u32(val).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "BSTAR" => {
                        bstar = Some(BStar::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "BTERM" => {
                        bterm = Some(M2kg::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "MEAN_MOTION_DOT" => {
                        mean_motion_dot = Some(MeanMotionDot::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    "MEAN_MOTION_DDOT" => {
                        mean_motion_ddot =
                            Some(MeanMotionDDot::from_kvn(val, unit).map_err(|e| {
                                ErrMode::Cut(ContextError::new().add_context(
                                    input,
                                    &input.checkpoint(),
                                    StrContext::Label(e.to_string().leak()),
                                ))
                            })?)
                    }
                    "AGOM" => {
                        agom = Some(M2kg::from_kvn(val, unit).map_err(|e| {
                            ErrMode::Cut(ContextError::new().add_context(
                                input,
                                &input.checkpoint(),
                                StrContext::Label(e.to_string().leak()),
                            ))
                        })?)
                    }
                    _ => unreachable!(),
                }
            }
            _ => break,
        }
    }

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

    if let Some(esn) = element_set_no {
        if esn > 9999 {
            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Label("ELEMENT_SET_NO must be in range [0, 9999]"),
            )));
        }
    }

    if bstar.is_some() && bterm.is_some() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Cannot have both BSTAR and BTERM"),
        )));
    }
    if bstar.is_none() && bterm.is_none() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Either BSTAR or BTERM must be present in TLE Parameters"),
        )));
    }

    if mean_motion_dot.is_none() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description("MEAN_MOTION_DOT")),
        )));
    }

    if mean_motion_ddot.is_some() && agom.is_some() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Cannot have both MEAN_MOTION_DDOT and AGOM"),
        )));
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
        mean_motion_dot,
        mean_motion_ddot,
        agom,
    }))
}

//----------------------------------------------------------------------
// OMM Data Parser
//----------------------------------------------------------------------

pub fn omm_data(input: &mut &str) -> ModalResult<OmmData> {
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

pub fn parse_omm(input: &mut &str) -> ModalResult<Omm> {
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
    fn parse_kvn(input: &mut &str) -> ModalResult<Self> {
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

        // TLE: neither BSTAR nor BTERM
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: missing MEAN_MOTION_DOT
        let mut input = "BSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: both MEAN_MOTION_DDOT and AGOM
        let mut input =
            "MEAN_MOTION_DOT = 0.000001\nBSTAR = 0.0001\nMEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // TLE: invalid ELEMENT_SET_NO
        let mut input = "MEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0\nBSTAR = 0.0001\nELEMENT_SET_NO = 10000\n";
        assert!(tle_parameters.parse_next(&mut input).is_err());

        // Invalid units for coverage
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
        assert!(tle_parameters.parse_next(&mut input).is_err()); // missing MEAN_MOTION_DOT

        let mut input = "MEAN_MOTION_DOT = 0\nBSTAR = 0\nREV_AT_EPOCH = 1\n";
        assert!(tle_parameters.parse_next(&mut input).is_ok());

        let mut input = "OBJECT_NAME = GOES 9\nREF_FRAME_EPOCH = INVALID\n";
        assert!(omm_metadata.parse_next(&mut input).is_err());
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
MEAN_ELEMENT_THEORY = SGP4
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
        assert_eq!(
            omm.body
                .segment
                .data
                .covariance_matrix
                .as_ref()
                .unwrap()
                .cx_x
                .value,
            1.0
        );
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
        assert!(omm.body.segment.data.mean_elements.mean_motion.is_none());
        assert_eq!(
            omm.body
                .segment
                .data
                .mean_elements
                .semi_major_axis
                .as_ref()
                .unwrap()
                .value,
            7000.0
        );
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
MEAN_ELEMENT_THEORY = SGP4
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
        assert!(omm
            .body
            .segment
            .data
            .mean_elements
            .semi_major_axis
            .is_none());
        assert_eq!(
            omm.body
                .segment
                .data
                .mean_elements
                .mean_motion
                .as_ref()
                .unwrap()
                .value,
            15.5
        );
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
