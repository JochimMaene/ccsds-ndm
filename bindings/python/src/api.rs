// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::errors::ccsds_error_to_pyerr;
use ccsds_ndm::options::ParseOptions;
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::{Message, Notation};
use ccsds_ndm::{Ndm, Validate};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::path::Path;

/// Build an optional validated core value, surfacing a rejection as a Python exception
/// instead of panicking.
pub fn checked_optional<T>(
    value: Option<f64>,
    make: impl FnOnce(f64) -> ccsds_ndm::error::Result<T>,
) -> PyResult<Option<T>> {
    value.map(make).transpose().map_err(ccsds_error_to_pyerr)
}

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

fn expect_typed<T: FromMessage>(message: Message) -> PyResult<T> {
    let actual = message.kind();
    T::from_message_type(message).ok_or_else(|| {
        PyValueError::new_err(format!(
            "expected {}, found {}",
            T::KIND.as_str(),
            actual.as_str()
        ))
    })
}

pub fn parse_options(max_input_bytes: Option<usize>, max_records: Option<usize>) -> ParseOptions {
    ParseOptions {
        max_input_bytes,
        max_records,
        ..ParseOptions::default()
    }
}

pub fn parse_typed_with_options<T: FromMessage>(
    data: &str,
    format: Option<&str>,
    options: &ParseOptions,
) -> PyResult<T> {
    let message = ccsds_ndm::from_str_with_options(data, selected_notation(format)?, options)
        .map_err(ccsds_error_to_pyerr)?;
    expect_typed(message)
}

pub fn parse_typed_file_with_options<T: FromMessage>(
    path: &Path,
    format: Option<&str>,
    options: &ParseOptions,
) -> PyResult<T> {
    let message = ccsds_ndm::from_file_with_options(path, selected_notation(format)?, options)
        .map_err(ccsds_error_to_pyerr)?;
    expect_typed(message)
}

pub fn validate_message<T: Validate>(message: &T) -> PyResult<()> {
    message.validate().map_err(ccsds_error_to_pyerr)
}

pub fn generate_string<T: Ndm>(message: &T, format: &str) -> PyResult<String> {
    match notation(format)? {
        Notation::Kvn => message.to_kvn(),
        Notation::Xml => message.to_xml(),
    }
    .map_err(ccsds_error_to_pyerr)
}

pub fn generate_file(message: &Message, path: &Path, format: &str) -> PyResult<()> {
    match notation(format)? {
        Notation::Kvn => message.to_kvn_file(path),
        Notation::Xml => message.to_xml_file(path),
    }
    .map_err(ccsds_error_to_pyerr)
}

fn unsupported_format(format: &str) -> PyErr {
    PyValueError::new_err(format!("Unsupported format '{format}'. Use 'kvn' or 'xml'",))
}
