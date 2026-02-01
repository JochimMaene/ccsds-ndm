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
