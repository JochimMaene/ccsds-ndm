// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! XML parsing and generation benchmarks for all message types.

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

/// Create a test OEM with the given number of state vectors.
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

fn bench_xml_parse_oem(c: &mut Criterion) {
    let oem = create_test_oem(10000);
    let xml_data = oem.to_xml().unwrap();

    c.bench_function("xml_parse_oem_10k", |b| {
        b.iter(|| Oem::from_xml(black_box(&xml_data)).unwrap())
    });
}

fn bench_xml_generate_oem(c: &mut Criterion) {
    let oem = create_test_oem(10000);

    c.bench_function("xml_generate_oem_10k", |b| {
        b.iter(|| black_box(&oem).to_xml().unwrap())
    });
}

fn bench_xml_parse_opm(c: &mut Criterion) {
    let opm_xml = include_str!("../../data/xml/opm_g5.xml");

    c.bench_function("xml_parse_opm", |b| {
        b.iter(|| Opm::from_xml(black_box(opm_xml)).unwrap())
    });
}

fn bench_xml_parse_omm(c: &mut Criterion) {
    let omm_xml = include_str!("../../data/xml/omm_g10.xml");

    c.bench_function("xml_parse_omm", |b| {
        b.iter(|| Omm::from_xml(black_box(omm_xml)).unwrap())
    });
}

fn bench_xml_parse_tdm(c: &mut Criterion) {
    let tdm_xml = include_str!("../../data/xml/tdm_e21.xml");

    c.bench_function("xml_parse_tdm", |b| {
        b.iter(|| Tdm::from_xml(black_box(tdm_xml)).unwrap())
    });
}

fn bench_xml_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_scaling");

    for size in [100, 1000, 10000] {
        let oem = create_test_oem(size);
        let xml_data = oem.to_xml().unwrap();

        group.bench_with_input(BenchmarkId::new("parse", size), &xml_data, |b, data| {
            b.iter(|| Oem::from_xml(black_box(data)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("generate", size), &oem, |b, oem| {
            b.iter(|| black_box(oem).to_xml().unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_xml_parse_oem,
    bench_xml_generate_oem,
    bench_xml_parse_opm,
    bench_xml_parse_omm,
    bench_xml_parse_tdm,
    bench_xml_scaling,
);
criterion_main!(benches);
