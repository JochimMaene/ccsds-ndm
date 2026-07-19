//! Notation conversion composed from strict typed parsers and validated generators.

use crate::error::Result;
use crate::options::{GenerateOptions, ParseOptions};
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
