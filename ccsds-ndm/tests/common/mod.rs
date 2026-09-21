// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

/// Changes a fixture only if the intended source text is present.
#[allow(dead_code)]
#[track_caller]
pub fn mutated(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture is missing mutation target {from:?}"
    );
    source.replace(from, to)
}

#[allow(dead_code)]
#[track_caller]
pub fn mutated_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture is missing mutation target {from:?}"
    );
    source.replacen(from, to, 1)
}

pub fn data_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("data")
}

/// Loads matching fixtures in filename order; an empty corpus is a test failure.
#[allow(dead_code)]
pub fn fixtures(prefix: &str, extension: &str) -> Vec<(String, String)> {
    let directory = data_dir().join(extension);
    let mut found: Vec<_> = std::fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("{} is not readable: {error}", directory.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|value| value == extension)
                && path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(prefix)
        })
        .map(|path| {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} is not readable: {error}", path.display()));
            (name, source)
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!found.is_empty(), "no {prefix}* {extension} fixtures found");
    found
}

/// Returns the validation cause without its field-path wrappers.
#[allow(dead_code)]
pub fn validation_error_source(
    mut error: &ccsds_ndm::error::ValidationError,
) -> &ccsds_ndm::error::ValidationError {
    while let ccsds_ndm::error::ValidationError::AtPath { source, .. } = error {
        error = source;
    }
    error
}

/// Checks the structured offending field, falling back to text for uncoded errors.
#[allow(dead_code)]
#[track_caller]
pub fn assert_validation_field(error: &ccsds_ndm::error::CcsdsNdmError, field: &str) {
    use ccsds_ndm::error::ValidationError;
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("expected validation error, got {error}"));
    match validation_error_source(validation) {
        ValidationError::MissingRequiredField { field: actual, .. }
        | ValidationError::InvalidValue { field: actual, .. }
        | ValidationError::OutOfRange { name: actual, .. } => {
            assert_eq!(actual.as_ref(), field, "wrong offending field: {error}");
        }
        ValidationError::Conflict { fields, .. }
        | ValidationError::InvalidChoice { fields, .. } => {
            assert!(
                fields.iter().any(|actual| actual == field),
                "wrong fields for {field}: {error}"
            );
        }
        _ => assert!(
            error.to_string().contains(field),
            "wrong diagnostic for {field}: {error}"
        ),
    }
}

/// Checks that `value` was refused as an epoch token, not for something else the mutation disturbed.
#[allow(dead_code)]
#[track_caller]
pub fn assert_invalid_epoch(error: &ccsds_ndm::error::CcsdsNdmError, value: &str) {
    let expected = format!("invalid epoch format: '{value}'");
    assert!(
        error.to_string().contains(&expected),
        "expected {expected:?}, got: {error}"
    );
}

/// Checks the exact lexical or structural diagnostic behind a strict-parsing refusal.
#[allow(dead_code)]
#[track_caller]
pub fn assert_invalid_format(error: &ccsds_ndm::error::CcsdsNdmError, expected: &str) {
    match error.as_format_error() {
        Some(ccsds_ndm::error::FormatError::InvalidFormat(message)) => {
            assert_eq!(message, expected, "wrong diagnostic: {error}");
        }
        _ => panic!("expected an invalid-format diagnostic, got: {error}"),
    }
}

/// Checks the offending field and the same validation failure across output paths.
#[allow(dead_code)]
#[track_caller]
pub fn assert_rejects<M: ccsds_ndm::Ndm>(message: &M, field: &str) {
    let expected = message
        .validate()
        .expect_err("invalid model passed validation");
    assert_validation_field(&expected, field);
    let validation = expected.as_validation_error().unwrap();
    for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
        assert_eq!(
            error.as_validation_error(),
            Some(validation),
            "{field}: {error}"
        );
    }
    let mut output = Vec::new();
    let error = message.write_kvn_to(&mut output).unwrap_err();
    assert_eq!(
        error.as_validation_error(),
        Some(validation),
        "{field}: {error}"
    );
    assert!(output.is_empty(), "streaming KVN wrote bytes for {field}");
    let error = message.write_xml_to(&mut output).unwrap_err();
    assert_eq!(
        error.as_validation_error(),
        Some(validation),
        "{field}: {error}"
    );
    assert!(output.is_empty(), "streaming XML wrote bytes for {field}");
}

/// Validates generated XML against the bundled reference schema.
///
/// libxml2 establishes structure, ordering and lexical form. It accepts NaN against bounding
/// facets, so this is not evidence of numeric validity; see the XSD oracle policy in the
/// validation contract.
#[allow(dead_code)]
pub fn validate_xml(label: &str, xml: &str) {
    let document = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(document.path(), xml).unwrap();
    let output = std::process::Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(data_dir().join("xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap_or_else(|error| panic!("xmllint is required for conformance evidence: {error}"));
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FailAfter {
    pub accepted: Vec<u8>,
    limit: usize,
}

#[allow(dead_code)]
impl FailAfter {
    pub fn new(limit: usize) -> Self {
        Self {
            accepted: Vec::new(),
            limit,
        }
    }
}

impl std::io::Write for FailAfter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.accepted.len());
        if remaining == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "deliberate test sink failure",
            ));
        }

        let accepted = remaining.min(buffer.len());
        self.accepted.extend_from_slice(&buffer[..accepted]);
        Ok(accepted)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
