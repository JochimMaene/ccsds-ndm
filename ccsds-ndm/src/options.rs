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
}

impl GenerateOptions {
    /// Preserve the version stored on the message.
    pub const fn source() -> Self {
        Self {
            target_version: TargetVersion::Source,
        }
    }

    /// Generate the latest implemented edition.
    pub const fn latest() -> Self {
        Self {
            target_version: TargetVersion::Latest,
        }
    }

    /// Generate a specific edition.
    pub fn version(version: impl Into<String>) -> Self {
        Self {
            target_version: TargetVersion::Exact(version.into()),
        }
    }
}
