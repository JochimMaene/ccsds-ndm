use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::acm::{Acm, AttLine};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::{GenerateOptions, VersionedNdm};
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn acm(records: usize) -> Acm {
    let mut message = Acm::from_kvn(include_str!("../data/kvn/acm_g7.kvn")).unwrap();
    let line = message.body.segment.data.att[0].att_lines[0].clone();
    message.body.segment.data.att[0].att_lines = vec![line; records];
    message
}

fn parse_stats(input: &str) -> Stats {
    let region = Region::new(GLOBAL);
    let message = Acm::from_kvn(black_box(input)).unwrap();
    let stats = region.change();
    black_box(message);
    stats
}

fn streaming_stats(message: &Acm, output_len: usize) -> Stats {
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
fn acm_history_parse_and_streaming_generation_have_bounded_allocation_growth() {
    let small = acm(10);
    let large = acm(1_000);
    let small_kvn = small.to_kvn().unwrap();
    let large_kvn = large.to_kvn().unwrap();

    let small_parse = parse_stats(&small_kvn);
    let large_parse = parse_stats(&large_kvn);
    assert!(
        large_parse.allocations <= small_parse.allocations + (1_000 - 10) + 12,
        "KVN parse exceeded one values allocation per record: small={small_parse:?}, large={large_parse:?}"
    );
    let typed_budget =
        (1_000 - 10) * (std::mem::size_of::<AttLine>() + 8 * std::mem::size_of::<f64>()) * 2
            + 8_192;
    assert!(
        large_parse.bytes_allocated <= small_parse.bytes_allocated + typed_budget,
        "KVN parse exceeded typed-history storage budget: small={small_parse:?}, large={large_parse:?}"
    );

    let small_stream = streaming_stats(&small, small_kvn.len());
    let large_stream = streaming_stats(&large, large_kvn.len());
    assert!(
        large_stream.allocations <= small_stream.allocations + 4,
        "streaming allocations scaled with records: small={small_stream:?}, large={large_stream:?}"
    );
    assert!(
        large_stream.bytes_allocated <= small_stream.bytes_allocated + 512,
        "streaming temporary storage scaled with records: small={small_stream:?}, large={large_stream:?}"
    );
}
