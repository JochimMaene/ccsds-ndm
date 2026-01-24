// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

pub mod vec_f64_space_sep {
    use serde::{Deserializer, Serializer, Deserialize};

    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
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
