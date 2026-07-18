// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing and generation options.

/// Selects the CCSDS edition used for generation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TargetVersion {
    /// Preserve the version stored on the message.
    #[default]
    Source,
    /// Use the latest edition implemented by the library.
    Latest,
    /// Generate a specific edition.
    Exact(String),
}

/// Configuration for generating an NDM message.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GenerateOptions {
    /// Target CCSDS edition.
    pub target_version: TargetVersion,
    /// Maximum complete generated document size in bytes.
    ///
    /// `None` keeps generation unlimited. This is a caller resource policy, not a CCSDS validity
    /// rule. Streaming writers preflight a configured limit before emitting any bytes.
    pub max_output_bytes: Option<usize>,
}

impl GenerateOptions {
    /// Preserve the version stored on the message.
    pub const fn source() -> Self {
        Self {
            target_version: TargetVersion::Source,
            max_output_bytes: None,
        }
    }

    /// Generate the latest implemented edition.
    pub const fn latest() -> Self {
        Self {
            target_version: TargetVersion::Latest,
            max_output_bytes: None,
        }
    }

    /// Generate a specific edition.
    pub fn version(version: impl Into<String>) -> Self {
        Self {
            target_version: TargetVersion::Exact(version.into()),
            max_output_bytes: None,
        }
    }

    /// Apply a caller-selected total generated-document limit.
    pub const fn with_max_output_bytes(mut self, max_output_bytes: usize) -> Self {
        self.max_output_bytes = Some(max_output_bytes);
        self
    }
}

/// Resource policy for parsing a complete NDM document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptions {
    /// Maximum input document size in bytes. `None` keeps the aggregate size unlimited.
    pub max_input_bytes: Option<usize>,
    /// Maximum XML element nesting depth.
    ///
    /// Current OPM/OEM schemas have small fixed depths; the default leaves generous headroom
    /// while bounding adversarial nesting.
    pub max_xml_depth: usize,
    /// Maximum number of materialized history records.
    ///
    /// `None` keeps record count unlimited. OEM counts state vectors and covariance matrices.
    /// The limit is checked during notation preflight, before the typed history is allocated.
    pub max_records: Option<usize>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            max_input_bytes: None,
            max_xml_depth: 16,
            max_records: None,
        }
    }
}

impl ParseOptions {
    pub fn with_max_input_bytes(mut self, max_input_bytes: usize) -> Self {
        self.max_input_bytes = Some(max_input_bytes);
        self
    }

    pub fn with_max_xml_depth(mut self, max_xml_depth: usize) -> Self {
        self.max_xml_depth = max_xml_depth;
        self
    }

    pub fn with_max_records(mut self, max_records: usize) -> Self {
        self.max_records = Some(max_records);
        self
    }
}
