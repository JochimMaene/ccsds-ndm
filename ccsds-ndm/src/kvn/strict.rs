use crate::error::{CcsdsNdmError, FormatError, KvnParseError, Result};

pub(crate) struct OdmAssignmentRules {
    pub context: &'static str,
    pub message_name: &'static str,
    pub rank: fn(&str) -> Option<u16>,
    pub comment_starts_block: fn(u16, &str) -> bool,
    pub allows_non_increasing: fn(u16, u16, &str) -> bool,
}

/// Validate the lexical, ordering, uniqueness, and comment-placement rules shared by standalone
/// ODM assignment documents. Message-specific functions register only their keyword layout.
pub(crate) fn validate_odm_assignments(kvn: &str, rules: &OdmAssignmentRules) -> Result<()> {
    fn invalid(
        rules: &OdmAssignmentRules,
        line: usize,
        offset: usize,
        message: impl Into<String>,
    ) -> CcsdsNdmError {
        CcsdsNdmError::Format(Box::new(FormatError::Kvn(Box::new(KvnParseError {
            line,
            column: 1,
            message: message.into(),
            contexts: vec![rules.context],
            offset,
        }))))
    }

    let mut previous = None;
    let mut pending_comment = false;
    let mut line_offset = 0usize;
    for (index, raw_line) in kvn.split('\n').enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.as_bytes().contains(&b'\r') {
            return Err(invalid(
                rules,
                line_number,
                line_offset,
                "lone carriage return",
            ));
        }
        if line.len() > 254 {
            return Err(invalid(
                rules,
                line_number,
                line_offset,
                "line exceeds the normative 254-character limit",
            ));
        }
        if !line.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
            return Err(invalid(
                rules,
                line_number,
                line_offset,
                "non-printable or non-ASCII character",
            ));
        }

        let line = line.trim();
        if line.is_empty() {
            line_offset += raw_line.len() + 1;
            continue;
        }
        if line == "COMMENT" || line.starts_with("COMMENT ") {
            pending_comment = true;
            line_offset += raw_line.len() + 1;
            continue;
        }
        if !line.contains('=') {
            return Err(invalid(
                rules,
                line_number,
                line_offset,
                "expected an assignment",
            ));
        }
        let (key, _) = line
            .split_once('=')
            .ok_or_else(|| invalid(rules, line_number, line_offset, "expected an assignment"))?;
        let key = key.trim();
        let current = (rules.rank)(key).ok_or_else(|| {
            invalid(
                rules,
                line_number,
                line_offset,
                format!("unknown {} keyword", rules.message_name),
            )
        })?;

        if pending_comment {
            let Some(previous_rank) = previous else {
                return Err(invalid(
                    rules,
                    line_number,
                    line_offset,
                    "comments before the version would be lost",
                ));
            };
            if !(rules.comment_starts_block)(previous_rank, key) {
                return Err(invalid(
                    rules,
                    line_number,
                    line_offset,
                    "COMMENT is not at the beginning of a logical block",
                ));
            }
            pending_comment = false;
        }

        if let Some(previous_rank) = previous {
            if current <= previous_rank
                && !(rules.allows_non_increasing)(previous_rank, current, key)
            {
                return Err(invalid(
                    rules,
                    line_number,
                    line_offset,
                    format!("duplicate or out-of-order {} keyword", rules.message_name),
                ));
            }
        }
        previous = Some(current);
        line_offset += raw_line.len() + 1;
    }

    if pending_comment {
        return Err(invalid(
            rules,
            kvn.lines().count().max(1),
            kvn.len(),
            "trailing COMMENT has no logical block",
        ));
    }
    Ok(())
}
