// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
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

use crate::error::CcsdsNdmError;
use winnow::ascii::{line_ending, space0, till_line_ending};
use winnow::combinator::{alt, opt, peek, repeat};
use winnow::error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::ModalResult;

//----------------------------------------------------------------------
// Error Handling
//----------------------------------------------------------------------

/// Converts a winnow error to our library's error type.
pub fn to_ccsds_error(
    input: &str,
    err: winnow::error::ParseError<&str, ContextError>,
) -> CcsdsNdmError {
    let consumed = err.offset();
    let line_num = input[..consumed].lines().count().max(1);

    CcsdsNdmError::KvnParse {
        line: line_num,
        message: format!("{}", err.inner()),
    }
}

//----------------------------------------------------------------------
// Low-level Token Parsers
//----------------------------------------------------------------------

/// Parses optional whitespace (spaces and tabs only, not newlines).
pub fn ws<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    space0.parse_next(input)
}

/// Parses a KVN keyword (uppercase letters, digits, underscores).
/// Keywords must start with a letter.
pub fn keyword<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    (
        take_while(1.., |c: char| c.is_ascii_uppercase()),
        take_while(0.., |c: char| {
            c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
        }),
    )
        .take()
        .parse_next(input)
}

/// Parses the `= ` separator in a key-value pair.
pub fn kv_sep(input: &mut &str) -> ModalResult<()> {
    (ws, '=', ws).void().parse_next(input)
}

/// Parses the value part of a key-value pair.
/// Handles values with or without units.
pub fn kvn_value<'a>(input: &mut &'a str) -> ModalResult<(&'a str, Option<&'a str>)> {
    let value_str = till_line_ending.parse_next(input)?;
    let value_str = value_str.trim();

    // Check if the value ends with a unit specification
    if value_str.ends_with(']') {
        if let Some(open) = value_str.rfind('[') {
            let val = value_str[..open].trim();
            let unit = value_str[open + 1..value_str.len() - 1].trim();
            return Ok((val, Some(unit)));
        }
    }

    Ok((value_str, None))
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
pub fn comment_line<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    ("COMMENT", space0).parse_next(input)?;
    till_line_ending.parse_next(input)
}

/// Parses a key-value pair line.
pub fn key_value_line<'a>(input: &mut &'a str) -> ModalResult<(&'a str, &'a str, Option<&'a str>)> {
    let key = keyword.parse_next(input)?;
    kv_sep.parse_next(input)?;
    let (value, unit) = kvn_value.parse_next(input)?;
    Ok((key, value, unit))
}

/// Parses a block start marker (e.g., META_START).
pub fn block_start<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let content = till_line_ending.parse_next(input)?;
    let content = content.trim();

    if let Some(prefix) = content.strip_suffix("_START") {
        if !prefix.contains(char::is_whitespace) {
            return Ok(prefix);
        }
    }
    Err(ErrMode::Backtrack(ContextError::new()))
}

/// Parses a block end marker (e.g., META_STOP or COVARIANCE_END).
pub fn block_end<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let content = till_line_ending.parse_next(input)?;
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
    Err(ErrMode::Backtrack(ContextError::new()))
}

/// Parses an empty line.
pub fn empty_line(input: &mut &str) -> ModalResult<()> {
    (
        space0,
        peek(alt((line_ending.void(), winnow::combinator::eof.void()))),
    )
        .void()
        .parse_next(input)
}

/// Parses a raw data line (no equals sign, not a keyword).
pub fn raw_line<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let content = till_line_ending.parse_next(input)?;
    let trimmed = content.trim();

    // Raw lines should not be empty, comments, or contain '='
    if trimmed.is_empty()
        || trimmed.starts_with("COMMENT")
        || trimmed.contains('=')
        || trimmed.ends_with("_START")
        || trimmed.ends_with("_STOP")
        || trimmed.ends_with("_END")
    {
        return Err(ErrMode::Backtrack(ContextError::new()));
    }

    Ok(trimmed)
}

/// Parses any KVN line into a token.
pub fn kvn_token<'a>(input: &mut &'a str) -> ModalResult<KvnToken<'a>> {
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

/// Skips whitespace and empty lines.
pub fn skip_empty_lines(input: &mut &str) -> ModalResult<()> {
    repeat(0.., (space0, line_ending))
        .map(|_: ()| ())
        .parse_next(input)
}

/// Parses an optional line ending.
pub fn opt_line_ending(input: &mut &str) -> ModalResult<()> {
    opt(line_ending).void().parse_next(input)
}

//----------------------------------------------------------------------
// Value Parsing Helpers
//----------------------------------------------------------------------

/// Parses an f64 value from a string slice.
pub fn parse_f64(value: &str) -> crate::error::Result<f64> {
    value
        .trim()
        .parse::<f64>()
        .map_err(|e| CcsdsNdmError::KvnParse {
            line: 0,
            message: format!("Invalid float '{}': {}", value, e),
        })
}

/// Parses an i32 value from a string slice.
pub fn parse_i32(value: &str) -> crate::error::Result<i32> {
    value
        .trim()
        .parse::<i32>()
        .map_err(|e| CcsdsNdmError::KvnParse {
            line: 0,
            message: format!("Invalid integer '{}': {}", value, e),
        })
}

/// Parses a u32 value from a string slice.
pub fn parse_u32(value: &str) -> crate::error::Result<u32> {
    value
        .trim()
        .parse::<u32>()
        .map_err(|e| CcsdsNdmError::KvnParse {
            line: 0,
            message: format!("Invalid unsigned integer '{}': {}", value, e),
        })
}

/// Parses a u64 value from a string slice.
pub fn parse_u64(value: &str) -> crate::error::Result<u64> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|e| CcsdsNdmError::KvnParse {
            line: 0,
            message: format!("Invalid unsigned 64-bit integer '{}': {}", value, e),
        })
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
    fn parse_kvn(input: &mut &str) -> ModalResult<Self>;

    /// Convenience method to parse from a string.
    fn from_kvn_str(s: &str) -> crate::error::Result<Self> {
        let mut input = s;
        Self::parse_kvn(&mut input).map_err(|e| match e {
            ErrMode::Backtrack(ctx) | ErrMode::Cut(ctx) => {
                let consumed = s.len() - input.len();
                let line_num = s[..consumed].lines().count().max(1);
                CcsdsNdmError::KvnParse {
                    line: line_num,
                    message: format!("{}", ctx),
                }
            }
            ErrMode::Incomplete(_) => CcsdsNdmError::UnexpectedEof {
                context: "Incomplete KVN input".into(),
            },
        })
    }
}

//----------------------------------------------------------------------
// Combinator Helpers
//----------------------------------------------------------------------

/// Parses a specific key-value pair by key name.
/// Returns the value and optional unit.
pub fn expect_key<'a>(
    expected_key: &'static str,
) -> impl FnMut(&mut &'a str) -> ModalResult<(&'a str, Option<&'a str>)> {
    move |input: &mut &'a str| {
        ws.parse_next(input)?;
        let key = keyword.parse_next(input)?;
        if key != expected_key {
            return Err(ErrMode::Backtrack(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(expected_key)),
            )));
        }
        kv_sep.parse_next(input)?;
        let (value, unit) = kvn_value.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        Ok((value, unit))
    }
}

/// Parses a key-value pair where the key matches a predicate.
/// Returns (key, value, unit).
pub fn key_matching<'a, F>(
    predicate: F,
) -> impl FnMut(&mut &'a str) -> ModalResult<(&'a str, &'a str, Option<&'a str>)>
where
    F: Fn(&str) -> bool + Copy,
{
    move |input: &mut &'a str| {
        ws.parse_next(input)?;
        let key = keyword.parse_next(input)?;
        if !predicate(key) {
            return Err(ErrMode::Backtrack(ContextError::new()));
        }
        kv_sep.parse_next(input)?;
        let (value, unit) = kvn_value.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        Ok((key, value, unit))
    }
}

/// Skips comment lines and collects them into a Vec.
pub fn collect_comments(input: &mut &str) -> ModalResult<Vec<String>> {
    let mut comments = Vec::new();

    loop {
        let _ = ws.parse_next(input);

        // Check if we're at end of input
        if input.is_empty() {
            break;
        }

        // Try to parse a comment
        let checkpoint = input.checkpoint();
        if let Ok(content) = comment_line.parse_next(input) {
            comments.push(content.trim().to_string());
            let _ = opt_line_ending.parse_next(input);
            continue;
        }
        input.reset(&checkpoint);

        // Try to skip empty lines
        let checkpoint = input.checkpoint();
        if empty_line.parse_next(input).is_ok() {
            let consumed = opt_line_ending.parse_next(input);
            // If we didn't consume anything (e.g., at EOF), break
            if consumed.is_err() || input.is_empty() {
                break;
            }
            continue;
        }
        input.reset(&checkpoint);

        break;
    }

    Ok(comments)
}

/// Skips empty lines and comments, discarding them.
pub fn skip_empty_and_comments(input: &mut &str) -> ModalResult<()> {
    loop {
        let _ = ws.parse_next(input);

        // Check if we're at end of input
        if input.is_empty() {
            break;
        }

        // Skip empty lines
        let checkpoint = input.checkpoint();
        if empty_line.parse_next(input).is_ok() {
            let consumed = opt_line_ending.parse_next(input);
            if consumed.is_err() || input.is_empty() {
                break;
            }
            continue;
        }
        input.reset(&checkpoint);

        // Skip comments
        let checkpoint = input.checkpoint();
        if comment_line.parse_next(input).is_ok() {
            let _ = opt_line_ending.parse_next(input);
            continue;
        }
        input.reset(&checkpoint);

        break;
    }
    Ok(())
}

/// Peeks at the next key without consuming input.
/// Returns None if the next token is not a key-value pair.
pub fn peek_key<'a>(input: &mut &'a str) -> ModalResult<Option<&'a str>> {
    let checkpoint = input.checkpoint();

    // Skip whitespace
    let _ = ws.parse_next(input);

    // Try to get the key
    let result = keyword.parse_next(input);

    // Restore position
    input.reset(&checkpoint);

    match result {
        Ok(key) => Ok(Some(key)),
        Err(_) => Ok(None),
    }
}

/// Checks if we're at a specific block start.
pub fn at_block_start(tag: &str, input: &mut &str) -> bool {
    let checkpoint = input.checkpoint();
    let _ = ws.parse_next(input);
    let result = block_start.parse_next(input);
    input.reset(&checkpoint);
    matches!(result, Ok(t) if t == tag)
}

/// Checks if we're at a specific block end.
pub fn at_block_end(tag: &str, input: &mut &str) -> bool {
    let checkpoint = input.checkpoint();
    let _ = ws.parse_next(input);
    let result = block_end.parse_next(input);
    input.reset(&checkpoint);
    matches!(result, Ok(t) if t == tag)
}

/// Expects a specific block start and consumes it.
pub fn expect_block_start<'a>(
    expected_tag: &'static str,
) -> impl FnMut(&mut &'a str) -> ModalResult<()> {
    move |input: &mut &'a str| {
        let _ = ws.parse_next(input);
        let tag = block_start.parse_next(input).map_err(|e| {
            e.add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(
                    format!("{}_START", expected_tag).leak(),
                )),
            )
        })?;
        if tag != expected_tag {
            return Err(ErrMode::Backtrack(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(
                    format!("{}_START", expected_tag).leak(),
                )),
            )));
        }
        let _ = opt_line_ending.parse_next(input);
        Ok(())
    }
}

/// Expects a specific block end and consumes it.
pub fn expect_block_end<'a>(
    expected_tag: &'static str,
) -> impl FnMut(&mut &'a str) -> ModalResult<()> {
    move |input: &mut &'a str| {
        let _ = ws.parse_next(input);
        let tag = block_end.parse_next(input).map_err(|e| {
            e.add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(
                    format!("{}_STOP", expected_tag).leak(),
                )),
            )
        })?;
        if tag != expected_tag {
            return Err(ErrMode::Backtrack(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(
                    format!("{}_STOP", expected_tag).leak(),
                )),
            )));
        }
        let _ = opt_line_ending.parse_next(input);
        Ok(())
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
}
