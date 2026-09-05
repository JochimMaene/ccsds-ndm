// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! One-shot 10k-record OCM harness for external peak-RSS measurement.
//!
//! Build once, then run each mode through `/usr/bin/time -v`:
//! `cargo build --release --example ocm_memory`
//! `time -v target/release/examples/ocm_memory xml-parse`
//! `time -v target/release/examples/ocm_memory cov-xml-parse`
//! `time -v target/release/examples/ocm_memory man-xml-parse`

use ccsds_ndm::common::OdmHeader;
use ccsds_ndm::messages::ocm::{
    CovLine, ManLine, Ocm, OcmBody, OcmCovarianceMatrix, OcmData, OcmManeuverParameters,
    OcmMetadata, OcmSegment, OcmTrajState, TrajLine,
};
use ccsds_ndm::types::Epoch;
use ccsds_ndm::Ndm;
use std::str::FromStr;

const RECORDS: usize = 10_000;

fn create_ocm() -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut lines = Vec::with_capacity(RECORDS);
    for i in 0..RECORDS {
        let line_epoch = Epoch::from_str(&format!(
            "2023-09-26T{:02}:{:02}:{:02}.123456Z",
            i / 3600,
            (i / 60) % 60,
            i % 60
        ))
        .unwrap();
        lines.push(TrajLine {
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
                                .traj_lines(lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
}

fn create_covariance_ocm() -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut lines = Vec::with_capacity(RECORDS);
    for i in 0..RECORDS {
        let line_epoch = Epoch::from_str(&format!(
            "2023-09-26T{:02}:{:02}:{:02}.123456Z",
            i / 3600,
            (i / 60) % 60,
            i % 60
        ))
        .unwrap();
        lines.push(CovLine {
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
                                .cov_lines(lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
}

fn create_maneuver_ocm() -> Ocm {
    let epoch = Epoch::from_str("2023-09-26T12:00:00.123456Z").unwrap();
    let mut lines = Vec::with_capacity(RECORDS);
    for i in 0..RECORDS {
        let line_epoch = Epoch::from_str(&format!("{}.5", i)).unwrap();
        lines.push(ManLine {
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
                                .man_lines(lines)
                                .build()],
                            ..OcmData::default()
                        })
                        .build(),
                ))
                .build(),
        )
        .build()
}

fn retained_bytes(ocm: &Ocm) -> usize {
    let lines = &ocm.body.segment.data.traj[0].traj_lines;
    lines.capacity() * std::mem::size_of::<TrajLine>()
        + lines
            .iter()
            .map(|line| line.values.capacity() * std::mem::size_of::<f64>())
            .sum::<usize>()
}

fn retained_covariance_bytes(ocm: &Ocm) -> usize {
    let lines = &ocm.body.segment.data.cov[0].cov_lines;
    lines.capacity() * std::mem::size_of::<CovLine>()
        + lines
            .iter()
            .map(|line| line.values.capacity() * std::mem::size_of::<f64>())
            .sum::<usize>()
}

fn retained_maneuver_bytes(ocm: &Ocm) -> usize {
    let lines = &ocm.body.segment.data.man[0].man_lines;
    lines.capacity() * std::mem::size_of::<ManLine>()
        + lines
            .iter()
            .map(|line| {
                line.values
                    .iter()
                    .map(|value| value.capacity())
                    .sum::<usize>()
            })
            .sum::<usize>()
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "model".into());
    let ocm = match mode.as_str() {
        "model" => create_ocm(),
        "xml-parse" => {
            let xml = create_ocm().to_xml().unwrap();
            Ocm::from_xml(std::hint::black_box(&xml)).unwrap()
        }
        "kvn-parse" => {
            let kvn = create_ocm().to_kvn().unwrap();
            Ocm::from_kvn(std::hint::black_box(&kvn)).unwrap()
        }
        "cov-model" => create_covariance_ocm(),
        "cov-xml-parse" => {
            let xml = create_covariance_ocm().to_xml().unwrap();
            Ocm::from_xml(std::hint::black_box(&xml)).unwrap()
        }
        "cov-kvn-parse" => {
            let kvn = create_covariance_ocm().to_kvn().unwrap();
            Ocm::from_kvn(std::hint::black_box(&kvn)).unwrap()
        }
        "man-model" => create_maneuver_ocm(),
        "man-xml-parse" => {
            let xml = create_maneuver_ocm().to_xml().unwrap();
            Ocm::from_xml(std::hint::black_box(&xml)).unwrap()
        }
        "man-kvn-parse" => {
            let kvn = create_maneuver_ocm().to_kvn().unwrap();
            Ocm::from_kvn(std::hint::black_box(&kvn)).unwrap()
        }
        _ => panic!(
            "expected model, xml-parse, kvn-parse, cov-model, cov-xml-parse, cov-kvn-parse, man-model, man-xml-parse, or man-kvn-parse"
        ),
    };
    if mode.starts_with("cov-") {
        println!(
            "retained covariance storage: {} bytes",
            retained_covariance_bytes(&ocm)
        );
    } else if mode.starts_with("man-") {
        println!(
            "retained maneuver storage: {} bytes",
            retained_maneuver_bytes(&ocm)
        );
    } else {
        println!(
            "retained trajectory storage: {} bytes",
            retained_bytes(&ocm)
        );
    }
    std::hint::black_box(&ocm);
}
