// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! XML parsing and generation benchmarks for all message types.
//!
//! Only the families that carry a record history are benchmarked: a flat message parses and
//! generates in microseconds, where measurement noise dominates the result.
//!
//! Groups are named `<family>_<notation>_<workload>`, except the cross-family
//! `xml_message_matrix`. The three `ocm_*_10k` groups compare both notations on one large
//! message, so they carry the notation on each case instead of in the group name.

mod common;

use ccsds_ndm::common::OdmHeader;
use ccsds_ndm::messages::ocm::{
    CovLine, ManLine, Ocm, OcmBody, OcmCovarianceMatrix, OcmData, OcmManeuverParameters,
    OcmMetadata, OcmSegment, OcmTrajState, TrajLine,
};
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::types::Epoch;
use ccsds_ndm::Ndm;
use common::create_test_oem;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::str::FromStr;

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

/// Create a test OCM with one CARTP covariance history of the requested size.
///
/// Six values per line is the CARTP lower triangle; CARTPV would carry twenty-one.
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
                                .cov_type("CARTP")
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

/// Register parse and generation in both notations against one OCM history.
///
/// The trajectory, covariance and maneuver benchmarks differ only in the message they build.
fn bench_ocm_operations(c: &mut Criterion, name: &str, records: usize, ocm: &Ocm) {
    let xml = ocm.to_xml().unwrap();
    let kvn = ocm.to_kvn().unwrap();
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(records as u64));

    group.bench_function("xml_parse", |b| {
        b.iter(|| Ocm::from_xml(black_box(&xml)).unwrap())
    });
    group.bench_function("xml_generate", |b| {
        b.iter(|| black_box(ocm).to_xml().unwrap())
    });
    group.bench_function("kvn_parse", |b| {
        b.iter(|| Ocm::from_kvn(black_box(&kvn)).unwrap())
    });
    group.bench_function("kvn_generate", |b| {
        b.iter(|| black_box(ocm).to_kvn().unwrap())
    });

    group.finish();
}

fn bench_ocm_trajectory_10k(c: &mut Criterion) {
    bench_ocm_operations(c, "ocm_trajectory_10k", 10_000, &create_test_ocm(10_000));
}

fn bench_ocm_covariance_10k(c: &mut Criterion) {
    bench_ocm_operations(
        c,
        "ocm_covariance_10k",
        10_000,
        &create_test_ocm_covariance(10_000),
    );
}

fn bench_ocm_maneuver_10k(c: &mut Criterion) {
    bench_ocm_operations(
        c,
        "ocm_maneuver_10k",
        10_000,
        &create_test_ocm_maneuver(10_000),
    );
}

fn bench_tdm_xml_history_scaling(c: &mut Criterion) {
    let mut template = Tdm::from_xml(include_str!("../data/xml/tdm_e21.xml")).unwrap();
    template.body.segments.truncate(1);
    let observation = template.body.segments[0].data.observations[0].clone();
    let mut group = c.benchmark_group("tdm_xml_history_scaling");

    for records in [100, 1_000, 10_000] {
        let mut message = template.clone();
        message.body.segments[0].data.observations = vec![observation.clone(); records];
        let input = message.to_xml().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| Tdm::from_xml(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_xml().unwrap()),
        );
    }
    group.finish();
}

fn bench_xml_message_matrix(c: &mut Criterion) {
    let acm = ccsds_ndm::from_str(include_str!("../data/kvn/acm_g6.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let cases = vec![
        ("oem", include_str!("../data/xml/oem_g14.xml").to_owned()),
        ("ocm", include_str!("../data/xml/ocm_g20.xml").to_owned()),
        ("tdm", include_str!("../data/xml/tdm_e21.xml").to_owned()),
        ("aem", include_str!("../data/xml/aem_g11.xml").to_owned()),
        ("acm", acm),
        // The combined fixture holds an APM, an AEM and an ACM, so it reaches history parsing
        // through the envelope.
        ("ndm", include_str!("../data/xml/ndm_g12.xml").to_owned()),
    ];
    let mut group = c.benchmark_group("xml_message_matrix");
    for (name, input) in &cases {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("parse", name), input, |b, input| {
            b.iter(|| ccsds_ndm::from_str(black_box(input)).unwrap())
        });

        // Generation writes a different number of bytes than it read, so its throughput is
        // measured against the output it actually produces.
        let message = ccsds_ndm::from_str(input).unwrap();
        group.throughput(Throughput::Bytes(message.to_xml().unwrap().len() as u64));
        group.bench_with_input(
            BenchmarkId::new("generate", name),
            &message,
            |b, message| b.iter(|| black_box(message).to_xml().unwrap()),
        );
    }
    group.finish();
}

fn bench_oem_xml_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("oem_xml_scaling");

    for size in [100, 1000, 10000] {
        let oem = create_test_oem(size);
        let xml_data = oem.to_xml().unwrap();
        group.throughput(Throughput::Elements(size as u64));

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
    bench_ocm_trajectory_10k,
    bench_ocm_covariance_10k,
    bench_ocm_maneuver_10k,
    bench_tdm_xml_history_scaling,
    bench_xml_message_matrix,
    bench_oem_xml_scaling,
);
criterion_main!(benches);
