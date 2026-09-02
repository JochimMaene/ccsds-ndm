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
    let supported = match format {
        OutputFormat::Kvn => spec.kvn_output_versions,
        OutputFormat::Xml => spec.xml_output_versions,
    };
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

    /// Validate the complete model and KVN-specific generation constraints.
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
        ToKvn::validate_kvn(message)?;
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
        self.validate_kvn_representability()
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
impl_versioned_ndm!(crate::messages::omm::Omm, Omm);

impl VersionedNdm for crate::messages::cdm::Cdm {
    const KIND: MessageKind = MessageKind::Cdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::aem::Aem {
    const KIND: MessageKind = MessageKind::Aem;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::ocm::Ocm {
    const KIND: MessageKind = MessageKind::Ocm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::tdm::Tdm {
    const KIND: MessageKind = MessageKind::Tdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::rdm::Rdm {
    const KIND: MessageKind = MessageKind::Rdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}

impl VersionedNdm for crate::messages::opm::Opm {
    const KIND: MessageKind = MessageKind::Opm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_text()
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()
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

    fn write_kvn_to<W: Write>(&self, output: &mut W) -> Result<()> {
        stream_kvn(self, output)
    }
}
