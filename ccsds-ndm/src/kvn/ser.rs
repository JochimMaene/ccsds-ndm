// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::UnitValue;
use std::fmt::{Display, Write};

/// A helper for writing Key-Value Notation (KVN) for CCSDS NDM messages.
pub struct KvnWriter {
    output: String,
}

impl KvnWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl Default for KvnWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl KvnWriter {
    /// Writes a simple `KEY = value` line.
    pub fn write_pair<V: Display>(&mut self, key: &str, value: V) {
        let _ = writeln!(self.output, "{:<20} = {}", key, value);
    }

    /// Writes `KEY = value [unit]`.
    /// Falls back to `write_pair` if no unit is provided.
    pub fn write_measure<V: Display, U: Display>(&mut self, key: &str, measure: &UnitValue<V, U>) {
        if let Some(ref u) = measure.units {
            let _ = writeln!(self.output, "{:<20} = {} [{}]", key, measure.value, u);
        } else {
            self.write_pair(key, &measure.value);
        }
    }

    /// Writes a raw line of text.
    pub fn write_line<V: Display>(&mut self, line: V) {
        let _ = writeln!(self.output, "{}", line);
    }

    /// Writes comment lines.
    pub fn write_comments(&mut self, comments: &[String]) {
        for c in comments {
            let _ = writeln!(self.output, "COMMENT {}", c);
        }
    }

    /// Writes a section tag (e.g., "META_START").
    pub fn write_section(&mut self, tag: &str) {
        let _ = writeln!(self.output, "{}", tag);
    }

    /// Inserts a blank line.
    pub fn write_empty(&mut self) {
        let _ = writeln!(self.output);
    }

    /// Returns the accumulated KVN content.
    pub fn finish(self) -> String {
        self.output
    }
}
