// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Key-Value Notation (KVN) support.
//!
//! This module handles parsing and generation of CCSDS messages in the KVN format.
//! KVN is a line-oriented, human-readable format consisting of `KEY = VALUE` pairs.
//!
//! # Format Specifics
//!
//! - **Units**: Physical quantities often include units in square brackets (e.g., `[km]`, `[deg]`).
//!   This parser validates that the units in the file match the expected units for the field.
//! - **Comments**: Comments start with `COMMENT` and can appear in Metadata and Data sections.
//! - **Case Sensitivity**: CCSDS keywords are generally uppercase (e.g., `OBJECT_NAME`).
//!
//! # Implementation Details
//!
//! - **Parsing**: Uses the [`winnow`](https://docs.rs/winnow) parser combinator library for high
//!   performance. Raw parsers are crate-internal; public callers parse complete messages through
//!   [`Ndm::from_kvn`](crate::traits::Ndm::from_kvn) or the crate-level auto-detection helpers.
//! - **Serialization**: Uses a custom `KvnWriter` to ensure correct formatting and indentation.

pub(crate) mod parser;
pub mod ser;
pub(crate) mod strict;

/// Shared front end of every family's KVN reader.
///
/// Expects the caller to have stripped one UTF-8 byte-order mark. Normalizes terminators and TABs, rejects non-printable or
/// non-ASCII content by line, byte column, and code point, and checks the `CCSDS_xxx_VERS`
/// edition before any structural parse, so an unsupported edition reports
/// `parse.unsupported_input_version` rather than a keyword that edition lacks. Line length is not
/// checked on read: producers commonly exceed the 254-character limit losslessly, and writers
/// still enforce it.
pub(crate) fn prepare_input(
    input: &str,
    kind: crate::validation::MessageKind,
) -> crate::error::Result<std::borrow::Cow<'_, str>> {
    let normalized = normalize_line_endings(input);
    if let Some(offset) = normalized
        .bytes()
        .position(|byte| !matches!(byte, b' '..=b'~' | b'\n' | b'\r'))
    {
        let character = normalized[offset..].chars().next().unwrap_or_default();
        // Every earlier byte on the line is ASCII, so the character column is the byte column.
        let (line, column) = crate::error::line_column(&normalized, offset);
        return Err(crate::error::CcsdsNdmError::Format(Box::new(
            crate::error::FormatError::Kvn(Box::new(crate::error::KvnParseError {
                line,
                column,
                message: format!(
                    "non-printable or non-ASCII character U+{:04X}",
                    u32::from(character)
                ),
                contexts: Vec::new(),
                offset,
            })),
        )));
    }
    if let Some(spec) = crate::versioning::spec(kind) {
        let version = normalized
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty() && !line.starts_with("COMMENT"))
            .and_then(|line| line.strip_prefix(spec.id_keyword))
            .and_then(|rest| rest.trim_start().strip_prefix('='))
            .map(str::trim);
        // A malformed version record is left to the structural parser's diagnostics.
        if let Some(version) =
            version.filter(|value| !value.is_empty() && !value.contains([' ', '[']))
        {
            crate::versioning::check_input_version(kind, version)?;
        }
    }
    Ok(normalized)
}

/// Normalize CR, LFCR, and CRLF terminators so line-oriented passes only have to handle LF, and
/// read TAB as the blank it stands for (writers never emit TAB).
///
/// Every rewrite is byte-length preserving (`\r` becomes `\n`, `\n\r` becomes `\r\n`, TAB becomes
/// a blank). That invariant is load-bearing: parse diagnostics carry byte offsets into the
/// normalized text, but are rendered against the caller's original input, so the two must stay
/// aligned.
pub(crate) fn normalize_line_endings(input: &str) -> std::borrow::Cow<'_, str> {
    if !input.contains(['\r', '\t']) {
        return std::borrow::Cow::Borrowed(input);
    }

    let bytes = input.as_bytes();
    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                normalized.extend_from_slice(b"\r\n");
                index += 2;
            }
            b'\n' if bytes.get(index + 1) == Some(&b'\r') => {
                normalized.extend_from_slice(b"\r\n");
                index += 2;
            }
            b'\r' => {
                normalized.push(b'\n');
                index += 1;
            }
            b'\t' => {
                normalized.push(b' ');
                index += 1;
            }
            byte => {
                normalized.push(byte);
                index += 1;
            }
        }
    }
    debug_assert_eq!(
        normalized.len(),
        bytes.len(),
        "normalization must preserve byte offsets for diagnostics"
    );
    std::borrow::Cow::Owned(String::from_utf8(normalized).expect("line endings preserve UTF-8"))
}
