// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::Ndm;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// Keep all allocation cases in this one test: the instrumented allocator is process-global.
#[test]
fn opm_kvn_generation_has_bounded_allocations() {
    let opm = Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).unwrap();
    let expected_len = opm.to_kvn().unwrap().len();

    let materialized_region = Region::new(GLOBAL);
    let materialized = black_box(&opm).to_kvn().unwrap();
    let materialized_stats = materialized_region.change();
    black_box(&materialized);

    let mut streamed = Vec::with_capacity(expected_len);
    let streaming_region = Region::new(GLOBAL);
    black_box(&opm)
        .write_kvn_to(black_box(&mut streamed))
        .unwrap();
    let streaming_stats = streaming_region.change();
    black_box(&streamed);

    assert!(
        materialized_stats.allocations <= 16,
        "materialized allocation count exceeded its budget: {materialized_stats:?}"
    );
    assert!(
        materialized_stats.reallocations <= 10,
        "materialized reallocation count exceeded its budget: {materialized_stats:?}"
    );
    assert!(
        materialized_stats.bytes_allocated <= expected_len * 2,
        "materialized allocated bytes exceeded twice the output size: {materialized_stats:?}"
    );
    assert!(
        materialized_stats.bytes_reallocated <= (expected_len * 2) as isize,
        "materialized reallocated bytes exceeded twice the output size: {materialized_stats:?}"
    );

    assert!(
        streaming_stats.allocations <= 16,
        "streaming allocation count exceeded its budget: {streaming_stats:?}"
    );
    assert!(
        streaming_stats.reallocations <= 4,
        "streaming reallocation count exceeded its budget: {streaming_stats:?}"
    );
    assert!(
        streaming_stats.bytes_allocated <= 512,
        "streaming allocated bytes exceeded its budget: {streaming_stats:?}"
    );
    assert!(
        streaming_stats.bytes_reallocated <= 128,
        "streaming reallocated bytes exceeded its budget: {streaming_stats:?}"
    );

    // Repeated blocks report indexed diagnostic paths, which must stay unbuilt while validation
    // is finding nothing wrong.
    let maneuvers = Opm::from_kvn(include_str!("../data/kvn/opm_g2.kvn")).unwrap();
    assert_eq!(maneuvers.body.segment.data.maneuver_parameters.len(), 2);
    let _ = maneuvers.to_kvn().unwrap();
    let maneuver_region = Region::new(GLOBAL);
    let generated = black_box(&maneuvers).to_kvn().unwrap();
    let maneuver_stats = maneuver_region.change();
    black_box(&generated);
    assert!(
        maneuver_stats.allocations <= 16,
        "maneuver allocation count exceeded its budget: {maneuver_stats:?}"
    );
}
