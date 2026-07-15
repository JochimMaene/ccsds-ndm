// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Validate;
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValidationMode {
    Strict,
    Permissive,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationIssue {
    pub message_kind: MessageKind,
    pub error: ValidationError,
}

thread_local! {
    static VALIDATION_MODE: Cell<ValidationMode> = const { Cell::new(ValidationMode::Strict) };
    static VALIDATION_WARNINGS: RefCell<Vec<ValidationIssue>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn current_mode() -> ValidationMode {
    VALIDATION_MODE.with(|mode| mode.get())
}

pub(crate) fn with_validation_mode<T>(
    mode: ValidationMode,
    f: impl FnOnce() -> Result<T>,
) -> Result<T> {
    struct Guard {
        prev: ValidationMode,
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            VALIDATION_MODE.with(|mode| mode.set(self.prev));
        }
    }

    let prev = VALIDATION_MODE.with(|m| {
        let prev = m.get();
        m.set(mode);
        prev
    });

    let _guard = Guard { prev };
    f()
}

pub(crate) fn take_warnings() -> Vec<ValidationIssue> {
    VALIDATION_WARNINGS.with(|warnings| warnings.borrow_mut().drain(..).collect())
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

pub(crate) fn validation_errors_from(result: Result<()>) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();
    collect_validation_result(&mut errors, result)?;
    Ok(errors)
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

pub fn validate_with_mode(kind: MessageKind, value: &impl Validate) -> Result<()> {
    if current_mode() == ValidationMode::Strict {
        return value.validate();
    }

    for error in value.validation_errors()? {
        handle_validation_error_inner(kind, error)?;
    }
    Ok(())
}

pub fn handle_validation_error(kind: MessageKind, err: CcsdsNdmError) -> Result<()> {
    match err {
        CcsdsNdmError::Validation(val) => handle_validation_error_inner(kind, *val),
        other => Err(other),
    }
}

fn handle_validation_error_inner(kind: MessageKind, err: ValidationError) -> Result<()> {
    match current_mode() {
        ValidationMode::Strict => Err(err.into()),
        ValidationMode::Permissive => {
            VALIDATION_WARNINGS.with(|warnings| {
                warnings.borrow_mut().push(ValidationIssue {
                    message_kind: kind,
                    error: err,
                });
            });
            Ok(())
        }
    }
}
