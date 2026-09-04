// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, FormatError, Result};
use crate::kvn::ser::KvnWriter;
use crate::traits::{Ndm, ToKvn};
use crate::MessageType;
use serde::{Deserialize, Serialize};
use std::io::Write;

fn is_ascii_whitespace(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_whitespace)
}

fn history_record_count(message: &MessageType) -> usize {
    match message {
        MessageType::Oem(message) => message
            .body
            .segment
            .iter()
            .map(|segment| segment.data.state_vector.len() + segment.data.covariance_matrix.len())
            .sum(),
        MessageType::Ocm(message) => {
            let data = &message.body.segment.data;
            data.traj
                .iter()
                .map(|block| block.traj_lines.len())
                .sum::<usize>()
                + data
                    .cov
                    .iter()
                    .map(|block| block.cov_lines.len())
                    .sum::<usize>()
                + data
                    .man
                    .iter()
                    .map(|block| block.man_lines.len())
                    .sum::<usize>()
        }
        MessageType::Tdm(message) => message
            .body
            .segments
            .iter()
            .map(|segment| segment.data.observations.len())
            .sum(),
        MessageType::Aem(message) => message
            .body
            .segment
            .iter()
            .map(|segment| segment.data.attitude_states.len())
            .sum(),
        MessageType::Acm(message) => {
            let data = &message.body.segment.data;
            data.att
                .iter()
                .map(|block| block.att_lines.len())
                .sum::<usize>()
                + data
                    .cov
                    .iter()
                    .map(|block| block.cov_lines.len())
                    .sum::<usize>()
                + data.man.len()
        }
        MessageType::Ndm(message) => message.messages.iter().map(history_record_count).sum(),
        MessageType::Opm(_)
        | MessageType::Omm(_)
        | MessageType::Cdm(_)
        | MessageType::Rdm(_)
        | MessageType::Apm(_) => 0,
    }
}

fn invalid_envelope(message: impl Into<String>) -> CcsdsNdmError {
    CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message.into())))
}

fn validate_combined_root_attributes(start: &quick_xml::events::BytesStart<'_>) -> Result<()> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| invalid_envelope(error.to_string()))?;
        if !matches!(
            attribute.key.as_ref(),
            b"xmlns"
                | b"xmlns:xsi"
                | b"xmlns:ndm"
                | b"xsi:noNamespaceSchemaLocation"
                | b"xsi:schemaLocation"
        ) {
            return Err(invalid_envelope(format!(
                "attribute '{}' is not allowed on the combined NDM root",
                String::from_utf8_lossy(attribute.key.as_ref())
            )));
        }
    }
    Ok(())
}

fn validate_combined_child_attributes(start: &quick_xml::events::BytesStart<'_>) -> Result<()> {
    let mut id = false;
    let mut version = false;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| invalid_envelope(error.to_string()))?;
        match attribute.key.as_ref() {
            b"id" if !id => id = true,
            b"version" if !version => version = true,
            name if name == b"xmlns" || name.starts_with(b"xmlns:") => {}
            name => {
                return Err(invalid_envelope(format!(
                    "attribute '{}' is not allowed on a combined NDM constituent",
                    String::from_utf8_lossy(name)
                )))
            }
        }
    }
    if !id || !version {
        return Err(invalid_envelope(
            "combined NDM constituents require exactly one id and version attribute",
        ));
    }
    Ok(())
}

fn validate_combined_xml_depth(xml: &str, limit: usize) -> Result<()> {
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut depth = 0usize;
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(_)) => {
                depth = depth.saturating_add(1);
                if depth > limit {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit,
                        actual: depth,
                    });
                }
            }
            Ok(quick_xml::events::Event::Empty(_)) => {
                let actual = depth.saturating_add(1);
                if actual > limit {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit,
                        actual,
                    });
                }
            }
            Ok(quick_xml::events::Event::End(_)) => {
                depth = depth.saturating_sub(1);
            }
            Ok(quick_xml::events::Event::Eof) => return Ok(()),
            Ok(_) => {}
            Err(error) => return Err(error.into()),
        }
    }
}

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
    #[serde(
        rename = "MESSAGE_ID",
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::nullable"
    )]
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
                MessageType::Ndm(_) => {
                    return Err(crate::error::ValidationError::InvalidValue {
                        field: "ndm".into(),
                        value: "nested combined NDM".into(),
                        expected: "a constituent standalone message".into(),
                        line: None,
                    }
                    .into())
                }
            }
        }
        Ok(())
    }
}

impl Ndm for CombinedNdm {
    fn to_kvn(&self) -> Result<String> {
        self.validate_kvn_envelope()?;
        for message in &self.messages {
            message.validate_for_generation(crate::generation::OutputFormat::Kvn)?;
        }
        let mut writer = KvnWriter::new();
        self.write_kvn(&mut writer);
        writer.finish_checked()
    }

    fn from_kvn(kvn: &str) -> Result<Self> {
        Self::from_kvn_with_options(kvn, &crate::options::ParseOptions::default())
    }

    fn to_xml(&self) -> Result<String> {
        self.validate_xml_envelope()?;
        crate::traits::Validate::validate(self)?;
        for message in &self.messages {
            message.validate_for_generation(crate::generation::OutputFormat::Xml)?;
        }
        crate::xml::to_string(self).map_err(|error| {
            error.with_generation_context(
                crate::validation::MessageKind::Ndm,
                crate::error::DiagnosticNotation::Xml,
                "combined",
            )
        })
    }

    fn from_xml(xml: &str) -> Result<Self> {
        Self::from_xml_with_options(xml, &crate::options::ParseOptions::default())
    }
}

impl CombinedNdm {
    fn validate_kvn_envelope(&self) -> Result<()> {
        crate::traits::Validate::validate(self)?;
        if self.id.is_some() {
            return Err(crate::error::ValidationError::InvalidValue {
                field: "MESSAGE_ID".into(),
                value: "present".into(),
                expected:
                    "omitted when generating the non-standard sequential KVN convenience form"
                        .into(),
                line: None,
            }
            .into());
        }
        for comment in &self.comments {
            if let Some(error) = crate::validation::kvn_comment_error(comment) {
                return Err(error.into());
            }
        }
        Ok(())
    }

    fn validate_xml_envelope(&self) -> Result<()> {
        if let Some(value) = &self.id {
            if let Some(error) = crate::validation::xml_text_error("MESSAGE_ID", value) {
                return Err(error.into());
            }
        }
        for value in &self.comments {
            if let Some(error) = crate::validation::xml_text_error("COMMENT", value) {
                return Err(error.into());
            }
        }
        Ok(())
    }

    fn validate_children_for_generation(
        &self,
        format: crate::generation::OutputFormat,
    ) -> Result<()> {
        crate::traits::Validate::validate(self)?;
        for message in &self.messages {
            message.validate_for_generation(format)?;
        }
        Ok(())
    }

    /// Stream the sequential KVN convenience representation.
    pub fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        self.validate_kvn_envelope()?;
        self.validate_children_for_generation(crate::generation::OutputFormat::Kvn)?;

        let mut writer = KvnWriter::from_io(output);
        self.write_kvn(&mut writer);
        writer.finish_io()
    }

    /// Stream the normative XML combined instantiation.
    pub fn write_xml_to<W: Write>(&self, output: &mut W) -> Result<()> {
        self.validate_xml_envelope()?;
        self.validate_children_for_generation(crate::generation::OutputFormat::Xml)?;
        crate::xml::to_writer(output, self).map_err(|error| {
            error.with_generation_context(
                crate::validation::MessageKind::Ndm,
                crate::error::DiagnosticNotation::Xml,
                "combined",
            )
        })
    }

    /// Parse the sequential KVN convenience representation with bounded child parsing.
    pub fn from_kvn_with_options(
        kvn: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        if options
            .max_input_bytes
            .is_some_and(|limit| kvn.len() > limit)
        {
            return Err(CcsdsNdmError::ResourceLimitExceeded {
                resource: "input_document",
                limit: options.max_input_bytes.unwrap(),
                actual: kvn.len(),
            });
        }
        let indices = crate::detect::kvn_message_offsets(kvn);

        if indices.is_empty() {
            return Err(crate::error::CcsdsNdmError::UnsupportedMessage(
                "No CCSDS KVN headers found in input".into(),
            ));
        }

        let mut comments = Vec::new();
        for line in kvn[..indices[0]].lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let Some(comment) = line.strip_prefix("COMMENT") else {
                return Err(CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(
                    "unexpected content before the first combined KVN message".into(),
                ))));
            };
            comments.push(comment.trim_start().to_owned());
        }

        let mut messages = Vec::new();
        let mut records = 0usize;
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

            let mut child_options = options.clone();
            if let Some(limit) = options.max_records {
                child_options.max_records = Some(limit.saturating_sub(records));
            }
            let msg = crate::from_str_with_options(
                chunk,
                Some(crate::detect::Notation::Kvn),
                &child_options,
            )?;
            records = records.saturating_add(history_record_count(&msg));
            if options.max_records.is_some_and(|limit| records > limit) {
                return Err(CcsdsNdmError::ResourceLimitExceeded {
                    resource: "history_records",
                    limit: options.max_records.unwrap(),
                    actual: records,
                });
            }
            messages.push(msg);
        }

        Ok(CombinedNdm {
            id: None, // Not applicable for KVN
            comments,
            messages,
        })
    }

    /// Strictly parse a combined XML instantiation with bounded child parsing.
    pub fn from_xml_with_options(
        xml: &str,
        options: &crate::options::ParseOptions,
    ) -> Result<Self> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        if options
            .max_input_bytes
            .is_some_and(|limit| xml.len() > limit)
        {
            return Err(CcsdsNdmError::ResourceLimitExceeded {
                resource: "input_document",
                limit: options.max_input_bytes.unwrap(),
                actual: xml.len(),
            });
        }
        validate_combined_xml_depth(xml, options.max_xml_depth)?;
        crate::xml::validate_document_root(xml, b"ndm", "combined NDM")?;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut id = None;
        let mut comments = Vec::new();
        let mut messages = Vec::new();
        let mut records = 0usize;

        let invalid = |message: &str| {
            CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message.into())))
        };

        // The first element must be the normative combined-instantiation root.
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.name().as_ref() == b"ndm" => {
                    validate_combined_root_attributes(&e)?;
                    break;
                }
                Event::Empty(e) if e.name().as_ref() == b"ndm" => {
                    validate_combined_root_attributes(&e)?;
                    loop {
                        buf.clear();
                        match reader.read_event_into(&mut buf)? {
                            Event::Eof => {
                                return Ok(CombinedNdm {
                                    id: None,
                                    comments: Vec::new(),
                                    messages: Vec::new(),
                                });
                            }
                            Event::Text(text) if is_ascii_whitespace(text.as_ref()) => {}
                            Event::Comment(_) => {}
                            _ => return Err(invalid("trailing content after combined NDM root")),
                        }
                    }
                }
                Event::Start(e) => {
                    return Err(invalid(&format!(
                        "expected <ndm> root, found <{}>",
                        String::from_utf8_lossy(e.name().as_ref())
                    )))
                }
                Event::Eof => {
                    return Err(crate::error::CcsdsNdmError::UnexpectedEof {
                        context: "Missing <ndm> root tag".into(),
                    })
                }
                Event::Text(text) if !is_ascii_whitespace(text.as_ref()) => {
                    return Err(invalid("non-whitespace content before <ndm>"));
                }
                Event::Decl(_) | Event::Comment(_) | Event::PI(_) | Event::Text(_) => {}
                _ => return Err(invalid("unexpected content before <ndm>")),
            }
            buf.clear();
        }
        buf.clear();

        // Schema sequence: optional MESSAGE_ID, comments, then any number of messages.
        let mut phase = 0u8;
        loop {
            let event_start_pos = reader.buffer_position() as usize;
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    let name_bytes = e.name();
                    let name = name_bytes.as_ref();

                    let actual_start_pos = xml[event_start_pos..]
                        .find('<')
                        .map(|o| event_start_pos + o)
                        .unwrap_or(event_start_pos);

                    match name {
                        b"MESSAGE_ID" => {
                            if phase != 0 || id.is_some() {
                                return Err(invalid(
                                    "MESSAGE_ID must occur at most once before comments and messages",
                                ));
                            }
                            let val = reader.read_text(name_bytes)?;
                            id = Some(val.to_string());
                        }
                        b"COMMENT" => {
                            if phase > 1 {
                                return Err(invalid("COMMENT must precede contained messages"));
                            }
                            phase = 1;
                            let val = reader.read_text(name_bytes)?;
                            comments.push(val.to_string());
                        }
                        // Extract the outer XML of the current element.
                        b"opm" | b"omm" | b"oem" | b"ocm" | b"cdm" | b"tdm" | b"rdm" | b"acm"
                        | b"aem" | b"apm" => {
                            validate_combined_child_attributes(&e)?;
                            phase = 2;
                            reader.read_to_end(name_bytes)?;
                            let end_pos = reader.buffer_position() as usize;
                            let full_element = &xml[actual_start_pos..end_pos];

                            let mut child_options = options.clone();
                            if let Some(limit) = options.max_records {
                                child_options.max_records = Some(limit.saturating_sub(records));
                            }
                            let msg = crate::from_str_with_options(
                                full_element,
                                Some(crate::detect::Notation::Xml),
                                &child_options,
                            )?;
                            records = records.saturating_add(history_record_count(&msg));
                            if options.max_records.is_some_and(|limit| records > limit) {
                                return Err(CcsdsNdmError::ResourceLimitExceeded {
                                    resource: "history_records",
                                    limit: options.max_records.unwrap(),
                                    actual: records,
                                });
                            }
                            messages.push(msg);
                        }
                        _ => {
                            return Err(invalid(&format!(
                                "unknown combined NDM child <{}>",
                                String::from_utf8_lossy(name)
                            )));
                        }
                    }
                }
                Event::End(e) if e.name().as_ref() == b"ndm" => {
                    break;
                }
                Event::End(e) => {
                    return Err(invalid(&format!(
                        "unexpected closing element </{}>",
                        String::from_utf8_lossy(e.name().as_ref())
                    )))
                }
                Event::Eof => return Err(invalid("combined NDM root is not closed")),
                Event::Text(text) if !is_ascii_whitespace(text.as_ref()) => {
                    return Err(invalid("unexpected text in combined NDM envelope"));
                }
                Event::Comment(_) | Event::Text(_) => {}
                _ => return Err(invalid("unexpected content in combined NDM envelope")),
            }
            buf.clear();
        }

        loop {
            buf.clear();
            match reader.read_event_into(&mut buf)? {
                Event::Eof => break,
                Event::Text(text) if is_ascii_whitespace(text.as_ref()) => {}
                Event::Comment(_) => {}
                _ => return Err(invalid("trailing content after combined NDM root")),
            }
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
        let kvn = "CCSDS_OPM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = NASA\nOBJECT_NAME = SAT\nOBJECT_ID = 1\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nEPOCH = 2023-01-01T00:00:00\nX = 1000.0\nY = 2000.0\nZ = 3000.0\nX_DOT = 1.0\nY_DOT = 2.0\nZ_DOT = 3.0\nCCSDS_OMM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = NASA\nOBJECT_NAME = SAT2\nOBJECT_ID = 2\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nMEAN_ELEMENT_THEORY = SGP4\nEPOCH = 2023-01-01T00:00:00\nMEAN_MOTION = 15.0\nECCENTRICITY = 0.001\nINCLINATION = 51.6\nRA_OF_ASC_NODE = 0.0\nARG_OF_PERICENTER = 0.0\nMEAN_ANOMALY = 0.0\nEPHEMERIS_TYPE = 0\nCLASSIFICATION_TYPE = U\nNORAD_CAT_ID = 12345\nELEMENT_SET_NO = 999\nREV_AT_EPOCH = 100\nBSTAR = 0.0001\nMEAN_MOTION_DOT = 0.000001\nMEAN_MOTION_DDOT = 0.0";
        let combined = CombinedNdm::from_kvn(kvn).unwrap();
        assert_eq!(combined.messages.len(), 2);
    }

    #[test]
    fn test_combined_ndm_xml() {
        let xml = r#"<ndm>
            <MESSAGE_ID>test-id</MESSAGE_ID>
            <COMMENT>NDM Level Comment</COMMENT>
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
