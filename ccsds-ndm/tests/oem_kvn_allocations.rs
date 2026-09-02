use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::traits::Ndm;
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

// Keep all cases in one test because the instrumented allocator is process-global.
#[test]
fn validated_kvn_generation_has_fixed_overhead_and_output_proportional_storage() {
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
}
