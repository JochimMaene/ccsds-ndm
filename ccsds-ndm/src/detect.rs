// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::MessageType;
use winnow::ascii::multispace1;
use winnow::combinator::{alt, repeat};
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::token::take_till;

type PResult<T> = std::result::Result<T, ErrMode<ContextError>>;

/// Detects the NDM message type from the input string (KVN or XML).
pub fn detect_message_type(s: &str) -> Result<MessageType> {
    let trimmed = s.trim_start();

    if trimmed.is_empty() {
        return Err(CcsdsNdmError::UnexpectedEof {
            context: "Empty input".into(),
        });
    }

    // XML Detection: Look for '<' at the start of the first non-whitespace content
    if trimmed.starts_with('<') {
        return detect_xml_type(s);
    }

    // KVN Detection
    detect_kvn_type(s)
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
        "CCSDS_OPM_VERS".value(NdmKind::Opm),
        "CCSDS_OMM_VERS".value(NdmKind::Omm),
        "CCSDS_OEM_VERS".value(NdmKind::Oem),
        "CCSDS_OCM_VERS".value(NdmKind::Ocm),
        "CCSDS_ACM_VERS".value(NdmKind::Acm),
        "CCSDS_CDM_VERS".value(NdmKind::Cdm),
        "CCSDS_TDM_VERS".value(NdmKind::Tdm),
        "CCSDS_RDM_VERS".value(NdmKind::Rdm),
        "CCSDS_AEM_VERS".value(NdmKind::Aem),
        "CCSDS_APM_VERS".value(NdmKind::Apm),
    ))
    .parse_next(input)
}

/// Detects and parses KVN message type
fn detect_kvn_type(s: &str) -> Result<MessageType> {
    // We need a mutable slice for winnow, but we don't want to consume "s" for the final parsing.
    let mut input = s;
    let kind = parse_kvn_kind
        .parse_next(&mut input)
        .map_err(|_| CcsdsNdmError::UnsupportedMessage("Could not identify KVN header".into()))?;

    // Simple heuristic: count occurrences of "CCSDS_..._VERS"
    let headers = [
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

    let mut count = 0;
    for header in headers {
        count += s.matches(header).count();
        if count > 1 {
            // Found multiple headers -> CombinedNdm
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Ndm);
        }
    }

    match kind {
        NdmKind::Opm => crate::traits::Ndm::from_kvn(s).map(MessageType::Opm),
        NdmKind::Omm => crate::traits::Ndm::from_kvn(s).map(MessageType::Omm),
        NdmKind::Oem => crate::traits::Ndm::from_kvn(s).map(MessageType::Oem),
        NdmKind::Ocm => crate::traits::Ndm::from_kvn(s).map(MessageType::Ocm),
        NdmKind::Acm => crate::traits::Ndm::from_kvn(s).map(MessageType::Acm),
        NdmKind::Cdm => crate::traits::Ndm::from_kvn(s).map(MessageType::Cdm),
        NdmKind::Tdm => crate::traits::Ndm::from_kvn(s).map(MessageType::Tdm),
        NdmKind::Rdm => crate::traits::Ndm::from_kvn(s).map(MessageType::Rdm),
        NdmKind::Aem => crate::traits::Ndm::from_kvn(s).map(MessageType::Aem),
        NdmKind::Apm => crate::traits::Ndm::from_kvn(s).map(MessageType::Apm),
    }
}

// XML Detection
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn detect_xml_type(s: &str) -> Result<MessageType> {
    let mut reader = Reader::from_str(s);
    reader.config_mut().trim_text_start = true;
    reader.config_mut().trim_text_end = true;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name_bytes = e.name();
                let name = String::from_utf8_lossy(name_bytes.as_ref()).to_lowercase();

                return match name.as_str() {
                    "oem" => crate::traits::Ndm::from_xml(s).map(MessageType::Oem),
                    "cdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Cdm),
                    "opm" => crate::traits::Ndm::from_xml(s).map(MessageType::Opm),
                    "omm" => crate::traits::Ndm::from_xml(s).map(MessageType::Omm),
                    "rdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Rdm),
                    "tdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Tdm),
                    "ocm" => crate::traits::Ndm::from_xml(s).map(MessageType::Ocm),
                    "acm" => crate::traits::Ndm::from_xml(s).map(MessageType::Acm),
                    "aem" => crate::traits::Ndm::from_xml(s).map(MessageType::Aem),
                    "apm" => crate::traits::Ndm::from_xml(s).map(MessageType::Apm),
                    "ndm" => crate::traits::Ndm::from_xml(s).map(MessageType::Ndm),
                    _ => Err(CcsdsNdmError::UnsupportedMessage(format!(
                        "Unknown or unsupported XML root tag: <{}>",
                        name
                    ))),
                };
            }
            Ok(Event::Decl(_))
            | Ok(Event::Comment(_))
            | Ok(Event::DocType(_))
            | Ok(Event::PI(_)) => {
                continue;
            }
            Ok(Event::Eof) => {
                return Err(CcsdsNdmError::UnexpectedEof {
                    context: "XML parsing ended without finding root tag".into(),
                });
            }
            Err(e) => return Err(CcsdsNdmError::from(e)),
            _ => continue,
        }
    }
}
