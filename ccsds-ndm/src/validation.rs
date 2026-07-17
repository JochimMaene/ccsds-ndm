// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Validate;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKind {
    Opm,
    Omm,
    Oem,
    Ocm,
    Acm,
    Aem,
    Apm,
    Cdm,
    Tdm,
    Rdm,
    Ndm,
}

impl MessageKind {
    /// Standard abbreviation for the message family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Opm => "OPM",
            Self::Omm => "OMM",
            Self::Oem => "OEM",
            Self::Ocm => "OCM",
            Self::Acm => "ACM",
            Self::Aem => "AEM",
            Self::Apm => "APM",
            Self::Cdm => "CDM",
            Self::Tdm => "TDM",
            Self::Rdm => "RDM",
            Self::Ndm => "NDM",
        }
    }
}

pub(crate) fn collect_validation_result(
    errors: &mut Vec<ValidationError>,
    result: Result<()>,
) -> Result<()> {
    match result {
        Ok(()) => Ok(()),
        Err(CcsdsNdmError::Validation(error)) => {
            errors.push(*error);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn xml_text_error(field: &'static str, value: &str) -> Option<ValidationError> {
    value
        .chars()
        .find(|character| {
            !matches!(*character, '\u{9}' | '\u{A}' | '\u{D}')
                && !('\u{20}'..='\u{D7FF}').contains(character)
                && !('\u{E000}'..='\u{FFFD}').contains(character)
                && !('\u{10000}'..='\u{10FFFF}').contains(character)
        })
        .map(|character| ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: format!("contains U+{:04X}", u32::from(character)),
            expected: Cow::Borrowed("text containing only XML 1.0 characters"),
            line: None,
        })
}

pub(crate) fn validation_errors_from(result: Result<()>) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();
    collect_validation_result(&mut errors, result)?;
    Ok(errors)
}

pub(crate) fn validate_at_field_path(result: Result<()>, parent_path: &'static str) -> Result<()> {
    match result {
        Err(CcsdsNdmError::Validation(error)) => Err((*error).at_field_in(parent_path).into()),
        result => result,
    }
}

pub(crate) fn at_field_paths(
    errors: Vec<ValidationError>,
    parent_path: &'static str,
) -> Vec<ValidationError> {
    errors
        .into_iter()
        .map(|error| error.at_field_in(parent_path))
        .collect()
}

pub(crate) fn collect_message_validation_errors(
    kind: MessageKind,
    id: &Option<String>,
    version: &str,
    header: &impl Validate,
    body: &impl Validate,
) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();
    collect_validation_result(
        &mut errors,
        crate::versioning::validate_root(kind, id, version),
    )?;
    errors.extend(header.validation_errors()?);
    errors.extend(body.validation_errors()?);
    Ok(errors)
}

pub(crate) fn missing_required_fields<const N: usize>(
    block: &'static str,
    fields: [(&'static str, bool); N],
) -> Vec<ValidationError> {
    fields
        .into_iter()
        .filter(|(_, missing)| *missing)
        .map(|(field, _)| ValidationError::MissingRequiredField {
            block: block.into(),
            field: field.into(),
            line: None,
        })
        .collect()
}
