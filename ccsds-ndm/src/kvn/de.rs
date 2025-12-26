// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::{CcsdsNdmError, Result};
use std::iter::Enumerate;
use std::str::Lines;

#[derive(Debug, Clone, PartialEq)]
pub enum KvnLine<'a> {
    /// A comment line (starts with COMMENT). Content is trimmed.
    Comment(&'a str),
    /// A Key-Value pair, optionally with a Unit.
    Pair {
        key: &'a str,
        val: &'a str,
        unit: Option<&'a str>,
    },
    /// A block start tag (e.g., META_START).
    BlockStart(&'a str),
    /// A block end tag (e.g., META_STOP).
    BlockEnd(&'a str),
    /// A raw data line (space-delimited numbers, no equals sign).
    Raw(&'a str),
    /// An empty or whitespace-only line.
    Empty,
}

/// Tokenizer for CCSDS KVN data.
pub struct KvnTokenizer<'a> {
    lines: Enumerate<Lines<'a>>,
}

impl<'a> KvnTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().enumerate(),
        }
    }
}

impl<'a> Iterator for KvnTokenizer<'a> {
    type Item = Result<KvnLine<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line_num0, raw_line) = self.lines.next()?;
        let line_num = line_num0 + 1;



        let line = raw_line.trim();

        if line.is_empty() {
            return Some(Ok(KvnLine::Empty));
        }

        if line.starts_with("COMMENT") {
            // Check boundary: "COMMENT" must be the whole line OR followed by whitespace
            if line.len() == 7 {
                return Some(Ok(KvnLine::Comment("")));
            }
            if line.as_bytes()[7].is_ascii_whitespace() {
                let content = line[7..].trim();
                return Some(Ok(KvnLine::Comment(content)));
            }
        }

        // Keywords must be uppercase, no spaces.
        if line.ends_with("_START") {
            let tag = line.trim_end_matches("_START");
            // Validation: Keyword must not contain spaces
            if !tag.contains(char::is_whitespace) {
                return Some(Ok(KvnLine::BlockStart(tag)));
            }
        }

        if line.ends_with("_STOP") || line.ends_with("_END") {
            let tag = if line.ends_with("_STOP") {
                line.trim_end_matches("_STOP")
            } else {
                line.trim_end_matches("_END")
            };

            if !tag.contains(char::is_whitespace) {
                return Some(Ok(KvnLine::BlockEnd(tag)));
            }
        }

        // Look for the *first* equals sign.
        if let Some((key_part, val_part)) = line.split_once('=') {
            let key = key_part.trim();

            // Validation: Keys must not contain spaces
            if key.contains(char::is_whitespace) {
                return Some(Err(CcsdsNdmError::KvnParse(format!(
                    "Line {}: Keyword '{}' contains invalid whitespace",
                    line_num, key
                ))));
            }

            let mut val_raw = val_part.trim();
            let mut unit = None;

            // Check for units [xxx] at the end
            if val_raw.ends_with(']') {
                if let Some(open_bracket) = val_raw.rfind('[') {
                    let unit_str = val_raw[open_bracket + 1..val_raw.len() - 1].trim();
                    let value_str = val_raw[..open_bracket].trim();

                    if unit_str.is_empty() {
                        return Some(Err(CcsdsNdmError::KvnParse(format!(
                            "Line {}: Empty unit brackets",
                            line_num
                        ))));
                    }

                    val_raw = value_str;
                    unit = Some(unit_str);
                }
            }

            return Some(Ok(KvnLine::Pair {
                key,
                val: val_raw,
                unit,
            }));
        }

        // If it's not empty, not a comment, not a block tag, and has no equals, it's raw data.
        Some(Ok(KvnLine::Raw(line)))
    }
}
