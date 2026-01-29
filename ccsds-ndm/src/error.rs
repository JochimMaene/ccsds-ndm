// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use std::borrow::Cow;
use thiserror::Error;
use winnow::error::{AddContext, ParserError, StrContext};
use winnow::stream::Stream;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseDiagnostic {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub contexts: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RawParsePosition {
    pub offset: usize,
    pub message: Cow<'static, str>,
    pub contexts: ContextStack,
}

impl RawParsePosition {
    pub fn into_parse_error(self, input: &str) -> KvnParseError {
        // Use 0 as default if we don't have location info yet,
        // to_ccsds_error + with_location will fix this.
        // Actually, with_location recalculates from offset.
        // But ParseDiagnostic computes everything.
        let diag = ParseDiagnostic::new(input, self.offset, &*self.message);
        KvnParseError {
            line: diag.line,
            column: diag.column,
            message: self.message.into_owned(),
            contexts: self.contexts.to_vec(),
            offset: self.offset,
        }
    }
}

impl ParseDiagnostic {
    /// Creates a new diagnostic from an input string and byte offset.
    pub fn new(input: &str, offset: usize, message: impl Into<String>) -> Self {
        let offset = offset.min(input.len());
        let prefix = &input[..offset];
        let line = prefix.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;
        let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = prefix[line_start..].chars().count();

        Self {
            line,
            column: column + 1,
            message: message.into(),
            contexts: Vec::new(),
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
    pub offset: usize, // Track raw offset for lazy location calculation
}

impl std::fmt::Display for KvnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KVN parsing error at line {}, column {}: {}",
            self.line, self.column, self.message
        )?;
        if !self.contexts.is_empty() {
            write!(f, "\nContext: {}", self.contexts.join(" > "))?;
        }
        Ok(())
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
#[non_exhaustive]
pub enum FormatError {
    /// Errors occurring during KVN parsing.
    #[error(transparent)]
    Kvn(#[from] Box<KvnParseError>),

    /// Errors occurring during XML parsing.
    #[error("XML error: {0}")]
    Xml(
        #[source]
        #[from]
        quick_xml::Error,
    ),

    /// Errors occurring during XML deserialization.
    #[error("XML deserialization error: {0}")]
    XmlDe(
        #[source]
        #[from]
        quick_xml::DeError,
    ),

    /// Errors occurring during XML serialization.
    #[error("XML serialization error: {0}")]
    XmlSer(
        #[source]
        #[from]
        quick_xml::se::SeError,
    ),

    /// Error when parsing a floating point number fails.
    #[error("Parse float error: {0}")]
    ParseFloat(
        #[source]
        #[from]
        std::num::ParseFloatError,
    ),

    /// Error when parsing an integer number fails.
    #[error("Parse int error: {0}")]
    ParseInt(
        #[source]
        #[from]
        std::num::ParseIntError,
    ),

    /// Error during enum parsing.
    #[error(transparent)]
    Enum(#[from] EnumParseError),

    /// Error when the format of a value or segment is invalid.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Errors occurring during XML deserialization with added context.
    #[error("{context}: {source}")]
    XmlWithContext {
        context: String,
        #[source]
        source: quick_xml::DeError,
    },
}

/// Errors related to the validation of NDM data against CCSDS rules.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum ValidationError {
    /// A required field was missing in the message.
    #[error("Missing required field: {field} in block {block}")]
    MissingRequiredField {
        block: Cow<'static, str>,
        field: Cow<'static, str>,
        line: Option<usize>,
    },

    /// Two or more fields are in conflict.
    #[error("Conflicting fields: {fields:?}")]
    Conflict {
        fields: Vec<Cow<'static, str>>,
        line: Option<usize>,
    },

    /// A value was provided that does not match the CCSDS specification.
    #[error("Invalid value for '{field}': '{value}' (expected {expected})")]
    InvalidValue {
        field: Cow<'static, str>,
        value: String,
        expected: Cow<'static, str>,
        line: Option<usize>,
    },

    /// Specific validation error for values out of expected range.
    #[error("Value for '{name}' is out of range: {value} (expected {expected})")]
    OutOfRange {
        name: Cow<'static, str>,
        value: String,
        expected: Cow<'static, str>,
        line: Option<usize>,
    },

    /// General validation errors for cases not covered by specific variants.
    #[error("Validation error: {message}")]
    Generic {
        message: Cow<'static, str>,
        line: Option<usize>,
    },
}

/// Trait for errors that can be enriched with line/column info.
pub trait WithLocation: Sized {
    /// Adds location information to the error.
    fn with_line(self, line: usize) -> Self;
}

impl WithLocation for ValidationError {
    fn with_line(mut self, line: usize) -> Self {
        match &mut self {
            ValidationError::OutOfRange {
                line: ref mut l, ..
            }
            | ValidationError::InvalidValue {
                line: ref mut l, ..
            }
            | ValidationError::MissingRequiredField {
                line: ref mut l, ..
            }
            | ValidationError::Conflict {
                line: ref mut l, ..
            }
            | ValidationError::Generic {
                line: ref mut l, ..
            } => {
                if l.is_none() {
                    *l = Some(line);
                }
            }
        }
        self
    }
}

/// The top-level error type for the CCSDS NDM library.
///
/// This enum wraps all possible errors that can occur during NDM parsing,
/// validation, serialization, and I/O.
///
/// # Example: Handling Parse Errors
/// ```no_run
/// use ccsds_ndm::messages::opm::Opm;
/// use ccsds_ndm::error::CcsdsNdmError;
/// use ccsds_ndm::kvn::parser::ParseKvn;
///
/// match Opm::from_kvn_str("CCSDS_OPM_VERS = 3.0\n...") {
///     Ok(opm) => println!("Parsed: {:?}", opm),
///     Err(e) => {
///         if let Some(enum_err) = e.as_enum_error() {
///             eprintln!("Invalid enum value '{}' for field '{}'", enum_err.value, enum_err.field);
///         } else if let Some(validation_err) = e.as_validation_error() {
///             eprintln!("Validation error: {}", validation_err);
///         } else {
///             eprintln!("Error: {}", e);
///         }
///     }
/// }
/// ```
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CcsdsNdmError {
    /// Errors occurring during I/O operations.
    #[error("I/O error: {0}")]
    Io(
        #[source]
        #[from]
        std::io::Error,
    ),

    /// Errors related to NDM format or syntax.
    #[error(transparent)]
    Format(#[from] Box<FormatError>),

    /// Errors related to NDM data validation.
    #[error(transparent)]
    Validation(#[from] Box<ValidationError>),

    /// Errors related to CCSDS Epochs.
    #[error("Epoch error: {0}")]
    Epoch(
        #[source]
        #[from]
        EpochError,
    ),

    /// Error for unsupported CCSDS message types.
    #[error("Unsupported message type: {0}")]
    UnsupportedMessage(String),

    /// Error when an unexpected end of input is reached.
    #[error("Unexpected end of input: {context}")]
    UnexpectedEof { context: String },
}

/// A stack-allocated collection of error contexts.
///
/// **Note**: Capacity is limited to 3 contexts. Additional contexts are silently ignored.
/// This is a deliberate trade-off for performance in the hot parsing path.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContextStack {
    contexts: [&'static str; 3],
    len: usize,
}

impl ContextStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, context: &'static str) {
        if self.len < self.contexts.len() {
            self.contexts[self.len] = context;
            self.len += 1;
        }
    }

    pub fn last(&self) -> Option<&&'static str> {
        if self.len > 0 {
            Some(&self.contexts[self.len - 1])
        } else {
            None
        }
    }

    pub fn to_vec(&self) -> Vec<&'static str> {
        self.contexts[..self.len].to_vec()
    }
}

/// A lightweight internal error type for winnow parsers.
#[derive(Debug, Clone, PartialEq)]
pub struct InternalParserError {
    pub message: Cow<'static, str>,
    pub contexts: ContextStack,
    pub kind: Box<ParserErrorKind>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ParserErrorKind {
    #[default]
    Kvn,
    MissingRequiredField {
        block: &'static str,
        field: &'static str,
    },
    Validation(ValidationError),
    Epoch(EpochError),
    Enum(EnumParseError),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
}

impl ParserError<&str> for InternalParserError {
    type Inner = ();
    fn from_input(_input: &&str) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::default()),
        }
    }

    fn into_inner(self) -> std::result::Result<Self::Inner, Self> {
        Ok(())
    }
}

impl winnow::error::FromExternalError<&str, EpochError> for InternalParserError {
    fn from_external_error(_input: &&str, e: EpochError) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::Epoch(e)),
        }
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseFloatError> for InternalParserError {
    fn from_external_error(_input: &&str, e: std::num::ParseFloatError) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::ParseFloat(e)),
        }
    }
}

impl winnow::error::FromExternalError<&str, std::num::ParseIntError> for InternalParserError {
    fn from_external_error(_input: &&str, e: std::num::ParseIntError) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::ParseInt(e)),
        }
    }
}

impl winnow::error::FromExternalError<&str, EnumParseError> for InternalParserError {
    fn from_external_error(_input: &&str, e: EnumParseError) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::Enum(e)),
        }
    }
}

impl winnow::error::FromExternalError<&str, ValidationError> for InternalParserError {
    fn from_external_error(_input: &&str, e: ValidationError) -> Self {
        Self {
            message: Cow::Borrowed(""),
            contexts: ContextStack::new(),
            kind: Box::new(ParserErrorKind::Validation(e)),
        }
    }
}

impl winnow::error::FromExternalError<&str, CcsdsNdmError> for InternalParserError {
    fn from_external_error(input: &&str, e: CcsdsNdmError) -> Self {
        match e {
            CcsdsNdmError::Validation(ve) => Self::from_external_error(input, *ve),
            CcsdsNdmError::Epoch(ee) => Self::from_external_error(input, ee),
            CcsdsNdmError::Format(fe) => match *fe {
                FormatError::Enum(ee) => Self::from_external_error(input, ee),
                FormatError::ParseFloat(pfe) => Self::from_external_error(input, pfe),
                FormatError::ParseInt(pie) => Self::from_external_error(input, pie),
                _ => Self {
                    message: Cow::Owned(fe.to_string()),
                    contexts: ContextStack::new(),
                    kind: Box::new(ParserErrorKind::default()),
                },
            },
            _ => Self {
                message: Cow::Owned(e.to_string()),
                contexts: ContextStack::new(),
                kind: Box::new(ParserErrorKind::default()),
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
            StrContext::Expected(e) => {
                self.message = Cow::Owned(format!("Expected {}", e));
            }
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

    /// Returns the inner format error if this is a FormatError.
    pub fn as_format_error(&self) -> Option<&FormatError> {
        match self {
            CcsdsNdmError::Format(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the inner epoch error if this is an EpochError.
    pub fn as_epoch_error(&self) -> Option<&EpochError> {
        match self {
            CcsdsNdmError::Epoch(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the inner I/O error if this is an IoError.
    pub fn as_io_error(&self) -> Option<&std::io::Error> {
        match self {
            CcsdsNdmError::Io(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the inner XML error if this is an XmlError.
    pub fn as_xml_error(&self) -> Option<&quick_xml::Error> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::Xml(ref xe) => Some(xe),
                _ => None,
            },
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

    /// Returns true if this is an I/O error.
    pub fn is_io_error(&self) -> bool {
        matches!(self, CcsdsNdmError::Io(_))
    }

    /// Returns true if this is an epoch error.
    pub fn is_epoch_error(&self) -> bool {
        matches!(self, CcsdsNdmError::Epoch(_))
    }

    /// Returns the inner EnumParseError if this is a FormatError::Enum.
    pub fn as_enum_error(&self) -> Option<&EnumParseError> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::Enum(ref ee) => Some(ee),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns the inner ParseIntError if this is a FormatError::ParseInt.
    pub fn as_parse_int_error(&self) -> Option<&std::num::ParseIntError> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::ParseInt(ref pie) => Some(pie),
                _ => None,
            },
            _ => None,
        }
    }

    /// Returns the inner ParseFloatError if this is a FormatError::ParseFloat.
    pub fn as_parse_float_error(&self) -> Option<&std::num::ParseFloatError> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::ParseFloat(ref pfe) => Some(pfe),
                _ => None,
            },
            _ => None,
        }
    }

    /// Populates location information for variants with line info.
    pub fn with_location(mut self, input: &str, offset: usize) -> Self {
        match self {
            CcsdsNdmError::Format(ref mut format_err) => {
                if let FormatError::Kvn(ref mut inner) = **format_err {
                    // Avoid re-calculating if already populated via RawParsePosition
                    if inner.line > 0 {
                        return self;
                    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvn_parse_error_display() {
        let err = KvnParseError {
            line: 10,
            column: 5,
            message: "Test error".into(),
            contexts: vec!["Header", "Version"],
            offset: 100,
        };
        let s = format!("{}", err);
        assert!(s.contains("line 10, column 5"));
        assert!(s.contains("Test error"));
        assert!(s.contains("Header > Version"));
    }

    #[test]
    fn test_enum_parse_error_display() {
        let err = EnumParseError {
            field: "FIELD",
            value: "VAL".into(),
            expected: "A or B",
        };
        let s = format!("{}", err); // uses default error display because of `thiserror`
                                    // We defined #[error("Invalid value '{value}' for field '{field}'; expected one of: {expected}")]
        assert!(s.contains("Invalid value 'VAL' for field 'FIELD'"));
        assert!(s.contains("expected one of: A or B"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::MissingRequiredField {
            block: "BLOCK".into(),
            field: "FIELD".into(),
            line: Some(42),
        };
        let s = format!("{}", err);
        assert!(s.contains("Missing required field: FIELD in block BLOCK"));
    }

    #[test]
    fn test_validation_error_with_location() {
        let mut err = ValidationError::OutOfRange {
            name: "N".into(),
            value: "V".into(),
            expected: "E".into(),
            line: None,
        };
        // Should set line
        err = err.with_line(123);
        if let ValidationError::OutOfRange { line, .. } = err {
            assert_eq!(line, Some(123));
        } else {
            panic!("Wrong variant");
        }

        // Should NOT overwrite line if already set
        err = err.with_line(456);
        if let ValidationError::OutOfRange { line, .. } = err {
            assert_eq!(line, Some(123));
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_ccsds_ndm_error_helpers() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io");
        let err: CcsdsNdmError = io_err.into();
        assert!(err.as_io_error().is_some());
        assert!(err.is_io_error());
        assert_eq!(format!("{}", err), "I/O error: io");

        let val_err = ValidationError::Generic {
            message: "g".into(),
            line: None,
        };
        let err: CcsdsNdmError = val_err.into();
        assert!(err.as_validation_error().is_some());
        assert!(err.is_validation_error());

        let fmt_err = FormatError::InvalidFormat("f".into());
        let err: CcsdsNdmError = fmt_err.into();
        assert!(err.as_format_error().is_some());
        assert!(err.is_format_error());
        assert!(!err.is_kvn_error());

        let enum_err = EnumParseError {
            field: "F",
            value: "V".into(),
            expected: "E",
        };
        let err: CcsdsNdmError = enum_err.into();
        assert!(err.as_enum_error().is_some());

        let pfe_err = "abc".parse::<f64>().unwrap_err();
        let err: CcsdsNdmError = pfe_err.into();
        assert!(err.as_parse_float_error().is_some());

        let pie_err = "abc".parse::<i32>().unwrap_err();
        let err: CcsdsNdmError = pie_err.into();
        assert!(err.as_parse_int_error().is_some());

        let epoch_err = EpochError::InvalidFormat("2023".into());
        let err: CcsdsNdmError = CcsdsNdmError::Epoch(epoch_err);
        assert!(err.as_epoch_error().is_some());
        assert!(err.is_epoch_error());

        let eof_err = CcsdsNdmError::UnexpectedEof {
            context: "ctx".into(),
        };
        assert_eq!(format!("{}", eof_err), "Unexpected end of input: ctx");

        let unsupported = CcsdsNdmError::UnsupportedMessage("type".into());
        assert_eq!(format!("{}", unsupported), "Unsupported message type: type");
    }

    #[test]
    fn test_format_error_variants() {
        let xml_err = quick_xml::Error::Io(std::sync::Arc::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "io",
        )));
        let err: CcsdsNdmError = FormatError::Xml(xml_err).into();
        assert!(err.as_xml_error().is_some());

        let fmt_err = FormatError::XmlWithContext {
            context: "ctx".into(),
            source: quick_xml::DeError::Custom("msg".into()),
        };
        assert!(format!("{}", fmt_err).contains("ctx"));
    }

    #[test]
    fn test_with_location() {
        let input = "LINE1\nLINE2\nLINE3";
        let mut err = CcsdsNdmError::Validation(Box::new(ValidationError::Generic {
            message: "msg".into(),
            line: None,
        }));
        // Offset 6 is start of LINE2
        err = err.with_location(input, 6);
        if let CcsdsNdmError::Validation(ve) = err {
            if let ValidationError::Generic { line, .. } = *ve {
                assert_eq!(line, Some(2));
            }
        }

        let kvn_err = KvnParseError {
            line: 0,
            column: 0,
            message: "msg".into(),
            contexts: vec![],
            offset: 0,
        };
        let mut err = CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(kvn_err))));
        err = err.with_location(input, 12); // start of LINE3
        if let Some(ke) = err.as_kvn_parse_error() {
            assert_eq!(ke.line, 3);
        }
    }

    #[test]
    fn test_context_stack() {
        let mut stack = ContextStack::new();
        assert_eq!(stack.last(), None);
        stack.push("A");
        stack.push("B");
        stack.push("C");
        assert_eq!(stack.last(), Some(&"C"));
        stack.push("D"); // Ignored, capacity 3
        assert_eq!(stack.last(), Some(&"C"));
        assert_eq!(stack.to_vec(), vec!["A", "B", "C"]);
    }

    #[test]
    fn test_internal_parser_error() {
        use winnow::error::ParserError;
        let input = "abc";
        let mut err = InternalParserError::from_input(&input);
        assert_eq!(err.message, "");
        err.contexts.push("ctx");
        assert_eq!(err.contexts.to_vec(), vec!["ctx"]);

        // Test from_external_error for InternalParserError
        use winnow::error::FromExternalError;
        let enum_err = EnumParseError {
            field: "F",
            value: "V".into(),
            expected: "E",
        };
        let err = InternalParserError::from_external_error(&input, enum_err);
        assert!(matches!(*err.kind, ParserErrorKind::Enum(_)));

        let ccsds_err = CcsdsNdmError::Validation(Box::new(ValidationError::Generic {
            message: "m".into(),
            line: None,
        }));
        let err = InternalParserError::from_external_error(&input, ccsds_err);
        assert!(matches!(*err.kind, ParserErrorKind::Validation(_)));
    }
}
