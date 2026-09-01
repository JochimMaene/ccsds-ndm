use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, MessageType};
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

fn combined(children: usize) -> CombinedNdm {
    let child = MessageType::Opm(Opm::from_kvn(OPM_KVN).unwrap());
    CombinedNdm {
        id: None,
        comments: vec!["allocation evidence".into()],
        messages: vec![child; children],
    }
}

fn streaming_stats(message: &CombinedNdm, output_len: usize) -> Stats {
    let mut output = Vec::with_capacity(output_len);
    let region = Region::new(GLOBAL);
    black_box(message)
        .write_kvn_to(
            black_box(&mut output),
            black_box(&GenerateOptions::source()),
        )
        .unwrap();
    let stats = region.change();
    assert_eq!(output.len(), output_len);
    black_box(output);
    stats
}

#[test]
fn combined_streaming_allocations_have_a_bounded_per_constituent_budget() {
    let small = combined(10);
    let large = combined(1_000);
    let small_len = small.to_kvn().unwrap().len();
    let large_len = large.to_kvn().unwrap().len();

    let small_stats = streaming_stats(&small, small_len);
    let large_stats = streaming_stats(&large, large_len);
    let additional_children = 1_000 - 10;
    assert!(
        large_stats.allocations <= small_stats.allocations + additional_children * 10 + 4,
        "streaming exceeded ten temporary allocations per constituent: small={small_stats:?}, large={large_stats:?}"
    );
    assert!(
        large_stats.bytes_allocated
            <= small_stats.bytes_allocated + additional_children * 128 + 512,
        "streaming exceeded 128 temporary bytes per constituent: small={small_stats:?}, large={large_stats:?}"
    );
}
