use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::ParseOptions;
use ccsds_ndm::VersionedNdm;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn oem(records: usize) -> Oem {
    let mut message = Oem::from_kvn(include_str!("../data/kvn/oem_g11.kvn")).unwrap();
    message.body.segment.truncate(1);
    let state = message.body.segment[0].data.state_vector[0].clone();
    message.body.segment[0].data.state_vector = vec![state; records];
    message.body.segment[0].data.covariance_matrix.clear();
    message
}

fn streaming_stats(message: &Oem, output_len: usize) -> Stats {
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

fn parse_stats(input: &str) -> Stats {
    let region = Region::new(GLOBAL);
    let message = Oem::from_kvn(black_box(input)).unwrap();
    let stats = region.change();
    black_box(message);
    stats
}

fn covariance_oem(records: usize) -> Oem {
    let mut message = Oem::from_kvn(include_str!("../data/kvn/oem_g13.kvn")).unwrap();
    let mut covariance = message.body.segment[0].data.covariance_matrix[0].clone();
    covariance.cov_ref_frame = None;
    covariance.comment.clear();
    message.body.segment[0].data.covariance_matrix = (0..records)
        .map(|index| {
            let mut covariance = covariance.clone();
            covariance.epoch = format!("2019-12-28T21:{:02}:{:02}", 29 + index / 60, index % 60)
                .parse()
                .unwrap();
            covariance
        })
        .collect();
    message
}

fn many_small_segments(count: usize) -> String {
    let mut source = String::from(
        "CCSDS_OEM_VERS = 3.0\nCREATION_DATE = 2023-01-01T00:00:00\nORIGINATOR = TEST\n",
    );
    for day in 1..=count {
        source.push_str(&format!(
            "META_START\nOBJECT_NAME = SAT\nOBJECT_ID = 2023-001A\nCENTER_NAME = EARTH\nREF_FRAME = GCRF\nTIME_SYSTEM = UTC\nSTART_TIME = 2023-{day:03}T00:00:00\nSTOP_TIME = 2023-{day:03}T00:02:00\nMETA_STOP\n2023-{day:03}T00:00:00 1.0 2.0 3.0 4.0 5.0 6.0\n2023-{day:03}T00:01:00 1.0 2.0 3.0 4.0 5.0 6.0\n2023-{day:03}T00:02:00 1.0 2.0 3.0 4.0 5.0 6.0\n"
        ));
    }
    source
}

// Keep all cases in one test because the instrumented allocator is process-global.
#[test]
fn oem_kvn_generation_and_parsing_have_bounded_storage() {
    let small = oem(10);
    let large = oem(1_000);
    let small_len = small.to_kvn().unwrap().len();
    let large_len = large.to_kvn().unwrap().len();

    let small_stream = streaming_stats(&small, small_len);
    let large_stream = streaming_stats(&large, large_len);
    assert!(
        large_stream.allocations <= small_stream.allocations + 4,
        "streaming allocations scaled with record count: small={small_stream:?}, large={large_stream:?}"
    );
    assert!(
        large_stream.bytes_allocated <= small_stream.bytes_allocated + 512,
        "streaming temporary storage scaled with record count: small={small_stream:?}, large={large_stream:?}"
    );
    assert!(
        large_stream.reallocations <= small_stream.reallocations + 2,
        "streaming reallocations scaled with record count: small={small_stream:?}, large={large_stream:?}"
    );

    let region = Region::new(GLOBAL);
    let generated = black_box(&large).to_kvn().unwrap();
    let materialized = region.change();
    black_box(&generated);
    assert!(materialized.allocations <= 64, "{materialized:?}");
    assert!(
        materialized.bytes_allocated <= large_len * 2,
        "materialized temporary storage exceeded twice the output: {materialized:?}"
    );

    let source = many_small_segments(100);
    let parsed =
        Oem::from_kvn_with_options(&source, &ParseOptions::default().with_max_records(300))
            .unwrap();
    let records: usize = parsed
        .body
        .segment
        .iter()
        .map(|segment| segment.data.state_vector.len())
        .sum();
    let reserved: usize = parsed
        .body
        .segment
        .iter()
        .map(|segment| segment.data.state_vector.capacity())
        .sum();
    assert!(
        reserved <= records * 2,
        "small segments retained disproportionate state-vector capacity: records={records}, reserved={reserved}"
    );

    let small_covariance_kvn = covariance_oem(10).to_kvn().unwrap();
    let large_covariance_kvn = covariance_oem(1_000).to_kvn().unwrap();
    let small_covariance_parse = parse_stats(&small_covariance_kvn);
    let large_covariance_parse = parse_stats(&large_covariance_kvn);
    assert!(
        large_covariance_parse.allocations <= small_covariance_parse.allocations + 16,
        "covariance parsing allocated scratch vectors per matrix: small={small_covariance_parse:?}, large={large_covariance_parse:?}"
    );
}
