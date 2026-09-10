// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Fixtures shared by the KVN and XML benchmark targets.

use ccsds_ndm::common::{OdmHeader, StateVectorAcc};
use ccsds_ndm::messages::oem::{Oem, OemBody, OemData, OemMetadata, OemSegment};
use ccsds_ndm::types::{
    CalendarEpoch, Epoch, InterpolationDegree, Position, PositionUnits, Velocity, VelocityUnits,
};
use std::num::NonZeroU32;
use std::str::FromStr;

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
