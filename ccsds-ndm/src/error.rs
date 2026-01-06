// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CcsdsNdmError {
    /// Errors occurring during I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors occurring during XML deserialization.
    #[error("XML deserialization error: {0}")]
    XmlDe(#[from] quick_xml::DeError),

    /// Errors occurring during XML serialization.
    #[error("XML serialization error: {0}")]
    XmlSer(#[from] quick_xml::se::SeError),

    /// Errors occurring during XML parsing (low-level).
    #[error("XML parsing error: {0}")]
    XmlParse(#[from] quick_xml::Error),

    /// Errors occurring during KVN parsing at a specific line.
    #[error("KVN parsing error at line {line}: {message}")]
    KvnParse { line: usize, message: String },

    /// Contextual error wrapping another error with a line number.
    #[error("Error at line {line}: {source}")]
    LineContext {
        line: usize,
        #[source]
        source: Box<CcsdsNdmError>,
    },

    /// Errors related to CCSDS Epochs.
    #[error("Epoch error: {0}")]
    Epoch(#[from] EpochError),

    /// Validation errors when data does not meet CCSDS requirements.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Specific validation error for values out of expected range.
    #[error("Value for '{name}' is out of range: {value} (expected {expected})")]
    OutOfRange {
        name: String,
        value: String,
        expected: String,
    },

    /// Error when a value is invalid for a specific field.
    #[error("Invalid value for '{key}': '{value}' (expected {expected})")]
    InvalidCcsdsValue {
        key: String,
        value: String,
        expected: String,
    },

    /// Error for unsupported CCSDS message types.
    #[error("Unsupported message type: {0}")]
    UnsupportedMessage(String),

    /// Error when a unit string is not recognized.
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    /// Error when the format of a value or segment is invalid.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Error when parsing a floating point number fails.
    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    /// Error when parsing an integer number fails.
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    /// Error when a required field is missing in KVN.
    #[error("Missing required KVN field: {0}")]
    MissingField(String),

    /// Error when a required segment is missing.
    #[error("Missing required segment: {0}")]
    MissingSegment(String),

    /// Error when fields are conflicting.
    #[error("Conflicting fields: {0}")]
    ConflictingFields(String),

    /// Error when an unexpected end of input is reached.
    #[error("Unexpected end of input: {context}")]
    UnexpectedEof { context: String },
}

impl CcsdsNdmError {
    /// Wraps the error with line context.
    pub fn at_line(self, line: usize) -> Self {
        CcsdsNdmError::LineContext {
            line,
            source: Box::new(self),
        }
    }
}

pub type Result<T> = std::result::Result<T, CcsdsNdmError>;
