// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! KVN parsing and generation benchmarks for all message types.
//!
//! `kvn_message_matrix` carries every standalone family, because the conformance files cite it
//! as each family's workload evidence. Beyond it, each family with a record history gets one
//! `<family>_kvn_history_scaling` group over [`common::KVN_HISTORY_SIZES`]; `xml_benches.rs`
//! registers the same families, at sizes chosen so every XML point has a KVN counterpart.

mod common;

use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;
use common::{
    create_test_acm, create_test_aem, create_test_ocm, create_test_ocm_covariance,
    create_test_ocm_maneuver, create_test_oem, create_test_tdm, KVN_HISTORY_SIZES,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

/// Register KVN parse and generation for one family across every history size.
fn bench_kvn_history_scaling<M, B>(c: &mut Criterion, name: &str, build: B)
where
    M: Ndm + PartialEq,
    B: Fn(usize) -> M,
{
    let mut group = c.benchmark_group(name);
    for records in KVN_HISTORY_SIZES {
        let message = build(records);
        let input = message.to_kvn().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| M::from_kvn(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
}

fn bench_oem_kvn_history_scaling(c: &mut Criterion) {
    bench_kvn_history_scaling::<Oem, _>(c, "oem_kvn_history_scaling", create_test_oem);
}

fn bench_tdm_kvn_history_scaling(c: &mut Criterion) {
    bench_kvn_history_scaling::<Tdm, _>(c, "tdm_kvn_history_scaling", create_test_tdm);
}

fn bench_aem_kvn_history_scaling(c: &mut Criterion) {
    bench_kvn_history_scaling::<Aem, _>(c, "aem_kvn_history_scaling", create_test_aem);
}

fn bench_acm_kvn_history_scaling(c: &mut Criterion) {
    bench_kvn_history_scaling::<Acm, _>(c, "acm_kvn_history_scaling", create_test_acm);
}

/// OCM carries three independent histories, each with its own line grammar.
fn bench_ocm_kvn_history_scaling(c: &mut Criterion) {
    bench_kvn_history_scaling::<Ocm, _>(c, "ocm_trajectory_kvn_history_scaling", create_test_ocm);
    bench_kvn_history_scaling::<Ocm, _>(
        c,
        "ocm_covariance_kvn_history_scaling",
        create_test_ocm_covariance,
    );
    bench_kvn_history_scaling::<Ocm, _>(
        c,
        "ocm_maneuver_kvn_history_scaling",
        create_test_ocm_maneuver,
    );
}

fn bench_kvn_message_matrix(c: &mut Criterion) {
    let cases = [
        ("opm", include_str!("../data/kvn/opm_g4.kvn")),
        ("omm", include_str!("../data/kvn/omm_g7.kvn")),
        ("oem", include_str!("../data/kvn/oem_g11.kvn")),
        ("ocm", include_str!("../data/kvn/ocm_g15.kvn")),
        ("cdm", include_str!("../data/kvn/cdm_362.kvn")),
        ("tdm", include_str!("../data/kvn/tdm_e1.kvn")),
        ("rdm", include_str!("../data/kvn/rdm_c1.kvn")),
        ("aem", include_str!("../data/kvn/aem_g4.kvn")),
        ("apm", include_str!("../data/kvn/apm_g1.kvn")),
        ("acm", include_str!("../data/kvn/acm_g6.kvn")),
    ];
    let mut group = c.benchmark_group("kvn_message_matrix");
    for (name, input) in cases {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("parse", name), input, |b, input| {
            b.iter(|| ccsds_ndm::from_str(black_box(input)).unwrap())
        });

        // Generation writes a different number of bytes than it read, so its throughput is
        // measured against the output it actually produces.
        let message = ccsds_ndm::from_str(input).unwrap();
        group.throughput(Throughput::Bytes(message.to_kvn().unwrap().len() as u64));
        group.bench_with_input(
            BenchmarkId::new("generate", name),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_kvn_message_matrix,
    bench_oem_kvn_history_scaling,
    bench_ocm_kvn_history_scaling,
    bench_tdm_kvn_history_scaling,
    bench_aem_kvn_history_scaling,
    bench_acm_kvn_history_scaling,
);
criterion_main!(benches);
