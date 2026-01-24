// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Result;
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::MessageType;
use serde::Serialize;

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
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename = "ndm")]
pub struct CombinedNdm {
    /// Message Identifier (optional).
    #[serde(rename = "MESSAGE_ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Comments (optional).
    #[serde(rename = "COMMENT", default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<String>,

    /// List of contained navigation messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<MessageType>,
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
            "CCSDS_CDM_VERS",
            "CCSDS_TDM_VERS",
            "CCSDS_RDM_VERS",
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

        // Find the root <ndm> tag first
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"ndm" => break,
                Event::Eof => {
                    return Err(crate::error::CcsdsNdmError::UnexpectedEof {
                        context: "Missing <ndm> root tag".into(),
                    })
                }
                _ => (), // Skip other things (declarations, comments, etc.)
            }
            buf.clear();
        }
        buf.clear();

        // Parse children
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    let name_bytes = e.name();
                    let name = String::from_utf8_lossy(name_bytes.as_ref()).to_lowercase();

                    match name.as_str() {
                        "message_id" => {
                            let val = reader.read_text(name_bytes)?;
                            id = Some(val.to_string());
                        }
                        "comment" => {
                            let val = reader.read_text(name_bytes)?;
                            comments.push(val.to_string());
                        }
                        // For messages, we need to extract the sub-XML.
                        // Ideally we'd use `read_to_end` relative to the current depth, but `quick-xml` 0.31+
                        // makes getting the raw span trickier without `GenericReader::read_text` for elements.
                        //
                        // Alternative: Delegate to standard deserializers by reconstructing a mini-document
                        // or using `from_reader` if we can align the cursor.
                        //
                        // Better approach for robust nesting:
                        // Extract the outer XML of the current element.
                        "opm" | "omm" | "oem" | "ocm" | "cdm" | "tdm" | "rdm" => {
                            // Capture the start position of this element in the original string.
                            // `reader.buffer_position()` is the byte offset after the last read event.
                            // The start tag `e` is what we just read.
                            // However, `from_str` reader doesn't easily expose global offset if we didn't track it.
                            // BUT, `quick-xml`'s `from_str` reader operates on a byte slice.
                            // We can use the Span returned by `read_to_end` combined with the *current* event's start.

                            // Actually, simplest way for mixed content without re-parsing everything:
                            // Function `extract_xml_element`?
                            // No, let's simply assume standard CCSDS structure where sub-messages are independent.

                            // We need to re-serialize the current event `e` (the start tag) + the content + end tag.
                            let start_open =
                                format!("<{}", String::from_utf8_lossy(e.name().as_ref()));
                            // Attributes? `e.attributes()`
                            let mut attrs_str = String::new();
                            for attr in e.attributes() {
                                let attr = attr.map_err(|e| {
                                    crate::error::CcsdsNdmError::Format(Box::new(
                                        crate::error::FormatError::Xml(
                                            quick_xml::Error::InvalidAttr(e),
                                        ),
                                    ))
                                })?;
                                attrs_str.push_str(&format!(
                                    " {}=\"{}\"",
                                    String::from_utf8_lossy(attr.key.as_ref()),
                                    String::from_utf8_lossy(&attr.value)
                                ));
                            }
                            let start_tag = format!("{}{}>", start_open, attrs_str);

                            // Read content events until end tag
                            // `read_to_end` returns the *span* of the content (excluding start/end tags usually? No, check docs).
                            // `read_to_end` consumes until `</name>`. It returns the span of the *inner content*.
                            let span = reader.read_to_end(name_bytes)?;
                            let content = &xml[span.start as usize..span.end as usize];

                            let full_element = format!("{}{}</{}>", start_tag, content, name);

                            // Now parse `full_element` as specific type
                            let msg = match name.as_str() {
                                "opm" => MessageType::Opm(Ndm::from_xml(&full_element)?),
                                "omm" => MessageType::Omm(Ndm::from_xml(&full_element)?),
                                "oem" => MessageType::Oem(Ndm::from_xml(&full_element)?),
                                "ocm" => MessageType::Ocm(Ndm::from_xml(&full_element)?),
                                "cdm" => MessageType::Cdm(Ndm::from_xml(&full_element)?),
                                "tdm" => MessageType::Tdm(Ndm::from_xml(&full_element)?),
                                "rdm" => MessageType::Rdm(Ndm::from_xml(&full_element)?),
                                _ => unreachable!(),
                            };
                            messages.push(msg);
                        }
                        _ => {
                            // Unknown tag, ignore
                            reader.read_to_end(e.name())?;
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
        // Comments and ID at the NDM level are not standard in KVN (based on current understanding),
        // but if they were to exist, they would likely be comments at the top.
        // For now, we'll write comments if present, but ignore ID as it has no standard KVN key here.
        writer.write_comments(&self.comments);

        for msg in &self.messages {
            match msg {
                MessageType::Opm(m) => m.write_kvn(writer),
                MessageType::Omm(m) => m.write_kvn(writer),
                MessageType::Oem(m) => m.write_kvn(writer),
                MessageType::Ocm(m) => m.write_kvn(writer),
                MessageType::Cdm(m) => m.write_kvn(writer),
                MessageType::Tdm(m) => m.write_kvn(writer),
                MessageType::Rdm(m) => m.write_kvn(writer),
                MessageType::Ndm(m) => m.write_kvn(writer), // Nested NDM? Unlikely but possible in structure.
            }
            // KVN messages are typically separated by whitespace/newlines, which the writer handles or we add explicit breaks.
            // The writer adds newlines after each pair/block.
        }
    }
}
