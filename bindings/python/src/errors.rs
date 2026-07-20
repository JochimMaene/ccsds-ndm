// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Python exception types for CCSDS NDM errors.
//!
//! This module defines custom Python exception classes that map to the Rust
//! `CcsdsNdmError` hierarchy. This allows Python consumers to catch specific
//! error types for more granular error handling.

use ccsds_ndm::error::{CcsdsNdmError, DiagnosticNotation, FormatError};
use ccsds_ndm::validation::MessageKind;
use ccsds_ndm::Notation;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyOSError, PyValueError};
use pyo3::prelude::*;

// Generic library error for failures without a more useful built-in category.
create_exception!(ccsds_ndm, NdmError, PyException, "Generic CCSDS NDM error.");

// Format/parsing errors.
create_exception!(
    ccsds_ndm,
    NdmFormatError,
    PyValueError,
    "Error during parsing of NDM data (KVN or XML)."
);
create_exception!(
    ccsds_ndm,
    NdmKvnParseError,
    NdmFormatError,
    "Error during KVN parsing."
);
create_exception!(
    ccsds_ndm,
    NdmXmlError,
    NdmFormatError,
    "Error during XML parsing or serialization."
);

// Validation errors.
create_exception!(
    ccsds_ndm,
    NdmValidationError,
    PyValueError,
    "Validation error against CCSDS rules."
);

// Epoch errors.
create_exception!(
    ccsds_ndm,
    NdmEpochError,
    PyValueError,
    "Error parsing a CCSDS epoch string."
);

// I/O errors.
create_exception!(
    ccsds_ndm,
    NdmIoError,
    PyOSError,
    "I/O error during file operations."
);

// Unsupported message type.
create_exception!(
    ccsds_ndm,
    NdmUnsupportedMessageError,
    PyValueError,
    "Unsupported CCSDS message type."
);

/// Converts a `CcsdsNdmError` into a `PyErr`.
///
/// This function maps each variant of the Rust error enum to the corresponding
/// Python exception type.
pub fn ccsds_error_to_pyerr(e: CcsdsNdmError) -> PyErr {
    match e {
        CcsdsNdmError::Generation { context, source } => {
            let code = source.code();
            let field_path = source.field_path();
            let error = ccsds_error_to_pyerr(*source);
            enrich_exception(
                error,
                "generate",
                Some(context.notation),
                Some(context.message_kind.as_str()),
                Some(context.source_edition),
                Some(context.target_edition),
                code,
                field_path,
                None,
                None,
                None,
                None,
                None,
            )
        }
        CcsdsNdmError::Parsing { context, source } => {
            let code = if matches!(
                source.as_format_error(),
                Some(FormatError::InvalidFormat(_))
            ) {
                Some(match context.notation {
                    DiagnosticNotation::Kvn => "parse.kvn.syntax",
                    DiagnosticNotation::Xml => "parse.xml.syntax",
                })
            } else {
                source.code()
            };
            let field_path = source.field_path();
            let error = ccsds_error_to_pyerr(*source);
            enrich_exception(
                error,
                "parse",
                Some(context.notation),
                Some(context.message_kind.as_str()),
                context.source_edition,
                None,
                code,
                field_path,
                context.line,
                context.column,
                context.byte_offset,
                context.original_token,
                context.expected,
            )
        }
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
        CcsdsNdmError::Validation(val_err) => {
            let code = val_err.code();
            let field_path = val_err.field_path();
            let error = NdmValidationError::new_err(val_err.to_string());
            enrich_exception(
                error,
                "validate",
                None,
                None,
                None,
                None,
                code,
                field_path,
                None,
                None,
                None,
                None,
                None,
            )
        }
        CcsdsNdmError::Epoch(epoch_err) => NdmEpochError::new_err(epoch_err.to_string()),
        CcsdsNdmError::UnsupportedMessage(msg) => NdmUnsupportedMessageError::new_err(msg),
        CcsdsNdmError::UnsupportedInputVersion {
            message_type,
            version,
            supported,
        } => enrich_exception(
            NdmValidationError::new_err(format!(
                "Unsupported input version {version} for {message_type}; supported versions: {supported}"
            )),
            "parse",
            None,
            Some(message_type),
            Some(version),
            None,
            Some("parse.unsupported_input_version"),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CcsdsNdmError::UnsupportedOutputVersion {
            message_type,
            format,
            version,
            supported,
        } => {
            let notation = match format.to_ascii_lowercase().as_str() {
                "kvn" => Some(DiagnosticNotation::Kvn),
                "xml" => Some(DiagnosticNotation::Xml),
                _ => None,
            };
            enrich_exception(
                NdmValidationError::new_err(format!(
                    "Unsupported {format} output version {version} for {message_type}; supported versions: {supported}"
                )),
                "generate",
                notation,
                Some(message_type),
                Some(version.clone()),
                Some(version),
                Some("generation.unsupported_output_version"),
                None,
                None,
                None,
                None,
                None,
                None,
            )
        }
        CcsdsNdmError::UnsupportedVersionConversion {
            message_type,
            source_version,
            target_version,
        } => enrich_exception(
            NdmValidationError::new_err(format!(
                "Unsupported version conversion for {message_type}: \
                 {source_version} to {target_version}"
            )),
            "generate",
            None,
            Some(message_type),
            Some(source_version),
            Some(target_version),
            Some("generation.unsupported_version_conversion"),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CcsdsNdmError::UnexpectedEof { context } => {
            NdmFormatError::new_err(format!("Unexpected end of input: {}", context))
        }
        error @ CcsdsNdmError::ResourceLimitExceeded { .. } => {
            let code = error.code();
            let operation = match &error {
                CcsdsNdmError::ResourceLimitExceeded {
                    resource: "generated_document",
                    ..
                } => "generate",
                _ => "parse",
            };
            enrich_exception(
                NdmError::new_err(error.to_string()),
                operation,
                None,
                None,
                None,
                None,
                code,
                None,
                None,
                None,
                None,
                None,
                None,
            )
        }
        _ => NdmError::new_err(e.to_string()),
    }
}

/// Add the context known by a file entry point when its bounded read fails before detection.
pub fn file_parse_error_to_pyerr(
    error: CcsdsNdmError,
    notation: Option<Notation>,
    message_kind: Option<MessageKind>,
) -> PyErr {
    if !matches!(&error, CcsdsNdmError::ResourceLimitExceeded { .. }) {
        return ccsds_error_to_pyerr(error);
    }

    let code = error.code();
    enrich_exception(
        NdmError::new_err(error.to_string()),
        "parse",
        notation.map(|notation| match notation {
            Notation::Kvn => DiagnosticNotation::Kvn,
            Notation::Xml => DiagnosticNotation::Xml,
        }),
        message_kind.map(MessageKind::as_str),
        None,
        None,
        code,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn enrich_exception(
    error: PyErr,
    operation: &'static str,
    notation: Option<DiagnosticNotation>,
    message_kind: Option<&'static str>,
    source_edition: Option<String>,
    target_edition: Option<String>,
    code: Option<&'static str>,
    field_path: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
    byte_offset: Option<usize>,
    original_token: Option<String>,
    expected: Option<&'static str>,
) -> PyErr {
    let notation = notation.map(|notation| match notation {
        DiagnosticNotation::Kvn => "kvn",
        DiagnosticNotation::Xml => "xml",
    });
    Python::attach(|py| {
        let value = error.value(py);
        let _ = value.setattr("severity", "error");
        let _ = value.setattr("operation", operation);
        let _ = value.setattr("notation", notation);
        let _ = value.setattr(
            "message_kind",
            message_kind.map(|kind| kind.to_ascii_lowercase()),
        );
        let _ = value.setattr("source_edition", source_edition);
        let _ = value.setattr("target_edition", target_edition);
        let _ = value.setattr("code", code);
        let _ = value.setattr("field_path", field_path);
        let _ = value.setattr("line", line);
        let _ = value.setattr("column", column);
        let _ = value.setattr("byte_offset", byte_offset);
        let _ = value.setattr("original_token", original_token);
        let _ = value.setattr("expected", expected);
        let _ = value.setattr("requirement", Option::<&str>::None);
        let _ = value.setattr("recovery", Option::<&str>::None);
    });
    error
}

/// Registers the exception classes with the Python module.
pub fn register_exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("NdmError", m.py().get_type::<NdmError>())?;
    m.add("NdmFormatError", m.py().get_type::<NdmFormatError>())?;
    m.add("NdmKvnParseError", m.py().get_type::<NdmKvnParseError>())?;
    m.add("NdmXmlError", m.py().get_type::<NdmXmlError>())?;
    m.add(
        "NdmValidationError",
        m.py().get_type::<NdmValidationError>(),
    )?;
    m.add("NdmEpochError", m.py().get_type::<NdmEpochError>())?;
    m.add("NdmIoError", m.py().get_type::<NdmIoError>())?;
    m.add(
        "NdmUnsupportedMessageError",
        m.py().get_type::<NdmUnsupportedMessageError>(),
    )?;
    Ok(())
}
