// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result, ValidationError};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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

pub(crate) fn is_xml_1_character(character: char) -> bool {
    matches!(character, '\u{9}' | '\u{A}' | '\u{D}')
        || ('\u{20}'..='\u{D7FF}').contains(&character)
        || ('\u{E000}'..='\u{FFFD}').contains(&character)
        || ('\u{10000}'..='\u{10FFFF}').contains(&character)
}

pub(crate) fn xml_text_error(field: &'static str, value: &str) -> Option<ValidationError> {
    value
        .chars()
        .find(|character| !is_xml_1_character(*character))
        .map(|character| ValidationError::InvalidValue {
            field: Cow::Borrowed(field),
            value: format!("contains U+{:04X}", u32::from(character)),
            expected: Cow::Borrowed("text containing only XML 1.0 characters"),
            line: None,
        })
}

pub(crate) fn kvn_comment_error(value: &str) -> Option<ValidationError> {
    for line in value.lines() {
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return Some(ValidationError::InvalidValue {
                field: Cow::Borrowed("COMMENT"),
                value: line.to_owned(),
                expected: Cow::Borrowed("printable ASCII characters and blanks"),
                line: None,
            });
        }
        let line_len = "COMMENT ".len() + line.len();
        if line_len > 254 {
            return Some(ValidationError::OutOfRange {
                name: Cow::Borrowed("COMMENT"),
                value: line_len.to_string(),
                expected: Cow::Borrowed("a KVN line no longer than 254 characters"),
                line: None,
            });
        }
    }
    None
}

pub(crate) fn validate_at_field_path(
    result: Result<()>,
    parent_path: impl Into<std::borrow::Cow<'static, str>>,
) -> Result<()> {
    match result {
        Err(CcsdsNdmError::Validation(error)) => Err((*error).at_field_in(parent_path).into()),
        result => result,
    }
}
