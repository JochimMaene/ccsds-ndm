// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Implicit-unit XML records require no per-record scratch allocation; explicit units have a bounded cost.
//!
//! `Epoch` owns a fixed-size buffer specifically so that a large ephemeris does not put one heap
//! allocation on every record, and the XML structural walker keeps open element names in a stack
//! buffer for the same reason. Both are invisible to a correctness test, so they are pinned here.

use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::Ndm;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn oem_xml(records: usize) -> String {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        "\n<oem xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' id=\"CCSDS_OEM_VERS\" version=\"3.0\">\n",
        "<header><CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>",
        "<ORIGINATOR>TEST</ORIGINATOR></header>\n<body><segment><metadata>",
        "<OBJECT_NAME>SAT</OBJECT_NAME><OBJECT_ID>2023-001A</OBJECT_ID>",
        "<CENTER_NAME>EARTH</CENTER_NAME><REF_FRAME>GCRF</REF_FRAME>",
        "<TIME_SYSTEM>UTC</TIME_SYSTEM>",
        "<START_TIME>2023-01-01T00:00:00</START_TIME>",
        "<STOP_TIME>2023-01-31T00:00:00</STOP_TIME></metadata><data>\n",
    ));
    for record in 0..records {
        let minute = record % 60;
        let hour = (record / 60) % 24;
        let day = 1 + record / 1_440;
        xml.push_str(&format!(
            "<stateVector><EPOCH>2023-01-{day:02}T{hour:02}:{minute:02}:00</EPOCH>\
             <X>1.0</X><Y>2.0</Y><Z>3.0</Z>\
             <X_DOT>4.0</X_DOT><Y_DOT>5.0</Y_DOT><Z_DOT>6.0</Z_DOT></stateVector>\n"
        ));
    }
    xml.push_str("</data></segment></body></oem>\n");
    xml
}

fn parse_stats(xml: &str, expected_records: usize) -> Stats {
    let region = Region::new(GLOBAL);
    let parsed = Oem::from_xml(black_box(xml)).unwrap();
    let stats = region.change();
    assert_eq!(
        parsed.body.segment[0].data.state_vector.len(),
        expected_records
    );
    black_box(parsed);
    stats
}

#[test]
fn oem_xml_parsing_has_bounded_scratch_allocations() {
    let small_records = 100;
    let large_records = 2_000;
    let small_xml = oem_xml(small_records);
    let large_xml = oem_xml(large_records);

    // Exercise optional acceleration elements too: their former nullable adapter allocated
    // JSON maps per component, invisible to a six-column-only budget.
    for (acceleration, allocations_per_record) in [
        ("", 0),
        (
            "<X_DDOT>0.1</X_DDOT><Y_DDOT>0.2</Y_DDOT><Z_DDOT>0.3</Z_DDOT>",
            0,
        ),
        (
            "<X_DDOT units=\"km/s**2\">0.1</X_DDOT><Y_DDOT>0.2</Y_DDOT><Z_DDOT>0.3</Z_DDOT>",
            3,
        ),
    ] {
        let small_xml =
            small_xml.replace("</stateVector>", &format!("{acceleration}</stateVector>"));
        let large_xml =
            large_xml.replace("</stateVector>", &format!("{acceleration}</stateVector>"));
        black_box(Oem::from_xml(&small_xml).unwrap());
        let small = parse_stats(&small_xml, small_records);
        let large = parse_stats(&large_xml, large_records);
        // quick-xml's attribute/nil checks allocate for explicit units. Keep that
        // cost bounded; implicit-unit records, including accelerations, allocate no scratch.
        assert!(
            large.allocations
                <= small.allocations + allocations_per_record * (large_records - small_records),
            "XML parsing exceeded its allocation budget: small={small:?}, large={large:?}"
        );
    }
}
