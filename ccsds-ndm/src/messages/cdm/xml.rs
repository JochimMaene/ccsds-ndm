// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{FormatError, Result};
use std::borrow::Cow;

/// Some operational CDMs represent unavailable optional values with empty or `nil="true"`
/// leaves (numbers, epochs, enumerations and text alike). Remove only those leaves, preserving byte offsets for later parse diagnostics.
pub(super) fn normalize_missing_values(xml: &str) -> Result<Cow<'_, str>> {
    use quick_xml::{
        events::{BytesStart, Event},
        Reader,
    };

    let mut reader = Reader::from_str(xml);
    // Reader events borrow the input; retaining them keeps ancestor names available without copies.
    let mut parents: Vec<BytesStart<'_>> = Vec::new();
    let mut namespaces = crate::xml::Namespaces::default();
    let mut child_form = None;
    let invalid = |message| FormatError::InvalidFormat(message).into();
    let mut missing = Vec::new();
    let mut candidate = None;
    loop {
        let start = reader.buffer_position() as usize;
        let event = reader.read_event().map_err(FormatError::from)?;
        let is_start = matches!(event, Event::Start(_));
        match event {
            Event::Start(element) | Event::Empty(element) => {
                // Validate namespace meaning before normalization can erase an empty leaf.
                namespaces.enter(&element, parents.len() + 1, &invalid)?;
                if !parents.is_empty() {
                    let form = crate::xml::element_form(&namespaces, element.name(), &invalid)?;
                    if *child_form.get_or_insert(form) != form {
                        return Err(invalid(
                            "qualified and unqualified NDM/XML elements cannot be mixed".into(),
                        ));
                    }
                }
                // A nested element is content, never an unavailable scalar.
                if let Some((_, _, ref mut empty)) = candidate {
                    *empty = false;
                }
                let parent = parents
                    .last()
                    .map(|element| element.local_name().into_inner())
                    .unwrap_or_default();
                let name = element.local_name().into_inner();
                let leaf = classify_leaf(parent, name);
                if leaf == Leaf::Required {
                    for attribute in element.attributes().flatten() {
                        if attribute.key.as_ref() == b"nil" {
                            return Err(FormatError::InvalidFormat(format!(
                                "{} is required; the nil attribute is not supported, including nil=false",
                                String::from_utf8_lossy(name)
                            ))
                            .into());
                        }
                    }
                }
                if leaf == Leaf::Optional {
                    let mut supported_attributes = true;
                    for attribute in element.attributes() {
                        let attribute = attribute
                            .map_err(quick_xml::Error::from)
                            .map_err(FormatError::from)?;
                        crate::xml::validate_attribute_value(&attribute, &|message| {
                            FormatError::InvalidFormat(message).into()
                        })?;
                        match attribute.key.as_ref() {
                            b"nil" if matches!(attribute.value.as_ref(), b"true" | b"1") => {}
                            b"units" if allows_units(name) => {}
                            _ if attribute.key.as_namespace_binding().is_some() => {}
                            _ => supported_attributes = false,
                        }
                    }
                    if supported_attributes {
                        if !is_start {
                            missing.push(start..reader.buffer_position() as usize);
                        } else {
                            candidate = Some((start, parents.len(), true));
                        }
                    }
                }
                if is_start {
                    parents.push(element);
                } else {
                    namespaces.leave(parents.len());
                }
            }
            Event::End(_) => {
                parents.pop();
                namespaces.leave(parents.len());
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

#[derive(Clone, Copy, PartialEq)]
enum Leaf {
    Required,
    Optional,
    Other,
}

fn classify_leaf(parent: &[u8], name: &[u8]) -> Leaf {
    if !cdm_xml_children(parent).is_some_and(|children| children.contains(&name))
        || name == b"COMMENT"
    {
        return Leaf::Other;
    }
    let optional = match parent {
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
    };
    if optional {
        Leaf::Optional
    } else if cdm_xml_children(name).is_none() {
        Leaf::Required
    } else {
        Leaf::Other
    }
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
    expected_unit(element).is_some()
}

fn expected_unit(element: &[u8]) -> Option<&'static str> {
    Some(match element {
        b"MISS_DISTANCE"
        | b"RELATIVE_POSITION_R"
        | b"RELATIVE_POSITION_T"
        | b"RELATIVE_POSITION_N"
        | b"SCREEN_VOLUME_X"
        | b"SCREEN_VOLUME_Y"
        | b"SCREEN_VOLUME_Z" => "m",
        b"RELATIVE_SPEED"
        | b"RELATIVE_VELOCITY_R"
        | b"RELATIVE_VELOCITY_T"
        | b"RELATIVE_VELOCITY_N" => "m/s",
        b"RECOMMENDED_OD_SPAN" | b"ACTUAL_OD_SPAN" => "d",
        b"RESIDUALS_ACCEPTED" => "%",
        b"AREA_PC" | b"AREA_DRG" | b"AREA_SRP" => "m**2",
        b"MASS" => "kg",
        b"CD_AREA_OVER_MASS" | b"CR_AREA_OVER_MASS" => "m**2/kg",
        b"THRUST_ACCELERATION" => "m/s**2",
        b"SEDR" => "W/kg",
        b"X" | b"Y" | b"Z" => "km",
        b"X_DOT" | b"Y_DOT" | b"Z_DOT" => "km/s",
        b"CR_R" | b"CT_R" | b"CT_T" | b"CN_R" | b"CN_T" | b"CN_N" => "m**2",
        b"CRDOT_R" | b"CRDOT_T" | b"CRDOT_N" | b"CTDOT_R" | b"CTDOT_T" | b"CTDOT_N"
        | b"CNDOT_R" | b"CNDOT_T" | b"CNDOT_N" => "m**2/s",
        b"CRDOT_RDOT" | b"CTDOT_RDOT" | b"CTDOT_TDOT" | b"CNDOT_RDOT" | b"CNDOT_TDOT"
        | b"CNDOT_NDOT" | b"CTHR_R" | b"CTHR_T" | b"CTHR_N" => "m**2/s**2",
        b"CDRG_R" | b"CDRG_T" | b"CDRG_N" | b"CSRP_R" | b"CSRP_T" | b"CSRP_N" => "m**3/kg",
        b"CDRG_RDOT" | b"CDRG_TDOT" | b"CDRG_NDOT" | b"CSRP_RDOT" | b"CSRP_TDOT" | b"CSRP_NDOT" => {
            "m**3/(kg*s)"
        }
        b"CDRG_DRG" | b"CSRP_DRG" | b"CSRP_SRP" => "m**4/kg**2",
        b"CTHR_RDOT" | b"CTHR_TDOT" | b"CTHR_NDOT" => "m**2/s**3",
        b"CTHR_DRG" | b"CTHR_SRP" => "m**3/(kg*s**2)",
        b"CTHR_THR" => "m**2/s**4",
        _ => return None,
    })
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

/// CDM 1.4.3.1: N/A denotes an unavailable value; comments and MANEUVERABLE remain opaque.
pub(super) fn without_unavailable_values(s: &str) -> Result<Cow<'_, str>> {
    use quick_xml::events::Event;

    let content = if s.starts_with("<?xml") {
        s.split_once("?>").map_or(s, |(_, rest)| rest)
    } else {
        s
    };
    if !s
        .as_bytes()
        .windows(3)
        .any(|token| token.eq_ignore_ascii_case(b"N/A"))
        && !s.contains("&#")
        && !s.contains("<![CDATA[")
        && !s.contains("<!--")
        && !content.contains("<?")
    {
        return Ok(std::borrow::Cow::Borrowed(s));
    }
    // Recognize the decoded scalar without allocating a string for each history record.
    fn append(matched: &mut Option<usize>, text: &str) {
        for byte in text.bytes() {
            let Some(index) = *matched else {
                break;
            };
            if matches!(byte, b' ' | b'\t' | b'\r' | b'\n') && matches!(index, 0 | 3) {
                continue;
            }
            *matched = b"N/A"
                .get(index)
                .filter(|expected| expected.eq_ignore_ascii_case(&byte))
                .map(|_| index + 1);
        }
    }
    let mut reader = quick_xml::Reader::from_str(s);
    let mut stack = Vec::new();
    let mut ranges = Vec::new();
    loop {
        let offset = reader.buffer_position() as usize;
        match reader.read_event()? {
            Event::Start(start) => {
                let mut invalid_unit = None;
                for attribute in start.attributes() {
                    let attribute = attribute.map_err(quick_xml::Error::from)?;
                    if attribute.key.as_ref() == b"units" {
                        let unit = attribute.unescape_value()?;
                        if expected_unit(start.local_name().as_ref()) != Some(unit.as_ref()) {
                            invalid_unit = Some(format!(
                                "{} units must be {}; got {unit:?}",
                                String::from_utf8_lossy(start.local_name().as_ref()),
                                expected_unit(start.local_name().as_ref()).unwrap_or("absent"),
                            ));
                        }
                    }
                }
                if let Some((_, matched, _)) = stack.last_mut() {
                    *matched = None;
                }
                let opaque = matches!(
                    start.local_name().as_ref(),
                    b"USER_DEFINED" | b"COMMENT" | b"MANEUVERABLE"
                );
                stack.push((offset, (!opaque).then_some(0), invalid_unit));
            }
            Event::Text(text) => {
                if let Some((_, matched, _)) = stack.last_mut() {
                    let value = text
                        .decode()
                        .map_err(|error| FormatError::InvalidFormat(error.to_string()))?;
                    append(matched, &value);
                }
            }
            Event::CData(text) => {
                if let Some((_, matched, _)) = stack.last_mut() {
                    let value = text
                        .decode()
                        .map_err(|error| FormatError::InvalidFormat(error.to_string()))?;
                    append(matched, &value);
                }
            }
            Event::GeneralRef(reference) => {
                if let Some((_, matched, _)) = stack.last_mut() {
                    match reference.resolve_char_ref()? {
                        Some(character) => {
                            let mut bytes = [0; 4];
                            append(matched, character.encode_utf8(&mut bytes));
                        }
                        None => *matched = None,
                    }
                }
            }
            Event::End(_) => {
                if let Some((start, Some(3), invalid_unit)) = stack.pop() {
                    // Ordinary values keep the deserializer's unit diagnostics, even on this path.
                    if let Some(message) = invalid_unit {
                        let (line, column) = crate::error::line_column(s, start);
                        return Err(crate::error::CcsdsNdmError::Parsing {
                            context: Box::new(crate::error::ParseErrorContext {
                                notation: crate::error::DiagnosticNotation::Xml,
                                message_kind: crate::validation::MessageKind::Cdm,
                                source_edition: None,
                                byte_offset: Some(start),
                                line: Some(line),
                                column: Some(column),
                                original_token: None,
                                expected: None,
                            }),
                            source: Box::new(
                                FormatError::XmlDe(quick_xml::DeError::Custom(message)).into(),
                            ),
                        });
                    }
                    ranges.push(start..reader.buffer_position() as usize);
                }
            }
            Event::Comment(_) | Event::PI(_) => {}
            Event::Eof => break,
            _ => {
                if let Some((_, matched, _)) = stack.last_mut() {
                    *matched = None;
                }
            }
        }
    }
    if ranges.is_empty() {
        return Ok(std::borrow::Cow::Borrowed(s));
    }
    let mut bytes = s.as_bytes().to_vec();
    for range in ranges {
        for byte in &mut bytes[range] {
            if !matches!(byte, b'\r' | b'\n') {
                *byte = b' ';
            }
        }
    }
    Ok(std::borrow::Cow::Owned(
        String::from_utf8(bytes).expect("ASCII replacement preserves UTF-8"),
    ))
}
