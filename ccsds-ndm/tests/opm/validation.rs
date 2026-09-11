// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::validate_xml;
use crate::{
    assert_invalid_value_diagnostic, assert_missing_required, assert_out_of_range_diagnostic,
    opm_with_maneuvers, OPM_3_KVN_FIXTURES,
};
use ccsds_ndm::error::{Result, ValidationError};
use ccsds_ndm::messages::opm::{KeplerianElements, Opm};
use ccsds_ndm::types::TimeUnits;
use ccsds_ndm::{Ndm, Validate};

#[test]
fn maneuver_diagnostics_identify_the_offending_maneuver() {
    const TWO_MANEUVERS: &str = include_str!("../../data/kvn/opm_g2.kvn");

    let mut opm = Opm::from_kvn(TWO_MANEUVERS).expect("fixture should parse");
    assert_eq!(opm.body.segment.data.maneuver_parameters.len(), 2);
    opm.body.segment.data.maneuver_parameters[1]
        .man_delta_mass
        .value = 1.0;

    let error = opm
        .validate()
        .expect_err("a positive delta mass is invalid");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters[1].man_delta_mass")
    );

    let mut opm = Opm::from_kvn(TWO_MANEUVERS).expect("fixture should parse");
    opm.body.segment.data.maneuver_parameters[1]
        .man_ref_frame
        .push('\u{1}');
    let error = opm
        .to_kvn()
        .expect_err("a control character is not KVN-representable");
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters[1].man_ref_frame")
    );
}

#[test]
fn xml_generation_rejects_non_finite_keplerian_values_after_mutation() {
    type Mutation = (&'static str, fn(&mut KeplerianElements));
    let mutations: [Mutation; 8] = [
        ("SEMI_MAJOR_AXIS", |elements| {
            elements.semi_major_axis.value = f64::NAN
        }),
        ("ECCENTRICITY", |elements| {
            elements.eccentricity.value = f64::INFINITY
        }),
        ("INCLINATION", |elements| {
            elements.inclination.angle.value = f64::NAN
        }),
        ("RA_OF_ASC_NODE", |elements| {
            elements.ra_of_asc_node.value = f64::NEG_INFINITY
        }),
        ("ARG_OF_PERICENTER", |elements| {
            elements.arg_of_pericenter.value = f64::NAN
        }),
        ("TRUE_ANOMALY", |elements| {
            elements.true_anomaly.as_mut().unwrap().value = f64::INFINITY
        }),
        ("MEAN_ANOMALY", |elements| {
            elements.true_anomaly = None;
            elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
            elements.mean_anomaly.as_mut().unwrap().value = f64::NAN;
        }),
        ("GM", |elements| elements.gm.value = f64::NAN),
    ];

    for (field, mutate) in mutations {
        let mut message = opm_with_maneuvers();
        mutate(
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap(),
        );
        let path = format!(
            "body.segment.data.keplerian_elements.{}",
            field.to_ascii_lowercase()
        );
        assert_invalid_value_diagnostic(field, message.to_xml(), &path);
    }
}

#[test]
fn xml_generation_enforces_keplerian_xsd_ranges_after_mutation() {
    type Mutation = (&'static str, fn(&mut KeplerianElements));
    let mutations: [Mutation; 7] = [
        ("ECCENTRICITY", |elements| {
            elements.eccentricity.value = -0.1
        }),
        ("INCLINATION", |elements| {
            elements.inclination.angle.value = 180.1
        }),
        ("RA_OF_ASC_NODE", |elements| {
            elements.ra_of_asc_node.value = 360.0
        }),
        ("ARG_OF_PERICENTER", |elements| {
            elements.arg_of_pericenter.value = -360.1
        }),
        ("TRUE_ANOMALY", |elements| {
            elements.true_anomaly.as_mut().unwrap().value = 360.0
        }),
        ("MEAN_ANOMALY", |elements| {
            elements.true_anomaly = None;
            elements.mean_anomaly = Some(ccsds_ndm::types::Angle::new(0.0, None).unwrap());
            elements.mean_anomaly.as_mut().unwrap().value = -360.1;
        }),
        ("GM", |elements| elements.gm.value = 0.0),
    ];

    for (field, mutate) in mutations {
        let mut message = opm_with_maneuvers();
        mutate(
            message
                .body
                .segment
                .data
                .keplerian_elements
                .as_mut()
                .unwrap(),
        );
        let path = format!(
            "body.segment.data.keplerian_elements.{}",
            field.to_ascii_lowercase()
        );
        assert_out_of_range_diagnostic(field, message.to_xml(), &path);
    }
}

#[test]
fn xml_generation_preserves_the_xsd_anomaly_choice() {
    let mut neither = opm_with_maneuvers();
    neither
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .true_anomaly = None;
    assert_invalid_choice_diagnostic(neither.to_xml(), &[]);

    let mut both = opm_with_maneuvers();
    let elements = both.body.segment.data.keplerian_elements.as_mut().unwrap();
    elements.mean_anomaly = elements.true_anomaly.clone();
    assert_invalid_choice_diagnostic(both.to_xml(), &["TRUE_ANOMALY", "MEAN_ANOMALY"]);
}

fn assert_invalid_choice_diagnostic<T: std::fmt::Debug>(
    result: Result<T>,
    expected_selected: &[&str],
) {
    let error = result.expect_err("invalid anomaly choice");
    assert_eq!(error.code(), Some("validation.invalid_choice"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.keplerian_elements")
    );
    let validation = error
        .as_validation_error()
        .expect("anomaly choice returned a non-validation error");
    let ValidationError::AtPath { source, .. } = validation else {
        panic!("choice diagnostic did not carry its containing model path");
    };
    let ValidationError::InvalidChoice {
        fields, selected, ..
    } = source.as_ref()
    else {
        panic!("choice diagnostic wrapped the wrong validation error");
    };
    assert_eq!(
        fields.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
        ["TRUE_ANOMALY", "MEAN_ANOMALY"]
    );
    assert_eq!(
        selected.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
        expected_selected
    );
}

#[test]
fn xml_generation_accepts_keplerian_xsd_boundaries() {
    let mut message = opm_with_maneuvers();
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

#[test]
fn opm_xml_generation_rejects_day_as_maneuver_duration_unit() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse OPM 3.0 fixture");
    opm.body.segment.data.maneuver_parameters[0]
        .man_duration
        .units = Some(TimeUnits::Day);

    let error = opm
        .to_xml()
        .expect_err("day is not an XSD-valid duration unit");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.maneuver_parameters[0].man_duration.units")
    );
    assert!(error.as_validation_error().is_some());
}

#[test]
fn opm_xml_generation_accepts_seconds_as_maneuver_duration_unit() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse OPM 3.0 fixture");
    opm.body.segment.data.maneuver_parameters[0]
        .man_duration
        .units = Some(TimeUnits::Seconds);

    let xml = opm.to_xml().expect("seconds is the XSD duration unit");
    assert!(xml.contains("<MAN_DURATION units=\"s\">"));
    assert!(!xml.contains("<MAN_DURATION units=\"d\">"));
}

#[test]
fn opm_3_xml_generation_reports_all_reachable_missing_required_paths() {
    type Mutation = (&'static str, &'static str, usize, fn(&mut Opm));
    let mutations: [Mutation; 9] = [
        ("id", "id", 0, |opm| opm.id = None),
        ("ORIGINATOR", "header.originator", 0, |opm| {
            opm.header.originator.clear()
        }),
        (
            "OBJECT_NAME",
            "body.segment.metadata.object_name",
            0,
            |opm| opm.body.segment.metadata.object_name.clear(),
        ),
        ("OBJECT_ID", "body.segment.metadata.object_id", 0, |opm| {
            opm.body.segment.metadata.object_id.clear()
        }),
        (
            "CENTER_NAME",
            "body.segment.metadata.center_name",
            0,
            |opm| opm.body.segment.metadata.center_name.clear(),
        ),
        ("REF_FRAME", "body.segment.metadata.ref_frame", 0, |opm| {
            opm.body.segment.metadata.ref_frame.clear()
        }),
        (
            "TIME_SYSTEM",
            "body.segment.metadata.time_system",
            0,
            |opm| opm.body.segment.metadata.time_system.clear(),
        ),
        (
            "MASS",
            "body.segment.data.spacecraft_parameters.mass",
            1,
            |opm| {
                opm.body
                    .segment
                    .data
                    .spacecraft_parameters
                    .as_mut()
                    .unwrap()
                    .mass = None
            },
        ),
        (
            "MAN_REF_FRAME",
            "body.segment.data.maneuver_parameters[0].man_ref_frame",
            1,
            |opm| {
                opm.body.segment.data.maneuver_parameters[0]
                    .man_ref_frame
                    .clear()
            },
        ),
    ];

    for (field, path, fixture, mutate) in mutations {
        let mut opm =
            Opm::from_kvn(OPM_3_KVN_FIXTURES[fixture].1).expect("failed to parse OPM fixture");
        mutate(&mut opm);
        assert_missing_required(field, opm.to_xml(), field, path);
    }
}

#[test]
fn opm_3_xml_generation_rejects_every_non_finite_state_vector_component() {
    type Mutation = (&'static str, fn(&mut Opm));
    let mutations: [Mutation; 6] = [
        ("X", |opm| {
            opm.body.segment.data.state_vector.x.value = f64::NAN
        }),
        ("Y", |opm| {
            opm.body.segment.data.state_vector.y.value = f64::INFINITY
        }),
        ("Z", |opm| {
            opm.body.segment.data.state_vector.z.value = f64::NEG_INFINITY
        }),
        ("X_DOT", |opm| {
            opm.body.segment.data.state_vector.x_dot.value = f64::NAN
        }),
        ("Y_DOT", |opm| {
            opm.body.segment.data.state_vector.y_dot.value = f64::INFINITY
        }),
        ("Z_DOT", |opm| {
            opm.body.segment.data.state_vector.z_dot.value = f64::NEG_INFINITY
        }),
    ];

    for (field, mutate) in mutations {
        let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
        mutate(&mut opm);
        let path = format!(
            "body.segment.data.state_vector.{}",
            field.to_ascii_lowercase()
        );
        assert_invalid_value_diagnostic(field, opm.to_xml(), &path);
    }
}

#[test]
fn opm_3_xml_generation_rejects_every_invalid_spacecraft_value() {
    type Mutation = (&'static str, bool, fn(&mut Opm));
    let mutations: [Mutation; 10] = [
        ("MASS", false, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .mass
                .as_mut()
                .unwrap()
                .value = -1.0
        }),
        ("MASS", true, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .mass
                .as_mut()
                .unwrap()
                .value = f64::NAN
        }),
        ("SOLAR_RAD_AREA", false, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .solar_rad_area
                .as_mut()
                .unwrap()
                .value = -1.0
        }),
        ("SOLAR_RAD_AREA", true, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .solar_rad_area
                .as_mut()
                .unwrap()
                .value = f64::NAN
        }),
        ("SOLAR_RAD_COEFF", false, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .solar_rad_coeff
                .as_mut()
                .unwrap()
                .value = -1.0
        }),
        ("SOLAR_RAD_COEFF", true, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .solar_rad_coeff
                .as_mut()
                .unwrap()
                .value = f64::INFINITY
        }),
        ("DRAG_AREA", false, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .drag_area
                .as_mut()
                .unwrap()
                .value = -1.0
        }),
        ("DRAG_AREA", true, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .drag_area
                .as_mut()
                .unwrap()
                .value = f64::NAN
        }),
        ("DRAG_COEFF", false, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .drag_coeff
                .as_mut()
                .unwrap()
                .value = -1.0
        }),
        ("DRAG_COEFF", true, |opm| {
            opm.body
                .segment
                .data
                .spacecraft_parameters
                .as_mut()
                .unwrap()
                .drag_coeff
                .as_mut()
                .unwrap()
                .value = f64::NEG_INFINITY
        }),
    ];

    for (field, is_invalid_value, mutate) in mutations {
        let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
        mutate(&mut opm);
        let path = format!(
            "body.segment.data.spacecraft_parameters.{}",
            field.to_ascii_lowercase()
        );
        if is_invalid_value {
            assert_invalid_value_diagnostic(field, opm.to_xml(), &path);
        } else {
            assert_out_of_range_diagnostic(field, opm.to_xml(), &path);
        }
    }
}

#[test]
fn opm_3_xml_generation_rejects_every_non_finite_covariance_component() {
    macro_rules! mutations {
        ($($field:ident),+ $(,)?) => {
            [
                $((
                    stringify!($field),
                    (|opm: &mut Opm| {
                        opm.body
                            .segment
                            .data
                            .covariance_matrix
                            .as_mut()
                            .unwrap()
                            .$field
                            .value = f64::NAN;
                    }) as fn(&mut Opm),
                )),+
            ]
        };
    }

    let mutations = mutations![
        cx_x,
        cy_x,
        cy_y,
        cz_x,
        cz_y,
        cz_z,
        cx_dot_x,
        cx_dot_y,
        cx_dot_z,
        cx_dot_x_dot,
        cy_dot_x,
        cy_dot_y,
        cy_dot_z,
        cy_dot_x_dot,
        cy_dot_y_dot,
        cz_dot_x,
        cz_dot_y,
        cz_dot_z,
        cz_dot_x_dot,
        cz_dot_y_dot,
        cz_dot_z_dot,
    ];

    for (field, mutate) in mutations {
        let mut opm =
            Opm::from_kvn(OPM_3_KVN_FIXTURES[3].1).expect("failed to parse covariance fixture");
        mutate(&mut opm);
        let path = format!(
            "body.segment.data.covariance_matrix.{}",
            field.to_ascii_lowercase()
        );
        assert_invalid_value_diagnostic(field, opm.to_xml(), &path);
    }
}

#[test]
fn opm_3_xml_generation_rejects_every_invalid_maneuver_value() {
    type Mutation = (&'static str, fn(&mut Opm));
    let mutations: [Mutation; 5] = [
        ("MAN_DURATION", |opm| {
            opm.body.segment.data.maneuver_parameters[0]
                .man_duration
                .value = f64::NAN
        }),
        ("MAN_DELTA_MASS", |opm| {
            opm.body.segment.data.maneuver_parameters[0]
                .man_delta_mass
                .value = f64::INFINITY
        }),
        ("MAN_DV_1", |opm| {
            opm.body.segment.data.maneuver_parameters[0].man_dv_1.value = f64::NAN
        }),
        ("MAN_DV_2", |opm| {
            opm.body.segment.data.maneuver_parameters[0].man_dv_2.value = f64::INFINITY
        }),
        ("MAN_DV_3", |opm| {
            opm.body.segment.data.maneuver_parameters[0].man_dv_3.value = f64::NEG_INFINITY
        }),
    ];

    for (field, mutate) in mutations {
        let mut opm =
            Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
        mutate(&mut opm);
        let path = format!(
            "body.segment.data.maneuver_parameters[0].{}",
            field.to_ascii_lowercase()
        );
        assert_invalid_value_diagnostic(field, opm.to_xml(), &path);
    }
}

#[test]
fn opm_3_xml_generation_validates_maneuver_boundaries() {
    let mut maneuver =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    maneuver.body.segment.data.maneuver_parameters[0]
        .man_duration
        .value = -1.0;
    assert_out_of_range_diagnostic(
        "MAN_DURATION",
        maneuver.to_xml(),
        "body.segment.data.maneuver_parameters[0].man_duration",
    );

    let mut zero_delta_mass =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    zero_delta_mass.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 0.0;
    let xml = zero_delta_mass
        .to_xml()
        .expect("the OPM 3.0 XML schema permits zero MAN_DELTA_MASS");
    validate_xml("zero MAN_DELTA_MASS", &xml);

    zero_delta_mass.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 1.0;
    assert_out_of_range_diagnostic(
        "MAN_DELTA_MASS",
        zero_delta_mass.to_xml(),
        "body.segment.data.maneuver_parameters[0].man_delta_mass",
    );
}
