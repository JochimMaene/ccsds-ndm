// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! A CCSDS XML message must appear at the document root. Wrapping one in any
//! outer element is rejected during notation/type detection, whether the
//! wrapper looks plausible (`<message>`) or arbitrary (`<somethingExtra>`).

mod common;

use ccsds_ndm::error::CcsdsNdmError;
use ccsds_ndm::from_str;

const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

fn valid_cdm() -> String {
    std::fs::read_to_string(common::data_dir().join("xml").join("cdm_44.xml"))
        .expect("CDM fixture is readable")
}

/// Wrap the fixture's root element in `wrapper`, keeping the XML declaration
/// first so the document stays well-formed and only the nesting is at fault.
fn wrapped_in(wrapper: &str) -> String {
    let cdm = valid_cdm();
    let body = cdm
        .split_once("?>")
        .map_or(cdm.as_str(), |(_, rest)| rest)
        .trim();
    format!("{XML_DECL}\n<{wrapper}>\n{body}\n</{wrapper}>\n")
}

/// Baseline: the fixture parses on its own, so a rejection below is caused by
/// the wrapper and not by an unrelated defect in the document.
#[test]
fn unwrapped_fixture_parses() {
    assert!(
        from_str(&valid_cdm()).is_ok(),
        "CDM fixture must parse unwrapped for the wrapper cases to mean anything"
    );
}

/// Detection reports the bad root as `UnsupportedMessage`, which `from_str`
/// then wraps in parse context. Look through that wrapper so the assertion
/// pins the cause rather than the layer it arrives in.
fn unsupported_message_detail(error: &CcsdsNdmError) -> Option<&str> {
    match error {
        CcsdsNdmError::UnsupportedMessage(detail) => Some(detail),
        CcsdsNdmError::Parsing { source, .. } => unsupported_message_detail(source),
        _ => None,
    }
}

#[test]
fn wrapped_message_is_rejected_by_root_detection() {
    for wrapper in ["message", "somethingExtra"] {
        let error = from_str(&wrapped_in(wrapper))
            .expect_err(&format!("<{wrapper}> wrapper must be rejected"));

        let detail = unsupported_message_detail(&error).unwrap_or_else(|| {
            panic!("expected UnsupportedMessage for <{wrapper}>, got {error:?}")
        });
        assert!(
            detail.contains(wrapper),
            "diagnostic for <{wrapper}> should name the offending root, got: {detail}"
        );
    }
}
