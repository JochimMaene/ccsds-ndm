use crate::common::mutated;
use ccsds_ndm::common::OdmHeader;
use ccsds_ndm::error::ValidationError;
use ccsds_ndm::messages::omm::{
    BStar, InvErUnits, MeanElements, MeanMotion, MeanMotionDDot, MeanMotionDot, Omm, OmmBody,
    OmmData, OmmMetadata, OmmSegment, RevPerDay2Units, RevPerDay3Units, RevPerDayUnits,
    TleParameters, TleToOmmOptions,
};
use ccsds_ndm::types::{Angle, Inclination, M2kg, NonNegativeDouble};
use ccsds_ndm::Ndm;
// =========================================================================
// OMM Roundtrip Tests (Kitchen Sink)
// =========================================================================

#[test]
fn full_optional_fields_roundtrip() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
COMMENT Header Comment 1
COMMENT Header Comment 2
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = MSG-001
OBJECT_NAME = SATELLITE
OBJECT_ID = 2023-001A
CENTER_NAME = EARTH
REF_FRAME = TEME
REF_FRAME_EPOCH = 2000-01-01T12:00:00
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2023-01-01T00:00:00
MEAN_MOTION = 15.5 [rev/day]
ECCENTRICITY = 0.001
INCLINATION = 98.0 [deg]
RA_OF_ASC_NODE = 10.0 [deg]
ARG_OF_PERICENTER = 20.0 [deg]
MEAN_ANOMALY = 30.0 [deg]
GM = 398600.4418 [km**3/s**2]
MASS = 1500.0 [kg]
SOLAR_RAD_AREA = 20.0 [m**2]
SOLAR_RAD_COEFF = 1.2
DRAG_AREA = 15.0 [m**2]
DRAG_COEFF = 2.2
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 99999
ELEMENT_SET_NO = 123
REV_AT_EPOCH = 500
BSTAR = 0.0001 [1/ER]
MEAN_MOTION_DOT = 0.000001 [rev/day**2]
MEAN_MOTION_DDOT = 0.0 [rev/day**3]
COV_REF_FRAME = TEME
CX_X = 1.0
CY_X = 0.1
CY_Y = 1.0
CZ_X = 0.1
CZ_Y = 0.1
CZ_Z = 1.0
CX_DOT_X = 0.1
CX_DOT_Y = 0.1
CX_DOT_Z = 0.1
CX_DOT_X_DOT = 1.0
CY_DOT_X = 0.1
CY_DOT_Y = 0.1
CY_DOT_Z = 0.1
CY_DOT_X_DOT = 0.1
CY_DOT_Y_DOT = 1.0
CZ_DOT_X = 0.1
CZ_DOT_Y = 0.1
CZ_DOT_Z = 0.1
CZ_DOT_X_DOT = 0.1
CZ_DOT_Y_DOT = 0.1
CZ_DOT_Z_DOT = 1.0
USER_DEFINED_FOO = BAR
USER_DEFINED_BAZ = QUX
"#;
    let omm = Omm::from_kvn(kvn).expect("Failed to parse kitchen sink OMM");

    // Verify some fields
    assert_eq!(omm.header.message_id, Some("MSG-001".to_string()));
    assert_eq!(omm.header.comment.len(), 2);

    let me = &omm.body.segment.data.mean_elements;
    assert_eq!(me.gm.as_ref().unwrap().value, 398600.4418);

    let sp = omm
        .body
        .segment
        .data
        .spacecraft_parameters
        .as_ref()
        .unwrap();
    assert_eq!(sp.mass.as_ref().unwrap().value, 1500.0);

    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert_eq!(tle.norad_cat_id, Some(99999));

    let binding = omm
        .body
        .segment
        .data
        .user_defined_parameters
        .as_ref()
        .unwrap();
    let ud = &binding.user_defined;
    assert_eq!(ud.len(), 2);
    assert_eq!(ud[0].parameter, "FOO");
    assert_eq!(ud[0].value, "BAR");

    // Roundtrip
    let kvn_out = omm.to_kvn().expect("Failed to serialize OMM");
    let omm2 = Omm::from_kvn(&kvn_out).expect("Failed to re-parse OMM");

    assert_eq!(omm, omm2);
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

#[test]
fn roundtrip_kvn_minimal() {
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
    let omm1 = Omm::from_kvn(kvn).expect("First parse failed");
    let kvn2 = omm1.to_kvn().expect("Serialization failed");
    let omm2 = Omm::from_kvn(&kvn2).expect("Second parse failed");

    assert_eq!(omm1.version, omm2.version);
    assert_eq!(omm1.header.originator, omm2.header.originator);
    assert_eq!(
        omm1.body.segment.metadata.object_name,
        omm2.body.segment.metadata.object_name
    );
    assert_eq!(
        omm1.body.segment.data.mean_elements.eccentricity,
        omm2.body.segment.data.mean_elements.eccentricity
    );
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

#[test]
fn omm_units_parsing() {
    use std::str::FromStr;
    assert!(InvErUnits::from_str("1/ER").is_ok());
    let error = InvErUnits::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDayUnits::from_str("rev/day").is_ok());
    assert!(RevPerDayUnits::from_str("REV/DAY").is_ok());
    let error = RevPerDayUnits::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDay2Units::from_str("rev/day**2").is_ok());
    assert!(RevPerDay2Units::from_str("REV/DAY**2").is_ok());
    let error = RevPerDay2Units::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");

    assert!(RevPerDay3Units::from_str("rev/day**3").is_ok());
    assert!(RevPerDay3Units::from_str("REV/DAY**3").is_ok());
    let error = RevPerDay3Units::from_str("INVALID").unwrap_err();
    assert_eq!(error.field, "unit");
    assert_eq!(error.value, "INVALID");
}

/// `TleParameters::validate` is public, so its theory dispatch is checked directly: the
/// error variants the KVN table only sees as text, the theories that impose no requirement,
/// and the satisfied SGP4-XP case.
#[test]
fn tle_parameters_validate_dispatches_on_theory() {
    let base = || {
        TleParameters::builder()
            .mean_motion_dot(MeanMotionDot::new(0.0, None))
            .build()
    };

    let mut conflicting = base();
    conflicting.bstar = Some(BStar::new(0.0001, None));
    conflicting.bterm = Some(M2kg::new(0.01, None));
    let error = conflicting.validate("SGP4").unwrap_err();
    assert!(error.as_validation_error().is_some_and(|error| {
        matches!(error, ValidationError::Conflict { fields, .. }
                if fields.iter().any(|field| field.as_ref() == "BSTAR")
                    && fields.iter().any(|field| field.as_ref() == "BTERM"))
    }));

    let mut conflicting = base();
    conflicting.mean_motion_ddot = Some(MeanMotionDDot::new(0.0, None));
    conflicting.agom = Some(M2kg::new(0.001, None));
    let error = conflicting.validate("SGP").unwrap_err();
    assert!(error.as_validation_error().is_some_and(|error| {
        matches!(error, ValidationError::Conflict { fields, .. }
                if fields.iter().any(|field| field.as_ref() == "MEAN_MOTION_DDOT")
                    && fields.iter().any(|field| field.as_ref() == "AGOM"))
    }));

    // An unrecognised theory imposes nothing beyond the conflict rules.
    assert!(base().validate("UNKNOWN").is_ok());

    let mut satisfied = base();
    satisfied.bterm = Some(M2kg::new(0.01, None));
    satisfied.agom = Some(M2kg::new(1.0, None));
    satisfied
        .validate("SGP4-XP")
        .expect("BTERM and AGOM satisfy SGP4-XP");
}

#[test]
fn omm_serialization_gaps() {
    let mut tle = TleParameters::builder()
        .mean_motion_dot(MeanMotionDot::new(0.0, None))
        .build();
    tle.bterm = Some(M2kg::new(0.01, None));
    tle.agom = Some(M2kg::new(1.0, None));

    let omm = Omm::builder()
        .version("3.0")
        .header(
            OdmHeader::builder()
                .creation_date("2023-01-01T00:00:00".parse().unwrap())
                .originator("ME")
                .build(),
        )
        .body(
            OmmBody::builder()
                .segment(
                    OmmSegment::builder()
                        .metadata(
                            OmmMetadata::builder()
                                .object_name("SAT")
                                .object_id("1")
                                .center_name("EARTH")
                                .ref_frame("TEME")
                                .time_system("UTC")
                                .mean_element_theory("SGP4-XP")
                                .build(),
                        )
                        .data(
                            OmmData::builder()
                                .mean_elements(
                                    MeanElements::builder()
                                        .epoch("2023-01-01T00:00:00".parse().unwrap())
                                        .mean_motion(MeanMotion::new(15.0, None))
                                        .eccentricity(NonNegativeDouble::new(0.001).unwrap())
                                        .inclination(Inclination::new(10.0, None).unwrap())
                                        .ra_of_asc_node(Angle::new(10.0, None).unwrap())
                                        .arg_of_pericenter(Angle::new(10.0, None).unwrap())
                                        .mean_anomaly(Angle::new(10.0, None).unwrap())
                                        .build(),
                                )
                                .tle_parameters(tle)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    let kvn = omm.to_kvn().unwrap();
    assert!(kvn.contains("BTERM"));
    assert!(kvn.contains("0.01"));
    assert!(kvn.contains("AGOM"));
    assert!(kvn.contains("1"));
}

#[test]
fn to_tle_lines_iss_example() {
    let kvn = r#"CCSDS_OMM_VERS = 2.0
CREATION_DATE = 2020-12-13T17:26:09
ORIGINATOR = 18 SPCS
OBJECT_NAME = ISS (ZARYA)
OBJECT_ID = 1998-067A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2020-12-13T16:36:04.502592
MEAN_MOTION = 15.49181153 [rev/day]
ECCENTRICITY = 0.00017790
INCLINATION = 51.6444 [deg]
RA_OF_ASC_NODE = 180.2777 [deg]
ARG_OF_PERICENTER = 128.5985 [deg]
MEAN_ANOMALY = 350.1361 [deg]
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 25544
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 25984
BSTAR = 0.00002412400000 [1/ER]
MEAN_MOTION_DOT = 0.00000888 [rev/day**2]
MEAN_MOTION_DDOT = 0.0000000000000 [rev/day**3]
"#;

    let omm = Omm::from_kvn(kvn).expect("failed to parse ISS OMM sample");
    let (line1, line2) = omm.to_tle_lines().expect("failed to generate TLE lines");
    assert_eq!(
        line1,
        "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
    );
    assert_eq!(
        line2,
        "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"
    );
}

#[test]
fn from_tle_lines_iss_example() {
    let line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995";
    let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";

    let omm = Omm::from_tle_lines(line1, line2).expect("failed to parse TLE lines");
    assert_eq!(omm.body.segment.metadata.object_id, "1998-067A");
    assert_eq!(omm.body.segment.metadata.object_name, "UNKNOWN");
    assert_eq!(omm.body.segment.metadata.center_name, "EARTH");
    assert_eq!(omm.body.segment.metadata.ref_frame, "TEME");
    assert_eq!(omm.body.segment.metadata.time_system, "UTC");
    assert_eq!(omm.body.segment.metadata.mean_element_theory, "SGP4");

    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert_eq!(tle.norad_cat_id, Some(25544));
    assert_eq!(tle.classification_type.as_deref(), Some("U"));
    assert_eq!(tle.ephemeris_type, Some(0));
    assert_eq!(tle.element_set_no.as_ref().map(|v| v.value), Some(999));
    assert_eq!(tle.rev_at_epoch, Some(25984));
    assert!(tle.mean_motion_ddot.is_some());
    assert!(tle.bstar.is_some());
    omm.validate().expect("generated OMM should validate");
}

#[test]
fn tle_roundtrip_with_options() {
    let line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995";
    let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";
    let options = TleToOmmOptions {
        object_name: Some("ISS (ZARYA)".to_string()),
        object_id: None,
        originator: Some("18 SPCS".to_string()),
        message_id: None,
        creation_date: Some("2021-01-01T00:00:00".parse().unwrap()),
    };

    let omm = Omm::from_tle_lines_with_options(line1, line2, &options)
        .expect("failed to parse TLE lines with options");
    assert_eq!(omm.header.creation_date.as_str(), "2021-01-01T00:00:00");
    assert_eq!(
        omm.body.segment.data.mean_elements.epoch.as_str(),
        "2020-12-13T16:36:04.502592"
    );
    let (line1_out, line2_out) = omm.to_tle_lines().expect("failed to regenerate TLE");
    assert_eq!(line1_out, line1);
    assert_eq!(line2_out, line2);
}

#[test]
fn to_tle_lines_accepts_unknown_object_id() {
    let kvn = r#"CCSDS_OMM_VERS = 2.0
CREATION_DATE = 2020-12-13T17:26:09
ORIGINATOR = 18 SPCS
OBJECT_NAME = ISS (ZARYA)
OBJECT_ID = UNKNOWN
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP4
EPOCH = 2020-12-13T16:36:04.502592
MEAN_MOTION = 15.49181153 [rev/day]
ECCENTRICITY = 0.00017790
INCLINATION = 51.6444 [deg]
RA_OF_ASC_NODE = 180.2777 [deg]
ARG_OF_PERICENTER = 128.5985 [deg]
MEAN_ANOMALY = 350.1361 [deg]
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 25544
ELEMENT_SET_NO = 999
REV_AT_EPOCH = 25984
BSTAR = 0.00002412400000 [1/ER]
MEAN_MOTION_DOT = 0.00000888 [rev/day**2]
MEAN_MOTION_DDOT = 0.0000000000000 [rev/day**3]
"#;
    let omm = Omm::from_kvn(kvn).expect("failed to parse OMM");
    let (line1, line2) = omm.to_tle_lines().expect("failed to generate TLE");
    assert_eq!(line1[9..17].to_string(), "        ");
    assert!(line1.starts_with("1 25544U"));
    assert!(line2.starts_with("2 25544"));
}

#[test]
fn to_tle_lines_accepts_sgp_slash_sgp4() {
    let kvn = r#"CCSDS_OMM_VERS = 3.0
CREATION_DATE = 2020-065T16:00:00
ORIGINATOR = NOAA
MESSAGE_ID = OMM 202013719185
OBJECT_NAME = GOES 9
OBJECT_ID = 1995-025A
CENTER_NAME = EARTH
REF_FRAME = TEME
TIME_SYSTEM = UTC
MEAN_ELEMENT_THEORY = SGP/SGP4
EPOCH = 2020-064T10:34:41.4264
MEAN_MOTION = 1.00273272
ECCENTRICITY = 0.0005013
INCLINATION = 3.0539
RA_OF_ASC_NODE = 81.7939
ARG_OF_PERICENTER = 249.2363
MEAN_ANOMALY = 150.1602
GM = 398600.8
EPHEMERIS_TYPE = 0
CLASSIFICATION_TYPE = U
NORAD_CAT_ID = 23581
ELEMENT_SET_NO = 0925
REV_AT_EPOCH = 4316
BSTAR = 0.0001
MEAN_MOTION_DOT = -0.00000113
MEAN_MOTION_DDOT = 0.0
"#;
    let omm = Omm::from_kvn(kvn).expect("failed to parse OMM");
    let (line1, line2) = omm.to_tle_lines().expect("failed to generate TLE");
    assert!(line1.starts_with("1 23581U 95025A"));
    assert!(line2.starts_with("2 23581"));
}

#[test]
fn from_tle_lines_rejects_non_upper_launch_piece() {
    let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";

    // Lowercase piece ('a')
    let line1_lower = "1 25544U 98067a   20348.69171878  .00000888  00000-0  24124-4 0  9995";
    crate::common::assert_validation_field(
        &Omm::from_tle_lines(line1_lower, line2).unwrap_err(),
        "LAUNCH_PIECE",
    );

    // Numeric piece ('1') with corrected checksum
    let line1_digit = "1 25544U 980671   20348.69171878  .00000888  00000-0  24124-4 0  9996";
    crate::common::assert_validation_field(
        &Omm::from_tle_lines(line1_digit, line2).unwrap_err(),
        "LAUNCH_PIECE",
    );
}

#[test]
fn from_tle_lines_accepts_space_padded_sat_number() {
    let line1 = "1    47U 60007C   26036.27771152  .00000811  00000-0  19320-3 0  9991";
    let line2 = "2    47  66.6644  41.0753 0225093 235.9156 122.0408 14.45156735431744";

    let omm = Omm::from_tle_lines(line1, line2).expect("failed to parse space-padded sat num");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert_eq!(tle.norad_cat_id, Some(47));

    // Output is canonicalized to 5-digit satellite number.
    let (out1, out2) = omm.to_tle_lines().expect("failed to regenerate TLE");
    assert!(out1.starts_with("1 00047U"));
    assert!(out2.starts_with("2 00047"));
}

#[test]
fn from_tle_lines_accepts_alpha5_sat_number() {
    let line1 = "1 T0330U          26034.73987184  .00001758  00000-0  31883-2 0  9994";
    let line2 = "2 T0330 100.3052 214.4660 0046156 188.8072 171.2250 13.42128068254585";

    let omm = Omm::from_tle_lines(line1, line2).expect("failed to parse Alpha-5 sat num");
    let tle = omm.body.segment.data.tle_parameters.as_ref().unwrap();
    assert_eq!(tle.norad_cat_id, Some(270330));
    assert_eq!(omm.body.segment.metadata.object_id, "UNKNOWN");

    let (out1, out2) = omm.to_tle_lines().expect("failed to regenerate TLE");
    assert_eq!(out1, line1);
    assert_eq!(out2, line2);
}

#[test]
fn from_tle_lines_accepts_blank_launch_designator() {
    let line1 = "1 81052U          26035.58850631 +.00006010 +00000+0 +15745-2 0  9998";
    let line2 = "2 81052  65.7690 253.2830 0562908 281.9235  71.9278 13.83808833607769";

    let omm = Omm::from_tle_lines(line1, line2).expect("failed to parse blank launch designator");
    assert_eq!(omm.body.segment.metadata.object_id, "UNKNOWN");
    let (out1, _out2) = omm.to_tle_lines().expect("failed to regenerate TLE");
    assert_eq!(out1[9..17].to_string(), "        ");
}

#[test]
fn from_tle_lines_rejects_non_ascii_without_panic() {
    let valid_line1 = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995";
    let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";

    // Keep 69-byte length while injecting a non-ASCII multi-byte character.
    let mut line1 = format!("{}é{}", &valid_line1[..8], &valid_line1[9..]);
    line1.pop();
    assert_eq!(line1.len(), 69);
    assert!(!line1.is_ascii());

    let err = Omm::from_tle_lines(&line1, line2).expect_err("non-ASCII TLE should fail");
    assert!(err
        .to_string()
        .contains("ASCII-only TLE line with fixed-width columns"));
}

#[test]
fn from_tle_lines_accepts_missing_checksums() {
    let line1_no_checksum = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  999";
    let line2_no_checksum = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.4918115325984";
    assert_eq!(line1_no_checksum.len(), 68);
    assert_eq!(line2_no_checksum.len(), 68);

    let omm = Omm::from_tle_lines(line1_no_checksum, line2_no_checksum)
        .expect("missing-checksum TLE should parse");
    let (line1, line2) = omm.to_tle_lines().expect("failed to regenerate TLE");
    assert_eq!(
        line1,
        "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  9995"
    );
    assert_eq!(
        line2,
        "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845"
    );
}

#[test]
fn from_tle_lines_rejects_invalid_length() {
    let line1_short = "1 25544U 98067A   20348.69171878  .00000888  00000-0  24124-4 0  99";
    let line2 = "2 25544  51.6444 180.2777 0001779 128.5985 350.1361 15.49181153259845";
    let err = Omm::from_tle_lines(line1_short, line2).expect_err("short TLE must fail");
    assert!(err
        .to_string()
        .contains("exactly 68 (no checksum) or 69 (with checksum) characters"));
}

#[test]
fn rev_per_day_units_display_all() {
    assert_eq!(format!("{}", RevPerDayUnits::RevPerDay), "rev/day");
    assert_eq!(format!("{}", RevPerDay2Units::RevPerDay2), "rev/day**2");
    assert_eq!(format!("{}", RevPerDay3Units::RevPerDay3), "rev/day**3");
}
