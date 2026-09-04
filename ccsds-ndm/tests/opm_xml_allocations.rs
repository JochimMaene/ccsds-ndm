use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::VersionedNdm;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// Keep all allocation cases in this one test: the instrumented allocator is process-global.
#[test]
fn opm_xml_generation_has_bounded_allocations() {
    let opm = Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).unwrap();
    let expected_len = opm.to_xml().unwrap().len();

    let materialized_region = Region::new(GLOBAL);
    let materialized = black_box(&opm).to_xml().unwrap();
    let materialized_stats = materialized_region.change();
    black_box(&materialized);

    let mut streamed = Vec::with_capacity(expected_len);
    let streaming_region = Region::new(GLOBAL);
    black_box(&opm)
        .write_xml_to(black_box(&mut streamed))
        .unwrap();
    let streaming_stats = streaming_region.change();
    black_box(&streamed);

    assert!(
        materialized_stats.allocations <= 60 && materialized_stats.bytes_allocated <= 35_000,
        "materialized XML allocation budget regressed for {expected_len} output bytes: {materialized_stats:?}"
    );
    assert!(
        streaming_stats.allocations <= 60 && streaming_stats.bytes_allocated <= 30_000,
        "streaming XML allocation budget regressed: {streaming_stats:?}"
    );
}
