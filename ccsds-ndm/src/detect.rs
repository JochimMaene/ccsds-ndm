// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::Message;
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

fn is_kvn_message_header(line: &str) -> bool {
    KVN_HEADERS.iter().any(|header| {
        line.strip_prefix(header).is_some_and(|rest| {
            rest.starts_with('=') || rest.as_bytes().first().is_some_and(u8::is_ascii_whitespace)
        })
    })
}

fn has_multiple_kvn_messages(input: &str) -> bool {
    input
        .lines()
        .filter(|line| is_kvn_message_header(line))
        .nth(1)
        .is_some()
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

pub(crate) fn detect_message_type(s: &str) -> Result<Message> {
    detect_message_type_as(s, None)
}

pub(crate) fn detect_message_type_as(s: &str, notation: Option<Notation>) -> Result<Message> {
    let input = without_utf8_bom(s);
    let notation = notation.map_or_else(|| detect_notation(input), Ok)?;
    let result = match notation {
        Notation::Kvn => detect_kvn_type(input),
        Notation::Xml => detect_xml_type(input),
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

    fn parse_kvn(self, input: &str) -> Result<Message> {
        match self {
            Self::Opm => crate::traits::Ndm::from_kvn(input).map(Message::Opm),
            Self::Omm => crate::traits::Ndm::from_kvn(input).map(Message::Omm),
            Self::Oem => crate::traits::Ndm::from_kvn(input).map(Message::Oem),
            Self::Ocm => crate::traits::Ndm::from_kvn(input).map(Message::Ocm),
            Self::Acm => crate::traits::Ndm::from_kvn(input).map(Message::Acm),
            Self::Cdm => crate::traits::Ndm::from_kvn(input).map(Message::Cdm),
            Self::Tdm => crate::traits::Ndm::from_kvn(input).map(Message::Tdm),
            Self::Rdm => crate::traits::Ndm::from_kvn(input).map(Message::Rdm),
            Self::Aem => crate::traits::Ndm::from_kvn(input).map(Message::Aem),
            Self::Apm => crate::traits::Ndm::from_kvn(input).map(Message::Apm),
            Self::Ndm => Err(CcsdsNdmError::UnsupportedMessage(
                "combined NDM is detected from multiple KVN headers".into(),
            )),
        }
    }

    fn parse_xml(self, input: &str) -> Result<Message> {
        match self {
            Self::Opm => crate::traits::Ndm::from_xml(input).map(Message::Opm),
            Self::Oem => crate::traits::Ndm::from_xml(input).map(Message::Oem),
            Self::Omm => crate::traits::Ndm::from_xml(input).map(Message::Omm),
            Self::Ocm => crate::traits::Ndm::from_xml(input).map(Message::Ocm),
            Self::Acm => crate::traits::Ndm::from_xml(input).map(Message::Acm),
            Self::Cdm => crate::traits::Ndm::from_xml(input).map(Message::Cdm),
            Self::Tdm => crate::traits::Ndm::from_xml(input).map(Message::Tdm),
            Self::Rdm => crate::traits::Ndm::from_xml(input).map(Message::Rdm),
            Self::Aem => crate::traits::Ndm::from_xml(input).map(Message::Aem),
            Self::Apm => crate::traits::Ndm::from_xml(input).map(Message::Apm),
            Self::Ndm => crate::traits::Ndm::from_xml(input).map(Message::Ndm),
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
fn detect_kvn_type(s: &str) -> Result<Message> {
    // We need a mutable slice for winnow, but we don't want to consume "s" for the final parsing.
    let mut input = s;
    let kind = parse_kvn_kind
        .parse_next(&mut input)
        .map_err(|_| CcsdsNdmError::UnsupportedMessage("Could not identify KVN header".into()))?;

    if has_multiple_kvn_messages(s) {
        return Err(crate::messages::ndm::combined_kvn_unsupported());
    }

    let result = kind.parse_kvn(s);
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

fn detect_xml_type(s: &str) -> Result<Message> {
    let mut reader = Reader::from_str(s);
    reader.config_mut().trim_text_start = true;
    reader.config_mut().trim_text_end = true;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e) | Event::Empty(e)) => {
                let root_name = e.name();
                let kind = NdmKind::from_xml_root(root_name.as_ref()).ok_or_else(|| {
                    CcsdsNdmError::UnsupportedMessage(format!(
                        "Unknown or unsupported XML root tag: <{}>",
                        String::from_utf8_lossy(root_name.as_ref())
                    ))
                })?;
                let message_kind = kind.message_kind();
                return with_xml_parse_context(kind.parse_xml(s), message_kind, s);
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
