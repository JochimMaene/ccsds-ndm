// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Parsing and generation options.

use crate::error::Result;
use crate::validation::{ValidationIssue, ValidationMode};

/// Controls whether parsing rejects or reports recoverable semantic violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseMode {
    /// Reject every syntax or semantic violation.
    #[default]
    Strict,
    /// Accept supported deviations and return diagnostics for each recovery.
    Permissive,
}

impl From<ParseMode> for ValidationMode {
    fn from(value: ParseMode) -> Self {
        match value {
            ParseMode::Strict => ValidationMode::Strict,
            ParseMode::Permissive => ValidationMode::Permissive,
        }
    }
}

/// A parsed message together with diagnostics produced by permissive recovery.
#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub struct ParseReport<T> {
    /// Parsed message.
    pub message: T,
    /// Diagnostics in source order.
    pub diagnostics: Vec<ValidationIssue>,
}

pub(crate) fn parse_with_mode<T>(
    mode: ParseMode,
    parser: impl FnOnce() -> Result<T>,
) -> Result<ParseReport<T>> {
    let _ = crate::validation::take_warnings();
    let result = crate::validation::with_validation_mode(mode.into(), parser);
    let diagnostics = crate::validation::take_warnings();
    result.map(|message| ParseReport {
        message,
        diagnostics,
    })
}

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
