// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Validate;
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    Strict,
    Lenient,
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

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub message_kind: MessageKind,
    pub error: ValidationError,
}

thread_local! {
    static VALIDATION_MODE: Cell<ValidationMode> = const { Cell::new(ValidationMode::Strict) };
    static VALIDATION_WARNINGS: RefCell<Vec<ValidationIssue>> = const { RefCell::new(Vec::new()) };
}

pub fn current_mode() -> ValidationMode {
    VALIDATION_MODE.with(|mode| mode.get())
}

pub fn with_validation_mode<T>(mode: ValidationMode, f: impl FnOnce() -> Result<T>) -> Result<T> {
    struct Guard {
        prev: ValidationMode,
    }

    let prev = VALIDATION_MODE.with(|m| {
        let prev = m.get();
        m.set(mode);
        prev
    });

    let _guard = Guard { prev };
    let res = f();

    VALIDATION_MODE.with(|m| m.set(_guard.prev));
    res
}

pub fn take_warnings() -> Vec<ValidationIssue> {
    VALIDATION_WARNINGS.with(|warnings| warnings.borrow_mut().drain(..).collect())
}

pub fn validate_with_mode(kind: MessageKind, value: &impl Validate) -> Result<()> {
    match value.validate() {
        Ok(()) => Ok(()),
        Err(err) => handle_validation_error(kind, err),
    }
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
        ValidationMode::Lenient => {
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
