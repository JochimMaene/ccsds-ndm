// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::UnitValue;
use std::fmt::{Display, Write as FmtWrite};
use std::io::Write as IoWrite;

/// A CCSDS-compatible spelling of a finite `f64`.
///
/// CCSDS permits at most 16 decimal digits in fixed-point values and floating-point mantissas.
/// Values whose shortest round-trip spelling needs 17 digits are rounded to 16 digits.
pub(crate) struct OdmFloat(f64);

impl OdmFloat {
    pub(crate) const fn new(value: f64) -> Self {
        Self(value)
    }

    pub(crate) const fn is_valid(value: f64) -> bool {
        value.is_finite()
    }

    /// Return the emitted length of a finite CCSDS number.
    pub(crate) fn formatted_len(value: f64) -> Option<usize> {
        if !value.is_finite() {
            return None;
        }

        struct Counter(usize);
        impl FmtWrite for Counter {
            fn write_str(&mut self, value: &str) -> std::fmt::Result {
                self.0 += value.len();
                Ok(())
            }
        }

        let mut counter = Counter(0);
        let _ = Self(value).write_to(&mut counter);
        Some(counter.0)
    }

    /// Write `value` using a CCSDS-compatible spelling.
    pub(crate) fn write_if_valid<W: FmtWrite>(value: f64, output: &mut W) -> bool {
        if !value.is_finite() {
            return false;
        }
        Self(value).write_to(output).is_ok()
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

    fn write_preformatted<W: FmtWrite>(value: &str, output: &mut W) -> std::fmt::Result {
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

    fn write_rounded_scientific<W: FmtWrite>(value: &str, output: &mut W) -> std::fmt::Result {
        let (mantissa, exponent) = value.find(['e', 'E']).map_or((value, 0i32), |index| {
            let exponent = &value.as_bytes()[index + 1..];
            let (negative, digits) = match exponent.first() {
                Some(b'-') => (true, &exponent[1..]),
                Some(b'+') => (false, &exponent[1..]),
                _ => (false, exponent),
            };
            let magnitude = digits
                .iter()
                .fold(0i32, |value, digit| value * 10 + i32::from(*digit - b'0'));
            (
                &value[..index],
                if negative { -magnitude } else { magnitude },
            )
        });
        let negative = mantissa.starts_with('-');
        let unsigned = mantissa.strip_prefix('-').unwrap_or(mantissa);
        let digits_before_decimal = unsigned.find('.').unwrap_or(unsigned.len()) as i32;

        let mut digits = [0u8; 17];
        let mut first = None;
        let mut significant = 0usize;
        for (digit_index, byte) in unsigned.bytes().filter(u8::is_ascii_digit).enumerate() {
            if first.is_none() && byte != b'0' {
                first = Some(digit_index as i32);
            }
            if first.is_some() && significant < digits.len() {
                digits[significant] = byte - b'0';
                significant += 1;
            }
        }

        let mut normalized_exponent =
            exponent + digits_before_decimal - first.expect("non-zero finite value") - 1;
        if digits[16] >= 5 {
            let mut index = 16;
            loop {
                index -= 1;
                if digits[index] < 9 {
                    digits[index] += 1;
                    break;
                }
                digits[index] = 0;
                if index == 0 {
                    digits[0] = 1;
                    normalized_exponent += 1;
                    break;
                }
            }
        }

        let last_fraction = (1..16).rev().find(|index| digits[*index] != 0).unwrap_or(1);

        let mut formatted = [0u8; 32];
        let mut length = 0usize;
        if negative {
            formatted[length] = b'-';
            length += 1;
        }
        formatted[length] = b'0' + digits[0];
        formatted[length + 1] = b'.';
        length += 2;
        for digit in &digits[1..=last_fraction] {
            formatted[length] = b'0' + *digit;
            length += 1;
        }
        formatted[length] = b'e';
        length += 1;

        let exponent_negative = normalized_exponent < 0;
        let mut magnitude = normalized_exponent.unsigned_abs();
        if exponent_negative {
            formatted[length] = b'-';
            length += 1;
        }
        let exponent_start = length;
        loop {
            formatted[length] = b'0' + (magnitude % 10) as u8;
            length += 1;
            magnitude /= 10;
            if magnitude == 0 {
                break;
            }
        }
        formatted[exponent_start..length].reverse();

        output.write_str(
            std::str::from_utf8(&formatted[..length])
                .expect("CCSDS numeric formatter produced ASCII"),
        )
    }

    fn write_to<W: FmtWrite>(&self, output: &mut W) -> std::fmt::Result {
        if !self.0.is_finite() {
            return Err(std::fmt::Error);
        }
        let mut buffer = zmij::Buffer::new();
        let shortest = buffer.format_finite(self.0);
        if Self::significant_digits(shortest) <= 16 {
            Self::write_preformatted(shortest, output)
        } else {
            Self::write_rounded_scientific(shortest, output)
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

    fn build_tdm_observation<E: Display>(line: &mut String, key: &str, epoch: &E, value: f64) {
        line.push_str(key);
        line.extend(std::iter::repeat_n(' ', 20usize.saturating_sub(key.len())));
        let _ = write!(line, " = {epoch} {value}");
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

    /// Builds one line and commits it only when the fallible builder succeeds.
    pub(crate) fn try_write_built_line<E>(
        &mut self,
        build: impl FnOnce(&mut String) -> std::result::Result<(), E>,
    ) -> std::result::Result<(), E> {
        let mut line = std::mem::take(&mut self.line_buffer);
        line.clear();
        let result = build(&mut line);
        if result.is_ok() {
            line.push('\n');
            let _ = self.write_str(&line);
        }
        self.line_buffer = line;
        result
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

    /// Writes a CCSDS-compatible ODM number without allocating a value string.
    pub(crate) fn write_odm_float_pair(&mut self, key: &str, value: f64) {
        if let KvnOutput::String(output) = &mut self.output {
            Self::build_odm_float_pair(output, key, value);
            output.push('\n');
            return;
        }
        self.write_built_line(|line| Self::build_odm_float_pair(line, key, value));
    }

    /// Writes a CCSDS-compatible ODM number and optional unit.
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

    /// Writes one TDM observation without allocating intermediate epoch/value strings.
    pub(crate) fn write_tdm_observation<E: Display>(&mut self, key: &str, epoch: &E, value: f64) {
        if let KvnOutput::String(output) = &mut self.output {
            Self::build_tdm_observation(output, key, epoch, value);
            output.push('\n');
            return;
        }
        self.write_built_line(|line| Self::build_tdm_observation(line, key, epoch, value));
    }

    /// Writes an OCM numeric history record without per-value or per-record temporary strings.
    pub(crate) fn write_ocm_numeric_history<E: Display>(&mut self, epoch: &E, values: &[f64]) {
        let build = |line: &mut String| {
            let _ = write!(line, "{epoch}");
            for value in values {
                line.push(' ');
                let _ = OdmFloat(*value).write_to(line);
            }
        };
        if let KvnOutput::String(output) = &mut self.output {
            build(output);
            output.push('\n');
            return;
        }
        self.write_built_line(build);
    }

    /// Write one AEM attitude-state record without allocating intermediate strings.
    pub(crate) fn write_aem_attitude_state<E: Display>(&mut self, epoch: &E, values: &[f64]) {
        let build = |line: &mut String| {
            let _ = write!(line, "{epoch}");
            for value in values {
                line.push(' ');
                let _ = OdmFloat(*value).write_to(line);
            }
        };
        if let KvnOutput::String(output) = &mut self.output {
            build(output);
            output.push('\n');
            return;
        }
        self.write_built_line(build);
    }

    /// Write a numeric history record without allocating intermediate value strings.
    pub(crate) fn write_numeric_record(&mut self, values: &[f64]) {
        let build = |line: &mut String| {
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    line.push(' ');
                }
                let _ = OdmFloat(*value).write_to(line);
            }
        };
        if let KvnOutput::String(output) = &mut self.output {
            build(output);
            output.push('\n');
            return;
        }
        self.write_built_line(build);
    }

    /// Write a numeric vector assignment and optional unit without temporary joins.
    pub(crate) fn write_numeric_vector<U: Display>(
        &mut self,
        key: &str,
        values: &[f64],
        unit: Option<&U>,
    ) {
        let build = |line: &mut String| {
            let _ = write!(line, "{key:<20} = ");
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    line.push(' ');
                }
                let _ = OdmFloat(*value).write_to(line);
            }
            if let Some(unit) = unit {
                let _ = write!(line, " [{unit}]");
            }
        };
        if let KvnOutput::String(output) = &mut self.output {
            build(output);
            output.push('\n');
            return;
        }
        self.write_built_line(build);
    }

    /// Writes an OCM maneuver history record without joining or formatting temporary strings.
    pub(crate) fn write_ocm_text_history<E: Display>(&mut self, epoch: &E, values: &[String]) {
        let build = |line: &mut String| {
            let _ = write!(line, "{epoch}");
            for value in values {
                line.push(' ');
                line.push_str(value);
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
            if c.is_empty() {
                let _ = writeln!(self, "COMMENT");
                continue;
            }
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
    fn write_comments_preserves_empty_comment() {
        let mut writer = KvnWriter::new();
        writer.write_comments(&[String::new()]);
        assert_eq!(writer.finish(), "COMMENT\n");
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
    fn odm_float_preserves_exact_values_when_they_fit() {
        for value in [-0.0, 0.0, 1.0, -12.5, 1.0e15, 1.25e-20, f64::from_bits(1)] {
            assert!(OdmFloat::is_valid(value));
            let spelling = OdmFloat(value).to_string();
            assert!(spelling.contains('.'));
            assert!(OdmFloat::significant_digits(&spelling) <= 16);
        }
    }

    #[test]
    fn odm_float_accepts_all_finite_values_and_rejects_non_finite_values() {
        assert!(OdmFloat::is_valid(1.234_567_890_123_456));
        assert!(OdmFloat::is_valid(1.234_567_890_123_456_7));
        assert!(OdmFloat::is_valid(f64::MIN_POSITIVE));
        assert!(OdmFloat::is_valid(f64::MAX));
        assert!(!OdmFloat::is_valid(f64::NAN));
        assert!(!OdmFloat::is_valid(f64::INFINITY));
    }

    #[test]
    fn odm_float_combines_validity_and_emitted_length() {
        for value in [
            -0.0,
            1.0,
            -12.5,
            1.0e15,
            1.25e-20,
            f64::from_bits(1),
            1.234_567_890_123_456_7,
            f64::MAX,
        ] {
            let mut output = String::new();
            assert!(OdmFloat::write_if_valid(value, &mut output));
            assert_eq!(OdmFloat::formatted_len(value), Some(output.len()));
            assert_eq!(output, OdmFloat(value).to_string());
            assert!(OdmFloat::significant_digits(&output) <= 16);
        }
        for value in [f64::NAN, f64::INFINITY] {
            let mut output = String::new();
            assert!(!OdmFloat::write_if_valid(value, &mut output));
            assert!(output.is_empty());
            assert_eq!(OdmFloat::formatted_len(value), None);
        }
    }

    #[test]
    fn fallible_line_builder_commits_only_successful_lines() {
        let mut writer = KvnWriter::new();
        writer
            .try_write_built_line(|line| -> Result<(), ()> {
                line.push_str("accepted");
                Ok(())
            })
            .unwrap();
        assert_eq!(
            writer.try_write_built_line(|line| -> Result<(), ()> {
                line.push_str("rejected");
                Err(())
            }),
            Err(())
        );
        assert_eq!(writer.finish(), "accepted\n");
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
        assert_eq!(
            OdmFloat(1.234_567_890_123_456_7).to_string(),
            "1.234567890123457e0"
        );
        assert_eq!(OdmFloat(f64::MAX).to_string(), "1.797693134862316e308");
    }
}
