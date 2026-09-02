// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! KVN parsing and generation benchmarks for all message types.

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::generation::VersionedNdm;
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::options::ParseOptions;
use ccsds_ndm::traits::{Ndm, Validate};
use ccsds_ndm::types::{
    CalendarEpoch, Epoch, InterpolationDegree, Position, PositionUnits, Velocity, VelocityUnits,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::num::NonZeroU32;
use std::str::FromStr;

fn create_test_oem(num_states: usize) -> Oem {
    let mut state_vectors = Vec::with_capacity(num_states);
    for i in 0..num_states {
        let second = i / 1_000;
        let millisecond = i % 1_000;
        state_vectors.push(StateVectorAcc {
            epoch: Epoch::from_str(&format!("2023-09-26T12:00:{second:02}.{millisecond:03}Z"))
                .unwrap(),
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
                    stop_time: Epoch::from_str("2023-09-26T12:01:00Z").unwrap(),
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

fn bench_generate_kvn_rounding(c: &mut Criterion) {
    let mut oem = create_test_oem(50000);
    for state in &mut oem.body.segment[0].data.state_vector {
        state.x.value = 1.234_567_890_123_456_7;
        state.y.value = 1.234_567_890_123_456_7;
        state.z.value = 1.234_567_890_123_456_7;
        state.x_dot.value = 1.234_567_890_123_456_7;
        state.y_dot.value = 1.234_567_890_123_456_7;
        state.z_dot.value = 1.234_567_890_123_456_7;
    }

    c.bench_function("kvn_generate_rounding", |b| {
        b.iter(|| black_box(&oem).to_kvn().unwrap())
    });
}

// --- Multi-message-type benchmarks ---

fn bench_parse_opm(c: &mut Criterion) {
    let opm_kvn = include_str!("../data/kvn/opm_g1.kvn");

    c.bench_function("kvn_parse_opm", |b| {
        b.iter(|| Opm::from_kvn(black_box(opm_kvn)).unwrap())
    });
}

fn bench_parse_opm_failures(c: &mut Criterion) {
    let valid = include_str!("../data/kvn/opm_g4.kvn");
    let invalid_late = format!("{valid}UNKNOWN_KEY = 1\n");
    let limited = ParseOptions::default().with_max_input_bytes(1);
    let mut group = c.benchmark_group("kvn_parse_opm_failure");
    group.bench_function("invalid_early", |b| {
        b.iter(|| black_box(Opm::from_kvn(black_box("not an OPM"))).unwrap_err())
    });
    group.bench_function("invalid_late", |b| {
        b.iter(|| black_box(Opm::from_kvn(black_box(&invalid_late))).unwrap_err())
    });
    group.bench_function("input_limit", |b| {
        b.iter(|| {
            black_box(Opm::from_kvn_with_options(
                black_box(valid),
                black_box(&limited),
            ))
            .unwrap_err()
        })
    });
    group.finish();
}

fn bench_generate_opm(c: &mut Criterion) {
    let opm = Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).unwrap();
    let output_len = opm.to_kvn().unwrap().len() as u64;
    let mut group = c.benchmark_group("kvn_generate_opm");
    group.throughput(Throughput::Bytes(output_len));
    group.bench_function("materialized", |b| {
        b.iter(|| black_box(&opm).to_kvn().unwrap())
    });
    let mut output = Vec::with_capacity(output_len as usize);
    group.bench_function("streaming_reused_vec", |b| {
        b.iter(|| {
            output.clear();
            black_box(&opm)
                .write_kvn_to(black_box(&mut output))
                .unwrap();
            black_box(&output);
        })
    });
    group.finish();
}

fn bench_generate_opm_maneuver_scaling(c: &mut Criterion) {
    let base = Opm::from_kvn(include_str!("../data/kvn/opm_g2.kvn")).unwrap();
    let maneuver = base.body.segment.data.maneuver_parameters[0].clone();
    let mut group = c.benchmark_group("kvn_generate_opm_maneuver_scaling");
    group.sample_size(60);

    for count in [1, 10, 100, 1000] {
        let mut opm = base.clone();
        opm.body.segment.data.maneuver_parameters = vec![maneuver.clone(); count];
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &opm, |b, opm| {
            b.iter(|| black_box(opm).to_kvn().unwrap())
        });
    }

    group.finish();
}

fn bench_validate_opm(c: &mut Criterion) {
    let valid = Opm::from_kvn(include_str!("../data/kvn/opm_g4.kvn")).unwrap();
    let mut invalid = valid.clone();
    invalid.header.originator.clear();
    invalid.body.segment.metadata.object_name.clear();
    invalid.body.segment.data.state_vector.x.value = f64::NAN;

    let maneuver_source = Opm::from_kvn(include_str!("../data/kvn/opm_g2.kvn")).unwrap();
    let mut maneuver_heavy = maneuver_source.clone();
    maneuver_heavy.body.segment.data.maneuver_parameters =
        vec![maneuver_source.body.segment.data.maneuver_parameters[0].clone(); 1000];

    let mut group = c.benchmark_group("opm_validate");
    group.bench_function("valid_rich", |b| {
        b.iter(|| black_box(&valid).validate().unwrap())
    });
    group.bench_function("invalid_aggregate", |b| {
        b.iter(|| black_box(&invalid).validation_errors().unwrap())
    });
    group.bench_function("valid_1000_maneuvers", |b| {
        b.iter(|| black_box(&maneuver_heavy).validate().unwrap())
    });
    group.finish();
}

fn bench_parse_omm(c: &mut Criterion) {
    let omm_kvn = include_str!("../data/kvn/omm_g7.kvn");

    c.bench_function("kvn_parse_omm", |b| {
        b.iter(|| Omm::from_kvn(black_box(omm_kvn)).unwrap())
    });
}

fn bench_parse_tdm(c: &mut Criterion) {
    let tdm_kvn = include_str!("../data/kvn/tdm_e1.kvn");

    c.bench_function("kvn_parse_tdm", |b| {
        b.iter(|| Tdm::from_kvn(black_box(tdm_kvn)).unwrap())
    });
}

fn bench_tdm_history_scaling(c: &mut Criterion) {
    let mut template = Tdm::from_kvn(include_str!("../data/kvn/tdm_e1.kvn")).unwrap();
    template.body.segments.truncate(1);
    let observation = template.body.segments[0].data.observations[0].clone();
    let mut group = c.benchmark_group("tdm_kvn_history_scaling");

    for records in [100, 1_000, 10_000, 50_000] {
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

fn bench_aem_history_scaling(c: &mut Criterion) {
    let template = Aem::from_kvn(include_str!("../data/kvn/aem_g5.kvn")).unwrap();
    let state = template.body.segment[0].data.attitude_states[0].clone();
    let mut group = c.benchmark_group("aem_kvn_history_scaling");

    for records in [100, 1_000, 10_000, 50_000] {
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
                state.spin.as_mut().unwrap().epoch = CalendarEpoch::from_str(&format!(
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

fn bench_acm_history_scaling(c: &mut Criterion) {
    let template = Acm::from_kvn(include_str!("../data/kvn/acm_g7.kvn")).unwrap();
    let line = template.body.segment.data.att[0].att_lines[0].clone();
    let mut group = c.benchmark_group("acm_kvn_history_scaling");

    for records in [100, 1_000, 10_000, 50_000] {
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
        ("opm", include_str!("../data/kvn/opm_g1.kvn")),
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
        let message = ccsds_ndm::from_str(input).unwrap();
        group.bench_with_input(
            BenchmarkId::new("generate", name),
            &message,
            |b, message| b.iter(|| black_box(message).to_kvn().unwrap()),
        );
    }
    group.finish();
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

    let mut epoch_group = c.benchmark_group("epoch_parse_formats");
    for (name, value) in [
        ("calendar", "2023-09-26T12:00:00.123456Z"),
        ("ordinal", "2023-269T12:00:00.123456Z"),
        ("relative_integer", "123456"),
        ("relative_decimal", "-123456.123456"),
    ] {
        epoch_group.bench_with_input(BenchmarkId::new("parse", name), value, |b, value| {
            b.iter(|| Epoch::from_str(black_box(value)).unwrap())
        });
    }
    epoch_group.finish();

    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    c.bench_function("micro_epoch_access_without_reparse", |b| {
        b.iter(|| black_box(&epoch).as_str())
    });

    let mut wrapper_group = c.benchmark_group("calendar_epoch_construction");
    for (name, value) in [
        ("calendar", "2023-09-26T12:00:00.123456Z"),
        ("ordinal", "2023-269T12:00:00.123456Z"),
    ] {
        wrapper_group.bench_with_input(BenchmarkId::new("calendar", name), value, |b, value| {
            b.iter(|| CalendarEpoch::from_str(black_box(value)).unwrap())
        });
    }
    wrapper_group.finish();

    // Float parsing - the core of data parsing
    c.bench_function("micro_float_parse", |b| {
        b.iter(|| fast_float::parse::<f64, _>(black_box("32021034790.7265")).unwrap())
    });
}

criterion_group!(
    benches,
    bench_parse_kvn,
    bench_generate_kvn,
    bench_generate_kvn_rounding,
    bench_parse_opm,
    bench_parse_opm_failures,
    bench_generate_opm,
    bench_generate_opm_maneuver_scaling,
    bench_validate_opm,
    bench_parse_omm,
    bench_parse_tdm,
    bench_tdm_history_scaling,
    bench_aem_history_scaling,
    bench_acm_history_scaling,
    bench_kvn_message_matrix,
    bench_kvn_scaling,
    bench_micro,
);
criterion_main!(benches);
