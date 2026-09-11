// SPDX-FileCopyrightText: 2026 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

use crate::common::validate_xml;
use crate::{
    assert_invalid_value_diagnostic, assert_missing_required, opm, opm_with_maneuvers,
    validation_error_source, OPM_3_KVN_FIXTURES, OPM_3_XML_FIXTURES,
};
use ccsds_ndm::error::{Result, ValidationError};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::types::{CalendarEpoch, GmUnits};
use ccsds_ndm::{Message, Ndm};
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn public_opm_xml_generation_signatures_remain_compatible() {
    let _typed: fn(&Opm) -> ccsds_ndm::error::Result<String> = <Opm as Ndm>::to_xml;
    let _typed_streaming: fn(&Opm, &mut Vec<u8>) -> ccsds_ndm::error::Result<()> =
        <Opm as Ndm>::write_xml_to::<Vec<u8>>;
    let _generic: fn(&Message) -> ccsds_ndm::error::Result<String> = Message::to_xml;
    let _generic_file: fn(&Message, PathBuf) -> ccsds_ndm::error::Result<()> =
        Message::to_xml_file::<PathBuf>;
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
        validate_xml(name, &xml);
    }

    for (name, xml) in OPM_3_XML_FIXTURES {
        let opm =
            Opm::from_xml(xml).unwrap_or_else(|error| panic!("failed to parse {name}: {error}"));
        assert_eq!(opm.version, "3.0", "{name} is not an OPM 3.0 fixture");
        let generated = opm
            .to_xml()
            .unwrap_or_else(|error| panic!("failed to regenerate XML for {name}: {error}"));
        validate_xml(name, &generated);
    }
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
    validate_xml("extreme finite values", &xml);
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

fn assignment_keys(kvn: &str) -> Vec<&str> {
    kvn.lines()
        .filter(|line| !line.starts_with("COMMENT"))
        .filter_map(|line| line.split_once('=').map(|(key, _)| key.trim()))
        .collect()
}

fn assignment_units(kvn: &str) -> Vec<(&str, Option<&str>)> {
    kvn.lines()
        .filter(|line| !line.starts_with("COMMENT"))
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            let value = value.trim();
            let unit = value
                .strip_suffix(']')
                .and_then(|value| value.rsplit_once('['))
                .map(|(_, unit)| unit);
            Some((key.trim(), unit))
        })
        .collect()
}

fn comments(kvn: &str) -> Vec<&str> {
    kvn.lines()
        .filter_map(|line| line.strip_prefix("COMMENT "))
        .collect()
}

fn structural_lines(kvn: &str) -> Vec<String> {
    kvn.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            if line.starts_with("COMMENT ") {
                line.to_owned()
            } else {
                line.split_once('=')
                    .map_or_else(|| line.to_owned(), |(key, _)| key.trim().to_owned())
            }
        })
        .collect()
}

fn significant_digits(value: &str) -> usize {
    let digits: Vec<_> = value
        .bytes()
        .take_while(|byte| *byte != b'e' && *byte != b'E')
        .filter(u8::is_ascii_digit)
        .collect();
    let first = digits.iter().position(|digit| *digit != b'0');
    let last = digits.iter().rposition(|digit| *digit != b'0');
    match (first, last) {
        (Some(first), Some(last)) => last - first + 1,
        _ => 1,
    }
}

#[test]
fn annex_g_opm_fixtures_generate_printable_ordered_kvn_with_preserved_units() {
    for source in [
        include_str!("../../data/kvn/opm_g1.kvn"),
        include_str!("../../data/kvn/opm_g2.kvn"),
        include_str!("../../data/kvn/opm_g3.kvn"),
        include_str!("../../data/kvn/opm_g4.kvn"),
    ] {
        let generated = Opm::from_kvn(source)
            .expect("Annex G OPM fixture should parse")
            .to_kvn()
            .expect("Annex G OPM should generate");

        assert_eq!(assignment_keys(&generated), assignment_keys(source));
        assert_eq!(assignment_units(&generated), assignment_units(source));
        assert_eq!(comments(&generated), comments(source));
        assert_eq!(structural_lines(&generated), structural_lines(source));
        assert!(generated.lines().all(|line| {
            line.len() <= 254 && line.bytes().all(|byte| (b' '..=b'~').contains(&byte))
        }));
        for value in generated.lines().filter_map(|line| {
            let (_, value) = line.split_once('=')?;
            let number = value
                .trim()
                .split_once(" [")
                .map_or(value.trim(), |(number, _)| number);
            number.parse::<f64>().ok().map(|_| number)
        }) {
            assert!(
                value.contains('.') && significant_digits(value) <= 16,
                "non-compliant numeric spelling: {value}"
            );
        }
    }
}

#[test]
fn opm_kvn_preserves_exact_extreme_values_when_the_odm_can_represent_them() {
    let mut message = opm();
    message.body.segment.data.state_vector.x.value = f64::from_bits(1);

    let generated = message
        .to_kvn()
        .expect("minimum subnormal is representable");
    assert!(generated.contains("X                    = 5.0e-324"));
    let reparsed = Opm::from_kvn(&generated).expect("generated KVN should parse");
    assert_eq!(
        reparsed.body.segment.data.state_vector.x.value.to_bits(),
        f64::from_bits(1).to_bits()
    );
}

#[test]
fn opm_kvn_rounds_values_that_need_seventeen_digits() {
    let mut message = opm();
    message.body.segment.data.state_vector.x.value = 1.234_567_890_123_456_7;

    let generated = message.to_kvn().expect("finite value should be rounded");
    assert!(generated.contains("X                    = 1.234567890123457e0"));

    let mut output = Vec::new();
    message
        .write_kvn_to(&mut output)
        .expect("streaming should use the same CCSDS rounding");
    assert_eq!(output, generated.as_bytes());

    message
        .to_xml()
        .expect("the KVN-specific precision rule must not affect XML");
}

#[test]
fn opm_kvn_rounds_numbers_in_every_optional_numeric_block() {
    const LOSSY: f64 = 1.234_567_890_123_456_7;
    type Mutation = (&'static str, &'static str, fn(&mut Opm));
    let mutations: [Mutation; 4] = [
        (
            include_str!("../../data/kvn/opm_g2.kvn"),
            "body.segment.data.keplerian_elements.semi_major_axis",
            |message| {
                message
                    .body
                    .segment
                    .data
                    .keplerian_elements
                    .as_mut()
                    .unwrap()
                    .semi_major_axis
                    .value = LOSSY
            },
        ),
        (
            include_str!("../../data/kvn/opm_g2.kvn"),
            "body.segment.data.spacecraft_parameters.mass",
            |message| {
                message
                    .body
                    .segment
                    .data
                    .spacecraft_parameters
                    .as_mut()
                    .unwrap()
                    .mass
                    .as_mut()
                    .unwrap()
                    .value = LOSSY
            },
        ),
        (
            include_str!("../../data/kvn/opm_g4.kvn"),
            "body.segment.data.covariance_matrix.cx_x",
            |message| {
                message
                    .body
                    .segment
                    .data
                    .covariance_matrix
                    .as_mut()
                    .unwrap()
                    .cx_x
                    .value = LOSSY
            },
        ),
        (
            include_str!("../../data/kvn/opm_g2.kvn"),
            "body.segment.data.maneuver_parameters.man_duration",
            |message| {
                message.body.segment.data.maneuver_parameters[0]
                    .man_duration
                    .value = LOSSY
            },
        ),
    ];

    for (source, _path, mutate) in mutations {
        let mut message = Opm::from_kvn(source).expect("fixture should parse");
        mutate(&mut message);

        assert!(message
            .to_kvn()
            .expect("finite value should be rounded")
            .contains("1.234567890123457e0"));
        message
            .to_xml()
            .expect("the KVN-specific precision rule must not affect XML");
    }
}

#[test]
fn opm_kvn_canonicalizes_gm_units_to_the_odm_spelling() {
    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g2.kvn")).expect("fixture should parse");
    message
        .body
        .segment
        .data
        .keplerian_elements
        .as_mut()
        .unwrap()
        .gm
        .units = Some(GmUnits::KM3PerS2);

    // ODM 7.7.1 admits only the keyword table spelling in KVN, so the uppercase spelling the
    // XML schema also permits is rewritten rather than rejected.
    let kvn = message
        .to_kvn()
        .expect("uppercase GM units should be canonicalized, not rejected");
    assert!(kvn.contains("[km**3/s**2]"), "{kvn}");
    assert!(!kvn.contains("KM**3/S**2"), "{kvn}");

    // The uppercase spelling stays valid on the XML side.
    assert!(message
        .to_xml()
        .expect("XML generation")
        .contains("KM**3/S**2"));
}

#[test]
fn opm_kvn_gm_units_survive_a_kvn_round_trip() {
    let source = include_str!("../../data/kvn/opm_g2.kvn").replace(
        "GM = 398600.4415 [km**3/s**2]",
        "GM = 398600.4415 [KM**3/S**2]",
    );
    let message = Opm::from_kvn(&source).expect("uppercase GM units should parse");
    let regenerated = message.to_kvn().expect("regeneration should succeed");
    Opm::from_kvn(&regenerated).expect("regenerated KVN should parse");
}

#[test]
fn opm_kvn_is_identical_across_public_generation_entry_points() {
    let message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
    let expected = message.to_kvn().expect("typed generation should succeed");

    assert_eq!(
        message
            .to_kvn()
            .expect("versioned generation should succeed"),
        expected
    );

    let mut streamed = Vec::new();
    message
        .write_kvn_to(&mut streamed)
        .expect("streaming generation should succeed");
    assert_eq!(streamed, expected.as_bytes());

    let erased = Message::Opm(message);
    assert_eq!(
        erased
            .to_kvn()
            .expect("type-erased generation should succeed"),
        expected
    );
    assert_eq!(
        erased
            .to_kvn()
            .expect("type-erased versioned generation should succeed"),
        expected
    );

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let path = directory.path().join("opm.kvn");
    erased
        .to_kvn_file(&path)
        .expect("file generation should succeed");
    assert_eq!(
        std::fs::read(path).expect("generated file should be readable"),
        expected.as_bytes()
    );
}

#[test]
fn opm_kvn_rejects_non_ascii_and_control_characters() {
    for invalid in ["ESOC 🚀", "ESOC\tFlight Dynamics", "ESOC\nFlight Dynamics"] {
        let mut message = opm();
        message.header.originator = invalid.to_owned();

        let error = message
            .to_kvn()
            .expect_err("invalid KVN text must be rejected");
        assert_eq!(error.code(), Some("validation.invalid_value"));
        assert_eq!(error.field_path().as_deref(), Some("header.originator"));
    }
}

#[test]
fn opm_kvn_rejects_invalid_user_defined_keyword_suffixes() {
    for invalid in [
        "",
        "earth_model",
        "EARTH MODEL",
        "EARTH=MODEL",
        "ÉARTH_MODEL",
    ] {
        let mut message =
            Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
        message
            .body
            .segment
            .data
            .user_defined_parameters
            .as_mut()
            .unwrap()
            .user_defined[0]
            .parameter = invalid.to_owned();

        let error = message
            .to_kvn()
            .expect_err("KVN keywords must be uppercase, non-empty, and contain no blanks or =");
        assert_eq!(error.code(), Some("validation.invalid_value"));
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment.data.user_defined_parameters.user_defined.parameter")
        );
    }
}

#[test]
fn opm_kvn_accepts_normative_user_defined_keyword_forms() {
    for valid in [
        "A",
        "C3",
        "3RD_BODY_PERTURBATION",
        "A_B2",
        "EARTH-MODEL",
        "EARTH.MODEL",
    ] {
        let mut message =
            Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
        message
            .body
            .segment
            .data
            .user_defined_parameters
            .as_mut()
            .unwrap()
            .user_defined[0]
            .parameter = valid.to_owned();

        let generated = message.to_kvn().expect("valid KVN keyword should generate");
        assert!(generated.contains(&format!("USER_DEFINED_{valid}")));
        Opm::from_kvn(&generated).expect("generated keyword should parse");
    }
}

#[test]
fn opm_kvn_enforces_the_254_character_line_limit() {
    let mut message = opm();
    message.header.comment = vec!["A".repeat(246)];

    let generated = message
        .to_kvn()
        .expect("a 254-character COMMENT line should be valid");
    assert_eq!(generated.lines().nth(1).unwrap().len(), 254);

    message.header.comment = vec!["A".repeat(247)];
    let error = message
        .to_kvn()
        .expect_err("a 255-character COMMENT line must be rejected");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert_eq!(error.field_path().as_deref(), Some("header.comment"));
}

#[test]
fn opm_kvn_enforces_the_line_limit_for_assignment_values() {
    let mut message = opm();
    message.header.originator = "A".repeat(231);

    let generated = message
        .to_kvn()
        .expect("a 254-character assignment line should be valid");
    let originator = generated
        .lines()
        .find(|line| line.starts_with("ORIGINATOR"))
        .expect("generated KVN should contain ORIGINATOR");
    assert_eq!(originator.len(), 254);

    message.header.originator.push('A');
    let error = message
        .to_kvn()
        .expect_err("a 255-character assignment line must be rejected");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert_eq!(error.field_path().as_deref(), Some("header.originator"));
}

#[test]
fn opm_kvn_attributes_an_oversized_user_defined_keyword_to_its_parameter() {
    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g4.kvn")).expect("fixture should parse");
    let parameter = &mut message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_mut()
        .unwrap()
        .user_defined[0];
    parameter.parameter = "A".repeat(237);
    parameter.value = "V".to_owned();

    let generated = message
        .to_kvn()
        .expect("a 254-character user-defined line should be valid");
    let user_defined = generated
        .lines()
        .find(|line| line.starts_with("USER_DEFINED_"))
        .expect("generated KVN should contain user-defined data");
    assert_eq!(user_defined.len(), 254);

    message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_mut()
        .unwrap()
        .user_defined[0]
        .parameter = "A".repeat(239);
    let error = message
        .to_kvn()
        .expect_err("a keyword that exceeds the line limit must be rejected");
    assert_eq!(error.code(), Some("validation.out_of_range"));
    assert_eq!(
        error.field_path().as_deref(),
        Some("body.segment.data.user_defined_parameters.user_defined.parameter")
    );
}

#[test]
fn opm_kvn_epoch_lines_are_bounded_by_the_epoch_type() {
    let longest_epoch = format!("2000-001T00:00:00.{}", "0".repeat(46));
    assert_eq!(longest_epoch.len(), 64);
    let epoch = CalendarEpoch::from_str(&longest_epoch).expect("64-byte epoch should be valid");
    assert!(CalendarEpoch::from_str(&format!("{longest_epoch}0")).is_err());

    let mut message =
        Opm::from_kvn(include_str!("../../data/kvn/opm_g2.kvn")).expect("fixture should parse");
    message.header.creation_date = epoch;
    message.body.segment.metadata.ref_frame_epoch = Some(epoch);
    message.body.segment.data.state_vector.epoch = epoch;
    message.body.segment.data.maneuver_parameters[0].man_epoch_ignition = epoch;

    let generated = message
        .to_kvn()
        .expect("maximum-width epochs should generate");
    assert!(generated.lines().all(|line| line.len() <= 254));
}

#[test]
fn invalid_opm_kvn_is_rejected_across_public_generation_entry_points() {
    let mut message = opm();
    message.header.originator = "ESOC 🚀".to_owned();

    assert!(message.to_kvn().is_err());
    assert!(message.to_kvn().is_err());

    let mut output = Vec::new();
    let error = message
        .write_kvn_to(&mut output)
        .expect_err("invalid KVN text must be rejected");
    assert_eq!(error.code(), Some("validation.invalid_value"));
    assert_eq!(error.field_path().as_deref(), Some("header.originator"));
    assert!(output.is_empty());

    let erased = Message::Opm(message);
    assert!(erased.to_kvn().is_err());
    assert!(erased.to_kvn().is_err());

    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let path = directory.path().join("opm.kvn");
    std::fs::write(&path, b"unchanged").expect("sentinel should be written");
    assert!(erased.to_kvn_file(&path).is_err());
    assert_eq!(
        std::fs::read(path).expect("sentinel should remain readable"),
        b"unchanged"
    );
}

const ROOT: &str = concat!(
    "<opm xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" ",
    "id=\"CCSDS_OPM_VERS\" version=\"3.0\">"
);

fn assert_opm_envelope(xml: &str) {
    let body = xml
        .strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
        .expect("generated XML omitted the required declaration");
    assert!(
        body.starts_with(ROOT),
        "generated OPM root did not contain the required namespace followed by final id/version attributes: {xml}"
    );
}

#[test]
fn typed_opm_string_generation_emits_the_required_root_namespace() {
    let opm = opm();
    let xml = opm.to_xml().expect("typed XML generation failed");
    assert_opm_envelope(&xml);
    validate_xml("OPM envelope", &xml);
}

fn calendar_epoch(value: &str) -> CalendarEpoch {
    CalendarEpoch::new(value)
        .unwrap_or_else(|error| panic!("test calendar epoch {value:?} was rejected: {error}"))
}

#[test]
fn xml_generation_accepts_calendar_ordinal_and_leap_second_boundaries() {
    for value in [
        "2000-02-29T23:59:60Z",
        "2000-366T23:59:59.1Z",
        "2001-365T00:00:00",
    ] {
        let mut message = opm_with_maneuvers();
        message.header.creation_date = calendar_epoch(value);
        message.body.segment.metadata.ref_frame_epoch = Some(calendar_epoch(value));
        message.body.segment.data.state_vector.epoch = calendar_epoch(value);
        message.body.segment.data.maneuver_parameters[0].man_epoch_ignition = calendar_epoch(value);
        message
            .to_xml()
            .unwrap_or_else(|error| panic!("valid boundary {value:?} was rejected: {error}"));
    }
}
