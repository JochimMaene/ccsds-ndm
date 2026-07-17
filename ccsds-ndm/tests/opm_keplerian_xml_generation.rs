// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::{CcsdsNdmError, Result};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

const OPM_WITH_KEPLERIAN_ELEMENTS: &str = include_str!("../../data/kvn/opm_g2.kvn");

fn opm() -> Opm {
    Opm::from_kvn(OPM_WITH_KEPLERIAN_ELEMENTS).expect("the OPM Keplerian fixture must remain valid")
}

#[test]
fn xml_generation_rejects_non_finite_keplerian_values_after_mutation() {
    type Mutation = (&'static str, fn(&mut Opm));
    let mutations: [Mutation; 8] = [
        ("SEMI_MAJOR_AXIS", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .semi_major_axis
                .value = f64::NAN
        }),
        ("ECCENTRICITY", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .eccentricity
                .value = f64::INFINITY
        }),
        ("INCLINATION", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .inclination
                .angle
                .value = f64::NAN
        }),
        ("RA_OF_ASC_NODE", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .ra_of_asc_node
                .value = f64::NEG_INFINITY
        }),
        ("ARG_OF_PERICENTER", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .arg_of_pericenter
                .value = f64::NAN
        }),
        ("TRUE_ANOMALY", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .true_anomaly
                .as_mut()
                .unwrap()
                .value = f64::INFINITY
        }),
        ("MEAN_ANOMALY", |message| {
            let elements = message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap();
            elements.true_anomaly = None;
            elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
            elements.mean_anomaly.as_mut().unwrap().value = f64::NAN;
        }),
        ("GM", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .gm
                .value = f64::NAN
        }),
    ];

    for (field, mutate) in mutations {
        let mut message = opm();
        mutate(&mut message);
        let path = format!(
            "body.segment.data.keplerian_elements.{}",
            field.to_ascii_lowercase()
        );
        assert_invalid_value_diagnostic(field, message.to_xml(), &path);
    }
}

fn assert_invalid_value_diagnostic<T: std::fmt::Debug>(
    field: &str,
    result: Result<T>,
    expected_path: &str,
) {
    match result.expect_err(field) {
        CcsdsNdmError::Validation(error) => {
            assert_eq!(error.code(), Some("validation.invalid_value"));
            assert_eq!(error.field_path().as_deref(), Some(expected_path));
        }
        error => panic!("{field} returned a non-validation error: {error}"),
    }
}

#[test]
fn xml_generation_enforces_keplerian_xsd_ranges_after_mutation() {
    type Mutation = (&'static str, fn(&mut Opm));
    let mutations: [Mutation; 7] = [
        ("ECCENTRICITY", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .eccentricity
                .value = -0.1
        }),
        ("INCLINATION", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .inclination
                .angle
                .value = 180.1
        }),
        ("RA_OF_ASC_NODE", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .ra_of_asc_node
                .value = 360.0
        }),
        ("ARG_OF_PERICENTER", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .arg_of_pericenter
                .value = -360.1
        }),
        ("TRUE_ANOMALY", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .true_anomaly
                .as_mut()
                .unwrap()
                .value = 360.0
        }),
        ("MEAN_ANOMALY", |message| {
            let elements = message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap();
            elements.true_anomaly = None;
            elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
            elements.mean_anomaly.as_mut().unwrap().value = -360.1;
        }),
        ("GM", |message| {
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap()
                .gm
                .value = 0.0
        }),
    ];

    for (field, mutate) in mutations {
        let mut message = opm();
        mutate(&mut message);
        let path = format!(
            "body.segment.data.keplerian_elements.{}",
            field.to_ascii_lowercase()
        );
        assert_out_of_range_diagnostic(field, message.to_xml(), &path);
    }
}

fn assert_out_of_range_diagnostic<T: std::fmt::Debug>(
    field: &str,
    result: Result<T>,
    expected_path: &str,
) {
    match result.expect_err(field) {
        CcsdsNdmError::Validation(error) => {
            assert_eq!(error.code(), Some("validation.out_of_range"));
            assert_eq!(error.field_path().as_deref(), Some(expected_path));
        }
        error => panic!("{field} returned a non-validation error: {error}"),
    }
}

#[test]
fn xml_generation_preserves_the_xsd_anomaly_choice() {
    let mut neither = opm();
    neither
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .true_anomaly = None;
    assert!(neither.to_xml().is_err());

    let mut both = opm();
    let elements = both.body.segment.data.keplerian_elements.as_mut().unwrap();
    elements.mean_anomaly = elements.true_anomaly.clone();
    assert!(both.to_xml().is_err());
}

#[test]
fn xml_generation_accepts_keplerian_xsd_boundaries() {
    let mut message = opm();
    let elements = message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap();
    elements.eccentricity.value = 0.0;
    elements.inclination.angle.value = 180.0;
    elements.ra_of_asc_node.value = -360.0;
    elements.arg_of_pericenter.value = 359.999;
    elements.true_anomaly = None;
    elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
    elements.gm.value = f64::MIN_POSITIVE;

    message
        .to_xml()
        .expect("XSD-inclusive Keplerian boundaries must generate");
}
