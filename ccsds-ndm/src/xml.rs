// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use quick_xml::de::from_str as from_xml_str;
use quick_xml::se::to_string as to_xml_string;
use serde::{de::DeserializeOwned, Serialize};

/// Header for CCSDS XML messages.
const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

/// Deserialize a CCSDS NDM message from an XML string.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    from_xml_str(s).map_err(CcsdsNdmError::XmlDe)
}

/// Serialize a CCSDS NDM message to an XML string.
///
/// Includes the standard XML declaration.
pub fn to_string<T: Serialize>(t: &T) -> Result<String> {
    let xml_body = to_xml_string(t).map_err(|e| CcsdsNdmError::XmlSer(e.to_string()))?;
    Ok(format!("{}\n{}", XML_HEADER, xml_body))
}
