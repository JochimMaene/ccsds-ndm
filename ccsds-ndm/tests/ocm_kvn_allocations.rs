use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::ocm::{CovLine, ManLine, Ocm, TrajLine};
use ccsds_ndm::Ndm;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn ocm(records: usize) -> Ocm {
    let mut message = Ocm::from_kvn(include_str!("../data/kvn/ocm_g15.kvn")).unwrap();
    let line = message.body.segment.data.traj[0].traj_lines[0].clone();
    message.body.segment.data.traj[0].traj_lines = (0..records)
        .map(|index| {
            let mut record = line.clone();
            record.epoch = index.to_string().parse().unwrap();
            record
        })
        .collect();
    message
}

fn covariance_ocm(records: usize) -> Ocm {
    let mut message = Ocm::from_kvn(include_str!("../data/kvn/ocm_g19.kvn")).unwrap();
    let line = message.body.segment.data.cov[0].cov_lines[0].clone();
    message.body.segment.data.traj.truncate(1);
    message.body.segment.data.traj[0].traj_lines.truncate(1);
    message.body.segment.data.cov.truncate(1);
    message.body.segment.data.cov[0].cov_lines = (0..records)
        .map(|index| {
            let mut record = line.clone();
            record.epoch = index.to_string().parse().unwrap();
            record
        })
        .collect();
    message
}

fn maneuver_ocm(records: usize) -> Ocm {
    let mut message = Ocm::from_kvn(include_str!("../data/kvn/ocm_g18.kvn")).unwrap();
    let line = message.body.segment.data.man[0].man_lines[0].clone();
    message.body.segment.data.traj.truncate(1);
    message.body.segment.data.traj[0].traj_lines.truncate(1);
    message.body.segment.data.man.truncate(1);
    message.body.segment.data.man[0].man_lines = (0..records)
        .map(|index| {
            let mut record = line.clone();
            record.epoch = index.to_string().parse().unwrap();
            record
        })
        .collect();
    message
}

fn parse_stats(input: &str) -> Stats {
    let region = Region::new(GLOBAL);
    let message = Ocm::from_kvn(black_box(input)).unwrap();
    let stats = region.change();
    black_box(message);
    stats
}

fn streaming_stats(message: &Ocm, output_len: usize) -> Stats {
    let mut output = Vec::with_capacity(output_len);
    let region = Region::new(GLOBAL);
    black_box(message)
        .write_kvn_to(black_box(&mut output))
        .unwrap();
    let stats = region.change();
    assert_eq!(output.len(), output_len);
    black_box(output);
    stats
}

fn assert_streaming_is_record_independent(
    label: &str,
    small: &Ocm,
    large: &Ocm,
    small_kvn: &str,
    large_kvn: &str,
) {
    let small_stream = streaming_stats(small, small_kvn.len());
    let large_stream = streaming_stats(large, large_kvn.len());
    assert!(
        large_stream.allocations <= small_stream.allocations + 4,
        "{label} streaming allocations scaled with records: small={small_stream:?}, large={large_stream:?}"
    );
    assert!(
        large_stream.bytes_allocated <= small_stream.bytes_allocated + 512,
        "{label} streaming temporary storage scaled with records: small={small_stream:?}, large={large_stream:?}"
    );
}

// Keep all cases in one test because the instrumented allocator is process-global.
#[test]
fn ocm_history_parse_and_streaming_generation_have_bounded_allocation_growth() {
    let small = ocm(10);
    let large = ocm(1_000);
    let small_kvn = small.to_kvn().unwrap();
    let large_kvn = large.to_kvn().unwrap();

    let small_parse = parse_stats(&small_kvn);
    let large_parse = parse_stats(&large_kvn);
    // Each TrajLine owns one values vector; parser scratch allocation and reallocation must not
    // otherwise scale with the record count.
    assert!(
        large_parse.allocations <= small_parse.allocations + (1_000 - 10) + 12,
        "KVN parse exceeded one owned values allocation per record: small={small_parse:?}, large={large_parse:?}"
    );
    assert!(
        large_parse.reallocations <= small_parse.reallocations + 12,
        "KVN parse reallocated per trajectory record: small={small_parse:?}, large={large_parse:?}"
    );
    let typed_history_budget = (1_000 - 10) * std::mem::size_of::<TrajLine>() * 2 + 16_384;
    assert!(
        large_parse.bytes_allocated <= small_parse.bytes_allocated + typed_history_budget,
        "KVN parse exceeded typed-history storage budget: small={small_parse:?}, large={large_parse:?}"
    );

    assert_streaming_is_record_independent("trajectory", &small, &large, &small_kvn, &large_kvn);

    let region = Region::new(GLOBAL);
    let materialized = black_box(&large).to_kvn().unwrap();
    let materialized_stats = region.change();
    black_box(&materialized);
    assert!(
        materialized_stats.bytes_allocated <= large_kvn.len() * 2,
        "materialized storage exceeded twice the output: {materialized_stats:?}"
    );

    let small_covariance = covariance_ocm(10);
    let large_covariance = covariance_ocm(1_000);
    let small_covariance_kvn = small_covariance.to_kvn().unwrap();
    let large_covariance_kvn = large_covariance.to_kvn().unwrap();
    let small_covariance_parse = parse_stats(&small_covariance_kvn);
    let large_covariance_parse = parse_stats(&large_covariance_kvn);
    assert!(
        large_covariance_parse.allocations
            <= small_covariance_parse.allocations + (1_000 - 10) + 12,
        "covariance parse exceeded one owned values allocation per record: small={small_covariance_parse:?}, large={large_covariance_parse:?}"
    );
    assert!(
        large_covariance_parse.reallocations <= small_covariance_parse.reallocations + 12,
        "covariance parse reallocated per record: small={small_covariance_parse:?}, large={large_covariance_parse:?}"
    );
    let covariance_budget =
        (1_000 - 10) * std::mem::size_of::<CovLine>() * 4 + large_covariance_kvn.len();
    assert!(
        large_covariance_parse.bytes_allocated
            <= small_covariance_parse.bytes_allocated + covariance_budget,
        "covariance parse exceeded typed-history storage budget: small={small_covariance_parse:?}, large={large_covariance_parse:?}"
    );
    assert_streaming_is_record_independent(
        "covariance",
        &small_covariance,
        &large_covariance,
        &small_covariance_kvn,
        &large_covariance_kvn,
    );

    let small_maneuver = maneuver_ocm(10);
    let large_maneuver = maneuver_ocm(1_000);
    let small_maneuver_kvn = small_maneuver.to_kvn().unwrap();
    let large_maneuver_kvn = large_maneuver.to_kvn().unwrap();
    let small_maneuver_parse = parse_stats(&small_maneuver_kvn);
    let large_maneuver_parse = parse_stats(&large_maneuver_kvn);
    let values_per_record = large_maneuver.body.segment.data.man[0].man_lines[0]
        .values
        .len();
    assert!(
        large_maneuver_parse.allocations
            <= small_maneuver_parse.allocations
                + (1_000 - 10) * (values_per_record + 2)
                + 16,
        "maneuver parse exceeded owned string/vector allocations: small={small_maneuver_parse:?}, large={large_maneuver_parse:?}"
    );
    let maneuver_budget =
        (1_000 - 10) * std::mem::size_of::<ManLine>() * 4 + large_maneuver_kvn.len() * 2;
    assert!(
        large_maneuver_parse.bytes_allocated
            <= small_maneuver_parse.bytes_allocated + maneuver_budget,
        "maneuver parse exceeded typed-history storage budget: small={small_maneuver_parse:?}, large={large_maneuver_parse:?}"
    );
    assert_streaming_is_record_independent(
        "maneuver",
        &small_maneuver,
        &large_maneuver,
        &small_maneuver_kvn,
        &large_maneuver_kvn,
    );
}
