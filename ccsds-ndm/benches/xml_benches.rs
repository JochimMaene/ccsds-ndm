// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! XML parsing and generation benchmarks for all message types.

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::messages::ocm::{
    CovLine, ManLine, Ocm, OcmBody, OcmCovarianceMatrix, OcmData, OcmManeuverParameters,
    OcmMetadata, OcmSegment, OcmTrajState, TrajLine,
};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::types::{
    CalendarEpoch, Epoch, InterpolationDegree, Position, PositionUnits, Velocity, VelocityUnits,
};
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
            creation_date: CalendarEpoch::from_str("2023-09-26T12:00:00Z").unwrap(),
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
                    interpolation_degree: NonZeroU32::new(5).map(InterpolationDegree),
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

/// Create a test OCM with one CARTPV trajectory block of the requested size.
fn create_test_ocm(num_states: usize) -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut traj_lines = Vec::with_capacity(num_states);
    for i in 0..num_states {
        let line_epoch = Epoch::from_str(&format!(
            "2023-09-26T{:02}:{:02}:{:02}.123456Z",
            i / 3600,
            (i / 60) % 60,
            i % 60
        ))
        .unwrap();
        traj_lines.push(TrajLine {
            epoch: line_epoch,
            values: vec![7000.0 + i as f64, 0.0, 0.0, 0.0, 7.5, 0.0],
        });
    }

    Ocm::builder()
        .header(
            OdmHeader::builder()
                .creation_date(epoch.as_str().parse().unwrap())
                .originator("BENCH")
                .build(),
        )
        .body(
            OcmBody::builder()
                .segment(Box::new(
                    OcmSegment::builder()
                        .metadata(
                            OcmMetadata::builder()
                                .time_system("UTC")
                                .epoch_tzero(epoch.as_str().parse().unwrap())
                                .build(),
                        )
                        .data(OcmData {
                            traj: vec![OcmTrajState::builder()
                                .center_name("EARTH")
                                .traj_ref_frame("GCRF")
                                .traj_type("CARTPV")
                                .traj_lines(traj_lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
}

/// Create a test OCM with one CARTPV covariance history of the requested size.
fn create_test_ocm_covariance(num_states: usize) -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut cov_lines = Vec::with_capacity(num_states);
    for i in 0..num_states {
        let line_epoch = Epoch::from_str(&format!(
            "2023-09-26T{:02}:{:02}:{:02}.123456Z",
            i / 3600,
            (i / 60) % 60,
            i % 60
        ))
        .unwrap();
        cov_lines.push(CovLine {
            epoch: line_epoch,
            values: vec![1.0 + i as f64, 0.0, 0.0, 0.0, 0.0, 1.0],
        });
    }

    Ocm::builder()
        .header(
            OdmHeader::builder()
                .creation_date(epoch.as_str().parse().unwrap())
                .originator("BENCH")
                .build(),
        )
        .body(
            OcmBody::builder()
                .segment(Box::new(
                    OcmSegment::builder()
                        .metadata(
                            OcmMetadata::builder()
                                .time_system("UTC")
                                .epoch_tzero(epoch.as_str().parse().unwrap())
                                .build(),
                        )
                        .data(OcmData {
                            cov: vec![OcmCovarianceMatrix::builder()
                                .cov_ref_frame("GCRF")
                                .cov_type("CARTPV")
                                .cov_lines(cov_lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
}

/// Create a test OCM with one relative-time maneuver history of the requested size.
fn create_test_ocm_maneuver(num_states: usize) -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut man_lines = Vec::with_capacity(num_states);
    for i in 0..num_states {
        let line_epoch = Epoch::from_str(&format!("{}.5", i)).unwrap();
        man_lines.push(ManLine {
            epoch: line_epoch,
            values: vec![
                format!("{}", 100.0 + i as f64),
                "0.0".to_string(),
                "10.0".to_string(),
            ],
        });
    }

    Ocm::builder()
        .header(
            OdmHeader::builder()
                .creation_date(epoch.as_str().parse().unwrap())
                .originator("BENCH")
                .build(),
        )
        .body(
            OcmBody::builder()
                .segment(Box::new(
                    OcmSegment::builder()
                        .metadata(
                            OcmMetadata::builder()
                                .time_system("UTC")
                                .epoch_tzero(epoch.as_str().parse().unwrap())
                                .build(),
                        )
                        .data(OcmData {
                            man: vec![OcmManeuverParameters::builder()
                                .man_id("MAN-1")
                                .man_device_id("THR-1")
                                .man_ref_frame("GCRF")
                                .man_composition("TIME_RELATIVE, MAN_DURA, THR_X, THR_Y")
                                .man_lines(man_lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
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

/// Baseline the format-independent validation walk used before OEM generation.
///
/// Epoch construction is intentionally outside this benchmark. A validated epoch
/// representation should not be reparsed while walking a large OEM history.
fn bench_validate_oem(c: &mut Criterion) {
    let oem = create_test_oem(10000);

    c.bench_function("validate_oem_10k", |b| {
        b.iter(|| black_box(&oem).validate().unwrap())
    });
}

fn bench_ocm_trajectory_10k(c: &mut Criterion) {
    let ocm = create_test_ocm(10000);
    let xml = ocm.to_xml().unwrap();
    let kvn = ocm.to_kvn().unwrap();
    let mut group = c.benchmark_group("ocm_trajectory_10k");

    group.bench_function("xml_parse", |b| {
        b.iter(|| Ocm::from_xml(black_box(&xml)).unwrap())
    });
    group.bench_function("xml_generate", |b| {
        b.iter(|| black_box(&ocm).to_xml().unwrap())
    });
    group.bench_function("kvn_parse", |b| {
        b.iter(|| Ocm::from_kvn(black_box(&kvn)).unwrap())
    });
    group.bench_function("kvn_generate", |b| {
        b.iter(|| black_box(&ocm).to_kvn().unwrap())
    });
    group.bench_function("validate", |b| {
        b.iter(|| black_box(&ocm).validate().unwrap())
    });

    group.finish();
}

fn bench_ocm_covariance_10k(c: &mut Criterion) {
    let ocm = create_test_ocm_covariance(10000);
    let xml = ocm.to_xml().unwrap();
    let kvn = ocm.to_kvn().unwrap();
    let mut group = c.benchmark_group("ocm_covariance_10k");

    group.bench_function("xml_parse", |b| {
        b.iter(|| Ocm::from_xml(black_box(&xml)).unwrap())
    });
    group.bench_function("xml_generate", |b| {
        b.iter(|| black_box(&ocm).to_xml().unwrap())
    });
    group.bench_function("kvn_parse", |b| {
        b.iter(|| Ocm::from_kvn(black_box(&kvn)).unwrap())
    });
    group.bench_function("kvn_generate", |b| {
        b.iter(|| black_box(&ocm).to_kvn().unwrap())
    });
    group.bench_function("validate", |b| {
        b.iter(|| black_box(&ocm).validate().unwrap())
    });

    group.finish();
}

fn bench_ocm_maneuver_10k(c: &mut Criterion) {
    let ocm = create_test_ocm_maneuver(10000);
    let xml = ocm.to_xml().unwrap();
    let kvn = ocm.to_kvn().unwrap();
    let mut group = c.benchmark_group("ocm_maneuver_10k");

    group.bench_function("xml_parse", |b| {
        b.iter(|| Ocm::from_xml(black_box(&xml)).unwrap())
    });
    group.bench_function("xml_generate", |b| {
        b.iter(|| black_box(&ocm).to_xml().unwrap())
    });
    group.bench_function("kvn_parse", |b| {
        b.iter(|| Ocm::from_kvn(black_box(&kvn)).unwrap())
    });
    group.bench_function("kvn_generate", |b| {
        b.iter(|| black_box(&ocm).to_kvn().unwrap())
    });
    group.bench_function("validate", |b| {
        b.iter(|| black_box(&ocm).validate().unwrap())
    });

    group.finish();
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
    bench_validate_oem,
    bench_ocm_trajectory_10k,
    bench_ocm_covariance_10k,
    bench_ocm_maneuver_10k,
    bench_xml_parse_opm,
    bench_xml_parse_omm,
    bench_xml_parse_tdm,
    bench_xml_scaling,
);
criterion_main!(benches);
