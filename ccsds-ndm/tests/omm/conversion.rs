use ccsds_ndm::messages::omm::{Omm, TleToOmmOptions};
use ccsds_ndm::Ndm;
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
