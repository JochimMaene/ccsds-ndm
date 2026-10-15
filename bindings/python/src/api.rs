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

pub trait FromMessage: Ndm {
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

/// Reject a message of another family; `origin` names the file it was read from, if any.
fn expect_typed<T: FromMessage>(message: Message, origin: Option<&Path>) -> PyResult<T> {
    let actual = message.kind();
    T::from_message_type(message).ok_or_else(|| {
        let found = format!("expected {}, found {}", T::KIND.as_str(), actual.as_str());
        crate::errors::NdmUnsupportedMessageError::new_err(match origin {
            Some(path) => format!("{}: {found}", path.display()),
            None => found,
        })
    })
}

// The Rust core holds no Python objects, so parsing, validation, generation, and file I/O run
// with the GIL released; other Python threads progress meanwhile. Converting between Python
// objects and the core model still needs the GIL and happens in the callers.

pub fn parse_typed<T: FromMessage>(
    py: Python<'_>,
    data: &str,
    format: Option<&str>,
) -> PyResult<T> {
    let notation = selected_notation(format)?;
    let message = py
        .detach(|| ccsds_ndm::from_str_with_notation(data, notation))
        .map_err(ccsds_error_to_pyerr)?;
    expect_typed(message, None)
}

pub fn parse_typed_file<T: FromMessage>(
    py: Python<'_>,
    path: &Path,
    format: Option<&str>,
) -> PyResult<T> {
    let notation = selected_notation(format)?;
    let message = py
        .detach(|| ccsds_ndm::from_file_with_notation(path, notation))
        .map_err(|error| {
            crate::errors::file_parse_error_to_pyerr(error, Some(path), notation, Some(T::KIND))
        })?;
    expect_typed(message, Some(path))
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
