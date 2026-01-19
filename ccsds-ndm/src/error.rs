// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use thiserror::Error;
use winnow::error::{AddContext, ParserError, StrContext};
use winnow::stream::Stream;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseDiagnostic {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub contexts: Vec<&'static str>,
    pub snippet: String,
}

impl ParseDiagnostic {
    /// Creates a new diagnostic from an input string and byte offset.
    pub fn new(input: &str, offset: usize, message: impl Into<String>) -> Self {
        let offset = offset.min(input.len());
        let prefix = &input[..offset];
        let line = prefix.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;

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
    pub fn with_contexts(mut self, contexts: Vec<&'static str>) -> Self {
        self.contexts = contexts;
        self
    }
}

/// Detailed error information for KVN parsing failures.
#[derive(Debug, Clone, PartialEq, Error)]
pub struct KvnParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub contexts: Vec<&'static str>,
    pub snippet: String,
    pub offset: usize, // Track raw offset for lazy location calculation
}

impl std::fmt::Display for KvnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "KVN parsing error at line {}, column {}: {}",
            self.line, self.column, self.message
        )?;
        writeln!(f, "Context: {}", self.contexts.join(" > "))?;
        write!(f, "{}", self.snippet)
    }
}

/// Lightweight error for enum string conversion.
#[derive(Debug, Clone, PartialEq, Error)]
#[error("Invalid value '{value}' for field '{field}'; expected one of: {expected}")]
pub struct EnumParseError {
    pub field: &'static str,
    pub value: String,
    pub expected: &'static str,
}

/// Errors related to the physical format or syntax of the NDM.
#[derive(Debug, Error)]
pub enum FormatError {
    /// Errors occurring during KVN parsing.
    #[error(transparent)]
    Kvn(#[from] Box<KvnParseError>),

    /// Errors occurring during XML parsing.
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// Errors occurring during XML deserialization.
    #[error("XML deserialization error: {0}")]
    XmlDe(#[from] quick_xml::DeError),

    /// Errors occurring during XML serialization.
    #[error("XML serialization error: {0}")]
    XmlSer(#[from] quick_xml::se::SeError),

    /// Error when parsing a floating point number fails.
    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    /// Error when parsing an integer number fails.
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    /// Error during enum parsing.
    #[error(transparent)]
    Enum(#[from] EnumParseError),

    /// Error when the format of a value or segment is invalid.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

/// Errors related to the validation of NDM data against CCSDS rules.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    /// A required field was missing in the message.
    #[error("Missing required field: {field} in block {block}")]
    MissingRequiredField {
        block: String,
        field: String,
        line: Option<usize>,
    },

    /// Two or more fields are in conflict.
    #[error("Conflicting fields: {fields:?}")]
    Conflict {
        fields: Vec<String>,
        line: Option<usize>,
    },

    /// A value was provided that does not match the CCSDS specification.
    #[error("Invalid value for '{field}': '{value}' (expected {expected})")]
    InvalidValue {
        field: String,
        value: String,
        expected: String,
        line: Option<usize>,
    },

    /// Specific validation error for values out of expected range.
    #[error("Value for '{name}' is out of range: {value} (expected {expected})")]
    OutOfRange {
        name: String,
        value: String,
        expected: String,
        line: Option<usize>,
    },

    /// General validation errors for cases not covered by specific variants.
    #[error("Validation error: {message}")]
    Generic {
        message: String,
        line: Option<usize>,
    },
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CcsdsNdmError {
    /// Errors occurring during I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors related to NDM format or syntax.
    #[error(transparent)]
    Format(#[from] Box<FormatError>),

    /// Errors related to NDM data validation.
    #[error(transparent)]
    Validation(#[from] Box<ValidationError>),

    /// Errors related to CCSDS Epochs.
    #[error("Epoch error: {0}")]
    Epoch(#[from] EpochError),

    /// Error for unsupported CCSDS message types.
    #[error("Unsupported message type: {0}")]
    UnsupportedMessage(String),

    /// Error when an unexpected end of input is reached.
    #[error("Unexpected end of input: {context}")]
    UnexpectedEof { context: String },
}

/// A lightweight internal error type for winnow parsers.
#[derive(Debug, Clone, PartialEq)]
pub struct InternalParserError {
    pub message: String,
    pub contexts: Vec<&'static str>,
    pub kind: ParserErrorKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ParserErrorKind {
    #[default]
    Kvn,
    MissingRequiredField {
        block: &'static str,
        field: &'static str,
    },
}

impl ParserError<&str> for InternalParserError {
    type Inner = ();
    fn from_input(_input: &&str) -> Self {
        Self {
            message: String::new(),
            contexts: Vec::new(),
            kind: ParserErrorKind::default(),
        }
    }

    fn into_inner(self) -> std::result::Result<Self::Inner, Self> {
        Ok(())
    }
}

impl winnow::error::FromExternalError<&str, EpochError> for InternalParserError {
    fn from_external_error(_input: &&str, e: EpochError) -> Self {
        Self {
            message: e.to_string(),
            contexts: Vec::new(),
            kind: ParserErrorKind::default(),
        }
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseFloatError> for InternalParserError {
    fn from_external_error(_input: &&str, e: std::num::ParseFloatError) -> Self {
        Self {
            message: e.to_string(),
            contexts: Vec::new(),
            kind: ParserErrorKind::default(),
        }
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseIntError> for InternalParserError {
    fn from_external_error(_input: &&str, e: std::num::ParseIntError) -> Self {
        Self {
            message: e.to_string(),
            contexts: Vec::new(),
            kind: ParserErrorKind::default(),
        }
    }
}

impl winnow::error::FromExternalError<&str, EnumParseError> for InternalParserError {
    fn from_external_error(_input: &&str, e: EnumParseError) -> Self {
        Self {
            message: e.to_string(),
            contexts: Vec::new(),
            kind: ParserErrorKind::default(),
        }
    }
}

impl winnow::error::FromExternalError<&str, ValidationError> for InternalParserError {
    fn from_external_error(_input: &&str, e: ValidationError) -> Self {
        match e {
            ValidationError::MissingRequiredField { .. } => {
                // We unfortunately have to leak these strings or use static ones if possible.
                // But during parsing they are usually static.
                // If they are not static, we might need a different approach.
                // For now, let's assume we can use labels or just the message.
                Self {
                    message: e.to_string(),
                    contexts: Vec::new(),
                    kind: ParserErrorKind::Kvn, // Fallback
                }
            }
            _ => Self {
                message: e.to_string(),
                contexts: Vec::new(),
                kind: ParserErrorKind::default(),
            },
        }
    }
}

impl AddContext<&str, StrContext> for InternalParserError {
    fn add_context(
        mut self,
        _input: &&str,
        _token: &<&str as Stream>::Checkpoint,
        context: StrContext,
    ) -> Self {
        match context {
            StrContext::Label(l) => {
                if self.contexts.last() != Some(&l) {
                    self.contexts.push(l);
                }
            }
            StrContext::Expected(e) => self.message = format!("Expected {}", e),
            _ => {} // Ignore other context types for now
        }
        self
    }
}

impl From<ValidationError> for CcsdsNdmError {
    fn from(e: ValidationError) -> Self {
        CcsdsNdmError::Validation(Box::new(e))
    }
}

impl From<FormatError> for CcsdsNdmError {
    fn from(e: FormatError) -> Self {
        CcsdsNdmError::Format(Box::new(e))
    }
}

impl From<EnumParseError> for CcsdsNdmError {
    fn from(e: EnumParseError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::Enum(e)))
    }
}

impl From<std::num::ParseFloatError> for CcsdsNdmError {
    fn from(e: std::num::ParseFloatError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::ParseFloat(e)))
    }
}

impl From<std::num::ParseIntError> for CcsdsNdmError {
    fn from(e: std::num::ParseIntError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::ParseInt(e)))
    }
}

impl From<quick_xml::DeError> for CcsdsNdmError {
    fn from(e: quick_xml::DeError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::XmlDe(e)))
    }
}

impl From<quick_xml::se::SeError> for CcsdsNdmError {
    fn from(e: quick_xml::se::SeError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::XmlSer(e)))
    }
}

impl From<quick_xml::Error> for CcsdsNdmError {
    fn from(e: quick_xml::Error) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::Xml(e)))
    }
}

impl winnow::error::FromExternalError<&str, EpochError> for CcsdsNdmError {
    fn from_external_error(_input: &&str, e: EpochError) -> Self {
        CcsdsNdmError::Epoch(e)
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseFloatError> for CcsdsNdmError {
    fn from_external_error(_input: &&str, e: std::num::ParseFloatError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::ParseFloat(e)))
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseIntError> for CcsdsNdmError {
    fn from_external_error(_input: &&str, e: std::num::ParseIntError) -> Self {
        CcsdsNdmError::Format(Box::new(FormatError::ParseInt(e)))
    }
}

impl AddContext<&str, StrContext> for CcsdsNdmError {
    fn add_context(
        mut self,
        _input: &&str,
        _token: &<&str as Stream>::Checkpoint,
        context: StrContext,
    ) -> Self {
        if let CcsdsNdmError::Format(ref mut format_err) = self {
            if let FormatError::Kvn(ref mut inner) = **format_err {
                match context {
                    StrContext::Label(l) => {
                        if inner.contexts.last() != Some(&l) {
                            inner.contexts.push(l);
                        }
                    }
                    StrContext::Expected(e) => inner.message = format!("Expected {}", e),
                    _ => {} // Ignore other context types for now
                }
            }
        }
        self
    }
}

impl CcsdsNdmError {
    /// Returns the inner KVN parse error if this is a FormatError::Kvn.
    pub fn as_kvn_parse_error(&self) -> Option<&KvnParseError> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::Kvn(ref err) => Some(err),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns the inner validation error if this is a ValidationError.
    pub fn as_validation_error(&self) -> Option<&ValidationError> {
        match self {
            CcsdsNdmError::Validation(e) => Some(e),
            _ => None,
        }
    }

    /// Returns true if this is any FormatError.
    pub fn is_format_error(&self) -> bool {
        matches!(self, CcsdsNdmError::Format(_))
    }

    /// Returns true if this is a KVN FormatError.
    pub fn is_kvn_error(&self) -> bool {
        self.as_kvn_parse_error().is_some()
    }

    /// Returns true if this is a ValidationError.
    pub fn is_validation_error(&self) -> bool {
        self.as_validation_error().is_some()
    }

    /// Populates location information for variants with line info.
    pub fn with_location(mut self, input: &str, offset: usize) -> Self {
        match self {
            CcsdsNdmError::Format(ref mut format_err) => {
                if let FormatError::Kvn(ref mut inner) = **format_err {
                    let target_offset = if offset > 0 {
                        offset
                    } else if inner.offset > 0 {
                        inner.offset
                    } else {
                        0
                    };

                    let diag = ParseDiagnostic::new(input, target_offset, "");
                    inner.line = diag.line;
                    inner.column = diag.column;
                    inner.snippet = diag.snippet;
                    inner.offset = target_offset;
                }
            }
            CcsdsNdmError::Validation(ref mut val_err) => match **val_err {
                ValidationError::InvalidValue { ref mut line, .. }
                | ValidationError::MissingRequiredField { ref mut line, .. }
                | ValidationError::Conflict { ref mut line, .. }
                | ValidationError::Generic { ref mut line, .. }
                | ValidationError::OutOfRange { ref mut line, .. } => {
                    if line.is_none() {
                        let diag = ParseDiagnostic::new(input, offset, "");
                        *line = Some(diag.line);
                    }
                }
            },
            _ => {} // Other variants don't have location info
        }
        self
    }
}

pub type Result<T> = std::result::Result<T, CcsdsNdmError>;
