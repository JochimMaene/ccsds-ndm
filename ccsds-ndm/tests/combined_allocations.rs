use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::ndm::CombinedNdm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::Message;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const OPM_KVN: &str = include_str!("../data/kvn/opm_g1.kvn");

fn combined(children: usize) -> CombinedNdm {
    let child = Message::Opm(Opm::from_kvn(OPM_KVN).unwrap());
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
        .write_xml_to(black_box(&mut output))
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
    let small_len = small.to_xml().unwrap().len();
    let large_len = large.to_xml().unwrap().len();

    let small_stats = streaming_stats(&small, small_len);
    let large_stats = streaming_stats(&large, large_len);
    let additional_children = 1_000 - 10;
    // Measured on the normative XML envelope: 19 temporary allocations and ~6.3 KiB per
    // additional OPM constituent. The budget keeps headroom while still failing if streaming
    // stops being linear in the constituent count.
    assert!(
        large_stats.allocations <= small_stats.allocations + additional_children * 20 + 4,
        "streaming exceeded twenty temporary allocations per constituent: small={small_stats:?}, large={large_stats:?}"
    );
    assert!(
        large_stats.bytes_allocated
            <= small_stats.bytes_allocated + additional_children * 8_192 + 512,
        "streaming exceeded 8 KiB of temporaries per constituent: small={small_stats:?}, large={large_stats:?}"
    );
}
