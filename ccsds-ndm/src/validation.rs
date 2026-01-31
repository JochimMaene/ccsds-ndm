// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use crate::traits::Validate;
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::sync::OnceLock;

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

impl MessageKind {
    fn as_str(self) -> &'static str {
        match self {
            MessageKind::Opm => "OPM",
            MessageKind::Omm => "OMM",
            MessageKind::Oem => "OEM",
            MessageKind::Ocm => "OCM",
            MessageKind::Acm => "ACM",
            MessageKind::Aem => "AEM",
            MessageKind::Apm => "APM",
            MessageKind::Cdm => "CDM",
            MessageKind::Tdm => "TDM",
            MessageKind::Rdm => "RDM",
            MessageKind::Ndm => "NDM",
        }
    }
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
        ValidationMode::Strict => {
            if policy_allows_warning(kind, &err) {
                VALIDATION_WARNINGS.with(|warnings| {
                    warnings.borrow_mut().push(ValidationIssue {
                        message_kind: kind,
                        error: err,
                    });
                });
                Ok(())
            } else {
                Err(err.into())
            }
        }
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

#[derive(Debug, Deserialize)]
struct ValidationPolicy {
    rules: Vec<PolicyRule>,
}

#[derive(Debug, Deserialize)]
struct PolicyRule {
    message_kind: Option<String>,
    error_kind: String,
    block_contains: Option<String>,
    field_contains: Option<String>,
    value_contains: Option<String>,
    expected_contains: Option<String>,
    message_contains: Option<String>,
    conflict_fields: Option<Vec<String>>,
    action: PolicyAction,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PolicyAction {
    Warn,
    Error,
}

fn policy() -> &'static ValidationPolicy {
    static POLICY: OnceLock<ValidationPolicy> = OnceLock::new();
    POLICY.get_or_init(|| {
        let raw = include_str!("../validation_policy.json");
        serde_json::from_str(raw).expect("validation_policy.json must be valid JSON")
    })
}

fn policy_allows_warning(kind: MessageKind, err: &ValidationError) -> bool {
    for rule in &policy().rules {
        if let Some(ref mk) = rule.message_kind {
            if mk != kind.as_str() {
                continue;
            }
        }

        if !matches_error_kind(&rule.error_kind, err) {
            continue;
        }

        if let Some(ref block_contains) = rule.block_contains {
            let block = match err {
                ValidationError::MissingRequiredField { block, .. } => block.as_ref(),
                _ => "",
            };
            if !block.contains(block_contains) {
                continue;
            }
        }

        if let Some(ref field_contains) = rule.field_contains {
            let field = match err {
                ValidationError::MissingRequiredField { field, .. } => field.as_ref(),
                ValidationError::InvalidValue { field, .. } => field.as_ref(),
                ValidationError::OutOfRange { name, .. } => name.as_ref(),
                _ => "",
            };
            if !field.contains(field_contains) {
                continue;
            }
        }

        if let Some(ref value_contains) = rule.value_contains {
            let value = match err {
                ValidationError::InvalidValue { value, .. } => value.as_str(),
                ValidationError::OutOfRange { value, .. } => value.as_str(),
                _ => "",
            };
            if !value.contains(value_contains) {
                continue;
            }
        }

        if let Some(ref expected_contains) = rule.expected_contains {
            let expected = match err {
                ValidationError::InvalidValue { expected, .. } => expected.as_ref(),
                ValidationError::OutOfRange { expected, .. } => expected.as_ref(),
                _ => "",
            };
            if !expected.contains(expected_contains) {
                continue;
            }
        }

        if let Some(ref message_contains) = rule.message_contains {
            let msg = match err {
                ValidationError::Generic { message, .. } => message.as_ref(),
                _ => "",
            };
            if !msg.contains(message_contains) {
                continue;
            }
        }

        if let Some(ref conflict_fields) = rule.conflict_fields {
            let fields = match err {
                ValidationError::Conflict { fields, .. } => fields,
                _ => continue,
            };
            let set: HashSet<_> = fields.iter().map(|f| f.as_ref()).collect();
            if !conflict_fields.iter().all(|f| set.contains(f.as_str())) {
                continue;
            }
        }

        return rule.action == PolicyAction::Warn;
    }

    false
}

fn matches_error_kind(kind: &str, err: &ValidationError) -> bool {
    matches!(
        (kind, err),
        (
            "missing_required_field",
            ValidationError::MissingRequiredField { .. }
        ) | ("conflict", ValidationError::Conflict { .. })
            | ("invalid_value", ValidationError::InvalidValue { .. })
            | ("out_of_range", ValidationError::OutOfRange { .. })
            | ("generic", ValidationError::Generic { .. })
    )
}
