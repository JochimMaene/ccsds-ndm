// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::traits::{FromKvnFloat, FromKvnValue};
use fast_float;
use serde::{Deserialize, Serialize};
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
/// This struct uses a stack-allocated buffer to avoid heap allocations
/// during parsing of large NDM files.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Epoch {
    bytes: [u8; 64],
    len: u8,
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
}

fn is_valid_epoch(s: &str) -> bool {
    fn parser(input: &mut &str) -> winnow::Result<()> {
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
            .void(),
            // Numeric format: [+-]?\d*(\.\d*)?
            seq!(
                opt(one_of(['+', '-'])),
                digit0,
                opt(seq!('.', digit0).void())
            )
            .void(),
        ))
        .parse_next(input)
    }

    terminated(parser, eof).parse(s).is_ok()
}

impl Epoch {
    pub fn new(value: &str) -> std::result::Result<Self, EpochError> {
        // Fast path for empty or very short strings which are common in some tests
        // and allowed by the regex [+-]?\d*(\.\d*)?
        if value.len() > 64 {
            return Err(EpochError::InvalidFormat(value.to_string()));
        }

        if value.is_empty() || is_valid_epoch(value) {
            let mut bytes = [0u8; 64];
            bytes[..value.len()].copy_from_slice(value.as_bytes());
            Ok(Epoch {
                bytes,
                len: value.len() as u8,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct UnitValue<V, U> {
    #[serde(rename = "$value")]
    pub value: V,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<U>,
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
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        pub struct $name {
            #[serde(rename = "$value")]
            pub value: f64,
            #[serde(rename = "@units")]
            pub units: $unit_enum,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Angle {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngleUnits>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DayInterval {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<DayIntervalUnits>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Gm {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<GmUnits>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Mass {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<MassUnits>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Area {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AreaUnits>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Percentage {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<PercentageUnits>,
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
pub struct PercentageRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: PercentageUnits,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Probability {
    #[serde(rename = "$value")]
    pub value: f64,
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

impl std::str::FromStr for Probability {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: f64 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct NonNegativeDouble {
    #[serde(rename = "$value")]
    pub value: f64,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct PositiveInteger {
    #[serde(rename = "$value")]
    pub value: u32,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct ElementSetNo {
    #[serde(rename = "$value")]
    pub value: u32,
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

impl std::str::FromStr for ElementSetNo {
    type Err = CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let v: u32 = s.parse().map_err(CcsdsNdmError::from)?;
        Self::new(v)
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
    #[serde(rename = "$value")]
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
    #[serde(rename = "$value")]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TimeOffset {
    #[serde(rename = "$value")]
    pub value: f64, // double
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<TimeUnits>,
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
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
            "ITRF2000" => Ok(Self::Itrf2000),
            "ITRF-93" => Ok(Self::Itrf93),
            "ITRF-97" => Ok(Self::Itrf97),
            "TOD" => Ok(Self::Tod),
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
    fn test_epoch_length_limit() {
        let long_epoch = "A".repeat(65);
        assert!(Epoch::new(&long_epoch).is_err());
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
}
