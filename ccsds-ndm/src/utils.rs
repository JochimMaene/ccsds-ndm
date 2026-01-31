// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

/// Unified helper module to deserialize optional fields that may be nil or empty.
///
/// In CCSDS XML, optional fields can be represented as:
/// - `<FIELD nil="true"/>` or `<FIELD xsi:nil="true"/>` — no content
/// - `<FIELD></FIELD>` — empty text content
///
/// This module handles both cases by returning `None` when:
/// 1. The `@nil` attribute is "true"
/// 2. The text content is empty (for enum/string types)
pub mod nullable {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Intermediate struct that captures nil attribute, text content, and value.
    #[derive(Deserialize)]
    struct NullableWrapper<T> {
        #[serde(rename = "@nil", default)]
        nil: Option<String>,
        #[serde(rename = "$text", default)]
        text: Option<String>,
        #[serde(flatten)]
        value: Option<T>,
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let wrapper: Option<NullableWrapper<T>> = Option::deserialize(deserializer)?;
        match wrapper {
            None => Ok(None),
            Some(w) => {
                // If nil="true", return None regardless of other content
                if let Some(ref nil) = w.nil {
                    if nil == "true" {
                        return Ok(None);
                    }
                }
                // If text content is empty, return None (handles enums with no value)
                if let Some(ref text) = w.text {
                    if text.trim().is_empty() {
                        return Ok(None);
                    }
                }
                Ok(w.value)
            }
        }
    }

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.serialize(serializer)
    }
}

pub mod vec_f64_space_sep {
    use serde::{Deserialize, Deserializer, Serializer};

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
        let s = String::deserialize(deserializer)?;
        s.split_whitespace()
            .map(|part| part.parse::<f64>().map_err(serde::de::Error::custom))
            .collect()
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
        // Serialization to JSON normally doesn't use the custom serializer unless we are serializing to a format that uses it,
        // but here we are using serde(with) so it should apply to the field.
        // However, serde_json might serialize the string as a JSON string.
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
