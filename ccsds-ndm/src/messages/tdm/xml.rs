// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::{TdmObservation, TdmObservationData};
use crate::error::Result;
use crate::types::CalendarEpoch;
use serde::Deserialize;

// XSD element defaults apply to present, empty elements, not omitted optional fields.
pub(super) fn default_zero<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<f64>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Value {
        Number(Option<f64>),
        Text(String),
        Element {
            #[serde(rename = "$text", default)]
            text: String,
        },
    }
    let text = match Value::deserialize(deserializer)? {
        Value::Number(value) => return Ok(value),
        Value::Text(text) | Value::Element { text } => text,
    };
    if text.is_empty() {
        Ok(Some(0.0))
    } else {
        crate::types::parse_xml_value(&text)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

pub(super) fn default_quality<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<crate::types::TdmDataQuality>, D::Error> {
    Option::<String>::deserialize(deserializer)?
        .map(|value| {
            if value.is_empty() {
                Ok(crate::types::TdmDataQuality::Raw)
            } else {
                value.parse().map_err(serde::de::Error::custom)
            }
        })
        .transpose()
}

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    let rule = |rank, repeatable| XmlSequenceRule::new(rank, repeatable);
    crate::xml::validate_element_sequences(
        xml,
        b"tdm",
        "TDM",
        |parent, child| {
            Some(match (parent, child) {
                (b"tdm", b"header") => rule(0, false),
                (b"tdm", b"body") => rule(1, false),
                (b"header", b"COMMENT") => rule(0, true),
                (b"header", b"CREATION_DATE") => rule(1, false),
                (b"header", b"ORIGINATOR") => rule(2, false),
                (b"header", b"MESSAGE_ID") => rule(3, false),
                (b"body", b"segment") => rule(0, true),
                (b"segment", b"metadata") => rule(0, false),
                (b"segment", b"data") => rule(1, false),
                (b"metadata", child) => {
                    rule(super::tdm_metadata_rank_bytes(child)?, child == b"COMMENT")
                }
                (b"data", b"COMMENT") => rule(0, true),
                (b"data", b"observation") => rule(1, true),
                (b"observation", b"EPOCH") => rule(0, false),
                (b"observation", child) if super::is_tdm_observation_key_bytes(child) => {
                    rule(1, false)
                }
                _ => return None,
            })
        },
        |element, attribute| {
            attribute == b"units" && matches!(element, b"ANGLE_1" | b"ANGLE_2" | b"RHUMIDITY")
        },
    )
}

use serde::de::{MapAccess, Visitor};
use std::fmt;

// Custom Deserialize to handle XML's flat structure correctly
impl<'de> Deserialize<'de> for TdmObservation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TdmObservationVisitor;

        impl<'de> Visitor<'de> for TdmObservationVisitor {
            type Value = TdmObservation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a TDM observation element")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut epoch: Option<CalendarEpoch> = None;
                let mut data: Option<TdmObservationData> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "EPOCH" => {
                            if epoch.is_some() {
                                return Err(serde::de::Error::duplicate_field("EPOCH"));
                            }
                            epoch = Some(map.next_value()?);
                        }
                        "ANGLE_1" | "ANGLE_2" => {
                            let value: crate::types::UnitValue<f64, crate::types::AngleUnits> =
                                map.next_value()?;
                            data = Some(
                                TdmObservationData::from_key_value(&key, value.value)
                                    .map_err(serde::de::Error::custom)?,
                            );
                        }
                        // RHUMIDITY keeps its optional units attribute; the range error names
                        // RHUMIDITY rather than the shared Percentage type.
                        "RHUMIDITY" => {
                            let value: crate::types::UnitValue<f64, crate::types::PercentageUnits> =
                                map.next_value()?;
                            crate::types::Percentage::validate_value(value.value, "RHUMIDITY")
                                .map_err(serde::de::Error::custom)?;
                            data = Some(TdmObservationData::Rhumidity(crate::types::Percentage {
                                value: value.value,
                                units: value.units,
                            }));
                        }
                        key => {
                            let mut value: String = map.next_value()?;
                            if matches!(key, "CLOCK_BIAS" | "CLOCK_DRIFT") && value.is_empty() {
                                value = "0.0".into();
                            }
                            data = Some(
                                TdmObservationData::from_key_val(key, &value)
                                    .map_err(serde::de::Error::custom)?,
                            );
                        }
                    }
                }

                let epoch = epoch.ok_or_else(|| serde::de::Error::missing_field("EPOCH"))?;
                let data = data.ok_or_else(|| {
                    serde::de::Error::custom(
                        "Missing TDM observation data (must have one of: ANGLE_1, RANGE, etc.)",
                    )
                })?;

                Ok(TdmObservation { epoch, data })
            }
        }

        deserializer.deserialize_map(TdmObservationVisitor)
    }
}
