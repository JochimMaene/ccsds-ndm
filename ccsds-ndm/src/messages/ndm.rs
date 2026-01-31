// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Result;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::MessageType;
use serde::{Deserialize, Serialize};

/// Combined Instantiation Navigation Data Message (NDM).
///
/// It is possible to create an XML instance that incorporates any number of NDM messages in a
/// logical suite called an ‘NDM combined instantiation’. Such combined instantiations may be
/// useful for some situations, for example: (1) a constellation of spacecraft in which
/// ephemeris data for all of the spacecraft is combined in a single XML message; (2) a
/// spacecraft attitude that depends upon a particular orbital state (an APM and its
/// associated OPM could be conveniently conveyed in a single NDM); (3) an ephemeris message
/// with the set of tracking data messages used in the orbit determination.
///
/// **CCSDS Reference**: 505.0-B-3, Section 4.11.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, bon::Builder)]
#[serde(rename = "ndm")]
pub struct CombinedNdm {
    /// Message Identifier (optional).
    #[serde(rename = "MESSAGE_ID", skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub id: Option<String>,

    /// Comments (optional).
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub comments: Vec<String>,

    /// List of contained navigation messages.
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub messages: Vec<MessageType>,
}

impl crate::traits::Validate for CombinedNdm {
    fn validate(&self) -> Result<()> {
        for msg in &self.messages {
            match msg {
                MessageType::Opm(m) => m.validate()?,
                MessageType::Omm(m) => m.validate()?,
                MessageType::Oem(m) => m.validate()?,
                MessageType::Ocm(m) => m.validate()?,
                MessageType::Acm(m) => m.validate()?,
                MessageType::Cdm(m) => m.validate()?,
                MessageType::Tdm(m) => m.validate()?,
                MessageType::Rdm(m) => m.validate()?,
                MessageType::Aem(m) => m.validate()?,
                MessageType::Apm(m) => m.validate()?,
                MessageType::Ndm(m) => m.validate()?,
            }
        }
        Ok(())
    }
}

impl Ndm for CombinedNdm {
    fn to_kvn(&self) -> Result<String> {
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        Ok(writer.finish())
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        let headers = [
            "CCSDS_OPM_VERS",
            "CCSDS_OMM_VERS",
            "CCSDS_OEM_VERS",
            "CCSDS_OCM_VERS",
            "CCSDS_ACM_VERS",
            "CCSDS_CDM_VERS",
            "CCSDS_TDM_VERS",
            "CCSDS_RDM_VERS",
            "CCSDS_AEM_VERS",
            "CCSDS_APM_VERS",
        ];

        let mut indices = Vec::new();
        for header in headers {
            for (idx, _) in kvn.match_indices(header) {
                indices.push(idx);
            }
        }
        indices.sort_unstable();

        if indices.is_empty() {
            return Err(crate::error::CcsdsNdmError::UnsupportedMessage(
                "No CCSDS KVN headers found in input".into(),
            ));
        }

        let mut messages = Vec::new();
        for i in 0..indices.len() {
            let start = indices[i];
            let end = if i + 1 < indices.len() {
                indices[i + 1]
            } else {
                kvn.len()
            };

            let chunk = &kvn[start..end];
            // We use from_str to auto-detect the type of this specific chunk.
            // Since the chunk contains exactly one header, it should return Opm/Omm/etc.
            // However, we must ensure `from_str` doesn't think it's XML (it won't, no <)
            // or empty.
            if chunk.trim().is_empty() {
                continue;
            }

            let msg = crate::from_str(chunk)?;
            messages.push(msg);
        }

        Ok(CombinedNdm {
            id: None,         // Not applicable for KVN
            comments: vec![], // Comments are likely inside the individual messages
            messages,
        })
    }

    fn to_xml(&self) -> Result<String> {
        crate::xml::to_string(self)
    }

    fn from_xml(xml: &str) -> Result<Self> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut id = None;
        let mut comments = Vec::new();
        let mut messages = Vec::new();

        // Find the root <ndm> or <message> tag first
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    let name = e.name();
                    if name.as_ref() == b"ndm" || name.as_ref() == b"message" {
                        break;
                    }
                }
                Event::Eof => {
                    return Err(crate::error::CcsdsNdmError::UnexpectedEof {
                        context: "Missing <ndm> or <message> root tag".into(),
                    })
                }
                _ => (), // Skip other things (declarations, comments, etc.)
            }
            buf.clear();
        }
        buf.clear();

        // Parse children
        loop {
            let event_start_pos = reader.buffer_position() as usize;
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    let name_bytes = e.name();
                    let name = String::from_utf8_lossy(name_bytes.as_ref()).to_lowercase();

                    let actual_start_pos = xml[event_start_pos..]
                        .find('<')
                        .map(|o| event_start_pos + o)
                        .unwrap_or(event_start_pos);

                    match name.as_str() {
                        "message_id" => {
                            let val = reader.read_text(name_bytes)?;
                            id = Some(val.to_string());
                        }
                        "comment" => {
                            let val = reader.read_text(name_bytes)?;
                            comments.push(val.to_string());
                        }
                        // Extract the outer XML of the current element.
                        "opm" | "omm" | "oem" | "ocm" | "cdm" | "tdm" | "rdm" | "acm" | "aem"
                        | "apm" => {
                            reader.read_to_end(name_bytes)?;
                            let end_pos = reader.buffer_position() as usize;
                            let full_element = &xml[actual_start_pos..end_pos];

                            // Now parse `full_element` as specific type
                            let msg = match name.as_str() {
                                "opm" => MessageType::Opm(Ndm::from_xml(full_element)?),
                                "omm" => MessageType::Omm(Ndm::from_xml(full_element)?),
                                "oem" => MessageType::Oem(Ndm::from_xml(full_element)?),
                                "ocm" => MessageType::Ocm(Ndm::from_xml(full_element)?),
                                "acm" => MessageType::Acm(Ndm::from_xml(full_element)?),
                                "cdm" => MessageType::Cdm(Ndm::from_xml(full_element)?),
                                "tdm" => MessageType::Tdm(Ndm::from_xml(full_element)?),
                                "rdm" => MessageType::Rdm(Ndm::from_xml(full_element)?),
                                "aem" => MessageType::Aem(Ndm::from_xml(full_element)?),
                                "apm" => MessageType::Apm(Ndm::from_xml(full_element)?),
                                _ => unreachable!(),
                            };
                            messages.push(msg);
                        }
                        _ => {
                            // Unknown tag, ignore
                            reader.read_to_end(name_bytes)?;
                        }
                    }
                }
                Event::End(e) if e.name().as_ref() == b"ndm" => break,
                Event::Eof => break,
                _ => (),
            }
            buf.clear();
        }

        Ok(CombinedNdm {
            id,
            comments,
            messages,
        })
    }
}

impl ToKvn for CombinedNdm {
    fn write_kvn(&self, writer: &mut KvnWriter) {
        // For KVN, there is no top-level "NDM" header or structure.
        // We just write out the messages sequentially.
        writer.write_comments(&self.comments);

        for msg in &self.messages {
            match msg {
                MessageType::Opm(m) => m.write_kvn(writer),
                MessageType::Omm(m) => m.write_kvn(writer),
                MessageType::Oem(m) => m.write_kvn(writer),
                MessageType::Ocm(m) => m.write_kvn(writer),
                MessageType::Acm(m) => m.write_kvn(writer),
                MessageType::Cdm(m) => m.write_kvn(writer),
                MessageType::Tdm(m) => m.write_kvn(writer),
                MessageType::Rdm(m) => m.write_kvn(writer),
                MessageType::Aem(m) => m.write_kvn(writer),
                MessageType::Apm(m) => m.write_kvn(writer),
                MessageType::Ndm(m) => m.write_kvn(writer), // Nested NDM? Unlikely but possible in structure.
            }
            // KVN messages are typically separated by whitespace/newlines, which the writer handles or we add explicit breaks.
            // The writer adds newlines after each pair/block.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_ndm_kvn() {
        let kvn = "CCSDS_OPM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = NASA\nOBJECT_NAME = SAT\nCENTER_NAME = EARTH\nOBJECT_ID = 1\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nEPOCH = 2023-01-01T00:00:00\nX = 1000.0\nY = 2000.0\nZ = 3000.0\nX_DOT = 1.0\nY_DOT = 2.0\nZ_DOT = 3.0\nCCSDS_OMM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = NASA\nOBJECT_NAME = SAT2\nOBJECT_ID = 2\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nMEAN_ELEMENT_THEORY = SGP4\nEPOCH = 2023-01-01T00:00:00\nMEAN_MOTION = 15.0\nECCENTRICITY = 0.001\nINCLINATION = 51.6\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nMEAN_ANOMALY = 0.0\nEPHEMERIS_TYPE = 0\nCLASSIFICATION_TYPE = U\nNORAD_CAT_ID = 12345\nELEMENT_SET_NO = 999\nREV_AT_EPOCH = 100\nBSTAR = 0.0001\nMEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0";
        let combined = CombinedNdm::from_kvn(kvn).unwrap();
        assert_eq!(combined.messages.len(), 2);
    }

    #[test]
    fn test_combined_ndm_xml() {
        let xml = r#"<ndm>
            <message_id>test-id</message_id>
            <comment>NDM Level Comment</comment>
            <opm id="CCSDS_OPM_VERS" version="3.0">
                <header>
                    <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
                    <ORIGINATOR>NASA</ORIGINATOR>
                </header>
                <body>
                    <segment>
                        <metadata>
                            <OBJECT_NAME>SAT</OBJECT_NAME>
                            <OBJECT_ID>12345</OBJECT_ID>
                            <CENTER_NAME>EARTH</CENTER_NAME>
                            <REF_FRAME>GCRF</REF_FRAME>
                            <TIME_SYSTEM>UTC</TIME_SYSTEM>
                        </metadata>
                        <data>
                            <stateVector>
                                <EPOCH>2023-01-01T00:00:00</EPOCH>
                                <X>1000</X><Y>2000</Y><Z>3000</Z>
                                <X_DOT>1</X_DOT><Y_DOT>2</Y_DOT><Z_DOT>3</Z_DOT>
                            </stateVector>
                        </data>
                    </segment>
                </body>
            </opm>
        </ndm>"#;
        let combined = CombinedNdm::from_xml(xml).unwrap();
        assert_eq!(combined.id, Some("test-id".into()));
        assert_eq!(combined.comments, vec!["NDM Level Comment".to_string()]);
        assert_eq!(combined.messages.len(), 1);
    }

    #[test]
    fn test_combined_ndm_empty_kvn() {
        assert!(CombinedNdm::from_kvn("").is_err());
    }
}
