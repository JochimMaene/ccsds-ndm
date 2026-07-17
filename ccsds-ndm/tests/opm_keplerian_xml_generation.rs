// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::traits::Ndm;

const OPM_WITH_KEPLERIAN_ELEMENTS: &str = include_str!("../../data/kvn/opm_g2.kvn");

fn opm() -> Opm {
    Opm::from_kvn(OPM_WITH_KEPLERIAN_ELEMENTS).expect("the OPM Keplerian fixture must remain valid")
}

#[test]
fn xml_generation_rejects_non_finite_keplerian_values_after_mutation() {
    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .semi_major_axis
        .value = f64::NAN;
    assert!(message.to_xml().is_err());

    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .eccentricity
        .value = f64::INFINITY;
    assert!(message.to_xml().is_err());

    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .inclination
        .angle
        .value = f64::NAN;
    assert!(message.to_xml().is_err());

    for field in [
        "RA_OF_ASC_NODE",
        "ARG_OF_PERICENTER",
        "TRUE_ANOMALY",
        "MEAN_ANOMALY",
    ] {
        let mut message = opm();
        let elements = message
            .body
            .segment
            .data
            .keplerian_elements
            .as_mut()
            .unwrap();
        match field {
            "RA_OF_ASC_NODE" => elements.ra_of_asc_node.value = f64::NEG_INFINITY,
            "ARG_OF_PERICENTER" => elements.arg_of_pericenter.value = f64::NAN,
            "TRUE_ANOMALY" => elements.true_anomaly.as_mut().unwrap().value = f64::INFINITY,
            "MEAN_ANOMALY" => {
                elements.true_anomaly = None;
                elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
                elements.mean_anomaly.as_mut().unwrap().value = f64::NAN;
            }
            _ => unreachable!(),
        }
        assert!(message.to_xml().is_err(), "accepted non-finite {field}");
    }

    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .gm
        .value = f64::NAN;
    assert!(message.to_xml().is_err());
}

#[test]
fn xml_generation_enforces_keplerian_xsd_ranges_after_mutation() {
    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .eccentricity
        .value = -0.1;
    assert!(message.to_xml().is_err());

    for inclination in [-0.1, 180.1] {
        let mut message = opm();
        message
            .body
            .segment
            .data
            .keplerian_elements
            .as_mut()
            .unwrap()
            .inclination
            .angle
            .value = inclination;
        assert!(
            message.to_xml().is_err(),
            "accepted inclination {inclination}"
        );
    }

    for angle in [-360.1, 360.0] {
        let mut message = opm();
        message
            .body
            .segment
            .data
            .keplerian_elements
            .as_mut()
            .unwrap()
            .ra_of_asc_node
            .value = angle;
        assert!(message.to_xml().is_err(), "accepted angle {angle}");
    }

    let mut message = opm();
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .gm
        .value = 0.0;
    assert!(message.to_xml().is_err());
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
