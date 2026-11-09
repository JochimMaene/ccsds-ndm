use crate::common::mutated;
use crate::{KVN, XML};
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::types::ControlledType;
use ccsds_ndm::Ndm;

#[test]
fn disintegration_alias_is_accepted_and_generates_schema_spelling() {
    // `MASS-LOSS + BREAKUP` is a tolerated transcription variant, in neither the book nor the XSD.
    let alias = mutated(XML, "MASS-LOSS + BREAK-UP", "MASS-LOSS + BREAKUP");
    let message = Rdm::from_xml(&alias).unwrap();
    assert_eq!(message, Rdm::from_xml(XML).unwrap());
    let xml = message.to_xml().unwrap();
    let kvn = message.to_kvn().unwrap();
    assert!(xml.contains("MASS-LOSS + BREAK-UP"));
    assert!(kvn.contains("MASS-LOSS + BREAK-UP"));
    crate::common::validate_xml("RDM canonical disintegration", &xml);
    assert_eq!(Rdm::from_xml(&xml).unwrap(), message);
    assert_eq!(Rdm::from_kvn(&kvn).unwrap(), message);
    // The diagnostic advertises only the book spellings.
    let error = Rdm::from_xml(&mutated(XML, "MASS-LOSS + BREAK-UP", "BOTH"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("MASS-LOSS + BREAK-UP"), "{error}");
    assert!(!error.contains("BREAKUP"), "{error}");
}

#[test]
fn rdm_kvn_method_enums_accept_lowercase_and_reject_invented_values() {
    const DISINTEGRATION: &str = "REENTRY_DISINTEGRATION = MASS-LOSS + BREAK-UP\n";
    let with = |keyword: &str, value: &str| {
        let line = format!("{keyword} = {value}\n");
        match keyword {
            "REENTRY_DISINTEGRATION" => mutated(KVN, DISINTEGRATION, &line),
            "REENTRY_UNCERTAINTY_METHOD" => mutated(KVN, DISINTEGRATION, &(line + DISINTEGRATION)),
            _ => mutated(KVN, DISINTEGRATION, &(DISINTEGRATION.to_owned() + &line)),
        }
    };
    for (keyword, value, canonical) in [
        (
            "REENTRY_DISINTEGRATION",
            "mass-loss + break-up",
            "MASS-LOSS + BREAK-UP",
        ),
        ("REENTRY_UNCERTAINTY_METHOD", "stochastic", "STOCHASTIC"),
        ("IMPACT_UNCERTAINTY_METHOD", "empirical", "EMPIRICAL"),
    ] {
        let output = Rdm::from_kvn(&with(keyword, value))
            .unwrap()
            .to_kvn()
            .unwrap();
        assert!(output.contains(&format!("= {canonical}\n")), "{keyword}");
    }
    // XSD 4.0 lists only NONE, ANALYTICAL, STOCHASTIC, EMPIRICAL (audit 2.2).
    for keyword in ["REENTRY_UNCERTAINTY_METHOD", "IMPACT_UNCERTAINTY_METHOD"] {
        for invented in ["COVARIANCE", "STATISTICAL"] {
            assert!(
                Rdm::from_kvn(&with(keyword, invented)).is_err(),
                "{keyword} = {invented}"
            );
        }
    }
}

#[test]
fn rdm_kvn_lone_cov_ref_frame_is_an_incomplete_covariance() {
    // Audit 2.5: a COV_REF_FRAME without matrix used to be dropped with its comment.
    let kvn: String = KVN
        .lines()
        .filter(|line| {
            !(line.starts_with("CX_") || line.starts_with("CY_") || line.starts_with("CZ_"))
        })
        .map(|line| format!("{line}\n"))
        .collect();
    assert!(kvn.contains("COV_REF_FRAME = RTN"));
    let error = Rdm::from_kvn(&kvn).unwrap_err().to_string();
    assert!(error.contains("CX_X"), "{error}");
}

#[test]
fn rdm_xml_empty_optional_blocks_read_as_absent_and_comment_only_blocks_are_refused_for_kvn() {
    let mut xml = XML.to_string();
    for (start, end) in [
        ("<groundImpactParameters>", "</groundImpactParameters>"),
        ("<spacecraftParameters>", "</spacecraftParameters>"),
        ("<odParameters>", "</odParameters>"),
    ] {
        let a = xml.find(start).unwrap() + start.len();
        let b = xml.find(end).unwrap();
        xml.replace_range(a..b, "");
    }
    let message = Rdm::from_xml(&xml).unwrap();
    let data = &message.body.segment.data;
    assert!(data.ground_impact_parameters.is_none());
    assert!(data.spacecraft_parameters.is_none());
    assert!(data.od_parameters.is_none());
    assert_eq!(
        Rdm::from_kvn(&message.to_kvn().unwrap()).unwrap().body,
        message.body
    );

    for block in [
        "groundImpactParameters",
        "spacecraftParameters",
        "odParameters",
    ] {
        let input = xml.replace(
            &format!("<{block}></{block}>"),
            &format!("<{block}><COMMENT>orphan</COMMENT></{block}>"),
        );
        let message = Rdm::from_xml(&input).unwrap();
        let error = message.to_kvn().unwrap_err().to_string();
        assert!(error.contains(block), "{error}");
        let mut output = Vec::new();
        assert!(message.write_kvn_to(&mut output).is_err());
        assert!(output.is_empty());
    }
}

#[test]
fn rdm_kvn_always_displays_the_fixed_units() {
    // RDM 5.2.4.1: applicable units "must be displayed", even when XML omitted the optional attribute.
    let xml = XML
        .replace(" units=\"kg\"", "")
        .replace(" units=\"km**2\"", "");
    let message = Rdm::from_xml(&xml).unwrap();
    let kvn = message.to_kvn().unwrap();
    assert!(kvn.contains("[kg]\n"), "{kvn}");
    assert!(
        kvn.lines()
            .filter(|l| l.starts_with("CX_X"))
            .all(|l| l.ends_with("[km**2]")),
        "{kvn}"
    );
}

#[test]
fn rdm_controlled_type_values() {
    for (val, expected) in [
        ("YES", ControlledType::Yes),
        ("yes", ControlledType::Yes),
        ("NO", ControlledType::No),
        ("no", ControlledType::No),
        ("UNKNOWN", ControlledType::Unknown),
        ("unknown", ControlledType::Unknown),
    ] {
        let kvn = mutated(
            KVN,
            "CONTROLLED_REENTRY = NO",
            &format!("CONTROLLED_REENTRY = {val}"),
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(rdm.body.segment.metadata.controlled_reentry, expected);
        assert_eq!(Rdm::from_kvn(&rdm.to_kvn().unwrap()).unwrap(), rdm);
    }
    let error = "MAYBE".parse::<ControlledType>().unwrap_err();
    assert_eq!(error.field, "CONTROLLED_REENTRY");
}

#[test]
fn rdm_object_type_enum() {
    for obj_type in ["PAYLOAD", "ROCKET BODY", "DEBRIS", "UNKNOWN", "OTHER"] {
        let kvn = mutated(
            KVN,
            "OBJECT_TYPE = ROCKET BODY",
            &format!("OBJECT_TYPE = {obj_type}"),
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(
            rdm.body.segment.metadata.object_type.unwrap().to_string(),
            obj_type
        );
    }
    // RDM 5.3.3.4: an underscore in a text value is equivalent to a blank (Orekit writes it).
    let kvn = crate::common::mutated(
        KVN,
        "OBJECT_TYPE = ROCKET BODY",
        "OBJECT_TYPE = ROCKET_BODY",
    );
    assert_eq!(Rdm::from_kvn(&kvn).unwrap(), Rdm::from_kvn(KVN).unwrap());
}

#[test]
fn rdm_intrack_thrust_yesno() {
    for val in ["YES", "NO"] {
        let kvn = mutated(
            KVN,
            "INTRACK_THRUST = NO",
            &format!("INTRACK_THRUST = {val}"),
        );
        let rdm = Rdm::from_kvn(&kvn).unwrap();
        assert_eq!(
            rdm.body
                .segment
                .metadata
                .intrack_thrust
                .unwrap()
                .to_string(),
            val
        );
    }
}

#[test]
fn nominal_impact_coordinates_accept_boundaries_and_interior_values() {
    const C1: &str = include_str!("../../data/kvn/rdm_c1.kvn");
    for (field, values) in [
        ("NOMINAL_IMPACT_LAT", &[-90.0, 0.0, 45.5, 90.0][..]),
        ("NOMINAL_IMPACT_LON", &[-180.0, -45.5, 0.0, 90.0, 180.0][..]),
        ("NOMINAL_IMPACT_ALT", &[-430.0, 0.0, 1000.0, 8000.0][..]),
    ] {
        for value in values {
            let lon = if field == "NOMINAL_IMPACT_LON" {
                *value
            } else {
                0.0
            };
            let lat = if field == "NOMINAL_IMPACT_LAT" {
                *value
            } else {
                0.0
            };
            let altitude = if field == "NOMINAL_IMPACT_ALT" {
                format!("NOMINAL_IMPACT_ALT = {value}\n")
            } else {
                String::new()
            };
            let input = format!("{C1}IMPACT_REF_FRAME = ITRF\nNOMINAL_IMPACT_LON = {lon}\nNOMINAL_IMPACT_LAT = {lat}\n{altitude}");
            let message = Rdm::from_kvn(&input).unwrap();
            let ground = message.body.segment.data.ground_impact_parameters.unwrap();
            let actual = match field {
                "NOMINAL_IMPACT_LAT" => ground.nominal_impact_lat.unwrap().value,
                "NOMINAL_IMPACT_LON" => ground.nominal_impact_lon.unwrap().value,
                _ => ground.nominal_impact_alt.unwrap().value,
            };
            assert_eq!(actual, *value, "{field}");
        }
    }
}

#[test]
fn rdm_spacecraft_parameters() {
    let base = include_str!("../../data/kvn/rdm_c1.kvn");
    let kvn = format!(
        "{base}WET_MASS = 3500 [kg]
DRY_MASS = 2000 [kg]
HAZARDOUS_SUBSTANCES = Hydrazine, Nuclear
SOLAR_RAD_AREA = 25.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 20.0 [m**2]
DRAG_COEFF = 2.2
RCS = 15.0 [m**2]
BALLISTIC_COEFF = 150.0
THRUST_ACCELERATION = 0.001
"
    );
    let rdm = Rdm::from_kvn(&kvn).unwrap();
    let sp = rdm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert!((sp.wet_mass.as_ref().unwrap().value - 3500.0).abs() < 1e-9);
    assert!((sp.dry_mass.as_ref().unwrap().value - 2000.0).abs() < 1e-9);
    assert_eq!(
        sp.hazardous_substances,
        Some("Hydrazine, Nuclear".to_string())
    );
    assert!(sp.ballistic_coeff.is_some());
    assert!(sp.thrust_acceleration.is_some());
}

#[test]
fn rdm_od_parameters() {
    let base = include_str!("../../data/kvn/rdm_c1.kvn");
    let kvn = format!(
        "{base}TIME_LASTOB_START = 2022-12-31T00:00:00
TIME_LASTOB_END = 2022-12-31T23:59:59
RECOMMENDED_OD_SPAN = 7.0 [d]
ACTUAL_OD_SPAN = 5.5 [d]
OBS_AVAILABLE = 100
OBS_USED = 95
TRACKS_AVAILABLE = 20
TRACKS_USED = 18
RESIDUALS_ACCEPTED = 95.5 [%]
WEIGHTED_RMS = 1.234
"
    );
    let rdm = Rdm::from_kvn(&kvn).unwrap();
    let od = rdm.body.segment.data.od_parameters.as_ref().unwrap();
    assert!(od.time_lastob_start.is_some());
    assert!(od.time_lastob_end.is_some());
    assert_eq!(od.obs_available, Some(100.into()));
    assert_eq!(od.obs_used, Some(95.into()));
    assert_eq!(od.tracks_available, Some(20.into()));
    assert_eq!(od.tracks_used, Some(18.into()));
}

#[test]
fn rdm_sample_c1_kvn() {
    let kvn = include_str!("../../data/kvn/rdm_c1.kvn");
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
    assert_eq!(rdm.id.as_deref(), Some("CCSDS_RDM_VERS"));
    assert!(rdm.header.comment.is_empty());
    assert_eq!(rdm.header.message_id, "ESA/20180422-001");
    let metadata = &rdm.body.segment.metadata;
    assert_eq!(metadata.international_designator, "2018-099B");
    assert_eq!(metadata.center_name, "EARTH");
    assert_eq!(metadata.time_system, "UTC");
    let atmospheric = &rdm.body.segment.data.atmospheric_reentry_parameters;
    assert_eq!(atmospheric.orbit_lifetime.value, 23.0);
    assert_eq!(atmospheric.reentry_altitude.value, 150.0);
    let mut alternate = rdm;
    alternate.body.segment.metadata.center_name = "TEST-CENTER".into();
    alternate.body.segment.metadata.time_system = "TAI".into();
    assert_eq!(
        Rdm::from_kvn(&alternate.to_kvn().unwrap()).unwrap(),
        alternate
    );
}

#[test]
fn rdm_sample_c2_kvn() {
    let kvn = mutated(KVN, "X = 4000.000000 [km]", "X = 7000.0 [km]");
    let kvn = mutated(&kvn, "CX_X = 0.10000 [km**2]", "CX_X = 1.0e-4 [km**2]");
    let rdm = Rdm::from_kvn(&kvn).unwrap();
    assert_eq!(
        rdm.body.segment.data.state_vector.as_ref().unwrap().x.value,
        7000.0
    );
    let covariance = rdm.body.segment.data.covariance_matrix.as_ref().unwrap();
    assert_eq!(covariance.cov_ref_frame.as_deref(), Some("RTN"));
    assert_eq!(covariance.cx_x.value, 1.0e-4);
    assert_eq!(
        rdm.body.segment.metadata.catalog_name,
        Some("SATCAT".to_string())
    );
    assert!(rdm
        .body
        .segment
        .data
        .atmospheric_reentry_parameters
        .nominal_reentry_epoch
        .is_some());
}

/// Every mandatory keyword, dropped one at a time from a shipped fixture.
///
/// This was eleven copies of the same minimal message, each missing one line. `rdm_c1.kvn`
/// carries exactly those keywords and nothing else, so dropping one line from it is the same
/// case without the copies — and the baseline parse proves the other ten are still present.
#[test]
fn every_mandatory_rdm_keyword_is_reported_by_name() {
    const FIXTURE: &str = include_str!("../../data/kvn/rdm_c1.kvn");
    Rdm::from_kvn(FIXTURE).expect("baseline fixture must carry every mandatory keyword");

    for (keyword, block) in [
        ("CREATION_DATE", "Header"),
        ("ORIGINATOR", "Header"),
        ("MESSAGE_ID", "Header"),
        ("OBJECT_NAME", "Metadata"),
        ("INTERNATIONAL_DESIGNATOR", "Metadata"),
        ("CONTROLLED_REENTRY", "Metadata"),
        ("CENTER_NAME", "Metadata"),
        ("TIME_SYSTEM", "Metadata"),
        ("EPOCH_TZERO", "Metadata"),
        ("ORBIT_LIFETIME", "Atmospheric Reentry"),
        ("REENTRY_ALTITUDE", "Atmospheric Reentry"),
    ] {
        let kept: Vec<&str> = FIXTURE
            .lines()
            .filter(|line| !line.starts_with(&format!("{keyword} =")))
            .collect();
        assert_eq!(
            kept.len(),
            FIXTURE.lines().count() - 1,
            "{keyword} matched no line, so nothing was dropped"
        );

        let error = Rdm::from_kvn(&format!("{}\n", kept.join("\n")))
            .expect_err("message with a mandatory keyword removed was accepted");
        // ponytail: matching the rendered diagnostic rather than the typed variant. Only
        // `MissingRequiredField` renders this way, so it is exact today; the ceiling is that
        // rewording the `Display` impl breaks these tests. Match on the variant if that lands.
        let expected = format!("Missing required field: {keyword} in block {block}");
        assert!(
            error.to_string().contains(&expected),
            "dropping {keyword} reported {error}"
        );
    }
}
#[test]
fn test_rdm_empty_file_error() {
    let error = Rdm::from_kvn("").unwrap_err();
    let parse = crate::common::kvn_parse_error(&error)
        .unwrap_or_else(|| panic!("expected a KVN parse error, got {error:?}"));
    assert_eq!((parse.line, parse.column), (1, 1));
}

#[test]
fn test_rdm_version_not_first_error() {
    let kvn = r#"OBJECT_NAME = TEST
CCSDS_RDM_VERS = 1.0
"#;
    let err = Rdm::from_kvn(kvn).unwrap_err();
    let parse = crate::common::kvn_parse_error(&err).unwrap();
    assert!(parse
        .message
        .to_lowercase()
        .contains("duplicate or out-of-order rdm keyword"));
}

#[test]
fn rdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = SPACEOBJECT";
    let designator = "INTERNATIONAL_DESIGNATOR = 2018-099B";
    let lifetime = "ORBIT_LIFETIME = 5.5 [d]";
    for (label, source) in [
        (
            "duplicate keyword",
            mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            mutated(
                KVN,
                &format!("{object_name}\n{designator}"),
                &format!("{designator}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            mutated(KVN, lifetime, &format!("{lifetime}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            mutated(KVN, lifetime, &format!("{lifetime}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
    ] {
        let error = Rdm::from_kvn(&source).unwrap_err();
        assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
    }
}

#[test]
fn rdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let lifetime = "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>";
    let altitude = "<REENTRY_ALTITUDE units=\"km\">80.0</REENTRY_ALTITUDE>";
    for (label, source) in [
        (
            "unknown atmospheric child",
            mutated(
                XML,
                "<atmosphericReentryParameters>",
                "<atmosphericReentryParameters><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown covariance child",
            mutated(
                XML,
                "<covarianceMatrix>",
                "<covarianceMatrix><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown container attribute",
            mutated(
                XML,
                "<groundImpactParameters>",
                "<groundImpactParameters unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                lifetime,
                "<ORBIT_LIFETIME units=\"d\" unexpected=\"value\">5.5</ORBIT_LIFETIME>",
            ),
        ),
        (
            "duplicate element",
            mutated(XML, lifetime, &format!("{lifetime}{lifetime}")),
        ),
        (
            "reordered elements",
            mutated(
                XML,
                &format!("{lifetime}\n{altitude}"),
                &format!("{altitude}\n{lifetime}"),
            ),
        ),
    ] {
        let error = Rdm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

/// Parse official RDM XML example C-3 (minimal)
#[test]
fn rdm_sample_c3_xml() {
    let xml = include_str!("../../data/xml/rdm_c3.xml");
    let rdm = Rdm::from_xml(xml).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.originator, "ESA");
    assert_eq!(rdm.header.message_id, "ESA/20180422-001");
    assert_eq!(rdm.body.segment.metadata.object_name, "SPACEOBJECT");
}

/// The conditional RDM rules, as single mutations of a shipped fixture.
#[test]
fn rdm_conditional_rules_name_the_offending_field() {
    const FIXTURE: &str = include_str!("../../data/kvn/rdm_c2.kvn");
    const STATE_VECTOR: &str = "COMMENT State vector at the last OD epoch";
    const COVARIANCE: &str = "COMMENT Position/velocity covariance matrix at last OD epoch";
    Rdm::from_kvn(FIXTURE).expect("baseline fixture must satisfy every rule");

    let window = mutated(FIXTURE, "REENTRY_ALTITUDE = 80.0 [km]", "REENTRY_ALTITUDE = 80.0 [km]\nORBIT_LIFETIME_WINDOW_START = 4.0 [d]\nORBIT_LIFETIME_WINDOW_END = 7.0 [d]");
    let positive = Rdm::from_kvn(&window).unwrap();
    let atmospheric = &positive.body.segment.data.atmospheric_reentry_parameters;
    assert_eq!(
        atmospheric
            .orbit_lifetime_window_start
            .as_ref()
            .unwrap()
            .value,
        4.0
    );
    assert_eq!(
        atmospheric
            .orbit_lifetime_window_end
            .as_ref()
            .unwrap()
            .value,
        7.0
    );

    let state_vector_at = FIXTURE.find(STATE_VECTOR).unwrap();
    let covariance_at = FIXTURE.find(COVARIANCE).unwrap();
    // Neither block is mandatory, so the truncation is a valid message in its own right.
    let no_state_or_covariance = &FIXTURE[..state_vector_at];
    Rdm::from_kvn(no_state_or_covariance)
        .expect("a message with neither optional block must still parse");
    let covariance_without_state =
        format!("{}{}", no_state_or_covariance, &FIXTURE[covariance_at..]);

    for (mutated, expected) in [
            (
                mutated(FIXTURE, "REENTRY_ALTITUDE = 80.0 [km]",
                    "REENTRY_ALTITUDE = 80.0 [km]\nORBIT_LIFETIME_WINDOW_START = 6.0 [d]\nORBIT_LIFETIME_WINDOW_END = 5.0 [d]",
                ),
                "ORBIT_LIFETIME_WINDOW_START",
            ),
            (
                mutated(FIXTURE, "REF_FRAME = EME2000\n", ""),
                "Missing required field: REF_FRAME (required when state vector is provided)",
            ),
            (
                covariance_without_state,
                "Missing required field: stateVector (required when covarianceMatrix is provided)",
            ),
            (
                mutated(FIXTURE, "OBJECT_NAME = SPACEOBJECT", "OBJECT_NAME ="),
                "Missing required field: OBJECT_NAME",
            ),
        ] {
            let error = Rdm::from_kvn(&mutated).expect_err("mutation accepted");
            assert!(
                error.to_string().contains(expected),
                "diagnostic did not name {expected}: {error}"
            );
        }
}

#[test]
fn all_normative_line_endings_preserve_the_message() {
    let expected = Rdm::from_kvn(crate::KVN).unwrap();
    for ending in ["\n", "\r", "\r\n", "\n\r"] {
        let input = crate::KVN.replace('\n', ending);
        assert_eq!(Rdm::from_kvn(&input).unwrap(), expected, "{ending:?}");
    }
}

#[test]
fn rdm_range_errors_name_the_keyword_in_both_notations() {
    // Audit 6.4: errors used to name Rust types ('Probability', 'PositiveInteger', ...).
    let cases = [
        ("PROBABILITY_OF_IMPACT = 0.0", "PROBABILITY_OF_IMPACT = 1.5", "PROBABILITY_OF_IMPACT"),
        ("TRACKS_USED = 17", "TRACKS_USED = 0", "TRACKS_USED"),
        (
            "PROBABILITY_OF_BURN_UP = 1.0",
            "PROBABILITY_OF_BURN_UP = 1.0\nIMPACT_REF_FRAME = EFG\nNOMINAL_IMPACT_LON = 181 [deg]\nNOMINAL_IMPACT_LAT = 0 [deg]",
            "NOMINAL_IMPACT_LON",
        ),
        (
            "REENTRY_WINDOW_END = 2018-04-27T22:12:56",
            "REENTRY_WINDOW_END = 2018-04-27T22:12:56\nORBIT_LIFETIME_CONFIDENCE_LEVEL = 101 [%]",
            "ORBIT_LIFETIME_CONFIDENCE_LEVEL",
        ),
    ];
    for (from, to, keyword) in cases {
        let kvn = KVN.replacen(from, to, 1);
        let error = Rdm::from_kvn(&kvn).unwrap_err();
        crate::common::assert_validation_field(&error, keyword);
    }
    for (from, to, keyword) in [
        (
            ">0.0</PROBABILITY_OF_IMPACT>",
            ">1.5</PROBABILITY_OF_IMPACT>",
            "PROBABILITY_OF_IMPACT",
        ),
        (">17</TRACKS_USED>", ">0</TRACKS_USED>", "TRACKS_USED"),
    ] {
        let error = Rdm::from_xml(&mutated(XML, from, to)).unwrap_err();
        crate::common::assert_validation_field(&error, keyword);
    }
}

#[test]
fn rdm_empty_mandatory_xml_strings_are_reported_as_empty() {
    let original = Rdm::from_xml(XML).unwrap().to_xml().unwrap();
    for keyword in [
        "ORIGINATOR",
        "MESSAGE_ID",
        "OBJECT_NAME",
        "INTERNATIONAL_DESIGNATOR",
        "CENTER_NAME",
        "TIME_SYSTEM",
    ] {
        let start_tag = format!("<{keyword}>");
        let end_tag = format!("</{keyword}>");
        let start = original.find(&start_tag).unwrap();
        let end = original[start..].find(&end_tag).unwrap() + start + end_tag.len();
        let mut xml = original.clone();
        xml.replace_range(start..end, &format!("<{keyword}/>"));
        let error = Rdm::from_xml(&xml).unwrap_err();
        assert_eq!(
            error.code(),
            Some("validation.missing_required_field"),
            "{error}"
        );
        assert!(
            error
                .to_string()
                .contains(&format!("Missing required field: {keyword}")),
            "{error}"
        );
    }
}

#[test]
fn nominal_impact_altitude_units_are_optional_in_xml_but_displayed_in_kvn() {
    let original = crate::generation::ground_impact_with_altitude(1.0);
    let mut xml = original.to_xml().unwrap();
    let start = xml.find("<NOMINAL_IMPACT_ALT units=\"m\"").unwrap();
    xml.replace_range(
        start..start + "<NOMINAL_IMPACT_ALT units=\"m\"".len(),
        "<NOMINAL_IMPACT_ALT",
    );
    crate::common::validate_xml("RDM unitless impact altitude", &xml);
    let message = Rdm::from_xml(&xml).unwrap();
    let altitude = message
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap()
        .nominal_impact_alt
        .as_ref()
        .unwrap();
    assert!(altitude.units.is_none());
    assert_eq!(Rdm::from_xml(&message.to_xml().unwrap()).unwrap(), message);
    let kvn = message.to_kvn().unwrap();
    let line = kvn
        .lines()
        .find(|line| line.starts_with("NOMINAL_IMPACT_ALT"))
        .unwrap();
    assert!(line.ends_with("[m]"), "{line}");
    let with_units = Rdm::from_kvn(&kvn).unwrap();
    assert!(with_units
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap()
        .nominal_impact_alt
        .as_ref()
        .unwrap()
        .units
        .is_some());
    assert_eq!(with_units.to_kvn().unwrap(), kvn);
}
