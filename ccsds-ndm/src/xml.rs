// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! XML format support.
//!
//! This module handles parsing and generation of CCSDS messages in the XML format.
//! It maps XML elements and attributes to Rust structs using `serde`.
//!
//! # Format Specifics
//!
//! - **Schema**: Compliant with the NDM/XML schemas (XSD) defined by CCSDS.
//! - **Attributes**: Some metadata (like `id` and `version`) are stored as XML attributes (e.g., `<opm id="..." version="3.0">`).
//! - **Units**: In XML, units are typically defined as attributes on the value element (e.g., `<X units="km">123.45</X>`).
//!
//! # Implementation Details
//!
//! - **Engine**: Uses [`quick-xml`](https://docs.rs/quick-xml) for efficient parsing and serialization.
//! - **Validation**: While this parser checks for correct types, full XSD validation is not performed at runtime.

use crate::error::{CcsdsNdmError, FormatError, Result};
use quick_xml::de::from_str as from_xml_str;
use quick_xml::events::Event;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

/// Header for CCSDS XML messages.
const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

pub(crate) fn validate_document_root(s: &str, root: &[u8], type_name: &str) -> Result<()> {
    let mut source_edition = None;
    validate_document(
        s,
        type_name,
        &mut source_edition,
        DocumentRules {
            root: Some(root),
            max_depth: None,
            allow_default_namespace: true,
            child_rule: None,
            attribute_allowed: None,
            is_record: None,
            max_records: None,
        },
    )
}

fn validate_root_start(
    start: &quick_xml::events::BytesStart<'_>,
    root: &[u8],
    type_name: &str,
    allow_default_namespace: bool,
    source_edition: &mut Option<String>,
    invalid: &impl Fn(String) -> CcsdsNdmError,
) -> Result<()> {
    if start.name().as_ref() != root {
        return Err(invalid(format!(
            "expected standalone {type_name} root element '{}'",
            String::from_utf8_lossy(root)
        )));
    }
    let mut unknown_attribute = None;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| invalid(error.to_string()))?;
        if attribute.key.as_ref() == b"version" {
            *source_edition = Some(
                attribute
                    .unescape_value()
                    .map_err(|error| invalid(error.to_string()))?
                    .into_owned(),
            );
        }
        if !(matches!(
            attribute.key.as_ref(),
            b"id"
                | b"version"
                | b"xmlns:xsi"
                | b"xmlns:ndm"
                | b"xsi:noNamespaceSchemaLocation"
                | b"xsi:schemaLocation"
        ) || allow_default_namespace && attribute.key.as_ref() == b"xmlns")
        {
            unknown_attribute.get_or_insert_with(|| attribute.key.as_ref().to_vec());
        }
    }
    if let Some(attribute) = unknown_attribute {
        return Err(invalid(format!(
            "unknown {type_name} root attribute '{}'",
            String::from_utf8_lossy(&attribute)
        )));
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub(crate) struct XmlSequenceRule {
    pub rank: u16,
    pub repeatable: bool,
    /// The enclosing `xsd:sequence` carries `maxOccurs="unbounded"`, so this child opens a fresh
    /// iteration of the group instead of regressing within the current one. `userDefinedType` is
    /// the only such content model in the shipped schemas: it lets `COMMENT` follow
    /// `USER_DEFINED`.
    pub restarts_sequence: bool,
}

impl XmlSequenceRule {
    /// A child of a plain `xsd:sequence`, which every sibling must respect in order.
    pub(crate) fn new(rank: u16, repeatable: bool) -> Self {
        Self {
            rank,
            repeatable,
            restarts_sequence: false,
        }
    }

    /// A child that may reopen its enclosing repeating `xsd:sequence`.
    pub(crate) fn restarting(rank: u16, repeatable: bool) -> Self {
        Self {
            rank,
            repeatable,
            restarts_sequence: true,
        }
    }
}

type ChildRule<'a> = dyn Fn(&[u8], &[u8]) -> Option<XmlSequenceRule> + 'a;
type AttributeRule<'a> = dyn Fn(&[u8], &[u8]) -> bool + 'a;

type RecordRule<'a> = dyn Fn(&[u8]) -> bool + 'a;

struct DocumentRules<'a> {
    root: Option<&'a [u8]>,
    max_depth: Option<usize>,
    allow_default_namespace: bool,
    child_rule: Option<&'a ChildRule<'a>>,
    attribute_allowed: Option<&'a AttributeRule<'a>>,
    /// Families with repeatable history records bound them during this pass, before serde
    /// materializes any of them.
    is_record: Option<&'a RecordRule<'a>>,
    max_records: Option<usize>,
}

/// The message-specific half of XML structural validation: which children a parent admits and in
/// what order, which attributes an element admits, and which elements are countable history
/// records. Everything else about the walk is family-independent.
pub(crate) struct MessageSchema<Child, Attribute, Record> {
    pub child_rule: Child,
    pub attribute_allowed: Attribute,
    pub is_record: Record,
}

/// Validate a standalone XML message in one event pass and retain its source edition for
/// diagnostics.
pub(crate) fn validate_standalone_document<Child, Attribute, Record>(
    s: &str,
    root: &[u8],
    type_name: &str,
    options: &crate::options::ParseOptions,
    source_edition: &mut Option<String>,
    schema: MessageSchema<Child, Attribute, Record>,
) -> Result<()>
where
    Child: Fn(&[u8], &[u8]) -> Option<XmlSequenceRule>,
    Attribute: Fn(&[u8], &[u8]) -> bool,
    Record: Fn(&[u8]) -> bool,
{
    let MessageSchema {
        child_rule,
        attribute_allowed,
        is_record,
    } = schema;
    validate_document(
        s,
        type_name,
        source_edition,
        DocumentRules {
            root: Some(root),
            max_depth: Some(options.max_xml_depth),
            allow_default_namespace: false,
            child_rule: Some(&child_rule),
            attribute_allowed: Some(&attribute_allowed),
            is_record: Some(&is_record),
            max_records: options.max_records,
        },
    )
}

/// Longest element name in the shipped NDM schemas (`ORBIT_LIFETIME_CONFIDENCE_LEVEL`) rounded
/// up. Keeping open element names in a stack buffer avoids one heap allocation per start tag,
/// which dominates the walker on large ephemeris documents.
const MAX_ELEMENT_NAME: usize = 32;

/// The name of an element that is currently open, retained only so that a later child can be
/// matched against its parent.
#[derive(Clone, Copy)]
struct ElementName {
    bytes: [u8; MAX_ELEMENT_NAME],
    len: u8,
}

impl ElementName {
    /// An overlong name is retained as empty. It cannot match any schema rule either way, and an
    /// element only reaches the stack after its own name has already been accepted.
    fn new(name: &[u8]) -> Self {
        let mut bytes = [0u8; MAX_ELEMENT_NAME];
        let len = if name.len() <= MAX_ELEMENT_NAME {
            bytes[..name.len()].copy_from_slice(name);
            name.len()
        } else {
            0
        };
        Self {
            bytes,
            len: len as u8,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }
}

fn validate_document(
    s: &str,
    type_name: &str,
    source_edition: &mut Option<String>,
    rules: DocumentRules<'_>,
) -> Result<()> {
    struct Frame {
        name: ElementName,
        last_rank: Option<u16>,
    }

    let invalid =
        |message: String| CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message)));
    let invalid_sequence =
        |message: String| invalid(format!("invalid {type_name} XML sequence: {message}"));
    let document = s.strip_prefix('\u{feff}').unwrap_or(s);
    let mut reader = quick_xml::Reader::from_str(document);
    let mut stack: Vec<Frame> = Vec::new();
    let mut depth = 0usize;
    let mut root_seen = false;
    let mut root_closed = false;
    let mut event_seen = false;
    let mut records = 0usize;

    fn count_record(records: &mut usize, child: &[u8], rules: &DocumentRules<'_>) -> Result<()> {
        let Some(is_record) = rules.is_record else {
            return Ok(());
        };
        if !is_record(child) {
            return Ok(());
        }
        *records += 1;
        if let Some(limit) = rules.max_records {
            if *records > limit {
                return Err(CcsdsNdmError::ResourceLimitExceeded {
                    resource: "history_records",
                    limit,
                    actual: *records,
                });
            }
        }
        Ok(())
    }

    loop {
        match reader.read_event() {
            Ok(Event::Decl(_)) => {
                if event_seen {
                    return Err(invalid(
                        "an XML declaration, when present, must begin the document".into(),
                    ));
                }
                event_seen = true;
            }
            Ok(Event::Start(start)) => {
                event_seen = true;
                if root_closed {
                    return Err(invalid(format!(
                        "trailing content after {type_name} document"
                    )));
                }
                let child = start.name();
                let child = child.as_ref();
                if !root_seen {
                    if let Some(root) = rules.root {
                        validate_root_start(
                            &start,
                            root,
                            type_name,
                            rules.allow_default_namespace,
                            source_edition,
                            &invalid,
                        )?;
                    }
                    root_seen = true;
                } else if let (Some(parent), Some(child_rule), Some(attribute_allowed)) =
                    (stack.last_mut(), rules.child_rule, rules.attribute_allowed)
                {
                    validate_attributes(&start, child, attribute_allowed, &invalid_sequence)?;
                    apply_sequence_rule(parent, child, child_rule, &invalid_sequence)?;
                    count_record(&mut records, child, &rules)?;
                }
                stack.push(Frame {
                    name: ElementName::new(child),
                    last_rank: None,
                });
                depth += 1;
                if let Some(limit) = rules.max_depth {
                    if depth > limit {
                        return Err(CcsdsNdmError::ResourceLimitExceeded {
                            resource: "xml_depth",
                            limit,
                            actual: depth,
                        });
                    }
                }
            }
            Ok(Event::Empty(start)) => {
                event_seen = true;
                if root_closed {
                    return Err(invalid(format!(
                        "trailing content after {type_name} document"
                    )));
                }
                let child = start.name();
                let child = child.as_ref();
                if !root_seen {
                    if let Some(root) = rules.root {
                        validate_root_start(
                            &start,
                            root,
                            type_name,
                            rules.allow_default_namespace,
                            source_edition,
                            &invalid,
                        )?;
                    }
                    root_seen = true;
                    root_closed = true;
                } else if let (Some(parent), Some(child_rule), Some(attribute_allowed)) =
                    (stack.last_mut(), rules.child_rule, rules.attribute_allowed)
                {
                    validate_attributes(&start, child, attribute_allowed, &invalid_sequence)?;
                    apply_sequence_rule(parent, child, child_rule, &invalid_sequence)?;
                    count_record(&mut records, child, &rules)?;
                }
                // A self-closing element occupies a level even though it never opens a frame,
                // so it has to be measured against the limit the same way a start tag is.
                if let Some(limit) = rules.max_depth {
                    let actual = depth + 1;
                    if actual > limit {
                        return Err(CcsdsNdmError::ResourceLimitExceeded {
                            resource: "xml_depth",
                            limit,
                            actual,
                        });
                    }
                }
            }
            Ok(Event::End(_)) => {
                event_seen = true;
                depth = depth.checked_sub(1).ok_or_else(|| {
                    invalid(format!(
                        "unexpected closing element in {type_name} document"
                    ))
                })?;
                stack.pop();
                if depth == 0 {
                    root_closed = true;
                }
            }
            Ok(Event::Text(text)) => {
                event_seen = true;
                if (root_closed || !root_seen)
                    && !text
                        .xml_content()
                        .map_err(|error| invalid(error.to_string()))?
                        .trim()
                        .is_empty()
                {
                    return Err(invalid(format!("text outside {type_name} root element")));
                }
            }
            Ok(Event::CData(_)) if root_closed || !root_seen => {
                return Err(invalid(format!("CDATA outside {type_name} root element")));
            }
            Ok(Event::DocType(_)) => {
                return Err(invalid(
                    "XML document type declarations are not supported".into(),
                ));
            }
            Ok(Event::Eof) => break,
            Ok(_) => event_seen = true,
            Err(error) => return Err(error.into()),
        }
    }
    fn apply_sequence_rule(
        parent: &mut Frame,
        child: &[u8],
        child_rule: &ChildRule<'_>,
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        let rule = child_rule(parent.name.as_bytes(), child).ok_or_else(|| {
            invalid(format!(
                "unknown child '{}' in '{}'",
                String::from_utf8_lossy(child),
                String::from_utf8_lossy(parent.name.as_bytes())
            ))
        })?;
        // A child of a repeating group that steps backwards is starting the next iteration of
        // that group, not breaking the order, so only a plain sequence rejects a lower rank.
        if parent.last_rank.is_some_and(|last| {
            (rule.rank < last && !rule.restarts_sequence) || (rule.rank == last && !rule.repeatable)
        }) {
            return Err(invalid(format!(
                "duplicate or out-of-order child '{}' in '{}'",
                String::from_utf8_lossy(child),
                String::from_utf8_lossy(parent.name.as_bytes())
            )));
        }
        parent.last_rank = Some(rule.rank);
        Ok(())
    }

    fn validate_attributes(
        start: &quick_xml::events::BytesStart<'_>,
        element: &[u8],
        attribute_allowed: &AttributeRule<'_>,
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|error| invalid(error.to_string()))?;
            if !attribute_allowed(element, attribute.key.as_ref()) {
                return Err(invalid(format!(
                    "unknown attribute '{}' on '{}'",
                    String::from_utf8_lossy(attribute.key.as_ref()),
                    String::from_utf8_lossy(element)
                )));
            }
        }
        Ok(())
    }

    if !root_seen || !root_closed {
        return Err(invalid(format!("incomplete {type_name} XML document")));
    }
    Ok(())
}

/// Enforce schema sequence order without loading an XSD at runtime. Callers provide only the
/// message-specific parent/child registration; serde remains responsible for typed values.
pub(crate) fn validate_element_sequences(
    s: &str,
    type_name: &str,
    child_rule: impl Fn(&[u8], &[u8]) -> Option<XmlSequenceRule>,
    attribute_allowed: impl Fn(&[u8], &[u8]) -> bool,
) -> Result<()> {
    let mut source_edition = None;
    validate_document(
        s,
        type_name,
        &mut source_edition,
        DocumentRules {
            root: None,
            max_depth: None,
            allow_default_namespace: true,
            child_rule: Some(&child_rule),
            attribute_allowed: Some(&attribute_allowed),
            is_record: None,
            max_records: None,
        },
    )
}

/// Deserialize an internal XML representation from a string.
///
/// Complete public messages use [`Ndm::from_xml`](crate::traits::Ndm::from_xml), which adds the
/// message-specific validation gate.
#[cfg(test)]
pub(crate) fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    Ok(from_xml_str(s)?)
}

/// Deserialize a CCSDS NDM message from an XML string with context for better error messages.
///
/// When deserialization fails, the error message includes the message type name
/// for easier debugging.
///
/// # Arguments
///
/// * `s` - The XML string to deserialize
/// * `type_name` - The name of the message type (e.g., "OPM", "CDM") for error context
pub(crate) fn from_str_with_context<T: DeserializeOwned>(s: &str, type_name: &str) -> Result<T> {
    if let Some(error) = crate::validation::xml_text_error("XML document", s) {
        return Err(CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(
            error.to_string(),
        ))));
    }
    from_xml_str(s).map_err(|e| {
        crate::error::CcsdsNdmError::Format(Box::new(FormatError::XmlWithContext {
            context: format!("Failed to parse {} from XML", type_name),
            source: e,
        }))
    })
}

/// Serialize a prevalidated CCSDS NDM message to an XML string.
///
/// This raw serde helper is crate-internal so public callers cannot bypass the validation and
/// edition checks provided by [`Ndm::to_xml`](crate::traits::Ndm::to_xml).
pub(crate) fn to_string<T: Serialize>(t: &T) -> Result<String> {
    let mut output = String::with_capacity(1024);
    output.push_str(XML_HEADER);
    output.push('\n');
    let mut writer = XmlStringWriter {
        output: &mut output,
        invalid_text: false,
    };
    let result = quick_xml::se::to_writer(&mut writer, t);
    if writer.invalid_text {
        return Err(invalid_xml_output());
    }
    result?;
    Ok(output)
}

fn invalid_xml_output() -> CcsdsNdmError {
    crate::error::ValidationError::Generic {
        message: "XML output must contain only XML 1.0 characters".into(),
        line: None,
    }
    .into()
}

struct XmlStringWriter<'a> {
    output: &'a mut String,
    invalid_text: bool,
}

impl FmtWrite for XmlStringWriter<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        if !value.chars().all(crate::validation::is_xml_1_character) {
            self.invalid_text = true;
            return Err(std::fmt::Error);
        }
        self.output.write_str(value)
    }
}

struct XmlPreflightWriter {
    bytes: usize,
    invalid_text: bool,
}

impl FmtWrite for XmlPreflightWriter {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        if !value.chars().all(crate::validation::is_xml_1_character) {
            self.invalid_text = true;
            return Err(std::fmt::Error);
        }
        self.bytes = self.bytes.saturating_add(value.len());
        Ok(())
    }
}

pub(crate) fn preflight<T: Serialize>(value: &T) -> Result<usize> {
    let mut writer = XmlPreflightWriter {
        bytes: XML_HEADER.len() + 1,
        invalid_text: false,
    };
    let result = quick_xml::se::to_writer(&mut writer, value);
    if writer.invalid_text {
        return Err(invalid_xml_output());
    }
    result?;
    Ok(writer.bytes)
}

pub(crate) fn validate_output_text<T: Serialize>(value: &T) -> Result<()> {
    preflight(value).map(|_| ())
}

struct IoFmtWriter<'a, W> {
    output: &'a mut W,
    error: Option<std::io::Error>,
}

impl<W: IoWrite> FmtWrite for IoFmtWriter<'_, W> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        self.output.write_all(value.as_bytes()).map_err(|error| {
            if self.error.is_none() {
                self.error = Some(error);
            }
            std::fmt::Error
        })
    }
}

pub(crate) fn to_writer<W: IoWrite, T: Serialize>(output: &mut W, value: &T) -> Result<()> {
    output.write_all(XML_HEADER.as_bytes())?;
    output.write_all(b"\n")?;

    let mut adapter = IoFmtWriter {
        output,
        error: None,
    };
    let serialization = quick_xml::se::to_writer(&mut adapter, value);
    if let Some(error) = adapter.error {
        return Err(error.into());
    }
    serialization?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Wrapper {
        #[serde(rename = "val")]
        val: String,
    }

    #[test]
    fn test_from_str_success() {
        let xml = r#"<Wrapper><val>hello</val></Wrapper>"#;
        let w: Wrapper = from_str(xml).unwrap();
        assert_eq!(w.val, "hello");
    }

    #[test]
    fn test_from_str_with_context_success() {
        let xml = r#"<Wrapper><val>hello</val></Wrapper>"#;
        let w: Wrapper = from_str_with_context(xml, "Wrapper").unwrap();
        assert_eq!(w.val, "hello");
    }

    #[test]
    fn test_from_str_with_context_error() {
        let xml = r#"<Wrapper><val>hello</val>"#; // malformed XML
        let res: Result<Wrapper> = from_str_with_context(xml, "Wrapper");
        assert!(res.is_err());
        let err = res.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Failed to parse Wrapper from XML"));

        let trailing = r#"<Wrapper><val>one</val></Wrapper><Wrapper><val>two</val></Wrapper>"#;
        assert!(validate_document_root(trailing, b"Wrapper", "Wrapper").is_err());
    }

    #[test]
    fn test_to_string() {
        let w = Wrapper {
            val: "world".to_string(),
        };
        let xml = to_string(&w).unwrap();
        assert!(xml.starts_with(XML_HEADER));
        assert!(xml.contains("<Wrapper>"));
        assert!(xml.contains("<val>world</val>"));
        assert!(xml.contains("</Wrapper>"));
    }
}
