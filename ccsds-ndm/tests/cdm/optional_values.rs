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
