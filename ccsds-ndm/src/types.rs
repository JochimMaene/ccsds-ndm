// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::traits::{FromKvnFloat, FromKvnValue};
use fast_float;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::str::FromStr;
use thiserror::Error;
use winnow::ascii::{digit0, digit1};
use winnow::combinator::{alt, eof, opt, seq, terminated};
use winnow::token::one_of;
use winnow::Parser;

// Base Types
//----------------------------------------------------------------------

/// Represents the `epochType` from the XSD (e.g., "2023-11-13T12:00:00.123Z").
///
/// This struct uses a stack-allocated buffer to avoid heap allocations during parsing of large NDM
/// files. The original lexical spelling remains the serialization source. Ordering keys are
/// computed only by validators that need them, avoiding permanent per-record comparison state.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Epoch {
    bytes: [u8; 64],
    len: u8,
    classification: EpochClassification,
}

/// Maximum retained lexical length of an epoch token.
///
/// This is a library storage limit, not a CCSDS or XSD maximum. It applies to numeric and
/// calendar epochs, including fractional digits, on every input route. Longer tokens are refused
/// rather than truncated; the fixed storage avoids a heap allocation for each history epoch.
pub const MAX_EPOCH_LEN: usize = 64;

/// Lexical branch of the NDM/XML `epochType` union.
///
/// The physical meaning of either branch still depends on the containing field and its time
/// system metadata.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EpochKind {
    /// Calendar-date or ordinal-date time code.
    Calendar,
    /// Signed decimal time tag.
    Numeric,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EpochClassification {
    CalendarValid,
    CalendarInvalid,
    Numeric,
}

impl Serialize for Epoch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Epoch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // An `Epoch` owns a fixed-size buffer, so the token is read straight out of the
        // deserializer's view of the document rather than through an owned `String`.
        crate::utils::deserialize_parsed(deserializer, "a CCSDS epoch value", Epoch::new)
    }
}

/// Failure to retain a schema-compatible lexical epoch token.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum EpochError {
    /// The token is not a valid epoch for the requested type.
    #[error("invalid epoch format: '{0}'")]
    InvalidFormat(String),
    #[error("epoch token is {length} bytes; maximum supported length is {maximum} bytes")]
    /// The token cannot fit the fixed epoch storage.
    TooLong {
        /// Length of the supplied token in bytes.
        length: usize,
        /// Maximum retained length in bytes.
        maximum: usize,
    },
}

fn classify_epoch(s: &str) -> Option<EpochClassification> {
    // Nearly every history record uses the normative four-digit calendar/ordinal spelling.
    // Recognize that common valid case in one pass; retain the schema-compatible parser below for
    // uncommon spellings and for values that must remain parseable-but-semantically-invalid.
    if common_calendar_fields_are_valid(s) {
        return Some(EpochClassification::CalendarValid);
    }

    fn parser(input: &mut &str) -> winnow::Result<EpochKind> {
        alt((
            // Calendar/Ordinal format: YYYY-MM-DDThh:mm:ss.sssZ
            seq!(
                opt('-'),
                digit1.verify(|s: &str| s.len() >= 4),
                '-',
                alt((
                    seq!(
                        digit1.verify(|s: &str| s.len() == 2),
                        '-',
                        digit1.verify(|s: &str| s.len() == 2)
                    )
                    .void(),
                    seq!(digit1.verify(|s: &str| s.len() == 3)).void(),
                )),
                'T',
                digit1.verify(|s: &str| s.len() == 2),
                ':',
                digit1.verify(|s: &str| s.len() == 2),
                ':',
                digit1.verify(|s: &str| s.len() == 2),
                opt(seq!('.', digit0).void()),
                opt(alt((
                    "Z".void(),
                    seq!(
                        one_of(['+', '-']),
                        digit1.verify(|s: &str| s.len() == 2),
                        ':',
                        digit1.verify(|s: &str| s.len() == 2)
                    )
                    .void()
                )))
            )
            .value(EpochKind::Calendar),
            // Numeric format: [+-]?\d*(\.\d*)?
            seq!(
                opt(one_of(['+', '-'])),
                digit0,
                opt(seq!('.', digit0).void())
            )
            .value(EpochKind::Numeric),
        ))
        .parse_next(input)
    }

    let kind = terminated(parser, eof).parse(s).ok()?;
    Some(match kind {
        EpochKind::Calendar if calendar_fields_are_valid(s) => EpochClassification::CalendarValid,
        EpochKind::Calendar => EpochClassification::CalendarInvalid,
        EpochKind::Numeric => EpochClassification::Numeric,
    })
}

fn common_calendar_fields_are_valid(value: &str) -> bool {
    #[inline(always)]
    fn decimal(bytes: &[u8]) -> Option<u16> {
        bytes.iter().try_fold(0_u16, |value, byte| {
            // Lazy: `*byte - b'0'` underflows for a non-digit byte.
            byte.is_ascii_digit()
                .then(|| value * 10 + u16::from(*byte - b'0'))
        })
    }

    let bytes = value.as_bytes();
    if bytes.len() < 17 || bytes.get(4) != Some(&b'-') {
        return false;
    }
    let (time_start, valid_date) = if bytes.get(7) == Some(&b'-') && bytes.get(10) == Some(&b'T') {
        let Some(year) = decimal(&bytes[0..4]) else {
            return false;
        };
        let Some(month) = decimal(&bytes[5..7]) else {
            return false;
        };
        let Some(day) = decimal(&bytes[8..10]) else {
            return false;
        };
        let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap_year => 29,
            2 => 28,
            _ => 0,
        };
        (11, day != 0 && day <= days_in_month)
    } else if bytes.get(8) == Some(&b'T') {
        let Some(year) = decimal(&bytes[0..4]) else {
            return false;
        };
        let Some(ordinal) = decimal(&bytes[5..8]) else {
            return false;
        };
        let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        (
            9,
            ordinal != 0 && ordinal <= if leap_year { 366 } else { 365 },
        )
    } else {
        return false;
    };
    if !valid_date || bytes.len() < time_start + 8 {
        return false;
    }
    let clock = &bytes[time_start..time_start + 8];
    if clock.get(2) != Some(&b':') || clock.get(5) != Some(&b':') {
        return false;
    }
    let (Some(hour), Some(minute), Some(second)) = (
        decimal(&clock[0..2]),
        decimal(&clock[3..5]),
        decimal(&clock[6..8]),
    ) else {
        return false;
    };
    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }

    match &bytes[time_start + 8..] {
        [] | [b'Z'] => true,
        [b'.', fraction @ ..] => {
            let fraction = fraction.strip_suffix(b"Z").unwrap_or(fraction);
            !fraction.is_empty() && fraction.iter().all(u8::is_ascii_digit)
        }
        _ => false,
    }
}

fn calendar_fields_are_valid(value: &str) -> bool {
    let Some((date, time)) = value.split_once('T') else {
        return false;
    };
    // The XSD admits negative and extended years, but the CCSDS books define absolute time tags
    // with exactly four year digits.
    if date.starts_with('-') {
        return false;
    }
    let Some((year, date_part)) = date.split_once('-') else {
        return false;
    };
    if year.len() != 4 {
        return false;
    }
    let year_mod_400 = year.bytes().fold(0_u16, |remainder, digit| {
        (remainder * 10 + u16::from(digit - b'0')) % 400
    });
    let leap_year = year_mod_400 % 4 == 0 && (year_mod_400 % 100 != 0 || year_mod_400 == 0);

    let valid_date = if let Some((month, day)) = date_part.split_once('-') {
        let month: u8 = month.parse().unwrap_or(0);
        let day: u8 = day.parse().unwrap_or(0);
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap_year => 29,
            2 => 28,
            _ => 0,
        };
        day != 0 && day <= days_in_month
    } else {
        let ordinal: u16 = date_part.parse().unwrap_or(0);
        ordinal != 0 && ordinal <= if leap_year { 366 } else { 365 }
    };

    valid_date && clock_is_valid(time)
}

/// Whether `value` is an ordinal-layout elapsed time `YYYY-DDDThh:mm:ss[.d→d][Z]`.
///
/// ODM 3.2.3.2: MET and MRT times denote a duration and use three-digit days, so day `000` and
/// days past the end of a calendar year are meaningful.
fn elapsed_fields_are_valid(value: &str) -> bool {
    let Some((date, time)) = value.split_once('T') else {
        return false;
    };
    let Some((year, day)) = date.split_once('-') else {
        return false;
    };
    year.len() == 4
        && day.len() == 3
        && year
            .bytes()
            .chain(day.bytes())
            .all(|byte| byte.is_ascii_digit())
        && clock_is_valid(time)
}

fn clock_is_valid(time: &str) -> bool {
    if time.len() < 8 {
        return false;
    }

    let timezone_start = time[8..]
        .char_indices()
        .find_map(|(offset, character)| matches!(character, '+' | '-').then_some(offset + 8));
    let (clock, timezone) = match timezone_start {
        Some(offset) => (&time[..offset], Some(&time[offset..])),
        None if time.ends_with('Z') => (&time[..time.len() - 1], Some(&time[time.len() - 1..])),
        None => (time, None),
    };

    if clock.len() < 8 {
        return false;
    }
    let hour: u8 = clock[0..2].parse().unwrap_or(24);
    let minute: u8 = clock[3..5].parse().unwrap_or(60);
    let second: u8 = clock[6..8].parse().unwrap_or(61);
    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }

    if let Some(fraction) = clock[8..].strip_prefix('.') {
        if fraction.is_empty() {
            return false;
        }
    } else if clock.len() != 8 {
        return false;
    }

    if let Some(timezone) = timezone {
        if timezone == "Z" {
            return true;
        }
        // The schema's union permits offsets, while ODM §7.5.10 and ADM §6.8.9 permit only the
        // optional `Z` terminator. Keep the XSD spelling parseable as `Epoch`, but classify it as
        // semantically invalid for strict message fields.
        return false;
    }
    true
}

/// An optional or required time tag, for lexical checks over mixed fields.
pub(crate) trait EpochText {
    fn epoch(&self) -> Option<&Epoch>;
}

impl EpochText for Epoch {
    fn epoch(&self) -> Option<&Epoch> {
        Some(self)
    }
}

impl EpochText for CalendarEpoch {
    fn epoch(&self) -> Option<&Epoch> {
        Some(&self.0)
    }
}

impl<T: EpochText> EpochText for Option<T> {
    fn epoch(&self) -> Option<&Epoch> {
        self.as_ref().and_then(EpochText::epoch)
    }
}

/// Number of fractional-second digits of a calendar or ordinal spelling; 0 for numeric ones.
pub(crate) fn fraction_digit_count(bytes: &[u8]) -> usize {
    // Count trailing digits back from an optional `Z`; they are a fraction only when a `.`
    // follows the `:ss` seconds field, which numeric epochs never have.
    let end = bytes.len() - usize::from(bytes.last() == Some(&b'Z'));
    let digits = bytes[..end]
        .iter()
        .rev()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    let dot = end - digits;
    let is_fraction = dot >= 4 && bytes[dot - 1] == b'.' && bytes[dot - 4] == b':';
    if is_fraction {
        digits
    } else {
        0
    }
}

impl Epoch {
    /// Retain a schema-compatible epoch spelling of at most MAX_EPOCH_LEN bytes.
    /// Message validation additionally enforces calendar and book-specific field rules.
    pub fn new(value: &str) -> std::result::Result<Self, EpochError> {
        // Fast path for empty or very short strings which are common in some tests
        // and allowed by the regex [+-]?\d*(\.\d*)?
        if value.len() > MAX_EPOCH_LEN {
            return Err(EpochError::TooLong {
                length: value.len(),
                maximum: MAX_EPOCH_LEN,
            });
        }

        let classification = if value.is_empty() {
            Some(EpochClassification::Numeric)
        } else {
            classify_epoch(value)
        };

        if let Some(classification) = classification {
            let mut bytes = [0u8; MAX_EPOCH_LEN];
            bytes[..value.len()].copy_from_slice(value.as_bytes());
            Ok(Epoch {
                bytes,
                len: value.len() as u8,
                classification,
            })
        } else {
            Err(EpochError::InvalidFormat(value.to_string()))
        }
    }
    /// Borrow the retained epoch spelling without normalization.
    pub fn as_str(&self) -> &str {
        // Bytes are validated to be ASCII/UTF-8 during creation.
        std::str::from_utf8(&self.bytes[..self.len as usize])
            .expect("Epoch bytes must be valid UTF-8")
    }

    /// Returns true if the epoch is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the lexical branch selected by this epoch.
    pub fn kind(&self) -> EpochKind {
        match self.classification {
            EpochClassification::CalendarValid | EpochClassification::CalendarInvalid => {
                EpochKind::Calendar
            }
            EpochClassification::Numeric => EpochKind::Numeric,
        }
    }

    /// Returns whether a calendar/ordinal epoch contains real date and clock fields.
    ///
    /// Numeric epochs return `None`; their meaning depends on the containing message context.
    pub fn calendar_fields_are_valid(&self) -> Option<bool> {
        match self.classification {
            EpochClassification::CalendarValid => Some(true),
            EpochClassification::CalendarInvalid => Some(false),
            EpochClassification::Numeric => None,
        }
    }

    /// Returns whether a numeric epoch contains at least one digit and does not end in a decimal
    /// point. The XSD regex also admits empty and sign-only spellings, so contextual validation may
    /// need this stronger check.
    pub fn numeric_is_non_degenerate(&self) -> Option<bool> {
        (self.kind() == EpochKind::Numeric).then(|| {
            let value = self.as_str();
            value.bytes().any(|byte| byte.is_ascii_digit()) && !value.ends_with('.')
        })
    }

    /// Returns whether this is an ordinal-layout elapsed time, as MET and MRT use.
    pub(crate) fn is_elapsed_time(&self) -> bool {
        self.kind() == EpochKind::Calendar && elapsed_fields_are_valid(self.as_str())
    }

    /// Returns the number of fractional-second digits of a calendar or ordinal epoch.
    pub(crate) fn fraction_digits(&self) -> usize {
        fraction_digit_count(&self.bytes[..usize::from(self.len)])
    }

    /// Compares two validated epochs without converting through a physical-time library.
    ///
    /// The comparison is meaningful only when both values use the same lexical branch. Numeric
    /// values are compared as decimal strings so significant digits are not lost to `f64`.
    /// Calendar values are compared by date, clock, and fractional seconds after book-level
    /// validation has rejected unsupported timezone-offset spellings. Comparison state is derived
    /// transiently so values that are never ordered carry no cache overhead.
    /// `None` means that the values cannot be compared by this context-free operation.
    pub(crate) fn cmp_same_branch(&self, other: &Self) -> Option<Ordering> {
        self.order_key()?.compare(&other.order_key()?)
    }

    /// Derives the transient comparison state used by history validators.
    pub(crate) fn order_key(&self) -> Option<EpochOrderKey<'_>> {
        let inner = match self.classification {
            EpochClassification::CalendarValid => EpochOrderKeyInner::Calendar {
                value: self.as_str(),
            },
            // Elapsed times share the calendar comparison: same-layout values compare
            // lexically and cross-layout values by day count and second of day.
            EpochClassification::CalendarInvalid if self.is_elapsed_time() => {
                EpochOrderKeyInner::Calendar {
                    value: self.as_str(),
                }
            }
            EpochClassification::CalendarInvalid => return None,
            EpochClassification::Numeric => {
                let parts = parse_decimal(self.as_str())?;
                EpochOrderKeyInner::Numeric {
                    parts,
                    magnitude: parts.magnitude(),
                }
            }
        };
        Some(EpochOrderKey(inner))
    }
}

/// Comparison state retained only for the duration of one validation pass.
#[derive(Clone, Copy)]
pub(crate) struct EpochOrderKey<'a>(EpochOrderKeyInner<'a>);

#[derive(Clone, Copy)]
enum EpochOrderKeyInner<'a> {
    Calendar {
        value: &'a str,
    },
    Numeric {
        parts: DecimalParts<'a>,
        magnitude: Option<DecimalMagnitude>,
    },
}

impl EpochOrderKey<'_> {
    pub(crate) fn compare(&self, other: &Self) -> Option<Ordering> {
        match (self.0, other.0) {
            (
                EpochOrderKeyInner::Calendar { value: left },
                EpochOrderKeyInner::Calendar { value: right },
            ) => compare_calendar(left, right),
            (
                EpochOrderKeyInner::Numeric {
                    parts: left_parts,
                    magnitude: left_magnitude,
                },
                EpochOrderKeyInner::Numeric {
                    parts: right_parts,
                    magnitude: right_magnitude,
                },
            ) => Some(compare_numeric_parts(
                left_parts,
                left_magnitude,
                right_parts,
                right_magnitude,
            )),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct DecimalParts<'a> {
    negative: bool,
    integer: &'a str,
    fraction: &'a str,
}

#[derive(Clone, Copy)]
struct DecimalMagnitude {
    first_in_integer: bool,
    first_index: usize,
    first_digit_position: i32,
    significant_length: usize,
}

fn parse_decimal(value: &str) -> Option<DecimalParts<'_>> {
    let (negative, unsigned) = match value.as_bytes().first().copied() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    if integer.is_empty() && fraction.is_empty() {
        return None;
    }
    if !integer
        .bytes()
        .chain(fraction.bytes())
        .all(|b| b.is_ascii_digit())
    {
        return None;
    }
    Some(DecimalParts {
        negative,
        integer,
        fraction,
    })
}

impl<'a> DecimalParts<'a> {
    fn magnitude(self) -> Option<DecimalMagnitude> {
        let integer_start = self
            .integer
            .as_bytes()
            .iter()
            .position(|&digit| digit != b'0')
            .unwrap_or(self.integer.len());
        if integer_start < self.integer.len() {
            return Some(DecimalMagnitude {
                first_in_integer: true,
                first_index: integer_start,
                first_digit_position: (self.integer.len() - integer_start - 1) as i32,
                significant_length: self.integer.len() - integer_start + self.fraction.len(),
            });
        }

        let fraction_start = self
            .fraction
            .as_bytes()
            .iter()
            .position(|&digit| digit != b'0')?;
        Some(DecimalMagnitude {
            first_in_integer: false,
            first_index: fraction_start,
            first_digit_position: -((fraction_start + 1) as i32),
            significant_length: self.fraction.len() - fraction_start,
        })
    }
}

fn compare_numeric_parts(
    left_parts: DecimalParts<'_>,
    left_magnitude: Option<DecimalMagnitude>,
    right_parts: DecimalParts<'_>,
    right_magnitude: Option<DecimalMagnitude>,
) -> Ordering {
    match (left_magnitude, right_magnitude) {
        (None, None) => return Ordering::Equal,
        (None, Some(_)) if right_parts.negative => return Ordering::Greater,
        (None, Some(_)) => return Ordering::Less,
        (Some(_), None) if left_parts.negative => return Ordering::Less,
        (Some(_), None) => return Ordering::Greater,
        (Some(_), Some(_)) => {}
    }

    if left_parts.negative != right_parts.negative {
        return if left_parts.negative {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }

    let left_magnitude = left_magnitude.expect("handled zero magnitude above");
    let right_magnitude = right_magnitude.expect("handled zero magnitude above");
    let magnitude_order = left_magnitude
        .first_digit_position
        .cmp(&right_magnitude.first_digit_position)
        .then_with(|| {
            let length = left_magnitude
                .significant_length
                .max(right_magnitude.significant_length);
            (0..length)
                .map(|index| {
                    numeric_digit(left_parts, left_magnitude, index).cmp(&numeric_digit(
                        right_parts,
                        right_magnitude,
                        index,
                    ))
                })
                .find(|ordering| *ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        });

    if left_parts.negative {
        magnitude_order.reverse()
    } else {
        magnitude_order
    }
}

fn numeric_digit(parts: DecimalParts<'_>, magnitude: DecimalMagnitude, index: usize) -> u8 {
    if magnitude.first_in_integer {
        let integer_length = parts.integer.len() - magnitude.first_index;
        if index < integer_length {
            return parts.integer.as_bytes()[magnitude.first_index + index];
        }
        return parts
            .fraction
            .as_bytes()
            .get(index - integer_length)
            .copied()
            .unwrap_or(b'0');
    }
    parts
        .fraction
        .as_bytes()
        .get(magnitude.first_index + index)
        .copied()
        .unwrap_or(b'0')
}

fn compare_calendar(left: &str, right: &str) -> Option<Ordering> {
    if left == right {
        return Some(Ordering::Equal);
    }

    let whole_seconds_end = |value: &str| {
        if value.as_bytes().get(7) == Some(&b'-') {
            19
        } else {
            17
        }
    };
    let left_end = whole_seconds_end(left);
    let right_end = whole_seconds_end(right);

    // Within one CCSDS date layout, the date and whole-second fields are fixed-width and sort
    // lexically. Across layouts, compare an exact day count and second of day instead.
    // Fractional seconds need numeric comparison because the optional `Z` terminator and
    // differing precision make a direct whole-string comparison incorrect.
    let whole = if left_end == right_end {
        left[..left_end].cmp(&right[..right_end])
    } else {
        whole_seconds(left)?.cmp(&whole_seconds(right)?)
    };
    Some(whole.then_with(|| {
        compare_fraction(
            calendar_fraction(left, left_end),
            calendar_fraction(right, right_end),
        )
    }))
}

fn calendar_fraction(value: &str, whole_seconds_end: usize) -> &[u8] {
    let suffix = &value[whole_seconds_end..];
    suffix
        .strip_suffix('Z')
        .unwrap_or(suffix)
        .strip_prefix('.')
        .unwrap_or("")
        .as_bytes()
}

/// Compare fractional-second digits, treating missing trailing digits as zeros.
fn compare_fraction(left: &[u8], right: &[u8]) -> Ordering {
    let digit = |fraction: &[u8], index: usize| fraction.get(index).copied().unwrap_or(b'0');
    (0..left.len().max(right.len()))
        .map(|index| digit(left, index).cmp(&digit(right, index)))
        .find(|ordering| *ordering != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

/// Return the proleptic-Gregorian day number and second of day of a calendar-valid epoch.
fn whole_seconds(value: &str) -> Option<(i64, i32)> {
    // `cmp_same_branch` admits only values classified as calendar-valid at construction.
    let (date, time) = value.split_once('T')?;
    let (year, date_part) = date.split_once('-')?;
    let year = year.parse::<i64>().ok()?;
    let day_of_year = if let Some((month, day)) = date_part.split_once('-') {
        let month = month.parse::<u8>().ok()?;
        let day = day.parse::<u8>().ok()?;
        let month_start = [0_u16, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut ordinal = month_start.get(month as usize).copied()?;
        if month > 2 && is_leap_year(year) {
            ordinal += 1;
        }
        ordinal + u16::from(day)
    } else {
        date_part.parse::<u16>().ok()?
    };
    // Use an actual proleptic-Gregorian day count so calendar and ordinal forms share one exact,
    // gap-free key across year boundaries.
    let day = days_before_year(year)?.checked_add(i64::from(day_of_year))?;

    let hour = parse_two_digits(time.get(0..2)?)?;
    let minute = parse_two_digits(time.get(3..5)?)?;
    let second = parse_two_digits(time.get(6..8)?)?;
    Some((
        day,
        i32::from(hour) * 3600 + i32::from(minute) * 60 + i32::from(second),
    ))
}

fn days_before_year(year: i64) -> Option<i64> {
    let previous_year = year.checked_sub(1)?;
    previous_year
        .checked_mul(365)?
        .checked_add(previous_year.div_euclid(4))?
        .checked_sub(previous_year.div_euclid(100))?
        .checked_add(previous_year.div_euclid(400))
}

fn parse_two_digits(value: &str) -> Option<u8> {
    let bytes = value.as_bytes();
    (bytes.len() == 2 && bytes[0].is_ascii_digit() && bytes[1].is_ascii_digit())
        .then(|| (bytes[0] - b'0') * 10 + bytes[1] - b'0')
}

fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Epoch {
    type Err = EpochError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for Epoch {
    type Error = EpochError;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}

/// An epoch known to use the calendar/ordinal branch of the CCSDS `epochType` union.
///
/// `Epoch` intentionally mirrors the XSD union and therefore accepts numeric spellings as well.
/// Use this wrapper when the containing field requires an absolute calendar or ordinal date. The
/// original lexical spelling is retained and conversions never normalize the value.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CalendarEpoch(Epoch);

impl CalendarEpoch {
    /// Parses a valid calendar or ordinal epoch while retaining its exact spelling.
    pub fn new(value: &str) -> std::result::Result<Self, EpochError> {
        let epoch = Epoch::new(value)?;
        if epoch.kind() != EpochKind::Calendar || epoch.calendar_fields_are_valid() != Some(true) {
            return Err(EpochError::InvalidFormat(value.to_string()));
        }
        Ok(Self(epoch))
    }

    /// Returns the original lexical spelling.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns true if the wrapped spelling is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts back to the XSD-compatible epoch union without changing its spelling.
    pub fn into_epoch(self) -> Epoch {
        self.0
    }
}

impl std::fmt::Display for CalendarEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CalendarEpoch {
    type Err = EpochError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Epoch> for CalendarEpoch {
    type Error = EpochError;

    fn try_from(epoch: Epoch) -> std::result::Result<Self, Self::Error> {
        if epoch.kind() != EpochKind::Calendar || epoch.calendar_fields_are_valid() != Some(true) {
            return Err(EpochError::InvalidFormat(epoch.as_str().to_string()));
        }
        Ok(Self(epoch))
    }
}

impl TryFrom<String> for CalendarEpoch {
    type Error = EpochError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl From<CalendarEpoch> for Epoch {
    fn from(epoch: CalendarEpoch) -> Self {
        epoch.0
    }
}

impl Serialize for CalendarEpoch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CalendarEpoch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::utils::deserialize_parsed(deserializer, "a CCSDS calendar epoch value", Self::new)
    }
}

/// A relative time using the ACM/ADM `relTimeType` lexical contract.
///
/// This is deliberately separate from the numeric branch of [`Epoch`]. The common `epochType`
/// numeric branch is a restricted decimal grammar, while `relTimeType` is based on XML Schema
/// `xsd:double` and permits the standard fixed-point and scientific-notation forms. The original
/// spelling is retained and no physical-time conversion is performed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RelativeTime {
    bytes: [u8; 64],
    len: u8,
}

impl RelativeTime {
    /// Parses a finite relative time while retaining its exact lexical spelling.
    pub fn new(value: &str) -> std::result::Result<Self, EpochError> {
        if value.len() > MAX_EPOCH_LEN {
            return Err(EpochError::TooLong {
                length: value.len(),
                maximum: MAX_EPOCH_LEN,
            });
        }
        if !relative_time_is_valid(value) {
            return Err(EpochError::InvalidFormat(value.to_string()));
        }
        let mut bytes = [0_u8; MAX_EPOCH_LEN];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self {
            bytes,
            len: value.len() as u8,
        })
    }

    /// Returns the original lexical spelling.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len as usize])
            .expect("RelativeTime bytes must be valid UTF-8")
    }
}

fn relative_time_is_valid(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_EPOCH_LEN
        && crate::kvn::parser::parse_ccsds_number(value).is_some()
}

impl std::fmt::Display for RelativeTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for RelativeTime {
    type Err = EpochError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for RelativeTime {
    type Error = EpochError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl Serialize for RelativeTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RelativeTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RelativeTimeVisitor;
        impl<'de> serde::de::Visitor<'de> for RelativeTimeVisitor {
            type Value = RelativeTime;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a CCSDS relative time in seconds")
            }
            fn visit_str<E: serde::de::Error>(
                self,
                value: &str,
            ) -> std::result::Result<Self::Value, E> {
                RelativeTime::new(value).map_err(E::custom)
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Self::Value, M::Error> {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "$text" | "$value" => value = Some(map.next_value::<String>()?),
                        "@units" => {
                            let units = Some(map.next_value::<TimeUnits>()?);
                            TimeUnits::validate_seconds(&units, "relative time")
                                .map_err(serde::de::Error::custom)?;
                        }
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let value =
                    value.ok_or_else(|| serde::de::Error::missing_field("relative time"))?;
                RelativeTime::new(&value).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(RelativeTimeVisitor)
    }
}

//----------------------------------------------------------------------
// Generic Unit/Value Types
//----------------------------------------------------------------------

/// A trait for types that can be deserialized from a KVN value and optional unit.
///
/// This trait provides a standardized way to parse key-value pairs from KVN files,
/// where a value might have an associated unit in brackets (e.g., `KEY = 123.45 [km]`).
pub(crate) trait FromKvn: Sized {
    /// Creates an instance from a KVN value string and an optional unit string.
    ///
    /// # Arguments
    /// * `value` - The string representation of the value.
    /// * `unit` - An optional string representation of the unit.
    ///
    /// # Returns
    /// A `Result` containing the parsed type or a `CcsdsNdmError`.
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self>;
}

/// A generic container for a value and its associated unit.
///
/// This struct is used throughout the library to represent measurements
/// like position, velocity, etc., which have a numerical value and an
/// optional unit enum.
///
/// # Type Parameters
/// * `V`: The type of the value (e.g., `f64`, `i32`).
/// * `U`: The type of the unit enum (e.g., `PositionUnits`).
#[derive(Serialize, Debug, PartialEq, Clone, Default)]
#[serde(bound(serialize = "V: Serialize + 'static, U: Serialize"))]
pub struct UnitValue<V, U> {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value", serialize_with = "serialize_xml_value")]
    pub value: V,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<U>,
}

/// Serialize a [`UnitValue`] value in its XML lexical form. ODM 8.13.4 uses the xsd:double
/// lexical space, which spells the special values `INF`, `-INF`, and `NaN` rather than Rust's
/// `inf` and `-inf`; every other value keeps its own serialization.
fn serialize_xml_value<V, S>(value: &V, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    V: Serialize + 'static,
    S: serde::Serializer,
{
    match (value as &dyn std::any::Any).downcast_ref::<f64>() {
        Some(value) if value.is_nan() => serializer.serialize_str("NaN"),
        Some(&f64::INFINITY) => serializer.serialize_str("INF"),
        Some(&f64::NEG_INFINITY) => serializer.serialize_str("-INF"),
        // `Display` never uses an exponent, so 1e308 would span 309 digits. Beyond the range
        // where it stays compact, write the shortest exact exponent form instead.
        Some(&value) if value != 0.0 && !(1e-5..1e16).contains(&value.abs()) => {
            serializer.serialize_str(zmij::Buffer::new().format_finite(value))
        }
        _ => value.serialize(serializer),
    }
}

/// Serialize a fixed-unit [`UnitValue`] as its value alone. Where the schema's `units`
/// attribute is optional and allows a single unit, it restates the implied unit, and the ODM
/// examples omit it.
pub(crate) fn serialize_without_units<V, U, S>(
    value: &UnitValue<V, U>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    V: Serialize + 'static,
    S: serde::Serializer,
{
    serialize_xml_value(&value.value, serializer)
}

/// [`serialize_without_units`] for an optional element, which the field skips when absent.
pub(crate) fn serialize_optional_without_units<V, U, S>(
    value: &Option<UnitValue<V, U>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    V: Serialize + 'static,
    S: serde::Serializer,
{
    match value {
        Some(value) => serialize_without_units(value, serializer),
        None => serializer.serialize_none(),
    }
}

/// Parse a [`UnitValue`] value from decoded XML text. Surrounding XML whitespace is collapsed;
/// special values must use the xsd:double spellings `INF`, `-INF`, and `NaN` (ODM 8.13.4),
/// which Rust's float parser would otherwise extend with `inf`, `+infinity`, `nan`, ...
pub(crate) fn parse_xml_value<V>(text: &str) -> std::result::Result<V, String>
where
    V: FromStr,
    V::Err: std::fmt::Display,
{
    let value = text.trim_matches([' ', '\t', '\r', '\n']);
    let special = value.trim_start_matches(['+', '-']);
    if ["inf", "infinity", "nan"]
        .iter()
        .any(|name| special.eq_ignore_ascii_case(name))
        && !matches!(value, "INF" | "-INF" | "NaN")
    {
        return Err(format!(
            "'{value}' is not an xsd:double value; use INF, -INF, or NaN"
        ));
    }
    value.parse::<V>().map_err(|error| error.to_string())
}

impl<V, U> FromStr for UnitValue<V, U>
where
    UnitValue<V, U>: FromKvn,
{
    type Err = crate::error::CcsdsNdmError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_kvn(s, None)
    }
}

impl<'de, V, U> serde::Deserialize<'de> for UnitValue<V, U>
where
    V: serde::Deserialize<'de> + std::str::FromStr,
    V::Err: std::fmt::Display,
    U: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// The three keys a `UnitValue` map can carry. Deserializing into this instead of a
        /// `String` keeps the value elements of a large ephemeris allocation-free: every record
        /// carries several of them, and the key text is only ever compared, never retained.
        enum UnitValueKey {
            Value,
            Units,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for UnitValueKey {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                crate::utils::deserialize_parsed(
                    deserializer,
                    "a UnitValue field name",
                    |key| -> std::result::Result<UnitValueKey, std::convert::Infallible> {
                        Ok(match key {
                            "$value" | "$text" => UnitValueKey::Value,
                            "@units" => UnitValueKey::Units,
                            _ => UnitValueKey::Other,
                        })
                    },
                )
            }
        }

        /// Deserializes element text through [`parse_xml_value`].
        struct XmlValueSeed<V>(std::marker::PhantomData<V>);

        impl<'de, V> serde::de::DeserializeSeed<'de> for XmlValueSeed<V>
        where
            V: std::str::FromStr,
            V::Err: std::fmt::Display,
        {
            type Value = V;

            fn deserialize<D>(self, deserializer: D) -> std::result::Result<V, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(self)
            }
        }

        /// XML supplies text, checked against the xsd:double spelling. Self-describing formats
        /// such as JSON round-trip the numbers `UnitValue` serializes, whose `Display` form is
        /// exact.
        impl<V> serde::de::Visitor<'_> for XmlValueSeed<V>
        where
            V: std::str::FromStr,
            V::Err: std::fmt::Display,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<V, E> {
                parse_xml_value::<V>(v).map_err(E::custom)
            }

            fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<V, E> {
                v.to_string().parse::<V>().map_err(E::custom)
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<V, E> {
                v.to_string().parse::<V>().map_err(E::custom)
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<V, E> {
                v.to_string().parse::<V>().map_err(E::custom)
            }
        }

        struct UnitValueVisitor<V, U>(std::marker::PhantomData<(V, U)>);

        impl<'de, V, U> serde::de::Visitor<'de> for UnitValueVisitor<V, U>
        where
            V: serde::Deserialize<'de> + std::str::FromStr,
            V::Err: std::fmt::Display,
            U: serde::Deserialize<'de>,
        {
            type Value = UnitValue<V, U>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = parse_xml_value::<V>(v).map_err(E::custom)?;
                Ok(UnitValue { value, units: None })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<UnitValueKey>()? {
                    match key {
                        UnitValueKey::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("$value"));
                            }
                            value = Some(
                                map.next_value_seed(XmlValueSeed::<V>(std::marker::PhantomData))?,
                            );
                        }
                        UnitValueKey::Units => {
                            if units.is_some() {
                                return Err(serde::de::Error::duplicate_field("@units"));
                            }
                            units = Some(map.next_value()?);
                        }
                        UnitValueKey::Other => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Ok(UnitValue { value, units })
            }
        }

        deserializer.deserialize_any(UnitValueVisitor(std::marker::PhantomData))
    }
}

impl<V: std::fmt::Display, U> std::fmt::Display for UnitValue<V, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<V, U> UnitValue<V, U> {
    /// Creates a new UnitValue with the given value and optional units.
    pub fn new(value: V, units: Option<U>) -> Self {
        Self { value, units }
    }
}

impl<V, U> FromKvn for UnitValue<V, U>
where
    V: FromStr,
    CcsdsNdmError: From<V::Err>,
    U: FromStr,
    CcsdsNdmError: From<U::Err>,
{
    /// Parses a `UnitValue` from a value string and an optional unit string.
    ///
    /// The value is parsed using its `FromStr` implementation. If a unit string
    /// is provided, it is parsed using the unit type's `FromStr` implementation.
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let value = value.parse::<V>()?;

        let units = match unit {
            Some(u_str) => Some(u_str.parse::<U>().map_err(CcsdsNdmError::from)?),
            None => None,
        };

        Ok(UnitValue { value, units })
    }
}

impl<U> FromKvnFloat for UnitValue<f64, U>
where
    U: FromStr,
    CcsdsNdmError: From<U::Err>,
{
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let units = match unit {
            Some(u_str) => Some(u_str.parse::<U>().map_err(CcsdsNdmError::from)?),
            None => None,
        };
        Ok(UnitValue { value, units })
    }
}

fn kvn_unit_or_default<U>(unit: Option<&str>, default: U) -> Result<U>
where
    U: FromStr,
    CcsdsNdmError: From<U::Err>,
{
    match unit {
        Some(value) => value.parse::<U>().map_err(CcsdsNdmError::from),
        None => Ok(default),
    }
}

fn reject_kvn_unit(unit: Option<&str>) -> Result<()> {
    match unit {
        Some(value) => Err(crate::error::EnumParseError {
            field: "unit",
            value: value.to_owned(),
            expected: "no unit",
        }
        .into()),
        None => Ok(()),
    }
}

/// Rejects NaN, both infinities, zero, and negative values for a field whose XSD type resolves to
/// `positiveDouble`.
pub(crate) fn require_positive(value: f64, name: &'static str) -> Result<()> {
    if !value.is_finite() {
        return Err(crate::error::ValidationError::InvalidValue {
            field: name.into(),
            value: value.to_string(),
            expected: "a finite number > 0".into(),
            line: None,
        }
        .into());
    }
    if value <= 0.0 {
        return Err(crate::error::ValidationError::OutOfRange {
            name: name.into(),
            value: value.to_string(),
            expected: "> 0".into(),
            line: None,
        }
        .into());
    }
    Ok(())
}

/// Rejects NaN, both infinities, and negative values for a nonnegative field, whether its domain
/// comes from book semantics or an XSD `nonNegativeDouble` facet.
///
/// The finiteness and sign failures stay distinct because they mean different things to a caller:
/// a non-finite value is not a number the standard can express at all, while a negative one is a
/// real number outside the field's domain.
pub(crate) fn require_non_negative(value: f64, name: &'static str) -> Result<()> {
    if !value.is_finite() {
        return Err(crate::error::ValidationError::InvalidValue {
            field: name.into(),
            value: value.to_string(),
            expected: "a finite number >= 0".into(),
            line: None,
        }
        .into());
    }
    if value < 0.0 {
        return Err(crate::error::ValidationError::OutOfRange {
            name: name.into(),
            value: value.to_string(),
            expected: ">= 0".into(),
            line: None,
        }
        .into());
    }
    Ok(())
}

/// Rejects NaN and both infinities for a numeric field that has no narrower domain.
///
/// CCSDS books describe every numeric keyword as a real measurement, so a non-finite value is
/// never a valid model value even where the XSD only says `xsd:double`.
pub(crate) fn require_finite(field: &'static str, value: f64) -> Result<()> {
    if value.is_finite() {
        return Ok(());
    }
    Err(crate::error::ValidationError::InvalidValue {
        field: field.into(),
        value: value.to_string(),
        expected: "a finite number".into(),
        line: None,
    }
    .into())
}

//----------------------------------------------------------------------
// Macros to reduce boilerplate for unit enums and wrappers
//----------------------------------------------------------------------

/// Defines a "required" wrapper struct that always carries units (no Option)
/// and constructs with the provided default unit variant.
///
/// Example:
/// define_required_type!(PositionRequired, PositionUnits, Km);
macro_rules! define_required_type {
    ($name:ident, $unit_enum:ident, $default_unit:ident) => {
        #[derive(Serialize, Debug, PartialEq, Clone, Default)]
        #[doc = concat!("Numeric quantity with a required `", stringify!($unit_enum), "` annotation.")]
        pub struct $name {
            /// Numeric value in the declared units.
            #[serde(rename = "$value")]
            pub value: f64,
            /// Required unit annotation; construction selects the standard default.
            #[serde(rename = "@units")]
            pub units: $unit_enum,
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a float value or map with $value and @units")
                    }

                    fn visit_f64<E>(self, _v: f64) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(E::custom(concat!(
                            "missing required units attribute for ",
                            stringify!($name)
                        )))
                    }

                    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let _ = v;
                        Err(E::custom(concat!(
                            "missing required units attribute for ",
                            stringify!($name)
                        )))
                    }

                    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value = None;
                        let mut units = None;
                        while let Some(key) = map.next_key::<String>()? {
                            if key == "$value" || key == "$text" {
                                value = Some(map.next_value::<f64>()?);
                            } else if key == "@units" {
                                units = Some(map.next_value::<$unit_enum>()?);
                            } else {
                                let _ = map.next_value::<serde::de::IgnoredAny>()?;
                            }
                        }
                        let value = value.ok_or_else(|| {
                            serde::de::Error::custom("an empty element is not a number")
                        })?;
                        let units =
                            units.ok_or_else(|| serde::de::Error::missing_field("@units"))?;
                        Ok($name { value, units })
                    }
                }
                deserializer.deserialize_any(Visitor)
            }
        }
        impl $name {
            /// Construct a quantity with its standard default unit; owning message validation checks its domain.
            pub fn new(value: f64) -> Self {
                Self {
                    value,
                    units: $unit_enum::$default_unit,
                }
            }
            /// Copy the quantity into an optional-unit wrapper, preserving the unit annotation.
            pub fn to_unit_value(&self) -> UnitValue<f64, $unit_enum> {
                UnitValue {
                    value: self.value,
                    units: Some(self.units.clone()),
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
        impl FromKvnFloat for $name {
            fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
                Ok(Self {
                    value,
                    units: kvn_unit_or_default(unit, $unit_enum::$default_unit)?,
                })
            }
        }
    };
}

// Local macro to define only unit enums with serde/Default/Display/FromStr
macro_rules! define_unit_enum {
    ($unit_enum:ident, $default_variant:ident, { $($variant:ident => $str_rep:expr),+ $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        #[doc = concat!("Unit spellings accepted by `", stringify!($unit_enum), "` quantities.")]
        pub enum $unit_enum { $(#[doc = $str_rep] #[serde(rename = $str_rep)] $variant),+ }
        impl Default for $unit_enum { fn default() -> Self { Self::$default_variant } }
        impl std::fmt::Display for $unit_enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self { $(Self::$variant => write!(f, $str_rep)),+ }
            }
        }
        impl std::str::FromStr for $unit_enum {
            type Err = crate::error::EnumParseError;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s { $($str_rep => Ok(Self::$variant)),+, _ => Err(crate::error::EnumParseError {
                    field: "unit",
                    value: s.to_string(),
                    expected: stringify!($($str_rep),+),
                }) }
            }
        }
    };
}

/// Defines a unit enum with [`define_unit_enum!`], plus a `UnitValue<f64, UnitEnum>` type alias
/// with the provided name.
///
/// Usage:
/// define_unit_type!(
///     Position, PositionUnits, Km, { Km => "km" }
/// );
macro_rules! define_unit_type {
    ($type_alias:ident, $unit_enum:ident, $default_variant:ident, { $($variant:ident => $str_rep:expr),+ $(,)? }) => {
        define_unit_enum!($unit_enum, $default_variant, { $($variant => $str_rep),+ });
        #[doc = concat!("Numeric quantity with optional `", stringify!($unit_enum), "` units. The containing field defines its meaning and range.")]
        pub type $type_alias = UnitValue<f64, $unit_enum>;
    };
}

/// Defines a unit enum and a required wrapper struct in one go.
macro_rules! define_required_unit_type {
    ($name:ident, $unit_enum:ident, $default_variant:ident, { $($variant:ident => $str_rep:expr),+ $(,)? }) => {
        define_unit_enum!($unit_enum, $default_variant, { $($variant => $str_rep),+ });
        define_required_type!($name, $unit_enum, $default_variant);
    };
}

//----------------------------------------------------------------------
// Unit/Value Types
//----------------------------------------------------------------------

// Unit for Acceleration: `accUnits` and alias `Acc`
define_unit_type!(
    Acc,
    AccUnits,
    KmPerS2,
    { KmPerS2 => "km/s**2" }
);

// --- Position ---
define_unit_type!(
    Position,
    PositionUnits,
    Km,
    { Km => "km" }
);

define_required_type!(PositionRequired, PositionUnits, Km);
// --- Velocity ---

define_unit_type!(
    Velocity,
    VelocityUnits,
    KmPerS,
    { KmPerS => "km/s" }
);

define_required_type!(VelocityRequired, VelocityUnits, KmPerS);
// Type alias for Distance used in Keplerian elements
/// Distance in kilometres; an alias of Position.
pub type Distance = Position;

// --- Angle ---

define_unit_enum!(AngleUnits, Deg, { Deg => "deg" });

/// Angle in degrees with the supported XSD interval [-360, 360).
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Angle {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngleUnits>,
}

impl std::str::FromStr for Angle {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val, None)
    }
}

impl<'de> serde::Deserialize<'de> for Angle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AngleVisitor;

        impl<'de> serde::de::Visitor<'de> for AngleVisitor {
            type Value = Angle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<Angle>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else if key == "@units" {
                        units = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Angle::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(AngleVisitor)
    }
}

impl Angle {
    /// XSD angleRange: -360.0 <= value < 360.0
    pub fn new(value: f64, units: Option<AngleUnits>) -> Result<Self> {
        Self::validate_value(value, "Angle")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(-360.0..360.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[-360, 360)".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, AngleUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for Angle {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AngleUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
// --- Angle Rate ---

define_unit_enum!(AngleRateUnits, DegPerS, { DegPerS => "deg/s" });

/// Angular rate in degrees per second.
pub type AngleRate = UnitValue<f64, AngleRateUnits>;

// --- Angular Momentum ---
define_unit_type!(AngMomentum, AngMomentumUnits, NmS, { NmS => "N*m*s" });

// --- Day Interval ---

define_unit_enum!(DayIntervalUnits, D, { D => "d" });

/// Positive day interval (XSD dayIntervalType), with optional units.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DayInterval {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<DayIntervalUnits>,
}

impl std::str::FromStr for DayInterval {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val, None)
    }
}

impl<'de> serde::Deserialize<'de> for DayInterval {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DayIntervalVisitor;

        impl<'de> serde::de::Visitor<'de> for DayIntervalVisitor {
            type Value = DayInterval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<DayInterval>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value::<f64>()?);
                    } else if key == "@units" {
                        units = Some(map.next_value::<DayIntervalUnits>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                DayInterval::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(DayIntervalVisitor)
    }
}

impl DayInterval {
    /// dayIntervalTypeUO: nonNegativeDouble
    pub fn new(value: f64, units: Option<DayIntervalUnits>) -> Result<Self> {
        Self::validate_value(value, "DayInterval")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_non_negative(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, DayIntervalUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for DayInterval {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, DayIntervalUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
impl std::fmt::Display for DayInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
/// Percentage in [0, 100], with an optional percent unit annotation.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Percentage {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<PercentageUnits>,
}

impl<'de> serde::Deserialize<'de> for Percentage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Percentage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a float value or map with $value and optionally @units")
            }

            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Percentage::new(v, None).map_err(E::custom)
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let val = v.parse::<f64>().map_err(E::custom)?;
                Percentage::new(val, None).map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value::<f64>()?);
                    } else if key == "@units" {
                        units = Some(map.next_value::<PercentageUnits>()?);
                    } else {
                        let _ = map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Percentage::new(value, units).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

impl Percentage {
    /// Construct a percentage in [0, 100]; reject values outside that domain.
    pub fn new(value: f64, units: Option<PercentageUnits>) -> Result<Self> {
        Self::validate_value(value, "Percentage")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(0.0..=100.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[0, 100]".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, PercentageUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for Percentage {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, PercentageUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
/// Positive day interval with a required day unit annotation.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DayIntervalRequired {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Required unit annotation for this quantity.
    #[serde(rename = "@units")]
    pub units: DayIntervalUnits,
}
impl DayIntervalRequired {
    /// A positive day interval (XSD `dayIntervalTypeUR`), applied in every notation.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "DayIntervalRequired")?;
        Ok(Self {
            value,
            units: DayIntervalUnits::D,
        })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_positive(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, DayIntervalUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl<'de> Deserialize<'de> for DayIntervalRequired {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct XmlValue {
            #[serde(rename = "$value")]
            value: f64,
            #[serde(rename = "@units")]
            units: DayIntervalUnits,
        }
        // Range is checked by `AtmosphericReentryParameters::validate`, naming the keyword.
        let value = XmlValue::deserialize(deserializer)?;
        Ok(Self {
            value: value.value,
            units: value.units,
        })
    }
}
impl FromKvnFloat for DayIntervalRequired {
    // Range is checked by the owning block's validator, which names the keyword.
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        Ok(Self {
            value,
            units: kvn_unit_or_default(unit, DayIntervalUnits::D)?,
        })
    }
}
impl std::fmt::Display for DayIntervalRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// --- Frequency ---

define_unit_enum!(FrequencyUnits, Hz, { Hz => "Hz" });

/// Positive frequency in hertz, with an optional unit annotation.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Frequency {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<FrequencyUnits>,
}
impl Frequency {
    /// Construct a positive frequency, following the XSD facet in every notation.
    pub fn new(value: f64, units: Option<FrequencyUnits>) -> Result<Self> {
        require_positive(value, "Frequency")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_xml_value(value: f64, name: &'static str) -> Result<()> {
        require_positive(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, FrequencyUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for Frequency {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let value = UnitValue::<f64, FrequencyUnits>::deserialize(deserializer)?;
        Self::validate_xml_value(value.value, "Frequency").map_err(serde::de::Error::custom)?;
        Ok(Self {
            value: value.value,
            units: value.units,
        })
    }
}
impl FromKvnFloat for Frequency {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, FrequencyUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
// --- Covariance Types ---

define_unit_type!(PositionCovariance, PositionCovarianceUnits, Km2, { Km2 => "km**2" });

define_unit_type!(VelocityCovariance, VelocityCovarianceUnits, Km2PerS2, { Km2PerS2 => "km**2/s**2" });

define_unit_type!(PositionVelocityCovariance, PositionVelocityCovarianceUnits, Km2PerS, { Km2PerS => "km**2/s" });

// --- GM ---

define_unit_enum!(GmUnits, Km3PerS2, { Km3PerS2 => "km**3/s**2", KM3PerS2 => "KM**3/S**2" });

/// Positive gravitational parameter in km**3/s**2.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Gm {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<GmUnits>,
}

impl std::str::FromStr for Gm {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val, None)
    }
}

impl<'de> serde::Deserialize<'de> for Gm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GmVisitor;

        impl<'de> serde::de::Visitor<'de> for GmVisitor {
            type Value = Gm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = v.parse::<Gm>().map_err(E::custom)?;
                Gm::validate_xml_value(value.value, "GM").map_err(E::custom)?;
                Ok(value)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value::<f64>()?);
                    } else if key == "@units" {
                        units = Some(map.next_value::<GmUnits>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Gm::validate_xml_value(value, "GM").map_err(serde::de::Error::custom)?;
                Gm::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(GmVisitor)
    }
}

impl Gm {
    /// Construct a finite GM; XML additionally requires a positive value.
    pub fn new(value: f64, units: Option<GmUnits>) -> Result<Self> {
        Self::validate_value(value, "GM")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_finite(name, value)
    }

    pub(crate) fn validate_xml_value(value: f64, name: &'static str) -> Result<()> {
        require_positive(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, GmUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for Gm {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, GmUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}

// --- Length ---

define_unit_type!(
    Length,
    LengthUnits,
    M,
    { M => "m" }
);

/// Altitude with a required length unit annotation; the owning field defines its range.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AltitudeRequired {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Required unit annotation for this quantity.
    #[serde(rename = "@units")]
    pub units: LengthUnits,
}
impl AltitudeRequired {
    /// The common 4.0 XSD's `altRange`, which is Earth-derived.
    ///
    /// RDM states no numeric range for `NOMINAL_IMPACT_ALT` and permits non-Earth body-fixed
    /// frames, so this is a representability limit of the XML edition rather than a semantic rule.
    /// The model keeps any finite value; XML generation rejects one this range cannot express.
    pub const XML_RANGE: std::ops::RangeInclusive<f64> = -430.5..=8848.0;

    /// Accepts any finite altitude, because the book imposes no range.
    pub fn new(value: f64) -> Result<Self> {
        require_finite("Altitude", value)?;
        Ok(Self {
            value,
            units: LengthUnits::M,
        })
    }

    /// Whether the selected XML edition can represent this altitude.
    pub fn is_xml_representable(&self) -> bool {
        Self::XML_RANGE.contains(&self.value)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, LengthUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl FromKvnFloat for AltitudeRequired {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let mut parsed = Self::new(value)?;
        parsed.units = kvn_unit_or_default(unit, LengthUnits::M)?;
        Ok(parsed)
    }
}
impl std::fmt::Display for AltitudeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- Power/Mass Ratio ---

define_required_unit_type!(Wkg, WkgUnits, WPerKg, { WPerKg => "W/kg" });

// --- Mass ---

define_unit_enum!(MassUnits, Kg, { Kg => "kg" });

/// Non-negative mass in kilograms, with optional units.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Mass {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
}

impl std::str::FromStr for Mass {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val, None)
    }
}

impl<'de> serde::Deserialize<'de> for Mass {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MassVisitor;

        impl<'de> serde::de::Visitor<'de> for MassVisitor {
            type Value = Mass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<Mass>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else if key == "@units" {
                        units = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Mass::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(MassVisitor)
    }
}

impl Mass {
    /// XSD massType: nonNegativeDouble
    pub fn new(value: f64, units: Option<MassUnits>) -> Result<Self> {
        Self::validate_value(value, "Mass")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_non_negative(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, MassUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}

impl FromKvnFloat for Mass {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, MassUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
impl std::fmt::Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

define_unit_enum!(AreaUnits, M2, { M2 => "m**2" });

/// Non-negative area in square metres, with optional units.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Area {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AreaUnits>,
}

impl std::str::FromStr for Area {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val, None)
    }
}

impl<'de> serde::Deserialize<'de> for Area {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AreaVisitor;

        impl<'de> serde::de::Visitor<'de> for AreaVisitor {
            type Value = Area;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<Area>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else if key == "@units" {
                        units = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Area::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(AreaVisitor)
    }
}

impl Area {
    /// XSD areaType: nonNegativeDouble
    pub fn new(value: f64, units: Option<AreaUnits>) -> Result<Self> {
        Self::validate_value(value, "Area")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_non_negative(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, AreaUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for Area {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AreaUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
define_required_unit_type!(Ms2, Ms2Units, MPerS2, { MPerS2 => "m/s**2" });

impl std::str::FromStr for Ms2 {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: f64 = s.parse()?;
        Ok(Self::new(v))
    }
}

define_unit_type!(Km2, Km2Units, Km2, { Km2 => "km**2" });

define_unit_type!(Km2s, Km2sUnits, Km2PerS, { Km2PerS => "km**2/s" });

define_unit_type!(Km2s2, Km2s2Units, Km2PerS2, { Km2PerS2 => "km**2/s**2" });

define_unit_type!(ManeuverFreq, NumPerYearUnits, PerYear, { PerYear => "#/yr" });

define_unit_type!(Thrust, ThrustUnits, N, { N => "N" });

define_unit_type!(Geomag, GeomagUnits, NanoTesla, { NanoTesla => "nT" });

define_unit_type!(
    SolarFlux,
    SolarFluxUnits,
    Sfu,
    {
        Sfu => "SFU",
        JanskyScaled => "10**4 Jansky",
        WPerM2Hz => "10**-22 W/(m**2/Hz)",
        ErgPerSCm2Hz => "10**-19 erg/(s*cm**2*Hz)"
    }
);

// --- Moment --- (restore)
define_unit_type!(Moment, MomentUnits, KgM2, { KgM2 => "kg*m**2" });

define_unit_type!(BallisticCoeff, BallisticCoeffUnits, KgPerM2, { KgPerM2 => "kg/m**2" });

define_unit_enum!(PercentageUnits, Percent, { Percent => "%" });

/// Percentage in [0, 100], with a required percent unit annotation.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PercentageRequired {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Required unit annotation for this quantity.
    #[serde(rename = "@units")]
    pub units: PercentageUnits,
}

impl<'de> serde::Deserialize<'de> for PercentageRequired {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PercentageRequired;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a float value or map with $value and @units")
            }

            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let _ = v;
                Err(E::custom(
                    "missing required units attribute for PercentageRequired",
                ))
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let _ = v;
                Err(E::custom(
                    "missing required units attribute for PercentageRequired",
                ))
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value::<f64>()?);
                    } else if key == "@units" {
                        units = Some(map.next_value::<PercentageUnits>()?);
                    } else {
                        let _ = map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                // Range is checked by the owning block's validator, naming the keyword.
                Ok(PercentageRequired {
                    value: value.ok_or_else(|| {
                        serde::de::Error::custom("an empty element is not a number")
                    })?,
                    units: units.ok_or_else(|| serde::de::Error::missing_field("@units"))?,
                })
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
impl PercentageRequired {
    /// Construct a percentage in [0, 100]; reject values outside that domain.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "PercentageRequired")?;
        Ok(Self {
            value,
            units: PercentageUnits::Percent,
        })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(0.0..=100.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[0, 100]".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, PercentageUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl std::fmt::Display for PercentageRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl FromKvnFloat for PercentageRequired {
    // Range is checked by the owning block's validator, which names the keyword.
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        Ok(Self {
            value,
            units: kvn_unit_or_default(unit, PercentageUnits::Percent)?,
        })
    }
}

/// Dimensionless probability in [0, 1].
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Probability {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
}

impl std::str::FromStr for Probability {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Self::new(val)
    }
}

impl<'de> serde::Deserialize<'de> for Probability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ProbabilityVisitor;

        impl<'de> serde::de::Visitor<'de> for ProbabilityVisitor {
            type Value = Probability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value")
            }

            // Range is checked by the owning block's validator, which names the keyword.
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = v.parse::<f64>().map_err(E::custom)?;
                Ok(Probability { value })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Ok(Probability { value })
            }
        }

        deserializer.deserialize_any(ProbabilityVisitor)
    }
}

impl Probability {
    /// Construct a probability in [0, 1]; reject values outside that domain.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "Probability")?;
        Ok(Self { value })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(0.0..=1.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[0, 1]".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromKvnFloat for Probability {
    // Range is checked by the owning block's validator, which names the keyword.
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        reject_kvn_unit(unit)?;
        Ok(Self { value })
    }
}

/// XSD nonNegativeDouble - value must be >= 0
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct NonNegativeDouble {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
}

impl<'de> serde::Deserialize<'de> for NonNegativeDouble {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NonNegativeDoubleVisitor;

        impl<'de> serde::de::Visitor<'de> for NonNegativeDoubleVisitor {
            type Value = NonNegativeDouble;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<NonNegativeDouble>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                NonNegativeDouble::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(NonNegativeDoubleVisitor)
    }
}

impl NonNegativeDouble {
    /// Construct a finite number at or above zero; reject values outside that domain.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "NonNegativeDouble")?;
        Ok(Self { value })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_non_negative(value, name)
    }
}

impl std::str::FromStr for NonNegativeDouble {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: f64 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
    }
}

impl std::fmt::Display for NonNegativeDouble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromKvnFloat for NonNegativeDouble {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        reject_kvn_unit(unit)?;
        Self::new(value)
    }
}

/// XSD positiveInteger - value must be > 0
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct PositiveInteger {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: u32,
}

impl<'de> serde::Deserialize<'de> for PositiveInteger {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PositiveIntegerVisitor;

        impl<'de> serde::de::Visitor<'de> for PositiveIntegerVisitor {
            type Value = PositiveInteger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value")
            }

            // Range is checked by `OdParameters::validate`, which names the keyword.
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = v.parse::<u32>().map_err(E::custom)?;
                Ok(PositiveInteger { value })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                Ok(PositiveInteger { value })
            }
        }

        deserializer.deserialize_any(PositiveIntegerVisitor)
    }
}

impl PositiveInteger {
    /// Construct an integer above zero; reject values outside that domain.
    pub fn new(value: u32) -> Result<Self> {
        Self::validate_value(value, "PositiveInteger")?;
        Ok(Self { value })
    }

    pub(crate) fn validate_value(value: u32, name: &'static str) -> Result<()> {
        if value == 0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "> 0".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

impl std::str::FromStr for PositiveInteger {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: u32 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
    }
}

/// A non-zero degree for interpolation.
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct InterpolationDegree(pub std::num::NonZeroU32);

impl std::str::FromStr for InterpolationDegree {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<u32>()?;
        let nz = std::num::NonZeroU32::new(val).ok_or_else(|| {
            crate::error::ValidationError::OutOfRange {
                name: "InterpolationDegree".into(),
                value: val.to_string(),
                expected: "> 0".into(),
                line: None,
            }
        })?;
        Ok(Self(nz))
    }
}

impl std::fmt::Display for InterpolationDegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::Deserialize<'de> for InterpolationDegree {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InterpolationDegreeVisitor;

        impl<'de> serde::de::Visitor<'de> for InterpolationDegreeVisitor {
            type Value = InterpolationDegree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<InterpolationDegree>().map_err(E::custom)
            }

            fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                std::num::NonZeroU32::new(v)
                    .map(InterpolationDegree)
                    .ok_or_else(|| E::custom("expected non-zero u32"))
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value::<u32>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                std::num::NonZeroU32::new(value)
                    .map(InterpolationDegree)
                    .ok_or_else(|| serde::de::Error::custom("expected non-zero u32"))
            }
        }

        deserializer.deserialize_any(InterpolationDegreeVisitor)
    }
}

impl From<std::num::NonZeroU32> for InterpolationDegree {
    fn from(val: std::num::NonZeroU32) -> Self {
        Self(val)
    }
}

impl From<InterpolationDegree> for std::num::NonZeroU32 {
    fn from(val: InterpolationDegree) -> Self {
        val.0
    }
}

impl std::fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for PositiveInteger {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

/// XSD elementSetNoType - value must be between 0 and 9999
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct ElementSetNo {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: u32,
}

impl std::str::FromStr for ElementSetNo {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: u32 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
    }
}

impl<'de> serde::Deserialize<'de> for ElementSetNo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElementSetNoVisitor;

        impl<'de> serde::de::Visitor<'de> for ElementSetNoVisitor {
            type Value = ElementSetNo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<ElementSetNo>().map_err(E::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        value = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value
                    .ok_or_else(|| serde::de::Error::custom("an empty element is not a number"))?;
                ElementSetNo::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(ElementSetNoVisitor)
    }
}

impl ElementSetNo {
    /// Construct a four-digit element-set number; reject values outside that domain.
    pub fn new(value: u32) -> Result<Self> {
        Self::validate_value(value, "ElementSetNo")
    }

    /// `elementSetNoType` restricts to `[0, 9999]` in both the OMM 2.0 and 3.0 schemas.
    pub(crate) fn validate_value(value: u32, name: &'static str) -> Result<Self> {
        if value > 9999 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[0, 9999]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value })
    }
}

impl std::fmt::Display for ElementSetNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for ElementSetNo {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

// Delta mass types (negative or non-positive)
/// Mass change strictly below zero, in kilograms.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeltaMass {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
}
impl DeltaMass {
    /// Construct a mass change, rejecting non-negative values.
    pub fn new(value: f64, units: Option<MassUnits>) -> Result<Self> {
        if value >= 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "DeltaMass".into(),
                value: value.to_string(),
                expected: "< 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
}

impl FromKvnFloat for DeltaMass {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, MassUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}

/// Mass change at or below zero, in kilograms.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeltaMassZ {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
}
impl DeltaMassZ {
    /// Construct a mass change, rejecting positive values.
    pub fn new(value: f64, units: Option<MassUnits>) -> Result<Self> {
        if value > 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "DeltaMassZ".into(),
                value: value.to_string(),
                expected: "<= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }

    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, MassUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}

impl FromKvnFloat for DeltaMassZ {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, MassUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}

// Quaternion dot component units (1/s)
define_unit_type!(QuaternionDotComponent, QuaternionDotUnits, PerS, { PerS => "1/s" });

// Latitude / Longitude / Altitude
define_unit_enum!(LatLonUnits, Deg, { Deg => "deg" });
/// Latitude with optional degree units; the owning field enforces its range.
pub type Latitude = UnitValue<f64, LatLonUnits>;
/// Longitude with optional degree units; the owning field enforces its range.
pub type Longitude = UnitValue<f64, LatLonUnits>;
/// Altitude with optional length units.
pub type Altitude = UnitValue<f64, LengthUnits>;

/// Latitude in [-90, 90] degrees, with a required unit annotation.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LatitudeRequired {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Required unit annotation for this quantity.
    #[serde(rename = "@units")]
    pub units: LatLonUnits,
}
impl LatitudeRequired {
    /// Construct a latitude in [-90, 90] degrees; reject values outside that domain.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "Latitude")?;
        Ok(Self {
            value,
            units: LatLonUnits::Deg,
        })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(-90.0..=90.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[-90, 90]".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, LatLonUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl std::fmt::Display for LatitudeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for LatitudeRequired {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: f64 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
    }
}

impl FromKvnFloat for LatitudeRequired {
    // Range is checked by the owning block's validator, which names the keyword.
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        Ok(Self {
            value,
            units: kvn_unit_or_default(unit, LatLonUnits::Deg)?,
        })
    }
}

/// Longitude in [-180, 180] degrees, with a required unit annotation.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LongitudeRequired {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64,
    /// Required unit annotation for this quantity.
    #[serde(rename = "@units")]
    pub units: LatLonUnits,
}
impl LongitudeRequired {
    /// Construct a longitude in [-180, 180] degrees; reject values outside that domain.
    pub fn new(value: f64) -> Result<Self> {
        Self::validate_value(value, "Longitude")?;
        Ok(Self {
            value,
            units: LatLonUnits::Deg,
        })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        if !(-180.0..=180.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: name.into(),
                value: value.to_string(),
                expected: "[-180, 180]".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }

    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, LatLonUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl std::fmt::Display for LongitudeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for LongitudeRequired {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: f64 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
    }
}

impl FromKvnFloat for LongitudeRequired {
    // Range is checked by the owning block's validator, which names the keyword.
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        Ok(Self {
            value,
            units: kvn_unit_or_default(unit, LatLonUnits::Deg)?,
        })
    }
}

// Torque
define_unit_type!(Torque, TorqueUnits, Nm, { Nm => "N*m" });

// Vector helper for cpType / targetMomentumType
/// Three-component length vector, used for a center-of-pressure position.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Vector3 {
    /// Three vector components in their declared unit and reference frame.
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub elements: Vec<f64>, // Expect length 3
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<LengthUnits>,
}
impl Vector3 {
    /// Construct a three-component position vector with the supplied unit annotation.
    pub fn new(elements: [f64; 3], units: Option<LengthUnits>) -> Self {
        Self {
            elements: elements.to_vec(),
            units,
        }
    }
}

// Target momentum vector (uses angular momentum units)
/// Three-component target angular momentum in N*m*s.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TargetMomentum {
    /// Three vector components in their declared unit and reference frame.
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub elements: Vec<f64>, // length 3
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngMomentumUnits>,
}
impl TargetMomentum {
    /// Construct a three-component angular momentum vector with the supplied units.
    pub fn new(elements: [f64; 3], units: Option<AngMomentumUnits>) -> Self {
        Self {
            elements: elements.to_vec(),
            units,
        }
    }
}

/// Closed KVN enum aliases permit upper- or lowercase tokens, never arbitrary mixed case.
fn enum_token(value: &str) -> std::borrow::Cow<'_, str> {
    if value.bytes().any(|byte| byte.is_ascii_lowercase())
        && !value.bytes().any(|byte| byte.is_ascii_uppercase())
    {
        std::borrow::Cow::Owned(value.to_ascii_uppercase())
    } else {
        std::borrow::Cow::Borrowed(value)
    }
}

// Categorical Enums
//----------------------------------------------------------------------

/// Object classification tokens shared by conjunction and orbit messages.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ObjectDescription {
    /// Operational payload.
    #[serde(rename = "PAYLOAD", alias = "payload")]
    Payload,
    /// Rocket body or upper stage.
    #[serde(rename = "ROCKET BODY", alias = "rocket body")]
    RocketBody,
    /// Orbital debris.
    #[serde(rename = "DEBRIS", alias = "debris")]
    Debris,
    /// The classification or status is unknown.
    #[serde(rename = "UNKNOWN", alias = "unknown")]
    Unknown,
    /// Other classification or method specified by the exchange partners.
    #[serde(rename = "OTHER", alias = "other")]
    Other,
}

impl std::str::FromStr for ObjectDescription {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "PAYLOAD" => Ok(Self::Payload),
            // CDM 6.3.2.5, RDM 5.3.3.4, ODM 7.5.9: an underscore in a text value equals a blank.
            "ROCKET BODY" | "ROCKET_BODY" => Ok(Self::RocketBody),
            "DEBRIS" => Ok(Self::Debris),
            "UNKNOWN" => Ok(Self::Unknown),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::EnumParseError {
                field: "OBJECT_TYPE",
                value: s.to_string(),
                expected: "PAYLOAD, ROCKET BODY, DEBRIS, UNKNOWN, or OTHER",
            }),
        }
    }
}
impl std::fmt::Display for ObjectDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ObjectDescription::Payload => "PAYLOAD",
            ObjectDescription::RocketBody => "ROCKET BODY",
            ObjectDescription::Debris => "DEBRIS",
            ObjectDescription::Unknown => "UNKNOWN",
            ObjectDescription::Other => "OTHER",
        };
        write!(f, "{}", s)
    }
}

/// Euler rotation sequence (ADM table 3-3 `EULER_ROT_SEQ`).
///
/// XML (serde) accepts exactly the twelve uppercase XSD `rotseqType` tokens. `FromStr`, used by
/// KVN and Python, also accepts lowercase per ADM 6.8.6 and the ADM 1.0 numeric spellings
/// (`313` for `ZXZ`, annex J) as a KVN-only legacy alias; every writer emits the uppercase letters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum RotSeq {
    /// Successive rotations about the X, Y, X axes (ADM §6.8.6).
    #[serde(rename = "XYX")]
    XYX,
    /// Successive rotations about the X, Y, Z axes (ADM §6.8.6).
    #[serde(rename = "XYZ")]
    XYZ,
    /// Successive rotations about the X, Z, X axes (ADM §6.8.6).
    #[serde(rename = "XZX")]
    XZX,
    /// Successive rotations about the X, Z, Y axes (ADM §6.8.6).
    #[serde(rename = "XZY")]
    XZY,
    /// Successive rotations about the Y, X, Y axes (ADM §6.8.6).
    #[serde(rename = "YXY")]
    YXY,
    /// Successive rotations about the Y, X, Z axes (ADM §6.8.6).
    #[serde(rename = "YXZ")]
    YXZ,
    /// Successive rotations about the Y, Z, X axes (ADM §6.8.6).
    #[serde(rename = "YZX")]
    YZX,
    /// Successive rotations about the Y, Z, Y axes (ADM §6.8.6).
    #[serde(rename = "YZY")]
    YZY,
    /// Successive rotations about the Z, X, Y axes (ADM §6.8.6).
    #[serde(rename = "ZXY")]
    ZXY,
    /// Successive rotations about the Z, X, Z axes (ADM §6.8.6).
    #[serde(rename = "ZXZ")]
    ZXZ,
    /// Successive rotations about the Z, Y, X axes (ADM §6.8.6).
    #[serde(rename = "ZYX")]
    ZYX,
    /// Successive rotations about the Z, Y, Z axes (ADM §6.8.6).
    #[serde(rename = "ZYZ")]
    ZYZ,
}

impl std::str::FromStr for RotSeq {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "XYX" => Ok(Self::XYX),
            "XYZ" => Ok(Self::XYZ),
            "XZX" => Ok(Self::XZX),
            "XZY" => Ok(Self::XZY),
            "YXY" => Ok(Self::YXY),
            "YXZ" => Ok(Self::YXZ),
            "YZX" => Ok(Self::YZX),
            "YZY" => Ok(Self::YZY),
            "ZXY" => Ok(Self::ZXY),
            "ZXZ" => Ok(Self::ZXZ),
            "ZYX" => Ok(Self::ZYX),
            "ZYZ" => Ok(Self::ZYZ),
            "121" => Ok(Self::XYX),
            "123" => Ok(Self::XYZ),
            "131" => Ok(Self::XZX),
            "132" => Ok(Self::XZY),
            "212" => Ok(Self::YXY),
            "213" => Ok(Self::YXZ),
            "231" => Ok(Self::YZX),
            "232" => Ok(Self::YZY),
            "312" => Ok(Self::ZXY),
            "313" => Ok(Self::ZXZ),
            "321" => Ok(Self::ZYX),
            "323" => Ok(Self::ZYZ),
            _ => Err(crate::error::EnumParseError {
                field: "EULER_ROT_SEQ",
                value: s.to_string(),
                expected: "XYX, XYZ, XZX, XZY, YXY, YXZ, YZX, YZY, ZXY, ZXZ, ZYX, ZYZ (either case), or ADM 1.0 numeric equivalents",
            }),
        }
    }
}

impl std::fmt::Display for RotSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XYX => write!(f, "XYX"),
            Self::XYZ => write!(f, "XYZ"),
            Self::XZX => write!(f, "XZX"),
            Self::XZY => write!(f, "XZY"),
            Self::YXY => write!(f, "YXY"),
            Self::YXZ => write!(f, "YXZ"),
            Self::YZX => write!(f, "YZX"),
            Self::YZY => write!(f, "YZY"),
            Self::ZXY => write!(f, "ZXY"),
            Self::ZXZ => write!(f, "ZXZ"),
            Self::ZYX => write!(f, "ZYX"),
            Self::ZYZ => write!(f, "ZYZ"),
        }
    }
}

/// Attitude determination method (ADM table 5-8).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AdMethod {
    /// Extended Kalman filter.
    #[serde(rename = "EKF", alias = "ekf")]
    Ekf,
    /// TRIAD attitude determination.
    #[serde(rename = "TRIAD", alias = "triad")]
    Triad,
    /// QUEST attitude determination.
    #[serde(rename = "QUEST", alias = "quest")]
    Quest,
    /// Batch attitude estimation.
    #[serde(rename = "BATCH", alias = "batch")]
    Batch,
    /// Quaternion q-method.
    #[serde(rename = "Q_METHOD", alias = "q_method")]
    QMethod,
    /// Combined filtering and smoothing.
    #[serde(rename = "FILTER_SMOOTHER", alias = "filter_smoother")]
    FilterSmoother,
    /// Other classification or method specified by the exchange partners.
    #[serde(rename = "OTHER", alias = "other")]
    Other,
}

impl std::str::FromStr for AdMethod {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "EKF" => Ok(Self::Ekf),
            "TRIAD" => Ok(Self::Triad),
            "QUEST" => Ok(Self::Quest),
            "BATCH" => Ok(Self::Batch),
            "Q_METHOD" => Ok(Self::QMethod),
            "FILTER_SMOOTHER" => Ok(Self::FilterSmoother),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::EnumParseError {
                field: "AD_METHOD",
                value: s.to_owned(),
                expected: "EKF, TRIAD, QUEST, BATCH, Q_METHOD, FILTER_SMOOTHER, or OTHER",
            }),
        }
    }
}

/// CCSDS boolean vocabulary expressed as YES or NO.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum YesNo {
    /// Affirmative status.
    #[serde(rename = "YES", alias = "yes")]
    Yes,
    /// Negative status.
    #[serde(rename = "NO", alias = "no")]
    No,
}
impl std::fmt::Display for YesNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            YesNo::Yes => "YES",
            YesNo::No => "NO",
        };
        write!(f, "{}", s)
    }
}
impl std::str::FromStr for YesNo {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "YES" | "yes" => Ok(YesNo::Yes),
            "NO" | "no" => Ok(YesNo::No),
            _ => Err(crate::error::EnumParseError {
                field: "YES/NO",
                value: s.to_string(),
                expected: "YES or NO",
            }),
        }
    }
}

/// Whether orbit revolution numbering begins at zero or one.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum RevNumBasis {
    /// The first orbit revolution is numbered zero.
    #[serde(rename = "0")]
    Zero,
    /// The first orbit revolution is numbered one.
    #[serde(rename = "1")]
    One,
}

impl<'de> Deserialize<'de> for RevNumBasis {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        crate::utils::deserialize_parsed(deserializer, "0 or 1", |value| {
            value.trim_matches([' ', '\t', '\r', '\n']).parse::<Self>()
        })
    }
}

impl std::fmt::Display for RevNumBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
        }
    }
}

impl std::str::FromStr for RevNumBasis {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(0) => Ok(Self::Zero),
            Ok(1) => Ok(Self::One),
            _ => Err(crate::error::EnumParseError {
                field: "ORB_REVNUM_BASIS",
                value: s.to_string(),
                expected: "0 or 1",
            }),
        }
    }
}

/// Basis of the maneuver time history data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ManBasis {
    /// Basis of this maneuver time history data is 'CANDIDATE' for a proposed operational or a
    /// hypothetical (i.e., mission design and optimization studies) future maneuver.
    #[serde(rename = "CANDIDATE")]
    Candidate,
    /// Basis of this maneuver time history data is 'PLANNED' for a currently planned future
    /// maneuver.
    #[serde(rename = "PLANNED")]
    Planned,
    /// Basis of this maneuver time history data is 'ANTICIPATED' for a non-cooperative future
    /// maneuver that is anticipated (i.e., likely) to occur (e.g., based upon patterns-of-life
    /// analysis).
    #[serde(rename = "ANTICIPATED")]
    Anticipated,
    /// Basis of this maneuver time history data is 'TELEMETRY' when the maneuver is determined
    /// directly from telemetry (e.g., based on inertial navigation systems or
    /// accelerometers).
    #[serde(rename = "TELEMETRY")]
    Telemetry,
    /// Basis of this maneuver time history data is 'DETERMINED' when a past maneuver is
    /// estimated from observation-based orbit determination reconstruction and/or
    /// calibration.
    #[serde(rename = "DETERMINED")]
    Determined,
    /// Basis of this maneuver time history data is 'SIMULATED' for generic maneuver
    /// simulations, future mission design studies, and optimization studies.
    #[serde(rename = "SIMULATED")]
    Simulated,
    /// Basis of this maneuver time history data is 'OTHER' for other bases of this data.
    #[serde(rename = "OTHER")]
    Other,
}

impl std::fmt::Display for ManBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => write!(f, "CANDIDATE"),
            Self::Planned => write!(f, "PLANNED"),
            Self::Anticipated => write!(f, "ANTICIPATED"),
            Self::Telemetry => write!(f, "TELEMETRY"),
            Self::Determined => write!(f, "DETERMINED"),
            Self::Simulated => write!(f, "SIMULATED"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}

impl std::str::FromStr for ManBasis {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "CANDIDATE" => Ok(Self::Candidate),
            "PLANNED" => Ok(Self::Planned),
            "ANTICIPATED" => Ok(Self::Anticipated),
            "TELEMETRY" => Ok(Self::Telemetry),
            "DETERMINED" => Ok(Self::Determined),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::EnumParseError {
                field: "MAN_BASIS",
                value: s.to_string(),
                expected:
                    "CANDIDATE, PLANNED, ANTICIPATED, TELEMETRY, DETERMINED, SIMULATED, or OTHER",
            }),
        }
    }
}

/// Maneuver duty cycle type per XSD dcTypeType.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum ManDc {
    /// Duty cycle type 'CONTINUOUS' denotes full/continuous thrust.
    #[default]
    #[serde(rename = "CONTINUOUS")]
    Continuous,
    /// Duty cycle type 'TIME' denotes a time-based duty cycle driven by time past a reference
    /// time and the duty cycle ON and OFF durations.
    #[serde(rename = "TIME")]
    Time,
    /// Duty cycle type 'TIME_AND_ANGLE' denotes a duty cycle driven by the phasing/clocking of
    /// a space object body frame 'trigger' direction past a reference direction.
    #[serde(rename = "TIME_AND_ANGLE")]
    TimeAndAngle,
}

impl std::fmt::Display for ManDc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Continuous => write!(f, "CONTINUOUS"),
            Self::Time => write!(f, "TIME"),
            Self::TimeAndAngle => write!(f, "TIME_AND_ANGLE"),
        }
    }
}

impl std::str::FromStr for ManDc {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "CONTINUOUS" => Ok(Self::Continuous),
            "TIME" => Ok(Self::Time),
            "TIME_AND_ANGLE" => Ok(Self::TimeAndAngle),
            _ => Err(crate::error::EnumParseError {
                field: "DC_TYPE",
                value: s.to_string(),
                expected: "CONTINUOUS, TIME, or TIME_AND_ANGLE",
            }),
        }
    }
}

/// Covariance ordering.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum CovOrder {
    /// Covariance ordering is Lower Triangular Matrix (LTM).
    #[default]
    #[serde(rename = "LTM")]
    Ltm,
    /// Covariance ordering is Upper Triangular Matrix (UTM).
    #[serde(rename = "UTM")]
    Utm,
    /// Covariance ordering is Full covariance matrix.
    #[serde(rename = "FULL")]
    Full,
    /// Covariance ordering is LTM covariance with cross-correlation information provided in
    /// upper triangle off-diagonal terms (LTMWCC).
    #[serde(rename = "LTMWCC")]
    LtmWcc,
    /// Covariance ordering is UTM covariance with cross-correlation information provided in
    /// lower triangle off-diagonal terms (UTMWCC).
    #[serde(rename = "UTMWCC")]
    UtmWcc,
}

impl std::fmt::Display for CovOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ltm => write!(f, "LTM"),
            Self::Utm => write!(f, "UTM"),
            Self::Full => write!(f, "FULL"),
            Self::LtmWcc => write!(f, "LTMWCC"),
            Self::UtmWcc => write!(f, "UTMWCC"),
        }
    }
}

impl std::str::FromStr for CovOrder {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "LTM" => Ok(Self::Ltm),
            "UTM" => Ok(Self::Utm),
            "FULL" => Ok(Self::Full),
            "LTMWCC" => Ok(Self::LtmWcc),
            "UTMWCC" => Ok(Self::UtmWcc),
            _ => Err(crate::error::EnumParseError {
                field: "COV_ORDERING",
                value: s.to_string(),
                expected: "LTM, UTM, FULL, LTMWCC, or UTMWCC",
            }),
        }
    }
}

/// Whether a reentry is controlled (RDM table 3-3).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ControlledType {
    /// Affirmative status.
    #[serde(rename = "YES", alias = "yes")]
    Yes,
    /// Negative status.
    #[serde(rename = "NO", alias = "no")]
    No,
    /// The classification or status is unknown.
    #[serde(rename = "UNKNOWN", alias = "unknown")]
    Unknown,
}
impl std::fmt::Display for ControlledType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ControlledType::Yes => "YES",
            ControlledType::No => "NO",
            ControlledType::Unknown => "UNKNOWN",
        };
        write!(f, "{}", s)
    }
}
impl std::str::FromStr for ControlledType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "YES" | "yes" => Ok(ControlledType::Yes),
            "NO" | "no" => Ok(ControlledType::No),
            "UNKNOWN" | "unknown" => Ok(ControlledType::Unknown),
            _ => Err(crate::error::EnumParseError {
                field: "CONTROLLED_TYPE",
                value: s.to_string(),
                expected: "YES, NO, or UNKNOWN",
            }),
        }
    }
}

// Time units ("s") plus Duration / RelTime / TimeOffset (optional units per XSD)
define_unit_enum!(TimeUnits, Seconds, { Seconds => "s", Day => "d" });

impl TimeUnits {
    pub(crate) fn validate_seconds(units: &Option<Self>, field: &'static str) -> Result<()> {
        if matches!(units, Some(Self::Day)) {
            return Err(crate::error::ValidationError::InvalidValue {
                field: format!("{field} units").into(),
                value: "d".into(),
                expected: "s".into(),
                line: None,
            }
            .into());
        }
        Ok(())
    }
}

/// Non-negative duration with an optional time unit annotation.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Duration {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64, // nonNegativeDouble
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
}
impl Duration {
    /// Construct a non-negative duration; reject values outside that domain.
    pub fn new(value: f64, units: Option<TimeUnits>) -> Result<Self> {
        Self::validate_value(value, "Duration")?;
        Ok(Self { value, units })
    }

    pub(crate) fn validate_value(value: f64, name: &'static str) -> Result<()> {
        require_non_negative(value, name)
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, TimeUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}
impl FromKvnFloat for Duration {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, TimeUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
/// Signed relative time, interpreted using the containing field and time system.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RelTime {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value")]
    pub value: f64, // double (can be negative)
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
}

/// Signed time offset with an optional time unit annotation.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TimeOffset {
    /// Numeric value; the containing type defines its unit and permitted domain.
    #[serde(rename = "$value", serialize_with = "serialize_xml_value")]
    pub value: f64, // double
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
}

impl std::str::FromStr for TimeOffset {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse::<f64>()?;
        Ok(Self {
            value: val,
            units: None,
        })
    }
}

impl<'de> serde::Deserialize<'de> for TimeOffset {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let value = UnitValue::<f64, TimeUnits>::deserialize(deserializer)?;
        Ok(Self {
            value: value.value,
            units: value.units,
        })
    }
}

impl FromKvnFloat for TimeOffset {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, TimeUnits>::from_kvn_float(value, unit)?;
        Ok(TimeOffset {
            value: uv.value,
            units: uv.units,
        })
    }
}
impl TimeOffset {
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, TimeUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}

// Inclination (0 ..= 180 deg)
/// Orbital inclination in [0, 180] degrees.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct Inclination {
    /// Inclination angle with its optional degree annotation.
    pub angle: Angle, // uses AngleUnits (deg)
}
impl Inclination {
    /// Construct an inclination in [0, 180] degrees; reject values outside that domain.
    pub fn new(value: f64, units: Option<AngleUnits>) -> Result<Self> {
        if !(0.0..=180.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Inclination".into(),
                value: value.to_string(),
                expected: "[0, 180]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            angle: Angle { value, units },
        })
    }
    /// Copy this quantity and its unit annotation into the shared measurement wrapper.
    pub fn to_unit_value(&self) -> UnitValue<f64, AngleUnits> {
        UnitValue {
            value: self.angle.value,
            units: self.angle.units.clone(),
        }
    }
}
impl FromKvnFloat for Inclination {
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AngleUnits>::from_kvn_float(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}

// Attitude related enums (acmAttitudeType, attRateType, attBasisType, acmCovarianceLineType, attitudeTypeType)
/// ACM attitude-state representation (ADM table 5-4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AcmAttitudeType {
    /// Quaternion components.
    #[serde(rename = "QUATERNION", alias = "quaternion")]
    Quaternion,
    /// Euler-angle components.
    #[serde(rename = "EULER_ANGLES", alias = "euler_angles")]
    EulerAngles,
    /// Direction cosine matrix components.
    #[serde(rename = "DCM", alias = "dcm")]
    Dcm,
    /// Angular-velocity components.
    #[serde(rename = "ANGVEL", alias = "angvel")]
    AngVel,
    /// Quaternion time derivatives.
    #[serde(rename = "Q_DOT", alias = "q_dot")]
    QDot,
    /// Euler-angle time derivatives.
    #[serde(rename = "EULER_RATE", alias = "euler_rate")]
    EulerRate,
    /// Gyroscope bias components.
    #[serde(rename = "GYRO_BIAS", alias = "gyro_bias")]
    GyroBias,
}

impl std::str::FromStr for AcmAttitudeType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "QUATERNION" | "quaternion" => Ok(Self::Quaternion),
            "EULER_ANGLES" | "euler_angles" => Ok(Self::EulerAngles),
            "DCM" | "dcm" => Ok(Self::Dcm),
            "ANGVEL" | "angvel" => Ok(Self::AngVel),
            "Q_DOT" | "q_dot" => Ok(Self::QDot),
            "EULER_RATE" | "euler_rate" => Ok(Self::EulerRate),
            "GYRO_BIAS" | "gyro_bias" => Ok(Self::GyroBias),
            _ => Err(crate::error::EnumParseError {
                field: "ATT_TYPE",
                value: s.to_string(),
                expected: "QUATERNION, EULER_ANGLES, DCM, ANGVEL, Q_DOT, EULER_RATE, or GYRO_BIAS",
            }),
        }
    }
}

impl std::fmt::Display for AcmAttitudeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Quaternion => "QUATERNION",
            Self::EulerAngles => "EULER_ANGLES",
            Self::Dcm => "DCM",
            Self::AngVel => "ANGVEL",
            Self::QDot => "Q_DOT",
            Self::EulerRate => "EULER_RATE",
            Self::GyroBias => "GYRO_BIAS",
        };
        write!(f, "{}", value)
    }
}

/// ACM attitude-rate representation (ADM table 5-4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttRateType {
    /// Angular-velocity components.
    #[serde(rename = "ANGVEL", alias = "angvel")]
    AngVel,
    /// Quaternion time derivatives.
    #[serde(rename = "Q_DOT", alias = "q_dot")]
    QDot,
    /// Euler-angle time derivatives.
    #[serde(rename = "EULER_RATE", alias = "euler_rate")]
    EulerRate,
    /// Gyroscope bias components.
    #[serde(rename = "GYRO_BIAS", alias = "gyro_bias")]
    GyroBias,
}

impl std::str::FromStr for AttRateType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ANGVEL" | "angvel" => Ok(Self::AngVel),
            "Q_DOT" | "q_dot" => Ok(Self::QDot),
            "EULER_RATE" | "euler_rate" => Ok(Self::EulerRate),
            "GYRO_BIAS" | "gyro_bias" => Ok(Self::GyroBias),
            _ => Err(crate::error::EnumParseError {
                field: "RATE_TYPE",
                value: s.to_string(),
                expected: "ANGVEL, Q_DOT, EULER_RATE, or GYRO_BIAS",
            }),
        }
    }
}

impl std::fmt::Display for AttRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::AngVel => "ANGVEL",
            Self::QDot => "Q_DOT",
            Self::EulerRate => "EULER_RATE",
            Self::GyroBias => "GYRO_BIAS",
        };
        write!(f, "{}", value)
    }
}

/// Provenance of an attitude history (ADM table 5-4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttBasisType {
    /// Predicted attitude.
    #[serde(rename = "PREDICTED", alias = "predicted")]
    Predicted,
    /// Attitude determined on the ground.
    #[serde(rename = "DETERMINED_GND", alias = "determined_gnd")]
    DeterminedGnd,
    /// Attitude determined by the onboard computer.
    #[serde(rename = "DETERMINED_OBC", alias = "determined_obc")]
    DeterminedObc,
    /// Simulated attitude.
    #[serde(rename = "SIMULATED", alias = "simulated")]
    Simulated,
}

impl std::str::FromStr for AttBasisType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PREDICTED" | "predicted" => Ok(Self::Predicted),
            "DETERMINED_GND" | "determined_gnd" => Ok(Self::DeterminedGnd),
            "DETERMINED_OBC" | "determined_obc" => Ok(Self::DeterminedObc),
            "SIMULATED" | "simulated" => Ok(Self::Simulated),
            _ => Err(crate::error::EnumParseError {
                field: "ATT_BASIS",
                value: s.to_string(),
                expected: "PREDICTED, DETERMINED_GND, DETERMINED_OBC, or SIMULATED",
            }),
        }
    }
}

impl std::fmt::Display for AttBasisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Predicted => "PREDICTED",
            Self::DeterminedGnd => "DETERMINED_GND",
            Self::DeterminedObc => "DETERMINED_OBC",
            Self::Simulated => "SIMULATED",
        };
        write!(f, "{}", value)
    }
}

/// ACM covariance composition (ADM table 5-6).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AcmCovarianceLineType {
    /// Attitude-angle covariance.
    #[serde(rename = "ANGLE", alias = "angle")]
    Angle,
    /// Combined attitude-angle and gyroscope-bias covariance.
    #[serde(rename = "ANGLE_GYROBIAS", alias = "angle_gyrobias")]
    AngleGyroBias,
    /// Combined attitude-angle and angular-velocity covariance.
    #[serde(rename = "ANGLE_ANGVEL", alias = "angle_angvel")]
    AngleAngVel,
    /// Quaternion components.
    #[serde(rename = "QUATERNION", alias = "quaternion")]
    Quaternion,
    /// Combined quaternion and gyroscope-bias covariance.
    #[serde(rename = "QUATERNION_GYROBIAS", alias = "quaternion_gyrobias")]
    QuaternionGyroBias,
    /// Quaternion components with angular-velocity components.
    #[serde(rename = "QUATERNION_ANGVEL", alias = "quaternion_angvel")]
    QuaternionAngVel,
}

impl std::str::FromStr for AcmCovarianceLineType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ANGLE" | "angle" => Ok(Self::Angle),
            "ANGLE_GYROBIAS" | "angle_gyrobias" => Ok(Self::AngleGyroBias),
            "ANGLE_ANGVEL" | "angle_angvel" => Ok(Self::AngleAngVel),
            "QUATERNION" | "quaternion" => Ok(Self::Quaternion),
            "QUATERNION_GYROBIAS" | "quaternion_gyrobias" => Ok(Self::QuaternionGyroBias),
            "QUATERNION_ANGVEL" | "quaternion_angvel" => Ok(Self::QuaternionAngVel),
            _ => Err(crate::error::EnumParseError {
                field: "COV_TYPE",
                value: s.to_string(),
                expected: "ANGLE, ANGLE_GYROBIAS, ANGLE_ANGVEL, QUATERNION, QUATERNION_GYROBIAS, or QUATERNION_ANGVEL",
            }),
        }
    }
}

impl std::fmt::Display for AcmCovarianceLineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Angle => "ANGLE",
            Self::AngleGyroBias => "ANGLE_GYROBIAS",
            Self::AngleAngVel => "ANGLE_ANGVEL",
            Self::Quaternion => "QUATERNION",
            Self::QuaternionGyroBias => "QUATERNION_GYROBIAS",
            Self::QuaternionAngVel => "QUATERNION_ANGVEL",
        };
        write!(f, "{}", value)
    }
}

/// AEM attitude-state representation and record layout (ADM table 4-4).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttitudeTypeType {
    /// Quaternion components.
    #[serde(rename = "QUATERNION", alias = "quaternion")]
    Quaternion,
    /// Quaternion components and their time derivatives.
    #[serde(rename = "QUATERNION/DERIVATIVE", alias = "quaternion/derivative")]
    QuaternionDerivative,
    /// Quaternion components with angular-velocity components.
    #[serde(rename = "QUATERNION/ANGVEL", alias = "quaternion/angvel")]
    QuaternionAngVel,
    /// Euler-angle components.
    #[serde(rename = "EULER_ANGLE", alias = "euler_angle")]
    EulerAngle,
    /// Euler angles and their time derivatives.
    #[serde(rename = "EULER_ANGLE/DERIVATIVE", alias = "euler_angle/derivative")]
    EulerAngleDerivative,
    /// Euler angles with angular-velocity components.
    #[serde(rename = "EULER_ANGLE/ANGVEL", alias = "euler_angle/angvel")]
    EulerAngleAngVel,
    /// Spin-axis direction, phase and rate.
    #[serde(rename = "SPIN", alias = "spin")]
    Spin,
    /// Spin state with a nutation description.
    #[serde(rename = "SPIN/NUTATION", alias = "spin/nutation")]
    SpinNutation,
    /// Spin state with angular-momentum and nutation-rate information.
    #[serde(rename = "SPIN/NUTATION_MOM", alias = "spin/nutation_mom")]
    SpinNutationMom,
}

impl AttitudeTypeType {
    /// Number of numeric components per history record, excluding its epoch (ADM table 4-4).
    pub const fn value_count(&self) -> usize {
        match self {
            Self::Quaternion | Self::Spin => 4,
            Self::QuaternionDerivative => 8,
            Self::QuaternionAngVel | Self::SpinNutation | Self::SpinNutationMom => 7,
            Self::EulerAngle => 3,
            Self::EulerAngleDerivative | Self::EulerAngleAngVel => 6,
        }
    }
}

impl std::str::FromStr for AttitudeTypeType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "quaternion" | "QUATERNION" => Ok(Self::Quaternion),
            "quaternion/derivative" | "QUATERNION/DERIVATIVE" => {
                Ok(Self::QuaternionDerivative)
            }
            "quaternion/angvel" | "QUATERNION/ANGVEL" => Ok(Self::QuaternionAngVel),
            "euler_angle" | "EULER_ANGLE" => Ok(Self::EulerAngle),
            "euler_angle/derivative" | "EULER_ANGLE/DERIVATIVE" => {
                Ok(Self::EulerAngleDerivative)
            }
            "euler_angle/angvel" | "EULER_ANGLE/ANGVEL" => Ok(Self::EulerAngleAngVel),
            "spin" | "SPIN" => Ok(Self::Spin),
            "spin/nutation" | "SPIN/NUTATION" => Ok(Self::SpinNutation),
            "spin/nutation_mom" | "SPIN/NUTATION_MOM" => Ok(Self::SpinNutationMom),
            _ => Err(crate::error::EnumParseError {
                field: "ATTITUDE_TYPE",
                value: s.to_string(),
                expected: "QUATERNION, QUATERNION/DERIVATIVE, QUATERNION/ANGVEL, EULER_ANGLE, EULER_ANGLE/DERIVATIVE, EULER_ANGLE/ANGVEL, SPIN, SPIN/NUTATION, or SPIN/NUTATION_MOM",
            }),
        }
    }
}

impl std::fmt::Display for AttitudeTypeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Quaternion => "QUATERNION",
            Self::QuaternionDerivative => "QUATERNION/DERIVATIVE",
            Self::QuaternionAngVel => "QUATERNION/ANGVEL",
            Self::EulerAngle => "EULER_ANGLE",
            Self::EulerAngleDerivative => "EULER_ANGLE/DERIVATIVE",
            Self::EulerAngleAngVel => "EULER_ANGLE/ANGVEL",
            Self::Spin => "SPIN",
            Self::SpinNutation => "SPIN/NUTATION",
            Self::SpinNutationMom => "SPIN/NUTATION_MOM",
        };
        write!(f, "{}", value)
    }
}

// APM rate frame
/// Euler frame used for attitude rate components (ADM table 3-3).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ApmRateFrame {
    /// Rates expressed in the first Euler frame.
    #[serde(rename = "EULER_FRAME_A")]
    EulerFrameA,
    /// Rates expressed in the second Euler frame.
    #[serde(rename = "EULER_FRAME_B")]
    EulerFrameB,
}

// SigmaU / SigmaV units and types
define_unit_enum!(SigmaUUnits, DegPerS15, { DegPerS15 => "deg/s**1.5" });
/// Uncertainty component along the U axis, with optional units.
pub type SigmaU = UnitValue<f64, SigmaUUnits>;

define_unit_enum!(SigmaVUnits, DegPerS05, { DegPerS05 => "deg/s**0.5" });
/// Uncertainty component along the V axis, with optional units.
pub type SigmaV = UnitValue<f64, SigmaVUnits>;

// Sensor noise (string with optional angle units)
/// Sensor noise standard deviations with optional angular units (ADM table 5-9).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SensorNoise {
    /// Noise standard deviations in sensor-defined component order.
    #[serde(rename = "$value", default, with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
    /// Unit annotation. Omission uses the fixed units specified by the containing field.
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngleUnits>,
}

/// Re-entry disintegration type.
///
/// KVN additionally accepts the all-lowercase spellings. Both notations tolerate
/// `MASS-LOSS + BREAKUP`, a transcription variant that is in neither the book nor the XSD (the
/// book's Figure C-4 breaks `BREAK-` / `UP` across a line); it reads as
/// [`DisintegrationType::MassLossAndBreakUp`] and is always written as `MASS-LOSS + BREAK-UP`.
/// Diagnostics list only the book spellings.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum DisintegrationType {
    /// No disintegration considered.
    #[serde(rename = "NONE")]
    None,
    /// Mass loss considered.
    #[serde(rename = "MASS-LOSS")]
    MassLoss,
    /// Break-up considered.
    #[serde(rename = "BREAK-UP")]
    BreakUp,
    /// Both mass loss and break-up considered.
    #[serde(rename = "MASS-LOSS + BREAK-UP")]
    MassLossAndBreakUp,
}

impl DisintegrationType {
    fn from_upper(s: &str) -> Option<Self> {
        match s {
            "NONE" => Some(Self::None),
            "MASS-LOSS" => Some(Self::MassLoss),
            "BREAK-UP" => Some(Self::BreakUp),
            "MASS-LOSS + BREAK-UP" | "MASS-LOSS + BREAKUP" => Some(Self::MassLossAndBreakUp),
            _ => None,
        }
    }

    fn parse_error(s: &str) -> crate::error::EnumParseError {
        crate::error::EnumParseError {
            field: "REENTRY_DISINTEGRATION",
            value: s.to_string(),
            expected: "NONE, MASS-LOSS, BREAK-UP, or MASS-LOSS + BREAK-UP",
        }
    }
}

impl std::str::FromStr for DisintegrationType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lower_only = s.bytes().all(|b| !b.is_ascii_uppercase());
        let canonical = if lower_only {
            s.to_ascii_uppercase()
        } else {
            s.to_string()
        };
        Self::from_upper(&canonical).ok_or_else(|| Self::parse_error(s))
    }
}

// XML follows the XSD enumeration (uppercase) plus the tolerated `BREAKUP` variant.
impl<'de> Deserialize<'de> for DisintegrationType {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_upper(&s).ok_or_else(|| serde::de::Error::custom(Self::parse_error(&s)))
    }
}

impl std::fmt::Display for DisintegrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::MassLoss => write!(f, "MASS-LOSS"),
            Self::BreakUp => write!(f, "BREAK-UP"),
            Self::MassLossAndBreakUp => write!(f, "MASS-LOSS + BREAK-UP"),
        }
    }
}

/// Impact uncertainty method.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ImpactUncertaintyType {
    /// No uncertainty method.
    #[serde(rename = "NONE")]
    None,
    /// Analytical uncertainty method.
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    /// Stochastic uncertainty method.
    #[serde(rename = "STOCHASTIC")]
    Stochastic,
    /// Empirical uncertainty method.
    #[serde(rename = "EMPIRICAL")]
    Empirical,
}

impl std::str::FromStr for ImpactUncertaintyType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" | "none" => Ok(Self::None),
            "ANALYTICAL" | "analytical" => Ok(Self::Analytical),
            "STOCHASTIC" | "stochastic" => Ok(Self::Stochastic),
            "EMPIRICAL" | "empirical" => Ok(Self::Empirical),
            _ => Err(crate::error::EnumParseError {
                field: "IMPACT_UNCERTAINTY_METHOD",
                value: s.to_string(),
                expected: "NONE, ANALYTICAL, STOCHASTIC, or EMPIRICAL",
            }),
        }
    }
}

impl std::fmt::Display for ImpactUncertaintyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Analytical => write!(f, "ANALYTICAL"),
            Self::Stochastic => write!(f, "STOCHASTIC"),
            Self::Empirical => write!(f, "EMPIRICAL"),
        }
    }
}

/// Re-entry uncertainty method.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ReentryUncertaintyMethodType {
    /// No uncertainty method.
    #[serde(rename = "NONE")]
    None,
    /// Analytical uncertainty method.
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    /// Stochastic uncertainty method.
    #[serde(rename = "STOCHASTIC")]
    Stochastic,
    /// Empirical uncertainty method.
    #[serde(rename = "EMPIRICAL")]
    Empirical,
}

impl std::str::FromStr for ReentryUncertaintyMethodType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" | "none" => Ok(Self::None),
            "ANALYTICAL" | "analytical" => Ok(Self::Analytical),
            "STOCHASTIC" | "stochastic" => Ok(Self::Stochastic),
            "EMPIRICAL" | "empirical" => Ok(Self::Empirical),
            _ => Err(crate::error::EnumParseError {
                field: "REENTRY_UNCERTAINTY_METHOD",
                value: s.to_string(),
                expected: "NONE, ANALYTICAL, STOCHASTIC, or EMPIRICAL",
            }),
        }
    }
}

impl std::fmt::Display for ReentryUncertaintyMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Analytical => write!(f, "ANALYTICAL"),
            Self::Stochastic => write!(f, "STOCHASTIC"),
            Self::Empirical => write!(f, "EMPIRICAL"),
        }
    }
}

// TimeSystemType: XSD has empty restriction; represent as a string newtype.
/// Time system string constrained externally by schema usage (e.g., TDB, UTC).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TimeSystemType(pub String);

impl std::fmt::Display for TimeSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// AngVelFrameType: XSD empty restriction (free-form string), used in APM angVelStateType.
/// Angular velocity frame identifier (schema leaves unrestricted).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AngVelFrameType(pub String);

impl std::str::FromStr for AngVelFrameType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl std::fmt::Display for AngVelFrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// USER DEFINED PARAMETERS block (`userDefinedType`).
/// User-defined parameters.
///
/// Allow for the exchange of any desired orbital data not already provided in the message.
///
/// **CCSDS Reference**: 502.0-B-3, Sections 3.2.4 (OPM) and 4.2.4 (OMM).
/// OCM 3.0 uses its separate ordered `OcmUserDefined` model.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct UserDefined {
    /// Comments and parameters in their original XML order.
    #[serde(rename = "$value", default)]
    pub entries: Vec<UserDefinedEntry>,
}

/// An ordered entry in a user-defined block.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum UserDefinedEntry {
    /// Comment text at this position in the user-defined block.
    #[serde(rename = "COMMENT")]
    Comment(String),
    /// A named user-defined value at this position in the block.
    #[serde(rename = "USER_DEFINED")]
    Parameter(UserDefinedParameter),
}

impl UserDefined {
    /// Iterate comment entries in source order, skipping parameters.
    pub fn comments(&self) -> impl Iterator<Item = &String> {
        self.entries.iter().filter_map(|entry| match entry {
            UserDefinedEntry::Comment(value) => Some(value),
            _ => None,
        })
    }

    /// Iterate parameter entries in source order, skipping comments.
    pub fn parameters(&self) -> impl Iterator<Item = &UserDefinedParameter> {
        self.entries.iter().filter_map(|entry| match entry {
            UserDefinedEntry::Parameter(value) => Some(value),
            _ => None,
        })
    }

    /// Mutably iterate parameter entries without changing the comment order.
    pub fn parameters_mut(&mut self) -> impl Iterator<Item = &mut UserDefinedParameter> {
        self.entries.iter_mut().filter_map(|entry| match entry {
            UserDefinedEntry::Parameter(value) => Some(value),
            _ => None,
        })
    }

    /// Checks that every parameter name can be written as a `USER_DEFINED_<name>` keyword.
    pub(crate) fn validate_kvn_parameter_names(&self) -> Result<()> {
        for value in self.parameters() {
            if value.parameter.is_empty()
                || !value.parameter.bytes().all(|byte| {
                    byte.is_ascii_graphic() && !byte.is_ascii_lowercase() && byte != b'='
                })
            {
                return Err(crate::error::ValidationError::InvalidValue {
                    field: "USER_DEFINED parameter".into(),
                    value: value.parameter.clone(),
                    expected: "a nonempty uppercase printable ASCII suffix without whitespace or ="
                        .into(),
                    line: None,
                }
                .into());
            }
        }
        Ok(())
    }

    pub(crate) fn validate_kvn_representability(&self) -> Result<()> {
        self.validate_kvn_parameter_names()?;
        let mut parameter_seen = false;
        for entry in &self.entries {
            match entry {
                UserDefinedEntry::Parameter(_) => {
                    parameter_seen = true;
                }
                UserDefinedEntry::Comment(_) if parameter_seen => {
                    return Err(crate::error::ValidationError::Generic {
                        message: "KVN user-defined comments must precede parameters; XML entry order cannot be preserved".into(),
                        line: None,
                    }.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl crate::traits::ToKvn for UserDefined {
    fn write_kvn(&self, writer: &mut crate::kvn::ser::KvnWriter) {
        for entry in &self.entries {
            match entry {
                UserDefinedEntry::Comment(value) => {
                    writer.write_comments(std::slice::from_ref(value))
                }
                UserDefinedEntry::Parameter(value) => {
                    writer.write_user_defined(&value.parameter, &value.value)
                }
            }
        }
    }
}

/// Single USER_DEFINED parameter.
///
/// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserDefinedParameter {
    /// Value of the user-defined parameter.
    #[serde(rename = "$value", default)]
    pub value: String,
    /// Name of the user-defined parameter.
    #[serde(rename = "@parameter")]
    pub parameter: String,
}

// -------------------- CDM TYPES --------------------

// Velocity delta-v units (m/s) and type (`dvType`)
define_required_unit_type!(Dv, DvUnits, MPerS, { MPerS => "m/s" });

// m**2 units and type (`m2Type`)
define_required_unit_type!(M2, M2Units, M2, { M2 => "m**2" });

// m**2/s units and type (`m2sType`)
define_required_unit_type!(M2s, M2sUnits, M2PerS, { M2PerS => "m**2/s" });

// m**2/s**2 units and type (`m2s2Type`)
define_required_unit_type!(M2s2, M2s2Units, M2PerS2, { M2PerS2 => "m**2/s**2" });

// m**3/kg units and type (`m3kgType`)
define_required_unit_type!(M3kg, M3kgUnits, M3PerKg, { M3PerKg => "m**3/kg" });

// m**3/(kg*s) units and type (`m3kgsType`)
define_required_unit_type!(M3kgs, M3kgsUnits, M3PerKgS, { M3PerKgS => "m**3/(kg*s)" });

// m**4/kg**2 units and type (`m4kg2Type`)
define_required_unit_type!(M4kg2, M4kg2Units, M4PerKg2, { M4PerKg2 => "m**4/kg**2" });

// m**2/s**3 units and type (`m2s3Type`)
define_required_unit_type!(M2s3, M2s3Units, M2PerS3, { M2PerS3 => "m**2/s**3" });

// m**3/(kg*s**2) units and type (`m3kgs2Type`)
define_required_unit_type!(M3kgs2, M3kgs2Units, M3PerKgS2, { M3PerKgS2 => "m**3/(kg*s**2)" });

// m**2/s**4 units and type (`m2s4Type`)
define_required_unit_type!(M2s4, M2s4Units, M2PerS4, { M2PerS4 => "m**2/s**4" });

// m**2/kg units and type (`m2kgType`)
define_unit_type!(M2kg, M2kgUnits, M2PerKg, { M2PerKg => "m**2/kg" });
define_required_type!(M2kgRequired, M2kgUnits, M2PerKg);

// CDM categorical simple types
/// CDM Object type (OBJECT1 or OBJECT2).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CdmObjectType {
    /// The object to which the metadata and data apply is OBJECT1.
    #[serde(rename = "OBJECT1", alias = "object1")]
    Object1,
    /// The object to which the metadata and data apply is OBJECT2.
    #[serde(rename = "OBJECT2", alias = "object2")]
    Object2,
}

impl std::str::FromStr for CdmObjectType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "OBJECT1" => Ok(Self::Object1),
            "OBJECT2" => Ok(Self::Object2),
            _ => Err(crate::error::EnumParseError {
                field: "OBJECT",
                value: s.to_string(),
                expected: "OBJECT1 or OBJECT2",
            }),
        }
    }
}

/// Screening volume frame type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ScreenVolumeFrameType {
    /// Radial, Transverse, and Normal (RTN) coordinate frame.
    #[serde(rename = "RTN", alias = "rtn")]
    Rtn,
    /// Transverse, Velocity, and Normal (TVN) coordinate frame.
    #[serde(rename = "TVN", alias = "tvn")]
    Tvn,
}

impl std::fmt::Display for ScreenVolumeFrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rtn => write!(f, "RTN"),
            Self::Tvn => write!(f, "TVN"),
        }
    }
}

impl std::str::FromStr for ScreenVolumeFrameType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "RTN" => Ok(Self::Rtn),
            "TVN" => Ok(Self::Tvn),
            _ => Err(crate::error::EnumParseError {
                field: "SCREEN_VOLUME_FRAME",
                value: s.to_string(),
                expected: "RTN or TVN",
            }),
        }
    }
}

/// Screening volume shape type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ScreenVolumeShapeType {
    /// Ellipsoid screening volume.
    #[serde(rename = "ELLIPSOID", alias = "ellipsoid")]
    Ellipsoid,
    /// Box screening volume.
    #[serde(rename = "BOX", alias = "box")]
    Box,
}

impl std::fmt::Display for ScreenVolumeShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ellipsoid => write!(f, "ELLIPSOID"),
            Self::Box => write!(f, "BOX"),
        }
    }
}

impl std::str::FromStr for ScreenVolumeShapeType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "ELLIPSOID" => Ok(Self::Ellipsoid),
            "BOX" => Ok(Self::Box),
            _ => Err(crate::error::EnumParseError {
                field: "SCREEN_VOLUME_SHAPE",
                value: s.to_string(),
                expected: "ELLIPSOID or BOX",
            }),
        }
    }
}

/// CDM reference frame type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ReferenceFrameType {
    /// Geocentric Celestial Reference Frame.
    #[serde(rename = "GCRF", alias = "gcrf")]
    Gcrf,
    /// Earth Mean Equinox and Equator of J2000.
    #[serde(rename = "EME2000", alias = "eme2000")]
    Eme2000,
    /// International Terrestrial Reference Frame.
    #[serde(rename = "ITRF", alias = "itrf")]
    Itrf,
}

impl std::fmt::Display for ReferenceFrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gcrf => write!(f, "GCRF"),
            Self::Eme2000 => write!(f, "EME2000"),
            Self::Itrf => write!(f, "ITRF"),
        }
    }
}

impl std::str::FromStr for ReferenceFrameType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "GCRF" => Ok(Self::Gcrf),
            "EME2000" => Ok(Self::Eme2000),
            "ITRF" => Ok(Self::Itrf),
            _ => Err(crate::error::EnumParseError {
                field: "REF_FRAME",
                value: s.to_string(),
                expected: "GCRF, EME2000, or ITRF",
            }),
        }
    }
}

/// Covariance method type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CovarianceMethodType {
    /// Covariance was calculated during the OD.
    #[serde(rename = "CALCULATED", alias = "calculated")]
    Calculated,
    /// An arbitrary, non-calculated default value was used.
    #[serde(rename = "DEFAULT", alias = "default")]
    Default,
}

impl std::fmt::Display for CovarianceMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Calculated => write!(f, "CALCULATED"),
            Self::Default => write!(f, "DEFAULT"),
        }
    }
}

impl std::str::FromStr for CovarianceMethodType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "CALCULATED" => Ok(Self::Calculated),
            "DEFAULT" => Ok(Self::Default),
            _ => Err(crate::error::EnumParseError {
                field: "COVARIANCE_METHOD",
                value: s.to_string(),
                expected: "CALCULATED or DEFAULT",
            }),
        }
    }
}

/// Maneuverable type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ManeuverableType {
    /// Object is maneuverable.
    #[serde(rename = "YES", alias = "yes")]
    Yes,
    /// Object is not maneuverable.
    #[serde(rename = "NO", alias = "no")]
    No,
    /// Maneuverability is not applicable or unknown.
    #[serde(rename = "N/A", alias = "n/a")]
    NA,
}

impl std::fmt::Display for ManeuverableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "YES"),
            Self::No => write!(f, "NO"),
            Self::NA => write!(f, "N/A"),
        }
    }
}

impl std::str::FromStr for ManeuverableType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "YES" => Ok(Self::Yes),
            "NO" => Ok(Self::No),
            "N/A" => Ok(Self::NA),
            _ => Err(crate::error::EnumParseError {
                field: "MANEUVERABLE",
                value: s.to_string(),
                expected: "YES, NO, or N/A",
            }),
        }
    }
}

//----------------------------------------------------------------------
// Vector Types
//----------------------------------------------------------------------

/// A 3-element vector of doubles (XSD vec3Double)
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3Double {
    /// First vector component; units and reference frame are supplied by the containing field.
    pub x: f64,
    /// Second vector component; units and reference frame are supplied by the containing field.
    pub y: f64,
    /// Third vector component; units and reference frame are supplied by the containing field.
    pub z: f64,
}

impl Vec3Double {
    /// Construct a vector in component order; the owning field defines its units.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Serialize for Vec3Double {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{} {} {}", self.x, self.y, self.z))
    }
}

impl<'de> Deserialize<'de> for Vec3Double {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::utils::deserialize_parsed(
            deserializer,
            "three space-separated numbers",
            Self::from_kvn_value,
        )
    }
}

impl FromKvnValue for Vec3Double {
    fn from_kvn_value(val: &str) -> Result<Self> {
        let parts: Vec<&str> = val.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(crate::error::FormatError::InvalidFormat(format!(
                "Vec3Double requires 3 values, got {}: {}",
                parts.len(),
                val
            ))
            .into());
        }
        let x = fast_float::parse(parts[0]).map_err(|_| {
            CcsdsNdmError::Validation(Box::new(crate::error::ValidationError::Generic {
                message: "Invalid X component".into(),
                line: None,
            }))
        })?;
        let y = fast_float::parse(parts[1]).map_err(|_| {
            CcsdsNdmError::Validation(Box::new(crate::error::ValidationError::Generic {
                message: "Invalid Y component".into(),
                line: None,
            }))
        })?;
        let z = fast_float::parse(parts[2]).map_err(|_| {
            CcsdsNdmError::Validation(Box::new(crate::error::ValidationError::Generic {
                message: "Invalid Z component".into(),
                line: None,
            }))
        })?;
        Ok(Self { x, y, z })
    }
}

impl std::fmt::Display for Vec3Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

/// A 4-element vector of doubles (XSD vec4Double)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Vec4Double {
    /// Four components in source order.
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl Vec4Double {
    /// Construct four components in their wire order; the owning field defines their meaning.
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            values: vec![a, b, c, d],
        }
    }
}

impl FromKvnValue for Vec4Double {
    fn from_kvn_value(val: &str) -> Result<Self> {
        let parts: Vec<&str> = val.split_whitespace().collect();
        if parts.len() != 4 {
            return Err(crate::error::FormatError::InvalidFormat(format!(
                "Vec4Double requires 4 values, got {}: {}",
                parts.len(),
                val
            ))
            .into());
        }
        let mut values = Vec::with_capacity(4);
        for p in parts {
            let v = fast_float::parse(p).map_err(|_| {
                crate::error::FormatError::InvalidFormat(format!(
                    "Vec4Double value parse failed: {}",
                    p
                ))
            })?;
            values.push(v);
        }
        Ok(Self { values })
    }
}

impl std::fmt::Display for Vec4Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.values.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for v in iter {
            write!(f, " {}", v)?;
        }
        Ok(())
    }
}

// -------------------- TDM TYPES --------------------

/// TDM angle type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum TdmAngleType {
    /// Azimuth, elevation (local horizontal).
    #[serde(rename = "AZEL")]
    Azel,
    /// Right ascension, declination or hour angle, declination (must be referenced to an
    /// inertial frame).
    #[serde(rename = "RADEC")]
    Radec,
    /// x-east, y-north.
    #[serde(rename = "XEYN")]
    Xeyn,
    /// x-south, y-east.
    #[serde(rename = "XSYE")]
    Xsye,
    /// A book-valid ICD or registry value outside the bundled XML enumeration.
    Other(String),
}

impl std::str::FromStr for TdmAngleType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "AZEL" => Ok(Self::Azel),
            "RADEC" => Ok(Self::Radec),
            "XEYN" => Ok(Self::Xeyn),
            "XSYE" => Ok(Self::Xsye),
            _ if !s.trim().is_empty() => Ok(Self::Other(s.to_owned())),
            _ => Err(crate::error::EnumParseError {
                field: "ANGLE_TYPE",
                value: s.to_string(),
                expected: "AZEL, RADEC, XEYN, or XSYE",
            }),
        }
    }
}

impl std::fmt::Display for TdmAngleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(value) => f.write_str(value),
            Self::Azel => f.write_str("AZEL"),
            Self::Radec => f.write_str("RADEC"),
            Self::Xeyn => f.write_str("XEYN"),
            Self::Xsye => f.write_str("XSYE"),
        }
    }
}

impl TryFrom<String> for TdmAngleType {
    type Error = crate::error::EnumParseError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "azel" => Ok(Self::Azel),
            "radec" => Ok(Self::Radec),
            "xeyn" => Ok(Self::Xeyn),
            "xsye" => Ok(Self::Xsye),
            _ if value.bytes().any(|byte| byte.is_ascii_lowercase()) => Ok(Self::Other(value)),
            _ => value.parse(),
        }
    }
}

impl From<TdmAngleType> for String {
    fn from(value: TdmAngleType) -> Self {
        value.to_string()
    }
}

/// TDM data quality.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmDataQuality {
    /// No quality check of the data has occurred.
    #[serde(rename = "RAW", alias = "raw")]
    Raw,
    /// Data quality has been checked, and passed tests.
    #[serde(rename = "VALIDATED", alias = "validated")]
    Validated,
    /// Data quality has been checked and quality issues exist.
    #[serde(rename = "DEGRADED", alias = "degraded")]
    Degraded,
}

impl std::str::FromStr for TdmDataQuality {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "RAW" => Ok(Self::Raw),
            "VALIDATED" => Ok(Self::Validated),
            "DEGRADED" => Ok(Self::Degraded),
            _ => Err(crate::error::EnumParseError {
                field: "DATA_QUALITY",
                value: s.to_string(),
                expected: "RAW, VALIDATED, or DEGRADED",
            }),
        }
    }
}

impl std::fmt::Display for TdmDataQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "RAW"),
            Self::Validated => write!(f, "VALIDATED"),
            Self::Degraded => write!(f, "DEGRADED"),
        }
    }
}

/// Indicates the relationship between the INTEGRATION_INTERVAL and the timetag.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmIntegrationRef {
    /// Timetag represents the start of the integration period.
    #[serde(rename = "START", alias = "start")]
    Start,
    /// Timetag represents the middle of the integration period.
    #[serde(rename = "MIDDLE", alias = "middle")]
    Middle,
    /// Timetag represents the end of the integration period.
    #[serde(rename = "END", alias = "end")]
    End,
}

impl std::str::FromStr for TdmIntegrationRef {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "START" => Ok(Self::Start),
            "MIDDLE" => Ok(Self::Middle),
            "END" => Ok(Self::End),
            _ => Err(crate::error::EnumParseError {
                field: "INTEGRATION_REF",
                value: s.to_string(),
                expected: "START, MIDDLE, or END",
            }),
        }
    }
}

impl std::fmt::Display for TdmIntegrationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "START"),
            Self::Middle => write!(f, "MIDDLE"),
            Self::End => write!(f, "END"),
        }
    }
}

/// TDM tracking mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmMode {
    /// The value ‘SEQUENTIAL’ applies for frequencies, phase, range, Doppler, carrier power,
    /// carrier-power-to-noise spectral density, ranging-power-to-noise spectral density,
    /// optical, angles, and line-of-sight ionosphere calibrations; the name implies a
    /// sequential signal path between tracking participants.
    #[serde(rename = "SEQUENTIAL", alias = "sequential")]
    Sequential,
    /// The value ‘SINGLE_DIFF’ applies only for differenced data.
    #[serde(rename = "SINGLE_DIFF", alias = "single_diff")]
    SingleDiff,
}

impl std::str::FromStr for TdmMode {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "SEQUENTIAL" => Ok(Self::Sequential),
            "SINGLE_DIFF" => Ok(Self::SingleDiff),
            _ => Err(crate::error::EnumParseError {
                field: "MODE",
                value: s.to_string(),
                expected: "SEQUENTIAL or SINGLE_DIFF",
            }),
        }
    }
}

impl std::fmt::Display for TdmMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sequential => write!(f, "SEQUENTIAL"),
            Self::SingleDiff => write!(f, "SINGLE_DIFF"),
        }
    }
}

/// TDM range mode.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmRangeMode {
    /// Range tones are coherent with the uplink carrier.
    #[serde(rename = "COHERENT", alias = "coherent")]
    Coherent,
    /// Range tones have a constant frequency.
    #[serde(rename = "CONSTANT", alias = "constant")]
    Constant,
    /// Used in Delta-DOR.
    #[serde(rename = "ONE_WAY", alias = "one_way")]
    OneWay,
}

impl std::str::FromStr for TdmRangeMode {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "COHERENT" => Ok(Self::Coherent),
            "CONSTANT" => Ok(Self::Constant),
            "ONE_WAY" => Ok(Self::OneWay),
            _ => Err(crate::error::EnumParseError {
                field: "RANGE_MODE",
                value: s.to_string(),
                expected: "COHERENT, CONSTANT, or ONE_WAY",
            }),
        }
    }
}

impl std::fmt::Display for TdmRangeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coherent => write!(f, "COHERENT"),
            Self::Constant => write!(f, "CONSTANT"),
            Self::OneWay => write!(f, "ONE_WAY"),
        }
    }
}

/// TDM range units.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmRangeUnits {
    /// Range is measured in kilometers.
    #[serde(rename = "km", alias = "KM")]
    Km,
    /// Range is measured in seconds.
    #[serde(rename = "s", alias = "S")]
    Seconds,
    /// Range units where the transmit frequency is changing.
    #[serde(rename = "RU", alias = "ru")]
    Ru,
}

impl std::str::FromStr for TdmRangeUnits {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "km" => Ok(Self::Km),
            "s" => Ok(Self::Seconds),
            "ru" => Ok(Self::Ru),
            _ => Err(crate::error::EnumParseError {
                field: "RANGE_UNITS",
                value: s.to_string(),
                expected: "km, s, or RU",
            }),
        }
    }
}

impl std::fmt::Display for TdmRangeUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Km => write!(f, "km"),
            Self::Seconds => write!(f, "s"),
            Self::Ru => write!(f, "RU"),
        }
    }
}

/// TDM reference frames, including open KVN values that XML cannot represent.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum TdmReferenceFrame {
    /// Earth mean equator and equinox of J2000.
    #[serde(rename = "EME2000")]
    Eme2000,
    /// International Celestial Reference Frame.
    #[serde(rename = "ICRF")]
    Icrf,
    /// International Terrestrial Reference Frame, 2000 realization.
    #[serde(rename = "ITRF2000")]
    Itrf2000,
    /// International Terrestrial Reference Frame, 1993 realization.
    #[serde(rename = "ITRF-93")]
    Itrf93,
    /// International Terrestrial Reference Frame, 1997 realization.
    #[serde(rename = "ITRF-97")]
    Itrf97,
    /// True equator and equinox of date.
    #[serde(rename = "TOD")]
    Tod,
    /// A book-valid ICD or registry value outside the bundled XML enumeration.
    Other(String),
}

impl std::str::FromStr for TdmReferenceFrame {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "EME2000" => Ok(Self::Eme2000),
            "ICRF" => Ok(Self::Icrf),
            "ITRF2000" | "ITRF-2000" => Ok(Self::Itrf2000),
            "ITRF-93" | "ITRF1993" | "ITRF93" => Ok(Self::Itrf93),
            "ITRF-97" => Ok(Self::Itrf97),
            "TOD" | "TOD_EARTH" => Ok(Self::Tod),
            // The TDM 2.0 XSD spells these `ITRF-93` and `TOD`, while the book's table spells the
            // same frames `ITRF1993` and `TOD_EARTH`. Both authorities are accepted, so both
            // spellings are advertised — listing only one set sends a reader of the other
            // authority looking for a mistake that is not there.
            _ if !s.trim().is_empty() => Ok(Self::Other(s.to_owned())),
            _ => Err(crate::error::EnumParseError {
                field: "REFERENCE_FRAME",
                value: s.to_string(),
                expected: "EME2000, ICRF, ITRF2000, ITRF-93 (or ITRF1993), ITRF-97, \
                           or TOD (or TOD_EARTH)",
            }),
        }
    }
}

impl std::fmt::Display for TdmReferenceFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(value) => f.write_str(value),
            Self::Eme2000 => f.write_str("EME2000"),
            Self::Icrf => f.write_str("ICRF"),
            Self::Itrf2000 => f.write_str("ITRF2000"),
            Self::Itrf93 => f.write_str("ITRF-93"),
            Self::Itrf97 => f.write_str("ITRF-97"),
            Self::Tod => f.write_str("TOD"),
        }
    }
}

impl TryFrom<String> for TdmReferenceFrame {
    type Error = crate::error::EnumParseError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "eme2000" => Ok(Self::Eme2000),
            "icrf" => Ok(Self::Icrf),
            "itrf2000" => Ok(Self::Itrf2000),
            "itrf-93" => Ok(Self::Itrf93),
            "itrf-97" => Ok(Self::Itrf97),
            "tod" => Ok(Self::Tod),
            _ if value.bytes().any(|byte| byte.is_ascii_lowercase()) => Ok(Self::Other(value)),
            _ => value.parse(),
        }
    }
}

impl From<TdmReferenceFrame> for String {
    fn from(value: TdmReferenceFrame) -> Self {
        value.to_string()
    }
}

/// Reference for time tags in the tracking data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmTimetagRef {
    /// Timetag is the transmit time.
    #[serde(rename = "TRANSMIT", alias = "transmit")]
    Transmit,
    /// Timetag is the receive time.
    #[serde(rename = "RECEIVE", alias = "receive")]
    Receive,
}

impl std::str::FromStr for TdmTimetagRef {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match enum_token(s).as_ref() {
            "TRANSMIT" => Ok(Self::Transmit),
            "RECEIVE" => Ok(Self::Receive),
            _ => Err(crate::error::EnumParseError {
                field: "TIMETAG_REF",
                value: s.to_string(),
                expected: "TRANSMIT or RECEIVE",
            }),
        }
    }
}

impl std::fmt::Display for TdmTimetagRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transmit => write!(f, "TRANSMIT"),
            Self::Receive => write!(f, "RECEIVE"),
        }
    }
}

/// Represents the signal path in a TDM (e.g., "1,2,1").
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct TdmPath(pub String);

impl std::str::FromStr for TdmPath {
    type Err = crate::error::ValidationError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Simple regex-like validation: \d{1},\d{1}(,\d{1})*
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() < 2 {
            return Err(crate::error::ValidationError::InvalidValue {
                field: "PATH".into(),
                value: s.to_string(),
                expected: "at least two participants (e.g., 1,2)".into(),
                line: None,
            });
        }
        for part in &parts {
            if part.len() != 1 || !part.chars().next().unwrap().is_ascii_digit() {
                return Err(crate::error::ValidationError::InvalidValue {
                    field: "PATH".into(),
                    value: s.to_string(),
                    expected: "single digit participant indices separated by commas".into(),
                    line: None,
                });
            }
            let idx = part.parse::<u8>().unwrap();
            if !(1..=5).contains(&idx) {
                return Err(crate::error::ValidationError::InvalidValue {
                    field: "PATH".into(),
                    value: s.to_string(),
                    expected: "participant indices in range 1..5".into(),
                    line: None,
                });
            }
        }
        Ok(Self(s.to_string()))
    }
}

impl std::fmt::Display for TdmPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_enums_reject_unknown_values_and_mixed_case() {
        assert!("PAYLOAD".parse::<ObjectDescription>().is_ok());
        assert!("payload".parse::<ObjectDescription>().is_ok());
        assert!("Payload".parse::<ObjectDescription>().is_err());
        assert!("NEW_OBJECT_TYPE".parse::<ObjectDescription>().is_err());
    }

    #[test]
    fn user_defined_xml_retains_interleaved_comments_and_exact_names() {
        let input = "<userDefinedParameters><USER_DEFINED parameter=\"USER_DEFINED_X\">1</USER_DEFINED><COMMENT>after</COMMENT><USER_DEFINED parameter=\"X\">2</USER_DEFINED></userDefinedParameters>";
        let block: UserDefined = quick_xml::de::from_str(input).unwrap();
        assert!(matches!(&block.entries[1], UserDefinedEntry::Comment(value) if value == "after"));
        let xml = quick_xml::se::to_string(&block).unwrap();
        let reparsed: UserDefined = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(block, reparsed);
        assert!(block.validate_kvn_representability().is_err());
        let mut block = block;
        block.entries.remove(1);
        block.validate_kvn_representability().unwrap();
        let mut writer = crate::kvn::ser::KvnWriter::new();
        crate::traits::ToKvn::write_kvn(&block, &mut writer);
        let kvn = writer.finish_checked().unwrap();
        assert!(kvn.contains("USER_DEFINED_USER_DEFINED_X"));
        assert_eq!(
            crate::kvn::parser::user_defined_parameters(&mut kvn.as_str()).unwrap(),
            Some(block)
        );
    }

    #[test]
    fn calendar_fields_with_a_non_digit_are_invalid_not_a_panic() {
        // Found by fuzzing: a non-digit byte in a numeric field underflowed `byte - b'0'`.
        for value in [
            "2008-071T17:08:*0",
            "2008-*1-01T00:00:00",
            "20*8-071T17:08:00",
        ] {
            assert!(!common_calendar_fields_are_valid(value), "{value}");
        }
    }

    #[test]
    fn test_non_negative_double() {
        assert!(NonNegativeDouble::new(0.0).is_ok());
        assert!(NonNegativeDouble::new(1.0).is_ok());
        assert!(NonNegativeDouble::new(-0.1).is_err());
    }

    #[test]
    fn constrained_numeric_types_reject_non_finite_values() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(DayInterval::new(value, None).is_err());
            assert!(Mass::new(value, None).is_err());
            assert!(Area::new(value, None).is_err());
            assert!(NonNegativeDouble::new(value).is_err());
            assert!(Percentage::new(value, None).is_err());
            assert!(PercentageRequired::new(value).is_err());
            assert!(Probability::new(value).is_err());
            assert!(LatitudeRequired::new(value).is_err());
            assert!(LongitudeRequired::new(value).is_err());
        }
    }

    #[test]
    fn test_positive_integer() {
        assert!(PositiveInteger::new(1).is_ok());
        assert!(PositiveInteger::new(100).is_ok());
        assert!(PositiveInteger::new(0).is_err());
    }

    #[test]
    fn attitude_type_value_counts_match_aem_layouts() {
        assert_eq!(AttitudeTypeType::Quaternion.value_count(), 4);
        assert_eq!(AttitudeTypeType::QuaternionDerivative.value_count(), 8);
        assert_eq!(AttitudeTypeType::QuaternionAngVel.value_count(), 7);
        assert_eq!(AttitudeTypeType::EulerAngle.value_count(), 3);
        assert_eq!(AttitudeTypeType::EulerAngleDerivative.value_count(), 6);
        assert_eq!(AttitudeTypeType::EulerAngleAngVel.value_count(), 6);
        assert_eq!(AttitudeTypeType::Spin.value_count(), 4);
        assert_eq!(AttitudeTypeType::SpinNutation.value_count(), 7);
        assert_eq!(AttitudeTypeType::SpinNutationMom.value_count(), 7);
    }

    #[test]
    fn test_element_set_no() {
        assert!(ElementSetNo::new(0).is_ok());
        assert!(ElementSetNo::new(9999).is_ok());
        assert!(ElementSetNo::new(10000).is_err());
    }

    #[test]
    fn test_epoch_xsd_compliance() {
        // Valid calendar/ordinal formats
        assert!(Epoch::new("2023-11-13T12:00:00").is_ok());
        assert!(Epoch::new("2023-11-13T12:00:00Z").is_ok());
        assert!(Epoch::new("2023-11-13T12:00:00.123Z").is_ok());
        assert!(Epoch::new("2023-317T12:00:00Z").is_ok()); // Ordinal day
        assert!(Epoch::new("2023-11-13T12:00:00+05:00").is_ok());
        assert!(Epoch::new("2023-11-13T12:00:00-05:00").is_ok());
        assert!(Epoch::new("-2023-11-13T12:00:00Z").is_ok()); // Negative year

        // Valid numeric formats
        assert!(Epoch::new("12345.678").is_ok());
        assert!(Epoch::new("+12345.678").is_ok());
        assert!(Epoch::new("-12345.678").is_ok());
        assert!(Epoch::new(".678").is_ok());
        assert!(Epoch::new("12345.").is_ok());
        assert!(Epoch::new("12345").is_ok());
        assert!(Epoch::new("+").is_ok()); // Technically valid according to XSD [+-]?\d*(\.\d*)?
        assert!(Epoch::new("-").is_ok()); // Technically valid according to XSD
        assert!(Epoch::new(".").is_ok()); // Technically valid according to XSD

        // Empty string
        assert!(Epoch::new("").is_ok());

        // Invalid formats
        assert!(Epoch::new("2023-11-13").is_err()); // Missing time
        assert!(Epoch::new("2023-11-13T12:00").is_err()); // Missing seconds
        assert!(Epoch::new("2023-11-13T12:00:00Z+05:00").is_err()); // Double TZ
        assert!(Epoch::new("not-a-date").is_err());
    }

    #[test]
    fn epoch_classification_is_computed_at_construction() {
        let calendar = Epoch::new("2000-366T23:59:60.123Z").unwrap();
        assert_eq!(calendar.kind(), EpochKind::Calendar);
        assert_eq!(calendar.calendar_fields_are_valid(), Some(true));
        assert_eq!(calendar.numeric_is_non_degenerate(), None);

        let invalid_calendar = Epoch::new("2001-366T24:00:00.").unwrap();
        assert_eq!(invalid_calendar.kind(), EpochKind::Calendar);
        assert_eq!(invalid_calendar.calendar_fields_are_valid(), Some(false));

        let relative = Epoch::new("-123.5").unwrap();
        assert_eq!(relative.kind(), EpochKind::Numeric);
        assert_eq!(relative.calendar_fields_are_valid(), None);
        assert_eq!(relative.numeric_is_non_degenerate(), Some(true));

        for value in ["", "+", "-", ".", "123."] {
            let epoch = Epoch::new(value).unwrap();
            assert_eq!(epoch.kind(), EpochKind::Numeric);
            assert_eq!(epoch.numeric_is_non_degenerate(), Some(false));
        }
    }

    #[test]
    fn epoch_same_branch_comparison_preserves_decimal_and_calendar_semantics() {
        use Ordering::{Equal, Greater, Less};

        let numeric = |value: &str| Epoch::new(value).unwrap();
        assert_eq!(numeric("1").cmp_same_branch(&numeric("+1.0")), Some(Equal));
        assert_eq!(numeric("-2").cmp_same_branch(&numeric("-1.9")), Some(Less));
        assert_eq!(
            numeric("0.0000000000000000001").cmp_same_branch(&numeric("0")),
            Some(Greater)
        );
        assert_eq!(
            numeric("2").cmp_same_branch(&numeric("1.9999999999999999999")),
            Some(Greater)
        );

        let calendar = |value: &str| Epoch::new(value).unwrap();
        assert_eq!(
            calendar("2023-001T00:00:00Z").cmp_same_branch(&calendar("2023-01-02T00:00:00Z")),
            Some(Less)
        );
        assert_eq!(
            calendar("2024-001T00:00:00Z").cmp_same_branch(&calendar("2024-01-01T00:00:00Z")),
            Some(Equal)
        );
        assert_eq!(
            calendar("2023-365T23:59:59Z").cmp_same_branch(&calendar("2024-001T00:00:00Z")),
            Some(Less)
        );
        assert_eq!(
            calendar("2016-366T23:59:60.5Z").cmp_same_branch(&calendar("2017-001T00:00:00Z")),
            Some(Less)
        );
        assert_eq!(
            calendar("2023-01-01T00:00:00Z").cmp_same_branch(&calendar("2023-01-01T00:00:00.0Z")),
            Some(Equal)
        );
        assert_eq!(
            calendar("2023-001T00:00:00.09Z").cmp_same_branch(&calendar("2023-001T00:00:00.1Z")),
            Some(Less)
        );
        // Mixed calendar and ordinal layouts with unequal fraction precision or a `Z`.
        for (left, right, ordering) in [
            ("2020-001T00:00:00", "2020-01-01T00:00:00.000", Equal),
            ("2020-01-01T00:00:00", "2020-001T00:00:00.0", Equal),
            ("2020-001T00:00:00.5Z", "2020-01-01T00:00:00.50", Equal),
            ("2020-001T00:00:00.000Z", "2020-01-01T00:00:00", Equal),
            ("2020-001T00:00:00", "2020-01-01T00:00:00.1", Less),
            ("2020-001T00:00:00.25Z", "2020-01-01T00:00:00.2", Greater),
        ] {
            assert_eq!(
                calendar(left).cmp_same_branch(&calendar(right)),
                Some(ordering),
                "{left} vs {right}"
            );
        }
        assert_eq!(
            calendar("2023-01-01T00:00:00Z").cmp_same_branch(&numeric("1")),
            None
        );
    }

    #[test]
    fn fraction_digit_count_reads_only_calendar_fractions() {
        for (value, digits) in [
            ("2020-01-01T00:00:00", 0),
            ("2020-01-01T00:00:00Z", 0),
            ("2020-01-01T00:00:00.123", 3),
            ("2020-001T00:00:00.1234567890123456Z", 16),
            ("0000-400T00:00:00.5", 1),
            ("123.456", 0),
            ("", 0),
            ("Z", 0),
        ] {
            assert_eq!(fraction_digit_count(value.as_bytes()), digits, "{value}");
        }
    }

    #[test]
    fn xml_unit_values_use_compact_exact_spellings() {
        #[derive(Serialize)]
        #[serde(rename = "Wrapper")]
        struct Wrapper {
            #[serde(rename = "X")]
            x: Position,
        }
        for (value, spelling) in [
            (1e308, "1e+308"),
            (-1.25e-9, "-1.25e-9"),
            (2789.6, "2789.6"),
            (0.00033313494, "0.00033313494"),
            (-280.0, "-280"),
            (0.0, "0"),
        ] {
            let xml = crate::xml::to_string(&Wrapper {
                x: Position::new(value, None),
            })
            .unwrap();
            assert!(
                xml.contains(&format!("<X>{spelling}</X>")),
                "{value}: {xml}"
            );
            assert_eq!(parse_xml_value::<f64>(spelling).unwrap(), value);
        }
    }

    #[test]
    fn unit_values_round_trip_through_self_describing_serde_formats() {
        // UnitValue serializes finite values as numbers; reading them back must not require
        // the XML text form.
        let position = Position::new(1.5, Some(PositionUnits::Km));
        let json = serde_json::to_string(&position).unwrap();
        assert_eq!(serde_json::from_str::<Position>(&json).unwrap(), position);
        let integral: Position = serde_json::from_str(r#"{"$value":2,"@units":"km"}"#).unwrap();
        assert_eq!(integral.value, 2.0);
    }

    #[test]
    fn contextual_epoch_wrappers_preserve_spelling_and_branch() {
        let calendar: CalendarEpoch = "2000-366T23:59:60.123Z".parse().unwrap();
        assert_eq!(calendar.as_str(), "2000-366T23:59:60.123Z");
        assert_eq!(calendar.to_string(), calendar.as_str());
        assert_eq!(calendar.into_epoch().kind(), EpochKind::Calendar);
    }

    #[test]
    fn calendar_epoch_rejects_other_xsd_branch_and_invalid_fields() {
        for value in ["123.5", "", "+", "123."] {
            assert!(CalendarEpoch::new(value).is_err());
        }
        assert!(CalendarEpoch::new("2001-365T00:00:00").is_ok());
        assert!(CalendarEpoch::new("2001-365T00:00:00+05:30").is_err());
        assert!(CalendarEpoch::new("2001-365T00:00:00+24:00").is_err());
        assert!(CalendarEpoch::new("02001-365T00:00:00Z").is_err());
        assert!(CalendarEpoch::new("-2001-365T00:00:00Z").is_err());
    }

    #[test]
    fn contextual_epoch_wrappers_have_no_per_value_size_overhead() {
        assert_eq!(
            std::mem::size_of::<CalendarEpoch>(),
            std::mem::size_of::<Epoch>()
        );
        assert_eq!(std::mem::size_of::<Epoch>(), 66);
        // `RelativeTime` is a compact spelling-only value (64-byte buffer + length).
        assert_eq!(std::mem::size_of::<RelativeTime>(), 65);
    }

    #[test]
    fn relative_time_preserves_supported_numeric_spelling() {
        for value in [
            "100",
            "+100.0",
            "-1.25",
            "2.587e-06",
            "-2.5E+3",
            ".5",
            "100.",
            "1e2",
            "1.2345678901234567",
            "-0",
            "-0.0",
        ] {
            let relative: RelativeTime = value.parse().unwrap();
            assert_eq!(relative.as_str(), value);
            assert_eq!(relative.to_string(), value);
        }
    }

    #[test]
    fn relative_time_rejects_non_numeric_and_unsupported_values() {
        for value in [
            "",
            "1.0e",
            "1.0e+",
            "2023-01-01T00:00:00",
            "NaN",
            "INF",
            "-INF",
            "1e400",
        ] {
            assert!(RelativeTime::new(value).is_err(), "accepted {value:?}");
        }
    }

    #[test]
    fn test_epoch_length_limit() {
        let long_epoch = "A".repeat(65);
        assert_eq!(
            Epoch::new(&long_epoch),
            Err(EpochError::TooLong {
                length: 65,
                maximum: MAX_EPOCH_LEN,
            })
        );
        let _max_epoch = "A".repeat(64);
        // "A" is not a valid epoch format, so it should fail anyway, but let's test length check
        // We can use numeric format for long valid epoch if needed, but 64 is huge for digits.
        let long_numeric = "1".repeat(64);
        assert!(Epoch::new(&long_numeric).is_ok());
        let too_long_numeric = "1".repeat(65);
        assert!(Epoch::new(&too_long_numeric).is_err());
        let prefix = "2026-001T00:00:00.";
        let calendar = format!("{prefix}{}", "0".repeat(MAX_EPOCH_LEN - prefix.len()));
        assert_eq!(Epoch::new(&calendar).unwrap().as_str(), calendar);
        assert!(CalendarEpoch::new(&calendar).is_ok());
        assert!(matches!(
            CalendarEpoch::new(&format!("{calendar}0")),
            Err(EpochError::TooLong { .. })
        ));
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_vec3double_from_kvn_error() {
        assert!(Vec3Double::from_kvn_value("1.0 2.0").is_err()); // missing 3rd
        assert!(Vec3Double::from_kvn_value("1.0 2.0 3.0 4.0").is_err()); // extra
        assert!(Vec3Double::from_kvn_value("1.0 foo 3.0").is_err()); // invalid float
        assert!(Vec3Double::from_kvn_value("invalid").is_err());
    }

    #[test]
    fn test_vec3double_display() {
        let v = Vec3Double::new(1.1, 2.2, 3.3);
        assert_eq!(format!("{}", v), "1.1 2.2 3.3");
    }

    macro_rules! test_enum_from_str {
        ($type:ty, $valid:expr, $invalid:expr) => {
            // Test valid
            assert!($valid.parse::<$type>().is_ok());
            // Test invalid
            let res = $invalid.parse::<$type>();
            assert!(res.is_err());
            // Check error message content if possible, or just strict existence
            let err = res.unwrap_err();
            assert!(!err.to_string().is_empty());
        };
    }

    #[test]
    fn test_enum_parsing_errors() {
        test_enum_from_str!(ReentryUncertaintyMethodType, "NONE", "INVALID");
        test_enum_from_str!(CdmObjectType, "OBJECT1", "INVALID");
        test_enum_from_str!(ScreenVolumeFrameType, "RTN", "INVALID");
        test_enum_from_str!(ScreenVolumeShapeType, "BOX", "INVALID");
        test_enum_from_str!(ReferenceFrameType, "GCRF", "INVALID");
        test_enum_from_str!(CovarianceMethodType, "CALCULATED", "INVALID");
        test_enum_from_str!(ManeuverableType, "YES", "INVALID");
        test_enum_from_str!(TdmAngleType, "AZEL", "");
        test_enum_from_str!(TdmDataQuality, "RAW", "INVALID");
        test_enum_from_str!(TdmIntegrationRef, "START", "INVALID");
        test_enum_from_str!(TdmMode, "SEQUENTIAL", "INVALID");
        test_enum_from_str!(TdmRangeMode, "COHERENT", "INVALID");
        test_enum_from_str!(TdmRangeUnits, "km", "INVALID");
        test_enum_from_str!(TdmReferenceFrame, "EME2000", "");
        test_enum_from_str!(TdmTimetagRef, "TRANSMIT", "INVALID");
    }

    #[test]
    fn test_unit_value_from_kvn() {
        let uv = UnitValue::<f64, PositionUnits>::from_kvn("123.45", Some("km")).unwrap();
        assert_eq!(uv.value, 123.45);
        assert_eq!(uv.units, Some(PositionUnits::Km));

        let uv_no_unit = UnitValue::<f64, PositionUnits>::from_kvn("123.45", None).unwrap();
        assert_eq!(uv_no_unit.units, None);
    }

    #[test]
    fn test_angle_validation() {
        assert!(Angle::new(359.9, None).is_ok());
        assert!(Angle::new(-359.9, None).is_ok());
        assert!(Angle::new(360.0, None).is_err());
        assert!(Angle::new(-360.1, None).is_err());
    }

    #[test]
    fn test_day_interval_validation() {
        assert!(DayInterval::new(10.0, None).is_ok());
        assert!(DayInterval::new(-0.1, None).is_err());
        assert!(DayIntervalRequired::new(0.1).is_ok());
        assert!(DayIntervalRequired::new(0.0).is_err());
        assert!(DayIntervalRequired::new(-1.0).is_err());
        assert!(DayIntervalRequired::new(f64::NAN).is_err());
        assert!(DayIntervalRequired::new(f64::INFINITY).is_err());
        assert!(DayIntervalRequired::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_frequency_validation() {
        assert!(Frequency::new(1.0, None).is_ok());
        assert!(Frequency::new(0.0, None).is_err());
        assert!(Frequency::validate_xml_value(0.0, "Frequency").is_err());
    }

    #[test]
    fn test_gm_validation() {
        assert!(Gm::new(1.0, None).is_ok());
        assert!(Gm::new(0.0, None).is_ok());
        assert!(Gm::validate_xml_value(0.0, "GM").is_err());
        assert!("KM**3/S**2".parse::<GmUnits>().is_ok());
    }

    #[test]
    fn test_altitude_required_validation() {
        // RDM states no range, so the model accepts any finite altitude, including values only a
        // non-Earth body-fixed frame would produce.
        assert!(AltitudeRequired::new(0.0).is_ok());
        assert!(AltitudeRequired::new(9000.0).is_ok());
        assert!(AltitudeRequired::new(-431.0).is_ok());
        assert!(AltitudeRequired::new(f64::NAN).is_err());
        assert!(AltitudeRequired::new(f64::INFINITY).is_err());

        // The XSD's Earth-derived range is a representability limit, reported separately.
        assert!(AltitudeRequired::new(0.0).unwrap().is_xml_representable());
        assert!(!AltitudeRequired::new(9000.0)
            .unwrap()
            .is_xml_representable());
        assert!(!AltitudeRequired::new(-431.0)
            .unwrap()
            .is_xml_representable());
    }

    #[test]
    fn test_mass_validation() {
        assert!(Mass::new(0.0, None).is_ok());
        assert!(Mass::new(-1.0, None).is_err());
    }

    #[test]
    fn test_area_validation() {
        assert!(Area::new(0.0, None).is_ok());
        assert!(Area::new(-1.0, None).is_err());
    }

    #[test]
    fn test_ms2_parsing() {
        let ms2 = Ms2::from_str("9.81").unwrap();
        assert_eq!(ms2.value, 9.81);
        assert_eq!(ms2.units, Ms2Units::MPerS2);
    }

    #[test]
    fn test_solar_flux_units() {
        test_enum_from_str!(SolarFluxUnits, "SFU", "INVALID");
        assert_eq!(format!("{}", SolarFluxUnits::JanskyScaled), "10**4 Jansky");
    }

    #[test]
    fn test_epoch_conversion() {
        let s = "2023-01-01T00:00:00Z";
        let e = Epoch::from_str(s).unwrap();
        assert_eq!(Epoch::try_from(s.to_string()).unwrap(), e);
        assert_eq!(e.as_str(), s);
        assert!(!e.is_empty());
    }

    #[test]
    fn test_percentage_validation() {
        assert!(Percentage::new(50.0, None).is_ok());
        assert!(Percentage::new(-0.1, None).is_err());
        assert!(Percentage::new(100.1, None).is_err());
        assert!(PercentageRequired::new(50.0).is_ok());
        assert!(PercentageRequired::new(-0.1).is_err());
        assert!(PercentageRequired::new(100.1).is_err());
    }

    #[test]
    fn test_unit_conversions() {
        let f = Frequency::new(10.0, Some(FrequencyUnits::Hz)).unwrap();
        let uv = f.to_unit_value();
        assert_eq!(uv.value, 10.0);
        assert_eq!(uv.units, Some(FrequencyUnits::Hz));

        let gm = Gm::new(1.0, Some(GmUnits::Km3PerS2)).unwrap();
        let uv = gm.to_unit_value();
        assert_eq!(uv.value, 1.0);
        assert_eq!(uv.units, Some(GmUnits::Km3PerS2));

        let a = Angle::new(1.0, Some(AngleUnits::Deg)).unwrap();
        let uv = a.to_unit_value();
        assert_eq!(uv.value, 1.0);
        assert_eq!(uv.units, Some(AngleUnits::Deg));
    }

    #[test]
    fn test_from_kvn_float() {
        let f = Frequency::from_kvn_float(10.0, Some("Hz")).unwrap();
        assert_eq!(f.value, 10.0);
        assert_eq!(f.units, Some(FrequencyUnits::Hz));

        let gm = Gm::from_kvn_float(1.0, Some("KM**3/S**2")).unwrap();
        assert_eq!(gm.value, 1.0);
    }

    #[test]
    fn required_kvn_units_accept_only_the_ccsds_unit() {
        let position = PositionRequired::from_kvn_float(42.0, Some("km")).unwrap();
        assert_eq!(position.units, PositionUnits::Km);

        let inferred = PositionRequired::from_kvn_float(42.0, None).unwrap();
        assert_eq!(inferred.units, PositionUnits::Km);

        let error = PositionRequired::from_kvn_float(42.0, Some("m")).unwrap_err();
        assert!(error.to_string().contains("expected one of: \"km\""));
    }

    #[test]
    fn test_additional_units() {
        test_enum_from_str!(AngleRateUnits, "deg/s", "INVALID");
        test_enum_from_str!(MomentUnits, "kg*m**2", "INVALID");
        test_enum_from_str!(QuaternionDotUnits, "1/s", "INVALID");
    }

    #[test]
    fn test_tdm_reference_frame_aliases() {
        assert!("ITRF1993".parse::<TdmReferenceFrame>().is_ok());
        assert!("ITRF93".parse::<TdmReferenceFrame>().is_ok());
        assert!("TOD_EARTH".parse::<TdmReferenceFrame>().is_ok());
    }

    #[test]
    fn test_tdm_path_index_range_validation() {
        assert!("1,2,1".parse::<TdmPath>().is_ok());
        assert!("1,6".parse::<TdmPath>().is_err());
        assert!("0,2".parse::<TdmPath>().is_err());
    }
}
