use crate::common::{assert_rejects, validate_xml};
use crate::{KVN, KVN_WITH_COVARIANCE};
use ccsds_ndm::common::OdmHeader;
use ccsds_ndm::messages::omm::{
    MeanElements, MeanMotion, MeanMotionDot, Omm, OmmBody, OmmData, OmmMetadata, OmmSegment,
    TleParameters,
};
use ccsds_ndm::types::{Angle, ElementSetNo, Inclination, M2kg, NonNegativeDouble};
use ccsds_ndm::Ndm;
/// `elementSetNoType` restricts to `[0, 9999]` in both bundled OMM schemas, but `ElementSetNo`
/// only enforces that in its constructor and the value field is public. Before this was routed
/// through the root, `ELEMENT_SET_NO = 100000` produced XML that `xmllint` rejected with
/// `[facet 'maxInclusive']`.
#[test]
fn omm_validation_enforces_the_element_set_number_range() {
    let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
    omm.body
        .segment
        .data
        .tle_parameters
        .as_mut()
        .expect("fixture has TLE parameters")
        .element_set_no = Some(ElementSetNo { value: 100_000 });

    assert_rejects(&omm, "ELEMENT_SET_NO");

    // The accepted boundary must still reach the reference schema.
    omm.body
        .segment
        .data
        .tle_parameters
        .as_mut()
        .unwrap()
        .element_set_no = Some(ElementSetNo { value: 9999 });
    omm.validate().expect("9999 is the inclusive maximum");
    validate_xml("OMM ELEMENT_SET_NO boundary", &omm.to_xml().unwrap());
}

/// Every OMM value the schema types as a double must be a real number before generation runs.
///
/// The schema range facets are comparisons, and comparisons against NaN are false, so a range
/// check alone lets NaN through to the output document.
#[test]
fn omm_generation_rejects_non_finite_values_in_every_numeric_block() {
    /// A named mutation that puts a non-finite value into one numeric block.
    type NonFiniteCase = (&'static str, fn(&mut Omm));

    let cases: [NonFiniteCase; 6] = [
        ("ECCENTRICITY", |omm| {
            omm.body.segment.data.mean_elements.eccentricity.value = f64::NAN
        }),
        ("MEAN_MOTION", |omm| {
            omm.body
                .segment
                .data
                .mean_elements
                .mean_motion
                .as_mut()
                .expect("fixture uses MEAN_MOTION")
                .value = f64::NAN
        }),
        ("GM", |omm| {
            omm.body
                .segment
                .data
                .mean_elements
                .gm
                .as_mut()
                .expect("fixture has GM")
                .value = f64::NAN
        }),
        ("BSTAR", |omm| {
            omm.body
                .segment
                .data
                .tle_parameters
                .as_mut()
                .expect("fixture has TLE parameters")
                .bstar
                .as_mut()
                .expect("fixture has BSTAR")
                .value = f64::NAN
        }),
        ("MEAN_MOTION_DOT", |omm| {
            omm.body
                .segment
                .data
                .tle_parameters
                .as_mut()
                .expect("fixture has TLE parameters")
                .mean_motion_dot
                .value = f64::NAN
        }),
        ("CX_X", |omm| {
            omm.body
                .segment
                .data
                .covariance_matrix
                .as_mut()
                .expect("fixture has a covariance matrix")
                .cx_x
                .value = f64::NAN
        }),
    ];

    for (label, mutate) in cases {
        let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
        mutate(&mut omm);
        assert_rejects(&omm, label);
    }
}

/// `inclinationType` narrows `angleRange` to `[0, 180]`, which the typed wrapper only enforces
/// through its constructor. Validation has to restate it for models that reach the field directly.
#[test]
fn omm_validation_enforces_the_inclination_range() {
    let mut omm = Omm::from_kvn(KVN).expect("fixture should parse");
    omm.body.segment.data.mean_elements.inclination.angle.value = 190.0;

    let error = omm.to_xml().expect_err("190 degrees is outside [0, 180]");
    assert_eq!(error.code(), Some("validation.out_of_range"));
}

/// The shared ODM covariance and state-vector writers must spell numbers the way ODM 7.7.1
/// requires, so that generated KVN reparses.
#[test]
fn omm_kvn_generation_spells_numbers_as_ccsds_numbers() {
    let mut omm = Omm::from_kvn(KVN_WITH_COVARIANCE).expect("fixture should parse");
    {
        let covariance = omm
            .body
            .segment
            .data
            .covariance_matrix
            .as_mut()
            .expect("fixture has a covariance matrix");
        covariance.cx_x.value = 1e-9;
        covariance.cy_x.value = 0.1 + 0.2;
        covariance.cy_y.value = 1.234_567_890_123_456_7;
    }

    let kvn = omm.to_kvn().expect("finite values should generate");
    assert!(kvn.contains("CX_X                 = 1.0e-9\n"), "{kvn}");
    assert!(kvn.contains("CY_X                 = 3.0e-1\n"), "{kvn}");
    assert!(
        kvn.contains("CY_Y                 = 1.234567890123457e0\n"),
        "{kvn}"
    );
    Omm::from_kvn(&kvn).expect("generated KVN should reparse");
}

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

#[test]
fn kvn_preserves_sgp4_xp_drag_and_solar_radiation_terms() {
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
    let reparsed = Omm::from_kvn(&kvn).unwrap();
    assert_eq!(
        reparsed.body.segment.data.tle_parameters,
        omm.body.segment.data.tle_parameters
    );
}

#[test]
fn omm_rejects_kvn_numbers_it_could_not_spell_back() {
    // The schema types these fields as plain doubles, so finiteness and range checks let through
    // magnitudes whose shortest round-tripping spelling blows past the 254-character line limit.
    let mut message = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    message.body.segment.data.mean_elements.eccentricity.value = f64::MAX;

    let error = message.to_kvn().unwrap_err().to_string();
    assert!(
        error.contains("representable CCSDS number"),
        "unexpected error: {error}"
    );
}

#[test]
fn omm_kvn_numbers_use_odm_spellings_in_every_block() {
    let mut message = Omm::from_kvn(include_str!("../../data/kvn/omm_g7.kvn")).unwrap();
    let data = &mut message.body.segment.data;
    data.mean_elements.eccentricity.value = 0.1 + 0.2;
    data.mean_elements.mean_motion.as_mut().unwrap().value = 0.1 + 0.2;
    let tle = data.tle_parameters.as_mut().unwrap();
    tle.mean_motion_dot.value = 0.1 + 0.2;

    let kvn = message.to_kvn().unwrap();
    assert!(
        !kvn.contains("0.30000000000000004"),
        "Display spelling leaked into generated KVN:\n{kvn}"
    );
    for key in ["ECCENTRICITY", "MEAN_MOTION ", "MEAN_MOTION_DOT"] {
        let line = kvn
            .lines()
            .find(|line| line.starts_with(key))
            .unwrap_or_else(|| panic!("missing {key}"));
        assert!(
            !line.contains("0000000000000"),
            "unexpected {key} line: {line}"
        );
        assert!(line.len() <= 254, "{key} line exceeds 254 characters");
    }
}
