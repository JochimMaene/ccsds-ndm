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
