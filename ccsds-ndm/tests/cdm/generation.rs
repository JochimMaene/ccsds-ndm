use crate::common::{assert_rejects, validate_xml};
use crate::KVN;
use ccsds_ndm::messages::cdm::Cdm;
use ccsds_ndm::Ndm;
#[test]
fn edited_shared_od_parameters_are_revalidated_in_cdm() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    message.body.segments[1]
        .data
        .od_parameters
        .as_mut()
        .unwrap()
        .weighted_rms
        .as_mut()
        .unwrap()
        .value = f64::NAN;

    assert_rejects(&message, "WEIGHTED_RMS");
}

#[test]
fn edited_cdm_numeric_values_are_validated_before_any_output() {
    for field in [
        "MASS",
        "AREA_PC",
        "AREA_DRG",
        "AREA_SRP",
        "CD_AREA_OVER_MASS",
        "CR_AREA_OVER_MASS",
        "SEDR",
        "THRUST_ACCELERATION",
        "COLLISION_PROBABILITY",
    ] {
        for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            if field == "THRUST_ACCELERATION" && value == -1.0 {
                continue; // The schema permits signed acceleration.
            }
            let mut message = Cdm::from_kvn(KVN).unwrap();
            let parameters = message.body.segments[0]
                .data
                .additional_parameters
                .as_mut()
                .unwrap();
            parameters.area_drg = parameters.area_pc.clone();
            parameters.area_srp = parameters.area_pc.clone();
            let target = match field {
                "MASS" => &mut parameters.mass.as_mut().unwrap().value,
                "AREA_PC" => &mut parameters.area_pc.as_mut().unwrap().value,
                "AREA_DRG" => &mut parameters.area_drg.as_mut().unwrap().value,
                "AREA_SRP" => &mut parameters.area_srp.as_mut().unwrap().value,
                "CD_AREA_OVER_MASS" => &mut parameters.cd_area_over_mass.as_mut().unwrap().value,
                "CR_AREA_OVER_MASS" => &mut parameters.cr_area_over_mass.as_mut().unwrap().value,
                "SEDR" => &mut parameters.sedr.as_mut().unwrap().value,
                "THRUST_ACCELERATION" => {
                    &mut parameters.thrust_acceleration.as_mut().unwrap().value
                }
                _ => {
                    &mut message
                        .body
                        .relative_metadata_data
                        .collision_probability
                        .as_mut()
                        .unwrap()
                        .value
                }
            };
            *target = value;
            assert_rejects(&message, field);
        }
    }
    for probability in [0.0, 1.0] {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        message
            .body
            .relative_metadata_data
            .collision_probability
            .as_mut()
            .unwrap()
            .value = probability;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .mass
            .as_mut()
            .unwrap()
            .value = 0.0;
        message.body.segments[0]
            .data
            .additional_parameters
            .as_mut()
            .unwrap()
            .thrust_acceleration
            .as_mut()
            .unwrap()
            .value = -1.0;
        validate_xml("numeric boundaries", &message.to_xml().unwrap());
        assert_eq!(Cdm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);
    }
}

#[test]
fn every_kvn_generation_gate_rejects_loss_or_ambiguity_before_output() {
    type CdmMutation = fn(&mut Cdm);
    let cases: [(&str, CdmMutation); 2] = [
        ("only printable ASCII records", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE €".to_owned();
        }),
        ("only printable ASCII records", |message| {
            message.body.segments[0].metadata.object_name = "SATELLITE\nA".to_owned();
        }),
    ];
    for (label, mutate) in cases {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        mutate(&mut message);
        let error = message.to_kvn().unwrap_err();
        assert!(error.to_string().contains(label), "{label}: {error}");
        let mut output = Vec::new();
        let streamed = message.write_kvn_to(&mut output).unwrap_err();
        assert_eq!(streamed.to_string(), error.to_string(), "{label}");
        assert!(output.is_empty(), "streaming wrote bytes for {label}");
    }
}

#[test]
fn cdm_kvn_rounds_computed_numbers_while_xml_retains_the_value() {
    let mut control = Cdm::from_kvn(KVN).unwrap();
    control.body.relative_metadata_data.miss_distance.value = 0.123_456_789_012_345_6;
    assert_eq!(Cdm::from_kvn(&control.to_kvn().unwrap()).unwrap(), control);

    for field in ["MISS_DISTANCE", "COLLISION_PROBABILITY", "X", "CR_R"] {
        let mut message = Cdm::from_kvn(KVN).unwrap();
        let value = 0.300_000_000_000_000_04;
        match field {
            "MISS_DISTANCE" => message.body.relative_metadata_data.miss_distance.value = value,
            "COLLISION_PROBABILITY" => {
                message
                    .body
                    .relative_metadata_data
                    .collision_probability
                    .as_mut()
                    .unwrap()
                    .value = value
            }
            "X" => message.body.segments[1].data.state_vector.x.value = value,
            _ => {
                message.body.segments[1]
                    .data
                    .covariance_matrix
                    .as_mut()
                    .unwrap()
                    .cr_r
                    .value = value
            }
        }
        message.validate().unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Cdm::from_xml(&xml).unwrap(), message);
        validate_xml("CDM full-precision double", &xml);
        let kvn = message.to_kvn().unwrap();
        let reparsed = Cdm::from_kvn(&kvn).unwrap();
        let rounded = match field {
            "MISS_DISTANCE" => reparsed.body.relative_metadata_data.miss_distance.value,
            "COLLISION_PROBABILITY" => {
                reparsed
                    .body
                    .relative_metadata_data
                    .collision_probability
                    .as_ref()
                    .unwrap()
                    .value
            }
            "X" => reparsed.body.segments[1].data.state_vector.x.value,
            _ => {
                reparsed.body.segments[1]
                    .data
                    .covariance_matrix
                    .as_ref()
                    .unwrap()
                    .cr_r
                    .value
            }
        };
        assert_eq!(rounded, 0.3, "{field}");
        let mut output = Vec::new();
        message.write_kvn_to(&mut output).unwrap();
        assert_eq!(output, kvn.as_bytes(), "{field}");
        assert_eq!(
            ccsds_ndm::Message::Cdm(message.clone()).to_kvn().unwrap(),
            kvn
        );
    }
}

#[test]
fn generation_rejects_partial_optional_covariance_rows() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    let covariance = message.body.segments[0]
        .data
        .covariance_matrix
        .as_mut()
        .unwrap();
    covariance.cdrg_t = None;

    let error = message
        .to_xml()
        .expect_err("a partial optional covariance row violates CCSDS 508.0-B-1 section 5.2.8");
    assert_eq!(error.code(), Some("validation.missing_required_field"));
    assert!(error.to_string().contains("CDRG_T"));
}

/// Every relative metadata number must be finite. Before
/// this was routed, `validate` and the XML writer accepted a non-finite value while the KVN
/// writer rejected it, and an infinity reached the document as the lexical `inf`, which `xmllint`
/// rejects as "not a valid value of the atomic type 'xs:double'".
#[test]
fn edited_cdm_relative_metadata_numbers_are_revalidated_before_output() {
    type Mutation = fn(&mut Cdm, f64);
    let cases: [(&str, Mutation); 5] = [
        ("MISS_DISTANCE", |cdm, value| {
            cdm.body.relative_metadata_data.miss_distance.value = value
        }),
        ("RELATIVE_SPEED", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_speed
                .as_mut()
                .expect("fixture has RELATIVE_SPEED")
                .value = value
        }),
        ("RELATIVE_POSITION_T", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_state_vector
                .as_mut()
                .expect("fixture has a relative state vector")
                .relative_position_t
                .value = value
        }),
        ("RELATIVE_VELOCITY_N", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .relative_state_vector
                .as_mut()
                .expect("fixture has a relative state vector")
                .relative_velocity_n
                .value = value
        }),
        ("SCREEN_VOLUME_Y", |cdm, value| {
            cdm.body
                .relative_metadata_data
                .screen_volume_y
                .as_mut()
                .expect("fixture has SCREEN_VOLUME_Y")
                .value = value
        }),
    ];

    for (field, mutate) in cases {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut cdm = Cdm::from_kvn(KVN).expect("fixture should parse");
            mutate(&mut cdm, value);

            assert_rejects(&cdm, field);
        }
    }

    // A finite replacement still reaches the reference schema.
    let mut cdm = Cdm::from_kvn(KVN).expect("fixture should parse");
    cdm.body.relative_metadata_data.miss_distance.value = 0.0;
    cdm.validate().expect("zero miss distance is representable");
    validate_xml("CDM relative metadata boundary", &cdm.to_xml().unwrap());
}

#[test]
fn cdm_kvn_comments_keep_their_normative_block_association() {
    let cdm = Cdm::from_kvn(KVN).unwrap();

    assert_eq!(
        cdm.body.relative_metadata_data.comment,
        ["Relative Metadata/Data"]
    );

    let first = &cdm.body.segments[0];
    assert_eq!(first.metadata.comment, ["Object1 Metadata"]);
    // KVN has no delimiter between the outer data comments and the first nested block's
    // comments. Preserve the leading run and its position without guessing a split.
    assert_eq!(
        first.data.comment,
        ["Object1 Data", "Object1 OD Parameters"]
    );
    let od = first.data.od_parameters.as_ref().unwrap();
    assert!(od.comment.is_empty());
    assert!(od.time_lastob_start.is_some());
    assert_eq!(
        first.data.additional_parameters.as_ref().unwrap().comment,
        [
            "Object1 Additional Parameters",
            "Apogee Altitude=779 km",
            "Perigee Altitude=765 km",
            "Inclination=86.4 deg",
        ]
    );
    assert_eq!(first.data.state_vector.comment, ["Object1 State Vector"]);
    assert_eq!(
        first.data.covariance_matrix.as_ref().unwrap().comment,
        ["Object1 Covariance in the RTN Coordinate Frame"]
    );

    let regenerated = cdm.to_kvn().unwrap();
    assert_eq!(Cdm::from_kvn(&regenerated).unwrap(), cdm);
}

#[test]
fn xml_nested_comments_merge_canonically_into_kvn() {
    // Space-Track layout: empty <data> COMMENT, a comment-only odParameters (all leaves nil),
    // then commented additionalParameters. KVN has one leading run for all of them.
    let mut xml = Cdm::from_kvn(KVN).unwrap();
    let data = &mut xml.body.segments[0].data;
    data.comment.clear();
    data.od_parameters = Some(ccsds_ndm::common::OdParameters {
        comment: vec!["od".into()],
        ..Default::default()
    });
    data.additional_parameters.as_mut().unwrap().comment = vec!["additional".into()];
    data.state_vector.comment = vec!["state".into()];
    // Second segment: comments after a valued first block stay with their own block.
    let second = &mut xml.body.segments[1].data;
    second.comment = vec!["outer".into()];
    second.od_parameters.as_mut().unwrap().comment = vec!["od2".into()];
    second.additional_parameters = None;
    second.state_vector.comment = vec!["state2".into()];
    let xml = Cdm::from_xml(&xml.to_xml().unwrap()).unwrap();

    let kvn = xml.to_kvn().unwrap();
    let merged = Cdm::from_kvn(&kvn).unwrap();
    let first = &merged.body.segments[0].data;
    assert_eq!(first.comment, ["od", "additional"]);
    assert!(first.od_parameters.is_none());
    assert!(first
        .additional_parameters
        .as_ref()
        .unwrap()
        .comment
        .is_empty());
    assert_eq!(first.state_vector.comment, ["state"]);
    let second = &merged.body.segments[1].data;
    assert_eq!(second.comment, ["outer", "od2"]);
    assert!(second.od_parameters.as_ref().unwrap().comment.is_empty());
    assert_eq!(second.state_vector.comment, ["state2"]);
    assert_eq!(merged.to_kvn().unwrap(), kvn, "the merge is idempotent");
}

#[test]
fn kvn_comment_lines_are_written_without_trailing_blanks() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    message.header.comment = vec!["padded   ".into()];
    message.body.relative_metadata_data.miss_distance.value = 0.300_000_000_000_000_04;
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.contains("COMMENT padded\n"), "{kvn}");
    let parsed = Cdm::from_kvn(&kvn).unwrap();
    assert_eq!(parsed.body.relative_metadata_data.miss_distance.value, 0.3);
    assert_eq!(parsed.to_kvn().unwrap(), kvn);
    let mut streamed = Vec::new();
    message.write_kvn_to(&mut streamed).unwrap();
    assert_eq!(streamed, kvn.as_bytes());
}

#[test]
fn kvn_generation_supplies_fixed_units_missing_from_the_model() {
    let source = crate::KVN
        .lines()
        .map(|line| line.split_once(" [").map_or(line, |(value, _)| value))
        .collect::<Vec<_>>()
        .join("\n");
    let message = Cdm::from_kvn(&source).unwrap();
    assert!(message
        .body
        .relative_metadata_data
        .miss_distance
        .units
        .is_none());
    let kvn = message.to_kvn().unwrap();
    for (field, units) in [
        ("MISS_DISTANCE", "m"),
        ("RELATIVE_SPEED", "m/s"),
        ("RELATIVE_POSITION_R", "m"),
        ("RELATIVE_VELOCITY_R", "m/s"),
        ("RECOMMENDED_OD_SPAN", "d"),
        ("RESIDUALS_ACCEPTED", "%"),
        ("AREA_PC", "m**2"),
        ("MASS", "kg"),
        ("X", "km"),
        ("X_DOT", "km/s"),
        ("CR_R", "m**2"),
    ] {
        let line = kvn
            .lines()
            .find(|line| {
                line.split_once('=')
                    .is_some_and(|(key, _)| key.trim() == field)
            })
            .unwrap();
        assert!(line.ends_with(&format!("[{units}]")), "{line}");
    }
    let mut output = Vec::new();
    message.write_kvn_to(&mut output).unwrap();
    assert_eq!(output, kvn.as_bytes());
    let reparsed = Cdm::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed.to_kvn().unwrap(), kvn);
    crate::common::validate_xml("CDM supplied KVN units", &reparsed.to_xml().unwrap());
}

#[test]
fn cdm_xml_output_is_printable_ascii_with_bounded_lines() {
    let mut message = Cdm::from_kvn(KVN).unwrap();
    let xml = message.to_xml().unwrap();
    assert!(
        xml.lines().count() > 100,
        "XML output is not split into records"
    );
    assert!(xml.lines().all(|line| line.len() <= 254 && line.is_ascii()));
    validate_xml("CDM 6.2.2 output", &xml);
    // Input stays lenient: one long line with non-ASCII text still reads.
    let lenient = crate::XML
        .replace('\n', "")
        .replace("SATELLITE A", "SATÉLITE A");
    Cdm::from_xml(&lenient).unwrap();

    message.body.segments[0].metadata.object_name = "SATÉLITE".into();
    message.to_xml().expect_err("non-ASCII XML output");
    let mut output = Vec::new();
    message
        .write_xml_to(&mut output)
        .expect_err("non-ASCII XML output");
    assert!(output.is_empty());

    message.body.segments[0].metadata.object_name = "SATELLITE".into();
    message.header.comment = vec!["x".repeat(255)];
    message
        .write_xml_to(&mut output)
        .expect_err("overlong XML comment");
    assert!(output.is_empty());
}

#[test]
fn xml_generation_refuses_present_empty_optional_strings() {
    let message = Cdm::from_kvn(KVN).unwrap();
    for edit in [
        |m: &mut Cdm| m.header.message_for = Some(String::new()),
        |m: &mut Cdm| m.body.segments[0].metadata.operator_phone = Some("   ".into()),
        |m: &mut Cdm| {
            m.body.relative_metadata_data.collision_probability_method = Some(String::new())
        },
    ] {
        let mut edited = message.clone();
        edit(&mut edited);
        edited
            .to_xml()
            .expect_err("empty optional string would read back as None");
    }
}

#[test]
fn generation_enforces_the_international_designator_layout() {
    // CARA Alfano test cases use `1001`; reading keeps them, output does not.
    let kvn = crate::common::mutated_once(
        KVN,
        "INTERNATIONAL_DESIGNATOR = 1997-030E",
        "INTERNATIONAL_DESIGNATOR = 1001",
    );
    let mut message = Cdm::from_kvn(&kvn).unwrap();
    for error in [message.to_kvn().unwrap_err(), message.to_xml().unwrap_err()] {
        crate::common::assert_validation_field(&error, "INTERNATIONAL_DESIGNATOR");
    }
    message.body.segments[0].metadata.international_designator = "UNKNOWN".into();
    message.to_kvn().unwrap();
    message.to_xml().unwrap();
}
