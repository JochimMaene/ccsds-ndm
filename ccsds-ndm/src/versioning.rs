// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Centralized version policy for CCSDS message roots.
//!
//! This module defines supported version lists per message type and
//! provides helpers for validation and parsing.

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::validation::MessageKind;

#[derive(Debug, Clone, Copy)]
pub struct VersionSpec {
    pub id_keyword: &'static str,
    pub supported_versions: &'static [&'static str],
    pub default_version: &'static str,
    pub kvn_output_versions: &'static [&'static str],
    pub xml_output_versions: &'static [&'static str],
}

pub fn spec(kind: MessageKind) -> Option<VersionSpec> {
    match kind {
        MessageKind::Opm => Some(VersionSpec {
            id_keyword: "CCSDS_OPM_VERS",
            supported_versions: &["1.0", "2.0", "3.0"],
            default_version: "3.0",
            kvn_output_versions: &["3.0"],
            xml_output_versions: &["3.0"],
        }),
        MessageKind::Omm => Some(VersionSpec {
            id_keyword: "CCSDS_OMM_VERS",
            supported_versions: &["2.0", "3.0"],
            default_version: "3.0",
            kvn_output_versions: &["2.0", "3.0"],
            xml_output_versions: &["3.0"],
        }),
        MessageKind::Oem => Some(VersionSpec {
            id_keyword: "CCSDS_OEM_VERS",
            supported_versions: &["1.0", "2.0", "3.0"],
            default_version: "3.0",
            kvn_output_versions: &["3.0"],
            xml_output_versions: &["3.0"],
        }),
        MessageKind::Ocm => Some(VersionSpec {
            id_keyword: "CCSDS_OCM_VERS",
            supported_versions: &["3.0"],
            default_version: "3.0",
            kvn_output_versions: &["3.0"],
            xml_output_versions: &["3.0"],
        }),
        MessageKind::Aem => Some(VersionSpec {
            id_keyword: "CCSDS_AEM_VERS",
            supported_versions: &["1.0", "2.0"],
            default_version: "2.0",
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Apm => Some(VersionSpec {
            id_keyword: "CCSDS_APM_VERS",
            supported_versions: &["1.0", "2.0"],
            default_version: "2.0",
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Acm => Some(VersionSpec {
            id_keyword: "CCSDS_ACM_VERS",
            supported_versions: &["1.0", "2.0"],
            default_version: "2.0",
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Cdm => Some(VersionSpec {
            id_keyword: "CCSDS_CDM_VERS",
            supported_versions: &["1.0"],
            default_version: "1.0",
            kvn_output_versions: &["1.0"],
            xml_output_versions: &["1.0"],
        }),
        MessageKind::Tdm => Some(VersionSpec {
            id_keyword: "CCSDS_TDM_VERS",
            supported_versions: &["1.0", "2.0"],
            default_version: "2.0",
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Rdm => Some(VersionSpec {
            id_keyword: "CCSDS_RDM_VERS",
            supported_versions: &["1.0"],
            default_version: "1.0",
            kvn_output_versions: &["1.0"],
            xml_output_versions: &["1.0"],
        }),
        MessageKind::Ndm => None,
    }
}

pub fn is_supported_version(kind: MessageKind, version: &str) -> bool {
    spec(kind)
        .map(|spec| spec.supported_versions.contains(&version))
        .unwrap_or(false)
}

pub fn validate_root(kind: MessageKind, id: &Option<String>, version: &str) -> Result<()> {
    let spec = match spec(kind) {
        Some(spec) => spec,
        None => return Ok(()),
    };

    match id.as_deref() {
        Some(value) if value == spec.id_keyword => (),
        Some(value) => {
            return Err(ValidationError::InvalidValue {
                field: "id".into(),
                value: value.to_string(),
                expected: spec.id_keyword.into(),
                line: None,
            }
            .into());
        }
        None => {
            return Err(ValidationError::MissingRequiredField {
                block: "Root".into(),
                field: "id".into(),
                line: None,
            }
            .into());
        }
    }

    if !spec.supported_versions.contains(&version) {
        return Err(CcsdsNdmError::UnsupportedInputVersion {
            message_type: kind.as_str(),
            version: version.to_string(),
            supported: spec.supported_versions.join(", "),
        });
    }

    Ok(())
}
