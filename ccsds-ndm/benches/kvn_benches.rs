// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! KVN parsing and generation benchmarks for all message types.
//!
//! `kvn_message_matrix` carries every standalone family, because the conformance files cite it
//! as each family's workload evidence. Dedicated groups beyond it exist only for the families
//! with a record history, where cost scales with the data.
//!
//! Groups are named `<family>_<notation>_<workload>`, except the cross-family
//! `kvn_message_matrix`.

mod common;

use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::types::CalendarEpoch;
use ccsds_ndm::Ndm;
use common::create_test_oem;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::str::FromStr;

fn bench_tdm_kvn_history_scaling(c: &mut Criterion) {
    let mut template = Tdm::from_kvn(include_str!("../data/kvn/tdm_e1.kvn")).unwrap();
    template.body.segments.truncate(1);
    let observation = template.body.segments[0].data.observations[0].clone();
    let mut group = c.benchmark_group("tdm_kvn_history_scaling");

    for records in [100, 10_000, 50_000] {
        let mut message = template.clone();
        message.body.segments[0].data.observations = vec![observation.clone(); records];
        let input = message.to_kvn().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| Tdm::from_kvn(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
}

fn bench_aem_kvn_history_scaling(c: &mut Criterion) {
    let template = Aem::from_kvn(include_str!("../data/kvn/aem_g5.kvn")).unwrap();
    let state = template.body.segment[0].data.attitude_states[0].clone();
    let mut group = c.benchmark_group("aem_kvn_history_scaling");

    for records in [100, 10_000, 50_000] {
        let mut message = template.clone();
        message.body.segment[0].metadata.stop_time =
            CalendarEpoch::from_str("2006-090T23:59:59.999").unwrap();
        message.body.segment[0].metadata.useable_stop_time =
            Some(CalendarEpoch::from_str("2006-090T23:59:59.999").unwrap());
        message.body.segment[0].data.attitude_states = (0..records)
            .map(|index| {
                let mut state = state.clone();
                let hours = 5 + index / 3_600;
                let minutes = (index % 3_600) / 60;
                let seconds = index % 60;
                let ccsds_ndm::common::AemAttitudeState::Spin(spin) = &mut state else {
                    unreachable!()
                };
                spin.epoch = CalendarEpoch::from_str(&format!(
                    "2006-090T{hours:02}:{minutes:02}:{seconds:02}.071"
                ))
                .unwrap();
                state
            })
            .collect();
        let input = message.to_kvn().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| Aem::from_kvn(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
}

fn bench_acm_kvn_history_scaling(c: &mut Criterion) {
    let template = Acm::from_kvn(include_str!("../data/kvn/acm_g7.kvn")).unwrap();
    let line = template.body.segment.data.att[0].att_lines[0].clone();
    let mut group = c.benchmark_group("acm_kvn_history_scaling");

    for records in [100, 10_000, 50_000] {
        let mut message = template.clone();
        message.body.segment.data.att[0].att_lines = vec![line.clone(); records];
        let input = message.to_kvn().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| Acm::from_kvn(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
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

// --- Scaling benchmarks ---

fn bench_oem_kvn_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("oem_kvn_scaling");

    for size in [10, 1_000, 50_000] {
        let oem = create_test_oem(size);
        let kvn_data = oem.to_kvn().unwrap();
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("parse", size), &kvn_data, |b, data| {
            b.iter(|| Oem::from_kvn(black_box(data)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("generate", size), &oem, |b, oem| {
            b.iter(|| black_box(oem).to_kvn().unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_tdm_kvn_history_scaling,
    bench_aem_kvn_history_scaling,
    bench_acm_kvn_history_scaling,
    bench_kvn_message_matrix,
    bench_oem_kvn_scaling,
);
criterion_main!(benches);
