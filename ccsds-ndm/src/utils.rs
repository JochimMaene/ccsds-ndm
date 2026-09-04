// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Utility functions and serialization helpers for CCSDS NDM.

use serde::{Deserializer, Serializer};

/// Deserialize a value from the deserializer's own view of the token, without an owned `String`.
///
/// Every caller parses a short lexical token into a fixed-size or numeric value, so routing
/// through `String` puts one heap allocation on each record of a large history. Measured on
/// `xml_parse_oem_10k`, removing it from the epoch path alone cut parse time by about a quarter.
pub(crate) fn deserialize_parsed<'de, D, T, E>(
    deserializer: D,
    expecting: &'static str,
    parse: impl Fn(&str) -> Result<T, E>,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    E: std::fmt::Display,
{
    struct ParsedVisitor<F> {
        expecting: &'static str,
        parse: F,
    }

    impl<T, E, F> serde::de::Visitor<'_> for ParsedVisitor<F>
    where
        E: std::fmt::Display,
        F: Fn(&str) -> Result<T, E>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(self.expecting)
        }

        fn visit_str<A>(self, value: &str) -> Result<T, A>
        where
            A: serde::de::Error,
        {
            (self.parse)(value).map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_str(ParsedVisitor { expecting, parse })
}

/// Serialization helper for `Vec<f64>` that uses space separation.
pub mod vec_f64_space_sep {
    use super::*;

    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        super::deserialize_parsed(deserializer, "space-separated numbers", |s| {
            if s.trim().is_empty() {
                return Ok(Vec::new());
            }
            s.split_whitespace()
                .map(|part| part.parse::<f64>())
                .collect::<Result<Vec<f64>, _>>()
        })
    }
}

/// Unified helper module to deserialize optional fields that may be nil or empty.
///
/// In CCSDS XML, optional fields can be represented as:
/// - `<FIELD nil="true"/>` or `<FIELD xsi:nil="true"/>` — no content
/// - `<FIELD></FIELD>` — empty text content
/// - `<FIELD>value</FIELD>` — scalar value
/// - `<FIELD units="km">value</FIELD>` — map with attributes
pub mod nullable {
    use super::*;
    use serde::de::{self, IntoDeserializer, MapAccess, Visitor};
    use serde_json::{Map as JsonMap, Value as JsonValue};
    use std::marker::PhantomData;

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: serde::Serialize,
    {
        match value {
            Some(v) => v.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: serde::de::DeserializeOwned,
    {
        struct OptionVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for OptionVisitor<T>
        where
            T: serde::de::DeserializeOwned,
        {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional value, empty string, or nil-attributed element")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Some)
            }

            // KVN / XML scalar case
            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.trim().is_empty() {
                    return Ok(None);
                }
                if v.trim().eq_ignore_ascii_case("n/a") {
                    return Ok(None);
                }

                // Try treating as valid JSON value first (handles numbers like u32 "23581")
                if let Ok(val) = serde_json::from_str::<T>(v) {
                    return Ok(Some(val));
                }

                let de = v.into_deserializer();
                T::deserialize(de).map(Some)
            }

            // XML Attribute case (<Tag attr="...">val</Tag>)
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // Buffer all map entries to support flexible deserialization strategies.
                // Store values as JSON so numeric-like text can be retyped for structured targets.
                let mut values = JsonMap::new();
                let mut nil = false;
                let mut text_val = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "@nil" || key == "@xsi:nil" {
                        let v: String = map.next_value()?;
                        if v == "true" {
                            nil = true;
                        }
                    } else if key == "$text" || key == "$value" {
                        // Normalize content key to "$value" for UnitValue compatibility
                        let v: String = map.next_value()?;
                        text_val = Some(v.clone());
                        if let Ok(json_value) = serde_json::from_str::<JsonValue>(&v) {
                            values.insert("$value".to_string(), json_value);
                        } else {
                            values.insert("$value".to_string(), JsonValue::String(v));
                        }
                    } else {
                        let v: String = map.next_value()?;
                        if let Ok(json_value) = serde_json::from_str::<JsonValue>(&v) {
                            values.insert(key, json_value);
                        } else {
                            values.insert(key, JsonValue::String(v));
                        }
                    }
                }

                // A nilled element is absent regardless of what else it carries. XSD lets a
                // nillable element keep its attributes, and the schemas use that: `MASS`,
                // `TRUE_ANOMALY` and friends may be spelled `<MASS units="kg" xsi:nil="true"/>`.
                // Which attributes are legal on which element is the envelope validator's call,
                // so accepting them here is not a gap.
                if nil {
                    return Ok(None);
                }
                if values.is_empty() {
                    return Ok(None);
                }

                // Strategy 1: Deserialize T from object representation (handles structured wrappers).
                match serde_json::from_value::<T>(JsonValue::Object(values.clone())) {
                    Ok(val) => Ok(Some(val)),
                    Err(error) => {
                        // Strategy 2: Fallback to deserializing T as a Primitive (e.g., f64, u32)
                        // If T is a primitive, Strategy 1 fails because it receives a Map.
                        // We extract just the text content and try again.
                        if let Some(txt) = text_val {
                            // Try JSON parsing for numbers first
                            if let Ok(val) = serde_json::from_str::<T>(&txt) {
                                return Ok(Some(val));
                            }

                            let sd = txt.into_deserializer();
                            T::deserialize(sd).map(Some)
                        } else {
                            // An attribute-bearing element is not an empty optional value. Preserve
                            // the typed error instead of silently discarding invalid attributes.
                            Err(de::Error::custom(error))
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(OptionVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Duration, TimeUnits};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Wrapper {
        #[serde(with = "vec_f64_space_sep")]
        values: Vec<f64>,
    }

    #[test]
    fn test_vec_f64_space_sep_serialize() {
        let w = Wrapper {
            values: vec![1.1, 2.2, 3.3],
        };
        let s = serde_json::to_string(&w).unwrap();
        assert_eq!(s, r#"{"values":"1.1 2.2 3.3"}"#);
    }

    #[test]
    fn test_vec_f64_space_sep_deserialize() {
        let s = r#"{"values":"1.1 2.2 3.3"}"#;
        let w: Wrapper = serde_json::from_str(s).unwrap();
        assert_eq!(w.values, vec![1.1, 2.2, 3.3]);
    }

    #[test]
    fn test_vec_f64_space_sep_empty() {
        let w = Wrapper { values: vec![] };
        let s = serde_json::to_string(&w).unwrap();
        assert_eq!(s, r#"{"values":""}"#);

        let w2: Wrapper = serde_json::from_str(&s).unwrap();
        assert_eq!(w2.values, Vec::<f64>::new());
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        data: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct NullableTestWrapper {
        #[serde(with = "crate::utils::nullable", default)]
        field: Option<String>,
        #[serde(with = "crate::utils::nullable", default)]
        struct_field: Option<TestStruct>,
        #[serde(with = "crate::utils::nullable", default)]
        u32_field: Option<u32>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct NullableDurationWrapper {
        #[serde(with = "crate::utils::nullable", default)]
        duration_field: Option<Duration>,
    }

    #[test]
    fn test_nullable_deserialization() {
        // Test 1: Explicit nil="true"
        let json = r#"{ "field": { "@nil": "true", "$text": "some text" } }"#;
        let w: NullableTestWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.field, None);

        // Test 2: Whitespace text
        let json = r#"{ "field": "   " }"#;
        let w: NullableTestWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.field, None);

        // Test 3: "n/a" text
        let json = r#"{ "field": "n/a" }"#;
        let w: NullableTestWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.field, None);

        // Test 4: u32 from string (fixes OMM regression)
        // Note: JSON input "23581" (string) for u32 field?
        let json = r#"{ "u32_field": "23581" }"#;
        let w: NullableTestWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.u32_field, Some(23581));

        // Test 5: u32 from Map (XML style)
        let json = r#"{ "u32_field": { "$text": "23581" } }"#;
        let w: NullableTestWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.u32_field, Some(23581));
    }

    #[test]
    fn test_nullable_structured_numeric_value() {
        let json = r#"{ "duration_field": { "$value": "1", "@units": "s" } }"#;
        let w: NullableDurationWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(
            w.duration_field,
            Some(Duration {
                value: 1.0,
                units: Some(TimeUnits::Seconds),
            })
        );
    }

    #[test]
    fn nullable_accepts_attributes_alongside_nil() {
        // XSD lets a nillable element keep its attributes, and the ODM schemas use that: a
        // nilled element is absent no matter what else it carries.
        let json = r#"{ "duration_field": { "@nil": "true", "@units": "s" } }"#;
        let w: NullableDurationWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.duration_field, None);
    }

    #[test]
    fn nullable_does_not_discard_attributes_on_a_valueless_element() {
        // Without `nil` the element is present, so an unusable attribute set is an error rather
        // than a silently absent value.
        let json = r#"{ "duration_field": { "@units": "km" } }"#;
        assert!(serde_json::from_str::<NullableDurationWrapper>(json).is_err());
    }
}
