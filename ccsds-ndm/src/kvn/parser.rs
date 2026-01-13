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

use crate::common::{OdmHeader, OpmCovarianceMatrix, SpacecraftParameters, StateVector};
pub use crate::error::CcsdsNdmError;
use crate::types::{UserDefined, UserDefinedParameter, *};
use std::str::FromStr;
use winnow::ascii::{float, line_ending, space0, till_line_ending};
use winnow::combinator::{alt, delimited, opt, peek, preceded, repeat, terminated};
use winnow::error::{AddContext, ErrMode, ParserError, StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::token::{one_of, take_till, take_while};

/// A result type for winnow parsers using the library's error type.
pub type KvnResult<O, E = CcsdsNdmError> = Result<O, ErrMode<E>>;

//----------------------------------------------------------------------
// Low-level fast parsers
//----------------------------------------------------------------------

/// Parses a float directly from the input.
pub fn parse_f64_winnow(input: &mut &str) -> KvnResult<f64> {
    float.parse_next(input)
}

/// Parses up to the next space or line ending.
pub fn till_space<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    take_till(1.., (' ', '\t', '\r', '\n')).parse_next(input)
}

/// Parses up to the next space or line ending, or end of input.
pub fn till_space_or_eol<'a>(input: &mut &'a str) -> KvnResult<&'a str> {
    take_till(1.., (' ', '\t', '\r', '\n')).parse_next(input)
}

//----------------------------------------------------------------------
// Error Handling
//----------------------------------------------------------------------

/// Converts a winnow error to our library's error type.
pub fn to_ccsds_error(
    _input: &str,
    err: winnow::error::ParseError<&str, CcsdsNdmError>,
) -> CcsdsNdmError {
    err.into_inner()
}

/// Creates a winnow ErrMode::Cut with a static context label.
pub fn cut_err(input: &mut &str, label: &'static str) -> ErrMode<CcsdsNdmError> {
    ErrMode::Cut(CcsdsNdmError::from_input(input).add_context(
        input,
        &input.checkpoint(),
        StrContext::Label(label),
    ))
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
        // If it's something like [km], and nothing else follows, we'll treat it as value for now
        // if it was really intended as a unit with no value, that's weird but possible.
        // Actually, let's just return the whole thing as value if it's bracketed and no value preceded.
        Ok((trimmed, None))
    } else {
        let unit =
            opt(delimited('[', take_till(0.., |c: char| c == ']'), ']')).parse_next(input)?;
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
    Err(ErrMode::Backtrack(CcsdsNdmError::from_input(input)))
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
    Err(ErrMode::Backtrack(CcsdsNdmError::from_input(input)))
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
        return Err(ErrMode::Backtrack(CcsdsNdmError::from_input(input)));
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
    terminated(
        (
            float,
            opt(preceded(
                ws,
                delimited('[', take_till(0.., |c: char| c == ']'), ']'),
            )),
        )
            .map(|(f, _)| f),
        opt_line_ending,
    )
    .parse_next(input)
}

/// Fast i32 parser for KVN values.
pub fn kv_i32(input: &mut &str) -> KvnResult<i32> {
    use winnow::ascii::dec_int;
    terminated(
        (
            dec_int,
            opt(preceded(
                ws,
                delimited('[', take_till(0.., |c: char| c == ']'), ']'),
            )),
        )
            .map(|(i, _)| i),
        opt_line_ending,
    )
    .parse_next(input)
}

/// Fast u32 parser for KVN values.
pub fn kv_u32(input: &mut &str) -> KvnResult<u32> {
    use winnow::ascii::dec_uint;
    terminated(
        (
            dec_uint,
            opt(preceded(
                ws,
                delimited('[', take_till(0.., |c: char| c == ']'), ']'),
            )),
        )
            .map(|(u, _)| u),
        opt_line_ending,
    )
    .parse_next(input)
}

/// Skips whitespace and empty lines.
pub fn skip_empty_lines(input: &mut &str) -> KvnResult<()> {
    repeat(0.., (space0, line_ending))
        .map(|_: ()| ())
        .parse_next(input)
}

/// Parses an optional line ending, consuming any trailing horizontal whitespace.
pub fn opt_line_ending(input: &mut &str) -> KvnResult<()> {
    (space0, opt(line_ending)).void().parse_next(input)
}

//----------------------------------------------------------------------
// Value Parsing Helpers
//----------------------------------------------------------------------

/// Parses an f64 value from a string slice.
pub fn parse_f64(value: &str) -> crate::error::Result<f64> {
    value
        .trim()
        .parse::<f64>()
        .map_err(CcsdsNdmError::ParseFloat)
}

/// Parses an i32 value from a string slice.
pub fn parse_i32(value: &str) -> crate::error::Result<i32> {
    value.trim().parse::<i32>().map_err(CcsdsNdmError::ParseInt)
}

/// Parses a u32 value from a string slice.
pub fn parse_u32(value: &str) -> crate::error::Result<u32> {
    value.trim().parse::<u32>().map_err(CcsdsNdmError::ParseInt)
}

/// Parses a u64 value from a string slice.
pub fn parse_u64(value: &str) -> crate::error::Result<u64> {
    value.trim().parse::<u64>().map_err(CcsdsNdmError::ParseInt)
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

/// Parses a specific key-value pair by key name.
/// Returns the value and optional unit.
pub fn expect_key<'a>(
    expected_key: &'static str,
) -> impl FnMut(&mut &'a str) -> KvnResult<(&'a str, Option<&'a str>)> {
    move |input: &mut &'a str| {
        (
            ws,
            keyword.context(StrContext::Label("KVN keyword")),
            kv_sep,
            kvn_value,
            opt_line_ending,
        )
            .verify(|(_, key, _, _, _)| *key == expected_key)
            .map(|(_, _, _, (val, unit), _)| (val, unit))
            .context(StrContext::Expected(StrContextValue::Description(
                expected_key,
            )))
            .parse_next(input)
    }
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
            return Err(ErrMode::Backtrack(CcsdsNdmError::from_input(input)));
        }
        kv_sep.parse_next(input)?;
        let (value, unit) = kvn_value.parse_next(input)?;
        opt_line_ending.parse_next(input)?;
        Ok((key, value, unit))
    }
}

/// Skips comment lines and collects them into a Vec.
pub fn collect_comments(input: &mut &str) -> KvnResult<Vec<String>> {
    repeat(
        0..,
        alt((
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
    repeat(
        0..,
        alt(((ws, comment_line).void(), (ws, line_ending).void())),
    )
    .parse_next(input)
}

/// Entry point for message parsers that handles leading whitespace.
pub fn kvn_entry<'a, O, P>(mut parser: P) -> impl FnMut(&mut &'a str) -> KvnResult<O>
where
    P: Parser<&'a str, O, ErrMode<CcsdsNdmError>>,
{
    move |input: &mut &'a str| {
        ws.parse_next(input)?;
        parser.parse_next(input)
    }
}

/// Peeks at the next key without consuming input.
/// Returns None if the next token is not a key-value pair.
pub fn peek_key<'a>(input: &mut &'a str) -> KvnResult<Option<&'a str>> {
    peek(opt(preceded(ws, keyword))).parse_next(input)
}

/// Checks if we're at a specific block start without full string scan.
pub fn at_block_start(tag: &str, input: &mut &str) -> bool {
    peek((
        ws,
        tag,
        "_START",
        alt((line_ending, winnow::combinator::eof)),
    ))
    .parse_next(input)
    .is_ok()
}

/// Checks if we're at a specific block end without full string scan.
pub fn at_block_end(tag: &str, input: &mut &str) -> bool {
    peek((
        ws,
        tag,
        alt(("_STOP", "_END")),
        alt((line_ending, winnow::combinator::eof)),
    ))
    .parse_next(input)
    .is_ok()
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

fn is_header_key(key: &str) -> bool {
    matches!(
        key,
        "CLASSIFICATION" | "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID"
    )
}

/// Parses the ODM header section.
pub fn odm_header(input: &mut &str) -> KvnResult<OdmHeader> {
    let mut comment = Vec::new();
    let mut classification = None;
    let mut creation_date = None;
    let mut originator = None;
    let mut message_id = None;

    loop {
        // Collect any comments
        let comments = collect_comments.parse_next(input)?;
        comment.extend(comments);

        // Check what's next
        let next_key = peek_key(input)?;

        match next_key {
            Some(key) if is_header_key(key) => {
                let (k, v, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match k {
                    "CLASSIFICATION" => classification = Some(v.to_string()),
                    "CREATION_DATE" => {
                        creation_date =
                            Some(Epoch::from_str(v).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    "ORIGINATOR" => originator = Some(v.to_string()),
                    "MESSAGE_ID" => message_id = Some(v.to_string()),
                    _ => {}
                }
            }
            // Any other key signals end of header
            _ => break,
        }
    }

    let creation_date = creation_date.ok_or_else(|| cut_err(input, "Missing required value"))?;
    let originator = originator.ok_or_else(|| cut_err(input, "Missing required value"))?;

    Ok(OdmHeader {
        comment,
        classification,
        creation_date,
        originator,
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

    loop {
        // Collect comments before state vector started
        if epoch.is_none() {
            let checkpoint = input.checkpoint();
            let comments = collect_comments.parse_next(input)?;
            if !comments.is_empty() {
                let next_key = peek_key(input)?;
                if !matches!(
                    next_key,
                    Some("EPOCH" | "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT")
                ) {
                    // No state vector follows, backtrack comments
                    input.reset(&checkpoint);
                    break;
                }
                comment.extend(comments);
            }
        }

        let next_key = peek_key(input)?;

        match next_key {
            Some(_k @ ("EPOCH" | "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT")) => {
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "EPOCH" => {
                        epoch = Some(
                            Epoch::from_str(val).map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "X" => {
                        x = Some(
                            Position::from_kvn(val, unit.or(Some("km")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "Y" => {
                        y = Some(
                            Position::from_kvn(val, unit.or(Some("km")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "Z" => {
                        z = Some(
                            Position::from_kvn(val, unit.or(Some("km")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "X_DOT" => {
                        x_dot = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "Y_DOT" => {
                        y_dot = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "Z_DOT" => {
                        z_dot = Some(
                            Velocity::from_kvn(val, unit.or(Some("km/s")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    let sv = StateVector {
        comment: Vec::new(), // comments are returned separately for proper placement
        epoch: epoch.ok_or_else(|| cut_err(input, "Missing required value"))?,
        x: x.ok_or_else(|| cut_err(input, "Missing required value"))?,
        y: y.ok_or_else(|| cut_err(input, "Missing required value"))?,
        z: z.ok_or_else(|| cut_err(input, "Missing required value"))?,
        x_dot: x_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
        y_dot: y_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
        z_dot: z_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
    };

    Ok((comment, sv))
}

fn is_covariance_key(key: &str) -> bool {
    key.starts_with("CX_")
        || key.starts_with("CY_")
        || key.starts_with("CZ_")
        || key == "COV_REF_FRAME"
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

    // Check if we have covariance keys
    let next_key = peek_key(input)?;
    if !matches!(next_key, Some(k) if is_covariance_key(k)) {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;
        if !comments.is_empty() {
            let next_key = peek_key(input)?;
            if !matches!(next_key, Some(k) if is_covariance_key(k)) {
                input.reset(&checkpoint);
                return Ok(None);
            }
            comment.extend(comments);
        } else {
            return Ok(None);
        }
    }

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if is_covariance_key(k) => {
                comment.extend(comments);
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "COV_REF_FRAME" => cov_ref_frame = Some(val.to_string()),
                    "CX_X" => {
                        cx_x = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_X" => {
                        cy_x = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_Y" => {
                        cy_y = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_X" => {
                        cz_x = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_Y" => {
                        cz_y = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_Z" => {
                        cz_z = Some(
                            PositionCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CX_DOT_X" => {
                        cx_dot_x = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CX_DOT_Y" => {
                        cx_dot_y = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CX_DOT_Z" => {
                        cx_dot_z = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CX_DOT_X_DOT" => {
                        cx_dot_x_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_DOT_X" => {
                        cy_dot_x = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_DOT_Y" => {
                        cy_dot_y = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_DOT_Z" => {
                        cy_dot_z = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_DOT_X_DOT" => {
                        cy_dot_x_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CY_DOT_Y_DOT" => {
                        cy_dot_y_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_X" => {
                        cz_dot_x = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_Y" => {
                        cz_dot_y = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_Z" => {
                        cz_dot_z = Some(
                            PositionVelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_X_DOT" => {
                        cz_dot_x_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_Y_DOT" => {
                        cz_dot_y_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    "CZ_DOT_Z_DOT" => {
                        cz_dot_z_dot = Some(
                            VelocityCovariance::from_kvn(val, unit)
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        )
                    }
                    _ => {}
                }
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }
    }

    // If we have covariance data, build the struct
    if cx_x.is_some() {
        Ok(Some(OpmCovarianceMatrix {
            comment,
            cov_ref_frame,
            cx_x: cx_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_x: cy_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_y: cy_y.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_x: cz_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_y: cz_y.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_z: cz_z.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cx_dot_x: cx_dot_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cx_dot_y: cx_dot_y.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cx_dot_z: cx_dot_z.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cx_dot_x_dot: cx_dot_x_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_dot_x: cy_dot_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_dot_y: cy_dot_y.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_dot_z: cy_dot_z.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_dot_x_dot: cy_dot_x_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cy_dot_y_dot: cy_dot_y_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_x: cz_dot_x.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_y: cz_dot_y.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_z: cz_dot_z.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_x_dot: cz_dot_x_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_y_dot: cz_dot_y_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
            cz_dot_z_dot: cz_dot_z_dot.ok_or_else(|| cut_err(input, "Missing required value"))?,
        }))
    } else {
        Ok(None)
    }
}

fn is_spacecraft_key(key: &str) -> bool {
    matches!(
        key,
        "MASS" | "SOLAR_RAD_AREA" | "SOLAR_RAD_COEFF" | "DRAG_AREA" | "DRAG_COEFF"
    )
}

/// Parses the optional spacecraft parameters section.
pub fn spacecraft_parameters(input: &mut &str) -> KvnResult<Option<SpacecraftParameters>> {
    let mut comment = Vec::new();
    let mut mass = None;
    let mut solar_rad_area = None;
    let mut solar_rad_coeff = None;
    let mut drag_area = None;
    let mut drag_coeff = None;

    // Check if we have any spacecraft keys
    let next_key = peek_key(input)?;
    if !matches!(next_key, Some(k) if is_spacecraft_key(k)) {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;
        if !comments.is_empty() {
            let next_key = peek_key(input)?;
            if !matches!(next_key, Some(k) if is_spacecraft_key(k)) {
                input.reset(&checkpoint);
                return Ok(None);
            }
            comment.extend(comments);
        } else {
            return Ok(None);
        }
    }

    loop {
        let checkpoint = input.checkpoint();
        let comments = collect_comments.parse_next(input)?;

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if is_spacecraft_key(k) => {
                comment.extend(comments);
                let (key, val, unit) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                match key {
                    "MASS" => {
                        mass = Some(
                            Mass::from_kvn(val, unit.or(Some("kg")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "SOLAR_RAD_AREA" => {
                        solar_rad_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "SOLAR_RAD_COEFF" => {
                        solar_rad_coeff =
                            Some(parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    "DRAG_AREA" => {
                        drag_area = Some(
                            Area::from_kvn(val, unit.or(Some("m**2")))
                                .map_err(|_| cut_err(input, "Invalid value"))?,
                        );
                    }
                    "DRAG_COEFF" => {
                        drag_coeff =
                            Some(parse_f64(val).map_err(|_| cut_err(input, "Invalid value"))?);
                    }
                    _ => {}
                }
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
        }
    }

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

        let next_key = peek_key(input)?;

        match next_key {
            Some(k) if k.starts_with("USER_DEFINED_") => {
                comment.extend(comments);
                let (key, val, _) = key_value_line.parse_next(input)?;
                opt_line_ending.parse_next(input)?;

                params.push(UserDefinedParameter {
                    parameter: key.to_string(),
                    value: val.to_string(),
                });
            }
            _ => {
                input.reset(&checkpoint);
                break;
            }
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
}
