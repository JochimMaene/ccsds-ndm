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
pub(crate) const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
pub(crate) const XML_DEPTH_LIMIT: usize = 16;

/// Namespace of NDM/XML elements in the qualified schema set (NDM/XML 4.3.4-4.3.5).
const NDM_NAMESPACE: &[u8] = b"urn:ccsds:schema:ndmxml";
/// XML Schema instance namespace that every root must declare (NDM/XML 4.3.3).
const XSI_NAMESPACE: &[u8] = b"http://www.w3.org/2001/XMLSchema-instance";

pub(crate) fn validate_document_root(s: &str, root: &[u8], type_name: &str) -> Result<()> {
    let mut source_edition = None;
    validate_document(
        s,
        type_name,
        &mut source_edition,
        DocumentRules {
            root: Some(root),
            child_rule: None,
            attribute_allowed: None,
        },
    )
}

/// Namespace declarations in scope during a walk. NDM/XML documents declare namespaces on the
/// root, if anywhere, so only tags whose attributes mention `xmlns` are inspected; every other
/// element costs one short byte scan.
#[derive(Default)]
struct Namespaces {
    /// Prefix (empty for the default namespace), namespace name, and declaring element depth.
    bindings: Vec<(Vec<u8>, Vec<u8>, usize)>,
    /// Whether a default namespace declaration is in scope.
    has_default: bool,
}

/// Whether raw attribute bytes contain `xmlns`, without scanning byte windows where no `x`
/// occurs.
fn mentions_xmlns(raw: &[u8]) -> bool {
    let mut rest = raw;
    while let Some(index) = rest.iter().position(|&byte| byte == b'x') {
        if rest[index..].starts_with(b"xmlns") {
            return true;
        }
        rest = &rest[index + 1..];
    }
    false
}

enum Resolved<'a> {
    Unbound,
    Bound(&'a [u8]),
    Unknown,
}

impl Namespaces {
    fn enter(
        &mut self,
        start: &quick_xml::events::BytesStart<'_>,
        depth: usize,
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        if !mentions_xmlns(start.attributes_raw()) {
            return Ok(());
        }
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|error| invalid(error.to_string()))?;
            let prefix: &[u8] = match attribute.key.as_namespace_binding() {
                Some(quick_xml::name::PrefixDeclaration::Default) => b"",
                Some(quick_xml::name::PrefixDeclaration::Named(prefix)) => prefix,
                None => continue,
            };
            let name = attribute
                .unescape_value()
                .map_err(|error| invalid(error.to_string()))?;
            if !prefix.is_empty() && name.is_empty() {
                return Err(invalid(
                    "XML 1.0 namespace prefixes must be bound to non-empty names".into(),
                ));
            }
            self.has_default |= prefix.is_empty();
            self.bindings
                .push((prefix.to_vec(), name.as_bytes().to_vec(), depth));
        }
        Ok(())
    }

    /// Drop the declarations of elements deeper than `depth`, which have closed.
    fn leave(&mut self, depth: usize) {
        let mut popped = false;
        while self
            .bindings
            .last()
            .is_some_and(|(_, _, declared)| *declared > depth)
        {
            self.bindings.pop();
            popped = true;
        }
        if popped {
            self.has_default = self.bindings.iter().any(|(prefix, _, _)| prefix.is_empty());
        }
    }

    /// Resolve a prefix. Unprefixed elements take the default namespace, unprefixed
    /// attributes never do (XML Namespaces 6.2), and `xml` is bound by definition.
    fn resolve(&self, prefix: Option<&[u8]>, element: bool) -> Resolved<'_> {
        const XML_NAMESPACE: &[u8] = b"http://www.w3.org/XML/1998/namespace";
        let prefix = match prefix {
            Some(b"xml") => return Resolved::Bound(XML_NAMESPACE),
            Some(prefix) => prefix,
            None if element => b"",
            None => return Resolved::Unbound,
        };
        match self
            .bindings
            .iter()
            .rev()
            .find(|(bound, _, _)| bound.as_slice() == prefix)
        {
            Some((_, name, _)) if name.is_empty() => Resolved::Unbound,
            Some((_, name, _)) => Resolved::Bound(name),
            None if prefix.is_empty() => Resolved::Unbound,
            None => Resolved::Unknown,
        }
    }
}

/// Whether an element uses the unqualified or the qualified NDM/XML schema set.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ElementForm {
    Unqualified,
    Qualified,
}

fn element_form(
    namespaces: &Namespaces,
    name: quick_xml::name::QName<'_>,
    invalid: &impl Fn(String) -> CcsdsNdmError,
) -> Result<ElementForm> {
    if !namespaces.has_default && !name.as_ref().contains(&b':') {
        return Ok(ElementForm::Unqualified);
    }
    match namespaces.resolve(name.prefix().map(|prefix| prefix.into_inner()), true) {
        Resolved::Unbound => Ok(ElementForm::Unqualified),
        Resolved::Bound(NDM_NAMESPACE) => Ok(ElementForm::Qualified),
        _ => Err(invalid(format!(
            "element '{}' is not in the NDM/XML namespace",
            String::from_utf8_lossy(name.as_ref())
        ))),
    }
}

/// How an attribute participates in NDM/XML validation.
enum AttributeKind<'a> {
    /// A namespace declaration, which XML Namespaces does not treat as an attribute.
    Declaration,
    /// An `xsi:schemaLocation` or `xsi:noNamespaceSchemaLocation` hint, which XSD
    /// permits on any element.
    SchemaLocation,
    /// An unprefixed attribute governed by the message schema.
    Plain(&'a [u8]),
}

fn attribute_kind<'a>(
    namespaces: &Namespaces,
    attribute: &'a quick_xml::events::attributes::Attribute<'_>,
    schema_hints_seen: &mut [bool; 2],
    invalid: &impl Fn(String) -> CcsdsNdmError,
) -> Result<AttributeKind<'a>> {
    let key = attribute.key.as_ref();
    if attribute.key.as_namespace_binding().is_some() {
        return Ok(AttributeKind::Declaration);
    }
    // XML Namespaces 6.2: an unprefixed attribute is in no namespace, whatever the default.
    if !key.contains(&b':') {
        return Ok(AttributeKind::Plain(key));
    }
    let local = attribute.key.local_name();
    match namespaces.resolve(
        attribute.key.prefix().map(|prefix| prefix.into_inner()),
        false,
    ) {
        Resolved::Unbound => Ok(AttributeKind::Plain(key)),
        Resolved::Bound(XSI_NAMESPACE)
            if matches!(
                local.as_ref(),
                b"schemaLocation" | b"noNamespaceSchemaLocation"
            ) =>
        {
            let index = usize::from(local.as_ref() == b"noNamespaceSchemaLocation");
            if std::mem::replace(&mut schema_hints_seen[index], true) {
                return Err(invalid("duplicate XML schema-location attribute".into()));
            }
            Ok(AttributeKind::SchemaLocation)
        }
        _ => Err(invalid(format!(
            "unsupported attribute '{}'",
            String::from_utf8_lossy(attribute.key.as_ref())
        ))),
    }
}

fn validate_root_start(
    namespaces: &Namespaces,
    start: &quick_xml::events::BytesStart<'_>,
    root: &[u8],
    type_name: &str,
    source_edition: &mut Option<String>,
    invalid: &impl Fn(String) -> CcsdsNdmError,
) -> Result<()> {
    if start.local_name().as_ref() != root {
        return Err(invalid(format!(
            "expected standalone {type_name} root element '{}'",
            String::from_utf8_lossy(root)
        )));
    }
    // The books show both an unprefixed and an `ndm:`-prefixed root for the qualified set.
    element_form(namespaces, start.name(), invalid)?;
    let mut xsi_declared = false;
    let mut unknown_attribute = None;
    let mut schema_hints_seen = [false; 2];
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| invalid(error.to_string()))?;
        validate_attribute_value(&attribute, invalid)?;
        let kind = match attribute_kind(namespaces, &attribute, &mut schema_hints_seen, invalid) {
            Ok(kind) => kind,
            Err(_) => {
                unknown_attribute.get_or_insert_with(|| attribute.key.as_ref().to_vec());
                continue;
            }
        };
        match kind {
            AttributeKind::Declaration => {
                xsi_declared |= attribute.key.as_ref() == b"xmlns:xsi"
                    && attribute.value.as_ref() == XSI_NAMESPACE;
            }
            AttributeKind::SchemaLocation | AttributeKind::Plain(b"id") => {}
            AttributeKind::Plain(b"version") => {
                *source_edition = Some(
                    attribute
                        .unescape_value()
                        .map_err(|error| invalid(error.to_string()))?
                        .into_owned(),
                );
            }
            AttributeKind::Plain(key) => {
                unknown_attribute.get_or_insert_with(|| key.to_vec());
            }
        }
    }
    if let Some(attribute) = unknown_attribute {
        return Err(invalid(format!(
            "unknown {type_name} root attribute '{}'",
            String::from_utf8_lossy(&attribute)
        )));
    }
    if !xsi_declared {
        return Err(invalid(format!(
            "the {type_name} root element must declare \
             xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\""
        )));
    }
    Ok(())
}

// Even ignored schema-location/namespace attributes must be well-formed XML.
fn validate_attribute_value(
    attribute: &quick_xml::events::attributes::Attribute<'_>,
    invalid: &impl Fn(String) -> CcsdsNdmError,
) -> Result<()> {
    // Printable ASCII without markup is valid as written, which covers `units` and most values.
    if attribute
        .value
        .iter()
        .all(|&byte| (b' '..=b'~').contains(&byte) && byte != b'<' && byte != b'&')
    {
        return Ok(());
    }
    if attribute.value.contains(&b'<') {
        return Err(invalid("XML attribute values must escape '<'".into()));
    }
    let value = attribute
        .unescape_value()
        .map_err(|error| invalid(error.to_string()))?;
    if !value.chars().all(crate::validation::is_xml_1_character) {
        return Err(invalid(
            "XML attributes must contain only XML 1.0 characters".into(),
        ));
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

struct DocumentRules<'a> {
    root: Option<&'a [u8]>,
    child_rule: Option<&'a ChildRule<'a>>,
    attribute_allowed: Option<&'a AttributeRule<'a>>,
}

/// The message-specific half of XML structural validation: which children a parent admits and in
/// what order, which attributes an element admits, and which elements are countable history
/// records. Everything else about the walk is family-independent.
pub(crate) struct MessageSchema<Child, Attribute> {
    pub child_rule: Child,
    pub attribute_allowed: Attribute,
}

/// Validate a standalone XML message in one event pass and retain its source edition for
/// diagnostics.
pub(crate) fn validate_standalone_document<Child, Attribute>(
    s: &str,
    root: &[u8],
    type_name: &str,
    source_edition: &mut Option<String>,
    schema: MessageSchema<Child, Attribute>,
) -> Result<()>
where
    Child: Fn(&[u8], &[u8]) -> Option<XmlSequenceRule>,
    Attribute: Fn(&[u8], &[u8]) -> bool,
{
    let MessageSchema {
        child_rule,
        attribute_allowed,
    } = schema;
    validate_document(
        s,
        type_name,
        source_edition,
        DocumentRules {
            root: Some(root),
            child_rule: Some(&child_rule),
            attribute_allowed: Some(&attribute_allowed),
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
        has_text: bool,
    }

    let invalid =
        |message: String| CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(message)));
    let invalid_sequence =
        |message: String| invalid(format!("invalid {type_name} XML sequence: {message}"));
    let document = s.strip_prefix('\u{feff}').unwrap_or(s);
    // NDM/XML 4.2 and ODM 8.2: the first line of each instantiation is exactly the XML
    // declaration.
    if rules.root.is_some()
        && !document
            .strip_prefix(XML_HEADER)
            .is_some_and(|rest| rest.starts_with(['\r', '\n']))
    {
        return Err(invalid(format!(
            "the first line of an XML instantiation must be exactly {XML_HEADER}"
        )));
    }
    let mut reader = quick_xml::Reader::from_str(document);
    reader.config_mut().check_comments = true;
    let mut namespaces = Namespaces::default();
    let mut child_form = None;
    let mut stack: Vec<Frame> = Vec::new();
    let mut depth = 0usize;
    let mut root_seen = false;
    let mut root_closed = false;
    let mut event_seen = false;

    loop {
        // Matched by reference so start tags are not moved out of the event.
        let event = reader.read_event();
        match event {
            Ok(Event::Decl(_)) => {
                if event_seen {
                    return Err(invalid(
                        "an XML declaration, when present, must begin the document".into(),
                    ));
                }
                event_seen = true;
            }
            Ok(Event::Start(ref start) | Event::Empty(ref start)) => {
                event_seen = true;
                let self_closing = matches!(event, Ok(Event::Empty(_)));
                if root_closed {
                    return Err(invalid(format!(
                        "trailing content after {type_name} document"
                    )));
                }
                namespaces.enter(start, depth + 1, &invalid)?;
                let child = start.local_name();
                let child = child.as_ref();
                if !root_seen {
                    if let Some(root) = rules.root {
                        validate_root_start(
                            &namespaces,
                            start,
                            root,
                            type_name,
                            source_edition,
                            &invalid,
                        )?;
                    }
                    root_seen = true;
                } else {
                    // NDM/XML 4.3.5: a qualified instantiation prefixes every element, so the
                    // two schema forms cannot be mixed below the root.
                    let form = element_form(&namespaces, start.name(), &invalid_sequence)?;
                    if *child_form.get_or_insert(form) != form {
                        return Err(invalid_sequence(
                            "qualified and unqualified NDM/XML elements cannot be mixed".into(),
                        ));
                    }
                    if let (Some(parent), Some(child_rule), Some(attribute_allowed)) =
                        (stack.last_mut(), rules.child_rule, rules.attribute_allowed)
                    {
                        validate_attributes(
                            &namespaces,
                            start,
                            child,
                            attribute_allowed,
                            &invalid_sequence,
                        )?;
                        apply_sequence_rule(parent, child, child_rule, &invalid_sequence)?;
                    }
                }
                // A self-closing element occupies a level even though it never opens a frame,
                // so it has to be measured against the limit the same way a start tag is.
                let actual = depth + 1;
                if actual > XML_DEPTH_LIMIT {
                    return Err(CcsdsNdmError::ResourceLimitExceeded {
                        resource: "xml_depth",
                        limit: XML_DEPTH_LIMIT,
                        actual,
                    });
                }
                if !self_closing {
                    stack.push(Frame {
                        name: ElementName::new(child),
                        last_rank: None,
                        has_text: false,
                    });
                    depth = actual;
                } else {
                    namespaces.leave(depth);
                    if depth == 0 {
                        root_closed = true;
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
                namespaces.leave(depth);
                if depth == 0 {
                    root_closed = true;
                }
            }
            Ok(Event::Text(text)) => {
                event_seen = true;
                // XML 1.0 2.4 forbids the CDATA closing delimiter in literal character data.
                if text.windows(3).any(|bytes| bytes == b"]]>") {
                    return Err(invalid("literal ']]>' is not allowed in XML text".into()));
                }
                // References arrive as separate events, so the raw bytes decide whitespace
                // without decoding every value.
                check_text(&text, &mut stack, &invalid)?;
            }
            Ok(Event::CData(text)) => {
                event_seen = true;
                if stack.is_empty() {
                    return Err(invalid(format!("CDATA outside {type_name} root element")));
                }
                check_text(&text, &mut stack, &invalid)?;
            }
            Ok(Event::GeneralRef(reference)) => {
                event_seen = true;
                if stack.is_empty() {
                    return Err(invalid(format!(
                        "entity reference outside {type_name} root element"
                    )));
                }
                let character = match reference.resolve_char_ref()? {
                    Some(character) => character,
                    None => match &*reference {
                        b"amp" => '&',
                        b"lt" => '<',
                        b"gt" => '>',
                        b"apos" => '\'',
                        b"quot" => '"',
                        _ => return Err(invalid("unknown XML entity reference".into())),
                    },
                };
                if !crate::validation::is_xml_1_character(character) {
                    return Err(invalid(
                        "character references must contain only XML 1.0 characters".into(),
                    ));
                }
                check_text(
                    character.encode_utf8(&mut [0; 4]).as_bytes(),
                    &mut stack,
                    &invalid,
                )?;
            }
            Ok(Event::DocType(_)) => {
                return Err(invalid(
                    "XML document type declarations are not supported".into(),
                ));
            }
            Ok(Event::PI(pi)) => {
                event_seen = true;
                if pi.target().is_empty() || pi.target().eq_ignore_ascii_case(b"xml") {
                    return Err(invalid("invalid XML processing-instruction target".into()));
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => event_seen = true,
            Err(error) => return Err(error.into()),
        }
    }
    fn check_text(
        text: &[u8],
        stack: &mut [Frame],
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        // XSD 1.0 3.4.4(2.3) tests character codes, including characters from
        // references and CDATA (XML Infoset 2.6), rather than their source spelling.
        if text
            .iter()
            .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        {
            return Ok(());
        }
        let parent = stack
            .last_mut()
            .ok_or_else(|| invalid("text outside XML root element".into()))?;
        if parent.last_rank.is_some() {
            return Err(invalid(
                "text is not allowed between XML child elements".into(),
            ));
        }
        parent.has_text = true;
        Ok(())
    }

    fn apply_sequence_rule(
        parent: &mut Frame,
        child: &[u8],
        child_rule: &ChildRule<'_>,
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        if parent.has_text {
            return Err(invalid(
                "text is not allowed before XML child elements".into(),
            ));
        }
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
        namespaces: &Namespaces,
        start: &quick_xml::events::BytesStart<'_>,
        element: &[u8],
        attribute_allowed: &AttributeRule<'_>,
        invalid: &impl Fn(String) -> CcsdsNdmError,
    ) -> Result<()> {
        let mut schema_hints_seen = [false; 2];
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|error| invalid(error.to_string()))?;
            validate_attribute_value(&attribute, invalid)?;
            let unknown =
                match attribute_kind(namespaces, &attribute, &mut schema_hints_seen, invalid) {
                    Ok(AttributeKind::Declaration | AttributeKind::SchemaLocation) => false,
                    Ok(AttributeKind::Plain(key)) => !attribute_allowed(element, key),
                    Err(_) => true,
                };
            if unknown {
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
            child_rule: Some(&child_rule),
            attribute_allowed: Some(&attribute_allowed),
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
    let document = without_namespace_attributes(s)?;
    from_xml_str(&document).map_err(|e| {
        crate::error::CcsdsNdmError::Format(Box::new(FormatError::XmlWithContext {
            context: format!("Failed to parse {} from XML", type_name),
            source: e,
        }))
    })
}

/// Remove namespace declarations and XSI schema-location hints below the root.
///
/// They are not message content, but serde would otherwise see them as unknown fields. The
/// document is copied only when it may contain such an attribute below the root.
fn without_namespace_attributes(s: &str) -> Result<std::borrow::Cow<'_, str>> {
    use quick_xml::name::{Namespace, ResolveResult};

    let mut reader = quick_xml::NsReader::from_str(s);
    let mut xsi_prefixes = vec!["xsi:".to_owned()];
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                for attribute in start.attributes().flatten() {
                    if let Some(quick_xml::name::PrefixDeclaration::Named(prefix)) =
                        attribute.key.as_namespace_binding()
                    {
                        if attribute.value.as_ref() == XSI_NAMESPACE {
                            xsi_prefixes.push(format!("{}:", String::from_utf8_lossy(prefix)));
                        }
                    }
                }
                break;
            }
            Event::Eof => return Ok(std::borrow::Cow::Borrowed(s)),
            _ => {}
        }
    }
    let below_root = &s[reader.buffer_position() as usize..];
    if !below_root.contains("xmlns")
        && !xsi_prefixes
            .iter()
            .any(|prefix| below_root.contains(prefix.as_str()))
    {
        return Ok(std::borrow::Cow::Borrowed(s));
    }

    let mut reader = quick_xml::NsReader::from_str(s);
    let mut writer = quick_xml::Writer::new(Vec::with_capacity(s.len()));
    let mut depth = 0usize;
    loop {
        let event = reader.read_event()?;
        let event = match event {
            Event::Eof => break,
            Event::Start(ref start) | Event::Empty(ref start) if depth > 0 => {
                let mut kept = start.to_owned();
                kept.clear_attributes();
                for attribute in start.attributes() {
                    let attribute = attribute.map_err(quick_xml::Error::from)?;
                    let hint = matches!(
                        reader.resolver().resolve_attribute(attribute.key),
                        (ResolveResult::Bound(Namespace(XSI_NAMESPACE)), local)
                            if matches!(
                                local.as_ref(),
                                b"schemaLocation" | b"noNamespaceSchemaLocation"
                            )
                    );
                    if attribute.key.as_namespace_binding().is_none() && !hint {
                        kept.push_attribute(attribute);
                    }
                }
                if matches!(event, Event::Start(_)) {
                    Event::Start(kept)
                } else {
                    Event::Empty(kept)
                }
            }
            event => event,
        };
        match &event {
            Event::Start(_) => depth += 1,
            Event::End(_) => depth -= 1,
            _ => {}
        }
        writer.write_event(event)?;
    }
    String::from_utf8(writer.into_inner())
        .map(std::borrow::Cow::Owned)
        .map_err(|error| {
            CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(error.to_string())))
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
    let result = quick_xml::se::to_writer(RootNamespaces::new(&mut writer), t);
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

/// Namespace declarations for generated roots: NDM/XML 4.3.3-4.3.4 and ODM 8.3.3.
const ROOT_NAMESPACES: &str = concat!(
    r#" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#,
    r#" xmlns:ndm="urn:ccsds:schema:ndmxml""#
);

/// Inserts [`ROOT_NAMESPACES`] and preserves carriage returns in serialized values.
struct RootNamespaces<W> {
    inner: W,
    state: RootNameState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RootNameState {
    BeforeName,
    InName,
    Done,
}

impl<W> RootNamespaces<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            state: RootNameState::BeforeName,
        }
    }
}

impl<W: FmtWrite> FmtWrite for RootNamespaces<W> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        // XML 1.0 normalizes literal CR and CRLF to LF. quick-xml leaves CR literal
        // in strings, so use character references to preserve the model's text.
        if value.contains('\r') {
            for (index, part) in value.split('\r').enumerate() {
                if index > 0 {
                    self.inner.write_str("&#13;")?;
                }
                self.write_str(part)?;
            }
            return Ok(());
        }
        if self.state == RootNameState::Done {
            return self.inner.write_str(value);
        }
        for (index, character) in value.char_indices() {
            match (self.state, character) {
                (RootNameState::BeforeName, '<') => self.state = RootNameState::InName,
                (RootNameState::InName, ' ' | '/' | '>') => {
                    self.state = RootNameState::Done;
                    self.inner.write_str(&value[..index])?;
                    self.inner.write_str(ROOT_NAMESPACES)?;
                    return self.inner.write_str(&value[index..]);
                }
                _ => {}
            }
        }
        self.inner.write_str(value)
    }
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
    let result = quick_xml::se::to_writer(RootNamespaces::new(&mut writer), value);
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
    let serialization = quick_xml::se::to_writer(RootNamespaces::new(&mut adapter), value);
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
        assert!(xml.contains(&format!("<Wrapper{ROOT_NAMESPACES}>")));
        assert!(xml.contains("<val>world</val>"));
        assert!(xml.contains("</Wrapper>"));
    }
}
