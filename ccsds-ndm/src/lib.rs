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
//! Parsing is strict.
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
//! Generation validates output and preserves the edition stored on the message.
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
mod utils;
pub mod validation;
pub mod versioning;
pub(crate) mod xml;

pub use conversion::{convert, convert_file, convert_file_with_options, convert_with_options};
pub use detect::Notation;
use error::{CcsdsNdmError, Result};
pub use generation::VersionedNdm;
pub(crate) use kvn::parser::parse_block;
pub use options::ParseOptions;
use std::fs;
use std::io::Write;
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
    /// Return the CCSDS message family represented by this value.
    pub const fn kind(&self) -> validation::MessageKind {
        match self {
            Self::Opm(_) => validation::MessageKind::Opm,
            Self::Omm(_) => validation::MessageKind::Omm,
            Self::Oem(_) => validation::MessageKind::Oem,
            Self::Ocm(_) => validation::MessageKind::Ocm,
            Self::Acm(_) => validation::MessageKind::Acm,
            Self::Aem(_) => validation::MessageKind::Aem,
            Self::Apm(_) => validation::MessageKind::Apm,
            Self::Cdm(_) => validation::MessageKind::Cdm,
            Self::Tdm(_) => validation::MessageKind::Tdm,
            Self::Rdm(_) => validation::MessageKind::Rdm,
            Self::Ndm(_) => validation::MessageKind::Ndm,
        }
    }

    fn source_edition(&self) -> &str {
        match self {
            Self::Opm(message) => &message.version,
            Self::Omm(message) => &message.version,
            Self::Oem(message) => &message.version,
            Self::Ocm(message) => &message.version,
            Self::Acm(message) => &message.version,
            Self::Aem(message) => &message.version,
            Self::Apm(message) => &message.version,
            Self::Cdm(message) => &message.version,
            Self::Tdm(message) => &message.version,
            Self::Rdm(message) => &message.version,
            Self::Ndm(_) => "combined",
        }
    }

    pub(crate) fn validate_for_generation(&self, format: generation::OutputFormat) -> Result<()> {
        fn validate<T: VersionedNdm + traits::ToKvn>(
            message: &T,
            format: generation::OutputFormat,
        ) -> Result<()> {
            let result = generation::validate_output_version(T::KIND, message.version(), format)
                .and_then(|()| match format {
                    generation::OutputFormat::Kvn => {
                        message.validate_kvn_output()?;
                        traits::ToKvn::validate_kvn(message)
                    }
                    generation::OutputFormat::Xml => {
                        traits::Validate::validate(message)?;
                        message.validate_xml_output()
                    }
                });
            result.map_err(|error| {
                error.with_generation_context(
                    T::KIND,
                    match format {
                        generation::OutputFormat::Kvn => error::DiagnosticNotation::Kvn,
                        generation::OutputFormat::Xml => error::DiagnosticNotation::Xml,
                    },
                    message.version(),
                )
            })
        }

        match self {
            MessageType::Oem(message) => validate(message, format),
            MessageType::Cdm(message) => validate(message, format),
            MessageType::Opm(message) => validate(message, format),
            MessageType::Omm(message) => validate(message, format),
            MessageType::Rdm(message) => validate(message, format),
            MessageType::Tdm(message) => validate(message, format),
            MessageType::Ocm(message) => validate(message, format),
            MessageType::Acm(message) => validate(message, format),
            MessageType::Aem(message) => validate(message, format),
            MessageType::Apm(message) => validate(message, format),
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
            MessageType::Oem(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Cdm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Opm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Omm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Rdm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Tdm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Ocm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Acm(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Aem(msg) => traits::Ndm::to_kvn(msg),
            MessageType::Apm(msg) => traits::Ndm::to_kvn(msg),
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
            MessageType::Oem(msg) => traits::Ndm::to_xml(msg),
            MessageType::Cdm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Opm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Omm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Rdm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Tdm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Ocm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Acm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Aem(msg) => traits::Ndm::to_xml(msg),
            MessageType::Apm(msg) => traits::Ndm::to_xml(msg),
            MessageType::Ndm(msg) => crate::traits::Ndm::to_xml(msg),
        }
    }

    /// Stream KVN using the edition stored on the message.
    pub fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        match self {
            MessageType::Oem(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Cdm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Opm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Omm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Rdm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Tdm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Ocm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Acm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Aem(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Apm(msg) => VersionedNdm::write_kvn_to(msg, output),
            MessageType::Ndm(msg) => msg.write_kvn_to(output),
        }
    }

    /// Stream XML using the edition stored on the message.
    pub fn write_xml_to<W: Write>(&self, output: &mut W) -> Result<()> {
        match self {
            MessageType::Oem(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Cdm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Opm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Omm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Rdm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Tdm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Ocm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Acm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Aem(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Apm(msg) => VersionedNdm::write_xml_to(msg, output),
            MessageType::Ndm(msg) => msg.write_xml_to(output),
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
        fsutil::atomic_write(path.as_ref(), |output| self.write_kvn_to(output)).map_err(|error| {
            error.with_generation_context(
                self.kind(),
                error::DiagnosticNotation::Kvn,
                self.source_edition(),
            )
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
        fsutil::atomic_write(path.as_ref(), |output| self.write_xml_to(output)).map_err(|error| {
            error.with_generation_context(
                self.kind(),
                error::DiagnosticNotation::Xml,
                self.source_edition(),
            )
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

/// Parse an NDM from a string with optional notation selection and resource limits.
pub fn from_str_with_options(
    input: &str,
    notation: Option<Notation>,
    options: &ParseOptions,
) -> Result<MessageType> {
    detect::detect_message_type_with_options(input, notation, options)
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

/// Parse an NDM file with bounded reading, optional notation selection, and parse limits.
pub fn from_file_with_options<P: AsRef<Path>>(
    path: P,
    notation: Option<Notation>,
    options: &ParseOptions,
) -> Result<MessageType> {
    let content = fsutil::read_to_string(path.as_ref(), options.max_input_bytes)?;
    from_str_with_options(&content, notation, options)
}
