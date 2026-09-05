// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use ccsds_ndm::error::{Result, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::{Message, Ndm};
use std::path::{Path, PathBuf};
use std::process::Command;

const OPM_3_KVN_FIXTURES: [(&str, &str); 4] = [
    ("opm_g1.kvn", include_str!("../data/kvn/opm_g1.kvn")),
    ("opm_g2.kvn", include_str!("../data/kvn/opm_g2.kvn")),
    ("opm_g3.kvn", include_str!("../data/kvn/opm_g3.kvn")),
    ("opm_g4.kvn", include_str!("../data/kvn/opm_g4.kvn")),
];
const OPM_3_XML_FIXTURES: [(&str, &str); 1] =
    [("opm_g5.xml", include_str!("../data/xml/opm_g5.xml"))];

#[test]
fn public_opm_xml_generation_signatures_remain_compatible() {
    let _typed: fn(&Opm) -> ccsds_ndm::error::Result<String> = <Opm as Ndm>::to_xml;
    let _typed_streaming: fn(&Opm, &mut Vec<u8>) -> ccsds_ndm::error::Result<()> =
        <Opm as Ndm>::write_xml_to::<Vec<u8>>;
    let _generic: fn(&Message) -> ccsds_ndm::error::Result<String> = Message::to_xml;
    let _generic_file: fn(&Message, PathBuf) -> ccsds_ndm::error::Result<()> =
        Message::to_xml_file::<PathBuf>;
}

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/xsd/ndmxml-4.0.0-master-4.0.xsd")
}

fn validate_with_official_xsd(label: &str, xml: &str) {
    let generated = tempfile::NamedTempFile::new().expect("failed to create temporary XML file");
    std::fs::write(generated.path(), xml).expect("failed to write generated XML");

    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema_path())
        .arg(generated.path())
        .output()
        .unwrap_or_else(|error| {
            panic!("xmllint is required for conformance tests; install libxml2-utils: {error}")
        });

    assert!(
        output.status.success(),
        "generated XML for {label} failed the official OPM 3.0 XSD:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_missing_required<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_field: &str,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.missing_required_field"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("{surface} returned a non-validation error: {error}"));
    let validation = validation_error_source(validation);
    assert!(
        matches!(
            validation,
            ValidationError::MissingRequiredField { field, .. }
                if field.as_ref() == expected_field
        ),
        "{surface} returned the wrong validation error: {validation}"
    );
}

fn assert_missing_object_name<T: std::fmt::Debug>(surface: &str, result: Result<T>) {
    assert_missing_required(
        surface,
        result,
        "OBJECT_NAME",
        "body.segment.metadata.object_name",
    );
}

#[test]
fn every_shipped_opm_3_fixture_generates_xsd_valid_xml() {
    for (name, kvn) in OPM_3_KVN_FIXTURES {
        let opm =
            Opm::from_kvn(kvn).unwrap_or_else(|error| panic!("failed to parse {name}: {error}"));
        assert_eq!(opm.version, "3.0", "{name} is not an OPM 3.0 fixture");
        let xml = opm
            .to_xml()
            .unwrap_or_else(|error| panic!("failed to generate XML for {name}: {error}"));
        validate_with_official_xsd(name, &xml);
    }

    for (name, xml) in OPM_3_XML_FIXTURES {
        let opm =
            Opm::from_xml(xml).unwrap_or_else(|error| panic!("failed to parse {name}: {error}"));
        assert_eq!(opm.version, "3.0", "{name} is not an OPM 3.0 fixture");
        let generated = opm
            .to_xml()
            .unwrap_or_else(|error| panic!("failed to regenerate XML for {name}: {error}"));
        validate_with_official_xsd(name, &generated);
    }
}

#[test]
fn opm_3_xml_generation_is_deterministic_across_rust_entry_points() {
    let opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    let expected = opm.to_xml().expect("typed generation failed");

    assert_eq!(opm.to_xml().expect("repeated generation failed"), expected);
    let mut streamed = Vec::new();
    opm.write_xml_to(&mut streamed)
        .expect("streaming generation failed");
    assert_eq!(streamed, expected.as_bytes());

    let generic = Message::Opm(opm);
    assert_eq!(
        generic.to_xml().expect("generic generation failed"),
        expected
    );
    validate_with_official_xsd("deterministic Rust entry points", &expected);
}

#[test]
fn every_opm_3_xml_generation_entry_point_rejects_an_invalid_model() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.body.segment.metadata.object_name.clear();

    assert_missing_object_name("Ndm::to_xml", opm.to_xml());
    let mut streamed = Vec::new();
    assert_missing_object_name("Ndm::write_xml_to", opm.write_xml_to(&mut streamed));
    assert!(
        streamed.is_empty(),
        "invalid streaming output was partially written"
    );

    let generic = Message::Opm(opm);
    assert_missing_object_name("Message::to_xml", generic.to_xml());
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let path = directory.path().join("invalid-opm.xml");
    assert_missing_object_name("Message::to_xml_file", generic.to_xml_file(&path));
    assert!(
        !path.exists(),
        "invalid file generation created an output file"
    );

    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.body.segment.data.state_vector.x.value = f64::NAN;
    assert_non_finite_rejected("Ndm::to_xml with non-finite state", opm.to_xml());
}

#[test]
fn opm_3_xml_generation_rejects_unaudited_editions_across_rust_entry_points() {
    let opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    let historical = {
        let mut message = opm.clone();
        message.version = "1.0".into();
        message
    };
    assert_unsupported_version(
        "Ndm::to_xml",
        "1.0",
        "generation.unsupported_output_version",
        historical.to_xml(),
    );
    let mut streamed = Vec::new();
    assert_unsupported_version(
        "Ndm::write_xml_to",
        "1.0",
        "generation.unsupported_output_version",
        historical.write_xml_to(&mut streamed),
    );
    assert!(
        streamed.is_empty(),
        "unsupported streaming generation wrote partial output"
    );

    let historical = Message::Opm(historical);
    assert_unsupported_version(
        "Message::to_xml",
        "1.0",
        "generation.unsupported_output_version",
        historical.to_xml(),
    );
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let path = directory.path().join("unsupported-opm.xml");
    assert_unsupported_version(
        "Message::to_xml_file",
        "1.0",
        "generation.unsupported_output_version",
        historical.to_xml_file(&path),
    );
    assert!(
        !path.exists(),
        "unsupported file generation created an output file"
    );
}

#[test]
fn opm_3_xml_file_generation_reports_output_failures_without_a_field_path() {
    let opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    let directory = tempfile::tempdir().expect("failed to create temporary directory");

    let error = Message::Opm(opm)
        .to_xml_file(directory.path())
        .expect_err("writing XML to a directory must fail");

    assert_eq!(error.code(), Some("io.error"));
    assert_eq!(error.field_path(), None);
}

#[test]
fn opm_3_xml_generation_reports_an_invalid_root_id() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.id = Some("WRONG_ID".into());

    assert_invalid_value_diagnostic("id", opm.to_xml(), "id");
}

#[test]
fn opm_3_xml_generation_rejects_strings_that_cannot_appear_in_xml() {
    type Mutation = (&'static str, &'static str, usize, fn(&mut Opm));
    let mutations: [Mutation; 21] = [
        ("header COMMENT", "header.comment", 0, |opm| {
            opm.header.comment.push("\u{1}".into())
        }),
        ("CLASSIFICATION", "header.classification", 0, |opm| {
            opm.header.classification = Some("\u{1}".into())
        }),
        ("ORIGINATOR", "header.originator", 0, |opm| {
            opm.header.originator.push('\u{1}')
        }),
        ("MESSAGE_ID", "header.message_id", 0, |opm| {
            opm.header.message_id = Some("\u{1}".into())
        }),
        (
            "metadata COMMENT",
            "body.segment.metadata.comment",
            0,
            |opm| opm.body.segment.metadata.comment.push("\u{1}".into()),
        ),
        (
            "OBJECT_NAME",
            "body.segment.metadata.object_name",
            0,
            |opm| opm.body.segment.metadata.object_name.push('\u{1}'),
        ),
        ("OBJECT_ID", "body.segment.metadata.object_id", 0, |opm| {
            opm.body.segment.metadata.object_id.push('\u{1}')
        }),
        (
            "CENTER_NAME",
            "body.segment.metadata.center_name",
            0,
            |opm| opm.body.segment.metadata.center_name.push('\u{1}'),
        ),
        ("REF_FRAME", "body.segment.metadata.ref_frame", 0, |opm| {
            opm.body.segment.metadata.ref_frame.push('\u{1}')
        }),
        (
            "TIME_SYSTEM",
            "body.segment.metadata.time_system",
            0,
            |opm| opm.body.segment.metadata.time_system.push('\u{1}'),
        ),
        ("data COMMENT", "body.segment.data.comment", 0, |opm| {
            opm.body.segment.data.comment.push("\u{1}".into())
        }),
        (
            "state-vector COMMENT",
            "body.segment.data.state_vector.comment",
            0,
            |opm| {
                opm.body
                    .segment
                    .data
                    .state_vector
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "Keplerian COMMENT",
            "body.segment.data.keplerian_elements.comment",
            1,
            |opm| {
                opm.body
                    .segment
                    .data
                    .keplerian_elements
                    .as_mut()
                    .unwrap()
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "spacecraft COMMENT",
            "body.segment.data.spacecraft_parameters.comment",
            0,
            |opm| {
                opm.body
                    .segment
                    .data
                    .spacecraft_parameters
                    .as_mut()
                    .unwrap()
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "covariance COMMENT",
            "body.segment.data.covariance_matrix.comment",
            3,
            |opm| {
                opm.body
                    .segment
                    .data
                    .covariance_matrix
                    .as_mut()
                    .unwrap()
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "COV_REF_FRAME",
            "body.segment.data.covariance_matrix.cov_ref_frame",
            3,
            |opm| {
                opm.body
                    .segment
                    .data
                    .covariance_matrix
                    .as_mut()
                    .unwrap()
                    .cov_ref_frame = Some("\u{1}".into())
            },
        ),
        (
            "maneuver COMMENT",
            "body.segment.data.maneuver_parameters[0].comment",
            1,
            |opm| {
                opm.body.segment.data.maneuver_parameters[0]
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "MAN_REF_FRAME",
            "body.segment.data.maneuver_parameters[0].man_ref_frame",
            1,
            |opm| {
                opm.body.segment.data.maneuver_parameters[0]
                    .man_ref_frame
                    .push('\u{1}')
            },
        ),
        (
            "USER_DEFINED parameter",
            "body.segment.data.user_defined_parameters.user_defined.parameter",
            3,
            |opm| {
                opm.body
                    .segment
                    .data
                    .user_defined_parameters
                    .as_mut()
                    .unwrap()
                    .user_defined[0]
                    .parameter
                    .push('\u{1}')
            },
        ),
        (
            "user-defined COMMENT",
            "body.segment.data.user_defined_parameters.comment",
            3,
            |opm| {
                opm.body
                    .segment
                    .data
                    .user_defined_parameters
                    .as_mut()
                    .unwrap()
                    .comment
                    .push("\u{1}".into())
            },
        ),
        (
            "USER_DEFINED value",
            "body.segment.data.user_defined_parameters.user_defined.value",
            3,
            |opm| {
                opm.body
                    .segment
                    .data
                    .user_defined_parameters
                    .as_mut()
                    .unwrap()
                    .user_defined[0]
                    .value
                    .push('\u{1}')
            },
        ),
    ];

    for (field, path, fixture, mutate) in mutations {
        let mut opm =
            Opm::from_kvn(OPM_3_KVN_FIXTURES[fixture].1).expect("failed to parse OPM fixture");
        mutate(&mut opm);
        assert_invalid_value_diagnostic(field, opm.to_xml(), path);
    }
}

#[test]
fn opm_3_xml_generation_accepts_xml_1_0_text_boundaries() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[0].1).expect("failed to parse OPM fixture");
    opm.header.comment.push("\t\n\r \u{FFFD}\u{10000}".into());

    opm.to_xml()
        .expect("generation rejected characters permitted by XML 1.0");
}

#[test]
fn opm_3_xml_generation_handles_extreme_finite_values_in_every_numeric_block() {
    let mut opm = Opm::from_kvn(OPM_3_KVN_FIXTURES[1].1).expect("failed to parse maneuver fixture");
    let optional_blocks =
        Opm::from_kvn(OPM_3_KVN_FIXTURES[3].1).expect("failed to parse covariance fixture");
    opm.body.segment.data.covariance_matrix = optional_blocks.body.segment.data.covariance_matrix;
    opm.body.segment.data.user_defined_parameters =
        optional_blocks.body.segment.data.user_defined_parameters;

    let state = &mut opm.body.segment.data.state_vector;
    state.x.value = f64::MAX;
    state.y.value = -f64::MAX;
    state.z.value = f64::MAX;
    state.x_dot.value = -f64::MAX;
    state.y_dot.value = f64::MAX;
    state.z_dot.value = -f64::MAX;

    let elements = opm.body.segment.data.keplerian_elements.as_mut().unwrap();
    elements.semi_major_axis.value = f64::MAX;
    elements.eccentricity.value = f64::MAX;
    elements.inclination.angle.value = 180.0;
    elements.ra_of_asc_node.value = -360.0;
    elements.arg_of_pericenter.value = 359.999;
    elements.true_anomaly.as_mut().unwrap().value = -360.0;
    elements.gm.value = f64::MAX;

    let spacecraft = opm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_mut()
        .unwrap();
    spacecraft.mass.as_mut().unwrap().value = f64::MAX;
    spacecraft.solar_rad_area.as_mut().unwrap().value = f64::MAX;
    spacecraft.solar_rad_coeff.as_mut().unwrap().value = f64::MAX;
    spacecraft.drag_area.as_mut().unwrap().value = f64::MAX;
    spacecraft.drag_coeff.as_mut().unwrap().value = f64::MAX;

    macro_rules! set_covariance_to_max {
        ($covariance:expr, $($field:ident),+ $(,)?) => {
            $($covariance.$field.value = f64::MAX;)+
        };
    }
    let covariance = opm.body.segment.data.covariance_matrix.as_mut().unwrap();
    set_covariance_to_max!(
        covariance,
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
    );

    for maneuver in &mut opm.body.segment.data.maneuver_parameters {
        maneuver.man_duration.value = f64::MAX;
        maneuver.man_delta_mass.value = -f64::MAX;
        maneuver.man_dv_1.value = f64::MAX;
        maneuver.man_dv_2.value = -f64::MAX;
        maneuver.man_dv_3.value = f64::MAX;
    }

    let xml = opm
        .to_xml()
        .expect("extreme finite values must not panic or fail generation");
    validate_with_official_xsd("extreme finite values", &xml);
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
    validate_with_official_xsd("zero MAN_DELTA_MASS", &xml);

    zero_delta_mass.body.segment.data.maneuver_parameters[0]
        .man_delta_mass
        .value = 1.0;
    assert_out_of_range_diagnostic(
        "MAN_DELTA_MASS",
        zero_delta_mass.to_xml(),
        "body.segment.data.maneuver_parameters[0].man_delta_mass",
    );
}

fn assert_non_finite_rejected<T: std::fmt::Debug>(surface: &str, result: Result<T>) {
    let error = result.expect_err(surface);
    let validation = error
        .as_validation_error()
        .unwrap_or_else(|| panic!("{surface} returned a non-validation error: {error}"));
    assert!(
        matches!(
            validation_error_source(validation),
            ValidationError::InvalidValue { expected, .. }
                if expected.as_ref() == "a finite number"
        ),
        "{surface} returned the wrong validation error: {validation}"
    );
}

fn validation_error_source(error: &ValidationError) -> &ValidationError {
    match error {
        ValidationError::AtPath { source, .. } => validation_error_source(source),
        error => error,
    }
}

fn assert_invalid_value_diagnostic<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.invalid_value"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    assert!(error.as_validation_error().is_some());
}

fn assert_out_of_range_diagnostic<T: std::fmt::Debug>(
    surface: &str,
    result: Result<T>,
    expected_path: &str,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some("validation.out_of_range"),
        "{surface} returned an unstable top-level diagnostic code"
    );
    assert_eq!(
        error.field_path().as_deref(),
        Some(expected_path),
        "{surface} returned an incomplete top-level field path"
    );
    assert!(error.as_validation_error().is_some());
}

fn assert_unsupported_version<T: std::fmt::Debug>(
    surface: &str,
    expected_edition: &str,
    expected_code: &str,
    result: Result<T>,
) {
    let error = result.expect_err(surface);
    assert_eq!(
        error.code(),
        Some(expected_code),
        "{surface} returned an unstable diagnostic code"
    );
    assert_eq!(
        error.field_path(),
        None,
        "{surface} invented a field path for an operation-level failure"
    );
    let diagnostic = error
        .diagnostic()
        .unwrap_or_else(|| panic!("{surface} omitted generation context: {error}"));
    assert_eq!(
        diagnostic.message_kind,
        ccsds_ndm::validation::MessageKind::Opm
    );
    assert_eq!(
        diagnostic.notation,
        ccsds_ndm::error::DiagnosticNotation::Xml
    );
    assert_eq!(diagnostic.source_edition, Some(expected_edition));
}
