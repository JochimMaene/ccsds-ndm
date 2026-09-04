// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing options.

/// Resource policy for parsing a complete NDM document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptions {
    /// Maximum input document size in bytes. `None` keeps the aggregate size unlimited.
    pub max_input_bytes: Option<usize>,
    /// Maximum XML element nesting depth.
    ///
    /// Current OPM/OEM schemas have small fixed depths; the default leaves generous headroom
    /// while bounding adversarial nesting.
    pub max_xml_depth: usize,
    /// Maximum number of materialized history records.
    ///
    /// `None` keeps record count unlimited. Record-bearing families count repeatable history
    /// entries during notation preflight, before typed records are allocated.
    pub max_records: Option<usize>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            max_input_bytes: None,
            max_xml_depth: 16,
            max_records: None,
        }
    }
}

impl ParseOptions {
    pub fn with_max_input_bytes(mut self, max_input_bytes: usize) -> Self {
        self.max_input_bytes = Some(max_input_bytes);
        self
    }

    pub fn with_max_xml_depth(mut self, max_xml_depth: usize) -> Self {
        self.max_xml_depth = max_xml_depth;
        self
    }

    pub fn with_max_records(mut self, max_records: usize) -> Self {
        self.max_records = Some(max_records);
        self
    }
}
