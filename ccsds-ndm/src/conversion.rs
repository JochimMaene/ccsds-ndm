//! Notation conversion composed from strict typed parsers and validated generators.

use crate::error::Result;
use std::path::Path;

use crate::detect::Notation;

/// Strictly convert any detected NDM message between KVN and XML.
pub fn convert(input: &str, target: Notation) -> Result<String> {
    let message = crate::from_str(input)?;
    match target {
        Notation::Kvn => message.to_kvn(),
        Notation::Xml => message.to_xml(),
    }
}

/// Strictly convert any NDM file and atomically replace the destination on success.
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
