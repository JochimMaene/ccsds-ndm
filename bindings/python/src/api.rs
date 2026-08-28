// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::errors::{ccsds_error_to_pyerr, file_parse_error_to_pyerr, NdmValidationError};
use ccsds_ndm::generation::VersionedNdm;
use ccsds_ndm::options::{GenerateOptions, ParseOptions};
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::{MessageType, Notation};
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::Path;

pub trait FromMessageType: Ndm {
    const KIND: ccsds_ndm::validation::MessageKind;

    fn from_message_type(message: MessageType) -> Option<Self>;
}

macro_rules! impl_from_message_type {
    ($type:path, $variant:ident, $kind:ident) => {
        impl FromMessageType for $type {
            const KIND: ccsds_ndm::validation::MessageKind =
                ccsds_ndm::validation::MessageKind::$kind;

            fn from_message_type(message: MessageType) -> Option<Self> {
                match message {
                    MessageType::$variant(message) => Some(message),
                    _ => None,
                }
            }
        }
    };
}

impl_from_message_type!(ccsds_ndm::messages::opm::Opm, Opm, Opm);
impl_from_message_type!(ccsds_ndm::messages::oem::Oem, Oem, Oem);
impl_from_message_type!(ccsds_ndm::messages::omm::Omm, Omm, Omm);
impl_from_message_type!(ccsds_ndm::messages::ocm::Ocm, Ocm, Ocm);
impl_from_message_type!(ccsds_ndm::messages::cdm::Cdm, Cdm, Cdm);
impl_from_message_type!(ccsds_ndm::messages::tdm::Tdm, Tdm, Tdm);
impl_from_message_type!(ccsds_ndm::messages::rdm::Rdm, Rdm, Rdm);
impl_from_message_type!(ccsds_ndm::messages::aem::Aem, Aem, Aem);
impl_from_message_type!(ccsds_ndm::messages::apm::Apm, Apm, Apm);
impl_from_message_type!(ccsds_ndm::messages::acm::Acm, Acm, Acm);
impl_from_message_type!(ccsds_ndm::messages::ndm::CombinedNdm, Ndm, Ndm);

fn selected_notation(format: Option<&str>) -> PyResult<Option<Notation>> {
    match format {
        Some("kvn") => Ok(Some(Notation::Kvn)),
        Some("xml") => Ok(Some(Notation::Xml)),
        Some(other) => Err(unsupported_format(other)),
        None => Ok(None),
    }
}

fn expect_typed<T: FromMessageType>(message: MessageType) -> PyResult<T> {
    T::from_message_type(message)
        .ok_or_else(|| PyValueError::new_err("input contains a different CCSDS NDM message type"))
}

pub fn parse_options(max_input_bytes: Option<usize>, max_records: Option<usize>) -> ParseOptions {
    ParseOptions {
        max_input_bytes,
        max_records,
        ..ParseOptions::default()
    }
}

pub fn parse_typed<T: FromMessageType>(
    _py: Python<'_>,
    data: &str,
    format: Option<&str>,
) -> PyResult<T> {
    parse_typed_with_options(data, format, &ParseOptions::default())
}

pub fn parse_typed_with_options<T: FromMessageType>(
    data: &str,
    format: Option<&str>,
    options: &ParseOptions,
) -> PyResult<T> {
    let message = ccsds_ndm::from_str_with_options(data, selected_notation(format)?, options)
        .map_err(ccsds_error_to_pyerr)?;
    expect_typed(message)
}

pub fn parse_typed_file_with_options<T: FromMessageType>(
    path: &str,
    format: Option<&str>,
    options: &ParseOptions,
) -> PyResult<T> {
    let notation = selected_notation(format)?;
    let message = ccsds_ndm::from_file_with_options(path, notation, options)
        .map_err(|error| file_parse_error_to_pyerr(error, notation, Some(T::KIND)))?;
    expect_typed(message)
}

pub fn validate_message<T: Validate>(message: &T) -> PyResult<()> {
    let errors = message.validation_errors().map_err(ccsds_error_to_pyerr)?;
    if errors.is_empty() {
        Ok(())
    } else {
        Err(NdmValidationError::new_err(
            errors
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        ))
    }
}

pub fn generate_options(version: Option<&str>) -> GenerateOptions {
    match version {
        None => GenerateOptions::source(),
        Some("latest") => GenerateOptions::latest(),
        Some(version) => GenerateOptions::version(version),
    }
}

pub fn generate_string<T: VersionedNdm>(
    message: &T,
    format: &str,
    version: Option<&str>,
) -> PyResult<String> {
    generate_string_with_limit(message, format, version, None)
}

pub fn generate_string_with_limit<T: VersionedNdm>(
    message: &T,
    format: &str,
    version: Option<&str>,
    max_output_bytes: Option<usize>,
) -> PyResult<String> {
    let mut options = generate_options(version);
    options.max_output_bytes = max_output_bytes;
    match format {
        "kvn" => message.to_kvn_with(&options).map_err(ccsds_error_to_pyerr),
        "xml" => message.to_xml_with(&options).map_err(ccsds_error_to_pyerr),
        other => Err(unsupported_format(other)),
    }
}

pub fn generate_file<T: VersionedNdm>(
    message: &T,
    path: &str,
    format: &str,
    version: Option<&str>,
) -> PyResult<()> {
    generate_file_with_limit(message, path, format, version, None)
}

pub fn generate_file_with_limit<T: VersionedNdm>(
    message: &T,
    path: &str,
    format: &str,
    version: Option<&str>,
    max_output_bytes: Option<usize>,
) -> PyResult<()> {
    let mut options = generate_options(version);
    options.max_output_bytes = max_output_bytes;
    if format != "kvn" && format != "xml" {
        return Err(unsupported_format(format));
    }

    atomic_write(path, |output| {
        match format {
            "kvn" => message.write_kvn_to(output, &options),
            "xml" => message.write_xml_to(output, &options),
            _ => unreachable!(),
        }
        .map_err(ccsds_error_to_pyerr)
    })
}

pub fn atomic_write(
    path: &str,
    write: impl FnOnce(&mut std::fs::File) -> PyResult<()>,
) -> PyResult<()> {
    let destination = Path::new(path);
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut builder = tempfile::Builder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions = destination
            .metadata()
            .map(|metadata| metadata.permissions())
            .unwrap_or_else(|_| std::fs::Permissions::from_mode(0o666));
        builder.permissions(permissions);
    }
    let mut output = builder
        .tempfile_in(parent)
        .map_err(|error| PyOSError::new_err(error.to_string()))?;

    write(output.as_file_mut())?;

    output
        .as_file_mut()
        .sync_all()
        .map_err(|error| PyOSError::new_err(error.to_string()))?;
    output
        .persist(destination)
        .map_err(|error| PyOSError::new_err(error.error.to_string()))?;
    Ok(())
}

pub fn unsupported_format(format: &str) -> PyErr {
    PyValueError::new_err(format!("Unsupported format '{format}'. Use 'kvn' or 'xml'",))
}
