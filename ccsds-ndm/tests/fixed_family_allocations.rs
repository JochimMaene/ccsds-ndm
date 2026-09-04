use std::alloc::System;
use std::hint::black_box;

use ccsds_ndm::messages::apm::Apm;
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::VersionedNdm;
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

fn omm_budget() {
    let input = include_str!("../data/kvn/omm_g7.kvn");
    let output_len = Omm::from_kvn(input).unwrap().to_kvn().unwrap().len();
    let (parse, generation) = measured(
        input,
        |input| Omm::from_kvn(input).unwrap(),
        |message, output| message.write_kvn_to(output).unwrap(),
        output_len,
    );
    assert!(parse.allocations <= 128 && parse.bytes_allocated <= 24_000);
    assert!(generation.allocations <= 48 && generation.bytes_allocated <= 2_000);
}

fn apm_budget() {
    let input = include_str!("../data/kvn/apm_g1.kvn");
    let output_len = Apm::from_kvn(input).unwrap().to_kvn().unwrap().len();
    let (parse, generation) = measured(
        input,
        |input| Apm::from_kvn(input).unwrap(),
        |message, output| message.write_kvn_to(output).unwrap(),
        output_len,
    );
    assert!(parse.allocations <= 136 && parse.bytes_allocated <= 24_000);
    assert!(generation.allocations <= 48 && generation.bytes_allocated <= 2_000);
}

fn rdm_budget() {
    let input = include_str!("../data/kvn/rdm_c1.kvn");
    let output_len = Rdm::from_kvn(input).unwrap().to_kvn().unwrap().len();
    let (parse, generation) = measured(
        input,
        |input| Rdm::from_kvn(input).unwrap(),
        |message, output| message.write_kvn_to(output).unwrap(),
        output_len,
    );
    assert!(parse.allocations <= 96 && parse.bytes_allocated <= 12_000);
    assert!(generation.allocations <= 40 && generation.bytes_allocated <= 2_000);
}

#[test]
fn fixed_size_family_strict_paths_have_recorded_allocation_budgets() {
    // One test keeps the global allocation instrumentation isolated from concurrent regions.
    omm_budget();
    apm_budget();
    rdm_budget();
}
