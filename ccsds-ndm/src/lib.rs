// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! # CCSDS NDM
//!
//! A high-performance, type-safe library for parsing and generating CCSDS Navigation Data Messages (NDM)
//! in both KVN (Key-Value Notation) and XML formats.
//!
//! This crate is designed for mission-critical space applications where correctness, performance,
//! and adherence to standards are paramount.
//!
//! ## Key Features
//!
//! - **Comprehensive Support**: Full support for OPM, OMM, OEM, OCM, CDM, TDM, RDM, AEM, APM, and ACM messages.
//! - **Format Agnostic**: Seamlessly convert between KVN and XML formats.
//! - **Type Safety**: Strictly typed units (e.g., `km`, `deg`, `s`) prevent physical unit errors.
//! - **High Performance Parsing**: Utilizes `winnow` and `quick-xml` for efficient, low-allocation parsing.
//! - **Ergonomic Construction**: Uses the builder pattern (via the [`bon`](https://docs.rs/bon) crate) for safe and easy message creation.
//! - **Standard Compliant**: Validates messages against CCSDS 502.0-B-3 and related standards.
//!
//! ## Architecture
//!
//! The library is organized around a few core concepts:
//!
//! - **[`Ndm`](traits::Ndm) Trait**: The unifying interface for all message types. It defines the standard `to_kvn`, `from_kvn`, `to_xml`, and `from_xml` methods.
//! - **[`MessageType`] Enum**: A container that holds any valid NDM. This is the primary return type when parsing files with unknown contents (auto-detection).
//! - **Strong Typing**: All physical quantities (Distance, Velocity, Mass, etc.) are wrapped in the [`UnitValue`](types::UnitValue) struct, ensuring that units are always tracked and validated.
//!
//! ## Quick Start
//!
//! ### 1. Parse any NDM file (auto-detection)
//!
//! The library automatically detects whether the input is KVN or XML and what message type it contains.
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
//!     MessageType::Oem(oem) => {
//!         println!("Ephemeris points: {}", oem.body.segment[0].data.state_vector.len());
//!     }
//!     _ => println!("Other message type"),
//! }
//! ```
//!
//! ### 2. Parse a specific message type
//!
//! If you know the message type in advance, you can parse it directly:
//!
//! ```no_run
//! use ccsds_ndm::messages::opm::Opm;
//! use ccsds_ndm::traits::Ndm;
//!
//! // Parses strict KVN for OPM
//! let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
//! ```
//!
//! ### 3. Generate a message using the Builder Pattern
//!
//! Creating messages from scratch is safe and verbose-free using the `builder()` methods.
//!
//! ```no_run
//! use ccsds_ndm::messages::opm::{Opm, OpmBody, OpmSegment, OpmMetadata, OpmData};
//! use ccsds_ndm::common::{OdmHeader, StateVector};
//! use ccsds_ndm::types::{Epoch, Position, Velocity};
//! use ccsds_ndm::traits::Ndm;
//!
//! let opm = Opm::builder()
//!     .version("3.0")
//!     .header(OdmHeader::builder()
//!         .creation_date("2024-01-01T00:00:00".parse().unwrap())
//!         .originator("EXAMPLE")
//!         .build())
//!     .body(OpmBody::builder()
//!         .segment(OpmSegment::builder()
//!             .metadata(OpmMetadata::builder()
//!                 .object_name("SATELLITE")
//!                 .object_id("2024-001A")
//!                 .center_name("EARTH")
//!                 .ref_frame("GCRF")
//!                 .time_system("UTC")
//!                 .build())
//!             .data(OpmData::builder()
//!                 .state_vector(StateVector::builder()
//!                     .epoch("2024-01-01T12:00:00".parse().unwrap())
//!                     .x(Position::new(7000.0, None))
//!                     .y(Position::new(0.0, None))
//!                     .z(Position::new(0.0, None))
//!                     .x_dot(Velocity::new(0.0, None))
//!                     .y_dot(Velocity::new(7.5, None))
//!                     .z_dot(Velocity::new(0.0, None))
//!                     .build())
//!                 .build())
//!             .build())
//!         .build())
//!     .build();
//!
//! // Convert to KVN string
//! println!("{}", opm.to_kvn().unwrap());
//! ```
//!
//! ### 4. Serialize to KVN or XML
//!
//! ```no_run
//! use ccsds_ndm::{from_file, MessageType};
//!
//! let ndm = from_file("example.opm").unwrap();
//!
//! // Serialize to string
//! let kvn_string = ndm.to_kvn().unwrap();
//! let xml_string = ndm.to_xml().unwrap();
//!
//! // Write to file
//! ndm.to_xml_file("output.xml").unwrap();
//! ```
//!
//! ## Modules
//!
//! - [`messages`]: Supported NDM message types (OPM, OEM, TDM, etc.).
//! - [`traits`]: Core traits like `Ndm` and `UnitValue` handling.
//! - [`types`]: Physical types (Distance, Velocity, Epoch, etc.) and CCSDS enumerations.
//! - [`kvn`] & [`xml`]: Format-specific parsing and serialization logic.

pub mod common;
pub mod detect;
pub mod error;
pub mod kvn;
pub mod messages;
pub mod traits;
pub mod types;
pub mod utils;
pub mod validation;
pub mod xml;

use error::{CcsdsNdmError, Result};
use std::fs;
use std::path::Path;
pub use validation::{take_warnings as take_validation_warnings, ValidationMode};

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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageType {
    /// Orbit Ephemeris Message - orbit state time series with optional covariance.
    #[serde(rename = "oem")]
    Oem(messages::oem::Oem),
    /// Conjunction Data Message - collision assessment data between two objects.
    #[serde(rename = "cdm")]
    Cdm(messages::cdm::Cdm),
    /// Orbit Parameter Message - single state vector and orbital parameters.
    #[serde(rename = "opm")]
    Opm(messages::opm::Opm),
    /// Orbit Mean-Elements Message - mean orbital elements (e.g., TLE-like).
    #[serde(rename = "omm")]
    Omm(messages::omm::Omm),
    /// Reentry Data Message - reentry prediction information.
    #[serde(rename = "rdm")]
    Rdm(messages::rdm::Rdm),
    /// Tracking Data Message - ground station tracking measurements.
    #[serde(rename = "tdm")]
    Tdm(messages::tdm::Tdm),
    /// Orbit Comprehensive Message - detailed orbit data with maneuvers.
    #[serde(rename = "ocm")]
    Ocm(messages::ocm::Ocm),
    /// Attitude Comprehensive Message - detailed attitude data with maneuvers.
    #[serde(rename = "acm")]
    Acm(messages::acm::Acm),
    /// Attitude Ephemeris Message - attitude state time series.
    #[serde(rename = "aem")]
    Aem(messages::aem::Aem),
    /// Attitude Parameter Message - attitude state and parameter data.
    #[serde(rename = "apm")]
    Apm(messages::apm::Apm),
    /// Combined Instantiation NDM - container for multiple messages.
    #[serde(rename = "ndm")]
    Ndm(messages::ndm::CombinedNdm),
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
            MessageType::Acm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Aem(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Apm(msg) => crate::traits::Ndm::to_kvn(msg),
            MessageType::Ndm(msg) => crate::traits::Ndm::to_kvn(msg),
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
            MessageType::Acm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Aem(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Apm(msg) => crate::traits::Ndm::to_xml(msg),
            MessageType::Ndm(msg) => crate::traits::Ndm::to_xml(msg),
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
    detect::detect_message_type(s)
}

/// Parse an NDM from a string with explicit validation mode.
pub fn from_str_with_mode(s: &str, mode: ValidationMode) -> Result<MessageType> {
    validation::with_validation_mode(mode, || detect::detect_message_type(s))
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

/// Parse an NDM from a file with explicit validation mode.
pub fn from_file_with_mode<P: AsRef<Path>>(path: P, mode: ValidationMode) -> Result<MessageType> {
    let content = fs::read_to_string(path).map_err(CcsdsNdmError::from)?;
    from_str_with_mode(&content, mode)
}
