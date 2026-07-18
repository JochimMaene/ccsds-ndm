use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::aem::{Aem, AemAttitudeStateWrapper};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::CalendarEpoch;
use ccsds_ndm::{GenerateOptions, VersionedNdm};
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn aem(records: usize) -> Aem {
    let mut message = Aem::from_kvn(include_str!("../../data/kvn/aem_g5.kvn")).unwrap();
    let state = message.body.segment[0].data.attitude_states[0].clone();
    message.body.segment[0].metadata.stop_time =
        "2006-090T23:59:59.999".parse::<CalendarEpoch>().unwrap();
    message.body.segment[0].metadata.useable_stop_time =
        Some("2006-090T23:59:59.999".parse::<CalendarEpoch>().unwrap());
    message.body.segment[0].data.attitude_states = (0..records)
        .map(|index| {
            let mut state = state.clone();
            let hours = 5 + index / 3_600;
            let minutes = (index % 3_600) / 60;
            let seconds = index % 60;
            state.spin.as_mut().unwrap().epoch =
                format!("2006-090T{hours:02}:{minutes:02}:{seconds:02}.071")
                    .parse()
                    .unwrap();
            state
        })
        .collect();
    message
}

fn parse_stats(input: &str) -> Stats {
    let region = Region::new(GLOBAL);
    let message = Aem::from_kvn(black_box(input)).unwrap();
    let stats = region.change();
    black_box(message);
    stats
}

fn streaming_stats(message: &Aem, output_len: usize) -> Stats {
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
fn aem_history_parse_and_streaming_generation_have_bounded_allocation_growth() {
    let small = aem(10);
    let large = aem(1_000);
    let small_kvn = small.to_kvn().unwrap();
    let large_kvn = large.to_kvn().unwrap();

    let small_parse = parse_stats(&small_kvn);
    let large_parse = parse_stats(&large_kvn);
    assert!(
        large_parse.allocations <= small_parse.allocations + (1_000 - 10) + 12,
        "KVN parse exceeded one owned epoch allocation per state: small={small_parse:?}, large={large_parse:?}"
    );
    let typed_budget = (1_000 - 10) * std::mem::size_of::<AemAttitudeStateWrapper>() * 2 + 8_192;
    assert!(
        large_parse.bytes_allocated <= small_parse.bytes_allocated + typed_budget,
        "KVN parse exceeded typed-state storage budget: small={small_parse:?}, large={large_parse:?}"
    );

    let small_stream = streaming_stats(&small, small_kvn.len());
    let large_stream = streaming_stats(&large, large_kvn.len());
    assert!(
        large_stream.allocations <= small_stream.allocations + 4,
        "streaming allocations scaled with states: small={small_stream:?}, large={large_stream:?}"
    );
    assert!(
        large_stream.bytes_allocated <= small_stream.bytes_allocated + 512,
        "streaming temporary storage scaled with states: small={small_stream:?}, large={large_stream:?}"
    );

    let region = Region::new(GLOBAL);
    let materialized = black_box(&large).to_kvn().unwrap();
    let materialized_stats = region.change();
    black_box(&materialized);
    assert!(
        materialized_stats.bytes_allocated <= large_kvn.len() * 2,
        "materialized storage exceeded twice the output: {materialized_stats:?}"
    );
}
