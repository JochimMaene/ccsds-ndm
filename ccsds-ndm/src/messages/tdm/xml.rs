// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use super::{TdmObservation, TdmObservationData};
use crate::error::Result;
use crate::types::CalendarEpoch;
use serde::Deserialize;

pub(super) fn validate_xml_sequences(xml: &str) -> Result<()> {
    use crate::xml::XmlSequenceRule;

    let rule = |rank, repeatable| XmlSequenceRule::new(rank, repeatable);
    crate::xml::validate_element_sequences(
        xml,
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
                        // RHUMIDITY uses Percentage so its optional units attribute is preserved.
                        "RHUMIDITY" => {
                            data = Some(TdmObservationData::Rhumidity(map.next_value()?));
                        }
                        key => {
                            let value: f64 = map.next_value()?;
                            data = Some(
                                TdmObservationData::from_key_value(key, value)
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
