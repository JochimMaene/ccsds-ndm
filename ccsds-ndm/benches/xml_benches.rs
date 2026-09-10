// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! XML parsing and generation benchmarks for all message types.
//!
//! `xml_message_matrix` carries every standalone family plus the combined NDM, because the
//! conformance files cite it as each family's workload evidence. Beyond it, each family with a
//! record history gets one `<family>_xml_history_scaling` group over
//! [`common::XML_HISTORY_SIZES`]; `kvn_benches.rs` registers the same families, and every size
//! here has a KVN counterpart.

mod common;

use ccsds_ndm::messages::acm::Acm;
use ccsds_ndm::messages::aem::Aem;
use ccsds_ndm::messages::ocm::Ocm;
use ccsds_ndm::messages::oem::Oem;
use ccsds_ndm::messages::tdm::Tdm;
use ccsds_ndm::Ndm;
use common::{
    create_test_acm, create_test_aem, create_test_ocm, create_test_ocm_covariance,
    create_test_ocm_maneuver, create_test_oem, create_test_tdm, XML_HISTORY_SIZES,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

/// Register XML parse and generation for one family across every history size.
fn bench_xml_history_scaling<M, B>(c: &mut Criterion, name: &str, build: B)
where
    M: Ndm + PartialEq,
    B: Fn(usize) -> M,
{
    let mut group = c.benchmark_group(name);
    for records in XML_HISTORY_SIZES {
        let message = build(records);
        let input = message.to_xml().unwrap();
        group.throughput(Throughput::Elements(records as u64));
        group.bench_with_input(BenchmarkId::new("parse", records), &input, |b, input| {
            b.iter(|| M::from_xml(black_box(input)).unwrap())
        });
        group.bench_with_input(
            BenchmarkId::new("generate", records),
            &message,
            |b, message| b.iter(|| black_box(message).to_xml().unwrap()),
        );
    }
    group.finish();
}

fn bench_oem_xml_history_scaling(c: &mut Criterion) {
    bench_xml_history_scaling::<Oem, _>(c, "oem_xml_history_scaling", create_test_oem);
}

fn bench_tdm_xml_history_scaling(c: &mut Criterion) {
    bench_xml_history_scaling::<Tdm, _>(c, "tdm_xml_history_scaling", create_test_tdm);
}

fn bench_aem_xml_history_scaling(c: &mut Criterion) {
    bench_xml_history_scaling::<Aem, _>(c, "aem_xml_history_scaling", create_test_aem);
}

fn bench_acm_xml_history_scaling(c: &mut Criterion) {
    bench_xml_history_scaling::<Acm, _>(c, "acm_xml_history_scaling", create_test_acm);
}

/// OCM carries three independent histories, each with its own element grammar.
fn bench_ocm_xml_history_scaling(c: &mut Criterion) {
    bench_xml_history_scaling::<Ocm, _>(c, "ocm_trajectory_xml_history_scaling", create_test_ocm);
    bench_xml_history_scaling::<Ocm, _>(
        c,
        "ocm_covariance_xml_history_scaling",
        create_test_ocm_covariance,
    );
    bench_xml_history_scaling::<Ocm, _>(
        c,
        "ocm_maneuver_xml_history_scaling",
        create_test_ocm_maneuver,
    );
}

fn bench_xml_message_matrix(c: &mut Criterion) {
    let acm = ccsds_ndm::from_str(include_str!("../data/kvn/acm_g6.kvn"))
        .unwrap()
        .to_xml()
        .unwrap();
    let cases = vec![
        ("opm", include_str!("../data/xml/opm_g5.xml").to_owned()),
        ("omm", include_str!("../data/xml/omm_g10.xml").to_owned()),
        ("oem", include_str!("../data/xml/oem_g14.xml").to_owned()),
        ("ocm", include_str!("../data/xml/ocm_g20.xml").to_owned()),
        ("cdm", include_str!("../data/xml/cdm_44.xml").to_owned()),
        ("tdm", include_str!("../data/xml/tdm_e21.xml").to_owned()),
        ("rdm", include_str!("../data/xml/rdm_c3.xml").to_owned()),
        ("aem", include_str!("../data/xml/aem_g11.xml").to_owned()),
        ("apm", include_str!("../data/xml/apm_g10.xml").to_owned()),
        ("acm", acm),
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

criterion_group!(
    benches,
    bench_xml_message_matrix,
    bench_oem_xml_history_scaling,
    bench_ocm_xml_history_scaling,
    bench_tdm_xml_history_scaling,
    bench_aem_xml_history_scaling,
    bench_acm_xml_history_scaling,
);
criterion_main!(benches);
