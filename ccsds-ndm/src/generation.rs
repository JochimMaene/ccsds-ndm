// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Validated generation and streaming helpers.

use crate::error::{CcsdsNdmError, DiagnosticNotation, Result};
use crate::traits::{Ndm, ToKvn, Validate};
use crate::validation::MessageKind;
use std::io::Write;

/// Output notation used when checking edition support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputFormat {
    Kvn,
    Xml,
}

impl OutputFormat {
    fn name(self) -> &'static str {
        match self {
            Self::Kvn => "KVN",
            Self::Xml => "XML",
        }
    }

    fn diagnostic(self) -> DiagnosticNotation {
        match self {
            Self::Kvn => DiagnosticNotation::Kvn,
            Self::Xml => DiagnosticNotation::Xml,
        }
    }
}

fn generation_error(
    error: CcsdsNdmError,
    kind: MessageKind,
    format: OutputFormat,
    edition: &str,
) -> CcsdsNdmError {
    error.with_generation_context(kind, format.diagnostic(), edition)
}

pub(crate) fn validate_output_version(
    kind: MessageKind,
    version: &str,
    format: OutputFormat,
) -> Result<()> {
    let spec = crate::versioning::spec(kind).ok_or_else(|| {
        CcsdsNdmError::UnsupportedMessage(format!("{} generation", kind.as_str()))
    })?;
    let supported = spec.output_versions;
    if !supported.contains(&version) {
        return Err(CcsdsNdmError::UnsupportedOutputVersion {
            message_type: kind.as_str(),
            format: format.name(),
            version: version.to_string(),
            supported: supported.join(", "),
        });
    }
    Ok(())
}

pub(crate) fn validate_for_generation(
    kind: MessageKind,
    version: &str,
    format: OutputFormat,
    value: &impl Validate,
) -> Result<()> {
    (|| {
        validate_output_version(kind, version, format)?;
        value.validate()
    })()
    .map_err(|error| generation_error(error, kind, format, version))
}

pub(crate) fn to_kvn_string<T: VersionedNdm + ToKvn>(message: &T) -> Result<String> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Kvn)?;
        message.validate_kvn_output()?;
        let mut writer = crate::kvn::ser::KvnWriter::new();
        ToKvn::write_kvn(message, &mut writer);
        writer.finish_checked()
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Kvn, message.version()))
}

pub(crate) fn to_xml_string<T: VersionedNdm>(message: &T) -> Result<String> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Xml)?;
        message.validate()?;
        message.validate_xml_output()?;
        crate::xml::to_string(message)
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Xml, message.version()))
}

/// Complete NDM messages that support validated streaming generation.
///
/// Generation always preserves the edition stored on the message.
pub trait VersionedNdm: Ndm {
    /// Message family used to resolve supported editions.
    const KIND: MessageKind;

    /// Return the edition stored on the message.
    fn version(&self) -> &str;

    /// Apply message-specific XML lexical checks before serialization.
    #[doc(hidden)]
    fn validate_xml_output(&self) -> Result<()> {
        crate::xml::validate_output_text(self)
    }

    /// The complete KVN preflight: model validation plus every notation-specific constraint.
    ///
    /// Both KVN entry points run this exactly once and nothing else, so streaming never emits
    /// bytes for a message that would fail validation. Implementors own the whole check.
    #[doc(hidden)]
    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()
    }

    /// Stream KVN using the edition stored on the message.
    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()>;

    /// Stream XML using the edition stored on the message.
    fn write_xml_to<W: Write>(&self, output: &mut W) -> Result<()> {
        (|| {
            validate_for_generation(Self::KIND, self.version(), OutputFormat::Xml, self)?;
            self.validate_xml_output()?;
            crate::xml::to_writer(output, self)
        })()
        .map_err(|error| generation_error(error, Self::KIND, OutputFormat::Xml, self.version()))
    }
}

fn stream_kvn<T: VersionedNdm + ToKvn, W: Write>(message: &T, output: &mut W) -> Result<()> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Kvn)?;
        message.validate_kvn_output()?;
        let mut writer = crate::kvn::ser::KvnWriter::from_io(output);
        ToKvn::write_kvn(message, &mut writer);
        writer.finish_io()
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Kvn, message.version()))
}

macro_rules! impl_versioned_ndm {
    ($type:path, $kind:ident) => {
        impl VersionedNdm for $type {
            const KIND: MessageKind = MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn validate_kvn_output(&self) -> Result<()> {
                self.validate()?;
                ToKvn::validate_kvn(self)
            }

            fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
                stream_kvn(self, output)
            }
        }
    };
    ($type:path, $kind:ident, kvn_representability) => {
        impl VersionedNdm for $type {
            const KIND: MessageKind = MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn validate_kvn_output(&self) -> Result<()> {
                self.validate()?;
                self.validate_kvn_representability()?;
                ToKvn::validate_kvn(self)
            }

            fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
                stream_kvn(self, output)
            }
        }
    };
}

impl VersionedNdm for crate::messages::acm::Acm {
    const KIND: MessageKind = MessageKind::Acm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()?;
        ToKvn::validate_kvn(self)
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_representability()?;
        crate::xml::validate_output_text(self)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl_versioned_ndm!(crate::messages::apm::Apm, Apm);

impl_versioned_ndm!(crate::messages::omm::Omm, Omm, kvn_representability);
impl_versioned_ndm!(crate::messages::cdm::Cdm, Cdm, kvn_representability);
impl_versioned_ndm!(crate::messages::aem::Aem, Aem, kvn_representability);
impl_versioned_ndm!(crate::messages::ocm::Ocm, Ocm, kvn_representability);
impl_versioned_ndm!(crate::messages::tdm::Tdm, Tdm, kvn_representability);
impl_versioned_ndm!(crate::messages::rdm::Rdm, Rdm, kvn_representability);

impl VersionedNdm for crate::messages::opm::Opm {
    const KIND: MessageKind = MessageKind::Opm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_text()
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        ToKvn::validate_kvn(self)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::oem::Oem {
    const KIND: MessageKind = MessageKind::Oem;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_text()
    }

    fn validate_kvn_output(&self) -> Result<()> {
        // OEM validates each record as it renders it, so its ToKvn pass is already complete.
        ToKvn::validate_kvn(self)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}
