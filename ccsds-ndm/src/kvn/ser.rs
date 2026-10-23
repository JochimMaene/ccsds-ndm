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

/// Largest magnitude whose CCSDS spelling still reads back as a finite `f64`.
///
/// Rounding to 16 digits can round *up*, and the two largest `f64` magnitudes round up to
/// `1.797693134862316e308`, which is greater than `f64::MAX` and therefore reparses as infinity.
/// Emitting them would produce a document this library rejects, so they are not representable.
const MAX_MAGNITUDE: f64 = f64::from_bits(f64::MAX.to_bits() - 2);

impl OdmFloat {
    /// Report whether `value` has a CCSDS spelling that reads back as a finite number.
    pub(crate) const fn is_valid(value: f64) -> bool {
        value.is_finite() && value.abs() <= MAX_MAGNITUDE
    }

    /// Write `value` using a CCSDS-compatible spelling.
    pub(crate) fn write_if_valid<W: FmtWrite>(value: f64, output: &mut W) -> bool {
        if !Self::is_valid(value) {
            return false;
        }
        Self::write_finite(value, output).is_ok()
    }

    /// Return the value read from the canonical spelling using the formatter's rounding rule.
    pub(crate) fn rounded_value(value: f64) -> Option<f64> {
        if !Self::is_valid(value) {
            return None;
        }
        if value == 0.0 {
            return Some(0.0);
        }
        let mut buffer = zmij::Buffer::new();
        let shortest = buffer.format_finite(value);
        Some(
            if shortest.len() <= 16 || Self::significant_digits(shortest) <= 16 {
                value
            } else {
                Self::rounded_to_16_digits(shortest)
            },
        )
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

    /// Round a 17-significant-digit spelling to 16 digits and return the double it denotes.
    fn rounded_to_16_digits(value: &str) -> f64 {
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

        std::str::from_utf8(&formatted[..length])
            .expect("CCSDS numeric formatter produced ASCII")
            .parse()
            .expect("rounded CCSDS spelling is a finite number")
    }

    fn write_finite<W: FmtWrite>(value: f64, output: &mut W) -> std::fmt::Result {
        // Books exclude negative zero; readers retain it, while KVN uses the same zero value.
        if value == 0.0 {
            return output.write_str("0.0");
        }
        let mut buffer = zmij::Buffer::new();
        let shortest = buffer.format_finite(value);
        // The overwhelmingly common case already satisfies the ODM lexical constraints. Avoid
        // rescanning short fixed-point spellings (and scientific spellings that already contain
        // the required decimal point) before copying them to the output.
        if shortest.len() <= 16 {
            let exponent = shortest
                .as_bytes()
                .iter()
                .position(|byte| matches!(byte, b'e' | b'E'));
            if exponent.is_none_or(|index| shortest.as_bytes()[..index].contains(&b'.')) {
                return output.write_str(shortest);
            }
        }
        if Self::significant_digits(shortest) <= 16 {
            Self::write_preformatted(shortest, output)
        } else {
            // Write the rounded double's own shortest spelling (at most 16 digits), so writing
            // what was read back yields the same text.
            Self::write_finite(Self::rounded_to_16_digits(shortest), output)
        }
    }

    fn write_to<W: FmtWrite>(&self, output: &mut W) -> std::fmt::Result {
        if !Self::is_valid(self.0) {
            return Err(std::fmt::Error);
        }
        Self::write_finite(self.0, output)
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
    /// First value-level refusal: non-ASCII text, an empty value, or an out-of-range integer.
    value_error: Option<crate::error::ValidationError>,
    unrepresentable_number: bool,
    line_length_error: Option<usize>,
    line_len: usize,
    allow_long_records: bool,
    always_units: bool,
    exact_floats: bool,
    line_buffer: String,
}

impl FmtWrite for KvnWriter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.finish_line();
            } else {
                if !(b' '..=b'~').contains(&byte) {
                    self.reject_text("KVN record");
                }
                self.line_len = self.line_len.saturating_add(1);
            }
        }
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
            value_error: None,
            unrepresentable_number: false,
            line_length_error: None,
            line_len: 0,
            allow_long_records: false,
            always_units: false,
            exact_floats: false,
            line_buffer: String::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            output: KvnOutput::String(String::with_capacity(capacity)),
            io_error: None,
            value_error: None,
            unrepresentable_number: false,
            line_length_error: None,
            line_len: 0,
            allow_long_records: false,
            always_units: false,
            exact_floats: false,
            line_buffer: String::new(),
        }
    }

    /// Preserve numeric identity while checking whether nonnumeric model content survives KVN.
    pub(crate) fn for_model_comparison() -> Self {
        Self {
            exact_floats: true,
            ..Self::new()
        }
    }
}

impl Default for KvnWriter<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> KvnWriter<'a> {
    fn build_odm_float_pair(line: &mut String, key: &str, value: f64, exact: bool) {
        line.push_str(key);
        line.extend(std::iter::repeat_n(' ', 20usize.saturating_sub(key.len())));
        line.push_str(" = ");
        if exact && value.is_finite() {
            line.push_str(zmij::Buffer::new().format_finite(value));
        } else {
            let _ = OdmFloat(value).write_to(line);
        }
    }

    fn build_tdm_observation<E: Display, V: Display>(
        line: &mut String,
        key: &str,
        epoch: &E,
        value: V,
    ) {
        line.push_str(key);
        line.extend(std::iter::repeat_n(' ', 20usize.saturating_sub(key.len())));
        let _ = write!(line, " = {epoch} {value}");
    }

    /// Create a writer that writes directly to an I/O sink.
    pub(crate) fn from_io<W: IoWrite>(output: &'a mut W) -> Self {
        Self {
            output: KvnOutput::Io(output),
            io_error: None,
            value_error: None,
            unrepresentable_number: false,
            line_length_error: None,
            line_len: 0,
            allow_long_records: false,
            always_units: false,
            exact_floats: false,
            line_buffer: String::new(),
        }
    }

    fn finish_line(&mut self) {
        if self.line_len > 254 && !self.allow_long_records {
            self.line_length_error.get_or_insert(self.line_len);
        }
        self.line_len = 0;
    }

    /// Allow arbitrary line lengths for ACM and OCM, as required by their CCSDS books.
    pub(crate) fn allow_long_records(&mut self) {
        self.allow_long_records = true;
    }

    /// Write the fixed unit of every measure even when the model holds none (RDM 5.2.4.1:
    /// applicable units "must be displayed").
    pub(crate) fn always_write_units(&mut self) {
        self.always_units = true;
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

    /// Builds one validated line and commits it only when the fallible builder succeeds.
    /// `line_start` is the offset at which the current line begins.
    pub(crate) fn try_write_validated_line<E>(
        &mut self,
        build: impl FnOnce(&mut String, usize) -> std::result::Result<(), E>,
    ) -> std::result::Result<(), E> {
        if let KvnOutput::String(output) = &mut self.output {
            let line_start = output.len();
            let result = build(output, line_start);
            if result.is_ok() {
                let line_len = output.len() - line_start;
                if line_len > 254 && !self.allow_long_records {
                    self.line_length_error.get_or_insert(line_len);
                }
                output.push('\n');
            } else {
                output.truncate(line_start);
            }
            return result;
        }

        let mut line = std::mem::take(&mut self.line_buffer);
        line.clear();
        let result = build(&mut line, 0);
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

    fn reject(&mut self, error: impl FnOnce() -> crate::error::ValidationError) {
        if self.value_error.is_none() {
            self.value_error = Some(error());
        }
    }

    fn reject_text(&mut self, field: &str) {
        self.reject(|| crate::error::ValidationError::Generic {
            message: format!("{field}: KVN output must contain only printable ASCII records")
                .into(),
            line: None,
        });
    }

    fn check_text(&mut self, field: &str, value: &str) {
        if !value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            self.reject_text(field);
        }
    }

    /// Writes a simple `KEY = value` line.
    ///
    /// Readers trim values, so surrounding whitespace (such as XML text padding) is not written.
    /// A blank or `N/A` value would read back as an absent keyword, so it is refused: KVN has no
    /// empty values, and an absent optional keyword is omitted instead.
    pub fn write_pair<V: Display>(&mut self, key: &str, value: V) {
        self.check_text(key, key);
        let mut line = std::mem::take(&mut self.line_buffer);
        line.clear();
        let _ = write!(line, "{key:<20} = ");
        let value_start = line.len();
        let _ = write!(line, "{value}");
        line.truncate(line.trim_end().len().max(value_start));
        let padding = line[value_start..].len() - line[value_start..].trim_start().len();
        line.drain(value_start..value_start + padding);
        if crate::kvn::parser::value_is_absent(key, &line[value_start..]) {
            let value = line[value_start..].to_owned();
            self.reject(|| crate::error::ValidationError::InvalidValue {
                field: key.to_owned().into(),
                value,
                expected: "a non-empty KVN value (omit an absent optional keyword)".into(),
                line: None,
            });
        }
        let invalid = !line[value_start..]
            .bytes()
            .all(|byte| (b' '..=b'~').contains(&byte));
        if invalid {
            let normalized = Self::normalize_inline_value(&line[value_start..]).into_owned();
            line.truncate(value_start);
            line.push_str(&normalized);
            self.reject_text(key);
        }
        line.push('\n');
        let _ = self.write_str(&line);
        self.line_buffer = line;
    }

    /// Writes an integer `KEY = value` line, refusing values outside the KVN integer range of
    /// −2³¹ through 2³¹−1 (ODM 7.5.4, ADM 6.8.4) that wider model integers can hold.
    pub(crate) fn write_integer_pair<V: Copy + Display + TryInto<i32>>(
        &mut self,
        key: &str,
        value: V,
    ) {
        if value.try_into().is_err() {
            self.reject(|| crate::error::ValidationError::OutOfRange {
                name: key.to_owned().into(),
                value: value.to_string(),
                expected: "an integer from -2147483648 through 2147483647 for KVN".into(),
                line: None,
            });
        }
        self.check_text(key, key);
        self.write_built_line(|line| {
            let _ = write!(line, "{key:<20} = {value}");
        });
    }

    /// Writes a CCSDS-compatible ODM number without allocating a value string.
    pub(crate) fn write_odm_float_pair(&mut self, key: &str, value: f64) {
        self.unrepresentable_number |= !OdmFloat::is_valid(value);
        let exact = self.exact_floats;
        self.write_built_line(|line| Self::build_odm_float_pair(line, key, value, exact));
    }

    /// Writes a CCSDS-compatible ODM number and optional unit.
    pub(crate) fn write_odm_float_measure<U: Display + Default>(
        &mut self,
        key: &str,
        measure: &UnitValue<f64, U>,
    ) {
        self.unrepresentable_number |= !OdmFloat::is_valid(measure.value);
        let exact = self.exact_floats;
        let default_unit = (measure.units.is_none() && self.always_units).then(U::default);
        let build = |line: &mut String| {
            Self::build_odm_float_pair(line, key, measure.value, exact);
            if let Some(unit) = measure.units.as_ref().or(default_unit.as_ref()) {
                line.push_str(" [");
                let _ = write!(line, "{unit}");
                line.push(']');
            }
        };
        self.write_built_line(build);
    }

    /// Writes one TDM observation without allocating intermediate epoch/value strings.
    pub(crate) fn write_tdm_observation<E: Display, V: Display>(
        &mut self,
        key: &str,
        epoch: &E,
        value: V,
    ) {
        self.write_built_line(|line| Self::build_tdm_observation(line, key, epoch, value));
    }

    /// Write an ordinary TDM number; exact phase-count strings use `write_tdm_observation`.
    pub(crate) fn write_tdm_numeric_observation<E: Display>(
        &mut self,
        key: &str,
        epoch: &E,
        value: f64,
    ) {
        self.unrepresentable_number |= !OdmFloat::is_valid(value);
        self.write_built_line(|line| {
            Self::build_tdm_observation(line, key, epoch, OdmFloat(value))
        });
    }

    /// Writes an OCM numeric history record without per-value or per-record temporary strings.
    pub(crate) fn write_ocm_numeric_history<E: Display>(&mut self, epoch: &E, values: &[f64]) {
        self.unrepresentable_number |= values.iter().any(|value| !OdmFloat::is_valid(*value));
        let build = |line: &mut String| {
            let _ = write!(line, "{epoch}");
            for value in values {
                line.push(' ');
                let _ = OdmFloat(*value).write_to(line);
            }
        };
        self.write_built_line(build);
    }

    /// Write one AEM attitude-state record without allocating intermediate strings.
    pub(crate) fn write_aem_attitude_state<E: Display>(&mut self, epoch: &E, values: &[f64]) {
        self.unrepresentable_number |= values.iter().any(|value| !OdmFloat::is_valid(*value));
        let build = |line: &mut String| {
            let _ = write!(line, "{epoch}");
            for value in values {
                line.push(' ');
                let _ = OdmFloat(*value).write_to(line);
            }
        };
        self.write_built_line(build);
    }

    /// Write a numeric vector assignment and optional unit without temporary joins.
    pub(crate) fn write_numeric_vector<U: Display>(
        &mut self,
        key: &str,
        values: &[f64],
        unit: Option<&U>,
    ) {
        self.unrepresentable_number |= values.iter().any(|value| !OdmFloat::is_valid(*value));
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
        self.write_built_line(build);
    }

    /// Preserve maneuver text columns and numeric spellings that fit; round excess numeric digits.
    pub(crate) fn write_ocm_text_history<E: Display>(
        &mut self,
        epoch: &E,
        values: &[String],
        numeric_columns: &[bool],
    ) {
        let mut line = std::mem::take(&mut self.line_buffer);
        line.clear();
        let _ = write!(line, "{epoch}");
        for (value, numeric) in values.iter().zip(numeric_columns) {
            self.check_text("OCM history record", value);
            line.push(' ');
            if !numeric {
                line.push_str(value);
                continue;
            }
            let number = crate::kvn::parser::parse_ccsds_number(value.trim())
                .filter(|number| OdmFloat::is_valid(*number));
            let Some(number) = number else {
                self.unrepresentable_number = true;
                line.push_str(value);
                continue;
            };
            let digits = value
                .split(['e', 'E'])
                .next()
                .unwrap_or("")
                .bytes()
                .filter(u8::is_ascii_digit)
                .count();
            if digits > 16 || (number == 0.0 && number.is_sign_negative()) {
                let _ = OdmFloat(number).write_to(&mut line);
            } else {
                line.push_str(value);
            }
        }
        line.push('\n');
        let _ = self.write_str(&line);
        self.line_buffer = line;
    }

    /// Writes a raw line of text.
    pub fn write_line<V: Display>(&mut self, line: V) {
        let _ = writeln!(self, "{}", line);
    }

    /// Writes comment lines.
    pub fn write_comments(&mut self, comments: &[String]) {
        for c in comments {
            if c.is_empty() {
                let _ = writeln!(self, "COMMENT ");
                continue;
            }
            for line in c.lines() {
                self.check_text("COMMENT", line);
                let normalized = Self::normalize_inline_value(line.trim_end());
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

    /// Writes the exact parameter name with the notation's USER_DEFINED_ prefix.
    pub fn write_user_defined(&mut self, parameter: &str, value: &str) {
        self.write_pair(&format!("USER_DEFINED_{parameter}"), value);
    }

    /// Returns the accumulated KVN content.
    pub fn finish(self) -> String {
        match self.output {
            KvnOutput::String(output) => output,
            KvnOutput::Io(_) => unreachable!("finish is only valid for string-backed writers"),
        }
    }

    fn validate_output(&self) -> crate::error::Result<()> {
        if let Some(line_len) = self.line_length_error {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "KVN record".into(),
                value: line_len.to_string(),
                expected: "a KVN line no longer than 254 characters".into(),
                line: None,
            }
            .into());
        }
        if let Some(error) = &self.value_error {
            return Err(error.clone().into());
        }
        if self.unrepresentable_number {
            return Err(crate::error::ValidationError::Generic {
                message: "KVN output contains a number without a representable CCSDS spelling"
                    .into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    /// Return string-backed output only when every source text value was valid KVN text.
    pub(crate) fn finish_checked(mut self) -> crate::error::Result<String> {
        if self.line_len > 0 {
            self.finish_line();
        }
        self.validate_output()?;
        Ok(self.finish())
    }

    /// Finish writing to an I/O sink and return any deferred write error.
    pub(crate) fn finish_io(mut self) -> crate::error::Result<()> {
        if self.line_len > 0 {
            self.finish_line();
        }
        if let Some(error) = self.io_error.take() {
            return Err(error.into());
        }
        self.validate_output()
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
        assert_eq!(writer.finish(), "COMMENT \n");
    }

    #[test]
    fn write_pair_rejects_multiline_value() {
        let mut writer = KvnWriter::new();
        writer.write_pair(
            "ORIGINATOR_POSITION",
            "Flight Dynamics Mission Design\nLead",
        );
        assert!(writer.finish_checked().is_err());
    }

    #[test]
    fn lexical_barrier_checks_units_written_without_write_pair() {
        let mut writer = KvnWriter::new();
        writer.write_odm_float_measure("X", &crate::types::UnitValue::new(1.0, Some("km\u{1}")));
        assert!(writer.finish_checked().is_err());
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
    fn kvn_normalizes_signed_zero_without_changing_its_numeric_value() {
        for value in [0.0, -0.0] {
            assert_eq!(OdmFloat(value).to_string(), "0.0");
        }
    }

    #[test]
    fn odm_float_accepts_representable_values_and_rejects_the_rest() {
        assert!(OdmFloat::is_valid(1.234_567_890_123_456));
        assert!(OdmFloat::is_valid(1.234_567_890_123_456_7));
        assert!(OdmFloat::is_valid(f64::MIN_POSITIVE));
        assert!(!OdmFloat::is_valid(f64::NAN));
        assert!(!OdmFloat::is_valid(f64::INFINITY));
        assert!(!OdmFloat::is_valid(f64::NEG_INFINITY));
    }

    #[test]
    fn odm_float_rejects_magnitudes_whose_rounded_spelling_overflows() {
        // Rounding to 16 digits pushes the top two magnitudes above `f64::MAX`.
        let magnitudes = [
            f64::MAX,
            f64::from_bits(f64::MAX.to_bits() - 1),
            f64::from_bits(f64::MAX.to_bits() - 2),
            f64::from_bits(f64::MAX.to_bits() - 3),
            1.0e308,
        ];
        for (index, magnitude) in magnitudes.into_iter().enumerate() {
            let representable = index >= 2;
            for value in [magnitude, -magnitude] {
                assert_eq!(
                    OdmFloat::is_valid(value),
                    representable,
                    "{value:e} representability"
                );

                let mut output = String::new();
                assert_eq!(OdmFloat::write_if_valid(value, &mut output), representable);
                if !representable {
                    assert!(output.is_empty());
                    continue;
                }
                // Every accepted spelling must read back as the finite value it came from.
                let reparsed: f64 = output.parse().expect("accepted spelling reparses");
                assert!(reparsed.is_finite(), "{output} reparsed as {reparsed}");
                assert_eq!(reparsed.is_sign_negative(), value.is_sign_negative());
            }
        }
    }

    #[test]
    fn odm_float_write_matches_its_display_spelling() {
        for value in [
            -0.0,
            1.0,
            -12.5,
            1.0e15,
            1.25e-20,
            f64::from_bits(1),
            1.234_567_890_123_456_7,
            f64::from_bits(f64::MAX.to_bits() - 2),
        ] {
            let mut output = String::new();
            assert!(OdmFloat::write_if_valid(value, &mut output));
            assert_eq!(output, OdmFloat(value).to_string());
            assert!(OdmFloat::significant_digits(&output) <= 16);
        }
        for value in [f64::NAN, f64::INFINITY, f64::MAX] {
            let mut output = String::new();
            assert!(!OdmFloat::write_if_valid(value, &mut output));
            assert!(output.is_empty());
        }
    }

    #[test]
    fn ocm_text_history_preserves_identifiers_and_checks_numeric_columns() {
        let mut writer = KvnWriter::new();
        writer.write_ocm_text_history(
            &"0",
            &[
                "12345678901234567".into(),
                "-0.0".into(),
                "1.2345678901234567".into(),
                "5e-324".into(),
            ],
            &[false, true, true, true],
        );
        assert_eq!(
            writer.finish_checked().unwrap(),
            "0 12345678901234567 0.0 1.234567890123457 5e-324\n"
        );
        for invalid in ["1e-400", "NaN", "1e400", "1.7976931348623157e308"] {
            let mut writer = KvnWriter::new();
            writer.write_ocm_text_history(&"0", &[invalid.into()], &[true]);
            assert!(writer.finish_checked().is_err(), "{invalid}");
        }
        let mut writer = KvnWriter::new();
        writer.write_ocm_text_history(&"0", &["é".into()], &[false]);
        assert!(writer
            .finish_checked()
            .unwrap_err()
            .to_string()
            .contains("OCM history record"));
    }

    #[test]
    fn rounded_value_matches_the_emitted_number() {
        let mut bits = 1u64;
        let generated = (0..10_000).map(|_| {
            bits = bits.wrapping_mul(6364136223846793005).wrapping_add(1);
            f64::from_bits(bits)
        });
        for value in [
            0.0,
            -0.0,
            f64::from_bits(1),
            -f64::from_bits(1),
            f64::MIN_POSITIVE,
            0.1 + 0.2,
            1.234_567_890_123_456_7,
            f64::from_bits(f64::MAX.to_bits() - 2),
            f64::MAX,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ]
        .into_iter()
        .chain(generated)
        {
            match OdmFloat::rounded_value(value) {
                Some(rounded) => {
                    let emitted: f64 = OdmFloat(value).to_string().parse().unwrap();
                    assert_eq!(rounded.to_bits(), emitted.to_bits(), "{value:e}");
                }
                None => assert!(!OdmFloat::is_valid(value)),
            }
        }
    }

    #[test]
    fn fallible_line_builder_commits_only_successful_lines() {
        let mut writer = KvnWriter::new();
        writer
            .try_write_validated_line(|line, _| -> Result<(), ()> {
                line.push_str("accepted");
                Ok(())
            })
            .unwrap();
        assert_eq!(
            writer.try_write_validated_line(|line, _| -> Result<(), ()> {
                line.push_str("rejected");
                Err(())
            }),
            Err(())
        );
        assert_eq!(writer.finish(), "accepted\n");
    }

    #[test]
    fn writer_enforces_the_kvn_record_limit() {
        let mut writer = KvnWriter::new();
        writer.write_line("x".repeat(254));
        assert!(writer.finish_checked().is_ok());

        let mut writer = KvnWriter::new();
        writer.write_line("x".repeat(255));
        let error = writer.finish_checked().unwrap_err();
        assert_eq!(error.code(), Some("validation.out_of_range"));
        assert!(error.to_string().contains("255"));

        let mut writer = KvnWriter::new();
        std::fmt::Write::write_str(&mut writer, &"x".repeat(255)).unwrap();
        assert!(writer.finish_checked().is_err());

        let mut output = Vec::new();
        let mut writer = KvnWriter::from_io(&mut output);
        writer.write_line("x".repeat(255));
        assert_eq!(
            writer.finish_io().unwrap_err().code(),
            Some("validation.out_of_range")
        );
    }

    #[test]
    fn writer_allows_arbitrary_line_lengths_when_the_message_standard_does() {
        let mut writer = KvnWriter::new();
        writer.allow_long_records();
        writer.write_pair("KEY", "x".repeat(240));
        assert!(writer.finish_checked().is_ok());
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
            "1.234567890123457"
        );
        // Rounded spellings are stable: writing what was read back gives the same text.
        for value in [-3.407_834_585_079_831_4, 1.234_567_890_123_456_7, 0.1 + 0.2] {
            let once = OdmFloat(value).to_string();
            assert_eq!(OdmFloat(once.parse().unwrap()).to_string(), once);
        }
        assert_eq!(
            OdmFloat(f64::from_bits(f64::MAX.to_bits() - 2)).to_string(),
            "1.797693134862315e+308"
        );
    }

    #[test]
    fn integer_pairs_preserve_values_and_reject_out_of_range_numbers() {
        for (value, fits) in [
            (i64::MIN, false),
            (i64::from(i32::MIN) - 1, false),
            (i64::from(i32::MIN), true),
            (0, true),
            (i64::from(i32::MAX), true),
            (i64::from(i32::MAX) + 1, false),
            (i64::MAX, false),
        ] {
            let mut writer = KvnWriter::new();
            writer.write_integer_pair("N", value);
            let output = writer.finish_checked();
            assert_eq!(output.is_ok(), fits, "{value}");
            match output {
                Ok(output) => {
                    assert_eq!(output, format!("N                    = {value}\n"));
                    let mut streamed = Vec::new();
                    let mut writer = KvnWriter::from_io(&mut streamed);
                    writer.write_integer_pair("N", value);
                    writer.finish_io().unwrap();
                    assert_eq!(streamed, output.as_bytes());
                }
                Err(error) => {
                    assert!(error.to_string().contains(&value.to_string()));
                }
            }
        }
        let mut writer = KvnWriter::new();
        writer.write_integer_pair("N", u64::MAX);
        assert!(writer.finish_checked().is_err());
        for key in ["bad\u{1}".to_owned(), "X".repeat(255)] {
            let mut writer = KvnWriter::new();
            writer.write_integer_pair(&key, 1);
            assert!(writer.finish_checked().is_err());
        }
    }
}
