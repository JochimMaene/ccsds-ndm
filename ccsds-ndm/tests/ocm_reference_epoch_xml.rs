// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::traits::Ndm;

const OCM_XML: &str = include_str!("../data/xml/ocm_g20.xml");

const OCM_FRAME_EPOCH_KVN: &str = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_FRAME_EPOCH = 2023-01-01T00:00:00
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
TRAJ_STOP
PHYS_START
OEB_PARENT_FRAME_EPOCH = 2023-01-01T00:00:00
PHYS_STOP
COV_START
COV_REF_FRAME = GCRF
COV_FRAME_EPOCH = 2023-01-01T00:00:00
COV_TYPE = CARTPV
COV_ORDERING = LTM
2023-01-01T00:00:00 1 0 0 0 0 1
COV_STOP
MAN_START
MAN_ID = MAN-1
MAN_DEVICE_ID = THR-1
MAN_REF_FRAME = GCRF
MAN_FRAME_EPOCH = 2023-01-01T00:00:00
MAN_COMPOSITION = TIME_ABSOLUTE, DV_X
2023-01-01T00:00:00 1
MAN_STOP
"#;

const OCM_TRAJ_USEABLE_KVN: &str = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
USEABLE_START_TIME = 2023-01-01T00:01:00
USEABLE_STOP_TIME = 2023-01-01T00:02:00
TRAJ_TYPE = CARTPV
2023-01-01T00:00:00 1 2 3 4 5 6
2023-01-01T00:00:10 7 8 9 10 11 12
TRAJ_STOP
"#;

#[test]
fn xml_parsing_rejects_numeric_ocm_epoch_tzero() {
    let invalid = OCM_XML.replace(
        "<EPOCH_TZERO>2022-12-18T00:00:00.0000</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    assert!(Ocm::from_xml(&invalid).is_err());
}

#[test]
fn xml_parsing_rejects_numeric_optional_reference_epochs() {
    let previous = OCM_XML.replace(
        "</EPOCH_TZERO>",
        "</EPOCH_TZERO><PREVIOUS_MESSAGE_EPOCH>123.5</PREVIOUS_MESSAGE_EPOCH>",
    );
    assert!(Ocm::from_xml(&previous).is_err());

    let next = OCM_XML.replace(
        "</EPOCH_TZERO>",
        "</EPOCH_TZERO><NEXT_MESSAGE_EPOCH>123.5</NEXT_MESSAGE_EPOCH>",
    );
    assert!(Ocm::from_xml(&next).is_err());

    let leap = OCM_XML.replace(
        "</TAIMUTC_AT_TZERO>",
        "</TAIMUTC_AT_TZERO><NEXT_LEAP_EPOCH>123.5</NEXT_LEAP_EPOCH>",
    );
    assert!(Ocm::from_xml(&leap).is_err());
}

#[test]
fn xml_parsing_rejects_numeric_frame_reference_epochs() {
    let xml = Ocm::from_kvn(OCM_FRAME_EPOCH_KVN)
        .expect("frame-reference KVN fixture should parse")
        .to_xml()
        .expect("frame-reference fixture should serialize");
    Ocm::from_xml(&xml).expect("generated frame-reference XML should parse");

    for tag in [
        "TRAJ_FRAME_EPOCH",
        "OEB_PARENT_FRAME_EPOCH",
        "COV_FRAME_EPOCH",
        "MAN_FRAME_EPOCH",
    ] {
        let needle = format!("<{tag}>2023-01-01T00:00:00</{tag}>");
        let replacement = format!("<{tag}>123.5</{tag}>");
        let invalid = xml.replace(&needle, &replacement);
        assert!(xml.contains(&needle), "generated XML should contain {tag}");
        assert!(
            Ocm::from_xml(&invalid).is_err(),
            "{tag} must reject a numeric epoch"
        );
    }
}

#[test]
fn xml_empty_optional_physical_block_remains_supported() {
    let start = OCM_XML.find("<phys>").expect("physical block start");
    let end = OCM_XML[start..]
        .find("</phys>")
        .map(|offset| start + offset + "</phys>".len())
        .expect("physical block end");
    let xml = format!("{}<phys/>{}", &OCM_XML[..start], &OCM_XML[end..]);
    assert!(Ocm::from_xml(&xml).is_ok());
}

#[test]
fn sclk_requires_both_reference_parameters_on_kvn_and_xml_paths() {
    let invalid_xml = OCM_XML.replace(
        "<TIME_SYSTEM>UT1</TIME_SYSTEM>",
        "<TIME_SYSTEM>SCLK</TIME_SYSTEM>",
    );
    assert!(Ocm::from_xml(&invalid_xml).is_err());

    let invalid_kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = SCLK
EPOCH_TZERO = 2023-01-01T00:00:00
META_STOP
TRAJ_START
CENTER_NAME = EARTH
TRAJ_REF_FRAME = GCRF
TRAJ_TYPE = CARTPV
0.0 1 2 3 4 5 6
TRAJ_STOP
"#;
    assert!(Ocm::from_kvn(invalid_kvn).is_err());

    let valid_xml = invalid_xml.replace(
        "</EPOCH_TZERO>",
        "</EPOCH_TZERO><SCLK_OFFSET_AT_EPOCH units=\"s\">0</SCLK_OFFSET_AT_EPOCH><SCLK_SEC_PER_SI_SEC units=\"s\">1</SCLK_SEC_PER_SI_SEC>",
    );
    assert!(Ocm::from_xml(&valid_xml).is_ok());

    let valid_kvn = invalid_kvn.replace(
        "EPOCH_TZERO = 2023-01-01T00:00:00\n",
        "EPOCH_TZERO = 2023-01-01T00:00:00\nSCLK_OFFSET_AT_EPOCH = 0 [s]\nSCLK_SEC_PER_SI_SEC = 1 [s]\n",
    );
    assert!(Ocm::from_kvn(&valid_kvn).is_ok());
}

#[test]
fn trajectory_useable_times_require_calendar_or_ordinal_form() {
    let ocm =
        Ocm::from_kvn(OCM_TRAJ_USEABLE_KVN).expect("trajectory useable-time fixture should parse");
    let trajectory = &ocm.body.segment.data.traj[0];
    assert_eq!(
        trajectory.useable_start_time.as_ref().unwrap().as_str(),
        "2023-01-01T00:01:00"
    );
    assert_eq!(
        trajectory.useable_stop_time.as_ref().unwrap().as_str(),
        "2023-01-01T00:02:00"
    );

    for tag in ["USEABLE_START_TIME", "USEABLE_STOP_TIME"] {
        let needle = if tag.ends_with("START_TIME") {
            format!("{tag} = 2023-01-01T00:01:00")
        } else {
            format!("{tag} = 2023-01-01T00:02:00")
        };
        let invalid = OCM_TRAJ_USEABLE_KVN.replace(&needle, &format!("{tag} = 123.5"));
        assert!(Ocm::from_kvn(&invalid).is_err(), "accepted numeric {tag}");
    }

    let xml = ocm
        .to_xml()
        .expect("trajectory useable-time fixture should serialize");
    for tag in ["USEABLE_START_TIME", "USEABLE_STOP_TIME"] {
        let needle = if tag.ends_with("START_TIME") {
            "2023-01-01T00:01:00"
        } else {
            "2023-01-01T00:02:00"
        };
        let invalid = xml.replace(
            &format!("<{tag}>{needle}</{tag}>"),
            &format!("<{tag}>123.5</{tag}>"),
        );
        assert!(
            Ocm::from_xml(&invalid).is_err(),
            "accepted numeric XML {tag}"
        );
    }
}

#[test]
fn trajectory_history_time_tags_must_use_one_epoch_branch() {
    let mixed = OCM_TRAJ_USEABLE_KVN.replace("TRAJ_STOP", "123.5 7 8 9 10 11 12\nTRAJ_STOP");
    assert!(
        Ocm::from_kvn(&mixed).is_err(),
        "accepted mixed calendar/numeric trajectory time tags"
    );

    let valid = Ocm::from_kvn(OCM_TRAJ_USEABLE_KVN).expect("fixture should parse");
    let xml = valid.to_xml().expect("fixture should serialize");
    let mixed_xml = xml.replace(
        "<trajLine>2023-01-01T00:00:10 7 8 9 10 11 12</trajLine>",
        "<trajLine>123.5 7 8 9 10 11 12</trajLine>",
    );
    assert!(
        Ocm::from_xml(&mixed_xml).is_err(),
        "accepted mixed calendar/numeric XML trajectory time tags"
    );
}
