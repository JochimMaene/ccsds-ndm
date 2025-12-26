// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::EpochError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CcsdsNdmError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML deserialization error: {0}")]
    XmlDe(#[from] quick_xml::DeError),

    #[error("XML parsing error: {0}")]
    XmlParse(#[from] quick_xml::Error),

    #[error("XML serialization error: {0}")]
    XmlSer(String),

    #[error("KVN Parsing Error: {0}")]
    KvnParse(String),

    #[error("Epoch error: {0}")]
    Epoch(#[from] EpochError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Unsupported message type: {0}")]
    UnsupportedMessage(String),

    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Missing required KVN field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, CcsdsNdmError>;
