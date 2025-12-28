// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::traits::FromKvnValue;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

// Base Types
//----------------------------------------------------------------------

/// Represents the `epochType` from the XSD (e.g., "2023-11-13T12:00:00.123Z").
///
/// This struct wraps a `String` and provides validation during deserialization
/// to ensure it conforms to the CCSDS epoch format.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String")]
pub struct Epoch(String);

#[derive(Error, Debug, PartialEq)]
pub enum EpochError {
    #[error("invalid epoch format: '{0}'")]
    InvalidFormat(String),
}

static EPOCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    // This regex is from the `epochType` pattern in the XSD.
    // It validates format, but not the logical value of the date/time components.
    Regex::new(
        r"^(\-?\d{4}\d*-((\d{2}-\d{2})|\d{3})T\d{2}:\d{2}:\d{2}(\.\d*)?(Z|[+-]\d{2}:\d{2})?|[+-]?\d*(\.\d*)?)$",
    )
    .expect("EPOCH_REGEX pattern is valid")
});

impl Epoch {
    pub fn new(value: &str) -> std::result::Result<Self, EpochError> {
        if EPOCH_REGEX.is_match(value) {
            Ok(Epoch(value.to_string()))
        } else {
            Err(EpochError::InvalidFormat(value.to_string()))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
    V::Err: std::error::Error + 'static,
    U: FromStr<Err = CcsdsNdmError>,
{
    /// Parses a `UnitValue` from a value string and an optional unit string.
    ///
    /// The value is parsed using its `FromStr` implementation. If a unit string
    /// is provided, it is parsed using the unit type's `FromStr` implementation.
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let value = value
            .parse::<V>()
            .map_err(|e| CcsdsNdmError::KvnParse(e.to_string()))?;

        let units = match unit {
            Some(u_str) => Some(u_str.parse::<U>()?),
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
            type Err = crate::error::CcsdsNdmError;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $($str_rep => Ok(Self::$variant)),+,
                    _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!("Unknown unit: {}", s)))
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
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        impl FromKvn for $name {
            fn from_kvn(value: &str, _unit: Option<&str>) -> Result<Self> {
                let v: f64 = value.parse().map_err(|e: std::num::ParseFloatError| {
                    CcsdsNdmError::KvnParse(e.to_string())
                })?;
                Ok(Self::new(v))
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
            type Err = crate::error::CcsdsNdmError;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s { $($str_rep => Ok(Self::$variant)),+, _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!("Unknown unit: {}", s))) }
            }
        }
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
            return Err(CcsdsNdmError::Validation(format!(
                "Angle out of range [-360,360): {}",
                value
            )));
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
impl FromKvn for Angle {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AngleUnits>::from_kvn(value, unit)?;
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
            return Err(CcsdsNdmError::Validation(format!(
                "DayInterval must be >= 0: {}",
                value
            )));
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
impl FromKvn for DayInterval {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, DayIntervalUnits>::from_kvn(value, unit)?;
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
            return Err(CcsdsNdmError::Validation(format!(
                "DayIntervalRequired must be > 0: {}",
                value
            )));
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
impl FromKvn for DayIntervalRequired {
    fn from_kvn(value: &str, _unit: Option<&str>) -> Result<Self> {
        let v: f64 = value
            .parse()
            .map_err(|e: std::num::ParseFloatError| CcsdsNdmError::KvnParse(e.to_string()))?;
        Self::new(v)
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
    #[serde(rename = "@units")]
    pub units: Option<FrequencyUnits>,
}
impl Frequency {
    /// frequencyType: positiveDouble (>0)
    pub fn new(value: f64, units: Option<FrequencyUnits>) -> Result<Self> {
        if value <= 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "Frequency must be > 0: {}",
                value
            )));
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
impl FromKvn for Frequency {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, FrequencyUnits>::from_kvn(value, unit)?;
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
    #[serde(rename = "@units")]
    pub units: Option<GmUnits>,
}
impl Gm {
    /// gmType: positiveDouble (>0)
    pub fn new(value: f64, units: Option<GmUnits>) -> Result<Self> {
        if value <= 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "GM must be > 0: {}",
                value
            )));
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
impl FromKvn for Gm {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, GmUnits>::from_kvn(value, unit)?;
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
            return Err(CcsdsNdmError::Validation(format!(
                "Altitude out of range [-430.5,8848]: {}",
                value
            )));
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
impl FromKvn for AltitudeRequired {
    fn from_kvn(value: &str, _unit: Option<&str>) -> Result<Self> {
        let v: f64 = value
            .parse()
            .map_err(|e: std::num::ParseFloatError| CcsdsNdmError::KvnParse(e.to_string()))?;
        Self::new(v)
    }
}
impl std::fmt::Display for AltitudeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- Power/Mass Ratio ---

define_unit_enum!(WkgUnits, WPerKg, { WPerKg => "W/kg" });

pub type Wkg = UnitValue<f64, WkgUnits>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WkgRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: WkgUnits,
}
impl WkgRequired {
    /// wkgType: nonNegativeDouble, units required
    pub fn new(value: f64) -> Result<Self> {
        if value < 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "W/kg must be >= 0: {}",
                value
            )));
        }
        Ok(Self {
            value,
            units: WkgUnits::WPerKg,
        })
    }
    pub fn to_unit_value(&self) -> UnitValue<f64, WkgUnits> {
        UnitValue {
            value: self.value,
            units: Some(self.units.clone()),
        }
    }
}
impl FromKvn for WkgRequired {
    fn from_kvn(value: &str, _unit: Option<&str>) -> Result<Self> {
        let v: f64 = value
            .parse()
            .map_err(|e: std::num::ParseFloatError| CcsdsNdmError::KvnParse(e.to_string()))?;
        Self::new(v)
    }
}

// --- Mass ---

define_unit_enum!(MassUnits, Kg, { Kg => "kg" });

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Mass {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: Option<MassUnits>,
}
impl Mass {
    /// XSD massType: nonNegativeDouble
    pub fn new(value: f64, units: Option<MassUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "Mass must be >= 0: {}",
                value
            )));
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

impl FromKvn for Mass {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, MassUnits>::from_kvn(value, unit)?;
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
    #[serde(rename = "@units")]
    pub units: Option<AreaUnits>,
}

impl Area {
    /// XSD areaType: nonNegativeDouble
    pub fn new(value: f64, units: Option<AreaUnits>) -> Result<Self> {
        if value < 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "Area must be >= 0: {}",
                value
            )));
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
impl FromKvn for Area {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AreaUnits>::from_kvn(value, unit)?;
        Self::new(uv.value, uv.units)
    }
}
impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
define_unit_type!(Ms2, Ms2Units, MPerS2, { MPerS2 => "m/s**2" });

define_required_type!(Ms2Required, Ms2Units, MPerS2);

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

define_unit_enum!(BallisticCoeffUnits, KgPerM2, { KgPerM2 => "kg/m**2" });

pub type BallisticCoeff = UnitValue<f64, BallisticCoeffUnits>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BallisticCoeffRequired {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@units")]
    pub units: BallisticCoeffUnits,
}
impl BallisticCoeffRequired {
    /// ballisticCoeffType: nonNegativeDouble, units required
    pub fn new(value: f64) -> Result<Self> {
        if value < 0.0 {
            return Err(CcsdsNdmError::Validation(format!(
                "Ballistic Coeff must be >= 0: {}",
                value
            )));
        }
        Ok(Self {
            value,
            units: BallisticCoeffUnits::KgPerM2,
        })
    }
}
impl std::fmt::Display for BallisticCoeffRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

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
            return Err(CcsdsNdmError::Validation(format!(
                "Percentage out of range: {}",
                value
            )));
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
impl FromKvn for Percentage {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, PercentageUnits>::from_kvn(value, unit)?;
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
            return Err(CcsdsNdmError::Validation(format!(
                "PercentageRequired out of range: {}",
                value
            )));
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
impl FromKvn for PercentageRequired {
    fn from_kvn(value: &str, _unit: Option<&str>) -> Result<Self> {
        let v: f64 = value
            .parse()
            .map_err(|e: std::num::ParseFloatError| CcsdsNdmError::KvnParse(e.to_string()))?;
        Self::new(v)
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
            return Err(CcsdsNdmError::Validation(format!(
                "Probability out of range: {}",
                value
            )));
        }
        Ok(Self { value })
    }
}
impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
            return Err(CcsdsNdmError::Validation(
                "DeltaMass must be negative".into(),
            ));
        }
        Ok(Self { value, units })
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
            return Err(CcsdsNdmError::Validation("DeltaMassZ must be <= 0".into()));
        }
        Ok(Self { value, units })
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
            return Err(CcsdsNdmError::Validation(format!(
                "Latitude out of range [-90,90]: {}",
                value
            )));
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
            return Err(CcsdsNdmError::Validation(format!(
                "Longitude out of range [-180,180]: {}",
                value
            )));
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
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PAYLOAD" => Ok(Self::Payload),
            "payload" => Ok(Self::PayloadLower),
            "ROCKET BODY" => Ok(Self::RocketBody),
            "rocket body" => Ok(Self::RocketBodyLower),
            "DEBRIS" => Ok(Self::Debris),
            "debris" => Ok(Self::DebrisLower),
            "UNKNOWN" => Ok(Self::Unknown),
            "unknown" => Ok(Self::UnknownLower),
            "OTHER" => Ok(Self::Other),
            "other" => Ok(Self::OtherLower),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown OBJECT_TYPE: {}",
                s
            ))),
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
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "YES" | "yes" => Ok(YesNo::Yes),
            "NO" | "no" => Ok(YesNo::No),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown YES/NO value: {}",
                s
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TrajBasis {
    #[serde(rename = "PREDICTED")]
    Predicted,
    #[serde(rename = "DETERMINED")]
    Determined,
    #[serde(rename = "TELEMETRY")]
    Telemetry,
    #[serde(rename = "SIMULATED")]
    Simulated,
    #[serde(rename = "OTHER")]
    Other,
}

impl std::str::FromStr for TrajBasis {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PREDICTED" => Ok(Self::Predicted),
            "DETERMINED" => Ok(Self::Determined),
            "TELEMETRY" => Ok(Self::Telemetry),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown TRAJ_BASIS: {}",
                s
            ))),
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

impl std::str::FromStr for RevNumBasis {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown ORB_REVNUM_BASIS: {}",
                s
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CovBasis {
    #[serde(rename = "PREDICTED")]
    Predicted,
    #[serde(rename = "DETERMINED")]
    Determined,
    #[serde(rename = "EMPIRICAL")]
    Empirical,
    #[serde(rename = "SIMULATED")]
    Simulated,
    #[serde(rename = "OTHER")]
    Other,
}

impl std::str::FromStr for CovBasis {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "PREDICTED" => Ok(Self::Predicted),
            "DETERMINED" => Ok(Self::Determined),
            "EMPIRICAL" => Ok(Self::Empirical),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown COV_BASIS: {}",
                s
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ManBasis {
    #[serde(rename = "CANDIDATE")]
    Candidate,
    #[serde(rename = "PLANNED")]
    Planned,
    #[serde(rename = "ANTICIPATED")]
    Anticipated,
    #[serde(rename = "TELEMETRY")]
    Telemetry,
    #[serde(rename = "DETERMINED")]
    Determined,
    #[serde(rename = "SIMULATED")]
    Simulated,
    #[serde(rename = "OTHER")]
    Other,
}

impl std::str::FromStr for ManBasis {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "CANDIDATE" => Ok(Self::Candidate),
            "PLANNED" => Ok(Self::Planned),
            "ANTICIPATED" => Ok(Self::Anticipated),
            "TELEMETRY" => Ok(Self::Telemetry),
            "DETERMINED" => Ok(Self::Determined),
            "SIMULATED" => Ok(Self::Simulated),
            "OTHER" => Ok(Self::Other),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown MAN_BASIS: {}",
                s
            ))),
        }
    }
}

/// Maneuver duty cycle type per XSD dcTypeType.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum ManDc {
    #[default]
    #[serde(rename = "CONTINUOUS")]
    Continuous,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "TIME_AND_ANGLE")]
    TimeAndAngle,
}

impl std::str::FromStr for ManDc {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "CONTINUOUS" => Ok(Self::Continuous),
            "TIME" => Ok(Self::Time),
            "TIME_AND_ANGLE" => Ok(Self::TimeAndAngle),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown DC_TYPE: {}",
                s
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum CovOrder {
    #[default]
    #[serde(rename = "LTM")]
    Ltm,
    #[serde(rename = "UTM")]
    Utm,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "LTMWCC")]
    LtmWcc,
    #[serde(rename = "UTMWCC")]
    UtmWcc,
}

impl std::str::FromStr for CovOrder {
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "LTM" => Ok(Self::Ltm),
            "UTM" => Ok(Self::Utm),
            "FULL" => Ok(Self::Full),
            "LTMWCC" => Ok(Self::LtmWcc),
            "UTMWCC" => Ok(Self::UtmWcc),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown COV_ORDERING: {}",
                s
            ))),
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
    type Err = crate::error::CcsdsNdmError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "YES" | "yes" => Ok(ControlledType::Yes),
            "NO" | "no" => Ok(ControlledType::No),
            "UNKNOWN" | "unknown" => Ok(ControlledType::Unknown),
            _ => Err(crate::error::CcsdsNdmError::UnsupportedFormat(format!(
                "Unknown CONTROLLED_TYPE value: {}",
                s
            ))),
        }
    }
}

// Time units ("s") plus Duration / RelTime / TimeOffset (optional units per XSD)
define_unit_enum!(TimeUnits, Seconds, { Seconds => "s" });

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
            return Err(CcsdsNdmError::Validation(format!(
                "Duration must be >= 0: {}",
                value
            )));
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
impl FromKvn for Duration {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, TimeUnits>::from_kvn(value, unit)?;
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

impl FromKvn for TimeOffset {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, TimeUnits>::from_kvn(value, unit)?;
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
            return Err(CcsdsNdmError::Validation(format!(
                "Inclination out of range: {}",
                value
            )));
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
impl FromKvn for Inclination {
    fn from_kvn(value: &str, unit: Option<&str>) -> Result<Self> {
        let uv = UnitValue::<f64, AngleUnits>::from_kvn(value, unit)?;
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
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "@units", default, skip_serializing_if = "Option::is_none")]
    pub units: Option<AngleUnits>,
}

// DisintegrationType
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DisintegrationType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MASS-LOSS")]
    MassLoss,
    #[serde(rename = "BREAK-UP")]
    BreakUp,
    #[serde(rename = "MASS-LOSS + BREAK-UP")]
    MassLossAndBreakUp,
}

// ImpactUncertaintyType per XSD
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ImpactUncertaintyType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    #[serde(rename = "STOCHASTIC")]
    Stochastic,
    #[serde(rename = "EMPIRICAL")]
    Empirical,
}

// ReentryUncertaintyMethodType per XSD
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ReentryUncertaintyMethodType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    #[serde(rename = "STOCHASTIC")]
    Stochastic,
    #[serde(rename = "EMPIRICAL")]
    Empirical,
}

// TimeSystemType: XSD has empty restriction; represent as a string newtype.
/// Time system string constrained externally by schema usage (e.g., TDB, UTC).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TimeSystemType(pub String);

// AngVelFrameType: XSD empty restriction (free-form string), used in APM angVelStateType.
/// Angular velocity frame identifier (schema leaves unrestricted).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AngVelFrameType(pub String);

/// USER DEFINED PARAMETERS block (`userDefinedType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDefined {
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<String>,
    #[serde(
        rename = "USER_DEFINED",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_defined: Vec<UserDefinedParameter>,
}

/// Single USER_DEFINED parameter (`userDefinedParameterType`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserDefinedParameter {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "@parameter")]
    pub parameter: String,
}

// -------------------- CDM TYPES --------------------

// Velocity delta-v units (m/s) and type (`dvType`)
define_unit_enum!(DvUnits, MPerS, { MPerS => "m/s" });
pub type Dv = UnitValue<f64, DvUnits>;

// m**2 units and type (`m2Type`)
define_unit_type!(M2, M2Units, M2, { M2 => "m**2" });

// m**2/s units and type (`m2sType`)
define_unit_type!(M2s, M2sUnits, M2PerS, { M2PerS => "m**2/s" });

// m**2/s**2 units and type (`m2s2Type`)
define_unit_type!(M2s2, M2s2Units, M2PerS2, { M2PerS2 => "m**2/s**2" });

// m**3/kg units and type (`m3kgType`)
define_unit_type!(M3kg, M3kgUnits, M3PerKg, { M3PerKg => "m**3/kg" });

// m**3/(kg*s) units and type (`m3kgsType`)
define_unit_type!(M3kgs, M3kgsUnits, M3PerKgS, { M3PerKgS => "m**3/(kg*s)" });

// m**4/kg**2 units and type (`m4kg2Type`)
define_unit_type!(M4kg2, M4kg2Units, M4PerKg2, { M4PerKg2 => "m**4/kg**2" });

// m**2/s**3 units and type (`m2s3Type`)
define_unit_type!(M2s3, M2s3Units, M2PerS3, { M2PerS3 => "m**2/s**3" });

// m**3/(kg*s**2) units and type (`m3kgs2Type`)
define_unit_type!(M3kgs2, M3kgs2Units, M3PerKgS2, { M3PerKgS2 => "m**3/(kg*s**2)" });

// m**2/s**4 units and type (`m2s4Type`)
define_unit_type!(M2s4, M2s4Units, M2PerS4, { M2PerS4 => "m**2/s**4" });

// m**2/kg units and type (`m2kgType`)
define_unit_type!(M2kg, M2kgUnits, M2PerKg, { M2PerKg => "m**2/kg" });

// CDM categorical simple types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CdmObjectType {
    #[serde(rename = "OBJECT1")]
    Object1,
    #[serde(rename = "object1")]
    Object1Lower,
    #[serde(rename = "OBJECT2")]
    Object2,
    #[serde(rename = "object2")]
    Object2Lower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ScreenVolumeFrameType {
    #[serde(rename = "RTN")]
    Rtn,
    #[serde(rename = "rtn")]
    RtnLower,
    #[serde(rename = "TVN")]
    Tvn,
    #[serde(rename = "tvn")]
    TvnLower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ScreenVolumeShapeType {
    #[serde(rename = "ELLIPSOID")]
    Ellipsoid,
    #[serde(rename = "ellipsoid")]
    EllipsoidLower,
    #[serde(rename = "BOX")]
    Box,
    #[serde(rename = "box")]
    BoxLower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ReferenceFrameType {
    #[serde(rename = "EME2000")]
    Eme2000,
    #[serde(rename = "eme2000")]
    Eme2000Lower,
    #[serde(rename = "GCRF")]
    Gcrf,
    #[serde(rename = "gcrf")]
    GcrfLower,
    #[serde(rename = "ITRF")]
    Itrf,
    #[serde(rename = "itrf")]
    ItrfLower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CovarianceMethodType {
    #[serde(rename = "CALCULATED")]
    Calculated,
    #[serde(rename = "calculated")]
    CalculatedLower,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "default")]
    DefaultLower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ManeuverableType {
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "yes")]
    YesLower,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "no")]
    NoLower,
    #[serde(rename = "N/A")]
    NA,
    #[serde(rename = "n/a")]
    NALower,
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
            return Err(CcsdsNdmError::KvnParse(format!(
                "Vec3Double requires 3 values, got {}: {}",
                parts.len(),
                val
            )));
        }
        let x = parts[0]
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid Vec3Double x: {}", e)))?;
        let y = parts[1]
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid Vec3Double y: {}", e)))?;
        let z = parts[2]
            .parse::<f64>()
            .map_err(|e| CcsdsNdmError::KvnParse(format!("Invalid Vec3Double z: {}", e)))?;
        Ok(Self { x, y, z })
    }
}

impl std::fmt::Display for Vec3Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
