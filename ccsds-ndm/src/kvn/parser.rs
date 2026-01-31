// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Winnow-based parser combinators for CCSDS KVN format.
//!
//! This module provides reusable building blocks for parsing KVN (Key-Value Notation)
//! files. KVN is a line-oriented format where each line is either:
//! - A key-value pair: `KEY = value` or `KEY = value [unit]`
//! - A comment: `COMMENT text`
//! - A block delimiter: `META_START`, `META_STOP`, `DATA_START`, etc.
//! - A raw data line (space-separated values)
//! - An empty line
//!
//! # Architecture
//!
//! The parsing is split into two layers:
//! 1. **Line-level**: Parse individual KVN lines into structured tokens
//! 2. **Message-level**: Compose line parsers to build complete message structures

use crate::common::{AdmHeader, OdmHeader, OpmCovarianceMatrix, SpacecraftParameters, StateVector};
use crate::error::{
    CcsdsNdmError, EnumParseError, FormatError, InternalParserError, ValidationError,
};
use crate::traits::{FromKvnFloat, FromKvnValue};
use crate::types::{UserDefined, UserDefinedParameter, *};
use fast_float;
use std::str::FromStr;
use winnow::ascii::{line_ending, space0, till_line_ending};
use winnow::combinator::{alt, delimited, opt, peek, preceded, repeat, terminated};
use winnow::error::{
    AddContext, ErrMode, FromExternalError, ParserError, StrContext, StrContextValue,
};
use winnow::prelude::*;
use winnow::stream::Offset;
use winnow::token::{one_of, take_till, take_while};

/// A result type for winnow parsers using the library's internal lightweight error type.
pub type KvnResult<O, E = InternalParserError> = Result<O, ErrMode<E>>;

//----------------------------------------------------------------------
// Low-level fast parsers
//----------------------------------------------------------------------

/// Parses a float directly from the input.
pub fn parse_f64_winnow(input: &mut &str) -> KvnResult<f64> {
    let s = take_while::<_, _, ()>(1.., ('0'..='9', '.', '-', '+', 'e', 'E'))
        .parse_next(input)
        .map_err(|_| cut_err(input, "Invalid float"))?;
    fast_float::parse(s).map_err(|_| cut_err(input, "Invalid float"))
}

/// Parses up to the next space or line ending, skipping leading whitespace.
pub fn till_space<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    preceded(ws, take_till(1.., (' ', '\t', '\r', '\n'))).parse_next(input)
}

/// Parses up to the next space or line ending, or end of input, skipping leading whitespace.
pub fn till_space_or_eol<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    preceded(ws, take_till(1.., (' ', '\t', '\r', '\n'))).parse_next(input)
}

/// Parses a float and its optional unit from a KVN line, allowing empty input.
/// Returns Ok((None, u)) if no float found (but valid line end or unit-only).
pub fn kv_float_unit_opt<'a>(input: &mut &'a str) -> KvnResult<(Option<f64>, Option<&'a str>)> {
    ws.parse_next(input)?;

    // Try to parse a float (peek first to ensure we don't consume partial match tokens if not float?)
    // parse_f64_winnow works by take_while. If it takes nothing, it fails.
    if peek(parse_f64_winnow).parse_next(input).is_ok() {
        let f = parse_f64_winnow.parse_next(input)?;
        let u = kv_unit.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        Ok((Some(f), u))
    } else {
        // No float. Could be just unit, or empty.
        let u = kv_unit.parse_next(input)?;

        // After optional unit, we MUST be at end of line/comment or whitespace.
        // If there's still non-whitespace content, it's invalid (garbage).
        let remainder = till_line_ending.parse_next(input)?;
        if !remainder.trim().is_empty() {
            return Err(cut_err(input, "Invalid float value"));
        }

        opt_line_ending.parse_next(input)?;
        Ok((None, u))
    }
}

//----------------------------------------------------------------------
// Error Handling
//----------------------------------------------------------------------

/// Converts a winnow error to our library's error type.
pub fn to_ccsds_error(
    input: &str,
    err: winnow::error::ParseError<&str, InternalParserError>,
) -> CcsdsNdmError {
    let offset = err.offset();
    let inner = err.into_inner();

    let base_err = match *inner.kind {
        crate::error::ParserErrorKind::Validation(e) => CcsdsNdmError::Validation(Box::new(e)),
        crate::error::ParserErrorKind::Epoch(e) => CcsdsNdmError::Epoch(e),
        crate::error::ParserErrorKind::Enum(e) => {
            CcsdsNdmError::Format(Box::new(FormatError::Enum(e)))
        }
        crate::error::ParserErrorKind::ParseInt(e) => {
            CcsdsNdmError::Format(Box::new(FormatError::ParseInt(e)))
        }
        crate::error::ParserErrorKind::ParseFloat(e) => {
            CcsdsNdmError::Format(Box::new(FormatError::ParseFloat(e)))
        }
        crate::error::ParserErrorKind::MissingRequiredField { block, field } => {
            return CcsdsNdmError::Validation(Box::new(ValidationError::MissingRequiredField {
                block: std::borrow::Cow::Borrowed(block),
                field: std::borrow::Cow::Borrowed(field),
                line: None,
            }))
            .with_location(input, offset);
        }
        _ => {
            let message = inner.message;

            let raw = crate::error::RawParsePosition {
                offset,
                message,
                contexts: inner.contexts,
            };
            CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(
                raw.into_parse_error(input),
            ))))
        }
    };

    base_err.with_location(input, offset)
}

/// Creates a winnow ErrMode::Cut with a static context label.
pub fn cut_err(input: &mut &str, label: &'static str) -> ErrMode<InternalParserError> {
    ErrMode::Cut(InternalParserError::from_input(input).add_context(
        input,
        &input.checkpoint(),
        StrContext::Label(label),
    ))
}

/// Creates a winnow ErrMode::Cut for a missing required field.
pub fn missing_field_err(
    _input: &mut &str,
    block: &'static str,
    field: &'static str,
) -> ErrMode<InternalParserError> {
    ErrMode::Cut(InternalParserError {
        message: std::borrow::Cow::Borrowed(""),
        contexts: crate::error::ContextStack::new(),
        kind: Box::new(crate::error::ParserErrorKind::MissingRequiredField { block, field }),
    })
}

//----------------------------------------------------------------------
// Low-level Token Parsers
//----------------------------------------------------------------------

/// Parses optional whitespace (spaces and tabs only, not newlines).
pub fn ws<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    space0.parse_next(input)
}

/// Parses a KVN keyword (uppercase letters, digits, underscores).
/// Keywords must start with a letter.
pub fn keyword<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    (
        one_of('A'..='Z'),
        take_while(0.., ('A'..='Z', '0'..='9', '_')),
    )
        .take()
        .parse_next(input)
}

/// Parses the `= ` separator in a key-value pair.
pub fn kv_sep(input: &mut &str) -> KvnResult<()> {
    (ws, '=', ws).void().parse_next(input)
}

/// Parses an optional unit in brackets: `[unit]`
/// If a `[` is encountered, a matching `]` is strictly enforced.
pub fn kv_unit<'a>(input: &mut &'a str) -> KvnResult<Option<&'a str>> {
    ws.parse_next(input)?;
    if input.starts_with('[') {
        let u = delimited('[', take_till(0.., |c: char| c == ']'), ']')
            .context(StrContext::Label("unit in brackets"))
            .parse_next(input)?;
        Ok(Some(u))
    } else {
        Ok(None)
    }
}

/// Parses the value part of a key-value pair.
/// Handles values with or without units.
pub fn kvn_value<'a>(input: &mut &'a str) -> KvnResult<(&'a str, Option<&'a str>)> {
    let val = take_till(0.., |c: char| c == '[' || c == '\r' || c == '\n')
        .map(|s: &str| s.trim())
        .parse_next(input)?;

    if val.is_empty() {
        // If it starts with '[', it could be a unit OR the value itself could start with '['
        // (like MAN_UNITS = [n/a, ...])
        // In KVN, units are typically at the end.
        // Let's take everything till the end of the line.
        let rest = till_line_ending.parse_next(input)?;
        let trimmed = rest.trim();
        Ok((trimmed, None))
    } else {
        let unit = kv_unit.parse_next(input)?;
        Ok((val, unit))
    }
}

//----------------------------------------------------------------------
// Line-level Parsers
//----------------------------------------------------------------------

/// A parsed KVN line.
#[derive(Debug, Clone, PartialEq)]
pub enum KvnToken<'a> {
    /// A key-value pair with optional unit.
    KeyValue {
        key: &'a str,
        value: &'a str,
        unit: Option<&'a str>,
    },
    /// A comment line.
    Comment(&'a str),
    /// A block start marker (e.g., "META" from "META_START").
    BlockStart(&'a str),
    /// A block end marker (e.g., "META" from "META_STOP").
    BlockEnd(&'a str),
    /// A raw data line (space-separated values).
    Raw(&'a str),
    /// An empty line.
    Empty,
}

/// Parses a COMMENT line.
pub fn comment_line<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    preceded((ws, "COMMENT", space0), till_line_ending).parse_next(input)
}

/// Parses a key-value pair line.
pub fn key_value_line<'a>(input: &mut &'a str) -> KvnResult<(&'a str, &'a str, Option<&'a str>)> {
    (preceded(ws, keyword), kv_sep, kvn_value)
        .map(|(key, _, (value, unit))| (key, value, unit))
        .parse_next(input)
}

/// Parses a block start marker (e.g., META_START).
pub fn block_start<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    let content = preceded(ws, till_line_ending).parse_next(input)?;
    let content = content.trim();

    if let Some(prefix) = content.strip_suffix("_START") {
        if !prefix.contains(char::is_whitespace) {
            return Ok(prefix);
        }
    }
    Err(ErrMode::Backtrack(InternalParserError::from_input(input)))
}

/// Parses a block end marker (e.g., META_STOP or COVARIANCE_END).
pub fn block_end<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    let content = preceded(ws, till_line_ending).parse_next(input)?;
    let content = content.trim();

    if let Some(prefix) = content.strip_suffix("_STOP") {
        if !prefix.contains(char::is_whitespace) {
            return Ok(prefix);
        }
    } else if let Some(prefix) = content.strip_suffix("_END") {
        if !prefix.contains(char::is_whitespace) {
            return Ok(prefix);
        }
    }
    Err(ErrMode::Backtrack(InternalParserError::from_input(input)))
}

/// Parses an empty line.
pub fn empty_line(input: &mut &str) -> KvnResult<()> {
    (
        ws,
        peek(alt((line_ending.void(), winnow::combinator::eof.void()))),
    )
        .void()
        .parse_next(input)
}

/// Parses a raw data line (no equals sign, not a keyword).
pub fn raw_line<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    let content = preceded(ws, till_line_ending).parse_next(input)?;
    let trimmed = content.trim();

    // Raw lines should not be empty, comments, or contain '='
    if trimmed.is_empty()
        || trimmed.starts_with("COMMENT")
        || trimmed.contains('=')
        || trimmed.ends_with("_START")
        || trimmed.ends_with("_STOP")
        || trimmed.ends_with("_END")
    {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }

    Ok(trimmed)
}

/// Parses any KVN line into a token.
pub fn kvn_token<'a>(input: &mut &'a str) -> KvnResult<KvnToken<'a>> {
    // Skip leading whitespace on the line
    ws.parse_next(input)?;

    alt((
        empty_line.map(|_| KvnToken::Empty),
        comment_line.map(KvnToken::Comment),
        block_start.map(KvnToken::BlockStart),
        block_end.map(KvnToken::BlockEnd),
        key_value_line.map(|(k, v, u)| KvnToken::KeyValue {
            key: k,
            value: v,
            unit: u,
        }),
        raw_line.map(KvnToken::Raw),
    ))
    .parse_next(input)
}

/// Parses "KEY =" and returns the key.
pub fn key_token<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    terminated(preceded(ws, keyword), kv_sep).parse_next(input)
}

/// Parses the rest of a KVN line (value and optional unit).
pub fn kv_rest<'a>(input: &mut &'a str) -> KvnResult<(&'a str, Option<&'a str>)> {
    terminated(kvn_value, opt_line_ending).parse_next(input)
}

/// Fast float parser for KVN values.
pub fn kv_float(input: &mut &str) -> KvnResult<f64> {
    let checkpoint = input.checkpoint();
    terminated(
        (
            parse_f64_winnow.context(StrContext::Label("float")),
            kv_unit,
        )
            .map(|(f, _)| f),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid float");
            ErrMode::Cut(err.add_context(input, &checkpoint, StrContext::Label("Invalid float")))
        } else {
            e
        }
    })
}

/// Fast i32 parser for KVN values.
pub fn kv_i32(input: &mut &str) -> KvnResult<i32> {
    let checkpoint = input.checkpoint();
    terminated(
        (
            take_while(1.., ('0'..='9', '-', '+'))
                .map(|s: &str| s.parse::<i32>())
                .verify(|res| res.is_ok())
                .map(|res| res.unwrap()),
            kv_unit,
        )
            .map(|(i, _)| i),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid integer");
            ErrMode::Cut(err.add_context(input, &checkpoint, StrContext::Label("Invalid integer")))
        } else {
            e
        }
    })
}

/// Fast u32 parser for KVN values.
pub fn kv_u32(input: &mut &str) -> KvnResult<u32> {
    let checkpoint = input.checkpoint();
    terminated(
        (
            take_while(1.., '0'..='9')
                .map(|s: &str| s.parse::<u32>())
                .verify(|res| res.is_ok())
                .map(|res| res.unwrap()),
            kv_unit,
        )
            .map(|(u, _)| u),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid unsigned integer");
            ErrMode::Cut(err.add_context(
                input,
                &checkpoint,
                StrContext::Label("Invalid unsigned integer"),
            ))
        } else {
            e
        }
    })
}

/// Parses an optional u32 value from a KVN line.
pub fn kv_u32_opt(input: &mut &str) -> KvnResult<Option<u32>> {
    let checkpoint = input.checkpoint();
    ws.parse_next(input)?;

    // Check if line contains only whitespace/unit or is empty
    let remainder = peek(till_line_ending).parse_next(input)?;
    if remainder.trim().is_empty() || remainder.trim().starts_with('[') {
        let _ = kv_unit.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        return Ok(None);
    }

    terminated(
        (
            take_while(1.., '0'..='9')
                .map(|s: &str| s.parse::<u32>())
                .verify(|res| res.is_ok())
                .map(|res| res.ok()),
            kv_unit,
        )
            .map(|(u, _)| u),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid unsigned integer");
            ErrMode::Cut(err.add_context(
                input,
                &checkpoint,
                StrContext::Label("Invalid unsigned integer"),
            ))
        } else {
            e
        }
    })
}

/// Fast u64 parser for KVN values.
pub fn kv_u64(input: &mut &str) -> KvnResult<u64> {
    let checkpoint = input.checkpoint();
    terminated(
        (
            take_while(1.., '0'..='9')
                .map(|s: &str| s.parse::<u64>())
                .verify(|res| res.is_ok())
                .map(|res| res.unwrap()),
            kv_unit,
        )
            .map(|(u, _)| u),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid unsigned integer");
            ErrMode::Cut(err.add_context(
                input,
                &checkpoint,
                StrContext::Label("Invalid unsigned integer"),
            ))
        } else {
            e
        }
    })
}

/// Parses an optional u64 value from a KVN line.
pub fn kv_u64_opt(input: &mut &str) -> KvnResult<Option<u64>> {
    let checkpoint = input.checkpoint();
    ws.parse_next(input)?;

    // Check if line contains only whitespace/unit or is empty
    let remainder = peek(till_line_ending).parse_next(input)?;
    if remainder.trim().is_empty() || remainder.trim().starts_with('[') {
        let _ = kv_unit.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        return Ok(None);
    }

    terminated(
        (
            take_while(1.., '0'..='9')
                .map(|s: &str| s.parse::<u64>())
                .verify(|res| res.is_ok())
                .map(|res| res.ok()),
            kv_unit,
        )
            .map(|(u, _)| u),
        opt_line_ending,
    )
    .parse_next(input)
    .map_err(|e| {
        if e.is_backtrack() {
            let mut err = InternalParserError::from_input(input);
            err.message = std::borrow::Cow::Borrowed("Invalid unsigned integer");
            ErrMode::Cut(err.add_context(
                input,
                &checkpoint,
                StrContext::Label("Invalid unsigned integer"),
            ))
        } else {
            e
        }
    })
}

/// Skips whitespace and empty lines.
pub fn skip_empty_lines(input: &mut &str) -> KvnResult<()> {
    repeat(0.., (space0, line_ending))
        .map(|_: ()| ()) // Corrected: map to () instead of consuming the tuple
        .parse_next(input)
}

/// Parses an optional line ending, consuming any trailing horizontal whitespace.
pub fn opt_line_ending(input: &mut &str) -> KvnResult<()> {
    (space0, opt(line_ending)).void().parse_next(input)
}

//----------------------------------------------------------------------
// Direct Value Parsers
//----------------------------------------------------------------------

/// Parses a string value from a KVN line.
pub fn kv_string(input: &mut &str) -> KvnResult<String> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    Ok(v.trim().to_string())
}

/// Parses an optional string value from a KVN line.
/// Returns None if empty.
pub fn kv_string_opt(input: &mut &str) -> KvnResult<Option<String>> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

/// Parses an Epoch value from a KVN line.
pub fn kv_epoch(input: &mut &str) -> KvnResult<Epoch> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    Epoch::from_str(v.trim())
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
}

/// Parses an optional Epoch value from a KVN line.
pub fn kv_epoch_opt(input: &mut &str) -> KvnResult<Option<Epoch>> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Epoch::from_str(trimmed)
            .map(Some)
            .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
    }
}

/// Parses an Epoch value as a single token (until next space).
pub fn kv_epoch_token(input: &mut &str) -> KvnResult<Epoch> {
    let v = till_space.parse_next(input)?;
    Epoch::from_str(v.trim())
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
}

/// Parses a boolean (YES/NO) from a KVN line.
pub fn kv_yes_no(input: &mut &str) -> KvnResult<YesNo> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    YesNo::from_str(v.trim())
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
}

/// Parses an optional boolean (YES/NO) from a KVN line.
pub fn kv_yes_no_opt(input: &mut &str) -> KvnResult<Option<YesNo>> {
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        YesNo::from_str(trimmed)
            .map(Some)
            .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
    }
}

/// Parses any type that implements FromStr from a KVN line.
pub fn kv_enum<T: FromStr>(input: &mut &str) -> KvnResult<T>
where
    EnumParseError: From<T::Err>,
{
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    T::from_str(v.trim()).map_err(|e| {
        ErrMode::Cut(InternalParserError::from_external_error(
            input,
            EnumParseError::from(e),
        ))
    })
}

/// Parses an optional type that implements FromStr from a KVN line.
pub fn kv_enum_opt<T: FromStr>(input: &mut &str) -> KvnResult<Option<T>>
where
    EnumParseError: From<T::Err>,
{
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        T::from_str(trimmed).map(Some).map_err(|e| {
            ErrMode::Cut(InternalParserError::from_external_error(
                input,
                EnumParseError::from(e),
            ))
        })
    }
}

/// Parses a value from a KVN line using the `FromKvnValue` trait.
pub fn kv_from_kvn_value<T: FromKvnValue>(input: &mut &str) -> KvnResult<T> {
    let (v, _) = kv_rest.parse_next(input)?;
    T::from_kvn_value(v)
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
}

/// Parses any type that implements FromKvnFloat from a KVN line.
pub fn kv_from_kvn<T: FromKvnFloat>(input: &mut &str) -> KvnResult<T> {
    let (v, u) = kv_float_unit.parse_next(input)?;
    T::from_kvn_float(v, u)
        .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
}

/// Parses any optional type that implements FromKvnFloat from a KVN line.
pub fn kv_from_kvn_opt<T: FromKvnFloat>(input: &mut &str) -> KvnResult<Option<T>> {
    let (v, u) = kv_float_unit_opt.parse_next(input)?;
    if let Some(val) = v {
        T::from_kvn_float(val, u)
            .map(Some)
            .map_err(|e| ErrMode::Cut(InternalParserError::from_external_error(input, e)))
    } else {
        Ok(None)
    }
}

/// Parses a float and its optional unit from a KVN line.
pub fn kv_float_unit<'a>(input: &mut &'a str) -> KvnResult<(f64, Option<&'a str>)> {
    terminated((parse_f64_winnow, kv_unit), opt_line_ending).parse_next(input)
}

//----------------------------------------------------------------------
// Value Parsing Helpers
//----------------------------------------------------------------------

/// Parses an f64 value from a string slice.
pub fn parse_f64(value: &str) -> crate::error::Result<f64> {
    value.trim().parse::<f64>().map_err(CcsdsNdmError::from)
}

/// Parses an i32 value from a string slice.
pub fn parse_i32(value: &str) -> crate::error::Result<i32> {
    value.trim().parse::<i32>().map_err(CcsdsNdmError::from)
}

/// Parses a u32 value from a string slice.
pub fn parse_u32(value: &str) -> crate::error::Result<u32> {
    value.trim().parse::<u32>().map_err(CcsdsNdmError::from)
}

/// Parses a u64 value from a string slice.
pub fn parse_u64(value: &str) -> crate::error::Result<u64> {
    value.trim().parse::<u64>().map_err(CcsdsNdmError::from)
}

//----------------------------------------------------------------------
// High-level Parsing Traits
//----------------------------------------------------------------------

/// Trait for types that can be parsed from KVN using winnow.
///
/// This is the primary trait for message-level parsing. Each message type
/// implements this trait to define how it parses from KVN.
pub trait ParseKvn: Sized {
    /// Parse the type from a KVN input stream.
    fn parse_kvn(input: &mut &str) -> KvnResult<Self>;

    /// Convenience method to parse from a string.
    fn from_kvn_str(s: &str) -> crate::error::Result<Self> {
        kvn_entry(Self::parse_kvn)
            .parse(s)
            .map_err(|e| to_ccsds_error(s, e))
    }
}

//----------------------------------------------------------------------
// Combinator Helpers
//----------------------------------------------------------------------

/// Parses a specific key-value pair by key name and applies a value parser.
/// Returns the parsed value and optional unit.
pub fn expect_kv<'a, T, P>(
    expected_key: &'static str,
    mut val_parser: P,
) -> impl FnMut(&mut &'a str) -> KvnResult<(T, Option<&'a str>)>
where
    P: winnow::Parser<&'a str, T, ErrMode<InternalParserError>>,
{
    move |input: &mut &'a str| {
        (
            ws,
            keyword.context(StrContext::Label("KVN keyword")),
            kv_sep,
        )
            .verify(|(_, key, _)| *key == expected_key)
            .context(StrContext::Expected(StrContextValue::Description(
                expected_key,
            )))
            .parse_next(input)?;

        let val = val_parser.parse_next(input)?;
        let unit = kv_unit.parse_next(input)?;
        opt_line_ending.parse_next(input)?;

        Ok((val, unit))
    }
}

/// Parses a specific key-value pair by key name.
/// Returns the value and optional unit.
pub fn expect_key<'a>(
    expected_key: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<(&'a str, Option<&'a str>)> {
    expect_kv(expected_key, kvn_value_only)
}

fn kvn_value_only<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    take_till(0.., |c: char| c == '[' || c == '\r' || c == '\n')
        .map(|s: &str| s.trim())
        .parse_next(input)
}

/// Parses a key-value pair where the key matches a predicate.
/// Returns (key, value, unit).
pub fn key_matching<'a, F>(
    predicate: F,
) -> impl FnMut(&mut &'a str) -> KvnResult<(&'a str, &'a str, Option<&'a str>)>
where
    F: Fn(&str) -> bool + Copy,
{
    move |input: &mut &'a str| {
        ws.parse_next(input)?;
        let key = keyword.parse_next(input)?;
        if !predicate(key) {
            return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
        }
        kv_sep.parse_next(input)?;
        let (value, unit) = kvn_value.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        Ok((key, value, unit))
    }
}

/// Skips comment lines and collects them into a Vec.
pub fn collect_comments(input: &mut &str) -> KvnResult<Vec<String>> {
    // Fast path: if no COMMENT or newline, return empty Vec immediately
    let checkpoint = input.checkpoint();
    let _ = ws.parse_next(input)?;
    if input.is_empty()
        || (!input.starts_with("COMMENT") && !input.starts_with('\r') && !input.starts_with('\n'))
    {
        input.reset(&checkpoint);
        return Ok(Vec::new());
    }
    input.reset(&checkpoint);

    repeat(
        0..,
        alt((
            // Corrected: removed unnecessary parentheses around alt
            preceded(ws, comment_line).map(|s| Some(s.trim().to_string())),
            (ws, line_ending).map(|_| None),
        )),
    )
    .fold(Vec::new, |mut acc: Vec<String>, item| {
        if let Some(s) = item {
            acc.push(s);
        }
        acc
    })
    .parse_next(input)
}

/// Skips empty lines and comments, discarding them.
pub fn skip_empty_and_comments(input: &mut &str) -> KvnResult<()> {
    loop {
        let checkpoint = input.checkpoint();
        if alt(((ws, comment_line).void(), (ws, line_ending).void()))
            .parse_next(input)
            .is_err()
        {
            input.reset(&checkpoint);
            break;
        }
    }
    let _ = ws.parse_next(input);
    Ok(())
}

/// Entry point for message parsers that handles leading whitespace.
pub fn kvn_entry<'a, O, P>(mut parser: P) -> impl FnMut(&mut &'a str) -> KvnResult<O>
where
    P: Parser<&'a str, O, ErrMode<InternalParserError>>,
{
    move |input: &mut &'a str| {
        skip_empty_and_comments.parse_next(input)?;
        let res = parser.parse_next(input)?;
        let _ = skip_empty_and_comments.parse_next(input);
        Ok(res)
    }
}

/// Macro for declarative KVN block parsing.
///
/// This macro generates a loop that matches keys using `dispatch!`,
/// handles comment collection, and provides helpful context on errors.
#[macro_export]
macro_rules! parse_block {
    // Flexible variant that supports both simple assignment and action blocks, with or without error label
    ($input:ident, $comments:expr, {
        $($($key:literal)|+ => $target:ident : $parser:expr $(=> $action:block)? ),* $(,)?
    }, $break_condition:expr $(, $error_label:expr)?)
     => {
        loop {
            let checkpoint = $input.checkpoint();
            let loop_comments = collect_comments.parse_next($input)?;

            if ($break_condition)(&mut *$input) {
                $comments.extend(loop_comments);
                break;
            }

            let key = match key_token.parse_next($input) {
                Ok(k) => k,
                Err(_) => {
                    $input.reset(&checkpoint);
                    break;
                }
            };

            match key {
                $( // Corrected: removed unnecessary parentheses around match arm
                    $($key)|+ => {
                        let val = $parser.parse_next($input)?;
                        parse_block!(@action $comments, loop_comments, val, $target $(, $action)?);
                    }
                )*
                _ => {
                    $input.reset(&checkpoint);
                    $( // Corrected: removed unnecessary parentheses around match arm
                        return Err(winnow::error::ErrMode::Cut(InternalParserError::from_input($input).add_context(
                            $input,
                            &$input.checkpoint(),
                            winnow::error::StrContext::Label($error_label),
                        )));
                    )?
                    #[allow(unreachable_code)]
                    {
                        break;
                    }
                }
            }
        }
    };

    // Internal helper for action handling
    (@action $comments:expr, $loop_comments:ident, $val:ident, $target:ident) => {
        $comments.extend($loop_comments);
        $target = Some($val);
    };
    (@action $comments:expr, $loop_comments:ident, $val:ident, $binding:ident, $action:block) => {
        $comments.extend($loop_comments);
        let $binding = $val;
        $action
    };
}

/// Checks if we're at a specific block start without full string scan.
pub fn at_block_start(tag: &str, input: &str) -> bool {
    let s = input.trim_start_matches([' ', '\t']);
    if let Some(rest) = s.strip_prefix(tag) {
        if let Some(suffix) = rest.strip_prefix("_START") {
            return suffix.starts_with('\r') || suffix.starts_with('\n') || suffix.is_empty();
        }
    }
    false
}

/// Checks if we're at a specific block end without full string scan.
pub fn at_block_end(tag: &str, input: &str) -> bool {
    let s = input.trim_start_matches([' ', '\t']);
    if let Some(rest) = s.strip_prefix(tag) {
        if let Some(suffix) = rest
            .strip_prefix("_STOP")
            .or_else(|| rest.strip_prefix("_END"))
        {
            return suffix.starts_with('\r') || suffix.starts_with('\n') || suffix.is_empty();
        }
    }
    false
}

/// Expects a specific block start and consumes it.
pub fn expect_block_start<'a>(
    expected_tag: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<()> {
    move |input: &mut &'a str| {
        (ws, block_start, opt_line_ending)
            .verify(|(_, tag, _)| *tag == expected_tag)
            .void()
            .context(StrContext::Label("Block start"))
            .context(StrContext::Expected(StrContextValue::Description(
                expected_tag,
            )))
            .parse_next(input)
    }
}

/// Expects a specific block end and consumes it.
pub fn expect_block_end<'a>(
    expected_tag: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<()> {
    move |input: &mut &'a str| {
        (ws, block_end, opt_line_ending)
            .verify(|(_, tag, _)| *tag == expected_tag)
            .void()
            .context(StrContext::Label("Block end"))
            .context(StrContext::Expected(StrContextValue::Description(
                expected_tag,
            )))
            .parse_next(input)
    }
}

/// Parses the ODM header section.
pub fn odm_header(input: &mut &str) -> KvnResult<OdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let checkpoint = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        let key = match preceded(ws, keyword).parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        // Stop if we encounter a metadata key
        if key == "OBJECT_NAME" || key == "META_START" {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = Some(kv_string.parse_next(input)?);
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = Some(kv_string.parse_next(input)?);
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(OdmHeader {
        comment,
        classification,
        creation_date: creation_date.ok_or_else(|| cut_err(input, "Expected CREATION_DATE"))?,
        originator: originator.ok_or_else(|| cut_err(input, "Expected ORIGINATOR"))?,
        message_id,
    })
}

/// Parses the ADM header section.
pub fn adm_header(input: &mut &str) -> KvnResult<AdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        let checkpoint = input.checkpoint();
        comment.extend(collect_comments.parse_next(input)?);

        let key = match preceded(ws, keyword).parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        // Stop if we encounter a metadata key
        // AEM/APM metadata usually starts with OBJECT_NAME or META_START
        if key == "OBJECT_NAME" || key == "META_START" {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = Some(kv_string.parse_next(input)?);
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = Some(kv_string.parse_next(input)?);
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    Ok(AdmHeader {
        comment,
        classification,
        creation_date: creation_date.ok_or_else(|| cut_err(input, "Expected CREATION_DATE"))?,
        originator: originator.ok_or_else(|| cut_err(input, "Expected ORIGINATOR"))?,
        message_id,
    })
}

//----------------------------------------------------------------------
// Common Parsers
//----------------------------------------------------------------------

/// Parses the state vector section.
pub fn state_vector(input: &mut &str) -> KvnResult<(Vec<String>, StateVector)> {
    let mut comment = Vec::new();
    let mut epoch = None;
    let mut x = None;
    let mut y = None;
    let mut z = None;
    let mut x_dot = None;
    let mut y_dot = None;
    let mut z_dot = None;

    parse_block!(input, comment, {
        "EPOCH" => epoch: kv_epoch,
        "X" => x: kv_from_kvn,
        "Y" => y: kv_from_kvn,
        "Z" => z: kv_from_kvn,
        "X_DOT" => x_dot: kv_from_kvn,
        "Y_DOT" => y_dot: kv_from_kvn,
        "Z_DOT" => z_dot: kv_from_kvn,
    }, |_| false);

    let sv = StateVector {
        comment: Vec::new(), // comments are returned separately for proper placement
        epoch: epoch.ok_or_else(|| missing_field_err(input, "State Vector", "EPOCH"))?,
        x: x.ok_or_else(|| missing_field_err(input, "State Vector", "X"))?,
        y: y.ok_or_else(|| missing_field_err(input, "State Vector", "Y"))?,
        z: z.ok_or_else(|| missing_field_err(input, "State Vector", "Z"))?,
        x_dot: x_dot.ok_or_else(|| missing_field_err(input, "State Vector", "X_DOT"))?,
        y_dot: y_dot.ok_or_else(|| missing_field_err(input, "State Vector", "Y_DOT"))?,
        z_dot: z_dot.ok_or_else(|| missing_field_err(input, "State Vector", "Z_DOT"))?,
    };

    Ok((comment, sv))
}

/// Parses the optional covariance matrix section.
pub fn covariance_matrix(input: &mut &str) -> KvnResult<Option<OpmCovarianceMatrix>> {
    let mut comment = Vec::new();
    let mut cov_ref_frame = None;
    let mut cx_x = None;
    let mut cy_x = None;
    let mut cy_y = None;
    let mut cz_x = None;
    let mut cz_y = None;
    let mut cz_z = None;
    let mut cx_dot_x = None;
    let mut cx_dot_y = None;
    let mut cx_dot_z = None;
    let mut cx_dot_x_dot = None;
    let mut cy_dot_x = None;
    let mut cy_dot_y = None;
    let mut cy_dot_z = None;
    let mut cy_dot_x_dot = None;
    let mut cy_dot_y_dot = None;
    let mut cz_dot_x = None;
    let mut cz_dot_y = None;
    let mut cz_dot_z = None;
    let mut cz_dot_x_dot = None;
    let mut cz_dot_y_dot = None;
    let mut cz_dot_z_dot = None;

    parse_block!(input, comment, {
        "COV_REF_FRAME" => cov_ref_frame: kv_string,
        "CX_X" => cx_x: kv_from_kvn,
        "CY_X" => cy_x: kv_from_kvn,
        "CY_Y" => cy_y: kv_from_kvn,
        "CZ_X" => cz_x: kv_from_kvn,
        "CZ_Y" => cz_y: kv_from_kvn,
        "CZ_Z" => cz_z: kv_from_kvn,
        "CX_DOT_X" => cx_dot_x: kv_from_kvn,
        "CX_DOT_Y" => cx_dot_y: kv_from_kvn,
        "CX_DOT_Z" => cx_dot_z: kv_from_kvn,
        "CX_DOT_X_DOT" => cx_dot_x_dot: kv_from_kvn,
        "CY_DOT_X" => cy_dot_x: kv_from_kvn,
        "CY_DOT_Y" => cy_dot_y: kv_from_kvn,
        "CY_DOT_Z" => cy_dot_z: kv_from_kvn,
        "CY_DOT_X_DOT" => cy_dot_x_dot: kv_from_kvn,
        "CY_DOT_Y_DOT" => cy_dot_y_dot: kv_from_kvn,
        "CZ_DOT_X" => cz_dot_x: kv_from_kvn,
        "CZ_DOT_Y" => cz_dot_y: kv_from_kvn,
        "CZ_DOT_Z" => cz_dot_z: kv_from_kvn,
        "CZ_DOT_X_DOT" => cz_dot_x_dot: kv_from_kvn,
        "CZ_DOT_Y_DOT" => cz_dot_y_dot: kv_from_kvn,
        "CZ_DOT_Z_DOT" => cz_dot_z_dot: kv_from_kvn,
    }, |_| false);

    // If we have covariance data, build the struct
    if cx_x.is_some() {
        Ok(Some(OpmCovarianceMatrix {
            comment,
            cov_ref_frame,
            cx_x: cx_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_X"))?,
            cy_x: cy_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_X"))?,
            cy_y: cy_y.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_Y"))?,
            cz_x: cz_x.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_X"))?,
            cz_y: cz_y.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_Y"))?,
            cz_z: cz_z.ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_Z"))?,
            cx_dot_x: cx_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_X"))?,
            cx_dot_y: cx_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_Y"))?,
            cx_dot_z: cx_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_Z"))?,
            cx_dot_x_dot: cx_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CX_DOT_X_DOT"))?,
            cy_dot_x: cy_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_X"))?,
            cy_dot_y: cy_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Y"))?,
            cy_dot_z: cy_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Z"))?,
            cy_dot_x_dot: cy_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_X_DOT"))?,
            cy_dot_y_dot: cy_dot_y_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CY_DOT_Y_DOT"))?,
            cz_dot_x: cz_dot_x
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_X"))?,
            cz_dot_y: cz_dot_y
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Y"))?,
            cz_dot_z: cz_dot_z
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Z"))?,
            cz_dot_x_dot: cz_dot_x_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_X_DOT"))?,
            cz_dot_y_dot: cz_dot_y_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Y_DOT"))?,
            cz_dot_z_dot: cz_dot_z_dot
                .ok_or_else(|| missing_field_err(input, "Covariance Matrix", "CZ_DOT_Z_DOT"))?,
        }))
    } else {
        Ok(None)
    }
}

/// Parses the optional spacecraft parameters section.
pub fn spacecraft_parameters(input: &mut &str) -> KvnResult<Option<SpacecraftParameters>> {
    let mut comment = Vec::new();
    let mut mass = None;
    let mut solar_rad_area = None;
    let mut solar_rad_coeff = None;
    let mut drag_area = None;
    let mut drag_coeff = None;

    parse_block!(input, comment, {
        "MASS" => mass: kv_from_kvn,
        "SOLAR_RAD_AREA" => solar_rad_area: kv_from_kvn,
        "SOLAR_RAD_COEFF" => solar_rad_coeff: kv_from_kvn,
        "DRAG_AREA" => drag_area: kv_from_kvn,
        "DRAG_COEFF" => drag_coeff: kv_from_kvn,
    }, |_| false);

    // If we have any spacecraft data, build the struct
    if mass.is_some() || solar_rad_area.is_some() || drag_area.is_some() {
        Ok(Some(SpacecraftParameters {
            comment,
            mass,
            solar_rad_area,
            solar_rad_coeff,
            drag_area,
            drag_coeff,
        }))
    } else {
        Ok(None)
    }
}

/// Parses user-defined parameters.
pub fn user_defined_parameters(input: &mut &str) -> KvnResult<Option<UserDefined>> {
    let mut comment = Vec::new();
    let mut params = Vec::new();

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        let key = match key_token.parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        if key.starts_with("USER_DEFINED_") {
            comment.extend(comments);
            let val = kv_string.parse_next(input)?;
            params.push(UserDefinedParameter {
                parameter: key.strip_prefix("USER_DEFINED_").unwrap().to_string(),
                value: val,
            });
        } else {
            // Backtrack and end user defined section
            input.reset(&checkpoint);
            break;
        }

        if input.offset_from(&checkpoint) == 0 {
            break;
        }
    }

    if params.is_empty() {
        Ok(None)
    } else {
        Ok(Some(UserDefined {
            comment,
            user_defined: params,
        }))
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        let mut input = "OBJECT_NAME";
        assert_eq!(keyword.parse_next(&mut input).unwrap(), "OBJECT_NAME");

        let mut input = "CCSDS_OPM_VERS";
        assert_eq!(keyword.parse_next(&mut input).unwrap(), "CCSDS_OPM_VERS");

        let mut input = "X_DOT";
        assert_eq!(keyword.parse_next(&mut input).unwrap(), "X_DOT");
    }

    #[test]
    fn test_kvn_value_without_unit() {
        let mut input = "SATELLITE-1\n";
        let (value, unit) = kvn_value.parse_next(&mut input).unwrap();
        assert_eq!(value, "SATELLITE-1");
        assert_eq!(unit, None);
    }

    #[test]
    fn test_kvn_value_with_unit() {
        let mut input = "6503.514 [km]\n";
        let (value, unit) = kvn_value.parse_next(&mut input).unwrap();
        assert_eq!(value, "6503.514");
        assert_eq!(unit, Some("km"));
    }

    #[test]
    fn test_key_value_line() {
        let mut input = "OBJECT_NAME = SATELLITE-1\n";
        let (key, value, unit) = key_value_line.parse_next(&mut input).unwrap();
        assert_eq!(key, "OBJECT_NAME");
        assert_eq!(value, "SATELLITE-1");
        assert_eq!(unit, None);

        let mut input = "X = 6503.514 [km]\n";
        let (key, value, unit) = key_value_line.parse_next(&mut input).unwrap();
        assert_eq!(key, "X");
        assert_eq!(value, "6503.514");
        assert_eq!(unit, Some("km"));
    }

    #[test]
    fn test_comment_line() {
        let mut input = "COMMENT This is a comment\n";
        let content = comment_line.parse_next(&mut input).unwrap();
        assert_eq!(content.trim(), "This is a comment");

        let mut input = "COMMENT\n";
        let content = comment_line.parse_next(&mut input).unwrap();
        assert_eq!(content.trim(), "");
    }

    #[test]
    fn test_block_start() {
        let mut input = "META_START\n";
        let tag = block_start.parse_next(&mut input).unwrap();
        assert_eq!(tag, "META");

        let mut input = "COVARIANCE_START\n";
        let tag = block_start.parse_next(&mut input).unwrap();
        assert_eq!(tag, "COVARIANCE");
    }

    #[test]
    fn test_block_end() {
        let mut input = "META_STOP\n";
        let tag = block_end.parse_next(&mut input).unwrap();
        assert_eq!(tag, "META");

        let mut input = "COVARIANCE_END\n";
        let tag = block_end.parse_next(&mut input).unwrap();
        assert_eq!(tag, "COVARIANCE");
    }

    #[test]
    fn test_expect_key() {
        let mut input = "OBJECT_NAME = SAT-1\n";
        let (value, unit) = expect_key("OBJECT_NAME").parse_next(&mut input).unwrap();
        assert_eq!(value, "SAT-1");
        assert_eq!(unit, None);
    }

    #[test]
    fn test_collect_comments() {
        let mut input = "COMMENT Line 1\nCOMMENT Line 2\nOBJECT_NAME = SAT\n";
        let comments = collect_comments.parse_next(&mut input).unwrap();
        assert_eq!(comments, vec!["Line 1", "Line 2"]);
    }

    #[test]
    fn test_raw_line() {
        let mut input = "2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0\n";
        let content = raw_line.parse_next(&mut input).unwrap();
        assert_eq!(content, "2023-01-01T00:00:00 1000 2000 3000 1.0 2.0 3.0");
    }

    #[test]
    fn test_malformed_key_value() {
        // Missing equals
        let mut input = "KEY value\n";
        assert!(key_value_line.parse_next(&mut input).is_err());

        // Missing value (valid in some contexts? No, value parser expects something)
        // Actually, empty values might be allowed if line ends?
        // kvn_value parses `take_till(...)`
        let mut input_empty = "KEY =\n";
        let (_, val, _) = key_value_line.parse_next(&mut input_empty).unwrap();
        assert_eq!(val, ""); // It allows empty values currently
    }

    #[test]
    fn test_whitespace_variations() {
        let mut input = "  KEY  =  value  [unit]  \n";
        let (key, val, unit) = key_value_line.parse_next(&mut input).unwrap();
        assert_eq!(key, "KEY");
        assert_eq!(val, "value");
        assert_eq!(unit, Some("unit"));
    }

    #[test]
    fn test_broken_unit() {
        let mut input = "KEY = value [unit\n"; // Missing closing bracket
        assert!(key_value_line.parse_next(&mut input).is_err());
    }
}
