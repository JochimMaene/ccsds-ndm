// SPDX-FileCopyrightText: 2025 Jochim Maene <16223990+JochimMaene@users.noreply.github.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::traits::Ndm;
use ccsds_ndm::types::{Epoch, Position, PositionUnits, Velocity, VelocityUnits};
use criterion::{criterion_group, criterion_main, Criterion};
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

criterion_group!(benches, bench_parse_kvn, bench_generate_kvn);
criterion_main!(benches);
