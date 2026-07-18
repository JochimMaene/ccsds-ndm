//! Notation conversion composed from strict typed parsers and validated generators.

use crate::detect::{detect_notation, without_utf8_bom};
use crate::error::Result;
use crate::messages::oem::Oem;
use crate::messages::opm::Opm;
use crate::options::{GenerateOptions, ParseOptions};
use crate::VersionedNdm;
use std::path::Path;

pub use crate::detect::Notation;

/// Strictly convert any detected NDM message between KVN and XML.
pub fn convert(
    input: &str,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<String> {
    let message = crate::from_str_with_options(input, Some(source), parse_options)?;
    match target {
        Notation::Kvn => message.to_kvn_with(generate_options),
        Notation::Xml => message.to_xml_with(generate_options),
    }
}

/// Strictly convert any NDM file and atomically replace the destination on success.
pub fn convert_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let message = crate::from_file_with_options(source_path, Some(source), parse_options)?;
    let output = match target {
        Notation::Kvn => message.to_kvn_with(generate_options),
        Notation::Xml => message.to_xml_with(generate_options),
    }?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}

/// Convert in-memory input and atomically replace the destination on success.
pub fn convert_to_file(
    input: &str,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let output = convert(input, source, target, parse_options, generate_options)?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}

/// Strictly parse an OPM file with bounded input reading and optional notation detection.
pub fn parse_opm_file(
    source_path: impl AsRef<Path>,
    source: Option<Notation>,
    options: &ParseOptions,
) -> Result<Opm> {
    let input = read_input(
        source_path.as_ref(),
        options.max_input_bytes,
        source,
        crate::validation::MessageKind::Opm,
    )?;
    let input = without_utf8_bom(&input);
    match source.map_or_else(|| detect_notation(input), Ok)? {
        Notation::Kvn => Opm::from_kvn_with_options(input, options),
        Notation::Xml => Opm::from_xml_with_options(input, options),
    }
}

/// Strictly parse an OEM file with bounded input reading and optional notation detection.
pub fn parse_oem_file(
    source_path: impl AsRef<Path>,
    source: Option<Notation>,
    options: &ParseOptions,
) -> Result<Oem> {
    let input = read_input(
        source_path.as_ref(),
        options.max_input_bytes,
        source,
        crate::validation::MessageKind::Oem,
    )?;
    let input = without_utf8_bom(&input);
    match source.map_or_else(|| detect_notation(input), Ok)? {
        Notation::Kvn => Oem::from_kvn_with_options(input, options),
        Notation::Xml => Oem::from_xml_with_options(input, options),
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

/// Convert a standalone OEM between KVN and XML without changing its edition by default.
pub fn convert_oem(
    input: &str,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<String> {
    let message = match source {
        Notation::Kvn => Oem::from_kvn_with_options(input, parse_options)?,
        Notation::Xml => Oem::from_xml_with_options(input, parse_options)?,
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

/// Convert an OEM file and atomically replace the destination only after conversion succeeds.
pub fn convert_oem_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let message = parse_oem_file(source_path, Some(source), parse_options)?;
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
    kind: crate::validation::MessageKind,
) -> Result<String> {
    match crate::fsutil::read_to_string(path, max_bytes) {
        Ok(input) => Ok(input),
        Err(error @ crate::error::CcsdsNdmError::ResourceLimitExceeded { .. }) => match source_hint
        {
            Some(notation) => Err(error.with_parse_context(
                kind,
                match notation {
                    Notation::Kvn => crate::error::DiagnosticNotation::Kvn,
                    Notation::Xml => crate::error::DiagnosticNotation::Xml,
                },
                "",
                None,
            )),
            None => Err(error),
        },
        Err(error) => Err(error),
    }
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

/// Convert in-memory OEM input and atomically write the complete destination.
pub fn convert_oem_to_file(
    input: &str,
    destination_path: impl AsRef<Path>,
    source: Notation,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let output = convert_oem(input, source, target, parse_options, generate_options)?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}
