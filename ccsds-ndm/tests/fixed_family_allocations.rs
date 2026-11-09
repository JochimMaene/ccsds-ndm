use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::Ndm;
use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn measured<T>(
    input: &str,
    parse: impl Fn(&str) -> T,
    generate: impl Fn(&T, &mut Vec<u8>),
    output_len: usize,
) -> (Stats, Stats) {
    let parse_region = Region::new(GLOBAL);
    let message = parse(black_box(input));
    let parse_stats = parse_region.change();
    black_box(&message);

    let mut output = Vec::with_capacity(output_len);
    let generation_region = Region::new(GLOBAL);
    generate(black_box(&message), black_box(&mut output));
    let generation_stats = generation_region.change();
    assert_eq!(output.len(), output_len);
    black_box(output);
    (parse_stats, generation_stats)
}

fn budget<M: Ndm>(
    input: &str,
    (parse_allocs, parse_bytes): (usize, usize),
    (gen_allocs, gen_bytes): (usize, usize),
) {
    let output_len = M::from_kvn(input).unwrap().to_kvn().unwrap().len();
    let (parse, generation) = measured(
        input,
        |input| M::from_kvn(input).unwrap(),
        |message, output| message.write_kvn_to(output).unwrap(),
        output_len,
    );
    assert!(parse.allocations <= parse_allocs && parse.bytes_allocated <= parse_bytes);
    assert!(generation.allocations <= gen_allocs && generation.bytes_allocated <= gen_bytes);
}

#[test]
fn fixed_size_family_strict_paths_have_recorded_allocation_budgets() {
    // One test keeps the global allocation instrumentation isolated from concurrent regions.
    budget::<Omm>(
        include_str!("../data/kvn/omm_g7.kvn"),
        (128, 24_000),
        (48, 2_000),
    );
    budget::<Apm>(
        include_str!("../data/kvn/apm_g1.kvn"),
        (136, 24_000),
        (48, 2_000),
    );
    budget::<Rdm>(
        include_str!("../data/kvn/rdm_c1.kvn"),
        (96, 12_000),
        (40, 2_000),
    );
}
