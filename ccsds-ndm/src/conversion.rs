//! Notation conversion composed from strict typed parsers and validated generators.

use crate::error::Result;
use crate::options::{GenerateOptions, ParseOptions};
use std::path::Path;

pub use crate::detect::Notation;

/// Strictly convert any detected NDM message between KVN and XML.
pub fn convert(input: &str, target: Notation) -> Result<String> {
    convert_with_options(
        input,
        target,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
}

/// Strictly convert with explicit parsing and generation controls.
pub fn convert_with_options(
    input: &str,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<String> {
    let message = crate::from_str_with_options(input, None, parse_options)?;
    match target {
        Notation::Kvn => message.to_kvn_with(generate_options),
        Notation::Xml => message.to_xml_with(generate_options),
    }
}

/// Strictly convert any NDM file and atomically replace the destination on success.
pub fn convert_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    target: Notation,
) -> Result<()> {
    convert_file_with_options(
        source_path,
        destination_path,
        target,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
}

/// Strictly convert a file with explicit parsing and generation controls.
pub fn convert_file_with_options(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let message = crate::from_file_with_options(source_path, None, parse_options)?;
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
    target: Notation,
) -> Result<()> {
    convert_to_file_with_options(
        input,
        destination_path,
        target,
        &ParseOptions::default(),
        &GenerateOptions::source(),
    )
}

/// Convert in-memory input to a file with explicit parsing and generation controls.
pub fn convert_to_file_with_options(
    input: &str,
    destination_path: impl AsRef<Path>,
    target: Notation,
    parse_options: &ParseOptions,
    generate_options: &GenerateOptions,
) -> Result<()> {
    let output = convert_with_options(input, target, parse_options, generate_options)?;
    crate::fsutil::atomic_write(destination_path.as_ref(), output.as_bytes())
}
