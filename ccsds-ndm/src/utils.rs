// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

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
