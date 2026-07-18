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
/// CCSDS time and numeric value rules fit within this bound. Keeping the limit explicit prevents
/// one epoch in a large history from introducing an unbounded allocation.
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
        let s = String::deserialize(deserializer)?;
        Epoch::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EpochError {
    #[error("invalid epoch format: '{0}'")]
    InvalidFormat(String),
    #[error("epoch token is {length} bytes; maximum supported length is {maximum} bytes")]
    TooLong { length: usize, maximum: usize },
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
            byte.is_ascii_digit()
                .then_some(value * 10 + u16::from(*byte - b'0'))
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

    if !valid_date {
        return false;
    }
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

impl Epoch {
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

    /// Returns whether this value is usable in a contextual epoch field.
    ///
    /// Contextual fields may use either branch of the XSD `epochType` union, but malformed
    /// calendar fields and degenerate numeric spellings are not useful time tags.
    #[inline(always)]
    pub(crate) fn is_contextually_valid(&self) -> bool {
        match self.classification {
            EpochClassification::CalendarValid => true,
            EpochClassification::CalendarInvalid => false,
            EpochClassification::Numeric => {
                let value = self.as_str();
                value.bytes().any(|byte| byte.is_ascii_digit()) && !value.ends_with('.')
            }
        }
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
                parts: parse_calendar(self.as_str())?,
            },
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
        parts: CalendarParts,
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
                EpochOrderKeyInner::Calendar {
                    value: left,
                    parts: left_parts,
                },
                EpochOrderKeyInner::Calendar {
                    value: right,
                    parts: right_parts,
                },
            ) => Some(compare_calendar_parts(left, left_parts, right, right_parts)),
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

#[derive(Clone, Copy)]
struct CalendarParts {
    day: i64,
    seconds: i32,
    fraction_start: u8,
    fraction_len: u8,
}

fn compare_calendar_parts(
    left: &str,
    left_parts: CalendarParts,
    right: &str,
    right_parts: CalendarParts,
) -> Ordering {
    left_parts
        .day
        .cmp(&right_parts.day)
        .then_with(|| left_parts.seconds.cmp(&right_parts.seconds))
        .then_with(|| {
            compare_fraction(
                left,
                left_parts.fraction_start,
                left_parts.fraction_len,
                right,
                right_parts.fraction_start,
                right_parts.fraction_len,
            )
        })
}

fn parse_calendar(value: &str) -> Option<CalendarParts> {
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

    let time_start = value.find('T')? + 1;
    let clock = time.strip_suffix('Z').unwrap_or(time);
    let hour = parse_two_digits(&clock[0..2])?;
    let minute = parse_two_digits(&clock[3..5])?;
    let second = parse_two_digits(&clock[6..8])?;
    let fraction = clock[8..].strip_prefix('.').unwrap_or("");
    if clock.len() > 8 && fraction.is_empty() {
        return None;
    }
    let seconds = i32::from(hour) * 3600 + i32::from(minute) * 60 + i32::from(second);
    Some(CalendarParts {
        day,
        seconds,
        fraction_start: if fraction.is_empty() {
            0
        } else {
            u8::try_from(time_start + 9).ok()?
        },
        fraction_len: u8::try_from(fraction.len()).ok()?,
    })
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

fn compare_fraction(
    left: &str,
    left_start: u8,
    left_len: u8,
    right: &str,
    right_start: u8,
    right_len: u8,
) -> Ordering {
    let length = usize::from(left_len).max(usize::from(right_len));
    (0..length)
        .map(|index| {
            left.as_bytes()
                .get(usize::from(left_start) + index)
                .copied()
                .unwrap_or(b'0')
                .cmp(
                    &right
                        .as_bytes()
                        .get(usize::from(right_start) + index)
                        .copied()
                        .unwrap_or(b'0'),
                )
        })
        .find(|ordering| *ordering != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
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
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
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
    if value.is_empty() || value.len() > MAX_EPOCH_LEN || !value.is_ascii() {
        return false;
    }

    let bytes = value.as_bytes();
    let mut index = 0;
    if matches!(bytes.first(), Some(b'+' | b'-')) {
        index = 1;
    }

    let integer_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    let integer_digits = index - integer_start;

    let mut fraction_digits = 0;
    let has_decimal = bytes.get(index) == Some(&b'.');
    if has_decimal {
        index += 1;
        let fraction_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        fraction_digits = index - fraction_start;
    }

    // ADM §6.8.4 requires a digit on both sides of the decimal point for fixed/scientific
    // values. A plain integer is also valid under §6.8.3.
    if integer_digits == 0 || (has_decimal && fraction_digits == 0) {
        return false;
    }

    let has_exponent = matches!(bytes.get(index), Some(b'e' | b'E'));
    if has_exponent {
        // The ADM floating-point form uses a one-digit mantissa before the decimal point.
        if !has_decimal || integer_digits != 1 {
            return false;
        }
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let exponent_start = index;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if index == exponent_start {
            return false;
        }
    }

    if index != bytes.len() {
        return false;
    }

    // The Blue Book limits the mantissa to 16 significant decimal digits and excludes NaN,
    // infinities, and negative zero. Parsing once here is construction-time work only; the
    // serializer writes the retained spelling without reparsing.
    if has_decimal && integer_digits + fraction_digits > 16 {
        return false;
    }
    let parsed = value.parse::<f64>().ok();
    let Some(parsed) = parsed else {
        return false;
    };
    parsed.is_finite() && !(value.starts_with('-') && parsed == 0.0)
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
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
    }
}

//----------------------------------------------------------------------
// Generic Unit/Value Types
//----------------------------------------------------------------------

/// A trait for types that can be deserialized from a KVN value and optional unit.
///
/// This trait provides a standardized way to parse key-value pairs from KVN files,
/// where a value might have an associated unit in brackets (e.g., `KEY = 123.45 [km]`).
pub trait FromKvn: Sized {
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
pub struct UnitValue<V, U> {
    #[serde(rename = "$value")]
    pub value: V,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<U>,
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
                let value = v.parse::<V>().map_err(E::custom)?;
                Ok(UnitValue { value, units: None })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut units = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "$value" || key == "$text" {
                        if value.is_some() {
                            return Err(serde::de::Error::duplicate_field("$value"));
                        }
                        value = Some(map.next_value()?);
                    } else if key == "@units" {
                        if units.is_some() {
                            return Err(serde::de::Error::duplicate_field("@units"));
                        }
                        units = Some(map.next_value()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
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

//----------------------------------------------------------------------
// Macros to reduce boilerplate for unit enums and wrappers
//----------------------------------------------------------------------

/// Defines a unit enum with serde renames, plus Display, Default, and FromStr,
/// and a `UnitValue<f64, UnitEnum>` type alias with the provided name.
///
/// Usage:
/// define_unit_type!(
///     Position, PositionUnits, Km, { Km => "km" }
/// );
macro_rules! define_unit_type {
    ($type_alias:ident, $unit_enum:ident, $default_variant:ident, { $($variant:ident => $str_rep:expr),+ $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        pub enum $unit_enum {
            $(#[serde(rename = $str_rep)] $variant),+
        }

        impl Default for $unit_enum {
            fn default() -> Self { Self::$default_variant }
        }

        impl std::fmt::Display for $unit_enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $str_rep)),+
                }
            }
        }

        impl std::str::FromStr for $unit_enum {
            type Err = crate::error::EnumParseError;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $($str_rep => Ok(Self::$variant)),+,
                    _ => Err(crate::error::EnumParseError {
                        field: "unit",
                        value: s.to_string(),
                        expected: stringify!($($str_rep),+),
                    })
                }
            }
        }

        pub type $type_alias = UnitValue<f64, $unit_enum>;
    };
}

/// Defines a "required" wrapper struct that always carries units (no Option)
/// and constructs with the provided default unit variant.
///
/// Example:
/// define_required_type!(PositionRequired, PositionUnits, Km);
macro_rules! define_required_type {
    ($name:ident, $unit_enum:ident, $default_unit:ident) => {
        #[derive(Serialize, Debug, PartialEq, Clone, Default)]
        pub struct $name {
            #[serde(rename = "$value")]
            pub value: f64,
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

                    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name::new(v))
                    }

                    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let val = v.parse::<f64>().map_err(E::custom)?;
                        Ok($name::new(val))
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
                        let value =
                            value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                        let mut s = $name::new(value);
                        if let Some(u) = units {
                            s.units = u;
                        }
                        Ok(s)
                    }
                }
                deserializer.deserialize_any(Visitor)
            }
        }
        impl $name {
            pub fn new(value: f64) -> Self {
                Self {
                    value,
                    units: $unit_enum::$default_unit,
                }
            }
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
            fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
                Ok(Self::new(value))
            }
        }
    };
}

// Local macro to define only unit enums with serde/Default/Display/FromStr
macro_rules! define_unit_enum {
    ($unit_enum:ident, $default_variant:ident, { $($variant:ident => $str_rep:expr),+ $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        pub enum $unit_enum { $(#[serde(rename = $str_rep)] $variant),+ }
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
pub type Distance = Position;

// --- Angle ---

define_unit_enum!(AngleUnits, Deg, { Deg => "deg" });

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Angle {
    #[serde(rename = "$value")]
    pub value: f64,
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Angle::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(AngleVisitor)
    }
}

impl Angle {
    /// XSD angleRange: -360.0 <= value < 360.0
    pub fn new(value: f64, units: Option<AngleUnits>) -> Result<Self> {
        if !(-360.0..360.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Angle".into(),
                value: value.to_string(),
                expected: "[-360, 360)".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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

pub type AngleRate = UnitValue<f64, AngleRateUnits>;

// --- Angular Momentum ---
define_unit_type!(AngMomentum, AngMomentumUnits, NmS, { NmS => "N*m*s" });

// --- Day Interval ---

define_unit_enum!(DayIntervalUnits, D, { D => "d" });

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DayInterval {
    #[serde(rename = "$value")]
    pub value: f64,
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                DayInterval::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(DayIntervalVisitor)
    }
}

impl DayInterval {
    /// dayIntervalTypeUO: nonNegativeDouble
    pub fn new(value: f64, units: Option<DayIntervalUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "DayInterval".into(),
                value: value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Percentage {
    #[serde(rename = "$value")]
    pub value: f64,
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
                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Percentage::new(value, units).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

impl Percentage {
    pub fn new(value: f64, units: Option<PercentageUnits>) -> Result<Self> {
        if !(0.0..=100.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Percentage".into(),
                value: value.to_string(),
                expected: "[0, 100]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DayIntervalRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: DayIntervalUnits,
}
impl DayIntervalRequired {
    /// dayIntervalTypeUR: positiveDouble (>0, units required)
    pub fn new(value: f64) -> Result<Self> {
        if value <= 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "DayIntervalRequired".into(),
                value: value.to_string(),
                expected: "> 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            value,
            units: DayIntervalUnits::D,
        })
    }
    pub fn to_unit_value(&self) -> UnitValue<f64, DayIntervalUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl FromKvnFloat for DayIntervalRequired {
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}
impl std::fmt::Display for DayIntervalRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// --- Frequency ---

define_unit_enum!(FrequencyUnits, Hz, { Hz => "Hz" });

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Frequency {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<FrequencyUnits>,
}
impl Frequency {
    /// frequencyType: positiveDouble (>0)
    pub fn new(value: f64, units: Option<FrequencyUnits>) -> Result<Self> {
        if value <= 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Frequency".into(),
                value: value.to_string(),
                expected: "> 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
    pub fn to_unit_value(&self) -> UnitValue<f64, FrequencyUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Gm {
    #[serde(rename = "$value")]
    pub value: f64,
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
                v.parse::<Gm>().map_err(E::custom)
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Gm::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(GmVisitor)
    }
}

impl Gm {
    /// gmType: positiveDouble (>0)
    pub fn new(value: f64, units: Option<GmUnits>) -> Result<Self> {
        if value <= 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "GM".into(),
                value: value.to_string(),
                expected: "> 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AltitudeRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: LengthUnits,
}
impl AltitudeRequired {
    /// altRange: -430.5 ..= 8848
    pub fn new(value: f64) -> Result<Self> {
        if !(-430.5..=8848.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Altitude".into(),
                value: value.to_string(),
                expected: "[-430.5, 8848]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            value,
            units: LengthUnits::M,
        })
    }
    pub fn to_unit_value(&self) -> UnitValue<f64, LengthUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl FromKvnFloat for AltitudeRequired {
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Mass {
    #[serde(rename = "$value")]
    pub value: f64,
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Mass::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(MassVisitor)
    }
}

impl Mass {
    /// XSD massType: nonNegativeDouble
    pub fn new(value: f64, units: Option<MassUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Mass".into(),
                value: value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Area {
    #[serde(rename = "$value")]
    pub value: f64,
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Area::new(value, units).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(AreaVisitor)
    }
}

impl Area {
    /// XSD areaType: nonNegativeDouble
    pub fn new(value: f64, units: Option<AreaUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Area".into(),
                value: value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PercentageRequired {
    #[serde(rename = "$value")]
    pub value: f64,
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
                PercentageRequired::new(v).map_err(E::custom)
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let val = v.parse::<f64>().map_err(E::custom)?;
                PercentageRequired::new(val).map_err(E::custom)
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
                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                let mut s = PercentageRequired::new(value).map_err(serde::de::Error::custom)?;
                if let Some(u) = units {
                    s.units = u;
                }
                Ok(s)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
impl PercentageRequired {
    pub fn new(value: f64) -> Result<Self> {
        if !(0.0..=100.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "PercentageRequired".into(),
                value: value.to_string(),
                expected: "[0, 100]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            value,
            units: PercentageUnits::Percent,
        })
    }

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
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Probability {
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

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<Probability>().map_err(E::custom)
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Probability::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(ProbabilityVisitor)
    }
}

impl Probability {
    pub fn new(value: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Probability".into(),
                value: value.to_string(),
                expected: "[0, 1]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value })
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromKvnFloat for Probability {
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}

/// XSD nonNegativeDouble - value must be >= 0
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct NonNegativeDouble {
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                NonNegativeDouble::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(NonNegativeDoubleVisitor)
    }
}

impl NonNegativeDouble {
    pub fn new(value: f64) -> Result<Self> {
        if value < 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "NonNegativeDouble".into(),
                value: value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value })
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
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}

/// XSD positiveInteger - value must be > 0
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct PositiveInteger {
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

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<PositiveInteger>().map_err(E::custom)
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                PositiveInteger::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(PositiveIntegerVisitor)
    }
}

impl PositiveInteger {
    pub fn new(value: u32) -> Result<Self> {
        if value == 0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "PositiveInteger".into(),
                value: value.to_string(),
                expected: "> 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value })
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                ElementSetNo::new(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(ElementSetNoVisitor)
    }
}

impl ElementSetNo {
    pub fn new(value: u32) -> Result<Self> {
        if value > 9999 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "ElementSetNo".into(),
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeltaMass {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
}
impl DeltaMass {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeltaMassZ {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
}
impl DeltaMassZ {
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
pub type Latitude = UnitValue<f64, LatLonUnits>;
pub type Longitude = UnitValue<f64, LatLonUnits>;
pub type Altitude = UnitValue<f64, LengthUnits>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LatitudeRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: LatLonUnits,
}
impl LatitudeRequired {
    pub fn new(value: f64) -> Result<Self> {
        if !(-90.0..=90.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Latitude".into(),
                value: value.to_string(),
                expected: "[-90, 90]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            value,
            units: LatLonUnits::Deg,
        })
    }

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
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LongitudeRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: LatLonUnits,
}
impl LongitudeRequired {
    pub fn new(value: f64) -> Result<Self> {
        if !(-180.0..=180.0).contains(&value) {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Longitude".into(),
                value: value.to_string(),
                expected: "[-180, 180]".into(),
                line: None,
            }
            .into());
        }
        Ok(Self {
            value,
            units: LatLonUnits::Deg,
        })
    }

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
    fn from_kvn_float(value: f64, _unit: Option<&str>) -> Result<Self> {
        Self::new(value)
    }
}

// Torque
define_unit_type!(Torque, TorqueUnits, Nm, { Nm => "N*m" });

// Vector helper for cpType / targetMomentumType
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Vector3 {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub elements: Vec<f64>, // Expect length 3
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<LengthUnits>,
}
impl Vector3 {
    pub fn new(elements: [f64; 3], units: Option<LengthUnits>) -> Self {
        Self {
            elements: elements.to_vec(),
            units,
        }
    }
}

// Target momentum vector (uses angular momentum units)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TargetMomentum {
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub elements: Vec<f64>, // length 3
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngMomentumUnits>,
}
impl TargetMomentum {
    pub fn new(elements: [f64; 3], units: Option<AngMomentumUnits>) -> Self {
        Self {
            elements: elements.to_vec(),
            units,
        }
    }
}

// Categorical Enums
//----------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ObjectDescription {
    #[serde(rename = "PAYLOAD")]
    Payload,
    #[serde(rename = "payload")]
    PayloadLower,
    #[serde(rename = "ROCKET BODY")]
    RocketBody,
    #[serde(rename = "rocket body")]
    RocketBodyLower,
    #[serde(rename = "DEBRIS")]
    Debris,
    #[serde(rename = "debris")]
    DebrisLower,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "unknown")]
    UnknownLower,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "other")]
    OtherLower,
}

impl std::str::FromStr for ObjectDescription {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAYLOAD" => Ok(Self::Payload),
            "ROCKET BODY" => Ok(Self::RocketBody),
            "DEBRIS" => Ok(Self::Debris),
            "UNKNOWN" => Ok(Self::Unknown),
            "OTHER" => Ok(Self::Other),
            _ => Ok(Self::Other),
        }
    }
}
impl std::fmt::Display for ObjectDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ObjectDescription::Payload | ObjectDescription::PayloadLower => "PAYLOAD",
            ObjectDescription::RocketBody | ObjectDescription::RocketBodyLower => "ROCKET BODY",
            ObjectDescription::Debris | ObjectDescription::DebrisLower => "DEBRIS",
            ObjectDescription::Unknown | ObjectDescription::UnknownLower => "UNKNOWN",
            ObjectDescription::Other | ObjectDescription::OtherLower => "OTHER",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum RotSeq {
    #[serde(rename = "XYX")]
    XYX,
    #[serde(rename = "XYZ")]
    XYZ,
    #[serde(rename = "XZX")]
    XZX,
    #[serde(rename = "XZY")]
    XZY,
    #[serde(rename = "YXY")]
    YXY,
    #[serde(rename = "YXZ")]
    YXZ,
    #[serde(rename = "YZX")]
    YZX,
    #[serde(rename = "YZY")]
    YZY,
    #[serde(rename = "ZXY")]
    ZXY,
    #[serde(rename = "ZXZ")]
    ZXZ,
    #[serde(rename = "ZYX")]
    ZYX,
    #[serde(rename = "ZYZ")]
    ZYZ,
}

impl std::str::FromStr for RotSeq {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
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
                expected: "XYX, XYZ, XZX, XZY, YXY, YXZ, YZX, YZY, ZXY, ZXZ, ZYX, ZYZ, or numeric equivalents",
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AdMethod {
    #[serde(rename = "EKF")]
    Ekf,
    #[serde(rename = "ekf")]
    EkfLower,
    #[serde(rename = "TRIAD")]
    Triad,
    #[serde(rename = "triad")]
    TriadLower,
    #[serde(rename = "QUEST")]
    Quest,
    #[serde(rename = "quest")]
    QuestLower,
    #[serde(rename = "BATCH")]
    Batch,
    #[serde(rename = "batch")]
    BatchLower,
    #[serde(rename = "Q_METHOD")]
    QMethod,
    #[serde(rename = "q_method")]
    QMethodLower,
    #[serde(rename = "FILTER_SMOOTHER")]
    FilterSmoother,
    #[serde(rename = "filter_smoother")]
    FilterSmootherLower,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "other")]
    OtherLower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum YesNo {
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "yes")]
    YesLower,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "no")]
    NoLower,
}
impl std::fmt::Display for YesNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            YesNo::Yes | YesNo::YesLower => "YES",
            YesNo::No | YesNo::NoLower => "NO",
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

/// Basis of the trajectory state time history data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TrajBasis {
    /// Basis of this trajectory state time history data is 'PREDICTED'.
    #[serde(rename = "PREDICTED")]
    Predicted,
    /// Basis of this trajectory state time history data is 'DETERMINED' when estimated from
    /// observation-based orbit determination, reconstruction, and/or calibration. For
    /// definitive OD performed onboard spacecraft whose solutions have been telemetered to the
    /// ground for inclusion in an OCM, the TRAJ_BASIS shall be DETERMINED.
    #[serde(rename = "DETERMINED")]
    Determined,
    /// Basis of this trajectory state time history data is 'TELEMETRY' when the trajectory
    /// states are read directly from telemetry, for example, based on inertial navigation
    /// systems or GNSS data.
    #[serde(rename = "TELEMETRY")]
    Telemetry,
    /// Basis of this trajectory state time history data is 'SIMULATED' for generic
    /// simulations, future mission design studies, and optimization studies.
    #[serde(rename = "SIMULATED")]
    Simulated,
    /// Basis of this trajectory state time history data is 'OTHER' for other bases of this data.
    #[serde(rename = "OTHER")]
    Other,
}

impl std::fmt::Display for TrajBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Predicted => write!(f, "PREDICTED"),
            Self::Determined => write!(f, "DETERMINED"),
            Self::Telemetry => write!(f, "TELEMETRY"),
            Self::Simulated => write!(f, "SIMULATED"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}

impl std::str::FromStr for TrajBasis {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PREDICTED" => Ok(Self::Predicted),
            "DETERMINED" => Ok(Self::Determined),
            "TELEMETRY" => Ok(Self::Telemetry),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::EnumParseError {
                field: "TRAJ_BASIS",
                value: s.to_string(),
                expected: "PREDICTED, DETERMINED, TELEMETRY, SIMULATED, or OTHER",
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum RevNumBasis {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
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
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            _ => Err(crate::error::EnumParseError {
                field: "ORB_REVNUM_BASIS",
                value: s.to_string(),
                expected: "0 or 1",
            }),
        }
    }
}

/// Basis of the covariance time history data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CovBasis {
    /// Basis of this covariance time history data is 'PREDICTED'.
    #[serde(rename = "PREDICTED")]
    Predicted,
    /// Basis of this covariance time history data is 'DETERMINED' when estimated from
    /// observation-based orbit determination, reconstruction and/or calibration. For
    /// definitive OD performed onboard spacecraft whose solutions have been telemetered to the ground for
    /// inclusion in an OCM, the COV_BASIS shall be considered to be DETERMINED.
    #[serde(rename = "DETERMINED")]
    Determined,
    /// Basis of this covariance time history data is 'EMPIRICAL' (for empirically determined
    /// such as overlap analyses).
    #[serde(rename = "EMPIRICAL")]
    Empirical,
    /// Basis of this covariance time history data is 'SIMULATED' for simulation-based
    /// (including Monte Carlo) estimations, future mission design studies, and optimization
    /// studies.
    #[serde(rename = "SIMULATED")]
    Simulated,
    /// Basis of this covariance time history data is 'OTHER' for other bases of this data.
    #[serde(rename = "OTHER")]
    Other,
}

impl std::fmt::Display for CovBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Predicted => write!(f, "PREDICTED"),
            Self::Determined => write!(f, "DETERMINED"),
            Self::Empirical => write!(f, "EMPIRICAL"),
            Self::Simulated => write!(f, "SIMULATED"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}

impl std::str::FromStr for CovBasis {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PREDICTED" => Ok(Self::Predicted),
            "DETERMINED" => Ok(Self::Determined),
            "EMPIRICAL" => Ok(Self::Empirical),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::EnumParseError {
                field: "COV_BASIS",
                value: s.to_string(),
                expected: "PREDICTED, DETERMINED, EMPIRICAL, SIMULATED, or OTHER",
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
        match s.to_uppercase().as_str() {
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
        match s.to_uppercase().as_str() {
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
        match s.to_uppercase().as_str() {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ControlledType {
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "yes")]
    YesLower,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "no")]
    NoLower,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "unknown")]
    UnknownLower,
}
impl std::fmt::Display for ControlledType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ControlledType::Yes | ControlledType::YesLower => "YES",
            ControlledType::No | ControlledType::NoLower => "NO",
            ControlledType::Unknown | ControlledType::UnknownLower => "UNKNOWN",
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Duration {
    #[serde(rename = "$value")]
    pub value: f64, // nonNegativeDouble
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
}
impl Duration {
    pub fn new(value: f64, units: Option<TimeUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(crate::error::ValidationError::OutOfRange {
                name: "Duration".into(),
                value: value.to_string(),
                expected: ">= 0".into(),
                line: None,
            }
            .into());
        }
        Ok(Self { value, units })
    }
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RelTime {
    #[serde(rename = "$value")]
    pub value: f64, // double (can be negative)
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TimeOffset {
    #[serde(rename = "$value")]
    pub value: f64, // double
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimeOffsetVisitor;

        impl<'de> serde::de::Visitor<'de> for TimeOffsetVisitor {
            type Value = TimeOffset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive value or a map with $value and optionally @units")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<TimeOffset>().map_err(E::custom)
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

                let value = value.ok_or_else(|| serde::de::Error::missing_field("$value"))?;
                Ok(TimeOffset { value, units })
            }
        }

        deserializer.deserialize_any(TimeOffsetVisitor)
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
    pub fn to_unit_value(&self) -> UnitValue<f64, TimeUnits> {
        UnitValue {
            value: self.value,
            units: self.units.clone(),
        }
    }
}

// Inclination (0 ..= 180 deg)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct Inclination {
    pub angle: Angle, // uses AngleUnits (deg)
}
impl Inclination {
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AcmAttitudeType {
    #[serde(rename = "QUATERNION")]
    Quaternion,
    #[serde(rename = "quaternion")]
    QuaternionLower,
    #[serde(rename = "EULER_ANGLES")]
    EulerAngles,
    #[serde(rename = "euler_angles")]
    EulerAnglesLower,
    #[serde(rename = "DCM")]
    Dcm,
    #[serde(rename = "dcm")]
    DcmLower,
    #[serde(rename = "ANGVEL")]
    AngVel,
    #[serde(rename = "angvel")]
    AngVelLower,
    #[serde(rename = "Q_DOT")]
    QDot,
    #[serde(rename = "q_dot")]
    QDotLower,
    #[serde(rename = "EULER_RATE")]
    EulerRate,
    #[serde(rename = "euler_rate")]
    EulerRateLower,
    #[serde(rename = "GYRO_BIAS")]
    GyroBias,
    #[serde(rename = "gyro_bias")]
    GyroBiasLower,
}

impl std::str::FromStr for AcmAttitudeType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "QUATERNION" => Ok(Self::Quaternion),
            "quaternion" => Ok(Self::QuaternionLower),
            "EULER_ANGLES" => Ok(Self::EulerAngles),
            "euler_angles" => Ok(Self::EulerAnglesLower),
            "DCM" => Ok(Self::Dcm),
            "dcm" => Ok(Self::DcmLower),
            "ANGVEL" => Ok(Self::AngVel),
            "angvel" => Ok(Self::AngVelLower),
            "Q_DOT" => Ok(Self::QDot),
            "q_dot" => Ok(Self::QDotLower),
            "EULER_RATE" => Ok(Self::EulerRate),
            "euler_rate" => Ok(Self::EulerRateLower),
            "GYRO_BIAS" => Ok(Self::GyroBias),
            "gyro_bias" => Ok(Self::GyroBiasLower),
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
            Self::Quaternion | Self::QuaternionLower => "QUATERNION",
            Self::EulerAngles | Self::EulerAnglesLower => "EULER_ANGLES",
            Self::Dcm | Self::DcmLower => "DCM",
            Self::AngVel | Self::AngVelLower => "ANGVEL",
            Self::QDot | Self::QDotLower => "Q_DOT",
            Self::EulerRate | Self::EulerRateLower => "EULER_RATE",
            Self::GyroBias | Self::GyroBiasLower => "GYRO_BIAS",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttRateType {
    #[serde(rename = "ANGVEL")]
    AngVel,
    #[serde(rename = "angvel")]
    AngVelLower,
    #[serde(rename = "Q_DOT")]
    QDot,
    #[serde(rename = "q_dot")]
    QDotLower,
    #[serde(rename = "EULER_RATE")]
    EulerRate,
    #[serde(rename = "euler_rate")]
    EulerRateLower,
    #[serde(rename = "GYRO_BIAS")]
    GyroBias,
    #[serde(rename = "gyro_bias")]
    GyroBiasLower,
}

impl std::str::FromStr for AttRateType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ANGVEL" => Ok(Self::AngVel),
            "angvel" => Ok(Self::AngVelLower),
            "Q_DOT" => Ok(Self::QDot),
            "q_dot" => Ok(Self::QDotLower),
            "EULER_RATE" => Ok(Self::EulerRate),
            "euler_rate" => Ok(Self::EulerRateLower),
            "GYRO_BIAS" => Ok(Self::GyroBias),
            "gyro_bias" => Ok(Self::GyroBiasLower),
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
            Self::AngVel | Self::AngVelLower => "ANGVEL",
            Self::QDot | Self::QDotLower => "Q_DOT",
            Self::EulerRate | Self::EulerRateLower => "EULER_RATE",
            Self::GyroBias | Self::GyroBiasLower => "GYRO_BIAS",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttBasisType {
    #[serde(rename = "PREDICTED")]
    Predicted,
    #[serde(rename = "predicted")]
    PredictedLower,
    #[serde(rename = "DETERMINED_GND")]
    DeterminedGnd,
    #[serde(rename = "determined_gnd")]
    DeterminedGndLower,
    #[serde(rename = "DETERMINED_OBC")]
    DeterminedObc,
    #[serde(rename = "determined_obc")]
    DeterminedObcLower,
    #[serde(rename = "SIMULATED")]
    Simulated,
    #[serde(rename = "simulated")]
    SimulatedLower,
}

impl std::str::FromStr for AttBasisType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PREDICTED" => Ok(Self::Predicted),
            "predicted" => Ok(Self::PredictedLower),
            "DETERMINED_GND" => Ok(Self::DeterminedGnd),
            "determined_gnd" => Ok(Self::DeterminedGndLower),
            "DETERMINED_OBC" => Ok(Self::DeterminedObc),
            "determined_obc" => Ok(Self::DeterminedObcLower),
            "SIMULATED" => Ok(Self::Simulated),
            "simulated" => Ok(Self::SimulatedLower),
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
            Self::Predicted | Self::PredictedLower => "PREDICTED",
            Self::DeterminedGnd | Self::DeterminedGndLower => "DETERMINED_GND",
            Self::DeterminedObc | Self::DeterminedObcLower => "DETERMINED_OBC",
            Self::Simulated | Self::SimulatedLower => "SIMULATED",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AcmCovarianceLineType {
    #[serde(rename = "ANGLE")]
    Angle,
    #[serde(rename = "angle")]
    AngleLower,
    #[serde(rename = "ANGLE_GYROBIAS")]
    AngleGyroBias,
    #[serde(rename = "angle_gyrobias")]
    AngleGyroBiasLower,
    #[serde(rename = "ANGLE_ANGVEL")]
    AngleAngVel,
    #[serde(rename = "angle_angvel")]
    AngleAngVelLower,
    #[serde(rename = "QUATERNION")]
    Quaternion,
    #[serde(rename = "quaternion")]
    QuaternionLower,
    #[serde(rename = "QUATERNION_GYROBIAS")]
    QuaternionGyroBias,
    #[serde(rename = "quaternion_gyrobias")]
    QuaternionGyroBiasLower,
    #[serde(rename = "QUATERNION_ANGVEL")]
    QuaternionAngVel,
    #[serde(rename = "quaternion_angvel")]
    QuaternionAngVelLower,
}

impl std::str::FromStr for AcmCovarianceLineType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ANGLE" => Ok(Self::Angle),
            "angle" => Ok(Self::AngleLower),
            "ANGLE_GYROBIAS" => Ok(Self::AngleGyroBias),
            "angle_gyrobias" => Ok(Self::AngleGyroBiasLower),
            "ANGLE_ANGVEL" => Ok(Self::AngleAngVel),
            "angle_angvel" => Ok(Self::AngleAngVelLower),
            "QUATERNION" => Ok(Self::Quaternion),
            "quaternion" => Ok(Self::QuaternionLower),
            "QUATERNION_GYROBIAS" => Ok(Self::QuaternionGyroBias),
            "quaternion_gyrobias" => Ok(Self::QuaternionGyroBiasLower),
            "QUATERNION_ANGVEL" => Ok(Self::QuaternionAngVel),
            "quaternion_angvel" => Ok(Self::QuaternionAngVelLower),
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
            Self::Angle | Self::AngleLower => "ANGLE",
            Self::AngleGyroBias | Self::AngleGyroBiasLower => "ANGLE_GYROBIAS",
            Self::AngleAngVel | Self::AngleAngVelLower => "ANGLE_ANGVEL",
            Self::Quaternion | Self::QuaternionLower => "QUATERNION",
            Self::QuaternionGyroBias | Self::QuaternionGyroBiasLower => "QUATERNION_GYROBIAS",
            Self::QuaternionAngVel | Self::QuaternionAngVelLower => "QUATERNION_ANGVEL",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AttitudeTypeType {
    #[serde(rename = "quaternion")]
    Quaternion,
    #[serde(rename = "QUATERNION")]
    QuaternionUpper,
    #[serde(rename = "quaternion/derivative")]
    QuaternionDerivative,
    #[serde(rename = "QUATERNION/DERIVATIVE")]
    QuaternionDerivativeUpper,
    #[serde(rename = "quaternion/angvel")]
    QuaternionAngVel,
    #[serde(rename = "QUATERNION/ANGVEL")]
    QuaternionAngVelUpper,
    #[serde(rename = "euler_angle")]
    EulerAngle,
    #[serde(rename = "EULER_ANGLE")]
    EulerAngleUpper,
    #[serde(rename = "euler_angle/derivative")]
    EulerAngleDerivative,
    #[serde(rename = "EULER_ANGLE/DERIVATIVE")]
    EulerAngleDerivativeUpper,
    #[serde(rename = "euler_angle/angvel")]
    EulerAngleAngVel,
    #[serde(rename = "EULER_ANGLE/ANGVEL")]
    EulerAngleAngVelUpper,
    #[serde(rename = "spin")]
    Spin,
    #[serde(rename = "SPIN")]
    SpinUpper,
    #[serde(rename = "spin/nutation")]
    SpinNutation,
    #[serde(rename = "SPIN/NUTATION")]
    SpinNutationUpper,
    #[serde(rename = "spin/nutation_mom")]
    SpinNutationMom,
    #[serde(rename = "SPIN/NUTATION_MOM")]
    SpinNutationMomUpper,
}

impl std::str::FromStr for AttitudeTypeType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "quaternion" => Ok(Self::Quaternion),
            "QUATERNION" => Ok(Self::QuaternionUpper),
            "quaternion/derivative" => Ok(Self::QuaternionDerivative),
            "QUATERNION/DERIVATIVE" => Ok(Self::QuaternionDerivativeUpper),
            "quaternion/angvel" => Ok(Self::QuaternionAngVel),
            "QUATERNION/ANGVEL" => Ok(Self::QuaternionAngVelUpper),
            "euler_angle" => Ok(Self::EulerAngle),
            "EULER_ANGLE" => Ok(Self::EulerAngleUpper),
            "euler_angle/derivative" => Ok(Self::EulerAngleDerivative),
            "EULER_ANGLE/DERIVATIVE" => Ok(Self::EulerAngleDerivativeUpper),
            "euler_angle/angvel" => Ok(Self::EulerAngleAngVel),
            "EULER_ANGLE/ANGVEL" => Ok(Self::EulerAngleAngVelUpper),
            "spin" => Ok(Self::Spin),
            "SPIN" => Ok(Self::SpinUpper),
            "spin/nutation" => Ok(Self::SpinNutation),
            "SPIN/NUTATION" => Ok(Self::SpinNutationUpper),
            "spin/nutation_mom" => Ok(Self::SpinNutationMom),
            "SPIN/NUTATION_MOM" => Ok(Self::SpinNutationMomUpper),
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
            Self::Quaternion | Self::QuaternionUpper => "QUATERNION",
            Self::QuaternionDerivative | Self::QuaternionDerivativeUpper => "QUATERNION/DERIVATIVE",
            Self::QuaternionAngVel | Self::QuaternionAngVelUpper => "QUATERNION/ANGVEL",
            Self::EulerAngle | Self::EulerAngleUpper => "EULER_ANGLE",
            Self::EulerAngleDerivative | Self::EulerAngleDerivativeUpper => {
                "EULER_ANGLE/DERIVATIVE"
            }
            Self::EulerAngleAngVel | Self::EulerAngleAngVelUpper => "EULER_ANGLE/ANGVEL",
            Self::Spin | Self::SpinUpper => "SPIN",
            Self::SpinNutation | Self::SpinNutationUpper => "SPIN/NUTATION",
            Self::SpinNutationMom | Self::SpinNutationMomUpper => "SPIN/NUTATION_MOM",
        };
        write!(f, "{}", value)
    }
}

// APM rate frame
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ApmRateFrame {
    #[serde(rename = "EULER_FRAME_A")]
    EulerFrameA,
    #[serde(rename = "EULER_FRAME_B")]
    EulerFrameB,
}

// SigmaU / SigmaV units and types
define_unit_enum!(SigmaUUnits, DegPerS15, { DegPerS15 => "deg/s**1.5" });
pub type SigmaU = UnitValue<f64, SigmaUUnits>;

define_unit_enum!(SigmaVUnits, DegPerS05, { DegPerS05 => "deg/s**0.5" });
pub type SigmaV = UnitValue<f64, SigmaVUnits>;

// Sensor noise (string with optional angle units)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SensorNoise {
    #[serde(rename = "$value", default, with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngleUnits>,
}

/// Re-entry disintegration type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    #[serde(rename = "MASS-LOSS + BREAK-UP", alias = "MASS-LOSS + BREAKUP")]
    MassLossAndBreakUp,
}

impl std::str::FromStr for DisintegrationType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "MASS-LOSS" => Ok(Self::MassLoss),
            "BREAK-UP" => Ok(Self::BreakUp),
            "MASS-LOSS + BREAK-UP" | "MASS-LOSS + BREAKUP" => Ok(Self::MassLossAndBreakUp),
            _ => Err(crate::error::EnumParseError {
                field: "REENTRY_DISINTEGRATION",
                value: s.to_string(),
                expected: "NONE, MASS-LOSS, BREAK-UP, or MASS-LOSS + BREAK-UP",
            }),
        }
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
    /// Covariance uncertainty method.
    #[serde(rename = "COVARIANCE")]
    Covariance,
    /// Statistical uncertainty method.
    #[serde(rename = "STATISTICAL")]
    Statistical,
}

impl std::str::FromStr for ImpactUncertaintyType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "ANALYTICAL" => Ok(Self::Analytical),
            "STOCHASTIC" => Ok(Self::Stochastic),
            "EMPIRICAL" => Ok(Self::Empirical),
            "COVARIANCE" => Ok(Self::Covariance),
            "STATISTICAL" => Ok(Self::Statistical),
            _ => Err(crate::error::EnumParseError {
                field: "IMPACT_UNCERTAINTY_METHOD",
                value: s.to_string(),
                expected: "NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL, COVARIANCE, or STATISTICAL",
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
            Self::Covariance => write!(f, "COVARIANCE"),
            Self::Statistical => write!(f, "STATISTICAL"),
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
    /// Covariance uncertainty method.
    #[serde(rename = "COVARIANCE")]
    Covariance,
    /// Statistical uncertainty method.
    #[serde(rename = "STATISTICAL")]
    Statistical,
}

impl std::str::FromStr for ReentryUncertaintyMethodType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Self::None),
            "ANALYTICAL" => Ok(Self::Analytical),
            "STOCHASTIC" => Ok(Self::Stochastic),
            "EMPIRICAL" => Ok(Self::Empirical),
            "COVARIANCE" => Ok(Self::Covariance),
            "STATISTICAL" => Ok(Self::Statistical),
            _ => Err(crate::error::EnumParseError {
                field: "REENTRY_UNCERTAINTY_METHOD",
                value: s.to_string(),
                expected: "NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL, COVARIANCE, or STATISTICAL",
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
            Self::Covariance => write!(f, "COVARIANCE"),
            Self::Statistical => write!(f, "STATISTICAL"),
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
/// **CCSDS Reference**: 502.0-B-3, Section 3.2.4 (OPM), Section 4.2.4 (OMM), Section 6.2.9 (OCM).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserDefined {
    /// Comments (see 7.8 for formatting rules).
    ///
    /// **CCSDS Reference**: 502.0-B-3, Section 6.2.9.
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    /// List of user-defined parameters.
    #[serde(
        rename = "USER_DEFINED",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_defined: Vec<UserDefinedParameter>,
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
    #[serde(rename = "OBJECT1")]
    Object1,
    /// The object to which the metadata and data apply is OBJECT2.
    #[serde(rename = "OBJECT2")]
    Object2,
}

impl std::str::FromStr for CdmObjectType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "RTN")]
    Rtn,
    /// Transverse, Velocity, and Normal (TVN) coordinate frame.
    #[serde(rename = "TVN")]
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
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "ELLIPSOID")]
    Ellipsoid,
    /// Box screening volume.
    #[serde(rename = "BOX")]
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
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "GCRF")]
    Gcrf,
    /// Earth Mean Equinox and Equator of J2000.
    #[serde(rename = "EME2000")]
    Eme2000,
    /// International Terrestrial Reference Frame.
    #[serde(rename = "ITRF")]
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
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "CALCULATED")]
    Calculated,
    /// An arbitrary, non-calculated default value was used.
    #[serde(rename = "DEFAULT")]
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
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "YES")]
    Yes,
    /// Object is not maneuverable.
    #[serde(rename = "NO")]
    No,
    /// Maneuverability is not applicable or unknown.
    #[serde(rename = "N/A")]
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
        match s.to_uppercase().as_str() {
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
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3Double {
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
        let value = String::deserialize(deserializer)?;
        Self::from_kvn_value(&value).map_err(serde::de::Error::custom)
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
    #[serde(rename = "$value", with = "crate::utils::vec_f64_space_sep")]
    pub values: Vec<f64>,
}

impl Vec4Double {
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
}

impl std::str::FromStr for TdmAngleType {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AZEL" => Ok(Self::Azel),
            "RADEC" => Ok(Self::Radec),
            "XEYN" => Ok(Self::Xeyn),
            "XSYE" => Ok(Self::Xsye),
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
            Self::Azel => write!(f, "AZEL"),
            Self::Radec => write!(f, "RADEC"),
            Self::Xeyn => write!(f, "XEYN"),
            Self::Xsye => write!(f, "XSYE"),
        }
    }
}

/// TDM data quality.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmDataQuality {
    /// No quality check of the data has occurred.
    #[serde(rename = "RAW")]
    Raw,
    /// Data quality has been checked, and passed tests.
    #[serde(rename = "VALIDATED")]
    Validated,
    /// Data quality has been checked and quality issues exist.
    #[serde(rename = "DEGRADED")]
    Degraded,
}

impl std::str::FromStr for TdmDataQuality {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "START")]
    Start,
    /// Timetag represents the middle of the integration period.
    #[serde(rename = "MIDDLE")]
    Middle,
    /// Timetag represents the end of the integration period.
    #[serde(rename = "END")]
    End,
}

impl std::str::FromStr for TdmIntegrationRef {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "SEQUENTIAL")]
    Sequential,
    /// The value ‘SINGLE_DIFF’ applies only for differenced data.
    #[serde(rename = "SINGLE_DIFF")]
    SingleDiff,
}

impl std::str::FromStr for TdmMode {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "COHERENT")]
    Coherent,
    /// Range tones have a constant frequency.
    #[serde(rename = "CONSTANT")]
    Constant,
    /// Used in Delta-DOR.
    #[serde(rename = "ONE_WAY")]
    OneWay,
}

impl std::str::FromStr for TdmRangeMode {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    #[serde(rename = "km")]
    Km,
    /// Range is measured in seconds.
    #[serde(rename = "s")]
    Seconds,
    /// Range units where the transmit frequency is changing.
    #[serde(rename = "RU")]
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
                expected: "km, s, or ru",
            }),
        }
    }
}

impl std::fmt::Display for TdmRangeUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Km => write!(f, "km"),
            Self::Seconds => write!(f, "s"),
            Self::Ru => write!(f, "ru"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmReferenceFrame {
    #[serde(rename = "EME2000")]
    Eme2000,
    #[serde(rename = "ICRF")]
    Icrf,
    #[serde(rename = "ITRF2000")]
    Itrf2000,
    #[serde(rename = "ITRF-93")]
    Itrf93,
    #[serde(rename = "ITRF-97")]
    Itrf97,
    #[serde(rename = "TOD")]
    Tod,
}

impl std::str::FromStr for TdmReferenceFrame {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "EME2000" => Ok(Self::Eme2000),
            "ICRF" => Ok(Self::Icrf),
            "ITRF2000" | "ITRF-2000" => Ok(Self::Itrf2000),
            "ITRF-93" | "ITRF1993" | "ITRF93" => Ok(Self::Itrf93),
            "ITRF-97" => Ok(Self::Itrf97),
            "TOD" | "TOD_EARTH" => Ok(Self::Tod),
            _ => Err(crate::error::EnumParseError {
                field: "REFERENCE_FRAME",
                value: s.to_string(),
                expected: "EME2000, ICRF, ITRF2000, ITRF-93, ITRF-97, or TOD",
            }),
        }
    }
}

impl std::fmt::Display for TdmReferenceFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eme2000 => write!(f, "EME2000"),
            Self::Icrf => write!(f, "ICRF"),
            Self::Itrf2000 => write!(f, "ITRF2000"),
            Self::Itrf93 => write!(f, "ITRF-93"),
            Self::Itrf97 => write!(f, "ITRF-97"),
            Self::Tod => write!(f, "TOD"),
        }
    }
}

/// Reference for time tags in the tracking data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TdmTimetagRef {
    /// Timetag is the transmit time.
    #[serde(rename = "TRANSMIT")]
    Transmit,
    /// Timetag is the receive time.
    #[serde(rename = "RECEIVE")]
    Receive,
}

impl std::str::FromStr for TdmTimetagRef {
    type Err = crate::error::EnumParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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
    fn test_non_negative_double() {
        assert!(NonNegativeDouble::new(0.0).is_ok());
        assert!(NonNegativeDouble::new(1.0).is_ok());
        assert!(NonNegativeDouble::new(-0.1).is_err());
    }

    #[test]
    fn test_positive_integer() {
        assert!(PositiveInteger::new(1).is_ok());
        assert!(PositiveInteger::new(100).is_ok());
        assert!(PositiveInteger::new(0).is_err());
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
            calendar("2023-01-01T00:00:00Z").cmp_same_branch(&numeric("1")),
            None
        );
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
        for value in ["100", "+100.0", "-1.25", "2.587e-06", "-2.5E+3"] {
            let relative: RelativeTime = value.parse().unwrap();
            assert_eq!(relative.as_str(), value);
            assert_eq!(relative.to_string(), value);
        }
    }

    #[test]
    fn relative_time_rejects_non_numeric_and_unsupported_values() {
        for value in [
            "",
            ".5",
            "100.",
            "1e2",
            "1.0e",
            "1.0e+",
            "1.2345678901234567",
            "2023-01-01T00:00:00",
            "NaN",
            "INF",
            "-INF",
            "-0",
            "-0.0",
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
        test_enum_from_str!(TdmAngleType, "AZEL", "INVALID");
        test_enum_from_str!(TdmDataQuality, "RAW", "INVALID");
        test_enum_from_str!(TdmIntegrationRef, "START", "INVALID");
        test_enum_from_str!(TdmMode, "SEQUENTIAL", "INVALID");
        test_enum_from_str!(TdmRangeMode, "COHERENT", "INVALID");
        test_enum_from_str!(TdmRangeUnits, "km", "INVALID");
        test_enum_from_str!(TdmReferenceFrame, "EME2000", "INVALID");
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
    }

    #[test]
    fn test_frequency_validation() {
        assert!(Frequency::new(1.0, None).is_ok());
        assert!(Frequency::new(0.0, None).is_err());
    }

    #[test]
    fn test_gm_validation() {
        assert!(Gm::new(1.0, None).is_ok());
        assert!(Gm::new(0.0, None).is_err());
        assert!("KM**3/S**2".parse::<GmUnits>().is_ok());
    }

    #[test]
    fn test_altitude_required_validation() {
        assert!(AltitudeRequired::new(0.0).is_ok());
        assert!(AltitudeRequired::new(9000.0).is_err());
        assert!(AltitudeRequired::new(-431.0).is_err());
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
