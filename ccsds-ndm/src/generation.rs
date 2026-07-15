// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Version-aware, validated generation helpers.

use crate::error::{CcsdsNdmError, Result};
use crate::options::{GenerateOptions, TargetVersion};
use crate::traits::{Ndm, ToKvn, Validate};
use crate::validation::MessageKind;
use std::borrow::Cow;
use std::io::Write;

/// Output notation used when checking edition support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
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
}

pub(crate) fn validate_for_generation(
    kind: MessageKind,
    version: &str,
    format: OutputFormat,
    value: &impl Validate,
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
    value.validate()
}

/// NDM messages that carry a CCSDS edition in their root element/header.
pub trait VersionedNdm: Ndm + Clone + ToKvn {
    /// Message family used to resolve supported editions.
    const KIND: MessageKind;

    /// Return the edition stored on the message.
    fn version(&self) -> &str;

    /// Update the edition stored on a cloned message before generation.
    fn set_version(&mut self, version: String);

    /// Generate KVN using explicit options.
    fn to_kvn_with(&self, options: &GenerateOptions) -> Result<String> {
        let version = self.target_version(options)?;
        if version.as_ref() == self.version() {
            return self.to_kvn();
        }

        let mut message = self.clone();
        message.set_version(version.into_owned());
        message.to_kvn()
    }

    /// Generate XML using explicit options.
    fn to_xml_with(&self, options: &GenerateOptions) -> Result<String> {
        let version = self.target_version(options)?;
        if version.as_ref() == self.version() {
            return self.to_xml();
        }

        let mut message = self.clone();
        message.set_version(version.into_owned());
        message.to_xml()
    }

    /// Write KVN directly to an I/O sink.
    fn write_kvn_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        let version = self.target_version(options)?;
        if version.as_ref() == self.version() {
            validate_for_generation(Self::KIND, self.version(), OutputFormat::Kvn, self)?;
            let mut writer = crate::kvn::ser::KvnWriter::from_io(output);
            ToKvn::write_kvn(self, &mut writer);
            return writer.finish_io().map_err(CcsdsNdmError::from);
        }

        let mut message = self.clone();
        message.set_version(version.into_owned());
        validate_for_generation(Self::KIND, message.version(), OutputFormat::Kvn, &message)?;
        let mut writer = crate::kvn::ser::KvnWriter::from_io(output);
        ToKvn::write_kvn(&message, &mut writer);
        writer.finish_io().map_err(CcsdsNdmError::from)
    }

    /// Write XML directly to an I/O sink.
    fn write_xml_to<W: Write>(&self, output: &mut W, options: &GenerateOptions) -> Result<()> {
        let version = self.target_version(options)?;
        if version.as_ref() == self.version() {
            validate_for_generation(Self::KIND, self.version(), OutputFormat::Xml, self)?;
            return crate::xml::to_writer(output, self);
        }

        let mut message = self.clone();
        message.set_version(version.into_owned());
        validate_for_generation(Self::KIND, message.version(), OutputFormat::Xml, &message)?;
        crate::xml::to_writer(output, &message)
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

#[macro_export]
macro_rules! impl_versioned_ndm {
    ($type:ty, $kind:ident) => {
        impl $crate::generation::VersionedNdm for $type {
            const KIND: $crate::validation::MessageKind = $crate::validation::MessageKind::$kind;

            fn version(&self) -> &str {
                &self.version
            }

            fn set_version(&mut self, version: String) {
                self.version = version;
            }
        }
    };
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
