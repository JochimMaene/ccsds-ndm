//! OPM notation conversion composed from the strict parser and validated generators.

use crate::error::Result;
use crate::messages::opm::Opm;
use crate::options::{GenerateOptions, ParseOptions};
use crate::VersionedNdm;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notation {
    Kvn,
    Xml,
}

/// Strictly parse an OPM file with bounded input reading and optional notation detection.
pub fn parse_opm_file(
    source_path: impl AsRef<Path>,
    source: Option<Notation>,
    options: &ParseOptions,
) -> Result<Opm> {
    let input = read_input(source_path.as_ref(), options.max_input_bytes, source)?;
    match source.unwrap_or_else(|| {
        if input.trim_start().starts_with('<') {
            Notation::Xml
        } else {
            Notation::Kvn
        }
    }) {
        Notation::Kvn => Opm::from_kvn_with_options(&input, options),
        Notation::Xml => Opm::from_xml_with_options(&input, options),
    }
}

/// Convert a standalone OPM between KVN and XML without changing its edition by default.
pub fn convert_opm(
    input: &str,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<String> {
    let message = match source {
        Notation::Kvn => Opm::from_kvn_with_options(input, parse_options)?,
        Notation::Xml => Opm::from_xml_with_options(input, parse_options)?,
    };
    match target {
        Notation::Kvn => message.to_kvn_with(generate_options),
        Notation::Xml => message.to_xml_with(generate_options),
    }
}

/// Convert an OPM file and atomically replace the destination only after conversion succeeds.
pub fn convert_opm_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let message = parse_opm_file(source_path, Some(source), parse_options)?;
    let output = match target {
        Notation::Kvn => message.to_kvn_with(generate_options),
        Notation::Xml => message.to_xml_with(generate_options),
    }?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}

fn read_input(
    path: &Path,
    max_bytes: Option<usize>,
    source_hint: Option<Notation>,
) -> Result<String> {
    let Some(limit) = max_bytes else {
        return Ok(std::fs::read_to_string(path)?);
    };

    let mut bytes = Vec::with_capacity(limit.min(64 * 1024).saturating_add(1));
    std::fs::File::open(path)?
        .take(limit.saturating_add(1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        let notation = source_hint.unwrap_or_else(|| {
            if bytes.iter().copied().find(|byte| !byte.is_ascii_whitespace()) == Some(b'<')
            {
                Notation::Xml
            } else {
                Notation::Kvn
            }
        });
        let error = crate::error::CcsdsNdmError::ResourceLimitExceeded {
            resource: "input_document",
            limit,
            actual: bytes.len(),
        };
        return Err(error.with_parse_context(
            crate::validation::MessageKind::Opm,
            match notation {
                Notation::Kvn => crate::error::DiagnosticNotation::Kvn,
                Notation::Xml => crate::error::DiagnosticNotation::Xml,
            },
            "",
            None,
        ));
    }
    Ok(String::from_utf8(bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?)
}

/// Convert in-memory OPM input and atomically write the complete destination.
pub fn convert_opm_to_file(
    input: &str,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let output = convert_opm(input, source, target, parse_options, generate_options)?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}
