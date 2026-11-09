// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, FormatError, Result};
use crate::traits::Ndm;
use crate::Message;
use serde::{Deserialize, Serialize};

fn is_ascii_whitespace(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_whitespace)
}

fn invalid_envelope(message: impl Into<String>) -> CcsdsNdmError {
    CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message.into())))
}

/// ODM 8.12.6: the combined root carries the standard namespace attributes, but no `id` or
/// `version`. The shared root check has already admitted only those attribute kinds. Returns the
/// root namespace declarations, which constituents inherit.
fn validate_combined_root_attributes(
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut declarations = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| invalid_envelope(error.to_string()))?;
        match attribute.key.as_ref() {
            b"id" | b"version" => {
                return Err(invalid_envelope(format!(
                    "attribute '{}' is not allowed on the combined NDM root",
                    String::from_utf8_lossy(attribute.key.as_ref())
                )));
            }
            _ if attribute.key.as_namespace_binding().is_some() => {
                declarations.push((attribute.key.as_ref().to_vec(), attribute.value.to_vec()));
            }
            _ => {}
        }
    }
    Ok(declarations)
}

/// Parse a constituent as a standalone message within the namespace scope of the combined root.
fn parse_constituent(
    document: &str,
    source_offset: usize,
    constituent: usize,
    element: &str,
    start: &quick_xml::events::BytesStart<'_>,
    root_declarations: &[(Vec<u8>, Vec<u8>)],
) -> Result<Message> {
    let inherited = root_declarations
        .iter()
        .filter(|(key, _)| {
            !start
                .attributes()
                .flatten()
                .any(|attribute| attribute.key.as_ref() == key.as_slice())
        })
        .fold(String::new(), |mut output, (key, value)| {
            output.push(' ');
            output.push_str(&String::from_utf8_lossy(key));
            // The raw value is valid attribute content except for its original delimiter,
            // which may have been `'`; escape `"` so it can be rewritten inside `"`.
            output.push_str("=\"");
            output.push_str(&String::from_utf8_lossy(value).replace('"', "&quot;"));
            output.push('"');
            output
        });
    let name_end = 1 + start.name().as_ref().len();
    let standalone = format!(
        "{}\n{}{inherited}{}",
        crate::xml::XML_HEADER,
        &element[..name_end],
        &element[name_end..]
    );
    crate::from_str_with_notation(&standalone, Some(crate::detect::Notation::Xml)).map_err(
        |error| {
            error.rebase_xml_location(
                document,
                |offset| {
                    let offset = offset.saturating_sub(crate::xml::XML_HEADER.len() + 1);
                    let offset = if offset <= name_end {
                        offset
                    } else {
                        offset.saturating_sub(inherited.len()).max(name_end)
                    };
                    source_offset + offset
                },
                constituent,
            )
        },
    )
}

/// ODM 8.12.7: "the only attributes that shall appear on the constituent message tags ... are
/// the 'id' and 'version' attributes", so constituents reject the schema-location hints a
/// standalone root may carry. Namespace declarations are still admitted: XML Namespaces does
/// not treat them as attributes, which is this library's reading of the rule.
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

/// The bundled combined schema fixes ODM to edition 3.0 and TDM to edition 2.0.
fn validate_combined_constituent_edition(message: &Message) -> Result<()> {
    let (version, required) = match message {
        Message::Opm(value) => (&value.version, "3.0"),
        Message::Omm(value) => (&value.version, "3.0"),
        Message::Oem(value) => (&value.version, "3.0"),
        Message::Tdm(value) => (&value.version, "2.0"),
        _ => return Ok(()),
    };
    if version != required {
        return Err(crate::error::ValidationError::InvalidValue {
            field: "constituent version".into(),
            value: version.clone(),
            expected: if required == "2.0" {
                "TDM 2.0 in the NDM/XML 4.0.0 combined envelope; use standalone XML for TDM 1.0"
            } else {
                "ODM 3.0 in the NDM/XML 4.0.0 combined envelope; use standalone XML for ODM 2.0"
            }
            .into(),
            line: None,
        }
        .into());
    }
    Ok(())
}

fn read_envelope_text(
    reader: &mut quick_xml::Reader<&[u8]>,
    start: &quick_xml::events::BytesStart<'_>,
    empty: bool,
    namespaces: &mut crate::xml::Namespaces,
) -> Result<String> {
    use quick_xml::events::Event;

    namespaces.enter(start, 2, &invalid_envelope)?;
    crate::xml::validate_attributes(
        namespaces,
        start,
        start.local_name().as_ref(),
        &|_, _| false,
        &invalid_envelope,
    )?;
    namespaces.leave(1);
    let mut value = String::new();
    if empty {
        return Ok(value);
    }
    loop {
        match reader.read_event()? {
            Event::Text(text) => {
                value.push_str(&text.xml_content().map_err(quick_xml::Error::from)?)
            }
            Event::CData(text) => {
                value.push_str(&text.xml_content().map_err(quick_xml::Error::from)?)
            }
            Event::GeneralRef(reference) => {
                if let Some(character) = reference.resolve_char_ref()? {
                    value.push(character);
                } else {
                    let name = reference.decode().map_err(quick_xml::Error::from)?;
                    value.push_str(
                        quick_xml::escape::resolve_predefined_entity(&name)
                            .ok_or_else(|| invalid_envelope("unknown XML entity reference"))?,
                    );
                }
            }
            Event::End(end) if end.name() == start.name() => return Ok(value),
            Event::Comment(_) | Event::PI(_) => {}
            _ => {
                return Err(invalid_envelope(
                    "combined NDM text fields require text-only content",
                ))
            }
        }
    }
}

fn validate_combined_xml_depth(xml: &str) -> Result<()> {
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut depth = 0usize;
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(_)) => {
                depth = depth.saturating_add(1);
                if depth > crate::xml::XML_DEPTH_LIMIT {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit: crate::xml::XML_DEPTH_LIMIT,
                        actual: depth,
                    });
                }
            }
            Ok(quick_xml::events::Event::Empty(_)) => {
                let actual = depth.saturating_add(1);
                if actual > crate::xml::XML_DEPTH_LIMIT {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit: crate::xml::XML_DEPTH_LIMIT,
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
    pub messages: Vec<Message>,
}

impl crate::traits::Validate for CombinedNdm {
    fn validate(&self) -> Result<()> {
        for msg in &self.messages {
            if matches!(msg, Message::Ndm(_)) {
                return Err(crate::error::ValidationError::invalid_value(
                    "ndm",
                    "nested combined NDM",
                    "a constituent standalone message",
                )
                .into());
            }
            msg.validate()?;
        }
        Ok(())
    }
}

pub(crate) fn combined_kvn_unsupported() -> CcsdsNdmError {
    CcsdsNdmError::UnsupportedNotation {
        message_type: "Combined NDM",
        requested: crate::error::DiagnosticNotation::Kvn,
    }
}

impl Ndm for CombinedNdm {
    /// Always returns `UnsupportedNotation`; combined NDM has no KVN representation.
    fn to_kvn(&self) -> Result<String> {
        Err(combined_kvn_unsupported().with_generation_context(
            crate::validation::MessageKind::Ndm,
            crate::error::DiagnosticNotation::Kvn,
            "combined",
        ))
    }

    /// Always returns `UnsupportedNotation`; combined NDM has no KVN representation.
    fn from_kvn(kvn: &str) -> Result<Self> {
        Err(combined_kvn_unsupported().with_parse_context(
            crate::validation::MessageKind::Ndm,
            crate::error::DiagnosticNotation::Kvn,
            kvn,
            None,
        ))
    }

    fn to_xml(&self) -> Result<String> {
        self.validate_xml_envelope()?;
        self.validate_children_for_generation(crate::generation::OutputFormat::Xml)?;
        crate::xml::to_string(self)
            .and_then(|xml| {
                if self
                    .messages
                    .iter()
                    .any(|message| matches!(message, Message::Cdm(_)))
                {
                    crate::messages::cdm::xml::format_output(&xml)
                } else {
                    Ok(xml)
                }
            })
            .map_err(|error| {
                error.with_generation_context(
                    crate::validation::MessageKind::Ndm,
                    crate::error::DiagnosticNotation::Xml,
                    "combined",
                )
            })
    }

    fn from_xml(xml: &str) -> Result<Self> {
        let xml = crate::detect::without_utf8_bom(xml);
        Self::from_xml_strict(xml).map_err(|error| {
            error.with_parse_context(
                crate::validation::MessageKind::Ndm,
                crate::error::DiagnosticNotation::Xml,
                xml,
                None,
            )
        })
    }

    /// Always returns `UnsupportedNotation`; combined NDM has no KVN representation.
    fn write_kvn_to<W: std::io::Write>(&self, _output: &mut W) -> Result<()> {
        Err(combined_kvn_unsupported().with_generation_context(
            crate::validation::MessageKind::Ndm,
            crate::error::DiagnosticNotation::Kvn,
            "combined",
        ))
    }

    fn write_xml_to<W: std::io::Write>(&self, output: &mut W) -> Result<()> {
        if self
            .messages
            .iter()
            .any(|message| matches!(message, Message::Cdm(_)))
        {
            output
                .write_all(self.to_xml()?.as_bytes())
                .map_err(|error| {
                    CcsdsNdmError::from(error).with_generation_context(
                        crate::validation::MessageKind::Ndm,
                        crate::error::DiagnosticNotation::Xml,
                        "combined",
                    )
                })?;
            return Ok(());
        }
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
}

impl CombinedNdm {
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
        for message in &self.messages {
            if matches!(message, Message::Ndm(_)) {
                return Err(crate::error::ValidationError::invalid_value(
                    "ndm",
                    "nested combined NDM",
                    "a constituent standalone message",
                )
                .into());
            }
            message.validate_for_generation(format)?;
            validate_combined_constituent_edition(message)?;
        }
        Ok(())
    }

    /// Strictly parse a combined XML instantiation.
    pub(crate) fn from_xml_strict(xml: &str) -> Result<Self> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        if let Some(error) = crate::validation::xml_text_error("XML document", xml) {
            return Err(invalid_envelope(error.to_string()));
        }
        validate_combined_xml_depth(xml)?;
        crate::xml::validate_document_root(xml, b"ndm", "combined NDM")?;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        let mut id = None;
        let mut comments = Vec::new();
        let mut messages = Vec::new();
        let mut namespaces = crate::xml::Namespaces::default();
        let root_declarations;
        let root_form;
        // NDM/XML 1.0 declares its elements in no namespace; record each envelope element's form.
        let mut qualified_children = false;

        let invalid = |message: &str| {
            CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message.into())))
        };

        // The first element must be the normative combined-instantiation root.
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"ndm" => {
                    namespaces.enter(&e, 1, &invalid_envelope)?;
                    root_declarations = validate_combined_root_attributes(&e)?;
                    root_form = crate::xml::element_form(&namespaces, e.name(), &invalid_envelope)?;
                    break;
                }
                Event::Empty(e) if e.local_name().as_ref() == b"ndm" => {
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
                            Event::Text(text)
                                if is_ascii_whitespace(
                                    text.xml_content()
                                        .map_err(quick_xml::Error::from)?
                                        .as_bytes(),
                                ) => {}
                            Event::Comment(_) | Event::PI(_) => {}
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
                Event::Text(text)
                    if !is_ascii_whitespace(
                        text.xml_content()
                            .map_err(quick_xml::Error::from)?
                            .as_bytes(),
                    ) =>
                {
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
            let event = reader.read_event_into(&mut buf)?;
            let empty = matches!(event, Event::Empty(_));
            match event {
                Event::Start(e) | Event::Empty(e) => {
                    let name_bytes = e.name();
                    let local_name = e.local_name();
                    let name = local_name.as_ref();

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
                            id = Some(read_envelope_text(&mut reader, &e, empty, &mut namespaces)?);
                        }
                        b"COMMENT" => {
                            if phase > 1 {
                                return Err(invalid("COMMENT must precede contained messages"));
                            }
                            phase = 1;
                            comments.push(read_envelope_text(
                                &mut reader,
                                &e,
                                empty,
                                &mut namespaces,
                            )?);
                        }
                        // Extract the outer XML of the current element.
                        b"opm" | b"omm" | b"oem" | b"ocm" | b"cdm" | b"tdm" | b"rdm" | b"acm"
                        | b"aem" | b"apm" => {
                            if empty {
                                return Err(invalid("combined NDM constituents cannot be empty"));
                            }
                            validate_combined_child_attributes(&e)?;
                            namespaces.enter(&e, 2, &invalid_envelope)?;
                            qualified_children |=
                                crate::xml::element_form(&namespaces, e.name(), &invalid_envelope)?
                                    == crate::xml::ElementForm::Qualified;
                            namespaces.leave(1);
                            phase = 2;
                            reader.read_to_end(name_bytes)?;
                            let end_pos = reader.buffer_position() as usize;
                            let full_element = &xml[actual_start_pos..end_pos];

                            let msg = parse_constituent(
                                xml,
                                actual_start_pos,
                                messages.len(),
                                full_element,
                                &e,
                                &root_declarations,
                            )?;
                            messages.push(msg);
                        }
                        _ => {
                            if empty {
                                return Err(invalid("unexpected content in combined NDM envelope"));
                            }
                            return Err(invalid(&format!(
                                "unknown combined NDM child <{}>",
                                String::from_utf8_lossy(name)
                            )));
                        }
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"ndm" => {
                    break;
                }
                Event::End(e) => {
                    return Err(invalid(&format!(
                        "unexpected closing element </{}>",
                        String::from_utf8_lossy(e.name().as_ref())
                    )))
                }
                Event::Eof => return Err(invalid("combined NDM root is not closed")),
                Event::Text(text) => {
                    if !is_ascii_whitespace(
                        text.xml_content()
                            .map_err(quick_xml::Error::from)?
                            .as_bytes(),
                    ) {
                        return Err(invalid("unexpected text in combined NDM envelope"));
                    }
                }
                Event::CData(text) => {
                    if !is_ascii_whitespace(
                        text.xml_content()
                            .map_err(quick_xml::Error::from)?
                            .as_bytes(),
                    ) {
                        return Err(invalid("unexpected text in combined NDM envelope"));
                    }
                }
                Event::GeneralRef(reference) => {
                    if !reference
                        .resolve_char_ref()?
                        .is_some_and(|character| matches!(character, ' ' | '\t' | '\r' | '\n'))
                    {
                        return Err(invalid("unexpected text in combined NDM envelope"));
                    }
                }
                Event::Comment(_) | Event::PI(_) => {}
                _ => return Err(invalid("unexpected content in combined NDM envelope")),
            }
            buf.clear();
        }

        loop {
            buf.clear();
            match reader.read_event_into(&mut buf)? {
                Event::Eof => break,
                Event::Text(text)
                    if is_ascii_whitespace(
                        text.xml_content()
                            .map_err(quick_xml::Error::from)?
                            .as_bytes(),
                    ) => {}
                Event::Comment(_) | Event::PI(_) => {}
                _ => return Err(invalid("trailing content after combined NDM root")),
            }
        }

        let legacy = messages.iter().any(|message| match message {
            Message::Opm(v) => v.version == "2.0",
            Message::Omm(v) => v.version == "2.0",
            Message::Oem(v) => v.version == "2.0",
            Message::Apm(v) => v.version == "1.0",
            Message::Aem(v) => v.version == "1.0",
            Message::Tdm(v) => v.version == "1.0",
            _ => false,
        });
        if legacy {
            if id.is_some() {
                return Err(invalid(
                    "MESSAGE_ID is not representable in the NDM/XML 1.0 envelope",
                ));
            }
            if root_form == crate::xml::ElementForm::Qualified || qualified_children {
                return Err(invalid(
                    "NDM/XML 1.0 envelope elements must be unqualified (in no namespace)",
                ));
            }
            for message in &messages {
                let compatible = match message {
                    Message::Opm(v) => v.version == "2.0",
                    Message::Omm(v) => v.version == "2.0",
                    Message::Oem(v) => v.version == "2.0",
                    Message::Apm(v) => v.version == "1.0",
                    Message::Aem(v) => v.version == "1.0",
                    Message::Tdm(v) => v.version == "1.0",
                    _ => false,
                };
                if !compatible {
                    return Err(invalid(
                        "constituent editions do not belong to one supported NDM/XML envelope",
                    ));
                }
            }
        }

        Ok(CombinedNdm {
            id,
            comments,
            messages,
        })
    }
}
