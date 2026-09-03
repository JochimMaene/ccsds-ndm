// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! The XML ephemeris path must not allocate once per state vector.
//!
//! `Epoch` owns a fixed-size buffer specifically so that a large ephemeris does not put one heap
//! allocation on every record, and the XML structural walker keeps open element names in a stack
//! buffer for the same reason. Both are invisible to a correctness test, so they are pinned here.

use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn oem_xml(records: usize) -> String {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        "\n<oem id=\"CCSDS_OEM_VERS\" version=\"3.0\">\n",
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
fn oem_xml_parsing_does_not_allocate_per_state_vector() {
    let small_records = 100;
    let large_records = 2_000;
    let small_xml = oem_xml(small_records);
    let large_xml = oem_xml(large_records);

    // Warm any one-time global state before measuring.
    black_box(Oem::from_xml(&small_xml).unwrap());

    let small = parse_stats(&small_xml, small_records);
    let large = parse_stats(&large_xml, large_records);

    let extra_records = large_records - small_records;
    let extra_allocations = large.allocations.saturating_sub(small.allocations);
    // Growing the state-vector `Vec` is a reallocation, not an allocation, so parsing a longer
    // ephemeris must not raise the allocation count at all.
    assert_eq!(
        extra_allocations, 0,
        "XML parsing allocated per state vector: {extra_allocations} extra allocations for \
         {extra_records} extra records (small={small:?}, large={large:?})"
    );
}
