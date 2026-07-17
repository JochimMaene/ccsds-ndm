// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::UnitValue;
use std::fmt::{Display, Write as FmtWrite};
use std::io::Write as IoWrite;

/// An exact, ODM-compatible spelling of a finite `f64`.
///
/// The representability check must run before this adapter is written. Formatting starts from
/// zmij's allocation-free shortest round-trip representation and only changes its spelling.
pub(crate) struct OdmFloat(f64);

impl OdmFloat {
    pub(crate) fn is_representable(value: f64) -> bool {
        if !value.is_finite() {
            return false;
        }
        let mut buffer = zmij::Buffer::new();
        Self::significant_digits(buffer.format_finite(value)) <= 16
    }

    fn significant_digits(value: &str) -> usize {
        let mut first = None;
        let mut last = None;
        let mut digit_index = 0;

        for byte in value
            .bytes()
            .take_while(|byte| *byte != b'e' && *byte != b'E')
        {
            if byte.is_ascii_digit() {
                if byte != b'0' {
                    first.get_or_insert(digit_index);
                    last = Some(digit_index);
                }
                digit_index += 1;
            }
        }

        match (first, last) {
            (Some(first), Some(last)) => last - first + 1,
            _ => 1,
        }
    }

    fn write_fixed_as_scientific<W: FmtWrite>(value: &str, output: &mut W) -> std::fmt::Result {
        let unsigned = value.strip_prefix('-').unwrap_or(value);
        if value.starts_with('-') {
            output.write_char('-')?;
        }

        let digits_before_decimal = unsigned.find('.').unwrap_or(unsigned.len());
        let mut digit_index = 0;
        let mut first = None;
        let mut last = None;
        for byte in unsigned.bytes() {
            if byte.is_ascii_digit() {
                if byte != b'0' {
                    first.get_or_insert(digit_index);
                    last = Some(digit_index);
                }
                digit_index += 1;
            }
        }

        let (first, last) = match (first, last) {
            (Some(first), Some(last)) => (first, last),
            _ => return output.write_str("0.0"),
        };
        let exponent = digits_before_decimal as isize - first as isize - 1;
        let mut wrote_fraction = false;
        for (current, byte) in unsigned.bytes().filter(u8::is_ascii_digit).enumerate() {
            if current == first {
                output.write_char(byte as char)?;
                output.write_char('.')?;
            } else if current > first && current <= last {
                output.write_char(byte as char)?;
                wrote_fraction = true;
            }
        }
        if !wrote_fraction {
            output.write_char('0')?;
        }
        write!(output, "e{exponent}")
    }

    fn write_to<W: FmtWrite>(&self, output: &mut W) -> std::fmt::Result {
        let mut buffer = zmij::Buffer::new();
        let value = buffer.format_finite(self.0);

        if let Some(exponent) = value.find(['e', 'E']) {
            let mantissa = &value[..exponent];
            output.write_str(mantissa)?;
            if !mantissa.contains('.') {
                output.write_str(".0")?;
            }
            output.write_str(&value[exponent..])
        } else if value.bytes().filter(u8::is_ascii_digit).count() <= 16 {
            output.write_str(value)
        } else {
            Self::write_fixed_as_scientific(value, output)
        }
    }
}

impl Display for OdmFloat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_to(formatter)
    }
}

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
    fn build_odm_float_pair(line: &mut String, key: &str, value: f64) {
        line.push_str(key);
        line.extend(std::iter::repeat_n(' ', 20usize.saturating_sub(key.len())));
        line.push_str(" = ");
        let _ = OdmFloat(value).write_to(line);
    }

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

    /// Writes an exactly representable ODM number without allocating a value string.
    pub(crate) fn write_odm_float_pair(&mut self, key: &str, value: f64) {
        if let KvnOutput::String(output) = &mut self.output {
            Self::build_odm_float_pair(output, key, value);
            output.push('\n');
            return;
        }
        self.write_built_line(|line| Self::build_odm_float_pair(line, key, value));
    }

    /// Writes an exactly representable ODM number and optional unit.
    pub(crate) fn write_odm_float_measure<U: Display>(
        &mut self,
        key: &str,
        measure: &UnitValue<f64, U>,
    ) {
        let build = |line: &mut String| {
            Self::build_odm_float_pair(line, key, measure.value);
            if let Some(unit) = &measure.units {
                line.push_str(" [");
                let _ = write!(line, "{unit}");
                line.push(']');
            }
        };
        if let KvnOutput::String(output) = &mut self.output {
            build(output);
            output.push('\n');
            return;
        }
        self.write_built_line(build);
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
    use super::{KvnWriter, OdmFloat};

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

    #[test]
    fn odm_float_preserves_exact_values_with_compliant_spelling() {
        for value in [-0.0, 0.0, 1.0, -12.5, 1.0e15, 1.25e-20, f64::from_bits(1)] {
            assert!(OdmFloat::is_representable(value));
            let spelling = OdmFloat(value).to_string();
            assert!(spelling.contains('.'));
            assert_eq!(spelling.parse::<f64>().unwrap().to_bits(), value.to_bits());
            assert!(OdmFloat::significant_digits(&spelling) <= 16);
        }
    }

    #[test]
    fn odm_float_rejects_lossy_and_non_finite_values() {
        assert!(OdmFloat::is_representable(1.234_567_890_123_456));
        assert!(!OdmFloat::is_representable(1.234_567_890_123_456_7));
        assert!(!OdmFloat::is_representable(f64::MIN_POSITIVE));
        assert!(!OdmFloat::is_representable(f64::MAX));
        assert!(!OdmFloat::is_representable(f64::NAN));
        assert!(!OdmFloat::is_representable(f64::INFINITY));
    }

    #[test]
    fn odm_float_normalizes_scientific_and_long_fixed_boundaries() {
        assert_eq!(OdmFloat(f64::from_bits(1)).to_string(), "5.0e-324");
        assert_eq!(OdmFloat(-f64::from_bits(1)).to_string(), "-5.0e-324");
        assert_eq!(OdmFloat(1.0e15).to_string(), "1.0e15");
        assert_eq!(OdmFloat(9_061_488_000_000_000.0).to_string(), "9.061488e15");
        assert_eq!(
            OdmFloat(1.234_567_890_123_456).to_string(),
            "1.234567890123456"
        );
    }
}
