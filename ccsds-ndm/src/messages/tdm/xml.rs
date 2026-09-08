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
                        // Explicit matching of all data types
                        "ANGLE_1" => {
                            data = Some(TdmObservationData::Angle1(map.next_value()?));
                        }
                        "ANGLE_2" => {
                            data = Some(TdmObservationData::Angle2(map.next_value()?));
                        }
                        "CARRIER_POWER" => {
                            data = Some(TdmObservationData::CarrierPower(map.next_value()?));
                        }
                        "CLOCK_BIAS" => {
                            data = Some(TdmObservationData::ClockBias(map.next_value()?));
                        }
                        "CLOCK_DRIFT" => {
                            data = Some(TdmObservationData::ClockDrift(map.next_value()?));
                        }
                        "DOPPLER_COUNT" => {
                            data = Some(TdmObservationData::DopplerCount(map.next_value()?));
                        }
                        "DOPPLER_INSTANTANEOUS" => {
                            data =
                                Some(TdmObservationData::DopplerInstantaneous(map.next_value()?));
                        }
                        "DOPPLER_INTEGRATED" => {
                            data = Some(TdmObservationData::DopplerIntegrated(map.next_value()?));
                        }
                        "DOR" => {
                            data = Some(TdmObservationData::Dor(map.next_value()?));
                        }
                        "MAG" => {
                            data = Some(TdmObservationData::Mag(map.next_value()?));
                        }
                        "PC_N0" => {
                            data = Some(TdmObservationData::PcN0(map.next_value()?));
                        }
                        "PR_N0" => {
                            data = Some(TdmObservationData::PrN0(map.next_value()?));
                        }
                        "PRESSURE" => {
                            data = Some(TdmObservationData::Pressure(map.next_value()?));
                        }
                        "RANGE" => {
                            data = Some(TdmObservationData::Range(map.next_value()?));
                        }
                        "RCS" => {
                            data = Some(TdmObservationData::Rcs(map.next_value()?));
                        }
                        "RECEIVE_FREQ" => {
                            data = Some(TdmObservationData::ReceiveFreq(map.next_value()?));
                        }
                        "RECEIVE_FREQ_1" => {
                            data = Some(TdmObservationData::ReceiveFreq1(map.next_value()?));
                        }
                        "RECEIVE_FREQ_2" => {
                            data = Some(TdmObservationData::ReceiveFreq2(map.next_value()?));
                        }
                        "RECEIVE_FREQ_3" => {
                            data = Some(TdmObservationData::ReceiveFreq3(map.next_value()?));
                        }
                        "RECEIVE_FREQ_4" => {
                            data = Some(TdmObservationData::ReceiveFreq4(map.next_value()?));
                        }
                        "RECEIVE_FREQ_5" => {
                            data = Some(TdmObservationData::ReceiveFreq5(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_1" => {
                            data = Some(TdmObservationData::ReceivePhaseCt1(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_2" => {
                            data = Some(TdmObservationData::ReceivePhaseCt2(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_3" => {
                            data = Some(TdmObservationData::ReceivePhaseCt3(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_4" => {
                            data = Some(TdmObservationData::ReceivePhaseCt4(map.next_value()?));
                        }
                        "RECEIVE_PHASE_CT_5" => {
                            data = Some(TdmObservationData::ReceivePhaseCt5(map.next_value()?));
                        }
                        "RHUMIDITY" => {
                            data = Some(TdmObservationData::Rhumidity(map.next_value()?));
                        }
                        "STEC" => {
                            data = Some(TdmObservationData::Stec(map.next_value()?));
                        }
                        "TEMPERATURE" => {
                            data = Some(TdmObservationData::Temperature(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_1" => {
                            data = Some(TdmObservationData::TransmitFreq1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_2" => {
                            data = Some(TdmObservationData::TransmitFreq2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_3" => {
                            data = Some(TdmObservationData::TransmitFreq3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_4" => {
                            data = Some(TdmObservationData::TransmitFreq4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_5" => {
                            data = Some(TdmObservationData::TransmitFreq5(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_1" => {
                            data = Some(TdmObservationData::TransmitFreqRate1(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_2" => {
                            data = Some(TdmObservationData::TransmitFreqRate2(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_3" => {
                            data = Some(TdmObservationData::TransmitFreqRate3(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_4" => {
                            data = Some(TdmObservationData::TransmitFreqRate4(map.next_value()?));
                        }
                        "TRANSMIT_FREQ_RATE_5" => {
                            data = Some(TdmObservationData::TransmitFreqRate5(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_1" => {
                            data = Some(TdmObservationData::TransmitPhaseCt1(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_2" => {
                            data = Some(TdmObservationData::TransmitPhaseCt2(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_3" => {
                            data = Some(TdmObservationData::TransmitPhaseCt3(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_4" => {
                            data = Some(TdmObservationData::TransmitPhaseCt4(map.next_value()?));
                        }
                        "TRANSMIT_PHASE_CT_5" => {
                            data = Some(TdmObservationData::TransmitPhaseCt5(map.next_value()?));
                        }
                        "TROPO_DRY" => {
                            data = Some(TdmObservationData::TropoDry(map.next_value()?));
                        }
                        "TROPO_WET" => {
                            data = Some(TdmObservationData::TropoWet(map.next_value()?));
                        }
                        "VLBI_DELAY" => {
                            data = Some(TdmObservationData::VlbiDelay(map.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::custom(format!(
                                "unknown TDM observation field '{key}'"
                            )));
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
