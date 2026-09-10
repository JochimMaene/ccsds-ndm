// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Fixtures shared by the KVN and XML benchmark targets.

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::ocm::{
    CovLine, ManLine, Ocm, OcmBody, OcmCovarianceMatrix, OcmData, OcmManeuverParameters,
    OcmMetadata, OcmSegment, OcmTrajState, TrajLine,
};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::types::{
    CalendarEpoch, Epoch, InterpolationDegree, Position, PositionUnits, Velocity, VelocityUnits,
};
use ccsds_ndm::Ndm;
use std::num::NonZeroU32;
use std::str::FromStr;

/// Record counts the KVN scaling groups use.
#[allow(dead_code)]
pub const KVN_HISTORY_SIZES: [usize; 3] = [100, 10_000, 50_000];

/// Record counts the XML scaling groups use.
#[allow(dead_code)]
pub const XML_HISTORY_SIZES: [usize; 2] = [100, 10_000];

/// An OEM with `num_states` state vectors on a millisecond cadence.
///
/// Both benchmark targets scale the same message, so they build it the same way.
pub fn create_test_oem(num_states: usize) -> Oem {
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

/// Create a test OCM with one CARTPV trajectory block of the requested size.
pub fn create_test_ocm(num_states: usize) -> Ocm {
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
pub fn create_test_ocm_covariance(num_states: usize) -> Ocm {
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
pub fn create_test_ocm_maneuver(num_states: usize) -> Ocm {
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

/// A TDM whose single segment repeats one shipped observation `records` times.
pub fn create_test_tdm(records: usize) -> Tdm {
    let mut message = Tdm::from_kvn(include_str!("../../data/kvn/tdm_e1.kvn")).unwrap();
    message.body.segments.truncate(1);
    let observation = message.body.segments[0].data.observations[0].clone();
    message.body.segments[0].data.observations = vec![observation; records];
    message
}

/// An AEM carrying `records` attitude states on a one-second cadence.
///
/// AEM epochs must strictly increase, so each cloned state is re-stamped.
pub fn create_test_aem(records: usize) -> Aem {
    let mut message = Aem::from_kvn(include_str!("../../data/kvn/aem_g5.kvn")).unwrap();
    let state = message.body.segment[0].data.attitude_states[0].clone();
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
    message
}

/// An ACM whose attitude block repeats one shipped line `records` times.
pub fn create_test_acm(records: usize) -> Acm {
    let mut message = Acm::from_kvn(include_str!("../../data/kvn/acm_g7.kvn")).unwrap();
    let line = message.body.segment.data.att[0].att_lines[0].clone();
    message.body.segment.data.att[0].att_lines = vec![line; records];
    message
}
