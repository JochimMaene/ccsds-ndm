// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use crate::kvn::de::{KvnLine, KvnTokenizer};
use serde::de::{self, IntoDeserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::iter::Peekable;

/// Context for the KVN Deserializer to handle implied transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
enum KvnContext<'de> {
    Root,
    Header,
    Body,
    Segment,
    Metadata,
    Data,
    StateVector,
    KeplerianElements,
    MeanElements,
    TleParameters,
    SpacecraftParameters,
    CovarianceMatrix,
    ManeuverParameters,
    UserDefined,
    UserDefinedParameter,
    UnitValue, // Handling $value and @units
    SyntheticValue(&'de str),
}

pub struct Deserializer<'de> {
    tokenizer: Peekable<KvnTokenizer<'de>>,
    context_stack: Vec<KvnContext<'de>>,
    // Store the last version key seen (e.g., CCSDS_OPM_VERS)
    version_key: Option<&'de str>,
    version_val: Option<&'de str>,
    // Current pair being processed (for UnitValue expansion)
    pub(crate) current_pair: Option<KvnLine<'de>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            tokenizer: KvnTokenizer::new(input).peekable(),
            context_stack: vec![KvnContext::Root],
            version_key: None,
            version_val: None,
            current_pair: None,
        }
    }

    fn peek(&mut self) -> Result<Option<KvnLine<'de>>> {
        loop {
            match self.tokenizer.peek() {
                Some(Ok(KvnLine::Empty { .. })) | Some(Ok(KvnLine::Comment { .. })) => {
                    self.tokenizer.next();
                }
                Some(Ok(line)) => return Ok(Some(line.clone())),
                Some(Err(_)) => {
                    return Err(self.tokenizer.next().unwrap().unwrap_err());
                }
                None => return Ok(None),
            }
        }
    }

    fn next(&mut self) -> Result<Option<KvnLine<'de>>> {
        while let Some(res) = self.tokenizer.next() {
            match res {
                Ok(KvnLine::Empty { .. }) | Ok(KvnLine::Comment { .. }) => continue,
                Ok(line) => return Ok(Some(line)),
                Err(e) => return Err(e),
            }
        }
        Ok(None)
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = CcsdsNdmError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            if let Ok(f) = val.parse::<f64>() {
                return visitor.visit_f64(f);
            }
            return visitor.visit_str(val);
        }

        match self.peek()? {
            Some(KvnLine::Pair { .. }) => self.deserialize_map(visitor),
            _ => Err(<CcsdsNdmError as de::Error>::custom(
                "KvnDeserializer: deserialize_any not implemented for this state",
            )),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match name {
            "Opm" | "opm" | "Omm" | "omm" => {
                self.context_stack.push(KvnContext::Root);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OdmHeader" | "Header" | "TdmHeader" | "OmmHeader" => {
                self.context_stack.push(KvnContext::Header);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OpmBody" | "OmmBody" | "Body" => {
                self.context_stack.push(KvnContext::Body);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OpmSegment" | "OmmSegment" | "Segment" => {
                self.context_stack.push(KvnContext::Segment);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OpmMetadata" | "OmmMetadata" | "Metadata" => {
                self.context_stack.push(KvnContext::Metadata);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OpmData" | "OmmData" | "Data" => {
                self.context_stack.push(KvnContext::Data);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "StateVector" => {
                self.context_stack.push(KvnContext::StateVector);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "KeplerianElements" => {
                self.context_stack.push(KvnContext::KeplerianElements);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "MeanElements" => {
                self.context_stack.push(KvnContext::MeanElements);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "TleParameters" => {
                self.context_stack.push(KvnContext::TleParameters);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "SpacecraftParameters" => {
                self.context_stack.push(KvnContext::SpacecraftParameters);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "OpmCovarianceMatrix" | "OmmCovarianceMatrix" | "OdmCovarianceMatrix" => {
                self.context_stack.push(KvnContext::CovarianceMatrix);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "ManeuverParameters" => {
                self.context_stack.push(KvnContext::ManeuverParameters);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "UserDefined" => {
                self.context_stack.push(KvnContext::UserDefined);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "UserDefinedParameter" => {
                self.context_stack.push(KvnContext::UserDefinedParameter);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            "UnitValue" | "Angle" | "DayInterval" | "Frequency" | "Gm" | "AltitudeRequired"
            | "BallisticCoeffRequired" | "Percentage" | "PercentageRequired" | "Mass" | "Area"
            | "PositionRequired" | "VelocityRequired" | "DayIntervalRequired" | "WkgRequired"
            | "Ms2Required" | "SensorNoise" | "DeltaMassZRaw" | "Duration" | "RelTime" => {
                self.context_stack.push(KvnContext::UnitValue);
                let val = visitor.visit_map(KvnMapAccess::new(self))?;
                self.context_stack.pop();
                Ok(val)
            }
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(KvnMapAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            return visitor.visit_str(val);
        }

        if let Some(KvnContext::UnitValue) = self.context_stack.last() {
            return Err(<CcsdsNdmError as de::Error>::custom(
                "KvnDeserializer: deserialize_str called in UnitValue context unexpectedly",
            ));
        }

        if let Some(KvnLine::Pair { val, .. }) = self.next()? {
            visitor.visit_str(val)
        } else {
            Err(<CcsdsNdmError as de::Error>::custom(
                "Expected KVN pair for string value",
            ))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            let f = val
                .parse::<f64>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            return visitor.visit_f64(f);
        }

        if let Some(KvnLine::Pair { val, .. }) = self.next()? {
            let f = val
                .parse::<f64>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            visitor.visit_f64(f)
        } else {
            Err(<CcsdsNdmError as de::Error>::custom(
                "Expected KVN pair for f64 value",
            ))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            let b = match val.to_uppercase().as_str() {
                "YES" | "TRUE" | "1" => true,
                "NO" | "FALSE" | "0" => false,
                _ => {
                    return Err(<CcsdsNdmError as de::Error>::custom(format!(
                        "Invalid boolean: {}",
                        val
                    )))
                }
            };
            return visitor.visit_bool(b);
        }
        if let Some(KvnLine::Pair { val, .. }) = self.next()? {
            let b = match val.to_uppercase().as_str() {
                "YES" | "TRUE" | "1" => true,
                "NO" | "FALSE" | "0" => false,
                _ => {
                    return Err(<CcsdsNdmError as de::Error>::custom(format!(
                        "Invalid boolean: {}",
                        val
                    )))
                }
            };
            visitor.visit_bool(b)
        } else {
            Err(<CcsdsNdmError as de::Error>::custom(
                "Expected KVN pair for bool",
            ))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            let n = val
                .parse::<i32>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            return visitor.visit_i32(n);
        }
        if let Some(KvnLine::Pair { val, .. }) = self.next()? {
            let n = val
                .parse::<i32>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            visitor.visit_i32(n)
        } else {
            Err(<CcsdsNdmError as de::Error>::custom(
                "Expected KVN pair for i32",
            ))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(KvnContext::SyntheticValue(val)) = self.context_stack.last() {
            let n = val
                .parse::<u32>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            return visitor.visit_u32(n);
        }
        if let Some(KvnLine::Pair { val, .. }) = self.next()? {
            let n = val
                .parse::<u32>()
                .map_err(<CcsdsNdmError as de::Error>::custom)?;
            visitor.visit_u32(n)
        } else {
            Err(<CcsdsNdmError as de::Error>::custom(
                "Expected KVN pair for u32",
            ))
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.next()?;
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(KvnSeqAccess::new(self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(KvnEnumAccess::new(self))
    }

    serde::forward_to_deserialize_any! {
        i8 i16 i64 i128 u8 u16 u64 u128 f32 char
        bytes byte_buf
        tuple tuple_struct
    }
}

struct KvnMapAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    current_implied_field: Option<ImpliedField>,
    seen_keys: Vec<&'de str>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ImpliedField {
    Version,
    Id,
    Header,
    Body,
    Data,
    // Data sub-fields
    StateVector,
    KeplerianElements,
    MeanElements,
    TleParameters,
    SpacecraftParameters,
    CovarianceMatrix,
    ManeuverParameters,
    UserDefinedParameters,
    Done,
    // UnitValue implied fields
    UnitValueValue,
    UnitValueUnits,
}

impl<'a, 'de> KvnMapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        KvnMapAccess {
            de,
            current_implied_field: None,
            seen_keys: Vec::new(),
        }
    }
}

impl<'de, 'a> MapAccess<'de> for KvnMapAccess<'a, 'de> {
    type Error = CcsdsNdmError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        let context = *self.de.context_stack.last().unwrap();

        match context {
            KvnContext::Root => match self.current_implied_field {
                None => {
                    if let Some(KvnLine::Pair { key, val, .. }) = self.de.peek()? {
                        if key.ends_with("_VERS") {
                            self.de.version_key = Some(key);
                            self.de.version_val = Some(val);
                            self.current_implied_field = Some(ImpliedField::Version);
                            let de: de::value::StrDeserializer<CcsdsNdmError> = "@version".into_deserializer();
                            return seed.deserialize(de).map(Some);
                        }
                    }
                    Ok(None)
                }
                Some(ImpliedField::Version) => {
                    self.current_implied_field = Some(ImpliedField::Id);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "@id".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                Some(ImpliedField::Id) => {
                    self.current_implied_field = Some(ImpliedField::Header);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "header".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                Some(ImpliedField::Header) => {
                    self.current_implied_field = Some(ImpliedField::Body);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "body".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                _ => Ok(None),
            },
            KvnContext::Header => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_header_key(key) {
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::Body => {
                if self.current_implied_field.is_none() {
                    self.current_implied_field = Some(ImpliedField::Done);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "segment".into_deserializer();
                    return seed.deserialize(de).map(Some);
                }
                Ok(None)
            }
            KvnContext::Segment => match self.current_implied_field {
                None => {
                    self.current_implied_field = Some(ImpliedField::Data);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "metadata".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                Some(ImpliedField::Data) => {
                    self.current_implied_field = Some(ImpliedField::Done);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "data".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                _ => Ok(None),
            },
            KvnContext::Metadata => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if !is_data_transition(key) {
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::Data => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    let is_omm = self
                        .de
                        .version_key
                        .map(|k| k.contains("OMM"))
                        .unwrap_or(false);
                    let mapped = if is_state_vector_key(key) && !is_omm {
                        ImpliedField::StateVector
                    } else if is_keplerian_key(key) && !is_omm {
                        ImpliedField::KeplerianElements
                    } else if is_mean_elements_key(key) && is_omm {
                        ImpliedField::MeanElements
                    } else if is_tle_key(key) && is_omm {
                        ImpliedField::TleParameters
                    } else if is_spacecraft_key(key) {
                        ImpliedField::SpacecraftParameters
                    } else if is_covariance_key(key) {
                        ImpliedField::CovarianceMatrix
                    } else if key.starts_with("MAN_") {
                        ImpliedField::ManeuverParameters
                    } else if key.starts_with("USER_DEFINED_") {
                        ImpliedField::UserDefinedParameters
                    } else {
                        return Ok(None);
                    };

                    if Some(mapped) != self.current_implied_field {
                        self.current_implied_field = Some(mapped);
                        let field_name = match mapped {
                            ImpliedField::StateVector => "stateVector",
                            ImpliedField::KeplerianElements => "keplerianElements",
                            ImpliedField::MeanElements => "meanElements",
                            ImpliedField::TleParameters => "tleParameters",
                            ImpliedField::SpacecraftParameters => "spacecraftParameters",
                            ImpliedField::CovarianceMatrix => "covarianceMatrix",
                            ImpliedField::ManeuverParameters => "maneuverParameters",
                            ImpliedField::UserDefinedParameters => "userDefined",
                            _ => unreachable!(),
                        };
                        let de: de::value::StrDeserializer<CcsdsNdmError> = field_name.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::StateVector => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_state_vector_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::KeplerianElements => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_keplerian_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::SpacecraftParameters => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_spacecraft_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::MeanElements => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_mean_elements_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::TleParameters => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_tle_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::CovarianceMatrix => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if is_covariance_key(key) && !self.seen_keys.contains(&key) {
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::ManeuverParameters => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    // Stop if we hit a key already seen (signaling start of next maneuver block)
                    if key.starts_with("MAN_") {
                        if self.seen_keys.contains(&key) {
                            return Ok(None);
                        }
                        self.seen_keys.push(key);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = key.into_deserializer();
                        return seed.deserialize(de).map(Some);
                    }
                }
                Ok(None)
            }
            KvnContext::UserDefined => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if key.starts_with("USER_DEFINED_") {
                        if !self.seen_keys.contains(&"userDefinedParameter") {
                            self.seen_keys.push("userDefinedParameter");
                            let de: de::value::StrDeserializer<CcsdsNdmError> =
                                "userDefinedParameter".into_deserializer();
                            return seed.deserialize(de).map(Some);
                        }
                    }
                }
                Ok(None)
            }
            KvnContext::UserDefinedParameter => {
                if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
                    if key.starts_with("USER_DEFINED_") {
                        if !self.seen_keys.contains(&"parameter") {
                            self.seen_keys.push("parameter");
                            let de: de::value::StrDeserializer<CcsdsNdmError> =
                                "parameter".into_deserializer();
                            return seed.deserialize(de).map(Some);
                        }
                        if !self.seen_keys.contains(&"value") {
                            self.seen_keys.push("value");
                            let de: de::value::StrDeserializer<CcsdsNdmError> =
                                "value".into_deserializer();
                            return seed.deserialize(de).map(Some);
                        }
                    }
                }
                Ok(None)
            }
            KvnContext::UnitValue => match self.current_implied_field {
                None => {
                    self.current_implied_field = Some(ImpliedField::UnitValueUnits);
                    let de: de::value::StrDeserializer<CcsdsNdmError> = "$value".into_deserializer();
                    seed.deserialize(de).map(Some)
                }
                Some(ImpliedField::UnitValueUnits) => {
                    if let Some(KvnLine::Pair { unit: Some(_), .. }) = &self.de.current_pair {
                        self.current_implied_field = Some(ImpliedField::Done);
                        let de: de::value::StrDeserializer<CcsdsNdmError> = "@units".into_deserializer();
                        seed.deserialize(de).map(Some)
                    } else {
                        self.de.next()?; // Consume it here if no units
                        Ok(None)
                    }
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let context = *self.de.context_stack.last().unwrap();
        match context {
            KvnContext::Root => match self.current_implied_field {
                Some(ImpliedField::Version) => {
                    let val = self.de.version_val.unwrap();
                    self.de.next()?; // Consume the version pair
                    let de: de::value::StrDeserializer<CcsdsNdmError> = val.into_deserializer();
                    seed.deserialize(de)
                }
                Some(ImpliedField::Id) => {
                    let key = self.de.version_key.unwrap();
                    self.de.context_stack.push(KvnContext::SyntheticValue(key));
                    let res = seed.deserialize(&mut *self.de);
                    self.de.context_stack.pop();
                    res
                }
                _ => seed.deserialize(&mut *self.de),
            },
            KvnContext::Header
            | KvnContext::Metadata
            | KvnContext::Data
            | KvnContext::StateVector
            | KvnContext::KeplerianElements
            | KvnContext::MeanElements
            | KvnContext::TleParameters
            | KvnContext::SpacecraftParameters
            | KvnContext::CovarianceMatrix
            | KvnContext::ManeuverParameters
            | KvnContext::UserDefined => {
                if let Some(line) = self.de.peek()? {
                    self.de.current_pair = Some(line);
                }
                seed.deserialize(&mut *self.de)
            }
            KvnContext::UserDefinedParameter => {
                if let Some(KvnLine::Pair { key, val, .. }) = self.de.peek()? {
                    let synth = if self.seen_keys.last() == Some(&"parameter") {
                        key
                    } else {
                        val
                    };
                    if self.seen_keys.last() == Some(&"value") {
                        self.de.next()?; // Consume the pair after we've returned both fields
                    }
                    self.de.context_stack.push(KvnContext::SyntheticValue(synth));
                    let res = seed.deserialize(&mut *self.de);
                    self.de.context_stack.pop();
                    res
                } else {
                    seed.deserialize(&mut *self.de)
                }
            }
            KvnContext::UnitValue => match self.current_implied_field {
                Some(ImpliedField::UnitValueUnits) | Some(ImpliedField::UnitValueValue) => {
                    if let Some(KvnLine::Pair { val, .. }) = &self.de.current_pair {
                        self.de.context_stack.push(KvnContext::SyntheticValue(val));
                        let res = seed.deserialize(&mut *self.de);
                        self.de.context_stack.pop();
                        res
                    } else {
                        Err(<CcsdsNdmError as de::Error>::custom(
                            "UnitValue context without current pair",
                        ))
                    }
                }
                Some(ImpliedField::Done) => {
                    if let Some(KvnLine::Pair { unit: Some(u), .. }) = &self.de.current_pair {
                        self.de.context_stack.push(KvnContext::SyntheticValue(u));
                        let res = seed.deserialize(&mut *self.de);
                        self.de.context_stack.pop();
                        self.de.next()?; // NOW consume it
                        res
                    } else {
                        Err(<CcsdsNdmError as de::Error>::custom(
                            "UnitValue context without unit but reached @units",
                        ))
                    }
                }
                _ => seed.deserialize(&mut *self.de),
            },
            _ => {
                if let Some(KvnContext::UnitValue) = self.de.context_stack.last() {
                    match self.current_implied_field {
                        Some(ImpliedField::UnitValueUnits) => {
                             if let Some(KvnLine::Pair { unit: None, .. }) = &self.de.current_pair {
                                 let res = seed.deserialize(&mut *self.de);
                                 self.de.next()?;
                                 return res;
                             }
                        }
                        _ => {}
                    }
                }
                seed.deserialize(&mut *self.de)
            }
        }
    }
}

fn is_header_key(key: &str) -> bool {
    matches!(
        key,
        "CREATION_DATE" | "ORIGINATOR" | "MESSAGE_ID" | "CLASSIFICATION"
    )
}

fn is_data_transition(key: &str) -> bool {
    is_state_vector_key(key)
        || is_keplerian_key(key)
        || is_mean_elements_key(key)
        || is_tle_key(key)
        || is_spacecraft_key(key)
        || is_covariance_key(key)
        || key.starts_with("MAN_")
        || key.starts_with("USER_DEFINED_")
}

fn is_state_vector_key(key: &str) -> bool {
    matches!(key, "EPOCH" | "X" | "Y" | "Z" | "X_DOT" | "Y_DOT" | "Z_DOT")
}

fn is_keplerian_key(key: &str) -> bool {
    matches!(
        key,
        "SEMI_MAJOR_AXIS"
            | "ECCENTRICITY"
            | "INCLINATION"
            | "RA_OF_ASC_NODE"
            | "ARG_OF_PERICENTER"
            | "TRUE_ANOMALY"
            | "MEAN_ANOMALY"
            | "GM"
    )
}

fn is_mean_elements_key(key: &str) -> bool {
    matches!(
        key,
        "EPOCH"
            | "SEMI_MAJOR_AXIS"
            | "MEAN_MOTION"
            | "ECCENTRICITY"
            | "INCLINATION"
            | "RA_OF_ASC_NODE"
            | "ARG_OF_PERICENTER"
            | "MEAN_ANOMALY"
            | "GM"
    )
}

fn is_tle_key(key: &str) -> bool {
    matches!(
        key,
        "EPHEMERIS_TYPE"
            | "CLASSIFICATION_TYPE"
            | "NORAD_CAT_ID"
            | "ELEMENT_SET_NO"
            | "REV_AT_EPOCH"
            | "BSTAR"
            | "BTERM"
            | "AGOM"
            | "MEAN_MOTION_DOT"
            | "MEAN_MOTION_DDOT"
    )
}

fn is_spacecraft_key(key: &str) -> bool {
    matches!(
        key,
        "MASS" | "SOLAR_RAD_AREA" | "SOLAR_RAD_COEFF" | "DRAG_AREA" | "DRAG_COEFF"
    )
}

fn is_covariance_key(key: &str) -> bool {
    key.starts_with("CX_")
        || key.starts_with("CY_")
        || key.starts_with("CZ_")
        || key == "COV_REF_FRAME"
}

struct KvnEnumAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> KvnEnumAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        KvnEnumAccess { de }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for KvnEnumAccess<'a, 'de> {
    type Error = CcsdsNdmError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> de::VariantAccess<'de> for KvnEnumAccess<'a, 'de> {
    type Error = CcsdsNdmError;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

struct KvnSeqAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> KvnSeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        KvnSeqAccess { de }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for KvnSeqAccess<'a, 'de> {
    type Error = CcsdsNdmError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(KvnLine::Pair { key, .. }) = self.de.peek()? {
            if key.starts_with("MAN_") || key.starts_with("USER_DEFINED_") {
                return seed.deserialize(&mut *self.de).map(Some);
            }
        }
        Ok(None)
    }
}
