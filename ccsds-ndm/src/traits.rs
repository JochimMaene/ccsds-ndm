// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Core traits for CCSDS NDM message handling.
//!
//! This module defines the primary traits used for parsing and serializing
//! NDM messages in both KVN and XML formats.

use crate::error::Result;
use crate::kvn::de::KvnLine;
use crate::kvn::ser::KvnWriter;
use std::iter::Peekable;

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
pub trait Ndm: Sized + serde::Serialize + serde::de::DeserializeOwned {
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
    T::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_kvn_value(s: &str) -> Result<Self> {
        s.parse::<T>()
            .map_err(|e| crate::error::CcsdsNdmError::KvnParse(e.to_string()))
    }
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

/// Trait for types that can be parsed from KVN token streams.
pub trait FromKvnTokens: Sized {
    /// Parse from a peekable sequence of KVN tokens.
    /// allows the parser to inspect the next token without consuming it.
    fn from_kvn_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self>
    where
        I: Iterator<Item = Result<KvnLine<'a>>>;
}
