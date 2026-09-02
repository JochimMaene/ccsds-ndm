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
pub(crate) struct VersionSpec {
    pub id_keyword: &'static str,
    pub supported_versions: &'static [&'static str],
    pub kvn_output_versions: &'static [&'static str],
    pub xml_output_versions: &'static [&'static str],
}

pub(crate) fn spec(kind: MessageKind) -> Option<VersionSpec> {
    match kind {
        MessageKind::Opm => Some(VersionSpec {
            id_keyword: "CCSDS_OPM_VERS",
            supported_versions: &["1.0", "2.0", "3.0"],
            kvn_output_versions: &["2.0", "3.0"],
            xml_output_versions: &["2.0", "3.0"],
        }),
        MessageKind::Omm => Some(VersionSpec {
            id_keyword: "CCSDS_OMM_VERS",
            supported_versions: &["2.0", "3.0"],
            kvn_output_versions: &["2.0", "3.0"],
            xml_output_versions: &["2.0", "3.0"],
        }),
        MessageKind::Oem => Some(VersionSpec {
            id_keyword: "CCSDS_OEM_VERS",
            supported_versions: &["1.0", "2.0", "3.0"],
            kvn_output_versions: &["2.0", "3.0"],
            xml_output_versions: &["2.0", "3.0"],
        }),
        MessageKind::Ocm => Some(VersionSpec {
            id_keyword: "CCSDS_OCM_VERS",
            supported_versions: &["3.0"],
            kvn_output_versions: &["3.0"],
            xml_output_versions: &["3.0"],
        }),
        MessageKind::Aem => Some(VersionSpec {
            id_keyword: "CCSDS_AEM_VERS",
            supported_versions: &["1.0", "2.0"],
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Apm => Some(VersionSpec {
            id_keyword: "CCSDS_APM_VERS",
            supported_versions: &["1.0", "2.0"],
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Acm => Some(VersionSpec {
            id_keyword: "CCSDS_ACM_VERS",
            supported_versions: &["1.0", "2.0"],
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Cdm => Some(VersionSpec {
            id_keyword: "CCSDS_CDM_VERS",
            supported_versions: &["1.0"],
            kvn_output_versions: &["1.0"],
            xml_output_versions: &["1.0"],
        }),
        MessageKind::Tdm => Some(VersionSpec {
            id_keyword: "CCSDS_TDM_VERS",
            supported_versions: &["1.0", "2.0"],
            kvn_output_versions: &["2.0"],
            xml_output_versions: &["2.0"],
        }),
        MessageKind::Rdm => Some(VersionSpec {
            id_keyword: "CCSDS_RDM_VERS",
            supported_versions: &["1.0"],
            kvn_output_versions: &["1.0"],
            xml_output_versions: &["1.0"],
        }),
        MessageKind::Ndm => None,
    }
}

/// Return the input editions accepted for a message family.
pub fn supported_versions(kind: MessageKind) -> Option<&'static [&'static str]> {
    spec(kind).map(|spec| spec.supported_versions)
}

pub(crate) fn validate_root(kind: MessageKind, id: &Option<String>, version: &str) -> Result<()> {
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

fn invalid_edition_value(
    field: &'static str,
    value: impl Into<String>,
    expected: &'static str,
    path: &'static str,
) -> CcsdsNdmError {
    ValidationError::InvalidValue {
        field: field.into(),
        value: value.into(),
        expected: expected.into(),
        line: None,
    }
    .at_path(path)
    .into()
}

fn validate_odm_2_header(header: &crate::common::OdmHeader) -> Result<()> {
    if header.classification.is_some() {
        return Err(invalid_edition_value(
            "CLASSIFICATION",
            "present",
            "absent in CCSDS 502.0-B-2",
            "header.classification",
        ));
    }
    if header.message_id.is_some() {
        return Err(invalid_edition_value(
            "MESSAGE_ID",
            "present",
            "absent in CCSDS 502.0-B-2",
            "header.message_id",
        ));
    }
    Ok(())
}

pub(crate) fn validate_opm_edition(message: &crate::messages::opm::Opm) -> Result<()> {
    if message.version != "2.0" {
        return Ok(());
    }
    validate_odm_2_header(&message.header)?;
    for (index, maneuver) in message
        .body
        .segment
        .data
        .maneuver_parameters
        .iter()
        .enumerate()
    {
        if maneuver.man_delta_mass.value == 0.0 {
            return Err(ValidationError::OutOfRange {
                name: "MAN_DELTA_MASS".into(),
                value: "0".into(),
                expected: "< 0 for CCSDS 502.0-B-2".into(),
                line: None,
            }
            .at_path(format!(
                "body.segment.data.maneuver_parameters[{index}].man_delta_mass"
            ))
            .into());
        }
    }
    Ok(())
}

pub(crate) fn validate_oem_edition(message: &crate::messages::oem::Oem) -> Result<()> {
    if message.version == "2.0" {
        validate_odm_2_header(&message.header)?;
    }
    Ok(())
}

pub(crate) fn validate_omm_edition(message: &crate::messages::omm::Omm) -> Result<()> {
    if message.version != "2.0" {
        return Ok(());
    }
    validate_odm_2_header(&message.header)?;
    let Some(tle) = &message.body.segment.data.tle_parameters else {
        return Ok(());
    };
    for (field, missing, path) in [
        (
            "NORAD_CAT_ID",
            tle.norad_cat_id.is_none(),
            "body.segment.data.tle_parameters.norad_cat_id",
        ),
        (
            "ELEMENT_SET_NO",
            tle.element_set_no.is_none(),
            "body.segment.data.tle_parameters.element_set_no",
        ),
        (
            "REV_AT_EPOCH",
            tle.rev_at_epoch.is_none(),
            "body.segment.data.tle_parameters.rev_at_epoch",
        ),
        (
            "BSTAR",
            tle.bstar.is_none(),
            "body.segment.data.tle_parameters.bstar",
        ),
        (
            "MEAN_MOTION_DDOT",
            tle.mean_motion_ddot.is_none(),
            "body.segment.data.tle_parameters.mean_motion_ddot",
        ),
    ] {
        if missing {
            return Err(ValidationError::MissingRequiredField {
                block: "OMM 2.0 TLE Parameters".into(),
                field: field.into(),
                line: None,
            }
            .at_path(path)
            .into());
        }
    }
    if tle.bterm.is_some() {
        return Err(invalid_edition_value(
            "BTERM",
            "present",
            "absent in CCSDS 502.0-B-2",
            "body.segment.data.tle_parameters.bterm",
        ));
    }
    if tle.agom.is_some() {
        return Err(invalid_edition_value(
            "AGOM",
            "present",
            "absent in CCSDS 502.0-B-2",
            "body.segment.data.tle_parameters.agom",
        ));
    }
    Ok(())
}
