//! Notation conversion through typed parsers and validated generators.

use crate::error::Result;
use std::path::Path;

use crate::detect::Notation;

/// Parse any detected NDM message and generate validated KVN or XML.
pub fn convert(input: &str, target: Notation) -> Result<String> {
    let message = crate::from_str(input)?;
    match target {
        Notation::Kvn => message.to_kvn(),
        Notation::Xml => message.to_xml(),
    }
}

/// Convert any NDM file and atomically replace the destination on success.
pub fn convert_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
    target: Notation,
) -> Result<()> {
    let message = crate::from_file(source_path)?;
    match target {
        Notation::Kvn => message.to_kvn_file(destination_path),
        Notation::Xml => message.to_xml_file(destination_path),
    }
}
