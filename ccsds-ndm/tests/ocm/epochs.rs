// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::Ndm;

const OCM_XML: &str = include_str!("../../data/xml/ocm_g20.xml");

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
    let invalid = mutated(
        OCM_XML,
        "<EPOCH_TZERO>2022-12-18T00:00:00.0000</EPOCH_TZERO>",
        "<EPOCH_TZERO>123.5</EPOCH_TZERO>",
    );
    crate::common::assert_invalid_epoch(&Ocm::from_xml(&invalid).unwrap_err(), "123.5");
}

#[test]
fn xml_parsing_rejects_numeric_optional_reference_epochs() {
    for (tag, after) in [
        ("PREVIOUS_MESSAGE_EPOCH", "</EPOCH_TZERO>"),
        ("NEXT_MESSAGE_EPOCH", "</EPOCH_TZERO>"),
        ("NEXT_LEAP_EPOCH", "</TAIMUTC_AT_TZERO>"),
    ] {
        let calendar = format!("<{tag}>2023-01-01T00:00:00</{tag}>");
        let valid = mutated(OCM_XML, after, &format!("{after}{calendar}"));
        Ocm::from_xml(&valid).unwrap_or_else(|error| panic!("{tag} control: {error}"));
        let invalid = mutated(&valid, &calendar, &format!("<{tag}>123.5</{tag}>"));
        crate::common::assert_invalid_epoch(&Ocm::from_xml(&invalid).unwrap_err(), "123.5");
    }
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
        let invalid = mutated(&xml, &needle, &replacement);
        assert!(xml.contains(&needle), "generated XML should contain {tag}");
        crate::common::assert_invalid_epoch(&Ocm::from_xml(&invalid).unwrap_err(), "123.5");
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
    let invalid_xml = mutated(
        OCM_XML,
        "<TIME_SYSTEM>UT1</TIME_SYSTEM>",
        "<TIME_SYSTEM>SCLK</TIME_SYSTEM>",
    );
    crate::common::assert_validation_field(
        &Ocm::from_xml(&invalid_xml).unwrap_err(),
        "SCLK_OFFSET_AT_EPOCH",
    );

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
    crate::common::assert_validation_field(
        &Ocm::from_kvn(invalid_kvn).unwrap_err(),
        "SCLK_OFFSET_AT_EPOCH",
    );

    let valid_xml = mutated(&invalid_xml, "</EPOCH_TZERO>",
        "</EPOCH_TZERO><SCLK_OFFSET_AT_EPOCH units=\"s\">0</SCLK_OFFSET_AT_EPOCH><SCLK_SEC_PER_SI_SEC units=\"s\">1</SCLK_SEC_PER_SI_SEC>",
    );
    assert!(Ocm::from_xml(&valid_xml).is_ok());

    let valid_kvn = mutated(invalid_kvn, "EPOCH_TZERO = 2023-01-01T00:00:00\n",
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
        let invalid = mutated(OCM_TRAJ_USEABLE_KVN, &needle, &format!("{tag} = 123.5"));
        crate::common::assert_invalid_epoch(&Ocm::from_kvn(&invalid).unwrap_err(), "123.5");
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
        let invalid = mutated(
            &xml,
            &format!("<{tag}>{needle}</{tag}>"),
            &format!("<{tag}>123.5</{tag}>"),
        );
        crate::common::assert_invalid_epoch(&Ocm::from_xml(&invalid).unwrap_err(), "123.5");
    }
}

#[test]
fn trajectory_history_time_tags_must_use_one_epoch_branch() {
    let mixed = mutated(
        OCM_TRAJ_USEABLE_KVN,
        "TRAJ_STOP",
        "123.5 7 8 9 10 11 12\nTRAJ_STOP",
    );
    crate::common::assert_validation_field(&Ocm::from_kvn(&mixed).unwrap_err(), "trajLine epoch");

    let valid = Ocm::from_kvn(OCM_TRAJ_USEABLE_KVN).expect("fixture should parse");
    let xml = valid.to_xml().expect("fixture should serialize");
    let mixed_xml = mutated(
        &xml,
        "<trajLine>2023-01-01T00:00:10 7 8 9 10 11 12</trajLine>",
        "<trajLine>123.5 7 8 9 10 11 12</trajLine>",
    );
    crate::common::assert_validation_field(
        &Ocm::from_xml(&mixed_xml).unwrap_err(),
        "trajLine epoch",
    );
}

#[test]
fn metadata_reference_epochs_require_calendar_form() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
META_START
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
PREVIOUS_MESSAGE_EPOCH = 2022-12-31T23:00:00
NEXT_MESSAGE_EPOCH = 2023-01-01T01:00:00
NEXT_LEAP_EPOCH = 2024-01-01T00:00:00
NEXT_LEAP_TAIMUTC = 37 [s]
META_STOP
"#;

    for key in [
        "EPOCH_TZERO",
        "PREVIOUS_MESSAGE_EPOCH",
        "NEXT_MESSAGE_EPOCH",
        "NEXT_LEAP_EPOCH",
    ] {
        let needle = format!("{key} = ");
        let start = kvn.find(&needle).expect("reference epoch key in fixture") + needle.len();
        let end = kvn[start..]
            .find('\n')
            .map_or(kvn.len(), |offset| start + offset);
        let invalid = format!("{}123.5{}", &kvn[..start], &kvn[end..]);
        crate::common::assert_invalid_epoch(&Ocm::from_kvn(&invalid).unwrap_err(), "123.5");
    }

    let ocm = Ocm::from_kvn(kvn).expect("calendar reference epochs should parse");
    assert_eq!(
        ocm.body.segment.metadata.epoch_tzero.as_str(),
        "2023-01-01T00:00:00"
    );
    assert_eq!(
        ocm.body.segment.metadata.next_leap_epoch.unwrap().as_str(),
        "2024-01-01T00:00:00"
    );
}

#[test]
fn frame_reference_epochs_require_calendar_form() {
    let kvn = r#"CCSDS_OCM_VERS = 3.0
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

    let ocm = Ocm::from_kvn(kvn).expect("calendar frame epochs should parse");
    let segment = &ocm.body.segment;
    assert_eq!(
        segment.data.traj[0].traj_frame_epoch.unwrap().as_str(),
        "2023-01-01T00:00:00"
    );
    assert_eq!(
        segment
            .data
            .phys
            .as_ref()
            .unwrap()
            .oeb_parent_frame_epoch
            .unwrap()
            .as_str(),
        "2023-01-01T00:00:00"
    );
    assert_eq!(
        segment.data.cov[0].cov_frame_epoch.unwrap().as_str(),
        "2023-01-01T00:00:00"
    );
    assert_eq!(
        segment.data.man[0].man_frame_epoch.unwrap().as_str(),
        "2023-01-01T00:00:00"
    );

    for key in [
        "TRAJ_FRAME_EPOCH",
        "OEB_PARENT_FRAME_EPOCH",
        "COV_FRAME_EPOCH",
        "MAN_FRAME_EPOCH",
    ] {
        let invalid = kvn.replace(
            &format!("{key} = 2023-01-01T00:00:00"),
            &format!("{key} = 123.5"),
        );
        crate::common::assert_invalid_epoch(&Ocm::from_kvn(&invalid).unwrap_err(), "123.5");
    }
}
