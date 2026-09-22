use crate::common::mutated;
use crate::{KVN, XML};
use ccsds_ndm::messages::omm::Omm;
use ccsds_ndm::Ndm;
#[test]
fn test_omm_validation() {
    // Construct a parsed OMM-like structure or just parse minimal incomplete one and validate
    // SGP4 Theory requires BSTAR
    let kvn_sgp4_missing_bstar = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
    crate::common::assert_validation_field(
        &Omm::from_kvn(kvn_sgp4_missing_bstar).unwrap_err(),
        "TLE_PARAMETERS",
    );

    // SGP4 with params but missing BSTAR
    let kvn_sgp4_missing_bstar_field = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
COMMENT TLE Params present but partial
MEAN_MOTION_DOT = 0.001 [rev/day**2]
"#;
    crate::common::assert_validation_field(
        &Omm::from_kvn(kvn_sgp4_missing_bstar_field).unwrap_err(),
        "BSTAR",
    );

    // Valid SGP4
    let kvn_sgp4_valid = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001
MEAN_MOTION_DOT = 0.0001
"#;
    assert!(Omm::from_kvn(kvn_sgp4_valid).is_ok());
}

#[test]
fn parse_omm_with_covariance() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
CX_X = 1.0 [km**2]
CY_X = 0.0 [km**2]
CY_Y = 1.0 [km**2]
CZ_X = 0.0 [km**2]
CZ_Y = 0.0 [km**2]
CZ_Z = 1.0 [km**2]
CX_DOT_X = 0.0 [km**2/s]
CX_DOT_Y = 0.0 [km**2/s]
CX_DOT_Z = 0.0 [km**2/s]
CX_DOT_X_DOT = 0.01 [km**2/s**2]
CY_DOT_X = 0.0 [km**2/s]
CY_DOT_Y = 0.0 [km**2/s]
CY_DOT_Z = 0.0 [km**2/s]
CY_DOT_X_DOT = 0.0 [km**2/s**2]
CY_DOT_Y_DOT = 0.01 [km**2/s**2]
CZ_DOT_X = 0.0 [km**2/s]
CZ_DOT_Y = 0.0 [km**2/s]
CZ_DOT_Z = 0.0 [km**2/s]
CZ_DOT_X_DOT = 0.0 [km**2/s**2]
CZ_DOT_Y_DOT = 0.0 [km**2/s**2]
CZ_DOT_Z_DOT = 0.01 [km**2/s**2]
"#;
    let omm = Omm::from_kvn(kvn).expect("OMM Covariance parse failed");
    assert!(omm.body.segment.data.covariance_matrix.is_some());
}

#[test]
fn test_mean_elements_choice_semi_major_axis_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = EME2000
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
SEMI_MAJOR_AXIS = 7000.0 [km]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with SEMI_MAJOR_AXIS");
    assert!(omm
        .body
        .segment
        .data
        .mean_elements
        .semi_major_axis
        .is_some());
}

#[test]
fn test_mean_elements_choice_mean_motion_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = DSST
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION");
    assert!(omm.body.segment.data.mean_elements.mean_motion.is_some());
}

#[test]
fn test_tle_choice_bstar_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with BSTAR");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert!(tle.bstar.is_some());
    assert!(tle.bterm.is_none());
}

#[test]
fn test_tle_choice_bterm_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with BTERM");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert!(tle.bterm.is_some());
    assert!(tle.bstar.is_none());
}

#[test]
fn test_tle_choice_mean_motion_ddot_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with MEAN_MOTION_DDOT");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert!(tle.mean_motion_ddot.is_some());
    assert!(tle.agom.is_none());
}

#[test]
fn test_tle_choice_agom_only() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
OBJECT_NAME = SAT
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4-XP
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
BTERM = 0.02 [m**2/kg]
MEAN_MOTION_DOT = 0.0 [rev/day**2]
AGOM = 0.01 [m**2/kg]
"#;
    let omm = Omm::from_kvn(kvn).expect("Should parse with AGOM");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert!(tle.agom.is_some());
    assert!(tle.mean_motion_ddot.is_none());
}

fn assert_kvn_rejected(label: &str, source: String) {
    let error = Omm::from_kvn(&source).unwrap_err();
    assert_eq!(error.code(), Some("parse.kvn.syntax"), "{label}: {error}");
}

#[test]
fn omm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = GOES 9";
    let object_id = "OBJECT_ID = 1995-025A";
    for (label, source) in [
        (
            "duplicate keyword",
            mutated(KVN, object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered keywords",
            mutated(
                KVN,
                &format!("{object_name}\n{object_id}"),
                &format!("{object_id}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            mutated(KVN, object_name, &format!("{object_name}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            mutated(
                KVN,
                object_name,
                &format!("{object_name}\nCOMMENT misplaced"),
            ),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            mutated(KVN, object_name, &format!("{object_name} €")),
        ),
    ] {
        assert_kvn_rejected(label, source);
    }
}

#[test]
fn omm_xml_rejects_unknown_nested_content_and_ordering_errors() {
    let epoch = "<EPOCH>2020-064T10:34:41.4264</EPOCH>";
    let mean_motion = "<MEAN_MOTION>1.00273272</MEAN_MOTION>";
    for (label, source) in [
        (
            "unknown mean-elements child",
            mutated(XML, "<meanElements>", "<meanElements><UNKNOWN>1</UNKNOWN>"),
        ),
        (
            "unknown TLE child",
            mutated(
                XML,
                "<tleParameters>",
                "<tleParameters><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown mean-elements attribute",
            mutated(XML, "<meanElements>", "<meanElements unexpected=\"value\">"),
        ),
        (
            "unknown leaf attribute",
            mutated(
                XML,
                epoch,
                "<EPOCH unexpected=\"value\">2020-064T10:34:41.4264</EPOCH>",
            ),
        ),
        (
            "duplicate element",
            mutated(XML, epoch, &format!("{epoch}{epoch}")),
        ),
        (
            "reordered elements",
            mutated(
                XML,
                &format!("{epoch}\n{mean_motion}"),
                &format!("{mean_motion}\n{epoch}"),
            ),
        ),
    ] {
        let error = Omm::from_xml(&source).unwrap_err();
        assert!(
            matches!(
                error.as_format_error(),
                Some(ccsds_ndm::error::FormatError::InvalidFormat(_))
            ),
            "{label}: {error}"
        );
    }
}

/// Each OMM keyword choice shares one ordering rank so either spelling may fill the slot. That
/// allowance must not extend to repeating one alternative: the KVN block parser keeps the last
/// assignment, so a repeat that reached it would silently discard a value.
#[test]
fn omm_kvn_separates_keyword_choices_from_repeated_alternatives() {
    for (label, key, line) in [
        (
            "MEAN_MOTION",
            "MEAN_MOTION = 1.00273272 [rev/day]",
            "MEAN_MOTION = 2.0",
        ),
        ("BSTAR", "BSTAR = 0.0001 [1/ER]", "BSTAR = 0.0002"),
        (
            "MEAN_MOTION_DDOT",
            "MEAN_MOTION_DDOT = 0.0 [rev/day**3]",
            "MEAN_MOTION_DDOT = 1.0",
        ),
    ] {
        assert!(KVN.contains(key), "fixture should contain {label}");
        assert_kvn_rejected(
            &format!("repeated {label}"),
            mutated(KVN, key, &format!("{key}\n{line}")),
        );
    }

    // The other alternative may still follow; it is rejected as a semantic conflict rather than
    // as an ordering error, so the diagnostic names both fields.
    let error = Omm::from_kvn(&mutated(
        KVN,
        "BSTAR = 0.0001 [1/ER]",
        "BSTAR = 0.0001 [1/ER]\nBTERM = 0.02",
    ))
    .expect_err("BSTAR and BTERM are mutually exclusive");
    // `ValidationError::Conflict` has no stabilized code yet, so match the diagnostic itself.
    assert!(
        error.to_string().contains("Conflicting fields"),
        "expected a conflict diagnostic, got {error}"
    );
}

#[test]
fn omm_xml_allows_a_comment_to_reopen_the_user_defined_group() {
    // The OMM carries the same `userDefinedType` block as the OPM and the RDM, so it has to
    // accept the repeating group on the same terms.
    let source = mutated(
        include_str!("../../data/xml/omm_g10.xml"),
        "</data>",
        "<userDefinedParameters>\
         <COMMENT>first</COMMENT>\
         <USER_DEFINED parameter=\"A\">1</USER_DEFINED>\
         <COMMENT>second</COMMENT>\
         <USER_DEFINED parameter=\"B\">2</USER_DEFINED>\
         </userDefinedParameters></data>",
    );
    let message = Omm::from_xml(&source).unwrap();
    let user_defined = message
        .body
        .segment
        .data
        .user_defined_parameters
        .as_ref()
        .unwrap();
    assert_eq!(user_defined.comment, ["first", "second"]);
    assert_eq!(user_defined.user_defined.len(), 2);
}

#[test]
fn omm_xml_still_rejects_a_genuinely_out_of_order_child() {
    // The relaxation is scoped to the repeating group: `metadata` is a plain xsd:sequence, so
    // CENTER_NAME cannot precede OBJECT_ID.
    let source = include_str!("../../data/xml/omm_g10.xml");
    let reordered = mutated(
        source,
        "<OBJECT_ID>1995-025A</OBJECT_ID>\n<CENTER_NAME>EARTH</CENTER_NAME>",
        "<CENTER_NAME>EARTH</CENTER_NAME>\n<OBJECT_ID>1995-025A</OBJECT_ID>",
    );
    assert_ne!(source, reordered, "fixture shape changed; update this test");
    crate::common::assert_invalid_format(
        &Omm::from_xml(&reordered).unwrap_err(),
        "invalid OMM XML sequence: duplicate or out-of-order child 'OBJECT_ID' in 'metadata'",
    );
}

#[test]
fn parse_xml_omm_g10() {
    let xml = include_str!("../../data/xml/omm_g10.xml");
    let omm = Omm::from_xml(xml).expect("Failed to parse omm_g10.xml");

    assert_eq!(omm.version, "3.0");
    assert_eq!(omm.body.segment.metadata.object_name, "GOES-9");
    assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");

    let me = &omm.body.segment.data.mean_elements;
    assert!(me.mean_motion.is_some());

    // Has covariance
    assert!(omm.body.segment.data.covariance_matrix.is_some());

    // Has TLE parameters
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert_eq!(tle.norad_cat_id, Some(23581));
}

/// Drops every TLE keyword, leaving a message whose theory still demands the block.
fn strip_tle_parameters(kvn: &str) -> String {
    let dropped = [
        "EPHEMERIS_TYPE",
        "CLASSIFICATION_TYPE",
        "NORAD_CAT_ID",
        "ELEMENT_SET_NO",
        "REV_AT_EPOCH",
        "BSTAR",
        "MEAN_MOTION_DOT",
        "MEAN_MOTION_DDOT",
    ];
    let kept: Vec<&str> = kvn
        .lines()
        .filter(|line| !dropped.iter().any(|key| line.starts_with(key)))
        .collect();
    format!("{}\n", kept.join("\n"))
}

/// Every message-level rule, exercised as a single mutation of a shipped fixture.
///
/// Hand-written messages had drifted out of the KVN grammar — they carried an XML-only
/// `TLE_PARAMETERS =` block header, so the parser rejected them as an unknown keyword and the
/// rule under test was never reached. Mutating a fixture that parses keeps each case honest,
/// and asserting on the diagnostic keeps it from passing on an unrelated failure.
#[test]
fn omm_kvn_validation_rules_name_the_offending_field() {
    const FIXTURE: &str = include_str!("../../data/kvn/omm_g7.kvn");
    // The shipped theory is SGP/SGP4, which imposes no TLE requirements of its own.
    let sgp4 = |kvn: &str| mutated(kvn, "THEORY = SGP/SGP4", "THEORY = SGP4");
    let sgp = |kvn: &str| mutated(kvn, "THEORY = SGP/SGP4", "THEORY = SGP");

    Omm::from_kvn(&sgp4(FIXTURE)).expect("baseline fixture must satisfy every rule");

    for (mutated, expected) in [
        // Mandatory metadata.
        (
            mutated(FIXTURE, "OBJECT_NAME = GOES 9\n", ""),
            "Missing required field: OBJECT_NAME",
        ),
        // Mean element choice: exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION.
        (
            mutated(
                &mutated(FIXTURE, "THEORY = SGP/SGP4", "THEORY = DSST"),
                "MEAN_MOTION = 1.00273272\n",
                "",
            ),
            "exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION",
        ),
        (
            mutated(
                &mutated(FIXTURE, "THEORY = SGP/SGP4", "THEORY = DSST"),
                "MEAN_MOTION =",
                "SEMI_MAJOR_AXIS = 7000.0\nMEAN_MOTION =",
            ),
            "exactly one of SEMI_MAJOR_AXIS or MEAN_MOTION",
        ),
        // Range-checked element values.
        (
            mutated(
                FIXTURE,
                "ECCENTRICITY = 0.0005013",
                "ECCENTRICITY = -0.0005013",
            ),
            "out of range",
        ),
        // SGP4 needs MEAN_MOTION, a TLE block, and BSTAR within it.
        (
            mutated(
                &sgp4(FIXTURE),
                "MEAN_MOTION = 1.00273272",
                "SEMI_MAJOR_AXIS = 7000.0",
            ),
            "Missing required field: MEAN_MOTION in block Mean Elements",
        ),
        (
            strip_tle_parameters(&sgp4(FIXTURE)),
            "Missing required field: TLE_PARAMETERS",
        ),
        (
            mutated(&sgp4(FIXTURE), "BSTAR = 0.0001\n", ""),
            "Missing required field: BSTAR",
        ),
        // SGP4-XP swaps BSTAR for BTERM and additionally needs AGOM.
        (
            mutated(
                &mutated(&sgp4(FIXTURE), "THEORY = SGP4", "THEORY = SGP4-XP"),
                "BSTAR = 0.0001\n",
                "",
            ),
            "Missing required field: BTERM",
        ),
        (
            mutated(
                &mutated(&sgp4(FIXTURE), "THEORY = SGP4", "THEORY = SGP4-XP"),
                "BSTAR = 0.0001",
                "BTERM = 0.01",
            ),
            "Missing required field: AGOM",
        ),
        // SGP and PPT3 need MEAN_MOTION_DDOT.
        (
            mutated(&sgp(FIXTURE), "MEAN_MOTION_DDOT = 0.0\n", ""),
            "Missing required field: MEAN_MOTION_DDOT",
        ),
        (
            mutated(
                &mutated(&sgp(FIXTURE), "THEORY = SGP", "THEORY = PPT3"),
                "MEAN_MOTION_DDOT = 0.0\n",
                "",
            ),
            "Missing required field: MEAN_MOTION_DDOT",
        ),
        // Mutually exclusive TLE parameters.
        (
            mutated(
                &sgp4(FIXTURE),
                "BSTAR = 0.0001",
                "BSTAR = 0.0001\nBTERM = 0.01",
            ),
            r#"Conflicting fields: ["BSTAR", "BTERM"]"#,
        ),
        (
            mutated(
                &sgp(FIXTURE),
                "MEAN_MOTION_DDOT = 0.0",
                "MEAN_MOTION_DDOT = 0.0\nAGOM = 0.0001",
            ),
            r#"Conflicting fields: ["MEAN_MOTION_DDOT", "AGOM"]"#,
        ),
    ] {
        let error = Omm::from_kvn(&mutated).expect_err("mutation accepted");
        assert!(
            error.to_string().contains(expected),
            "diagnostic did not name {expected}: {error}"
        );
    }
}
