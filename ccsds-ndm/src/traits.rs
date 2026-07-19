// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Core traits for CCSDS NDM message handling.
//!
//! This module defines the primary traits used for parsing and serializing
//! NDM messages in both KVN and XML formats.

use crate::error::{Result, ValidationError};
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
        match self.validation_errors()?.into_iter().next() {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
    }

    /// Return every semantic validation error found on the object.
    ///
    /// Strict parsing and generation use [`Validate::validate`] to fail fast. Permissive parsing
    /// uses this method to produce a complete audit trail. Implementations must explicitly define
    /// exhaustive reporting so adding new validation rules cannot silently fall back to returning
    /// only the first error.
    fn validation_errors(&self) -> Result<Vec<ValidationError>>;
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
    /// Generate KVN using the edition stored on the message.
    ///
    /// Implementations validate the complete message and confirm that KVN generation supports the
    /// stored edition before serializing. Use
    /// [`VersionedNdm::to_kvn_with`](crate::generation::VersionedNdm::to_kvn_with) to select a
    /// different target edition explicitly.
    ///
    /// # Returns
    ///
    /// A complete KVN document.
    ///
    /// # Errors
    ///
    /// Returns an error when the stored edition cannot be generated or the model is invalid.
    fn to_kvn(&self) -> Result<String>;

    /// Parse and validate a complete message from KVN (Key-Value Notation).
    ///
    /// Implementations enforce the message parser's supported syntax and self-contained
    /// validation rules before returning a typed model.
    ///
    /// # Arguments
    ///
    /// * `kvn` - The KVN content as a string
    ///
    /// # Errors
    ///
    /// Returns an error when the input is malformed, unsupported, or invalid.
    fn from_kvn(kvn: &str) -> Result<Self>;

    /// Generate XML using the edition stored on the message.
    ///
    /// Implementations validate the complete message and confirm that XML generation supports
    /// the stored edition before serializing. Use
    /// [`VersionedNdm::to_xml_with`](crate::generation::VersionedNdm::to_xml_with) to select a
    /// different target edition explicitly.
    ///
    /// # Returns
    ///
    /// A complete XML document, including its XML declaration.
    ///
    /// # Errors
    ///
    /// Returns an error when the stored edition cannot be generated, the model is invalid, or XML
    /// serialization fails.
    fn to_xml(&self) -> Result<String>;

    /// Parse and validate a complete message from XML.
    ///
    /// Implementations deserialize the typed message and apply its self-contained validation rules
    /// before returning it. Runtime validation does not invoke an external XSD engine.
    ///
    /// # Arguments
    ///
    /// * `xml` - The XML content as a string
    ///
    /// # Errors
    ///
    /// Returns an error when the input is malformed, unsupported, or invalid.
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

/// Trait to check if a value is considered "null" or "empty" in CCSDS context.
///
/// This unifies the logic for XML (nil="true" or empty text) and KVN (empty value).
pub trait CcsdsNullable {
    /// Returns true if the value represents a null/empty state.
    fn is_null(&self) -> bool;
}

impl CcsdsNullable for String {
    fn is_null(&self) -> bool {
        self.trim().is_empty() || self.trim().eq_ignore_ascii_case("n/a")
    }
}

impl CcsdsNullable for str {
    fn is_null(&self) -> bool {
        self.trim().is_empty() || self.trim().eq_ignore_ascii_case("n/a")
    }
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

/// Internal trait for composing KVN output after the public generation gate has validated a
/// complete message.
///
/// Implementors write their KVN representation to the provided [`KvnWriter`].
pub(crate) trait ToKvn {
    /// Validate notation-specific constraints that must hold before any KVN bytes are written.
    fn validate_kvn(&self) -> Result<()> {
        Ok(())
    }

    /// Write the KVN representation to the writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The KVN writer to output to
    fn write_kvn(&self, writer: &mut KvnWriter<'_>);
}

#[cfg(test)]
mod validate_default_tests {
    use super::Validate;
    use crate::error::{Result, ValidationError};

    struct AggregateOnly;

    impl Validate for AggregateOnly {
        fn validation_errors(&self) -> Result<Vec<ValidationError>> {
            Ok(vec![ValidationError::generic("aggregate-only failure")])
        }
    }

    #[test]
    fn default_validate_fails_when_aggregate_validation_reports_an_error() {
        assert!(AggregateOnly.validate().is_err());
    }
}
