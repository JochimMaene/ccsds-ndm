// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Utility functions and serialization helpers for CCSDS NDM.

use serde::{Deserializer, Serializer};

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
/// Displays values separated by single spaces, so serializers can stream them via
/// `collect_str` instead of building a joined `String`.
pub(crate) struct SpaceSeparated<'a, T>(pub(crate) &'a [T]);

impl<T: std::fmt::Display> std::fmt::Display for SpaceSeparated<'_, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, value) in self.0.iter().enumerate() {
            if index > 0 {
                formatter.write_str(" ")?;
            }
            value.fmt(formatter)?;
        }
        Ok(())
    }
}

pub mod vec_f64_space_sep {
    use super::*;

    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&super::SpaceSeparated(values))
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
