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

pub(crate) mod acm;
pub(crate) mod aem;
pub(crate) mod apm;
pub(crate) mod cdm;
pub(crate) mod ocm;
pub(crate) mod oem;
pub(crate) mod omm;
pub(crate) mod opm;
pub(crate) mod parser;
pub(crate) mod rdm;
pub mod ser;
pub(crate) mod strict;
pub(crate) mod tdm;

/// Normalize CR, LFCR, and CRLF terminators so line-oriented passes only have to handle LF.
///
/// Every rewrite is byte-length preserving (`\r` becomes `\n`, `\n\r` becomes `\r\n`). That
/// invariant is load-bearing: parse diagnostics carry byte offsets into the normalized text, but
/// are rendered against the caller's original input, so the two must stay aligned.
///
/// Only OEM enables this today; the other families still reject a lone carriage return in
/// [`strict::validate_odm_assignments`]. Widening it is a per-family conformance decision.
pub(crate) fn normalize_line_endings(input: &str) -> std::borrow::Cow<'_, str> {
    if !input.contains('\r') {
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
