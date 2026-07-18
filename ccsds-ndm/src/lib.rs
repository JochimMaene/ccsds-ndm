// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! # CCSDS NDM
//!
//! A high-performance, type-safe library for parsing and generating CCSDS Navigation Data Messages (NDM)
//! in both KVN (Key-Value Notation) and XML formats.
//!
//! This crate is designed for demanding space-data exchange where correctness, predictable
//! performance, and adherence to the applicable standards are important.
//!
//! ## Key Features
//!
//! - **Broad Message Coverage**: Typed models, parsers, and serializers for OPM, OMM, OEM, OCM,
//!   CDM, TDM, RDM, AEM, APM, and ACM messages. Model availability does not by itself establish
//!   conformance for every edition, notation, or operation.
//! - **Format Agnostic**: Seamlessly convert between KVN and XML formats.
//! - **Type Safety**: Strictly typed units (e.g., `km`, `deg`, `s`) prevent physical unit errors.
//! - **High Performance Parsing**: Utilizes `winnow` and `quick-xml` for efficient, low-allocation parsing.
//! - **Ergonomic Construction**: Uses the builder pattern (via the [`bon`](https://docs.rs/bon) crate) for safe and easy message creation.
//! - **Validation Infrastructure**: Shared syntax and semantic validation for CCSDS NDM workflows.
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
//! Parsing is strict. A future permissive surface will be added only alongside explicit,
//! deterministic recovery rules and structured diagnostics.
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
//! Generation validates output and preserves the source edition by default. An explicit target
//! can be selected without mutating the parsed message:
//!
//! ```no_run
//! use ccsds_ndm::{GenerateOptions, VersionedNdm};
//! # use ccsds_ndm::messages::opm::Opm;
//! # use ccsds_ndm::traits::Ndm;
//! # let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...")?;
//! let xml = opm.to_xml_with(&GenerateOptions::latest())?;
//! # Ok::<(), ccsds_ndm::error::CcsdsNdmError>(())
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
//!
//! Complete messages are parsed and generated through [`Ndm`](traits::Ndm) and
//! [`VersionedNdm`]; notation-specific parser and writer mechanics remain internal.

pub mod common;
pub mod conversion;
pub mod detect;
pub mod error;
mod fsutil;
pub mod generation;
pub(crate) mod kvn;
pub mod messages;
pub mod options;
pub mod traits;
pub mod types;
pub mod utils;
pub mod validation;
pub mod versioning;
pub(crate) mod xml;

pub use conversion::{
    convert_oem, convert_oem_file, convert_oem_to_file, convert_opm, convert_opm_file,
    convert_opm_to_file, parse_oem_file, parse_opm_file, Notation,
};
use error::{CcsdsNdmError, Result};
pub use generation::VersionedNdm;
pub(crate) use kvn::parser::parse_block;
pub use options::{GenerateOptions, ParseOptions, TargetVersion};
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
    pub(crate) fn validate_for_generation(&self, format: generation::OutputFormat) -> Result<()> {
        match self {
            MessageType::Oem(msg) => generation::validate_for_generation(
                validation::MessageKind::Oem,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Cdm(msg) => generation::validate_for_generation(
                validation::MessageKind::Cdm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Opm(msg) => generation::validate_for_generation(
                validation::MessageKind::Opm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Omm(msg) => generation::validate_for_generation(
                validation::MessageKind::Omm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Rdm(msg) => generation::validate_for_generation(
                validation::MessageKind::Rdm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Tdm(msg) => generation::validate_for_generation(
                validation::MessageKind::Tdm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Ocm(msg) => generation::validate_for_generation(
                validation::MessageKind::Ocm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Acm(msg) => generation::validate_for_generation(
                validation::MessageKind::Acm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Aem(msg) => generation::validate_for_generation(
                validation::MessageKind::Aem,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Apm(msg) => generation::validate_for_generation(
                validation::MessageKind::Apm,
                &msg.version,
                format,
                msg,
            ),
            MessageType::Ndm(msg) => {
                for message in &msg.messages {
                    message.validate_for_generation(format)?;
                }
                Ok(())
            }
        }
    }

    /// Generate KVN using the edition stored on the message.
    ///
    /// The complete message is validated before serialization.
    ///
    /// # Errors
    ///
    /// Returns an error when the stored edition cannot be generated or the model is invalid.
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

    /// Generate XML using the edition stored on the message.
    ///
    /// The complete message is validated before serialization.
    ///
    /// # Errors
    ///
    /// Returns an error when the stored edition cannot be generated, the model is invalid, or XML
    /// serialization fails.
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

    /// Generate KVN using an explicit target-edition policy.
    ///
    /// The selected message is fully validated before serialization. Selecting a different
    /// edition does not mutate the contained message. A combined NDM accepts only
    /// [`TargetVersion::Source`] because it has no single root edition.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated, the model is invalid, or
    /// the target policy is not applicable to a combined NDM.
    pub fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        match self {
            MessageType::Oem(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Cdm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Opm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Omm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Rdm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Tdm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Ocm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Acm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Aem(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Apm(msg) => generation::VersionedNdm::to_kvn_with(msg, options),
            MessageType::Ndm(msg) => match &options.target_version {
                TargetVersion::Source => crate::traits::Ndm::to_kvn(msg),
                _ => Err(CcsdsNdmError::UnsupportedMessage(
                    "A combined NDM has no single target version".into(),
                )),
            },
        }
    }

    /// Generate XML using an explicit target-edition policy.
    ///
    /// The selected message is fully validated before serialization. Selecting a different
    /// edition does not mutate the contained message. A combined NDM accepts only
    /// [`TargetVersion::Source`] because it has no single root edition.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated, the model is invalid, the
    /// target policy is not applicable to a combined NDM, or XML serialization fails.
    pub fn to_xml_with(&self, options: &GenerateOptions) -> Result<String> {
        match self {
            MessageType::Oem(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Cdm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Opm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Omm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Rdm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Tdm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Ocm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Acm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Aem(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Apm(msg) => generation::VersionedNdm::to_xml_with(msg, options),
            MessageType::Ndm(msg) => match &options.target_version {
                TargetVersion::Source => crate::traits::Ndm::to_xml(msg),
                _ => Err(CcsdsNdmError::UnsupportedMessage(
                    "A combined NDM has no single target version".into(),
                )),
            },
        }
    }

    /// Generate KVN using the stored edition and write the complete document to a file.
    ///
    /// Generation completes before the destination is opened, so validation errors do not modify
    /// an existing destination.
    ///
    /// # Errors
    ///
    /// Returns a KVN-generation error or an I/O error from writing the destination.
    pub fn to_kvn_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let kvn = self.to_kvn()?;
        fsutil::atomic_write(path.as_ref(), kvn.as_bytes()).map_err(|error| match self {
            MessageType::Opm(message) => error.with_generation_context(
                validation::MessageKind::Opm,
                error::DiagnosticNotation::Kvn,
                &message.version,
                &message.version,
            ),
            _ => error,
        })
    }

    /// Generate XML using the stored edition and write the complete document to a file.
    ///
    /// Generation completes before the destination is opened, so validation or serialization
    /// errors do not modify an existing destination.
    ///
    /// # Errors
    ///
    /// Returns an XML-generation error or an I/O error from writing the destination.
    pub fn to_xml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.to_xml()?;
        fsutil::atomic_write(path.as_ref(), xml.as_bytes()).map_err(|error| match self {
            MessageType::Opm(message) => error.with_generation_context(
                validation::MessageKind::Opm,
                error::DiagnosticNotation::Xml,
                &message.version,
                &message.version,
            ),
            _ => error,
        })
    }
}

/// Parse an NDM from a string, auto-detecting the message format (KVN or XML) and type.
///
/// This function inspects the input to determine whether it's KVN or XML format,
/// then parses and validates the appropriate complete message based on the version header (KVN)
/// or root element (XML).
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
/// # Errors
///
/// Returns an error when detection fails or the selected strict parser rejects the input.
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
/// # Errors
///
/// Returns an I/O error or an error from the selected strict parser.
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
