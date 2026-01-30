// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

/// Helper module to deserialize optional fields that may have nil="true".
///
/// In CCSDS XML, optional fields can be represented as `<FIELD nil="true"/>`
/// which has no text content. This module handles that case by checking the
/// `@nil` attribute and treating `nil="true"` as `None`.
pub mod nullable_value {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Intermediate struct that captures both the nil attribute and optionally the value.
    #[derive(Deserialize)]
    struct NullableWrapper<T> {
        #[serde(rename = "@nil", default)]
        nil: Option<String>,
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

/// Helper module to deserialize optional enum fields that may have empty text.
///
/// In CCSDS XML, optional fields can be represented as `<FIELD nil="true"/>`
/// which results in an empty text (`$text`) being deserialized. Standard serde
/// enum deserialization would fail because empty string is not a valid variant.
/// This module handles that case by treating empty strings as `None`.
pub mod nullable_enum {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Intermediate struct that checks for nil="true" first.
    #[derive(Deserialize)]
    struct NullableEnumWrapper<T> {
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
        let wrapper: Option<NullableEnumWrapper<T>> = Option::deserialize(deserializer)?;
        match wrapper {
            None => Ok(None),
            Some(w) => {
                // If nil="true", return None regardless of other content
                if let Some(ref nil) = w.nil {
                    if nil == "true" {
                        return Ok(None);
                    }
                }
                // If text is empty, return None
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
