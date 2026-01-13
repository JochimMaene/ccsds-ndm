// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use thiserror::Error;
use winnow::error::{AddContext, ParserError, StrContext};
use winnow::stream::{Offset, Stream};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseDiagnostic {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub contexts: Vec<String>,
    pub snippet: String,
}

impl ParseDiagnostic {
    /// Creates a new diagnostic from an input string and byte offset.
    pub fn new(input: &str, offset: usize, message: impl Into<String>) -> Self {
        let offset = offset.min(input.len());
        let prefix = &input[..offset];
        let line = prefix.lines().count().max(1);

        let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let suffix = &input[offset..];
        let line_end = suffix.find('\n').map(|i| i + offset).unwrap_or(input.len());

        let line_text = &input[line_start..line_end];
        let column = prefix[line_start..].chars().count();
        let snippet = format!("{}\n{}^", line_text, " ".repeat(column));

        Self {
            line,
            column: column + 1,
            message: message.into(),
            contexts: Vec::new(),
            snippet,
        }
    }

    /// Adds contexts to the diagnostic.
    pub fn with_contexts(mut self, contexts: Vec<String>) -> Self {
        self.contexts = contexts;
        self
    }
}

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

    /// Errors occurring during KVN parsing.
    #[error("KVN parsing error at line {line}, column {column}: {message}\nContext: {}\n{snippet}", .contexts.join(" > "))]
    KvnParse {
        line: usize,
        column: usize,
        message: String,
        contexts: Vec<String>,
        snippet: String,
    },

    /// A required field was missing in the message.
    #[error("Missing required field: {field} in block {block}")]
    MissingRequiredField { block: String, field: String },

    /// Legacy variant for missing fields.
    #[error("Missing required KVN field: {0}")]
    MissingField(String),

    /// Legacy variant for missing segments.
    #[error("Missing required segment: {0}")]
    MissingSegment(String),

    /// Two or more fields are in conflict (e.g., SEMI_MAJOR_AXIS and MEAN_MOTION).
    #[error("Conflicting fields: {fields:?}")]
    Conflict { fields: Vec<String> },

    /// Legacy variant for conflicting fields.
    #[error("Conflicting fields: {0}")]
    ConflictingFields(String),

    /// A value was provided that does not match the CCSDS specification for that field.
    #[error("Invalid value for {field}: '{value}' (expected {expected})")]
    InvalidValue {
        field: String,
        value: String,
        expected: String,
    },

    /// Legacy variant for invalid CCSDS values.
    #[error("Invalid value for '{key}': '{value}' (expected {expected})")]
    InvalidCcsdsValue {
        key: String,
        value: String,
        expected: String,
    },

    /// Contextual error wrapping another error with a description.
    #[error("{context}: {source}")]
    Context {
        context: String,
        #[source]
        source: Box<CcsdsNdmError>,
    },

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

    /// General validation errors for cases not covered by specific variants.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Legacy variant for validation errors.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Specific validation error for values out of expected range.
    #[error("Value for '{name}' is out of range: {value} (expected {expected})")]
    OutOfRange {
        name: String,
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

    /// Error when an unexpected end of input is reached.
    #[error("Unexpected end of input: {context}")]
    UnexpectedEof { context: String },
}

impl ParserError<&str> for CcsdsNdmError {
    type Inner = ();
    fn from_input(input: &&str) -> Self {
        let diag = ParseDiagnostic::new(input, 0, "Parse error");
        CcsdsNdmError::KvnParse {
            line: diag.line,
            column: diag.column,
            message: diag.message,
            contexts: Vec::new(),
            snippet: diag.snippet,
        }
    }

    fn into_inner(self) -> std::result::Result<Self::Inner, Self> {
        Ok(())
    }
}

impl AddContext<&str, StrContext> for CcsdsNdmError {
    fn add_context(
        mut self,
        input: &&str,
        token: &<&str as Stream>::Checkpoint,
        context: StrContext,
    ) -> Self {
        if let CcsdsNdmError::KvnParse {
            ref mut line,
            ref mut column,
            ref mut message,
            ref mut contexts,
            ref mut snippet,
        } = self
        {
            let offset = input.offset_from(token);
            let diag = ParseDiagnostic::new(input, offset, "");
            *line = diag.line;
            *column = diag.column;
            *snippet = diag.snippet;

            match context {
                StrContext::Label(l) => contexts.push(l.to_string()),
                StrContext::Expected(e) => *message = format!("Expected {}", e),
                _ => {}
            }
        }
        self
    }
}

impl CcsdsNdmError {
    /// Wraps the error with line context.
    pub fn at_line(self, line: usize) -> Self {
        CcsdsNdmError::LineContext {
            line,
            source: Box::new(self),
        }
    }

    /// Wraps the error with a descriptive context.
    pub fn context<S: Into<String>>(self, context: S) -> Self {
        CcsdsNdmError::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Populates location information for KvnParse variants.
    pub fn with_location(mut self, input: &str, offset: usize) -> Self {
        if let CcsdsNdmError::KvnParse {
            ref mut line,
            ref mut column,
            ref mut snippet,
            ..
        } = self
        {
            let diag = ParseDiagnostic::new(input, offset, "");
            *line = diag.line;
            *column = diag.column;
            *snippet = diag.snippet;
        }
        self
    }
}

pub type Result<T> = std::result::Result<T, CcsdsNdmError>;
