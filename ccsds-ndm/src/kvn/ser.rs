// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::UnitValue;
use std::fmt::{Display, Write};

/// A helper for writing Key-Value Notation (KVN) for CCSDS NDM messages.
pub struct KvnWriter {
    output: String,
}

impl Write for KvnWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.write_str(s)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.output.write_fmt(args)
    }
}

impl KvnWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            output: String::with_capacity(capacity),
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
        let _ = writeln!(self, "{:<20} = {}", key, value);
    }

    /// Writes `KEY = value [unit]`.
    /// Falls back to `write_pair` if no unit is provided.
    pub fn write_measure<V: Display, U: Display>(&mut self, key: &str, measure: &UnitValue<V, U>) {
        if let Some(ref u) = measure.units {
            let _ = writeln!(self, "{:<20} = {} [{}]", key, measure.value, u);
        } else {
            self.write_pair(key, &measure.value);
        }
    }

    /// Writes a raw line of text.
    pub fn write_line<V: Display>(&mut self, line: V) {
        let _ = writeln!(self, "{}", line);
    }

    /// Writes comment lines.
    pub fn write_comments(&mut self, comments: &[String]) {
        for c in comments {
            let _ = writeln!(self, "COMMENT {}", c);
        }
    }

    /// Writes a section tag (e.g., "META_START").
    pub fn write_section(&mut self, tag: &str) {
        let _ = writeln!(self, "{}", tag);
    }

    /// Inserts a blank line.
    pub fn write_empty(&mut self) {
        let _ = writeln!(self);
    }

    /// Writes a user-defined parameter, ensuring the "USER_DEFINED_" prefix is present.
    pub fn write_user_defined(&mut self, parameter: &str, value: &str) {
        let key = if parameter.starts_with("USER_DEFINED_") {
            std::borrow::Cow::Borrowed(parameter)
        } else {
            std::borrow::Cow::Owned(format!("USER_DEFINED_{}", parameter))
        };
        self.write_pair(&key, value);
    }

    /// Returns the accumulated KVN content.
    pub fn finish(self) -> String {
        self.output
    }
}
