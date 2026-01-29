// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Core traits for CCSDS NDM message handling.
//!
//! This module defines the primary traits used for parsing and serializing
//! NDM messages in both KVN and XML formats.

use crate::error::Result;
use crate::kvn::ser::KvnWriter;

/// Core trait for NDM message types.
///
/// All CCSDS message types (OPM, OEM, CDM, etc.) implement this trait,
/// providing a uniform interface for parsing and serialization.
///
/// # Example
///
/// ```no_run
/// use ccsds_ndm::messages::opm::Opm;
/// use ccsds_ndm::traits::Ndm;
///
/// // Parse from KVN
/// let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
///
/// // Serialize to XML
/// let xml = opm.to_xml().unwrap();
/// ```
/// Trait for types that provide semantic validation.
pub trait Validate {
    /// Perform semantic validation on the object.
    ///
    /// Checks for logical consistency beyond syntactic correctness.
    /// For example: `START_TIME <= STOP_TIME`, or `MASS >= 0`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, or a `ValidationError` if invalid.
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Core trait for NDM message types.
///
/// All CCSDS message types (OPM, OEM, CDM, etc.) implement this trait,
/// providing a uniform interface for parsing and serialization.
///
/// # Example
///
/// ```no_run
/// use ccsds_ndm::messages::opm::Opm;
/// use ccsds_ndm::traits::Ndm;
///
/// // Parse from KVN
/// let opm = Opm::from_kvn("CCSDS_OPM_VERS = 3.0\n...").unwrap();
///
/// // Serialize to XML
/// let xml = opm.to_xml().unwrap();
/// ```
pub trait Ndm: Sized + serde::Serialize + Validate {
    /// Serialize the message to KVN (Key-Value Notation) format.
    ///
    /// # Returns
    ///
    /// A string containing the KVN representation of the message.
    fn to_kvn(&self) -> Result<String>;

    /// Parse a message from KVN (Key-Value Notation) format.
    ///
    /// # Arguments
    ///
    /// * `kvn` - The KVN content as a string
    fn from_kvn(kvn: &str) -> Result<Self>;

    /// Serialize the message to XML format.
    ///
    /// # Returns
    ///
    /// A string containing the XML representation of the message.
    fn to_xml(&self) -> Result<String>;

    /// Parse a message from XML format.
    ///
    /// # Arguments
    ///
    /// * `xml` - The XML content as a string
    fn from_xml(xml: &str) -> Result<Self>;
}

/// Trait for types that can be parsed from a KVN value string.
///
/// This is automatically implemented for any type that implements `FromStr`.
pub trait FromKvnValue: Sized {
    /// Parse a value from its KVN string representation.
    ///
    /// # Arguments
    ///
    /// * `s` - The value string (without key or unit)
    fn from_kvn_value(s: &str) -> Result<Self>;
}

impl<T> FromKvnValue for T
where
    T: std::str::FromStr,
    T::Err: Into<crate::error::CcsdsNdmError>,
{
    fn from_kvn_value(s: &str) -> Result<Self> {
        s.parse::<T>().map_err(Into::into)
    }
}

/// Trait for types that can be parsed directly from a float and optional unit.
///
/// This avoids the overhead of formatting a float to a string and then parsing it back.
pub trait FromKvnFloat: Sized {
    /// Create an instance from a float value and optional unit string.
    ///
    /// # Arguments
    ///
    /// * `value` - The float value
    /// * `unit` - The optional unit string
    fn from_kvn_float(value: f64, unit: Option<&str>) -> Result<Self>;
}

/// Trait for types that can be serialized to KVN format.
///
/// Implementors write their KVN representation to the provided [`KvnWriter`].
pub trait ToKvn {
    /// Write the KVN representation to the writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The KVN writer to output to
    fn write_kvn(&self, writer: &mut KvnWriter);
}
