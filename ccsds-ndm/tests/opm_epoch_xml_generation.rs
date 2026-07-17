// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::CalendarEpoch;

const OPM_WITH_MANEUVERS: &str = include_str!("../../data/kvn/opm_g2.kvn");

fn opm() -> Opm {
    Opm::from_kvn(OPM_WITH_MANEUVERS).expect("failed to parse OPM fixture")
}

fn calendar_epoch(value: &str) -> CalendarEpoch {
    CalendarEpoch::new(value)
        .unwrap_or_else(|error| panic!("test calendar epoch {value:?} was rejected: {error}"))
}

#[test]
fn calendar_epoch_rejects_invalid_opm_values() {
    for value in [
        "",
        "+",
        ".",
        "12345",
        "12345.",
        "2023-02-29T00:00:00",
        "2023-01-01T24:00:00",
        "2023-01-01T00:60:00",
        "2023-01-01T00:00:00.",
    ] {
        assert!(
            CalendarEpoch::new(value).is_err(),
            "invalid OPM epoch was accepted: {value:?}"
        );
    }
}

fn replace_element(xml: &str, tag: &str, value: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml
        .find(&open)
        .unwrap_or_else(|| panic!("missing opening tag {open}"));
    let content_start = start + open.len();
    let end = xml[content_start..]
        .find(&close)
        .map(|offset| content_start + offset)
        .unwrap_or_else(|| panic!("missing closing tag {close}"));
    format!(
        "{}{}{}{}{}",
        &xml[..start],
        open,
        value,
        close,
        &xml[end + close.len()..]
    )
}

#[test]
fn xml_parsing_rejects_non_calendar_opm_epochs() {
    let xml = opm().to_xml().expect("fixture should serialize").replacen(
        "</REF_FRAME>",
        "</REF_FRAME><REF_FRAME_EPOCH>2023-01-01T00:00:00</REF_FRAME_EPOCH>",
        1,
    );
    for tag in ["REF_FRAME_EPOCH", "EPOCH", "MAN_EPOCH_IGNITION"] {
        let invalid = replace_element(&xml, tag, "12345.5");
        assert!(
            Opm::from_xml(&invalid).is_err(),
            "numeric {tag} should be rejected"
        );
    }
}

#[test]
fn xml_generation_accepts_calendar_ordinal_and_leap_second_boundaries() {
    for value in [
        "2000-02-29T23:59:60Z",
        "2000-366T23:59:59.1Z",
        "2001-365T00:00:00",
    ] {
        let mut message = opm();
        message.header.creation_date = calendar_epoch(value);
        message.body.segment.metadata.ref_frame_epoch = Some(calendar_epoch(value));
        message.body.segment.data.state_vector.epoch = calendar_epoch(value);
        message.body.segment.data.maneuver_parameters[0].man_epoch_ignition = calendar_epoch(value);
        message
            .to_xml()
            .unwrap_or_else(|error| panic!("valid boundary {value:?} was rejected: {error}"));
    }
}

#[test]
fn xml_parsing_rejects_book_forbidden_timezone_offsets() {
    let xml = opm().to_xml().expect("fixture should serialize");
    for tag in ["CREATION_DATE", "EPOCH", "MAN_EPOCH_IGNITION"] {
        let invalid = replace_element(&xml, tag, "2023-01-01T00:00:00+05:00");
        assert!(
            Opm::from_xml(&invalid).is_err(),
            "timezone offset in {tag} should be rejected by ODM 7.5.10"
        );
    }
}

#[test]
fn xml_parsing_rejects_non_calendar_odm_creation_date() {
    let xml = opm().to_xml().expect("fixture should serialize");
    let invalid = xml.replacen(
        "<CREATION_DATE>2021-06-03T05:33:00.000</CREATION_DATE>",
        "<CREATION_DATE>12345</CREATION_DATE>",
        1,
    );
    assert!(Opm::from_xml(&invalid).is_err());
}
