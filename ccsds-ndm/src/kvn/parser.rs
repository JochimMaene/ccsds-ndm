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
use crate::traits::{CcsdsNullable, FromKvnFloat, FromKvnValue};
use crate::types::{UserDefined, UserDefinedParameter, *};
use std::str::FromStr;
use winnow::ascii::{line_ending, space0, till_line_ending};
use winnow::combinator::{alt, delimited, peek, preceded, repeat, terminated};
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
    parse_ccsds_number(s).ok_or_else(|| cut_err(input, "Invalid finite number or double underflow"))
}

/// Return whether a token has the shape of a CCSDS integer, fixed-point, or floating-point number
/// (ODM 7.5.4-7.5.7).
///
/// Reading does not apply the 16-digit cap of 7.5.6/7.5.7b or the 32-bit integer range of 7.5.4
/// to data values: many producers write the 17-digit shortest round-trip spelling of a double,
/// which loses nothing. Generation still writes at most 16 digits.
pub(crate) fn valid_ccsds_number(token: &str) -> bool {
    let bytes = token.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    if index == bytes.len() {
        return false;
    }

    let integer_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    let integer_digits = index - integer_start;
    let mut fraction_digits = 0;
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        let fraction_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        fraction_digits = index - fraction_start;
    }
    if integer_digits + fraction_digits == 0 {
        return false;
    }

    if index == bytes.len() {
        return true;
    }
    if !matches!(bytes.get(index), Some(b'e' | b'E')) {
        return false;
    }

    index += 1;
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }
    let exponent_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    index == bytes.len() && index > exponent_start
}

/// Parse a token in the CCSDS number grammar whose value lies within the double range of
/// ODM 7.5.7e. Overflow to infinity and a non-zero value that underflows to zero are rejected;
/// subnormal values are kept.
pub(crate) fn parse_ccsds_number(token: &str) -> Option<f64> {
    if !valid_ccsds_number(token) {
        return None;
    }
    let value: f64 = fast_float::parse(token).ok()?;
    let mantissa = token.split(['e', 'E']).next().unwrap_or(token);
    let underflow = value == 0.0 && mantissa.bytes().any(|byte| matches!(byte, b'1'..=b'9'));
    (value.is_finite() && !underflow).then_some(value)
}

/// Parses up to the next space or line ending, or end of input, skipping leading whitespace.
pub fn till_space<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    preceded(ws, take_till(1.., (' ', '\t', '\r', '\n'))).parse_next(input)
}

/// Parses a float and its optional unit from a KVN line, allowing empty input.
/// Returns Ok((None, u)) if no float found (but valid line end or unit-only).
pub fn kv_float_unit_opt<'a>(input: &mut &'a str) -> KvnResult<(Option<f64>, Option<&'a str>)> {
    ws.parse_next(input)?;

    // Try to parse a float (peek first to ensure we don't consume partial match tokens if not float?)
    // parse_f64_winnow works by take_while. If it takes nothing, it fails.
    if peek(parse_f64_winnow).parse_next(input).is_ok() {
        let value_start = *input;
        let f = parse_f64_winnow.parse_next(input)?;
        let u = kv_unit.parse_next(input)?;
        value_end(input, value_start, "Invalid float")?;
        Ok((Some(f), u))
    } else {
        // No float. Could be just unit, or empty.
        let u = kv_unit.parse_next(input)?;

        // After optional unit, we MUST be at end of line/comment or whitespace.
        // If there's still non-whitespace content, it's invalid (garbage).
        let remainder = till_line_ending.parse_next(input)?;
        if !remainder.is_null() {
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
        crate::error::ParserErrorKind::Validation(mut e) => {
            if let ValidationError::OutOfRange { name, .. } = &mut e {
                if let Some(key) = assignment_key(input, offset) {
                    *name = key.to_owned().into();
                }
            }
            CcsdsNdmError::Validation(Box::new(e))
        }
        // A bad KVN time tag is a value error like a bad number: report where it is.
        crate::error::ParserErrorKind::Epoch(e) => {
            let (line, column) = crate::error::line_column(input, offset);
            CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(
                crate::error::KvnParseError {
                    line,
                    column,
                    message: e.to_string(),
                    contexts: inner.contexts.to_vec(),
                    offset,
                },
            ))))
        }
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
            // A mandatory keyword with a blank value reads as absent; point at that record.
            let offset = blank_assignment_offset(input, field).unwrap_or(offset);
            return CcsdsNdmError::Validation(Box::new(ValidationError::MissingRequiredField {
                block: std::borrow::Cow::Borrowed(block),
                field: std::borrow::Cow::Borrowed(field),
                line: None,
            }))
            .with_location(input, offset);
        }
        _ => {
            let (line, column) = crate::error::line_column(input, offset);
            let mut message = inner.message.into_owned();
            if message.is_empty() {
                message = inner
                    .contexts
                    .last()
                    .map_or("unexpected or misplaced KVN record", |label| label)
                    .to_owned();
            }
            let mut contexts = inner.contexts.to_vec();
            // Value parsers do not know their keyword; the record they stopped on does.
            if message.starts_with("Invalid") {
                if let Some(key) = assignment_key(input, offset) {
                    contexts.retain(|context| *context != message);
                    message = format!("{message} for {key}");
                }
            }
            CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(
                crate::error::KvnParseError {
                    line,
                    column,
                    message,
                    contexts,
                    offset,
                },
            ))))
        }
    };

    base_err.with_location(input, offset)
}

/// Offset of the first `field =` record whose value is absent under [`value_is_absent`].
fn blank_assignment_offset(input: &str, field: &str) -> Option<usize> {
    let mut offset = 0;
    for line in input.split_inclusive('\n') {
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == field && value_is_absent(field, value) {
                return Some(offset);
            }
        }
        offset += line.len();
    }
    None
}

/// The keyword of the `KEY = value` record containing `offset`, if that record is an assignment.
fn assignment_key(input: &str, offset: usize) -> Option<&str> {
    let offset = offset.min(input.len());
    let start = input[..offset].rfind('\n').map_or(0, |index| index + 1);
    let end = input[start..]
        .find(['\r', '\n'])
        .map_or(input.len(), |index| start + index);
    let key = input[start..end].split_once('=')?.0.trim();
    (!key.is_empty() && key.bytes().all(|byte| byte.is_ascii_graphic())).then_some(key)
}

/// Creates a winnow ErrMode::Cut with a static context label.
pub fn cut_err(input: &mut &str, label: &'static str) -> ErrMode<InternalParserError> {
    let mut error = InternalParserError::from_input(input);
    error.message = std::borrow::Cow::Borrowed(label);
    ErrMode::Cut(error.add_context(input, &input.checkpoint(), StrContext::Label(label)))
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

/// Extracts a required field from an optional parser result.
pub fn require_field<T>(
    input: &mut &str,
    block: &'static str,
    field: &'static str,
    value: Option<T>,
) -> KvnResult<T> {
    value.ok_or_else(|| missing_field_err(input, block, field))
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

/// Parses a COMMENT line.
pub fn comment_line<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    ws.parse_next(input)?;
    "COMMENT".parse_next(input)?;
    if let Some(rest) = input.strip_prefix(' ') {
        *input = rest;
    } else if !input.is_empty() && !input.starts_with(['\r', '\n']) {
        return Err(ErrMode::Backtrack(InternalParserError::from_input(input)));
    }
    // Shared KVN readers normalize trailing spaces in comments, including fixed-width padding.
    till_line_ending.map(str::trim_end).parse_next(input)
}

/// Parses a key-value pair line.
#[cfg(test)]
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

/// Parses "KEY =" and returns the key.
pub fn key_token<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    terminated(preceded(ws, keyword), kv_sep).parse_next(input)
}

/// Parses a user-defined KVN keyword.
///
/// ODM 7.4.4 requires uppercase keywords without blanks but does not make Annex F's recommended
/// alphanumeric-and-underscore regex normative. `=` remains excluded because it delimits the
/// single assignment permitted by 7.4.3.
pub(crate) fn user_defined_key_token<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    terminated(
        preceded(
            ws,
            take_while(1.., |character: char| {
                character.is_ascii_graphic() && !character.is_ascii_lowercase() && character != '='
            }),
        ),
        kv_sep,
    )
    .parse_next(input)
}

/// Parses the rest of a KVN line (value and optional unit).
pub fn kv_rest<'a>(input: &mut &'a str) -> KvnResult<(&'a str, Option<&'a str>)> {
    terminated(kvn_value, opt_line_ending).parse_next(input)
}

/// Fast float parser for KVN values.
pub fn kv_float(input: &mut &str) -> KvnResult<f64> {
    let (value, unit) = kv_float_unit(input)?;
    if unit.is_some() {
        return Err(cut_err(
            input,
            "Units are not allowed for this dimensionless or implicitly scaled field",
        ));
    }
    Ok(value)
}

/// Whether only blanks remain before the end of the line or input.
fn at_value_end(input: &str) -> bool {
    let rest = input.trim_start_matches([' ', '\t']);
    rest.is_empty() || rest.starts_with(['\r', '\n'])
}

/// Parse a signed or unsigned KVN integer. Every book allows an optional leading sign (ODM
/// 7.5.4); an unsigned field still rejects `-`. A value such as `5.0` or `5x` fails as a whole,
/// not leaving `.0` for the next keyword.
fn kv_integer<T: FromStr>(input: &mut &str, label: &'static str) -> KvnResult<T> {
    let checkpoint = input.checkpoint();
    let parsed = (
        take_while(1.., ('0'..='9', '-', '+')).verify_map(|s: &str| s.parse::<T>().ok()),
        kv_unit,
    )
        .parse_next(input);
    let (value, unit) = match parsed {
        Ok(parsed) if at_value_end(input) => parsed,
        Err(e) if !e.is_backtrack() => return Err(e),
        _ => {
            input.reset(&checkpoint);
            *input = input.trim_start_matches([' ', '\t']);
            return Err(cut_err(input, label));
        }
    };
    opt_line_ending(input)?;
    if unit.is_some() {
        return Err(cut_err(input, "Units are not allowed for integer fields"));
    }
    Ok(value)
}

/// Fast i32 parser for KVN values.
pub fn kv_i32(input: &mut &str) -> KvnResult<i32> {
    kv_integer(input, "Invalid integer")
}

/// Fast u32 parser for KVN values.
pub fn kv_u32(input: &mut &str) -> KvnResult<u32> {
    kv_integer(input, "Invalid unsigned integer")
}

/// Parses an optional u32 value from a KVN line.
pub fn kv_u32_opt(input: &mut &str) -> KvnResult<Option<u32>> {
    kv_optional(kv_u32).parse_next(input)
}

/// Fast u64 parser for KVN values.
pub fn kv_u64(input: &mut &str) -> KvnResult<u64> {
    kv_integer(input, "Invalid unsigned integer")
}

/// Skips whitespace and empty lines.
pub fn skip_empty_lines(input: &mut &str) -> KvnResult<()> {
    repeat(0.., (space0, line_ending))
        .map(|_: ()| ()) // Corrected: map to () instead of consuming the tuple
        .parse_next(input)
}

/// Parses an optional line ending, consuming any trailing horizontal whitespace.
pub fn opt_line_ending(input: &mut &str) -> KvnResult<()> {
    *input = input.trim_start_matches([' ', '\t']);
    if let Some(rest) = input.strip_prefix("\r\n") {
        *input = rest;
    } else if let Some(rest) = input.strip_prefix('\n') {
        *input = rest;
    }
    Ok(())
}

/// Skips any run of blank lines.
pub fn blank_lines(input: &mut &str) -> KvnResult<()> {
    *input = after_blank_lines(input);
    Ok(())
}

/// The input after any run of blank lines; every book allows blank lines anywhere.
fn after_blank_lines(mut input: &str) -> &str {
    loop {
        let line = input.trim_start_matches([' ', '\t']);
        if let Some(rest) = line.strip_prefix("\r\n") {
            input = rest;
        } else if let Some(rest) = line.strip_prefix('\n') {
            input = rest;
        } else {
            return input;
        }
    }
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
    if trimmed.is_null() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

/// Parses an Epoch value from a KVN line.
pub fn kv_epoch(input: &mut &str) -> KvnResult<Epoch> {
    let value_start = *input;
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    Epoch::from_str(v.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Build a value error positioned at the start of the value rather than after its line, so
/// diagnostics name the line and column that hold the bad token.
fn external_error_at<'a, E>(
    input: &mut &'a str,
    value_start: &'a str,
    error: E,
) -> ErrMode<InternalParserError>
where
    InternalParserError: winnow::error::FromExternalError<&'a str, E>,
{
    *input = value_start.trim_start_matches([' ', '\t']);
    ErrMode::Cut(InternalParserError::from_external_error(input, error))
}

/// Parses an optional keyword value, treating a blank or `N/A` value field as absent.
///
/// ODM 7.5.1 requires non-empty values only for mandatory keywords.
pub fn kv_optional<'a, T>(
    mut value: impl FnMut(&mut &'a str) -> KvnResult<T>,
) -> impl FnMut(&mut &'a str) -> KvnResult<Option<T>> {
    move |input: &mut &'a str| {
        if value_is_absent("", input) {
            skip_record(input);
            return Ok(None);
        }
        value(input).map(Some)
    }
}

/// The shared optional-value rule for KVN: a blank value or case-insensitive `N/A` means the
/// keyword is absent. Block parsers apply it before any value parser, so it covers optional
/// numbers, epochs, enums, and strings alike; a mandatory keyword left blank is then reported as
/// missing. CDM `MANEUVERABLE` keeps `N/A` because the book lists it as a value (508.0-B-1
/// table 3-3).
pub(crate) fn value_is_absent(key: &str, rest: &str) -> bool {
    let value = rest[..rest.find(['\r', '\n']).unwrap_or(rest.len())].trim();
    value.is_empty() || (value.eq_ignore_ascii_case("N/A") && key != "MANEUVERABLE")
}

/// Consume the remainder of the current record and its line ending.
pub(crate) fn skip_record(input: &mut &str) {
    *input = &input[input.find(['\r', '\n']).unwrap_or(input.len())..];
    let _ = opt_line_ending(input);
}

/// Parses a calendar/ordinal epoch value from a KVN line.
pub fn kv_calendar_epoch(input: &mut &str) -> KvnResult<CalendarEpoch> {
    let value_start = *input;
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    CalendarEpoch::from_str(v.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses an optional calendar/ordinal epoch value from a KVN line.
pub fn kv_calendar_epoch_opt(input: &mut &str) -> KvnResult<Option<CalendarEpoch>> {
    let value_start = *input;
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_null() {
        Ok(None)
    } else {
        CalendarEpoch::from_str(trimmed)
            .map(Some)
            .map_err(|e| external_error_at(input, value_start, e))
    }
}

/// Parses an Epoch value as a single token (until next space).
pub fn kv_epoch_token(input: &mut &str) -> KvnResult<Epoch> {
    let value_start = *input;
    let v = till_space.parse_next(input)?;
    Epoch::from_str(v.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses a calendar/ordinal epoch value as a single KVN token (until the next space).
pub fn kv_calendar_epoch_token(input: &mut &str) -> KvnResult<CalendarEpoch> {
    let value_start = *input;
    let v = till_space.parse_next(input)?;
    CalendarEpoch::from_str(v.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses a finite relative time using the ADM/ACM `relTimeType` lexical rules.
pub fn kv_relative_time(input: &mut &str) -> KvnResult<RelativeTime> {
    let value_start = *input;
    let (value, units) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    if units.is_some_and(|unit| unit != "s") {
        return Err(cut_err(input, "relative time units must be s"));
    }
    RelativeTime::from_str(value.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses a boolean (YES/NO) from a KVN line.
pub fn kv_yes_no(input: &mut &str) -> KvnResult<YesNo> {
    let value_start = *input;
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    YesNo::from_str(v.trim()).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses an optional boolean (YES/NO) from a KVN line.
pub fn kv_yes_no_opt(input: &mut &str) -> KvnResult<Option<YesNo>> {
    let value_start = *input;
    let v = terminated(till_line_ending, opt_line_ending).parse_next(input)?;
    let trimmed = v.trim();
    if trimmed.is_null() {
        Ok(None)
    } else {
        YesNo::from_str(trimmed)
            .map(Some)
            .map_err(|e| external_error_at(input, value_start, e))
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
    if trimmed.is_null() {
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
    let value_start = *input;
    let (v, _) = kv_rest.parse_next(input)?;
    T::from_kvn_value(v).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses any type that implements FromKvnFloat from a KVN line.
pub fn kv_from_kvn<T: FromKvnFloat>(input: &mut &str) -> KvnResult<T> {
    let value_start = *input;
    let (v, u) = kv_float_unit.parse_next(input)?;
    T::from_kvn_float(v, u).map_err(|e| external_error_at(input, value_start, e))
}

/// Parses any optional type that implements FromKvnFloat from a KVN line.
pub fn kv_from_kvn_opt<T: FromKvnFloat>(input: &mut &str) -> KvnResult<Option<T>> {
    let value_start = *input;
    let (v, u) = kv_float_unit_opt.parse_next(input)?;
    if let Some(val) = v {
        T::from_kvn_float(val, u)
            .map(Some)
            .map_err(|e| external_error_at(input, value_start, e))
    } else {
        Ok(None)
    }
}

/// Parses a blank-separated list of CCSDS numbers and an optional unit, reporting a bad number
/// at its own record.
pub(crate) fn kv_float_list<'a>(input: &mut &'a str) -> KvnResult<(Vec<f64>, Option<&'a str>)> {
    let value_start = *input;
    let (values, unit) = terminated(kvn_value, opt_line_ending).parse_next(input)?;
    let Some(values) = values.split_whitespace().map(parse_ccsds_number).collect() else {
        *input = value_start.trim_start_matches([' ', '\t']);
        return Err(cut_err(input, "Invalid float"));
    };
    Ok((values, unit))
}

/// Parses a float and its optional unit from a KVN line.
pub fn kv_float_unit<'a>(input: &mut &'a str) -> KvnResult<(f64, Option<&'a str>)> {
    let value_start = *input;
    let value = (parse_f64_winnow, kv_unit).parse_next(input)?;
    value_end(input, value_start, "Invalid float")?;
    Ok(value)
}

/// Require that only blanks remain on the record, so `25x70 [km]` or `1000x` is reported as one
/// bad value instead of ending the block and blaming the next keyword.
fn value_end<'a>(input: &mut &'a str, value_start: &'a str, label: &'static str) -> KvnResult<()> {
    if at_value_end(input) {
        return opt_line_ending(input);
    }
    *input = value_start.trim_start_matches([' ', '\t']);
    Err(cut_err(input, label))
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

/// Parses a keyword whose value never carries units, such as a `CCSDS_xxx_VERS` line or an
/// epoch, rejecting a bracketed unit instead of silently dropping it.
pub fn expect_unitless_key<'a>(
    expected_key: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<&'a str> {
    move |input: &mut &'a str| {
        let line_start = *input;
        let (value, unit) = expect_key(expected_key).parse_next(input)?;
        if unit.is_some() {
            *input = line_start.trim_start_matches([' ', '\t']);
            return Err(cut_err(input, "units are not allowed on this keyword"));
        }
        Ok(value)
    }
}

fn kvn_value_only<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    take_till(0.., |c: char| c == '[' || c == '\r' || c == '\n')
        .map(|s: &str| s.trim())
        .parse_next(input)
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
            preceded(ws, comment_line).map(|s| Some(s.to_string())),
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
/// Names a range error after the KVN keyword that carried the value: shared value types such as
/// `Angle` or `Duration` serve many keywords and only know their own type name.
pub(crate) fn name_range_error(
    key: &str,
    mut error: ErrMode<InternalParserError>,
) -> ErrMode<InternalParserError> {
    if let ErrMode::Cut(inner) | ErrMode::Backtrack(inner) = &mut error {
        if let crate::error::ParserErrorKind::Validation(ValidationError::OutOfRange {
            name, ..
        }) = inner.kind.as_mut()
        {
            *name = key.to_string().into();
        }
    }
    error
}

/// This macro generates a loop that matches keys using `dispatch!`,
/// handles comment collection, and provides helpful context on errors.
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
                        if crate::kvn::parser::value_is_absent(key, $input) {
                            crate::kvn::parser::skip_record($input);
                            $comments.extend(loop_comments);
                            continue;
                        }
                        let val = $parser
                            .parse_next($input)
                            .map_err(|e| crate::kvn::parser::name_range_error(key, e))?;
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

pub(crate) use parse_block;

/// Parse a flattened KVN block while allowing the caller to associate comments with the logical
/// sub-block selected by the following keyword. Unlike `parse_block!`, comments at a boundary can
/// remain unread for the following parser.
macro_rules! parse_routed_block {
    ($input:ident, $route_comments:expr, {
        $($($key:literal)|+ => $target:ident : $parser:expr $(=> $action:block)? ),* $(,)?
    }, $break_condition:expr, $error_label:expr) => {
        loop {
            let checkpoint = $input.checkpoint();
            let loop_comments = collect_comments.parse_next($input)?;

            if ($break_condition)(&mut *$input) {
                $input.reset(&checkpoint);
                break;
            }

            let key = match key_token.parse_next($input) {
                Ok(key) => key,
                Err(_) => {
                    $input.reset(&checkpoint);
                    break;
                }
            };

            match key {
                $($($key)|+ => {
                    if crate::kvn::parser::value_is_absent(key, $input) {
                        crate::kvn::parser::skip_record($input);
                        ($route_comments)(key, loop_comments);
                        continue;
                    }
                    let value = $parser.parse_next($input)?;
                    ($route_comments)(key, loop_comments);
                    parse_routed_block!(@action value, $target $(, $action)?);
                })*
                _ => {
                    $input.reset(&checkpoint);
                    return Err(winnow::error::ErrMode::Cut(
                        winnow::error::AddContext::add_context(
                            crate::error::InternalParserError::from_input($input),
                            $input,
                            &$input.checkpoint(),
                            winnow::error::StrContext::Label($error_label),
                        ),
                    ));
                }
            }
        }
    };
    (@action $value:ident, $target:ident) => {
        $target = Some($value);
    };
    (@action $value:ident, $binding:ident, $action:block) => {
        let $binding = $value;
        $action
    };
}

pub(crate) use parse_routed_block;

/// Checks if we're at a specific block start without full string scan.
pub fn at_block_start(tag: &str, input: &str) -> bool {
    let s = after_blank_lines(input).trim_start_matches([' ', '\t']);
    if let Some(rest) = s.strip_prefix(tag) {
        if let Some(suffix) = rest.strip_prefix("_START") {
            return ends_line(suffix);
        }
    }
    false
}

/// Checks if we're at a specific block end without full string scan.
pub fn at_block_end(tag: &str, input: &str) -> bool {
    let s = after_blank_lines(input).trim_start_matches([' ', '\t']);
    if let Some(rest) = s.strip_prefix(tag) {
        if let Some(suffix) = rest
            .strip_prefix("_STOP")
            .or_else(|| rest.strip_prefix("_END"))
        {
            return ends_line(suffix);
        }
    }
    false
}

/// ODM 7.4.7: white space immediately before the end of a line is not significant.
fn ends_line(rest: &str) -> bool {
    let rest = rest.trim_start_matches([' ', '\t']);
    rest.is_empty() || rest.starts_with(['\r', '\n'])
}

/// Expects a specific block start and consumes it.
pub fn expect_block_start<'a>(
    expected_tag: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<()> {
    move |input: &mut &'a str| {
        blank_lines(input)?;
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
        blank_lines(input)?;
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
    odm_header_with_comments(input, false)
}

/// Optionally collect comments between header assignments for OEM producer compatibility.
pub(crate) fn odm_header_with_comments(
    input: &mut &str,
    comments_anywhere: bool,
) -> KvnResult<OdmHeader> {
    // Strict readers collect only initial comments. OEM also collects comments between header
    // assignments, leaving META_START for the body parser.
    let mut comment = collect_comments.parse_next(input)?;
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        if comments_anywhere {
            comment.extend(collect_comments.parse_next(input)?);
        }
        // ODM 7.3.5: blank lines may appear at any position.
        blank_lines.parse_next(input)?;
        let checkpoint = input.checkpoint();

        let key = match preceded(ws, keyword).parse_next(input) {
            Ok(k) => k,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };

        // Stop if we encounter a metadata key
        if key == "COMMENT" || key == "OBJECT_NAME" || key == "META_START" {
            input.reset(&checkpoint);
            break;
        }

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = kv_optional(kv_string).parse_next(input)?;
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_calendar_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = kv_optional(kv_string).parse_next(input)?;
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
        creation_date: creation_date
            .ok_or_else(|| missing_field_err(input, "Header", "CREATION_DATE"))?,
        originator: originator.ok_or_else(|| missing_field_err(input, "Header", "ORIGINATOR"))?,
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
        // Comments directly before the metadata belong to it (ADM 6.10.2), so they are kept
        // only once a header keyword follows.
        let loop_comments = collect_comments.parse_next(input)?;

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
        if !matches!(
            key,
            "CLASSIFICATION" | "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID"
        ) {
            input.reset(&checkpoint);
            break;
        }
        comment.extend(loop_comments);

        kv_sep.parse_next(input)?;
        match key {
            "CLASSIFICATION" => {
                classification = kv_optional(kv_string).parse_next(input)?;
            }
            "CREATION_DATE" => {
                creation_date = Some(kv_calendar_epoch.parse_next(input)?);
            }
            "ORIGINATOR" => {
                originator = Some(kv_string.parse_next(input)?);
            }
            "MESSAGE_ID" => {
                message_id = kv_optional(kv_string).parse_next(input)?;
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
        "EPOCH" => epoch: kv_calendar_epoch,
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

    let has_covariance = cov_ref_frame.is_some()
        || [
            cx_x.is_some(),
            cy_x.is_some(),
            cy_y.is_some(),
            cz_x.is_some(),
            cz_y.is_some(),
            cz_z.is_some(),
            cx_dot_x.is_some(),
            cx_dot_y.is_some(),
            cx_dot_z.is_some(),
            cx_dot_x_dot.is_some(),
            cy_dot_x.is_some(),
            cy_dot_y.is_some(),
            cy_dot_z.is_some(),
            cy_dot_x_dot.is_some(),
            cy_dot_y_dot.is_some(),
            cz_dot_x.is_some(),
            cz_dot_y.is_some(),
            cz_dot_z.is_some(),
            cz_dot_x_dot.is_some(),
            cz_dot_y_dot.is_some(),
            cz_dot_z_dot.is_some(),
        ]
        .into_iter()
        .any(|present| present);
    if has_covariance {
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
    if mass.is_some()
        || solar_rad_area.is_some()
        || solar_rad_coeff.is_some()
        || drag_area.is_some()
        || drag_coeff.is_some()
        || !comment.is_empty()
    {
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
    let mut entries = Vec::new();
    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;
        let key = match user_defined_key_token.parse_next(input) {
            Ok(key) => key,
            Err(_) => {
                input.reset(&checkpoint);
                break;
            }
        };
        let Some(parameter) = key.strip_prefix("USER_DEFINED_") else {
            input.reset(&checkpoint);
            break;
        };
        entries.extend(comments.into_iter().map(UserDefinedEntry::Comment));
        if value_is_absent(key, input) {
            skip_record(input);
            continue;
        }
        let value = kv_string.parse_next(input)?;
        entries.push(UserDefinedEntry::Parameter(UserDefinedParameter {
            parameter: parameter.to_string(),
            value,
        }));
    }
    Ok((!entries.is_empty()).then_some(UserDefined { entries }))
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_kvn_markers_consume_the_complete_record() {
        let mut input = "N/A\nNEXT = 1\n";
        assert_eq!(kv_u32_opt(&mut input).unwrap(), None);
        assert_eq!(input, "NEXT = 1\n");
    }

    #[test]
    fn numeric_reader_rejects_overflow_underflow_and_malformed_tokens() {
        for token in [".5", "5.", "1e3", "12.5e-3", "1.2345678901234567", "5e-324"] {
            assert!(parse_ccsds_number(token).is_some(), "{token}");
        }
        for token in ["1e999", "1e-999", "1..0", ".", "+", "1e", "NaN", "INF"] {
            assert!(parse_ccsds_number(token).is_none(), "{token}");
        }
    }

    #[test]
    fn partial_covariance_is_an_error_and_spacecraft_coefficients_are_preserved() {
        assert!(covariance_matrix(&mut "CY_Y = 1.0\n").is_err());
        assert!(covariance_matrix(&mut "COV_REF_FRAME = GCRF\n").is_err());
        let block = spacecraft_parameters(&mut "DRAG_COEFF = 0.5\n")
            .unwrap()
            .unwrap();
        assert_eq!(block.drag_coeff.unwrap().value, 0.5);
    }

    #[test]
    fn ccsds_number_lexical_validation_matches_book_forms() {
        for value in [
            "0",
            "-2147483648",
            "2147483647",
            "+00000000000000000001",
            "-00000000002147483648",
            "0.0",
            "-12.5",
            "1.234567890123456",
            "1.0e0",
            "-1.234567890123456E+308",
            // Beyond the book's 16-digit and 32-bit limits, which reading does not apply.
            "2147483648",
            "00000000002147483648",
            "1.2345678901234567",
            "-2757.3016318893897",
            ".5",
            "1.",
            "1.e3",
            "12e3",
            "1e3",
            "12.5e3",
        ] {
            assert!(valid_ccsds_number(value), "{value}");
        }
        for value in ["", "+", ".", "1.0e", "1.0e+", "1e2e3", "1..2", "--1"] {
            assert!(!valid_ccsds_number(value), "{value}");
        }
    }

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
        assert_eq!(content, "This is a comment");

        let mut input = "COMMENT\n";
        let content = comment_line.parse_next(&mut input).unwrap();
        assert_eq!(content, "");

        let mut input = "COMMENT    indented   \n";
        let content = comment_line.parse_next(&mut input).unwrap();
        assert_eq!(content, "   indented");
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

    #[test]
    fn test_require_field_success() {
        let mut input = "";
        let value = require_field(&mut input, "TestBlock", "TEST_FIELD", Some(42_u32)).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_require_field_missing() {
        let mut input = "";
        let err = require_field::<u32>(&mut input, "TestBlock", "TEST_FIELD", None).unwrap_err();
        match err {
            ErrMode::Cut(inner) => match *inner.kind {
                crate::error::ParserErrorKind::MissingRequiredField { block, field } => {
                    assert_eq!(block, "TestBlock");
                    assert_eq!(field, "TEST_FIELD");
                }
                _ => panic!("Expected MissingRequiredField"),
            },
            _ => panic!("Expected cut error"),
        }
    }

    #[test]
    fn adm_header_creation_date_requires_calendar_epoch() {
        for value in ["123.5", "2023-02-29T00:00:00", ""] {
            let input = format!("CREATION_DATE = {value}\nORIGINATOR = TEST\n");
            assert!(
                adm_header.parse(input.as_str()).is_err(),
                "accepted {value:?}"
            );
        }

        let mut input = "CREATION_DATE = 2023-060T00:00:00\nORIGINATOR = TEST\n";
        let header = adm_header.parse_next(&mut input).unwrap();
        assert_eq!(header.creation_date.as_str(), "2023-060T00:00:00");
    }
}
