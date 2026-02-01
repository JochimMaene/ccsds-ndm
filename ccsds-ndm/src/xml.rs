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

use crate::error::{FormatError, Result};
use quick_xml::de::from_str as from_xml_str;
use quick_xml::se::to_string as to_xml_string;
use serde::{de::DeserializeOwned, Serialize};

/// Header for CCSDS XML messages.
const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

/// Deserialize a CCSDS NDM message from an XML string.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
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
pub fn from_str_with_context<T: DeserializeOwned>(s: &str, type_name: &str) -> Result<T> {
    from_xml_str(s).map_err(|e| {
        crate::error::CcsdsNdmError::Format(Box::new(FormatError::XmlWithContext {
            context: format!("Failed to parse {} from XML", type_name),
            source: e,
        }))
    })
}

/// Serialize a CCSDS NDM message to an XML string.
///
/// Includes the standard XML declaration.
pub fn to_string<T: Serialize>(t: &T) -> Result<String> {
    let xml_body = to_xml_string(t)?;
    Ok(format!("{}\n{}", XML_HEADER, xml_body))
}


use std::borrow::Cow;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Extract the raw XML content for a specific message tag, searching deeply in the document.
///
/// This function scans the XML document to find the *first* occurrence of the specified
/// nested tag (e.g. "cdm", "opm", "ndm"). It returns the raw substring corresponding
/// to that element, including its start and end tags.
///
/// This allows parsing messages that are wrapped in non-standard envelopes (e.g. SpaceTrack's
/// `<message>...</message>`) or standard wrappers (e.g. `<ndm>...</ndm>`) without strict
/// validation of the root element.
///
/// If the tag is not found, it returns `Err`.
pub fn extract_message_content<'a>(xml: &'a str, tag_name: &str) -> Option<Cow<'a, str>> {
    let mut reader = Reader::from_str(xml);
    // reader.config_mut().trim_text(true); // Don't match whitespace, we want raw extraction

    let mut buf = Vec::new();
    let mut depth = 0;
    let mut start_pos = None;

    loop {
        // We capture position BEFORE reading the event
        let event_start = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name_str = String::from_utf8_lossy(name.as_ref());

                // check if this is the tag we are looking for
                let is_match = if name_str == tag_name {
                    true
                } else if let Some(idx) = name_str.find(':') {
                    &name_str[idx+1..] == tag_name
                } else {
                    false
                };

                if start_pos.is_none() && is_match {
                    // Found the start of our target element
                    start_pos = Some(event_start);
                    depth = 1;
                } else if start_pos.is_some() {
                    depth += 1;
                }
            }
            Ok(Event::End(ref _e)) => {
                if start_pos.is_some() {
                    depth -= 1;
                    if depth == 0 {
                        // Found the matching end tag
                        // reader.buffer_position() is now AFTER the end tag
                        let end_pos = reader.buffer_position() as usize;
                        let start_idx = start_pos.unwrap();
                        return Some(Cow::Borrowed(&xml[start_idx..end_pos]));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => return None,
            _ => (),
        }
        buf.clear();
    }

    None
}

// ... in tests ...
    #[test]
    fn test_extract_message_content() {
        let xml = r#"<message>
    <other>stuff</other>
    <cdm id="123">
        <header>XYZ</header>
        <body>...</body>
    </cdm>
    <tail>end</tail>
</message>"#;
        
        let extracted = extract_message_content(xml, "cdm").expect("Should find cdm");
        println!("Extracted: '{}'", extracted);
        assert!(extracted.trim().starts_with("<cdm"));
        assert!(extracted.contains("id=\"123\""));
        assert!(extracted.trim().ends_with("</cdm>"));
        
        // Should also work for root element
        let extracted_root = extract_message_content(xml, "message").expect("Should find message");
        assert!(extracted_root.trim().starts_with("<message"));
    }
