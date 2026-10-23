// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use std::borrow::Cow;
use std::fmt;
use thiserror::Error;
use winnow::error::{AddContext, ParserError, StrContext};
use winnow::stream::Stream;

/// Public operation that produced a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticOperation {
    /// Serializing a validated model into a wire notation.
    Generate,
    /// Reading a wire document into a model.
    Parse,
}

/// Wire notation selected for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticNotation {
    /// CCSDS key-value notation.
    Kvn,
    /// CCSDS XML notation.
    Xml,
}

/// Context stored only when generation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationErrorContext {
    /// Wire notation selected for this operation.
    pub notation: DiagnosticNotation,
    /// CCSDS family being read or written.
    pub message_kind: crate::validation::MessageKind,
    /// Edition stored on the model being generated.
    pub edition: String,
}

/// Context stored when parsing fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseErrorContext {
    /// Wire notation selected for this operation.
    pub notation: DiagnosticNotation,
    /// CCSDS family being read or written.
    pub message_kind: crate::validation::MessageKind,
    /// Edition read from the source header, when available.
    pub source_edition: Option<String>,
    /// Zero-based byte offset in the input, when available.
    pub byte_offset: Option<usize>,
    /// One-based source line, when available.
    pub line: Option<usize>,
    /// One-based source column, when available.
    pub column: Option<usize>,
    /// Source record excerpt, limited to 128 Unicode characters.
    pub original_token: Option<String>,
    /// Expected token supplied by the parser, when available; excludes internal parser labels.
    pub expected: Option<&'static str>,
}

impl fmt::Display for DiagnosticNotation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Kvn => "KVN",
            Self::Xml => "XML",
        })
    }
}

impl fmt::Display for GenerationErrorContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to generate {} {} {}",
            self.message_kind.as_str(),
            self.notation,
            self.edition
        )
    }
}

impl fmt::Display for ParseErrorContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to parse {} {}",
            self.message_kind.as_str(),
            self.notation
        )?;
        if let Some(edition) = &self.source_edition {
            write!(formatter, " {edition}")?;
        }
        Ok(())
    }
}

/// Borrowed, machine-readable diagnostic information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic<'a> {
    /// Operation that produced the error.
    pub operation: DiagnosticOperation,
    /// Wire notation selected for this operation.
    pub notation: DiagnosticNotation,
    /// CCSDS family being read or written.
    pub message_kind: crate::validation::MessageKind,
    /// Edition read from the source header, when available.
    pub source_edition: Option<&'a str>,
    /// Stable machine-readable category; wording may change independently.
    pub code: Option<&'static str>,
    /// Canonical model path, when the containing validator provides one.
    pub field_path: Option<String>,
    /// One-based line and column in the source document.
    pub source_location: Option<(usize, usize)>,
    /// Zero-based byte offset in the input, when available.
    pub byte_offset: Option<usize>,
    /// Source record excerpt, limited to 128 Unicode characters.
    pub original_token: Option<&'a str>,
    /// Expected token supplied by the parser, when available; excludes internal parser labels.
    pub expected: Option<&'static str>,
}

pub(crate) fn line_column(input: &str, offset: usize) -> (usize, usize) {
    let prefix = &input[..offset.min(input.len())];
    let line = prefix.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    (line, prefix[line_start..].chars().count() + 1)
}

/// Detailed error information for KVN parsing failures.
#[derive(Debug, Clone, PartialEq, Error)]
pub struct KvnParseError {
    /// One-based source line, when available.
    pub line: usize,
    /// One-based source column, when available.
    pub column: usize,
    /// Human-readable explanation of the failure.
    pub message: String,
    /// Internal parser contexts retained for inspection.
    pub contexts: Vec<&'static str>,
    /// Zero-based byte offset used to calculate the source location.
    pub offset: usize, // Track raw offset for lazy location calculation
}

impl std::fmt::Display for KvnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KVN parsing error at line {}, column {}: {}",
            self.line, self.column, self.message
        )?;
        Ok(())
    }
}

/// Lightweight error for enum string conversion.
#[derive(Debug, Clone, PartialEq, Error)]
#[error("Invalid value '{value}' for field '{field}'; expected one of: {expected}")]
pub struct EnumParseError {
    /// CCSDS keyword or field whose value failed.
    pub field: &'static str,
    /// Rejected value as text.
    pub value: String,
    /// Expected token supplied by the parser, when available; excludes internal parser labels.
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
        /// Operation or structure in which the error occurred.
        context: String,
        /// Underlying error, retained for category and source inspection.
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
        /// Logical block containing the required field.
        block: Cow<'static, str>,
        /// CCSDS keyword or field whose value failed.
        field: Cow<'static, str>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// Two or more fields are in conflict.
    #[error("Conflicting fields: {fields:?}")]
    Conflict {
        /// Fields participating in the conflicting or exclusive choice.
        fields: Vec<Cow<'static, str>>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// A schema choice selected either none or more than one of its alternatives.
    #[error("Invalid choice: expected exactly one of {fields:?}, selected {selected:?}")]
    InvalidChoice {
        /// Fields participating in the conflicting or exclusive choice.
        fields: Vec<Cow<'static, str>>,
        /// Alternatives actually selected in the model.
        selected: Vec<Cow<'static, str>>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// A value was provided that does not match the CCSDS specification.
    #[error("Invalid value for '{field}': '{value}' (expected {expected})")]
    InvalidValue {
        /// CCSDS keyword or field whose value failed.
        field: Cow<'static, str>,
        /// Rejected value as text.
        value: String,
        /// Expected token supplied by the parser, when available; excludes internal parser labels.
        expected: Cow<'static, str>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// A mandatory value is present but empty. The books require a value (ODM 7.5.1) where
    /// the XSD string and epoch types also admit an empty one; see the validation contract.
    #[error("{field} is empty")]
    EmptyValue {
        /// CCSDS keyword or field whose value failed.
        field: Cow<'static, str>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// Specific validation error for values out of expected range.
    #[error("Value for '{name}' is out of range: {value} (expected {expected})")]
    OutOfRange {
        /// CCSDS keyword or numeric domain whose range failed.
        name: Cow<'static, str>,
        /// Rejected value as text.
        value: String,
        /// Expected token supplied by the parser, when available; excludes internal parser labels.
        expected: Cow<'static, str>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// General validation errors for cases not covered by specific variants.
    #[error("Validation error: {message}")]
    Generic {
        /// Human-readable explanation of the failure.
        message: Cow<'static, str>,
        /// One-based source line, when available.
        line: Option<usize>,
    },

    /// An existing validation error enriched with its canonical model path.
    #[error("{source} (field {path})")]
    AtPath {
        /// Canonical path relative to the message root.
        path: Cow<'static, str>,
        /// Underlying error, retained for category and source inspection.
        #[source]
        source: Box<ValidationError>,
    },
}

impl ValidationError {
    /// Stable machine-readable category. Path wrappers retain their source category.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::MissingRequiredField { .. } => Some("validation.missing_required_field"),
            Self::InvalidChoice { .. } => Some("validation.invalid_choice"),
            Self::InvalidValue { .. } | Self::EmptyValue { .. } => Some("validation.invalid_value"),
            Self::OutOfRange { .. } => Some("validation.out_of_range"),
            Self::AtPath { source, .. } => source.code(),
            Self::Conflict { .. } => Some("validation.conflict"),
            Self::Generic { .. } => Some("validation.generic"),
        }
    }

    /// Complete model field path relative to the message root, when the containing block has a
    /// canonical path mapping.
    pub fn field_path(&self) -> Option<String> {
        match self {
            Self::AtPath { path, .. } => Some(path.to_string()),
            Self::MissingRequiredField { block, field, .. } => {
                let block_path = match block.as_ref() {
                    "Root" => "",
                    "ODM Header" => "header",
                    "OPM Metadata" => "body.segment.metadata",
                    "Spacecraft Parameters" => "body.segment.data.spacecraft_parameters",
                    "Maneuver Parameters" => "body.segment.data.maneuver_parameters",
                    _ => return None,
                };
                let field = field.to_ascii_lowercase();
                Some(match block_path {
                    "" => field,
                    _ => format!("{block_path}.{field}"),
                })
            }
            _ => None,
        }
    }

    pub(crate) fn at_field_in(self, parent_path: impl Into<Cow<'static, str>>) -> Self {
        let parent_path = parent_path.into();
        if matches!(self, Self::AtPath { .. }) {
            return self;
        }
        // A block name derives its own path (see `field_path`), which is kept unless the caller
        // supplies an indexed parent path: only the caller knows which repeat failed.
        if matches!(self, Self::MissingRequiredField { .. })
            && self.field_path().is_some()
            && !parent_path.ends_with(']')
        {
            return self;
        }
        let field = match &self {
            Self::MissingRequiredField { field, .. }
            | Self::InvalidValue { field, .. }
            | Self::EmptyValue { field, .. } => field.as_ref(),
            Self::OutOfRange { name, .. } => name.as_ref(),
            Self::InvalidChoice { .. } => return self.at_path(parent_path),
            Self::Conflict { .. } | Self::Generic { .. } | Self::AtPath { .. } => return self,
        };
        let field = field.to_ascii_lowercase();
        let field = match field.strip_suffix(" units") {
            Some(field) => format!("{field}.units"),
            None => field.replace(' ', "_"),
        };
        let path = match parent_path.as_ref() {
            "" => field,
            _ => format!("{parent_path}.{field}"),
        };
        self.at_path(path)
    }

    pub(crate) fn at_path(self, path: impl Into<Cow<'static, str>>) -> Self {
        if matches!(self, Self::AtPath { .. }) {
            return self;
        }
        Self::AtPath {
            path: path.into(),
            source: Box::new(self),
        }
    }

    /// Prefix a validation path while retaining the underlying diagnostic category.
    ///
    /// OEM uses this for indexed segments and history records, where a static parent path cannot
    /// identify the failing item.
    pub(crate) fn within_path(self, parent_path: impl Into<Cow<'static, str>>) -> Self {
        let parent_path = parent_path.into();
        if let Self::AtPath { path, source } = self {
            return Self::AtPath {
                path: format!("{parent_path}.{path}").into(),
                source,
            };
        }
        self.at_path(parent_path)
    }

    /// One-based source line, when a parser located the failing record.
    pub fn line(&self) -> Option<usize> {
        match self {
            Self::AtPath { source, .. } => source.line(),
            Self::MissingRequiredField { line, .. }
            | Self::Conflict { line, .. }
            | Self::InvalidChoice { line, .. }
            | Self::InvalidValue { line, .. }
            | Self::EmptyValue { line, .. }
            | Self::OutOfRange { line, .. }
            | Self::Generic { line, .. } => *line,
        }
    }

    fn line_suffix(&self) -> String {
        self.line()
            .map_or_else(String::new, |line| format!(" (line {line})"))
    }

    fn set_line_if_missing(&mut self, line: usize) {
        match self {
            Self::OutOfRange {
                line: error_line, ..
            }
            | Self::InvalidValue {
                line: error_line, ..
            }
            | Self::EmptyValue {
                line: error_line, ..
            }
            | Self::MissingRequiredField {
                line: error_line, ..
            }
            | Self::Conflict {
                line: error_line, ..
            }
            | Self::InvalidChoice {
                line: error_line, ..
            }
            | Self::Generic {
                line: error_line, ..
            } => {
                if error_line.is_none() {
                    *error_line = Some(line);
                }
            }
            Self::AtPath { source, .. } => source.set_line_if_missing(line),
        }
    }

    /// Convenience constructor for a missing required field error.
    pub fn missing_required(
        block: impl Into<Cow<'static, str>>,
        field: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::MissingRequiredField {
            block: block.into(),
            field: field.into(),
            line: None,
        }
    }

    /// Convenience constructor for an empty mandatory value.
    pub(crate) fn empty_value(field: impl Into<Cow<'static, str>>) -> Self {
        Self::EmptyValue {
            field: field.into(),
            line: None,
        }
    }

    /// Convenience constructor for an invalid value error.
    pub fn invalid_value(
        field: impl Into<Cow<'static, str>>,
        value: impl Into<String>,
        expected: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidValue {
            field: field.into(),
            value: value.into(),
            expected: expected.into(),
            line: None,
        }
    }

    /// Convenience constructor for a generic validation error.
    pub fn generic(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Generic {
            message: message.into(),
            line: None,
        }
    }

    /// Convenience constructor for conflicting fields.
    pub fn conflict<I>(fields: I) -> Self
    where
        I: IntoIterator<Item = Cow<'static, str>>,
    {
        Self::Conflict {
            fields: fields.into_iter().collect(),
            line: None,
        }
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
/// use ccsds_ndm::Ndm;
///
/// match Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...") {
///     Ok(opm) => println!("Parsed: {:?}", opm),
///     Err(e) => eprintln!("{}: {}", e.code().unwrap_or("unknown"), e),
/// }
/// ```
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CcsdsNdmError {
    /// An existing error enriched with public generation context.
    #[error("{context}: {source}")]
    Generation {
        /// Operation or structure in which the error occurred.
        context: Box<GenerationErrorContext>,
        /// Underlying error, retained for category and source inspection.
        #[source]
        source: Box<CcsdsNdmError>,
    },

    /// An existing error enriched with parsing context.
    #[error("{context}: {source}")]
    Parsing {
        /// Operation or structure in which the error occurred.
        context: Box<ParseErrorContext>,
        /// Underlying error, retained for category and source inspection.
        #[source]
        source: Box<CcsdsNdmError>,
    },

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
    #[error("{error}{line}", error = .0, line = .0.line_suffix())]
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

    /// Error for a CCSDS edition that this library cannot interpret safely.
    #[error(
        "Unsupported input version {version} for {message_type}; supported versions: {supported}"
    )]
    UnsupportedInputVersion {
        /// CCSDS family abbreviation.
        message_type: &'static str,
        /// Unsupported edition requested or read.
        version: String,
        /// Editions supported for this operation.
        supported: String,
    },

    /// Error for a CCSDS edition that has no conforming writer implementation.
    #[error(
        "Unsupported {format} output version {version} for {message_type}; supported versions: {supported}"
    )]
    UnsupportedOutputVersion {
        /// CCSDS family abbreviation.
        message_type: &'static str,
        /// Requested output notation.
        format: &'static str,
        /// Unsupported edition requested or read.
        version: String,
        /// Editions supported for this operation.
        supported: String,
    },

    /// A caller-selected resource policy was exceeded.
    #[error("Resource limit exceeded for {resource}: {actual} (limit: {limit})")]
    ResourceLimitExceeded {
        /// Resource whose configured limit was exceeded.
        resource: &'static str,
        /// Maximum permitted resource amount.
        limit: usize,
        /// Observed resource amount.
        actual: usize,
    },

    /// Error when an unexpected end of input is reached.
    #[error("Unexpected end of input: {context}")]
    UnexpectedEof {
        /// Structure or operation interrupted by the end of input.
        context: String,
    },

    /// Error for a notation in which CCSDS defines no representation of a message family.
    #[error("{message_type} has no CCSDS-defined {requested} representation")]
    UnsupportedNotation {
        /// CCSDS family abbreviation.
        message_type: &'static str,
        /// Requested wire notation.
        requested: DiagnosticNotation,
    },
}

/// A stack-allocated collection of error contexts.
///
/// **Note**: Capacity is limited to 3 contexts. Additional contexts are silently ignored.
/// This is a deliberate trade-off for performance in the hot parsing path.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct ContextStack {
    contexts: [&'static str; 3],
    len: usize,
}

impl ContextStack {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, context: &'static str) {
        if self.len < self.contexts.len() {
            self.contexts[self.len] = context;
            self.len += 1;
        }
    }

    pub(crate) fn last(&self) -> Option<&&'static str> {
        self.contexts[..self.len].last()
    }

    pub(crate) fn to_vec(&self) -> Vec<&'static str> {
        self.contexts[..self.len].to_vec()
    }
}

/// A lightweight internal error type for winnow parsers.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InternalParserError {
    pub(crate) message: Cow<'static, str>,
    pub(crate) contexts: ContextStack,
    pub(crate) kind: Box<ParserErrorKind>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) enum ParserErrorKind {
    #[default]
    Kvn,
    MissingRequiredField {
        /// Logical block containing the required field.
        block: &'static str,
        /// CCSDS keyword or field whose value failed.
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
                if !self.message.starts_with("Expected ") && self.contexts.last() != Some(&l) {
                    self.contexts.push(l);
                }
            }
            StrContext::Expected(e) => {
                self.message = Cow::Owned(format!("Expected {}", e));
                if let winnow::error::StrContextValue::Description(expected) = e {
                    self.contexts.push(expected);
                }
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
    pub(crate) fn with_generation_context(
        self,
        message_kind: crate::validation::MessageKind,
        notation: DiagnosticNotation,
        edition: &str,
    ) -> Self {
        let source = match self {
            Self::Generation { source, .. } | Self::Parsing { source, .. } => source,
            error => Box::new(error),
        };
        Self::Generation {
            context: Box::new(GenerationErrorContext {
                notation,
                message_kind,
                edition: edition.to_owned(),
            }),
            source,
        }
    }

    pub(crate) fn with_parse_context(
        self,
        message_kind: crate::validation::MessageKind,
        notation: DiagnosticNotation,
        input: &str,
        source_edition: Option<&str>,
    ) -> Self {
        let source = match self {
            Self::Generation { source, .. } | Self::Parsing { source, .. } => source,
            error => Box::new(error),
        };
        let kvn = source.as_kvn_parse_error();
        let (byte_offset, line, column, original_token, expected) = match kvn {
            Some(error) => {
                let excerpt = input
                    .get(error.offset..)
                    .and_then(|tail| tail.split(['\r', '\n']).next())
                    .map(|token| token.chars().take(128).collect());
                (
                    Some(error.offset),
                    Some(error.line),
                    Some(error.column),
                    excerpt,
                    error
                        .message
                        .strip_prefix("Expected ")
                        .and_then(|expected| {
                            error
                                .contexts
                                .iter()
                                .copied()
                                .find(|context| *context == expected)
                        }),
                )
            }
            None => match source.as_validation_error().and_then(ValidationError::line) {
                Some(line) => {
                    let normalized = crate::kvn::normalize_line_endings(input);
                    let offset = normalized
                        .split_inclusive('\n')
                        .take(line.saturating_sub(1))
                        .map(str::len)
                        .sum();
                    let token = input
                        .get(offset..)
                        .and_then(|tail| tail.split(['\r', '\n']).next())
                        .map(|token| token.chars().take(128).collect());
                    (Some(offset), Some(line), Some(1), token, None)
                }
                None => (None, None, None, None, None),
            },
        };
        Self::Parsing {
            context: Box::new(ParseErrorContext {
                notation,
                message_kind,
                source_edition: source_edition.map(str::to_owned),
                byte_offset,
                line,
                column,
                original_token,
                expected,
            }),
            source,
        }
    }

    /// Return structured context for a public parse or generation failure.
    ///
    /// Direct model validation has no wire notation and returns `None`; use `code()` and
    /// `field_path()` on that error instead.
    pub fn diagnostic(&self) -> Option<Diagnostic<'_>> {
        match self {
            Self::Generation { context, source } => Some(Diagnostic {
                operation: DiagnosticOperation::Generate,
                notation: context.notation,
                message_kind: context.message_kind,
                source_edition: Some(&context.edition),
                code: source.code(),
                field_path: source.field_path(),
                source_location: None,
                byte_offset: None,
                original_token: None,
                expected: None,
            }),
            Self::Parsing { context, source } => Some(Diagnostic {
                operation: DiagnosticOperation::Parse,
                notation: context.notation,
                message_kind: context.message_kind,
                source_edition: context.source_edition.as_deref(),
                code: if matches!(
                    source.as_format_error(),
                    Some(FormatError::InvalidFormat(_))
                ) {
                    Some(match context.notation {
                        DiagnosticNotation::Kvn => "parse.kvn.syntax",
                        DiagnosticNotation::Xml => "parse.xml.syntax",
                    })
                } else {
                    source.code()
                },
                field_path: source.field_path(),
                source_location: context.line.zip(context.column),
                byte_offset: context.byte_offset,
                original_token: context.original_token.as_deref(),
                expected: context.expected,
            }),
            _ => None,
        }
    }

    /// Stable machine-readable category; wording may improve independently.
    ///
    /// Validation and operation wrappers retain their underlying category. Invalid-format errors
    /// wrapped by a parser receive that notation's syntax code.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::Generation { source, .. } => source.code(),
            Self::Parsing { context, source } => {
                if matches!(
                    source.as_format_error(),
                    Some(FormatError::InvalidFormat(_))
                ) {
                    return Some(match context.notation {
                        DiagnosticNotation::Kvn => "parse.kvn.syntax",
                        DiagnosticNotation::Xml => "parse.xml.syntax",
                    });
                }
                source.code()
            }
            Self::Validation(error) => error.code(),
            Self::Format(error) => match error.as_ref() {
                FormatError::Kvn(_) => Some("parse.kvn.syntax"),
                FormatError::Xml(_)
                | FormatError::XmlDe(_)
                | FormatError::XmlWithContext { .. } => Some("parse.xml.syntax"),
                FormatError::XmlSer(_) => Some("generation.xml.syntax"),
                FormatError::ParseFloat(_) => Some("parse.invalid_float"),
                FormatError::ParseInt(_) => Some("parse.invalid_integer"),
                FormatError::Enum(_) => Some("parse.invalid_enum"),
                FormatError::InvalidFormat(_) => Some("format.invalid"),
            },
            Self::Epoch(_) => Some("parse.invalid_epoch"),
            Self::UnsupportedMessage(_) => Some("parse.unsupported_message_type"),
            Self::Io(_) => Some("io.error"),
            Self::UnsupportedInputVersion { .. } => Some("parse.unsupported_input_version"),
            Self::UnexpectedEof { .. } => Some("parse.unexpected_eof"),
            Self::UnsupportedNotation { .. } => Some("unsupported.notation"),
            Self::UnsupportedOutputVersion { .. } => Some("generation.unsupported_output_version"),
            Self::ResourceLimitExceeded {
                resource: "xml_depth",
                ..
            } => Some("resource.xml_depth_limit_exceeded"),
            Self::ResourceLimitExceeded { .. } => Some("resource.limit_exceeded"),
        }
    }

    /// Complete model field path for field-specific diagnostics.
    ///
    /// Operation-level failures such as an unsupported target version or an I/O error do not
    /// concern a model field and therefore return `None`.
    pub fn field_path(&self) -> Option<String> {
        match self {
            Self::Generation { source, .. } | Self::Parsing { source, .. } => source.field_path(),
            Self::Validation(error) => error.field_path(),
            _ => None,
        }
    }

    /// Returns the inner KVN parse error if this is a FormatError::Kvn.
    pub(crate) fn as_kvn_parse_error(&self) -> Option<&KvnParseError> {
        match self {
            CcsdsNdmError::Generation { source, .. } | CcsdsNdmError::Parsing { source, .. } => {
                source.as_kvn_parse_error()
            }
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
            CcsdsNdmError::Generation { source, .. } | CcsdsNdmError::Parsing { source, .. } => {
                source.as_validation_error()
            }
            CcsdsNdmError::Validation(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the inner format error if this is a FormatError.
    pub fn as_format_error(&self) -> Option<&FormatError> {
        match self {
            CcsdsNdmError::Generation { source, .. } | CcsdsNdmError::Parsing { source, .. } => {
                source.as_format_error()
            }
            CcsdsNdmError::Format(e) => Some(e),
            _ => None,
        }
    }

    /// Returns the inner I/O error if this is an IoError.
    pub fn as_io_error(&self) -> Option<&std::io::Error> {
        match self {
            CcsdsNdmError::Generation { source, .. } | CcsdsNdmError::Parsing { source, .. } => {
                source.as_io_error()
            }
            CcsdsNdmError::Io(e) => Some(e),
            _ => None,
        }
    }

    /// Returns true if this is any FormatError.
    #[cfg(test)]
    pub(crate) fn is_format_error(&self) -> bool {
        matches!(self, CcsdsNdmError::Format(_))
    }

    /// Returns true if this is a ValidationError.
    #[cfg(test)]
    pub(crate) fn is_validation_error(&self) -> bool {
        self.as_validation_error().is_some()
    }

    /// Returns the inner EnumParseError if this is a FormatError::Enum.
    #[cfg(test)]
    pub(crate) fn as_enum_error(&self) -> Option<&EnumParseError> {
        match self {
            CcsdsNdmError::Format(e) => match **e {
                FormatError::Enum(ref ee) => Some(ee),
                _ => None,
            },
            _ => None,
        }
    }

    /// Populates location information for variants with line info.
    pub(crate) fn with_location(mut self, input: &str, offset: usize) -> Self {
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

                    let (line, column) = line_column(input, target_offset);
                    inner.line = line;
                    inner.column = column;
                    inner.offset = target_offset;
                }
            }
            CcsdsNdmError::Validation(ref mut val_err) => {
                let (line, _) = line_column(input, offset);
                val_err.set_line_if_missing(line);
            }
            _ => {} // Other variants don't have location info
        }
        self
    }
}

/// Result returned by parsing, validation, generation, and file operations.
pub type Result<T> = std::result::Result<T, CcsdsNdmError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_parse_diagnostics_retain_the_record_and_model_path() {
        for ending in ["\n", "\r", "\r\n", "\n\r"] {
            let input = format!("HEADER{ending}X = bad{ending}");
            let mut error = ValidationError::invalid_value("X", "bad", "finite")
                .at_path("body.segment.data.state_vector.x");
            error.set_line_if_missing(2);
            let error: CcsdsNdmError = error.into();
            let error = error.with_parse_context(
                crate::validation::MessageKind::Opm,
                DiagnosticNotation::Kvn,
                &input,
                Some("3.0"),
            );
            let diagnostic = error.diagnostic().unwrap();
            assert_eq!(diagnostic.source_location, Some((2, 1)));
            assert_eq!(diagnostic.byte_offset, Some(6 + ending.len()));
            assert_eq!(diagnostic.original_token, Some("X = bad"));
            assert!(error.to_string().contains("line 2"));
            assert!(error
                .to_string()
                .contains("body.segment.data.state_vector.x"));
        }
    }

    #[test]
    fn previously_uncoded_errors_have_machine_categories() {
        let cases: Vec<(CcsdsNdmError, &str)> = vec![
            (
                ValidationError::conflict(["A".into(), "B".into()]).into(),
                "validation.conflict",
            ),
            (
                ValidationError::generic("cannot represent comments").into(),
                "validation.generic",
            ),
            (
                FormatError::InvalidFormat("bad".into()).into(),
                "format.invalid",
            ),
            (
                "bad".parse::<f64>().unwrap_err().into(),
                "parse.invalid_float",
            ),
            (
                "bad".parse::<i32>().unwrap_err().into(),
                "parse.invalid_integer",
            ),
            (
                EnumParseError {
                    field: "KEY",
                    value: "bad".into(),
                    expected: "A",
                }
                .into(),
                "parse.invalid_enum",
            ),
            (
                EpochError::InvalidFormat("bad".into()).into(),
                "parse.invalid_epoch",
            ),
            (
                CcsdsNdmError::UnsupportedMessage("bad".into()),
                "parse.unsupported_message_type",
            ),
            (
                CcsdsNdmError::ResourceLimitExceeded {
                    resource: "test",
                    limit: 1,
                    actual: 2,
                },
                "resource.limit_exceeded",
            ),
        ];
        for (error, code) in cases {
            assert_eq!(error.code(), Some(code), "{error}");
        }
    }

    #[test]
    fn public_error_stays_compact_when_diagnostics_are_enriched() {
        assert!(std::mem::size_of::<CcsdsNdmError>() <= 96);
    }

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
        assert!(!s.contains("Context:"));
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
    fn validation_path_preserves_error_behavior() {
        let error = ValidationError::invalid_value("X", "NaN", "a finite number");
        let display = error.to_string();
        let mut error = error.at_field_in("body.segment.data.state_vector");
        error.set_line_if_missing(12);

        assert_eq!(error.code(), Some("validation.invalid_value"));
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment.data.state_vector.x")
        );
        assert!(error.to_string().starts_with(&display));
        assert!(error
            .to_string()
            .contains("body.segment.data.state_vector.x"));
        match error {
            ValidationError::AtPath { source, .. } => {
                assert!(matches!(
                    *source,
                    ValidationError::InvalidValue { line: Some(12), .. }
                ));
            }
            error => panic!("expected a path-enriched error, got {error:?}"),
        }
    }

    #[test]
    fn top_level_diagnostic_accessors_delegate_without_inventing_paths() {
        let validation: CcsdsNdmError = ValidationError::invalid_value("X", "NaN", "finite")
            .at_field_in("state_vector")
            .into();
        assert_eq!(validation.code(), Some("validation.invalid_value"));
        assert_eq!(validation.field_path().as_deref(), Some("state_vector.x"));

        let unsupported = CcsdsNdmError::UnsupportedOutputVersion {
            message_type: "OPM",
            format: "XML",
            version: "2.0".into(),
            supported: "3.0".into(),
        };
        assert_eq!(
            unsupported.code(),
            Some("generation.unsupported_output_version")
        );
        assert_eq!(unsupported.field_path(), None);

        let io = CcsdsNdmError::Io(std::io::Error::other("failed"));
        assert_eq!(io.code(), Some("io.error"));
        assert_eq!(io.field_path(), None);
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
        err.set_line_if_missing(123);
        if let ValidationError::OutOfRange { line, .. } = err {
            assert_eq!(line, Some(123));
        } else {
            panic!("Wrong variant");
        }

        // Should NOT overwrite line if already set
        err.set_line_if_missing(456);
        if let ValidationError::OutOfRange { line, .. } = err {
            assert_eq!(line, Some(123));
        } else {
            panic!("Wrong variant");
        }
    }

    #[test]
    fn test_ccsds_ndm_error_helpers() {
        let io_err = std::io::Error::other("io");
        let err: CcsdsNdmError = io_err.into();
        assert!(err.as_io_error().is_some());
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

        let enum_err = EnumParseError {
            field: "F",
            value: "V".into(),
            expected: "E",
        };
        let err: CcsdsNdmError = enum_err.into();
        assert!(err.as_enum_error().is_some());

        // The `From` conversions are the behaviour worth pinning; matching the variant does that
        // without an accessor that only this test would call.
        let err: CcsdsNdmError = "abc".parse::<f64>().unwrap_err().into();
        assert!(matches!(
            err.as_format_error(),
            Some(FormatError::ParseFloat(_))
        ));

        let err: CcsdsNdmError = "abc".parse::<i32>().unwrap_err().into();
        assert!(matches!(
            err.as_format_error(),
            Some(FormatError::ParseInt(_))
        ));

        let err = CcsdsNdmError::Epoch(EpochError::InvalidFormat("2023".into()));
        assert!(matches!(err, CcsdsNdmError::Epoch(_)));

        let eof_err = CcsdsNdmError::UnexpectedEof {
            context: "ctx".into(),
        };
        assert_eq!(format!("{}", eof_err), "Unexpected end of input: ctx");

        let unsupported = CcsdsNdmError::UnsupportedMessage("type".into());
        assert_eq!(format!("{}", unsupported), "Unsupported message type: type");
    }

    #[test]
    fn test_format_error_variants() {
        let xml_err = quick_xml::Error::Io(std::sync::Arc::new(std::io::Error::other("io")));
        let err: CcsdsNdmError = FormatError::Xml(xml_err).into();
        assert!(matches!(err.as_format_error(), Some(FormatError::Xml(_))));

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
        assert!(matches!(
            err.as_validation_error(),
            Some(ValidationError::Generic { line: Some(2), .. })
        ));

        let kvn_err = KvnParseError {
            line: 0,
            column: 0,
            message: "msg".into(),
            contexts: vec![],
            offset: 0,
        };
        let mut err = CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(kvn_err))));
        err = err.with_location(input, 12); // start of LINE3
        let ke = err
            .as_kvn_parse_error()
            .expect("KVN error must be preserved");
        assert_eq!((ke.line, ke.column), (3, 1));
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
