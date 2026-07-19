// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::options::ParseOptions;
use crate::MessageType;
use winnow::ascii::multispace1;
use winnow::combinator::{alt, repeat};
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::token::take_till;

type PResult<T> = std::result::Result<T, ErrMode<ContextError>>;

const KVN_HEADERS: [&str; 10] = [
    "CCSDS_OPM_VERS",
    "CCSDS_OMM_VERS",
    "CCSDS_OEM_VERS",
    "CCSDS_OCM_VERS",
    "CCSDS_ACM_VERS",
    "CCSDS_CDM_VERS",
    "CCSDS_TDM_VERS",
    "CCSDS_RDM_VERS",
    "CCSDS_AEM_VERS",
    "CCSDS_APM_VERS",
];

pub(crate) fn kvn_message_offsets(input: &str) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut offset = 0usize;
    for line in input.split_inclusive('\n') {
        let line_without_eol = line.strip_suffix('\n').unwrap_or(line);
        let line_without_eol = line_without_eol
            .strip_suffix('\r')
            .unwrap_or(line_without_eol);
        if KVN_HEADERS.iter().any(|header| {
            line_without_eol.strip_prefix(header).is_some_and(|rest| {
                rest.starts_with('=')
                    || rest.as_bytes().first().is_some_and(u8::is_ascii_whitespace)
            })
        }) {
            offsets.push(offset);
        }
        offset += line.len();
    }
    offsets
}

/// Wire notation used by complete NDM documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notation {
    Kvn,
    Xml,
}

/// Remove one leading UTF-8 byte-order mark.
pub(crate) fn without_utf8_bom(input: &str) -> &str {
    input.strip_prefix('\u{feff}').unwrap_or(input)
}

/// Detect whether a complete NDM document uses KVN or XML notation.
///
/// Detection ignores a leading UTF-8 byte-order mark and leading whitespace. Empty input is
/// rejected rather than classified as KVN.
pub fn detect_notation(input: &str) -> Result<Notation> {
    detect_notation_bytes(input.as_bytes())
}

pub(crate) fn detect_notation_bytes(input: &[u8]) -> Result<Notation> {
    let input = input.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(input);
    match input
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace())
    {
        Some(b'<') => Ok(Notation::Xml),
        Some(_) => Ok(Notation::Kvn),
        None => Err(CcsdsNdmError::UnexpectedEof {
            context: "Empty input".into(),
        }),
    }
}

/// Detects the NDM message type from the input string (KVN or XML).
pub fn detect_message_type(s: &str) -> Result<MessageType> {
    detect_message_type_with_options(s, None, &ParseOptions::default())
}

/// Parse a complete NDM document with optional notation selection and resource limits.
pub fn detect_message_type_with_options(
    s: &str,
    notation: Option<Notation>,
    options: &ParseOptions,
) -> Result<MessageType> {
    let input = without_utf8_bom(s);
    let notation = notation.map_or_else(|| detect_notation(input), Ok)?;
    let result = match notation {
        Notation::Kvn => detect_kvn_type(input, options),
        Notation::Xml => detect_xml_type(input, options),
    };
    result.map_err(|error| {
        if error.diagnostic().is_some() {
            error
        } else {
            error.with_parse_context(
                crate::validation::MessageKind::Ndm,
                match notation {
                    Notation::Kvn => crate::error::DiagnosticNotation::Kvn,
                    Notation::Xml => crate::error::DiagnosticNotation::Xml,
                },
                input,
                None,
            )
        }
    })
}

fn validate_input_size(
    input: &str,
    options: &ParseOptions,
    kind: crate::validation::MessageKind,
    notation: crate::error::DiagnosticNotation,
) -> Result<()> {
    if let Some(limit) = options.max_input_bytes {
        if input.len() > limit {
            return Err(CcsdsNdmError::ResourceLimitExceeded {
                resource: "input_document",
                limit,
                actual: input.len(),
            }
            .with_parse_context(kind, notation, input, None));
        }
    }
    Ok(())
}

fn record_limit_error(limit: usize, actual: usize) -> CcsdsNdmError {
    CcsdsNdmError::ResourceLimitExceeded {
        resource: "history_records",
        limit,
        actual,
    }
}

fn validate_kvn_record_limit(
    input: &str,
    kind: crate::validation::MessageKind,
    options: &ParseOptions,
) -> Result<()> {
    let Some(limit) = options.max_records else {
        return Ok(());
    };
    use crate::validation::MessageKind;
    if !matches!(
        kind,
        MessageKind::Ocm | MessageKind::Tdm | MessageKind::Aem | MessageKind::Acm
    ) {
        return Ok(());
    }

    let mut section = None;
    let mut records = 0usize;
    for raw_line in input.lines() {
        let line = raw_line.trim();
        match line {
            "DATA_START" if matches!(kind, MessageKind::Tdm | MessageKind::Aem) => {
                section = Some("DATA")
            }
            "DATA_STOP" => section = None,
            "TRAJ_START" if kind == MessageKind::Ocm => section = Some("TRAJ"),
            "TRAJ_STOP" => section = None,
            "COV_START" if matches!(kind, MessageKind::Ocm | MessageKind::Acm) => {
                section = Some("COV")
            }
            "COV_STOP" => section = None,
            "MAN_START" if kind == MessageKind::Ocm => section = Some("MAN"),
            "MAN_START" if kind == MessageKind::Acm => {
                records += 1;
                section = None;
            }
            "MAN_STOP" => section = None,
            "ATT_START" if kind == MessageKind::Acm => section = Some("ATT"),
            "ATT_STOP" => section = None,
            _ if section.is_some()
                && !line.is_empty()
                && !line.starts_with("COMMENT")
                && (matches!(kind, MessageKind::Tdm | MessageKind::Aem) || !line.contains('=')) =>
            {
                records += 1;
            }
            _ => {}
        }
        if records > limit {
            return Err(record_limit_error(limit, records).with_parse_context(
                kind,
                crate::error::DiagnosticNotation::Kvn,
                input,
                None,
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum NdmKind {
    Opm,
    Omm,
    Oem,
    Ocm,
    Acm,
    Cdm,
    Tdm,
    Rdm,
    Aem,
    Apm,
    Ndm,
}

impl NdmKind {
    fn message_kind(self) -> crate::validation::MessageKind {
        use crate::validation::MessageKind;
        match self {
            Self::Opm => MessageKind::Opm,
            Self::Omm => MessageKind::Omm,
            Self::Oem => MessageKind::Oem,
            Self::Ocm => MessageKind::Ocm,
            Self::Acm => MessageKind::Acm,
            Self::Cdm => MessageKind::Cdm,
            Self::Tdm => MessageKind::Tdm,
            Self::Rdm => MessageKind::Rdm,
            Self::Aem => MessageKind::Aem,
            Self::Apm => MessageKind::Apm,
            Self::Ndm => MessageKind::Ndm,
        }
    }

    fn from_xml_root(name: &[u8]) -> Option<Self> {
        match name {
            b"opm" => Some(Self::Opm),
            b"omm" => Some(Self::Omm),
            b"oem" => Some(Self::Oem),
            b"ocm" => Some(Self::Ocm),
            b"acm" => Some(Self::Acm),
            b"cdm" => Some(Self::Cdm),
            b"tdm" => Some(Self::Tdm),
            b"rdm" => Some(Self::Rdm),
            b"aem" => Some(Self::Aem),
            b"apm" => Some(Self::Apm),
            b"ndm" => Some(Self::Ndm),
            _ => None,
        }
    }

    fn parse_kvn(self, input: &str, options: &ParseOptions) -> Result<MessageType> {
        match self {
            Self::Opm => crate::messages::opm::Opm::from_kvn_with_options(input, options)
                .map(MessageType::Opm),
            Self::Omm => crate::traits::Ndm::from_kvn(input).map(MessageType::Omm),
            Self::Oem => crate::messages::oem::Oem::from_kvn_with_options(input, options)
                .map(MessageType::Oem),
            Self::Ocm => crate::traits::Ndm::from_kvn(input).map(MessageType::Ocm),
            Self::Acm => crate::traits::Ndm::from_kvn(input).map(MessageType::Acm),
            Self::Cdm => crate::traits::Ndm::from_kvn(input).map(MessageType::Cdm),
            Self::Tdm => crate::traits::Ndm::from_kvn(input).map(MessageType::Tdm),
            Self::Rdm => crate::traits::Ndm::from_kvn(input).map(MessageType::Rdm),
            Self::Aem => crate::traits::Ndm::from_kvn(input).map(MessageType::Aem),
            Self::Apm => crate::traits::Ndm::from_kvn(input).map(MessageType::Apm),
            Self::Ndm => Err(CcsdsNdmError::UnsupportedMessage(
                "combined NDM is detected from multiple KVN headers".into(),
            )),
        }
    }

    fn parse_xml(self, input: &str, options: &ParseOptions) -> Result<MessageType> {
        match self {
            Self::Opm => crate::messages::opm::Opm::from_xml_with_options(input, options)
                .map(MessageType::Opm),
            Self::Oem => crate::messages::oem::Oem::from_xml_with_options(input, options)
                .map(MessageType::Oem),
            Self::Omm => crate::traits::Ndm::from_xml(input).map(MessageType::Omm),
            Self::Ocm => crate::traits::Ndm::from_xml(input).map(MessageType::Ocm),
            Self::Acm => crate::traits::Ndm::from_xml(input).map(MessageType::Acm),
            Self::Cdm => crate::traits::Ndm::from_xml(input).map(MessageType::Cdm),
            Self::Tdm => crate::traits::Ndm::from_xml(input).map(MessageType::Tdm),
            Self::Rdm => crate::traits::Ndm::from_xml(input).map(MessageType::Rdm),
            Self::Aem => crate::traits::Ndm::from_xml(input).map(MessageType::Aem),
            Self::Apm => crate::traits::Ndm::from_xml(input).map(MessageType::Apm),
            Self::Ndm => crate::messages::ndm::CombinedNdm::from_xml_with_options(input, options)
                .map(MessageType::Ndm),
        }
    }
}

fn ensure_parse_context(
    error: CcsdsNdmError,
    kind: crate::validation::MessageKind,
    notation: crate::error::DiagnosticNotation,
    input: &str,
) -> CcsdsNdmError {
    if error.diagnostic().is_some() {
        error
    } else {
        error.with_parse_context(kind, notation, input, None)
    }
}

fn with_xml_parse_context<T>(
    result: Result<T>,
    kind: crate::validation::MessageKind,
    input: &str,
) -> Result<T> {
    result.map_err(|error| {
        ensure_parse_context(error, kind, crate::error::DiagnosticNotation::Xml, input)
    })
}

/// Winnow parser to identify the KVN message type from the header
fn parse_kvn_kind(input: &mut &str) -> PResult<NdmKind> {
    // Skip whitespace and comments
    // Using explicit type annotation for the accumulated value to ensure type inference works
    let _: () = repeat(
        0..,
        alt((
            multispace1.void(),
            ("COMMENT", take_till(0.., ('\r', '\n'))).void(),
        )),
    )
    .parse_next(input)?;

    // Check for CCSDS_..._VERS header
    alt((
        alt((
            "CCSDS_OPM_VERS".value(NdmKind::Opm),
            "CCSDS_OMM_VERS".value(NdmKind::Omm),
            "CCSDS_OEM_VERS".value(NdmKind::Oem),
            "CCSDS_OCM_VERS".value(NdmKind::Ocm),
            "CCSDS_ACM_VERS".value(NdmKind::Acm),
        )),
        alt((
            "CCSDS_CDM_VERS".value(NdmKind::Cdm),
            "CCSDS_TDM_VERS".value(NdmKind::Tdm),
            "CCSDS_RDM_VERS".value(NdmKind::Rdm),
            "CCSDS_AEM_VERS".value(NdmKind::Aem),
            "CCSDS_APM_VERS".value(NdmKind::Apm),
        )),
    ))
    .parse_next(input)
}

/// Detects and parses KVN message type
fn detect_kvn_type(s: &str, options: &ParseOptions) -> Result<MessageType> {
    // We need a mutable slice for winnow, but we don't want to consume "s" for the final parsing.
    let mut input = s;
    let kind = parse_kvn_kind
        .parse_next(&mut input)
        .map_err(|_| CcsdsNdmError::UnsupportedMessage("Could not identify KVN header".into()))?;

    if kvn_message_offsets(s).len() > 1 {
        validate_input_size(
            s,
            options,
            crate::validation::MessageKind::Ndm,
            crate::error::DiagnosticNotation::Kvn,
        )?;
        return crate::messages::ndm::CombinedNdm::from_kvn_with_options(s, options)
            .map(MessageType::Ndm)
            .map_err(|error| {
                ensure_parse_context(
                    error,
                    crate::validation::MessageKind::Ndm,
                    crate::error::DiagnosticNotation::Kvn,
                    s,
                )
            });
    }

    validate_input_size(
        s,
        options,
        kind.message_kind(),
        crate::error::DiagnosticNotation::Kvn,
    )?;
    validate_kvn_record_limit(s, kind.message_kind(), options)?;

    let result = kind.parse_kvn(s, options);
    result.map_err(|error| {
        ensure_parse_context(
            error,
            kind.message_kind(),
            crate::error::DiagnosticNotation::Kvn,
            s,
        )
    })
}

// XML Detection
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn detect_xml_type(s: &str, options: &ParseOptions) -> Result<MessageType> {
    let mut reader = Reader::from_str(s);
    reader.config_mut().trim_text_start = true;
    reader.config_mut().trim_text_end = true;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let root_name = e.name();
                let kind = NdmKind::from_xml_root(root_name.as_ref()).ok_or_else(|| {
                    CcsdsNdmError::UnsupportedMessage(format!(
                        "Unknown or unsupported XML root tag: <{}>",
                        String::from_utf8_lossy(root_name.as_ref())
                    ))
                })?;
                let message_kind = kind.message_kind();
                validate_input_size(
                    s,
                    options,
                    message_kind,
                    crate::error::DiagnosticNotation::Xml,
                )?;
                // OPM and OEM apply their specialised strict preflight, including these limits.
                if !matches!(kind, NdmKind::Opm | NdmKind::Oem) {
                    validate_xml_limits(s, options, message_kind)?;
                }
                return with_xml_parse_context(kind.parse_xml(s, options), message_kind, s);
            }
            Ok(Event::Decl(_))
            | Ok(Event::Comment(_))
            | Ok(Event::DocType(_))
            | Ok(Event::PI(_)) => {
                continue;
            }
            Ok(Event::Eof) => {
                return Err(CcsdsNdmError::UnexpectedEof {
                    context: "No XML root tag found".into(),
                });
            }
            Ok(Event::Text(text)) if ascii_whitespace(text.as_ref()) => continue,
            Ok(Event::Text(_)) => {
                return Err(CcsdsNdmError::UnsupportedMessage(
                    "Non-whitespace content before the XML root".into(),
                ));
            }
            Ok(e) => {
                return Err(CcsdsNdmError::UnsupportedMessage(format!(
                    "Unexpected XML event during detection: {:?}",
                    e
                )))
            }
            Err(e) => return Err(e.into()),
        }
    }
}

fn ascii_whitespace(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_whitespace)
}

fn validate_xml_limits(
    input: &str,
    options: &ParseOptions,
    kind: crate::validation::MessageKind,
) -> Result<()> {
    let mut reader = Reader::from_str(input);
    let mut depth = 0usize;
    let mut records = 0usize;
    let mut combined_child = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(start)) => {
                if kind == crate::validation::MessageKind::Ndm && depth == 1 {
                    combined_child = xml_message_kind(start.name().as_ref());
                }
                depth = depth.saturating_add(1);
                if depth > options.max_xml_depth {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit: options.max_xml_depth,
                        actual: depth,
                    }
                    .with_parse_context(
                        kind,
                        crate::error::DiagnosticNotation::Xml,
                        input,
                        None,
                    ));
                }
                if is_xml_record(combined_child.unwrap_or(kind), start.name().as_ref()) {
                    records += 1;
                    enforce_xml_record_limit(input, options, kind, records)?;
                }
            }
            Ok(Event::Empty(start)) => {
                let actual_depth = depth.saturating_add(1);
                if actual_depth > options.max_xml_depth {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit: options.max_xml_depth,
                        actual: actual_depth,
                    }
                    .with_parse_context(
                        kind,
                        crate::error::DiagnosticNotation::Xml,
                        input,
                        None,
                    ));
                }
                if is_xml_record(combined_child.unwrap_or(kind), start.name().as_ref()) {
                    records += 1;
                    enforce_xml_record_limit(input, options, kind, records)?;
                }
            }
            Ok(Event::End(_)) => {
                depth = depth.saturating_sub(1);
                if kind == crate::validation::MessageKind::Ndm && depth == 1 {
                    combined_child = None;
                }
            }
            Ok(Event::Eof) => return Ok(()),
            Ok(_) => {}
            Err(error) => return Err(error.into()),
        }
    }
}

fn is_xml_record(kind: crate::validation::MessageKind, name: &[u8]) -> bool {
    use crate::validation::MessageKind;
    match kind {
        MessageKind::Opm => name == b"maneuverParameters",
        MessageKind::Oem => matches!(name, b"stateVector" | b"covarianceMatrix"),
        MessageKind::Tdm => name == b"observation",
        MessageKind::Aem => name == b"attitudeState",
        MessageKind::Ocm => matches!(name, b"trajLine" | b"covLine" | b"manLine"),
        MessageKind::Acm => matches!(name, b"attLine" | b"covLine" | b"man"),
        _ => false,
    }
}

fn xml_message_kind(name: &[u8]) -> Option<crate::validation::MessageKind> {
    NdmKind::from_xml_root(name).map(NdmKind::message_kind)
}

fn enforce_xml_record_limit(
    input: &str,
    options: &ParseOptions,
    kind: crate::validation::MessageKind,
    records: usize,
) -> Result<()> {
    if let Some(limit) = options.max_records {
        if records > limit {
            return Err(record_limit_error(limit, records).with_parse_context(
                kind,
                crate::error::DiagnosticNotation::Xml,
                input,
                None,
            ));
        }
    }
    Ok(())
}
