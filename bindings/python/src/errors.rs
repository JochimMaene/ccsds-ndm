// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Python exception types for CCSDS NDM errors.
//!
//! This module defines custom Python exception classes that map to the Rust
//! `CcsdsNdmError` hierarchy. This allows Python consumers to catch specific
//! error types for more granular error handling.

use ccsds_ndm::error::CcsdsNdmError;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyIOError, PyValueError};
use pyo3::prelude::*;

// Base exception for all CCSDS NDM errors.
// Inherits from Exception.
create_exception!(ccsds_ndm, NdmError, PyException, "Base exception for all CCSDS NDM errors.");

// Format/parsing errors - inherit from both NdmError and ValueError for backward compat.
create_exception!(ccsds_ndm, NdmFormatError, PyValueError, "Error during parsing of NDM data (KVN or XML).");
create_exception!(ccsds_ndm, NdmKvnParseError, NdmFormatError, "Error during KVN parsing.");
create_exception!(ccsds_ndm, NdmXmlError, NdmFormatError, "Error during XML parsing or serialization.");

// Validation errors.
create_exception!(ccsds_ndm, NdmValidationError, NdmError, "Validation error against CCSDS rules.");

// Epoch errors.
create_exception!(ccsds_ndm, NdmEpochError, PyValueError, "Error parsing a CCSDS epoch string.");

// I/O errors - inherit from both NdmError and IOError.
create_exception!(ccsds_ndm, NdmIoError, PyIOError, "I/O error during file operations.");

// Unsupported message type.
create_exception!(ccsds_ndm, NdmUnsupportedMessageError, NdmError, "Unsupported CCSDS message type.");

/// Converts a `CcsdsNdmError` into a `PyErr`.
///
/// This function maps each variant of the Rust error enum to the corresponding
/// Python exception type.
pub fn ccsds_error_to_pyerr(e: CcsdsNdmError) -> PyErr {
    match e {
        CcsdsNdmError::Io(io_err) => NdmIoError::new_err(io_err.to_string()),
        CcsdsNdmError::Format(format_err) => {
            use ccsds_ndm::error::FormatError;
            match *format_err {
                FormatError::Kvn(kvn_err) => NdmKvnParseError::new_err(kvn_err.to_string()),
                FormatError::Xml(xml_err) => NdmXmlError::new_err(xml_err.to_string()),
                FormatError::XmlDe(xml_de_err) => NdmXmlError::new_err(xml_de_err.to_string()),
                FormatError::XmlSer(xml_ser_err) => NdmXmlError::new_err(xml_ser_err.to_string()),
                FormatError::XmlWithContext { context, source } => {
                    NdmXmlError::new_err(format!("{}: {}", context, source))
                }
                FormatError::ParseFloat(pf_err) => NdmFormatError::new_err(pf_err.to_string()),
                FormatError::ParseInt(pi_err) => NdmFormatError::new_err(pi_err.to_string()),
                FormatError::Enum(enum_err) => NdmFormatError::new_err(enum_err.to_string()),
                FormatError::InvalidFormat(msg) => NdmFormatError::new_err(msg),
                _ => NdmFormatError::new_err(format_err.to_string()),
            }
        }
        CcsdsNdmError::Validation(val_err) => NdmValidationError::new_err(val_err.to_string()),
        CcsdsNdmError::Epoch(epoch_err) => NdmEpochError::new_err(epoch_err.to_string()),
        CcsdsNdmError::UnsupportedMessage(msg) => NdmUnsupportedMessageError::new_err(msg),
        CcsdsNdmError::UnexpectedEof { context } => {
            NdmFormatError::new_err(format!("Unexpected end of input: {}", context))
        }
        _ => NdmError::new_err(e.to_string()),
    }
}

/// Registers the exception classes with the Python module.
pub fn register_exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("NdmError", m.py().get_type::<NdmError>())?;
    m.add("NdmFormatError", m.py().get_type::<NdmFormatError>())?;
    m.add("NdmKvnParseError", m.py().get_type::<NdmKvnParseError>())?;
    m.add("NdmXmlError", m.py().get_type::<NdmXmlError>())?;
    m.add("NdmValidationError", m.py().get_type::<NdmValidationError>())?;
    m.add("NdmEpochError", m.py().get_type::<NdmEpochError>())?;
    m.add("NdmIoError", m.py().get_type::<NdmIoError>())?;
    m.add(
        "NdmUnsupportedMessageError",
        m.py().get_type::<NdmUnsupportedMessageError>(),
    )?;
    Ok(())
}
