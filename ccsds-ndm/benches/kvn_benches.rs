// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! KVN parsing and generation benchmarks for all message types.

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Epoch, Position, PositionUnits, Velocity, VelocityUnits};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::num::NonZeroU32;
use std::str::FromStr;

fn create_test_oem(num_states: usize) -> Oem {
    let mut state_vectors = Vec::with_capacity(num_states);
    for i in 0..num_states {
        state_vectors.push(StateVectorAcc {
            epoch: Epoch::from_str("2023-09-26T12:00:00Z").unwrap(),
            x: Position {
                units: Some(PositionUnits::Km),
                value: 7000.0 + i as f64,
            },
            y: Position {
                units: Some(PositionUnits::Km),
                value: 0.0,
            },
            z: Position {
                units: Some(PositionUnits::Km),
                value: 0.0,
            },
            x_dot: Velocity {
                units: Some(VelocityUnits::KmPerS),
                value: 0.0,
            },
            y_dot: Velocity {
                units: Some(VelocityUnits::KmPerS),
                value: 7.5,
            },
            z_dot: Velocity {
                units: Some(VelocityUnits::KmPerS),
                value: 0.0,
            },
            x_ddot: None,
            y_ddot: None,
            z_ddot: None,
        });
    }

    Oem {
        id: Some("CCSDS_OEM_VERS".to_string()),
        version: "3.0".to_string(),
        header: OdmHeader {
            comment: vec!["This is a header comment.".to_string()],
            classification: None,
            creation_date: Epoch::from_str("2023-09-26T12:00:00Z").unwrap(),
            originator: "NASA/JPL".to_string(),
            message_id: None,
        },
        body: OemBody {
            segment: vec![OemSegment {
                metadata: OemMetadata {
                    comment: vec![],
                    object_name: "SATELLITE".to_string(),
                    object_id: "12345".to_string(),
                    center_name: "EARTH".to_string(),
                    ref_frame: "GCRF".to_string(),
                    ref_frame_epoch: None,
                    time_system: "UTC".to_string(),
                    start_time: Epoch::from_str("2023-09-26T12:00:00Z").unwrap(),
                    useable_start_time: None,
                    useable_stop_time: None,
                    stop_time: Epoch::from_str("2023-09-26T12:02:00Z").unwrap(),
                    interpolation: Some("LAGRANGE".to_string()),
                    interpolation_degree: NonZeroU32::new(5),
                },
                data: OemData {
                    comment: vec![],
                    state_vector: state_vectors,
                    covariance_matrix: vec![],
                },
            }],
        },
    }
}

// --- Core OEM benchmarks (original) ---

fn bench_parse_kvn(c: &mut Criterion) {
    let oem = create_test_oem(50000);
    let kvn_data = oem.to_kvn().unwrap();

    c.bench_function("kvn_parse", |b| {
        b.iter(|| Oem::from_kvn(black_box(&kvn_data)).unwrap())
    });
}

fn bench_generate_kvn(c: &mut Criterion) {
    let oem = create_test_oem(50000);

    c.bench_function("kvn_generate", |b| {
        b.iter(|| black_box(&oem).to_kvn().unwrap())
    });
}

// --- Multi-message-type benchmarks ---

fn bench_parse_opm(c: &mut Criterion) {
    let opm_kvn = include_str!("../../data/kvn/opm_g1.kvn");

    c.bench_function("kvn_parse_opm", |b| {
        b.iter(|| Opm::from_kvn(black_box(opm_kvn)).unwrap())
    });
}

fn bench_parse_omm(c: &mut Criterion) {
    let omm_kvn = include_str!("../../data/kvn/omm_g7.kvn");

    c.bench_function("kvn_parse_omm", |b| {
        b.iter(|| Omm::from_kvn(black_box(omm_kvn)).unwrap())
    });
}

fn bench_parse_tdm(c: &mut Criterion) {
    let tdm_kvn = include_str!("../../data/kvn/tdm_e1.kvn");

    c.bench_function("kvn_parse_tdm", |b| {
        b.iter(|| Tdm::from_kvn(black_box(tdm_kvn)).unwrap())
    });
}

// --- Scaling benchmarks ---

fn bench_kvn_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("kvn_scaling");

    for size in [10, 100, 1000, 10000, 50000] {
        let oem = create_test_oem(size);
        let kvn_data = oem.to_kvn().unwrap();

        group.bench_with_input(BenchmarkId::new("parse", size), &kvn_data, |b, data| {
            b.iter(|| Oem::from_kvn(black_box(data)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("generate", size), &oem, |b, oem| {
            b.iter(|| black_box(oem).to_kvn().unwrap())
        });
    }

    group.finish();
}

// --- Micro-benchmarks for hot paths ---

fn bench_micro(c: &mut Criterion) {
    // Epoch parsing - used in every message
    c.bench_function("micro_epoch_parse", |b| {
        b.iter(|| Epoch::from_str(black_box("2023-09-26T12:00:00.123456Z")).unwrap())
    });

    // Float parsing - the core of data parsing
    c.bench_function("micro_float_parse", |b| {
        b.iter(|| fast_float::parse::<f64, _>(black_box("32021034790.7265")).unwrap())
    });
}

criterion_group!(
    benches,
    bench_parse_kvn,
    bench_generate_kvn,
    bench_parse_opm,
    bench_parse_omm,
    bench_parse_tdm,
    bench_kvn_scaling,
    bench_micro,
);
criterion_main!(benches);
