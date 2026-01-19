// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

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
        crate::error::CcsdsNdmError::Format(Box::new(FormatError::InvalidFormat(format!(
            "Failed to parse {} from XML: {}",
            type_name, e
        ))))
    })
}

/// Serialize a CCSDS NDM message to an XML string.
///
/// Includes the standard XML declaration.
pub fn to_string<T: Serialize>(t: &T) -> Result<String> {
    let xml_body = to_xml_string(t)?;
    Ok(format!("{}\n{}", XML_HEADER, xml_body))
}
