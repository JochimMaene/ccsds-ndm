// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::mutated;

use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;

#[test]
fn empty_optional_kvn_values_are_none() {
    let kvn = mutated(
        &mutated(
            &mutated(
                &mutated(
                    &mutated(
                        &mutated(
                            &mutated(
                                include_str!("../../data/kvn/cdm_363.kvn"),
                                "START_SCREEN_PERIOD = 2010-03-12T18:29:32.212",
                                "START_SCREEN_PERIOD =",
                            ),
                            "STOP_SCREEN_PERIOD = 2010-03-15T18:29:32.212",
                            "STOP_SCREEN_PERIOD =",
                        ),
                        "COLLISION_PROBABILITY = 4.835E-05",
                        "COLLISION_PROBABILITY =",
                    ),
                    "COLLISION_PROBABILITY_METHOD = FOSTER-1992",
                    "COLLISION_PROBABILITY_METHOD =",
                ),
                "TIME_LASTOB_START = 2010-03-12T02:14:12.746",
                "TIME_LASTOB_START =",
            ),
            "TIME_LASTOB_END = 2010-03-12T02:14:12.746",
            "TIME_LASTOB_END =",
        ),
        "AREA_PC = 5.2 [m**2]",
        "AREA_PC = [m**2]",
    );

    let cdm = Cdm::from_kvn(&kvn).unwrap();
    let relative = &cdm.body.relative_metadata_data;
    assert!(relative.start_screen_period.is_none());
    assert!(relative.stop_screen_period.is_none());
    assert!(relative.collision_probability.is_none());
    assert!(relative.collision_probability_method.is_none());

    let data = &cdm.body.segments[0].data;
    let od = data.od_parameters.as_ref().unwrap();
    assert!(od.time_lastob_start.is_none());
    assert!(od.time_lastob_end.is_none());
    assert!(data
        .additional_parameters
        .as_ref()
        .unwrap()
        .area_pc
        .is_none());
}

#[test]
fn omitted_optional_xml_values_are_none() {
    let xml = mutated(
        &mutated(
            include_str!("../../data/xml/cdm_44.xml"),
            "<START_SCREEN_PERIOD>2010-03-12T18:29:32.212</START_SCREEN_PERIOD>\n",
            "",
        ),
        "<STOP_SCREEN_PERIOD>2010-03-15T18:29:32.212</STOP_SCREEN_PERIOD>\n",
        "",
    );

    let cdm = Cdm::from_xml(&xml).unwrap();
    assert!(cdm
        .body
        .relative_metadata_data
        .start_screen_period
        .is_none());
    assert!(cdm.body.relative_metadata_data.stop_screen_period.is_none());
}

#[test]
fn operational_xml_missing_leaves_are_absent_without_erasing_text() {
    let original = Cdm::from_kvn(crate::KVN).unwrap().to_xml().unwrap();
    for (field, missing) in [
        ("RELATIVE_SPEED", "<RELATIVE_SPEED/>"),
        (
            "START_SCREEN_PERIOD",
            "<START_SCREEN_PERIOD>  </START_SCREEN_PERIOD>",
        ),
        ("SCREEN_VOLUME_SHAPE", "<SCREEN_VOLUME_SHAPE nil=\"true\"/>"),
        ("AREA_PC", "<AREA_PC units=\"m**2\" nil=\"1\"/>"),
        ("OBJECT_TYPE", "<OBJECT_TYPE/>"),
        ("MESSAGE_FOR", "<MESSAGE_FOR nil=\"true\"/>"),
    ] {
        let mut expected = Cdm::from_xml(&original).unwrap();
        match field {
            "RELATIVE_SPEED" => expected.body.relative_metadata_data.relative_speed = None,
            "START_SCREEN_PERIOD" => {
                expected.body.relative_metadata_data.start_screen_period = None
            }
            "SCREEN_VOLUME_SHAPE" => {
                expected.body.relative_metadata_data.screen_volume_shape = None
            }
            "AREA_PC" => {
                expected.body.segments[0]
                    .data
                    .additional_parameters
                    .as_mut()
                    .unwrap()
                    .area_pc = None
            }
            "OBJECT_TYPE" => expected.body.segments[0].metadata.object_type = None,
            "MESSAGE_FOR" => expected.header.message_for = None,
            _ => unreachable!(),
        }
        let start = original.find(&format!("<{field}")).unwrap();
        let close = format!("</{field}>");
        let end = original[start..].find(&close).unwrap() + start + close.len();
        let mut input = original.clone();
        input.replace_range(start..end, missing);
        let parsed = Cdm::from_xml(&input).unwrap();
        assert_eq!(parsed, expected, "{field}");
        let output = parsed.to_xml().unwrap();
        crate::common::validate_xml("CDM operational missing leaves", &output);
        assert_eq!(Cdm::from_kvn(&parsed.to_kvn().unwrap()).unwrap(), parsed);
    }
    // Empty optional text is an unavailable value too, so the message still converts to KVN.
    let empty_text = mutated(
        &original,
        "<MESSAGE_FOR>SATELLITE A</MESSAGE_FOR>",
        "<MESSAGE_FOR/>",
    );
    let parsed = Cdm::from_xml(&empty_text).unwrap();
    assert_eq!(parsed.header.message_for, None);
    parsed.to_kvn().unwrap();
}

#[test]
fn blank_optional_kvn_values_create_no_block() {
    let segment = |kvn: &str| {
        let start = kvn.find("COMMENT Object1 OD Parameters").unwrap();
        let end = kvn.find("COMMENT Object1 State Vector").unwrap();
        (start, end)
    };
    let (start, end) = segment(crate::KVN);
    let mut kvn = crate::KVN.to_owned();
    kvn.replace_range(
        start..end,
        "COMMENT od\nOBS_USED =\nCOMMENT additional\nMASS = [kg]\n",
    );
    let cdm = Cdm::from_kvn(&kvn).unwrap();
    let data = &cdm.body.segments[0].data;
    assert!(data.od_parameters.is_none());
    assert!(data.additional_parameters.is_none());
    // No data keyword has a value before X, so the whole run is the leading data run.
    assert_eq!(
        data.comment,
        ["Object1 Data", "od", "additional", "Object1 State Vector"]
    );
    assert!(data.state_vector.comment.is_empty());
    assert_eq!(Cdm::from_kvn(&cdm.to_kvn().unwrap()).unwrap(), cdm);
}

#[test]
fn empty_comment_free_xml_blocks_are_absent() {
    let original = Cdm::from_kvn(crate::KVN).unwrap().to_xml().unwrap();
    let start = original.find("<odParameters>").unwrap();
    let end = original.find("</odParameters>").unwrap() + "</odParameters>".len();
    let mut xml = original.clone();
    xml.replace_range(
        start..end,
        "<odParameters><OBS_USED nil=\"true\"/><WEIGHTED_RMS/></odParameters>",
    );
    let cdm = Cdm::from_xml(&xml).unwrap();
    assert!(cdm.body.segments[0].data.od_parameters.is_none());
    Cdm::from_kvn(&cdm.to_kvn().unwrap()).unwrap();
}

#[test]
fn nil_and_empty_required_values_name_the_problem() {
    let original = Cdm::from_kvn(crate::KVN).unwrap().to_xml().unwrap();
    let start = original.find("<X ").unwrap();
    let end = original[start..].find("</X>").unwrap() + start + "</X>".len();
    let mut nil = original.clone();
    nil.replace_range(start..end, "<X units=\"km\" nil=\"true\"/>");
    let error = Cdm::from_xml(&nil).unwrap_err().to_string();
    assert!(error.contains("X is required and cannot be nil"), "{error}");

    let empty = mutated(
        &original,
        &format!(
            "<ORIGINATOR>{}</ORIGINATOR>",
            Cdm::from_kvn(crate::KVN).unwrap().header.originator
        ),
        "<ORIGINATOR/>",
    );
    let error = Cdm::from_xml(&empty).unwrap_err();
    assert_eq!(error.code(), Some("validation.invalid_value"), "{error}");
    assert!(error.to_string().contains("ORIGINATOR is empty"), "{error}");
}

#[test]
fn nil_cannot_hide_required_values_content_or_unknown_attributes() {
    let original = Cdm::from_kvn(crate::KVN).unwrap().to_xml().unwrap();
    for (field, replacement) in [
        ("TCA", "<TCA nil=\"true\"/>"),
        ("MESSAGE_FOR", "<MESSAGE_FOR nil=\"true\" units=\"m\"/>"),
        (
            "COLLISION_PROBABILITY",
            "<COLLISION_PROBABILITY units=\"n/a\">0.5</COLLISION_PROBABILITY>",
        ),
        (
            "WEIGHTED_RMS",
            "<WEIGHTED_RMS units=\"n/a\">0.5</WEIGHTED_RMS>",
        ),
        (
            "RELATIVE_SPEED",
            "<RELATIVE_SPEED nil=\"true\">7</RELATIVE_SPEED>",
        ),
        ("RELATIVE_SPEED", "<RELATIVE_SPEED unexpected=\"value\"/>"),
        ("RELATIVE_SPEED", "<RELATIVE_SPEED nil=\"false\"/>"),
        ("RELATIVE_SPEED", "<RELATIVE_SPEED>\u{c}</RELATIVE_SPEED>"),
        (
            "RELATIVE_SPEED",
            "<RELATIVE_SPEED nil=\"true\" units=\"<\"/>",
        ),
        (
            "RELATIVE_SPEED",
            "<RELATIVE_SPEED nil=\"true\" units=\"&unknown;\"/>",
        ),
        (
            "RELATIVE_SPEED",
            "<RELATIVE_SPEED nil=\"true\"><unknown/></RELATIVE_SPEED>",
        ),
    ] {
        let start = original.find(&format!("<{field}")).unwrap();
        let close = format!("</{field}>");
        let end = original[start..].find(&close).unwrap() + start + close.len();
        let mut input = original.clone();
        input.replace_range(start..end, replacement);
        assert!(Cdm::from_xml(&input).is_err(), "{replacement}");
    }
}

#[test]
fn velocity_only_kvn_is_rejected_instead_of_silently_discarded() {
    let input = crate::KVN
        .lines()
        .filter(|line| !line.starts_with("RELATIVE_POSITION_"))
        .collect::<Vec<_>>()
        .join("\n");
    let error = Cdm::from_kvn(&input).unwrap_err();
    assert!(error.to_string().contains("RELATIVE_POSITION_R"), "{error}");
}
