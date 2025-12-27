// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! # CCSDS NDM
//!
//! A library for parsing and generating CCSDS Navigation Data Messages (NDM)
//! in both KVN (Key-Value Notation) and XML formats.
//!
//! ## Supported Message Types
//!
//! - **OPM** - Orbit Parameter Message
//! - **OMM** - Orbit Mean-Elements Message
//! - **OEM** - Orbit Ephemeris Message
//! - **OCM** - Orbit Comprehensive Message
//! - **CDM** - Conjunction Data Message
//! - **TDM** - Tracking Data Message
//! - **RDM** - Reentry Data Message
//!
//! ## Quick Start
//!
//! ### Parse any NDM file (auto-detection)
//!
//! ```no_run
//! use ccsds_ndm::{from_file, MessageType};
//!
//! let ndm = from_file("example.opm").unwrap();
//!
//! match ndm {
//!     MessageType::Opm(opm) => {
//!         println!("Object: {}", opm.body.segment.metadata.object_name);
//!     }
//!     _ => {}
//! }
//! ```
//!
//! ### Parse a specific message type
//!
//! ```no_run
//! use ccsds_ndm::messages::opm::Opm;
//! use ccsds_ndm::traits::Ndm;
//!
//! let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
//! ```
//!
//! ### Serialize to KVN or XML
//!
//! ```no_run
//! use ccsds_ndm::{from_file, MessageType};
//!
//! let ndm = from_file("example.opm").unwrap();
//! let kvn_string = ndm.to_kvn().unwrap();
//! let xml_string = ndm.to_xml().unwrap();
//! ```

pub mod common;
pub mod error;
pub mod kvn;
pub mod messages;
pub mod traits;
pub mod types;
pub mod xml;

use error::{CcsdsNdmError, Result};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::Path;

/// A generic container for any parsed NDM message.
///
/// This enum wraps all supported CCSDS message types, allowing uniform handling
/// of messages when the type is not known at compile time.
///
/// # Example
///
/// ```no_run
/// use ccsds_ndm::{from_str, MessageType};
///
/// let ndm = from_str("CCSDS_OPM_VERS = 3.0\n...").unwrap();
///
/// match ndm {
///     MessageType::Opm(opm) => println!("Got OPM"),
///     MessageType::Oem(oem) => println!("Got OEM"),
///     _ => println!("Other message type"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Orbit Ephemeris Message - orbit state time series with optional covariance.
    Oem(messages::oem::Oem),
    /// Conjunction Data Message - collision assessment data between two objects.
    Cdm(messages::cdm::Cdm),
    /// Orbit Parameter Message - single state vector and orbital parameters.
    Opm(messages::opm::Opm),
    /// Orbit Mean-Elements Message - mean orbital elements (e.g., TLE-like).
    Omm(messages::omm::Omm),
    /// Reentry Data Message - reentry prediction information.
    Rdm(messages::rdm::Rdm),
    /// Tracking Data Message - ground station tracking measurements.
    Tdm(messages::tdm::Tdm),
    /// Orbit Comprehensive Message - detailed orbit data with maneuvers.
    Ocm(messages::ocm::Ocm),
}

impl MessageType {
    /// Serialize NDM to a KVN string.
    pub fn to_kvn(&self) -> Result<String> {
        match self {
            MessageType::Oem(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Cdm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Opm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Omm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Rdm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Tdm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Ocm(msg) => crate::traits::Ndm::to_kvn(msg),
        }
    }

    /// Serialize NDM to an XML string.
    pub fn to_xml(&self) -> Result<String> {
        match self {
            MessageType::Oem(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Cdm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Opm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Omm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Rdm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Tdm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Ocm(msg) => crate::traits::Ndm::to_xml(msg),
        }
    }

    /// Write KVN to a file.
    pub fn to_kvn_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let kvn = self.to_kvn()?;
        fs::write(path, kvn).map_err(CcsdsNdmError::from)
    }

    /// Write XML to a file.
    pub fn to_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.to_xml()?;
        fs::write(path, xml).map_err(CcsdsNdmError::from)
    }
}

/// Parse an NDM from a string, auto-detecting the message format (KVN or XML) and type.
///
/// This function inspects the input to determine whether it's KVN or XML format,
/// then parses the appropriate message type based on the version header (KVN) or
/// root element (XML).
///
/// # Arguments
///
/// * `s` - The NDM content as a string (KVN or XML format)
///
/// # Returns
///
/// A [`MessageType`] variant containing the parsed message, or an error if
/// parsing fails or the message type is not supported.
///
/// # Example
///
/// ```no_run
/// use ccsds_ndm::from_str;
///
/// let kvn = "CCSDS_OPM_VERS = 3.0\nCREATION_DATE = 2024-01-01\n...";
/// let ndm = from_str(kvn).unwrap();
/// ```
pub fn from_str(s: &str) -> Result<MessageType> {
    let trimmed = s.trim_start();

    // XML Detection
    if trimmed.starts_with('<') {
        return detect_and_parse_xml(s);
    }

    // KVN Detection
    detect_and_parse_kvn(s)
}

/// Parse an NDM from a file path, auto-detecting the message format (KVN or XML) and type.
///
/// Reads the file contents and delegates to [`from_str`] for parsing.
///
/// # Arguments
///
/// * `path` - Path to the NDM file
///
/// # Returns
///
/// A [`MessageType`] variant containing the parsed message, or an error if
/// the file cannot be read or parsing fails.
///
/// # Example
///
/// ```no_run
/// use ccsds_ndm::from_file;
///
/// let ndm = from_file("satellite.opm").unwrap();
/// ```
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<MessageType> {
    let content = fs::read_to_string(path).map_err(CcsdsNdmError::from)?;
    from_str(&content)
}

/// Helper to detect XML message type by sniffing the root tag.
fn detect_and_parse_xml(s: &str) -> Result<MessageType> {
    let mut reader = Reader::from_str(s);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name_bytes = e.name();
                let name = String::from_utf8_lossy(name_bytes.as_ref()).to_lowercase();

                return match name.as_str() {
                    "oem" => crate::traits::Ndm::from_xml(s).map(MessageType::Oem),
                    "cdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Cdm),
                    "opm" => crate::traits::Ndm::from_xml(s).map(MessageType::Opm),
                    "omm" => crate::traits::Ndm::from_xml(s).map(MessageType::Omm),
                    "rdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Rdm),
                    "tdm" => crate::traits::Ndm::from_xml(s).map(MessageType::Tdm),
                    "ocm" => crate::traits::Ndm::from_xml(s).map(MessageType::Ocm),
                    _ => Err(CcsdsNdmError::UnsupportedMessage(format!(
                        "Unknown or unsupported XML root tag: <{}>",
                        name
                    ))),
                };
            }
            Ok(Event::Decl(_)) | Ok(Event::Comment(_)) | Ok(Event::DocType(_)) => {
                // Skip declarations and comments looking for the root tag
                continue;
            }
            Ok(Event::Eof) => {
                return Err(CcsdsNdmError::Validation(
                    "XML parsing ended prematurely without finding root tag".into(),
                ));
            }
            Err(e) => return Err(CcsdsNdmError::XmlParse(e)),
            _ => continue, // Ignore other events like text/PI before root
        }
    }
}

/// Helper to detect KVN message type by scanning headers.
fn detect_and_parse_kvn(s: &str) -> Result<MessageType> {
    for line in s.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("COMMENT") {
            continue;
        }

        if trimmed_line.starts_with("CCSDS_OEM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Oem);
        }

        if trimmed_line.starts_with("CCSDS_CDM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Cdm);
        }

        if trimmed_line.starts_with("CCSDS_OPM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Opm);
        }

        if trimmed_line.starts_with("CCSDS_OMM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Omm);
        }

        if trimmed_line.starts_with("CCSDS_RDM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Rdm);
        }

        if trimmed_line.starts_with("CCSDS_TDM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Tdm);
        }

        if trimmed_line.starts_with("CCSDS_OCM_VERS") {
            return crate::traits::Ndm::from_kvn(s).map(MessageType::Ocm);
        }

        return Err(CcsdsNdmError::UnsupportedMessage(format!(
            "Could not determine NDM type from KVN header line: '{}'",
            trimmed_line
        )));
    }

    Err(CcsdsNdmError::MissingField("Empty KVN file".into()))
}
