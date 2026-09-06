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

pub(crate) fn to_kvn_string<T: GenerationMetadata + ToKvn>(message: &T) -> Result<String> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Kvn)?;
        message.validate_kvn_model()?;
        // One pass only. The dry run this replaces was a second render into `io::sink()` whose
        // sole outcome was `finish_io`'s lexical check, and `finish_checked` applies that same
        // check to the real pass -- returning an error instead of the string, so nothing leaks.
        let mut writer = crate::kvn::ser::KvnWriter::new();
        ToKvn::write_kvn(message, &mut writer);
        writer.finish_checked()
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Kvn, message.version()))
}

pub(crate) fn to_xml_string<T: GenerationMetadata>(message: &T) -> Result<String> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Xml)?;
        message.validate()?;
        message.validate_xml_model()?;
        // `to_string` applies the same XML 1.0 character check as the preflight and returns an
        // error instead of a string, so a second counting pass would only double the work.
        crate::xml::to_string(message)
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Xml, message.version()))
}

pub(crate) trait GenerationMetadata: Ndm {
    /// Message family used to resolve supported editions.
    const KIND: MessageKind;

    /// Return the edition stored on the message.
    fn version(&self) -> &str;

    /// Model-level XML checks: representability and lexical constraints read off the model.
    ///
    /// Excludes the serialization preflight, so buffered generation can serialize exactly once.
    #[doc(hidden)]
    fn validate_xml_model(&self) -> Result<()> {
        Ok(())
    }

    /// The full XML preflight: model checks plus a serialization pass that never emits bytes.
    ///
    /// Streaming needs this because it writes to the caller's sink directly.
    #[doc(hidden)]
    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_model()?;
        crate::xml::validate_output_text(self)
    }

    /// Model-level KVN checks: semantic validation plus any representability constraint.
    ///
    /// This deliberately excludes the serialization dry run. Buffered generation runs these
    /// checks and then serializes exactly once, because `finish_checked` applies the same
    /// lexical check to the real pass that the dry run applied to a throwaway one. Streaming
    /// still runs the dry run: it writes to the caller's sink directly, so it needs the
    /// separate pass to guarantee an invalid message emits zero bytes.
    #[doc(hidden)]
    fn validate_kvn_model(&self) -> Result<()> {
        self.validate()
    }
}

pub(crate) fn write_kvn_to<T: GenerationMetadata + ToKvn, W: Write>(
    message: &T,
    output: &mut W,
) -> Result<()> {
    (|| {
        validate_output_version(T::KIND, message.version(), OutputFormat::Kvn)?;
        message.validate_kvn_model()?;
        // Streaming writes straight to the caller's sink, so the dry run is what buys the
        // documented guarantee that an invalid message emits zero bytes.
        ToKvn::validate_kvn(message)?;
        let mut writer = crate::kvn::ser::KvnWriter::from_io(output);
        ToKvn::write_kvn(message, &mut writer);
        writer.finish_io()
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Kvn, message.version()))
}

pub(crate) fn write_xml_to<T: GenerationMetadata, W: Write>(
    message: &T,
    output: &mut W,
) -> Result<()> {
    (|| {
        validate_for_generation(T::KIND, message.version(), OutputFormat::Xml, message)?;
        message.validate_xml_output()?;
        crate::xml::to_writer(output, message)
    })()
    .map_err(|error| generation_error(error, T::KIND, OutputFormat::Xml, message.version()))
}

macro_rules! impl_generation_metadata {
    ($type:path, $kind:ident) => {
        impl GenerationMetadata for $type {
            const KIND: MessageKind = MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn validate_kvn_model(&self) -> Result<()> {
                self.validate()
            }
        }
    };
    ($type:path, $kind:ident, kvn_representability) => {
        impl GenerationMetadata for $type {
            const KIND: MessageKind = MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn validate_kvn_model(&self) -> Result<()> {
                self.validate()?;
                self.validate_kvn_representability()
            }
        }
    };
}

impl GenerationMetadata for crate::messages::acm::Acm {
    const KIND: MessageKind = MessageKind::Acm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_model(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn validate_xml_model(&self) -> Result<()> {
        self.validate_xml_representability()
    }
}

impl_generation_metadata!(crate::messages::apm::Apm, Apm);

impl_generation_metadata!(crate::messages::omm::Omm, Omm, kvn_representability);
impl_generation_metadata!(crate::messages::cdm::Cdm, Cdm, kvn_representability);
impl_generation_metadata!(crate::messages::aem::Aem, Aem, kvn_representability);
impl GenerationMetadata for crate::messages::ocm::Ocm {
    const KIND: MessageKind = MessageKind::Ocm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_model(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn validate_xml_model(&self) -> Result<()> {
        self.validate()?;
        self.validate_xml_representability()
    }
}
impl_generation_metadata!(crate::messages::tdm::Tdm, Tdm, kvn_representability);
impl GenerationMetadata for crate::messages::rdm::Rdm {
    const KIND: MessageKind = MessageKind::Rdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_kvn_model(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn validate_xml_model(&self) -> Result<()> {
        self.validate()?;
        self.validate_xml_representability()
    }
}

impl GenerationMetadata for crate::messages::opm::Opm {
    const KIND: MessageKind = MessageKind::Opm;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_xml_model(&self) -> Result<()> {
        self.validate_xml_text()
    }

    // The model walk above covers every text value this family can emit, so neither entry point
    // runs a serialization preflight. Streaming XML writes the prolog before serializing, so it
    // never promised zero bytes here in the first place.
    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_model()
    }

    fn validate_kvn_model(&self) -> Result<()> {
        self.validate()?;
        // OPM's `validate_kvn` walks the model -- it derives line lengths from key and value
        // widths rather than rendering -- so it is a model check, not a serialization preflight,
        // and buffered generation still needs it.
        ToKvn::validate_kvn(self)
    }
}

impl GenerationMetadata for crate::messages::oem::Oem {
    const KIND: MessageKind = MessageKind::Oem;

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_xml_model(&self) -> Result<()> {
        self.validate_xml_text()
    }

    // The model walk above covers every text value this family can emit, so neither entry point
    // runs a serialization preflight. Streaming XML writes the prolog before serializing, so it
    // never promised zero bytes here in the first place.
    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_model()
    }

    fn validate_kvn_model(&self) -> Result<()> {
        // OEM has no separate model pass: its render pass validates every record as it emits it.
        Ok(())
    }
}
