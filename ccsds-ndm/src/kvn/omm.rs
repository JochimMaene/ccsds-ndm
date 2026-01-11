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

pub fn omm_version<'a>(input: &mut &'a str) -> ModalResult<String> {
    let _ = collect_comments.parse_next(input)?;
    let (value, _) = expect_key("CCSDS_OMM_VERS").parse_next(input)?;
    Ok(value.to_string())
}

//----------------------------------------------------------------------
// OMM Metadata Parser
//----------------------------------------------------------------------

pub fn omm_metadata<'a>(input: &mut &'a str) -> ModalResult<OmmMetadata> {
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
                        ref_frame_epoch = Some(
                            Epoch::from_str(v).map_err(|_| ErrMode::Cut(ContextError::new()))?,
                        );
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

pub fn mean_elements<'a>(input: &mut &'a str) -> ModalResult<(Vec<String>, MeanElements)> {
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

pub fn tle_parameters<'a>(input: &mut &'a str) -> ModalResult<Option<TleParameters>> {
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
    if mean_motion_ddot.is_none() && agom.is_none() {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Label("Either MEAN_MOTION_DDOT or AGOM must be present in TLE Parameters"),
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

pub fn omm_data<'a>(input: &mut &'a str) -> ModalResult<OmmData> {
    let (me_comment, mean_elements) = mean_elements.parse_next(input)?;

    // Spacecraft parameters
    let spacecraft_parameters = crate::kvn::opm::spacecraft_parameters.parse_next(input)?;

    // TLE parameters
    let tle_parameters = tle_parameters.parse_next(input)?;

    // Covariance matrix
    let covariance_matrix = crate::kvn::opm::covariance_matrix.parse_next(input)?;

    // User defined
    let user_defined_parameters = crate::kvn::opm::user_defined_parameters.parse_next(input)?;

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

pub fn parse_omm<'a>(input: &mut &'a str) -> ModalResult<Omm> {
    let version = omm_version.parse_next(input)?;
    let header = crate::kvn::opm::odm_header.parse_next(input)?;
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
}
