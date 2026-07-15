// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::UnitValue;
use std::fmt::{Display, Write as FmtWrite};
use std::io::Write as IoWrite;

enum KvnOutput<'a> {
    String(String),
    Io(&'a mut dyn IoWrite),
}

/// A helper for writing Key-Value Notation (KVN) for CCSDS NDM messages.
pub struct KvnWriter<'a> {
    output: KvnOutput<'a>,
    io_error: Option<std::io::Error>,
    line_buffer: String,
}

impl FmtWrite for KvnWriter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match &mut self.output {
            KvnOutput::String(output) => output.write_str(s),
            KvnOutput::Io(output) => output.write_all(s.as_bytes()).map_err(|error| {
                if self.io_error.is_none() {
                    self.io_error = Some(error);
                }
                std::fmt::Error
            }),
        }
    }
}

impl KvnWriter<'static> {
    pub fn new() -> Self {
        Self {
            output: KvnOutput::String(String::new()),
            io_error: None,
            line_buffer: String::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            output: KvnOutput::String(String::with_capacity(capacity)),
            io_error: None,
            line_buffer: String::new(),
        }
    }
}

impl Default for KvnWriter<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> KvnWriter<'a> {
    /// Create a writer that writes directly to an I/O sink.
    pub(crate) fn from_io<W: IoWrite>(output: &'a mut W) -> Self {
        Self {
            output: KvnOutput::Io(output),
            io_error: None,
            line_buffer: String::new(),
        }
    }

    /// Builds and writes one raw line using a reusable growable buffer.
    pub(crate) fn write_built_line(&mut self, build: impl FnOnce(&mut String)) {
        let mut line = std::mem::take(&mut self.line_buffer);
        line.clear();
        build(&mut line);
        line.push('\n');
        let _ = self.write_str(&line);
        self.line_buffer = line;
    }

    fn normalize_inline_value(value: &str) -> std::borrow::Cow<'_, str> {
        if value.contains('\n') || value.contains('\r') || value.contains('\t') {
            return std::borrow::Cow::Owned(value.split_whitespace().collect::<Vec<_>>().join(" "));
        }
        std::borrow::Cow::Borrowed(value)
    }

    /// Writes a simple `KEY = value` line.
    pub fn write_pair<V: Display>(&mut self, key: &str, value: V) {
        let raw = value.to_string();
        let normalized = Self::normalize_inline_value(&raw);
        let _ = writeln!(self, "{:<20} = {}", key, normalized);
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
            for line in c.lines() {
                let normalized = Self::normalize_inline_value(line);
                let _ = writeln!(self, "COMMENT {}", normalized);
            }
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
        match self.output {
            KvnOutput::String(output) => output,
            KvnOutput::Io(_) => unreachable!("finish is only valid for string-backed writers"),
        }
    }

    /// Finish writing to an I/O sink and return any deferred write error.
    pub(crate) fn finish_io(self) -> std::io::Result<()> {
        match self.io_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KvnWriter;

    #[test]
    fn write_comments_splits_multiline_entries() {
        let mut writer = KvnWriter::new();
        writer.write_comments(&[String::from("line1\nline2"), String::from("line3")]);
        let out = writer.finish();

        assert_eq!(out, "COMMENT line1\nCOMMENT line2\nCOMMENT line3\n");
    }

    #[test]
    fn write_pair_normalizes_multiline_value() {
        let mut writer = KvnWriter::new();
        writer.write_pair(
            "ORIGINATOR_POSITION",
            "Flight Dynamics Mission Design\nLead",
        );
        let out = writer.finish();

        assert_eq!(
            out,
            "ORIGINATOR_POSITION  = Flight Dynamics Mission Design Lead\n"
        );
    }
}
