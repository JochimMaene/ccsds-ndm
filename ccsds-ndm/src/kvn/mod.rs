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
//! - **Parsing**: Uses the [`winnow`](https://docs.rs/winnow) parser combinator library for high performance.
//! - **Serialization**: Uses a custom `KvnWriter` to ensure correct formatting and indentation.

pub mod acm;
pub mod aem;
pub mod apm;
pub mod cdm;
pub mod ocm;
pub mod oem;
pub mod omm;
pub mod opm;
pub mod parser;
pub mod rdm;
pub mod ser;
pub mod tdm;
