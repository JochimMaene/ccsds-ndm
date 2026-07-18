// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Version-aware, validated generation helpers.

use crate::error::{CcsdsNdmError, DiagnosticNotation, Result};
use crate::options::{GenerateOptions, TargetVersion};
use crate::traits::{Ndm, ToKvn, Validate};
use crate::validation::MessageKind;
use std::borrow::Cow;
use std::io::Write;

#[derive(Default)]
struct CountingWriter {
    bytes: usize,
}

impl Write for CountingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(buffer.len());
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

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
    source_edition: &str,
    target_edition: &str,
) -> CcsdsNdmError {
    error.with_generation_context(kind, format.diagnostic(), source_edition, target_edition)
}

pub(crate) fn enforce_output_limit(actual: usize, options: &GenerateOptions) -> Result<()> {
    if let Some(limit) = options.max_output_bytes {
        if actual > limit {
            return Err(CcsdsNdmError::ResourceLimitExceeded {
                resource: "generated_document",
                limit,
                actual,
            });
        }
    }
    Ok(())
}

pub(crate) fn preflight_xml_limit<T: serde::Serialize>(
    value: &T,
    options: &GenerateOptions,
) -> Result<()> {
    if options.max_output_bytes.is_none() {
        return Ok(());
    }
    let mut counter = CountingWriter::default();
    crate::xml::to_writer(&mut counter, value)?;
    enforce_output_limit(counter.bytes, options)
}

pub(crate) fn preflight_kvn_limit<T: ToKvn>(value: &T, options: &GenerateOptions) -> Result<()> {
    if options.max_output_bytes.is_none() {
        return Ok(());
    }
    let mut counter = CountingWriter::default();
    let mut writer = crate::kvn::ser::KvnWriter::from_io(&mut counter);
    value.write_kvn(&mut writer);
    writer.finish_io().map_err(CcsdsNdmError::from)?;
    enforce_output_limit(counter.bytes, options)
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
    validate_output_version(kind, version, format)?;
    value.validate()
}

/// NDM messages that carry a CCSDS edition in their root element/header.
///
/// This trait is implemented by the library's complete message types. Notation-specific
/// serialization mechanics remain internal so callers cannot bypass the validation performed by
/// these generation methods.
pub trait VersionedNdm: Ndm + Clone {
    /// Message family used to resolve supported editions.
    const KIND: MessageKind;

    /// Return the edition stored on the message.
    fn version(&self) -> &str;

    /// Update the edition stored on a cloned message before generation.
    fn set_version(&mut self, version: String);

    /// Apply message-specific XML lexical checks before serialization.
    ///
    /// Most message models need no additional check. OPM and OEM use this hook to keep XML
    /// character rules out of notation-neutral model validation while retaining one shared
    /// writer path.
    #[doc(hidden)]
    fn validate_xml_output(&self) -> Result<()> {
        Ok(())
    }

    /// Validate the complete model and KVN-specific generation constraints.
    ///
    /// Most messages have no additional KVN rules beyond model validation. OEM overrides this
    /// hook to validate its large history and notation constraints in one traversal.
    #[doc(hidden)]
    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()
    }

    /// Generate a complete KVN document using an explicit target-edition policy.
    ///
    /// [`TargetVersion::Source`] preserves the stored edition. A different supported target is
    /// applied to a clone, leaving `self` unchanged. The selected message is fully validated
    /// before serialization.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated or the selected model is
    /// invalid.
    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String>;

    /// Generate a complete XML document using an explicit target-edition policy.
    ///
    /// [`TargetVersion::Source`] preserves the stored edition. A different supported target is
    /// applied to a clone, leaving `self` unchanged. The selected message is fully validated
    /// before serialization.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated, the selected model is
    /// invalid, or XML serialization fails.
    fn to_xml_with(&self, options: &GenerateOptions) -> Result<String> {
        with_target_message(self, options, OutputFormat::Xml, |message| {
            message.to_xml().and_then(|output| {
                enforce_output_limit(output.len(), options)?;
                Ok(output)
            })
        })
    }

    /// Stream KVN to an I/O sink using an explicit target-edition policy.
    ///
    /// The selected message is fully validated before any bytes are written. A sink failure can
    /// leave a prefix of the document in `output`, as with other streaming writers.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated, the selected model is
    /// invalid, or the sink rejects a write.
    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()>;

    /// Stream XML to an I/O sink using an explicit target-edition policy.
    ///
    /// The selected message is fully validated before any bytes are written. A sink failure can
    /// leave a prefix of the document in `output`, as with other streaming writers.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected edition cannot be generated, the selected model is
    /// invalid, XML serialization fails, or the sink rejects a write.
    fn write_xml_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        with_target_message(self, options, OutputFormat::Xml, |message| {
            validate_for_generation(Self::KIND, message.version(), OutputFormat::Xml, message)?;
            message.validate_xml_output()?;
            preflight_xml_limit(message, options)?;
            crate::xml::to_writer(output, message)
        })
    }

    fn target_version<'a>(&'a self, options: &'a GenerateOptions) -> Result<Cow<'a, str>> {
        match &options.target_version {
            TargetVersion::Source => Ok(Cow::Borrowed(self.version())),
            TargetVersion::Latest => Ok(Cow::Borrowed(
                crate::versioning::spec(Self::KIND)
                    .ok_or_else(|| {
                        CcsdsNdmError::UnsupportedMessage(format!(
                            "{} generation",
                            Self::KIND.as_str()
                        ))
                    })?
                    .default_version,
            )),
            TargetVersion::Exact(version) => Ok(Cow::Borrowed(version)),
        }
    }
}

fn with_target_message<T, R>(
    message: &T,
    options: &GenerateOptions,
    format: OutputFormat,
    operation: impl FnOnce(&T) -> Result<R>,
) -> Result<R>
where
    T: VersionedNdm,
{
    let target_version = message.target_version(options)?;
    if target_version.as_ref() == message.version() {
        return operation(message).map_err(|error| {
            generation_error(
                error,
                T::KIND,
                format,
                message.version(),
                target_version.as_ref(),
            )
        });
    }

    let source_version = message.version();
    let mut selected = message.clone();
    selected.set_version(target_version.into_owned());
    operation(&selected).map_err(|error| {
        generation_error(error, T::KIND, format, source_version, selected.version())
    })
}

fn generate_kvn<T>(message: &T, options: &GenerateOptions) -> Result<String>
where
    T: VersionedNdm + ToKvn,
{
    with_target_message(message, options, OutputFormat::Kvn, |selected| {
        selected.to_kvn().and_then(|output| {
            enforce_output_limit(output.len(), options)?;
            Ok(output)
        })
    })
}

fn stream_kvn<T, W>(message: &T, output: &mut W, options: &GenerateOptions) -> Result<()>
where
    T: VersionedNdm + ToKvn,
    W: Write,
{
    with_target_message(message, options, OutputFormat::Kvn, |selected| {
        (|| {
            validate_output_version(T::KIND, selected.version(), OutputFormat::Kvn)?;
            selected.validate_kvn_output()?;
            preflight_kvn_limit(selected, options)?;
            let mut writer = crate::kvn::ser::KvnWriter::from_io(output);
            ToKvn::write_kvn(selected, &mut writer);
            writer.finish_io().map_err(CcsdsNdmError::from)
        })()
    })
}

macro_rules! impl_versioned_ndm {
    ($type:ty, $kind:ident) => {
        impl VersionedNdm for $type {
            const KIND: MessageKind = MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn set_version(&mut self, version: String) {
                self.version = version;
            }

            fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
                generate_kvn(self, options)
            }

            fn write_kvn_to<W: Write>(
                &self,
                output: &mut W,
                options: &GenerateOptions,
            ) -> Result<()> {
                stream_kvn(self, output, options)
            }
        }
    };
}

impl VersionedNdm for crate::messages::acm::Acm {
    const KIND: MessageKind = MessageKind::Acm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}
impl_versioned_ndm!(crate::messages::apm::Apm, Apm);
impl_versioned_ndm!(crate::messages::omm::Omm, Omm);

impl VersionedNdm for crate::messages::cdm::Cdm {
    const KIND: MessageKind = MessageKind::Cdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::aem::Aem {
    const KIND: MessageKind = MessageKind::Aem;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::ocm::Ocm {
    const KIND: MessageKind = MessageKind::Ocm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::tdm::Tdm {
    const KIND: MessageKind = MessageKind::Tdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::rdm::Rdm {
    const KIND: MessageKind = MessageKind::Rdm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        self.validate_kvn_representability()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::opm::Opm {
    const KIND: MessageKind = MessageKind::Opm;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_text()
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate()?;
        crate::traits::ToKvn::validate_kvn(self)
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

impl VersionedNdm for crate::messages::oem::Oem {
    const KIND: MessageKind = MessageKind::Oem;

    fn version(&self) -> &str {
        &self.version
    }

    fn set_version(&mut self, version: String) {
        self.version = version;
    }

    fn validate_xml_output(&self) -> Result<()> {
        self.validate_xml_text()
    }

    fn validate_kvn_output(&self) -> Result<()> {
        self.validate_kvn_generation()
    }

    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        generate_kvn(self, options)
    }

    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        stream_kvn(self, output, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kvn::ser::KvnWriter;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CLONES: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, serde::Serialize)]
    struct CloneTrackedMessage {
        version: String,
    }

    impl Clone for CloneTrackedMessage {
        fn clone(&self) -> Self {
            CLONES.fetch_add(1, Ordering::Relaxed);
            Self {
                version: self.version.clone(),
            }
        }
    }

    impl Validate for CloneTrackedMessage {
        fn validation_errors(&self) -> Result<Vec<crate::error::ValidationError>> {
            Ok(Vec::new())
        }
    }

    impl ToKvn for CloneTrackedMessage {
        fn write_kvn(&self, writer: &mut KvnWriter<'_>) {
            writer.write_pair("CCSDS_OMM_VERS", &self.version);
        }
    }

    impl Ndm for CloneTrackedMessage {
        fn to_kvn(&self) -> Result<String> {
            validate_for_generation(MessageKind::Omm, &self.version, OutputFormat::Kvn, self)?;
            let mut writer = KvnWriter::new();
            self.write_kvn(&mut writer);
            Ok(writer.finish())
        }

        fn from_kvn(_kvn: &str) -> Result<Self> {
            Err(CcsdsNdmError::UnsupportedMessage("test parser".into()))
        }

        fn to_xml(&self) -> Result<String> {
            validate_for_generation(MessageKind::Omm, &self.version, OutputFormat::Xml, self)?;
            crate::xml::to_string(self)
        }

        fn from_xml(_xml: &str) -> Result<Self> {
            Err(CcsdsNdmError::UnsupportedMessage("test parser".into()))
        }
    }

    impl VersionedNdm for CloneTrackedMessage {
        const KIND: MessageKind = MessageKind::Omm;

        fn version(&self) -> &str {
            &self.version
        }

        fn set_version(&mut self, version: String) {
            self.version = version;
        }

        fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
            generate_kvn(self, options)
        }

        fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
            stream_kvn(self, output, options)
        }
    }

    #[test]
    fn generation_clones_only_when_the_version_changes() {
        CLONES.store(0, Ordering::Relaxed);
        let message = CloneTrackedMessage {
            version: "2.0".into(),
        };

        message.to_kvn_with(&GenerateOptions::source()).unwrap();
        let mut output = Vec::new();
        message
            .write_kvn_to(&mut output, &GenerateOptions::source())
            .unwrap();

        assert_eq!(CLONES.load(Ordering::Relaxed), 0);

        let output = message.to_kvn_with(&GenerateOptions::latest()).unwrap();

        assert!(output.ends_with("3.0\n"));
        assert_eq!(message.version, "2.0");
        assert_eq!(CLONES.load(Ordering::Relaxed), 1);
    }
}
