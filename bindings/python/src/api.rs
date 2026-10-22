// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::errors::ccsds_error_to_pyerr;
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::{Message, Notation};
use ccsds_ndm::{Ndm, Validate};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::path::Path;

pub trait FromMessage: Ndm + Send {
    const KIND: MessageKind;
    fn from_message_type(message: Message) -> Option<Self>;
}

macro_rules! impl_from_message_type {
    ($type:path, $variant:ident) => {
        impl FromMessage for $type {
            const KIND: MessageKind = MessageKind::$variant;
            fn from_message_type(message: Message) -> Option<Self> {
                match message {
                    Message::$variant(message) => Some(message),
                    _ => None,
                }
            }
        }
    };
}

impl_from_message_type!(ccsds_ndm::messages::opm::Opm, Opm);
impl_from_message_type!(ccsds_ndm::messages::oem::Oem, Oem);
impl_from_message_type!(ccsds_ndm::messages::omm::Omm, Omm);
impl_from_message_type!(ccsds_ndm::messages::ocm::Ocm, Ocm);
impl_from_message_type!(ccsds_ndm::messages::cdm::Cdm, Cdm);
impl_from_message_type!(ccsds_ndm::messages::tdm::Tdm, Tdm);
impl_from_message_type!(ccsds_ndm::messages::rdm::Rdm, Rdm);
impl_from_message_type!(ccsds_ndm::messages::aem::Aem, Aem);
impl_from_message_type!(ccsds_ndm::messages::apm::Apm, Apm);
impl_from_message_type!(ccsds_ndm::messages::acm::Acm, Acm);
impl_from_message_type!(ccsds_ndm::messages::ndm::CombinedNdm, Ndm);

fn selected_notation(format: Option<&str>) -> PyResult<Option<Notation>> {
    format.map(notation).transpose()
}

pub fn notation(format: &str) -> PyResult<Notation> {
    if format.eq_ignore_ascii_case("kvn") {
        Ok(Notation::Kvn)
    } else if format.eq_ignore_ascii_case("xml") {
        Ok(Notation::Xml)
    } else {
        Err(unsupported_format(format))
    }
}

/// Check the family before parsing its contents, so a malformed other family is still a mismatch.
fn expect_kind<T: FromMessage>(
    py: Python<'_>,
    data: &str,
    notation: Option<Notation>,
    origin: Option<&Path>,
) -> PyResult<()> {
    let actual = py
        .detach(|| {
            let actual = ccsds_ndm::detect::detect_message_header(data, notation)?;
            if actual != T::KIND {
                // Keep concatenated-input refusals ahead of wrong-family errors.
                ccsds_ndm::detect::detect_message_kind(data, notation)?;
            }
            Ok::<_, ccsds_ndm::error::CcsdsNdmError>(actual)
        })
        .map_err(ccsds_error_to_pyerr)?;
    if actual == T::KIND {
        return Ok(());
    }
    let found = format!("expected {}, found {}", T::KIND.as_str(), actual.as_str());
    let error = crate::errors::NdmUnsupportedMessageError::new_err(match origin {
        Some(path) => format!("{}: {found}", path.display()),
        None => found,
    });
    Err(crate::errors::enrich_exception(
        error,
        "parse",
        notation.map(|notation| match notation {
            Notation::Kvn => ccsds_ndm::error::DiagnosticNotation::Kvn,
            Notation::Xml => ccsds_ndm::error::DiagnosticNotation::Xml,
        }),
        Some(T::KIND.as_str()),
        None,
        Some("parse.unexpected_message_type"),
        None,
        None,
        None,
        None,
        None,
        None,
    ))
}

// The Rust core holds no Python objects, so parsing, validation, generation, and file I/O run
// with the GIL released; other Python threads progress meanwhile. Converting between Python
// objects and the core model still needs the GIL and happens in the callers.

pub fn parse_typed<T: FromMessage>(
    py: Python<'_>,
    data: &str,
    format: Option<&str>,
) -> PyResult<T> {
    parse_typed_input(py, data, selected_notation(format)?, None)
}

fn parse_typed_input<T: FromMessage>(
    py: Python<'_>,
    data: &str,
    notation: Option<Notation>,
    origin: Option<&Path>,
) -> PyResult<T> {
    expect_kind::<T>(py, data, notation, origin)?;
    let message = py
        .detach(|| ccsds_ndm::from_str_with_notation(data, notation))
        .map_err(ccsds_error_to_pyerr)?;
    T::from_message_type(message).ok_or_else(|| {
        ccsds_error_to_pyerr(ccsds_ndm::error::CcsdsNdmError::UnsupportedMessage(
            "parsed message family differs from the requested type".into(),
        ))
    })
}

pub fn parse_typed_file<T: FromMessage>(
    py: Python<'_>,
    path: &Path,
    format: Option<&str>,
) -> PyResult<T> {
    let notation = selected_notation(format)?;
    let bytes = py
        .detach(|| std::fs::read(path))
        .map_err(|error| crate::errors::file_error_to_pyerr(error.into(), path))?;
    let data = py
        .detach(|| ccsds_ndm::detect::decode_utf8(&bytes))
        .map_err(|error| crate::errors::file_error_to_pyerr(error, path))?;
    parse_typed_input::<T>(py, data, notation, Some(path))
}

pub fn validate_message<T: Validate + Sync>(py: Python<'_>, message: &T) -> PyResult<()> {
    py.detach(|| message.validate())
        .map_err(ccsds_error_to_pyerr)
}

pub fn generate_string<T: Ndm + Sync>(
    py: Python<'_>,
    message: &T,
    format: &str,
) -> PyResult<String> {
    let notation = notation(format)?;
    py.detach(|| match notation {
        Notation::Kvn => message.to_kvn(),
        Notation::Xml => message.to_xml(),
    })
    .map_err(ccsds_error_to_pyerr)
}

pub fn generate_file(py: Python<'_>, message: &Message, path: &Path, format: &str) -> PyResult<()> {
    let notation = notation(format)?;
    py.detach(|| match notation {
        Notation::Kvn => message.to_kvn_file(path),
        Notation::Xml => message.to_xml_file(path),
    })
    .map_err(|error| crate::errors::file_error_to_pyerr(error, path))
}

fn unsupported_format(format: &str) -> PyErr {
    PyValueError::new_err(format!("Unsupported format '{format}'. Use 'kvn' or 'xml'",))
}
