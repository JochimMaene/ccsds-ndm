// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Utility functions and serialization helpers for CCSDS NDM.

use serde::{Deserializer, Serializer};

/// XML integer value equality includes `-0`; retain the native unsigned storage bound.
pub(crate) struct XmlU32(pub u32);

impl<'de> serde::Deserialize<'de> for XmlU32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = XmlU32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an unsigned 32-bit integer")
            }

            fn visit_str<E: serde::de::Error>(self, text: &str) -> Result<XmlU32, E> {
                let token = text.trim_matches([' ', '\t', '\r', '\n']);
                let negative_zero = token
                    .strip_prefix('-')
                    .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b == b'0'));
                if negative_zero {
                    Ok(XmlU32(0))
                } else {
                    token.parse().map(XmlU32).map_err(E::custom)
                }
            }

            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<XmlU32, E> {
                u32::try_from(value).map(XmlU32).map_err(E::custom)
            }

            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<XmlU32, E> {
                u32::try_from(value).map(XmlU32).map_err(E::custom)
            }

            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> Result<XmlU32, M::Error> {
                let key = map.next_key::<String>()?;
                if !matches!(key.as_deref(), Some("$text" | "$value")) {
                    return Err(serde::de::Error::custom("expected integer element text"));
                }
                let value = map.next_value::<XmlU32>()?;
                if map.next_key::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom("unexpected content after integer"));
                }
                Ok(value)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
pub(crate) fn deserialize_optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    Option::<XmlU32>::deserialize(deserializer).map(|value| value.map(|value| value.0))
}

/// Deserialize a value from the deserializer's own view of the token, without an owned `String`.
///
/// Every caller parses a short lexical token into a fixed-size or numeric value, so routing
/// through `String` puts one heap allocation on each record of a large history.
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
