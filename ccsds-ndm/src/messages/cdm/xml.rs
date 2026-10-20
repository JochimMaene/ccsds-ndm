// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{FormatError, Result};
use std::borrow::Cow;

/// Some operational CDMs represent unavailable optional values with empty or `nil="true"`
/// leaves. Remove only those leaves, preserving byte offsets for later parse diagnostics.
pub(super) fn normalize_missing_values(xml: &str) -> Result<Cow<'_, str>> {
    use quick_xml::{events::Event, Reader};

    let mut reader = Reader::from_str(xml);
    let mut parents: Vec<Vec<u8>> = Vec::new();
    let mut missing = Vec::new();
    let mut candidate = None;
    loop {
        let start = reader.buffer_position() as usize;
        let event = reader.read_event().map_err(FormatError::from)?;
        match event {
            Event::Start(ref element) | Event::Empty(ref element) => {
                // A nested element is content, never an unavailable scalar.
                if let Some((_, _, ref mut empty)) = candidate {
                    *empty = false;
                }
                let parent = parents.last().map(Vec::as_slice).unwrap_or_default();
                let name = element.name();
                if optional_leaf(parent, name.as_ref()) {
                    let mut nil = false;
                    let mut supported_attributes = true;
                    for attribute in element.attributes() {
                        let attribute = attribute
                            .map_err(quick_xml::Error::from)
                            .map_err(FormatError::from)?;
                        crate::xml::validate_attribute_value(&attribute, &|message| {
                            FormatError::InvalidFormat(message).into()
                        })?;
                        match attribute.key.as_ref() {
                            b"nil" if matches!(attribute.value.as_ref(), b"true" | b"1") => {
                                nil = true
                            }
                            b"units" if allows_units(name.as_ref()) => {}
                            _ => supported_attributes = false,
                        }
                    }
                    if supported_attributes && (nil || empty_is_absent(name.as_ref())) {
                        if matches!(event, Event::Empty(_)) {
                            missing.push(start..reader.buffer_position() as usize);
                        } else {
                            candidate = Some((start, parents.len(), true));
                        }
                    }
                }
                if matches!(event, Event::Start(_)) {
                    parents.push(name.as_ref().to_vec());
                }
            }
            Event::End(_) => {
                parents.pop();
                if candidate
                    .as_ref()
                    .is_some_and(|(_, depth, _)| *depth == parents.len())
                {
                    let (start, _, empty) = candidate.take().unwrap();
                    if empty {
                        missing.push(start..reader.buffer_position() as usize);
                    }
                }
            }
            Event::Text(text) => {
                let text: &[u8] = text.as_ref();
                if !text
                    .iter()
                    .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
                {
                    if let Some((_, _, ref mut empty)) = candidate {
                        *empty = false;
                    }
                }
            }
            Event::Eof => break,
            _ => {
                if let Some((_, _, ref mut empty)) = candidate {
                    *empty = false;
                }
            }
        }
    }
    if missing.is_empty() {
        return Ok(Cow::Borrowed(xml));
    }
    let mut normalized = xml.as_bytes().to_vec();
    for range in missing {
        for byte in &mut normalized[range] {
            if !matches!(*byte, b'\r' | b'\n') {
                *byte = b' ';
            }
        }
    }
    Ok(Cow::Owned(
        String::from_utf8(normalized).expect("only ASCII missing leaves replaced"),
    ))
}

fn optional_leaf(parent: &[u8], name: &[u8]) -> bool {
    if !cdm_xml_children(parent).is_some_and(|children| children.contains(&name))
        || name == b"COMMENT"
    {
        return false;
    }
    match parent {
        b"header" => name == b"MESSAGE_FOR",
        b"relativeMetadataData" => {
            !matches!(name, b"TCA" | b"MISS_DISTANCE" | b"relativeStateVector")
        }
        b"metadata" => !matches!(
            name,
            b"OBJECT"
                | b"OBJECT_DESIGNATOR"
                | b"CATALOG_NAME"
                | b"OBJECT_NAME"
                | b"INTERNATIONAL_DESIGNATOR"
                | b"EPHEMERIS_NAME"
                | b"COVARIANCE_METHOD"
                | b"MANEUVERABLE"
                | b"REF_FRAME"
        ),
        b"odParameters" | b"additionalParameters" => true,
        b"covarianceMatrix" => {
            name.starts_with(b"CDRG_") || name.starts_with(b"CSRP_") || name.starts_with(b"CTHR_")
        }
        _ => false,
    }
}

fn empty_is_absent(name: &[u8]) -> bool {
    !matches!(
        name,
        b"MESSAGE_FOR"
            | b"COLLISION_PROBABILITY_METHOD"
            | b"OPERATOR_CONTACT_POSITION"
            | b"OPERATOR_ORGANIZATION"
            | b"OPERATOR_PHONE"
            | b"OPERATOR_EMAIL"
            | b"ORBIT_CENTER"
            | b"GRAVITY_MODEL"
            | b"ATMOSPHERIC_MODEL"
            | b"N_BODY_PERTURBATIONS"
    )
}

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    crate::xml::validate_element_sequences(
        xml,
        b"cdm",
        "CDM",
        |parent, child| {
            let children = cdm_xml_children(parent)?;
            let rank = children.iter().position(|candidate| *candidate == child)? as u16;
            let repeatable = child == b"COMMENT" || (parent == b"body" && child == b"segment");
            Some(XmlSequenceRule::new(rank, repeatable))
        },
        |element, attribute| attribute == b"units" && allows_units(element),
    )
}

fn allows_units(element: &[u8]) -> bool {
    matches!(
        element,
        b"MISS_DISTANCE"
            | b"RELATIVE_SPEED"
            | b"RELATIVE_POSITION_R"
            | b"RELATIVE_POSITION_T"
            | b"RELATIVE_POSITION_N"
            | b"RELATIVE_VELOCITY_R"
            | b"RELATIVE_VELOCITY_T"
            | b"RELATIVE_VELOCITY_N"
            | b"SCREEN_VOLUME_X"
            | b"SCREEN_VOLUME_Y"
            | b"SCREEN_VOLUME_Z"
            | b"RECOMMENDED_OD_SPAN"
            | b"ACTUAL_OD_SPAN"
            | b"RESIDUALS_ACCEPTED"
            | b"AREA_PC"
            | b"AREA_DRG"
            | b"AREA_SRP"
            | b"MASS"
            | b"CD_AREA_OVER_MASS"
            | b"CR_AREA_OVER_MASS"
            | b"THRUST_ACCELERATION"
            | b"SEDR"
            | b"X"
            | b"Y"
            | b"Z"
            | b"X_DOT"
            | b"Y_DOT"
            | b"Z_DOT"
            | b"CR_R"
            | b"CT_R"
            | b"CT_T"
            | b"CN_R"
            | b"CN_T"
            | b"CN_N"
            | b"CRDOT_R"
            | b"CRDOT_T"
            | b"CRDOT_N"
            | b"CRDOT_RDOT"
            | b"CTDOT_R"
            | b"CTDOT_T"
            | b"CTDOT_N"
            | b"CTDOT_RDOT"
            | b"CTDOT_TDOT"
            | b"CNDOT_R"
            | b"CNDOT_T"
            | b"CNDOT_N"
            | b"CNDOT_RDOT"
            | b"CNDOT_TDOT"
            | b"CNDOT_NDOT"
            | b"CDRG_R"
            | b"CDRG_T"
            | b"CDRG_N"
            | b"CDRG_RDOT"
            | b"CDRG_TDOT"
            | b"CDRG_NDOT"
            | b"CDRG_DRG"
            | b"CSRP_R"
            | b"CSRP_T"
            | b"CSRP_N"
            | b"CSRP_RDOT"
            | b"CSRP_TDOT"
            | b"CSRP_NDOT"
            | b"CSRP_DRG"
            | b"CSRP_SRP"
            | b"CTHR_R"
            | b"CTHR_T"
            | b"CTHR_N"
            | b"CTHR_RDOT"
            | b"CTHR_TDOT"
            | b"CTHR_NDOT"
            | b"CTHR_DRG"
            | b"CTHR_SRP"
            | b"CTHR_THR"
    )
}

fn cdm_xml_children(parent: &[u8]) -> Option<&'static [&'static [u8]]> {
    Some(match parent {
        b"cdm" => &[b"header", b"body"],
        b"header" => &[
            b"COMMENT",
            b"CREATION_DATE",
            b"ORIGINATOR",
            b"MESSAGE_FOR",
            b"MESSAGE_ID",
        ],
        b"body" => &[b"relativeMetadataData", b"segment"],
        b"relativeMetadataData" => &[
            b"COMMENT",
            b"TCA",
            b"MISS_DISTANCE",
            b"RELATIVE_SPEED",
            b"relativeStateVector",
            b"START_SCREEN_PERIOD",
            b"STOP_SCREEN_PERIOD",
            b"SCREEN_VOLUME_FRAME",
            b"SCREEN_VOLUME_SHAPE",
            b"SCREEN_VOLUME_X",
            b"SCREEN_VOLUME_Y",
            b"SCREEN_VOLUME_Z",
            b"SCREEN_ENTRY_TIME",
            b"SCREEN_EXIT_TIME",
            b"COLLISION_PROBABILITY",
            b"COLLISION_PROBABILITY_METHOD",
        ],
        b"relativeStateVector" => &[
            b"RELATIVE_POSITION_R",
            b"RELATIVE_POSITION_T",
            b"RELATIVE_POSITION_N",
            b"RELATIVE_VELOCITY_R",
            b"RELATIVE_VELOCITY_T",
            b"RELATIVE_VELOCITY_N",
        ],
        b"segment" => &[b"metadata", b"data"],
        b"metadata" => &[
            b"COMMENT",
            b"OBJECT",
            b"OBJECT_DESIGNATOR",
            b"CATALOG_NAME",
            b"OBJECT_NAME",
            b"INTERNATIONAL_DESIGNATOR",
            b"OBJECT_TYPE",
            b"OPERATOR_CONTACT_POSITION",
            b"OPERATOR_ORGANIZATION",
            b"OPERATOR_PHONE",
            b"OPERATOR_EMAIL",
            b"EPHEMERIS_NAME",
            b"COVARIANCE_METHOD",
            b"MANEUVERABLE",
            b"ORBIT_CENTER",
            b"REF_FRAME",
            b"GRAVITY_MODEL",
            b"ATMOSPHERIC_MODEL",
            b"N_BODY_PERTURBATIONS",
            b"SOLAR_RAD_PRESSURE",
            b"EARTH_TIDES",
            b"INTRACK_THRUST",
        ],
        b"data" => &[
            b"COMMENT",
            b"odParameters",
            b"additionalParameters",
            b"stateVector",
            b"covarianceMatrix",
        ],
        b"odParameters" => &[
            b"COMMENT",
            b"TIME_LASTOB_START",
            b"TIME_LASTOB_END",
            b"RECOMMENDED_OD_SPAN",
            b"ACTUAL_OD_SPAN",
            b"OBS_AVAILABLE",
            b"OBS_USED",
            b"TRACKS_AVAILABLE",
            b"TRACKS_USED",
            b"RESIDUALS_ACCEPTED",
            b"WEIGHTED_RMS",
        ],
        b"additionalParameters" => &[
            b"COMMENT",
            b"AREA_PC",
            b"AREA_DRG",
            b"AREA_SRP",
            b"MASS",
            b"CD_AREA_OVER_MASS",
            b"CR_AREA_OVER_MASS",
            b"THRUST_ACCELERATION",
            b"SEDR",
        ],
        b"stateVector" => &[b"COMMENT", b"X", b"Y", b"Z", b"X_DOT", b"Y_DOT", b"Z_DOT"],
        b"covarianceMatrix" => &[
            b"COMMENT",
            b"CR_R",
            b"CT_R",
            b"CT_T",
            b"CN_R",
            b"CN_T",
            b"CN_N",
            b"CRDOT_R",
            b"CRDOT_T",
            b"CRDOT_N",
            b"CRDOT_RDOT",
            b"CTDOT_R",
            b"CTDOT_T",
            b"CTDOT_N",
            b"CTDOT_RDOT",
            b"CTDOT_TDOT",
            b"CNDOT_R",
            b"CNDOT_T",
            b"CNDOT_N",
            b"CNDOT_RDOT",
            b"CNDOT_TDOT",
            b"CNDOT_NDOT",
            b"CDRG_R",
            b"CDRG_T",
            b"CDRG_N",
            b"CDRG_RDOT",
            b"CDRG_TDOT",
            b"CDRG_NDOT",
            b"CDRG_DRG",
            b"CSRP_R",
            b"CSRP_T",
            b"CSRP_N",
            b"CSRP_RDOT",
            b"CSRP_TDOT",
            b"CSRP_NDOT",
            b"CSRP_DRG",
            b"CSRP_SRP",
            b"CTHR_R",
            b"CTHR_T",
            b"CTHR_N",
            b"CTHR_RDOT",
            b"CTHR_TDOT",
            b"CTHR_NDOT",
            b"CTHR_DRG",
            b"CTHR_SRP",
            b"CTHR_THR",
        ],
        _ => return None,
    })
}
