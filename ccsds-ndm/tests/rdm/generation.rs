use crate::common::{assert_rejects, mutated, validate_xml};
use crate::{KVN, XML};
use ccsds_ndm::common::{GroundImpactParameters, OdParameters, RdmSpacecraftParameters};
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::types::{
    Area, BallisticCoeff, DayInterval, Distance, LatitudeRequired, LongitudeRequired, Mass, Ms2,
    NonNegativeDouble, Percentage, PercentageRequired, PositionUnits, PositiveInteger, Probability,
};
use ccsds_ndm::{Ndm, Validate};
#[test]
fn rdm_kvn_preserves_each_logical_blocks_comments() {
    let mut message = Rdm::from_xml(XML).unwrap();
    // KVN COMMENT lexical whitespace is a separator, not message content. Normalize the single
    // leading space present in the Annex C XML sample before comparing semantic models.
    for comment in &mut message
        .body
        .segment
        .data
        .covariance_matrix
        .as_mut()
        .unwrap()
        .comment
    {
        *comment = comment.trim().to_string();
    }
    let kvn = message.to_kvn().unwrap();
    let reparsed = Rdm::from_kvn(&kvn).unwrap();
    assert_eq!(reparsed, message);

    let data = &reparsed.body.segment.data;
    assert_eq!(
        data.ground_impact_parameters.as_ref().unwrap().comment,
        ["Short term re-entry prediction results"]
    );
    assert_eq!(
        data.state_vector.as_ref().unwrap().comment,
        ["State vector at the last OD epoch"]
    );
    assert_eq!(data.covariance_matrix.as_ref().unwrap().comment.len(), 1);
    assert_eq!(
        data.spacecraft_parameters.as_ref().unwrap().comment.len(),
        1
    );
    assert_eq!(data.od_parameters.as_ref().unwrap().comment.len(), 1);
}

#[test]
fn rdm_kvn_rejects_an_xml_only_outer_data_comment_before_writing() {
    let mut message = Rdm::from_xml(XML).unwrap();
    message.body.segment.data.comment.push("outer".into());
    crate::common::assert_validation_field(&message.to_kvn().unwrap_err(), "data-level COMMENT");
    let mut output = Vec::new();
    crate::common::assert_validation_field(
        &message.write_kvn_to(&mut output).unwrap_err(),
        "data-level COMMENT",
    );
    assert!(output.is_empty());
}

#[test]
fn orbit_lifetime_rule_is_enforced_at_every_public_boundary() {
    for value in [-1.0, 0.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message
            .body
            .segment
            .data
            .atmospheric_reentry_parameters
            .orbit_lifetime
            .value = value;

        let error = message.validate().unwrap_err();
        assert_eq!(
            error.field_path().as_deref(),
            Some("body.segment.data.atmospheric_reentry_parameters.orbit_lifetime")
        );
        assert_rejects(&message, "ORBIT_LIFETIME");
    }

    for value in ["-1", "0", "NaN", "INF", "-INF"] {
        let kvn = mutated(
            KVN,
            "ORBIT_LIFETIME = 5.5 [d]",
            &format!("ORBIT_LIFETIME = {value} [d]"),
        );
        let error = Rdm::from_kvn(&kvn).unwrap_err();
        if matches!(value, "-1" | "0") {
            crate::common::assert_validation_field(&error, "DayIntervalRequired");
        } else {
            assert_eq!(error.code(), Some("parse.kvn.syntax"), "{value}: {error}");
            assert!(
                crate::common::kvn_parse_error(&error)
                    .unwrap()
                    .contexts
                    .contains(&"Invalid float"),
                "{error}"
            );
        }

        let xml = mutated(
            XML,
            "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>",
            &format!("<ORBIT_LIFETIME units=\"d\">{value}</ORBIT_LIFETIME>"),
        );
        crate::common::assert_validation_field(&Rdm::from_xml(&xml).unwrap_err(), "ORBIT_LIFETIME");
    }
}

fn complete_ground_impact_parameters() -> GroundImpactParameters {
    let probability = || Some(Probability::new(0.5).unwrap());
    let confidence = |value| Some(PercentageRequired::new(value).unwrap());
    let lon = || Some(LongitudeRequired::new(1.0).unwrap());
    let lat = || Some(LatitudeRequired::new(1.0).unwrap());
    let cross_track = || Some(Distance::new(1.0, Some(PositionUnits::Km)));
    GroundImpactParameters {
        probability_of_impact: probability(),
        probability_of_burn_up: probability(),
        probability_of_break_up: probability(),
        probability_of_land_impact: probability(),
        probability_of_casualty: probability(),
        impact_ref_frame: Some("ITRF".into()),
        nominal_impact_lon: lon(),
        nominal_impact_lat: lat(),
        impact_1_confidence: confidence(50.0),
        impact_1_start_lon: lon(),
        impact_1_start_lat: lat(),
        impact_1_stop_lon: lon(),
        impact_1_stop_lat: lat(),
        impact_1_cross_track: cross_track(),
        impact_2_confidence: confidence(75.0),
        impact_2_start_lon: lon(),
        impact_2_start_lat: lat(),
        impact_2_stop_lon: lon(),
        impact_2_stop_lat: lat(),
        impact_2_cross_track: cross_track(),
        impact_3_confidence: confidence(95.0),
        impact_3_start_lon: lon(),
        impact_3_start_lat: lat(),
        impact_3_stop_lon: lon(),
        impact_3_stop_lat: lat(),
        impact_3_cross_track: cross_track(),
        ..Default::default()
    }
}

#[test]
fn edited_ground_impact_values_are_revalidated_before_output() {
    for field in [
        "PROBABILITY_OF_IMPACT",
        "PROBABILITY_OF_BURN_UP",
        "PROBABILITY_OF_BREAK_UP",
        "PROBABILITY_OF_LAND_IMPACT",
        "PROBABILITY_OF_CASUALTY",
        "NOMINAL_IMPACT_LON",
        "NOMINAL_IMPACT_LAT",
        "IMPACT_1_CONFIDENCE",
        "IMPACT_2_CONFIDENCE",
        "IMPACT_3_CONFIDENCE",
        "IMPACT_1_START_LON",
        "IMPACT_1_STOP_LON",
        "IMPACT_2_START_LON",
        "IMPACT_2_STOP_LON",
        "IMPACT_3_START_LON",
        "IMPACT_3_STOP_LON",
        "IMPACT_1_START_LAT",
        "IMPACT_1_STOP_LAT",
        "IMPACT_2_START_LAT",
        "IMPACT_2_STOP_LAT",
        "IMPACT_3_START_LAT",
        "IMPACT_3_STOP_LAT",
        "IMPACT_1_CROSS_TRACK",
        "IMPACT_2_CROSS_TRACK",
        "IMPACT_3_CROSS_TRACK",
    ] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message.body.segment.data.ground_impact_parameters =
            Some(complete_ground_impact_parameters());
        let parameters = message
            .body
            .segment
            .data
            .ground_impact_parameters
            .as_mut()
            .unwrap();
        let target = match field {
            "PROBABILITY_OF_IMPACT" => {
                &mut parameters.probability_of_impact.as_mut().unwrap().value
            }
            "PROBABILITY_OF_BURN_UP" => {
                &mut parameters.probability_of_burn_up.as_mut().unwrap().value
            }
            "PROBABILITY_OF_BREAK_UP" => {
                &mut parameters.probability_of_break_up.as_mut().unwrap().value
            }
            "PROBABILITY_OF_LAND_IMPACT" => {
                &mut parameters
                    .probability_of_land_impact
                    .as_mut()
                    .unwrap()
                    .value
            }
            "PROBABILITY_OF_CASUALTY" => {
                &mut parameters.probability_of_casualty.as_mut().unwrap().value
            }
            "NOMINAL_IMPACT_LON" => &mut parameters.nominal_impact_lon.as_mut().unwrap().value,
            "NOMINAL_IMPACT_LAT" => &mut parameters.nominal_impact_lat.as_mut().unwrap().value,
            "IMPACT_1_CONFIDENCE" => &mut parameters.impact_1_confidence.as_mut().unwrap().value,
            "IMPACT_2_CONFIDENCE" => &mut parameters.impact_2_confidence.as_mut().unwrap().value,
            "IMPACT_3_CONFIDENCE" => &mut parameters.impact_3_confidence.as_mut().unwrap().value,
            "IMPACT_1_START_LON" => &mut parameters.impact_1_start_lon.as_mut().unwrap().value,
            "IMPACT_1_STOP_LON" => &mut parameters.impact_1_stop_lon.as_mut().unwrap().value,
            "IMPACT_2_START_LON" => &mut parameters.impact_2_start_lon.as_mut().unwrap().value,
            "IMPACT_2_STOP_LON" => &mut parameters.impact_2_stop_lon.as_mut().unwrap().value,
            "IMPACT_3_START_LON" => &mut parameters.impact_3_start_lon.as_mut().unwrap().value,
            "IMPACT_3_STOP_LON" => &mut parameters.impact_3_stop_lon.as_mut().unwrap().value,
            "IMPACT_1_START_LAT" => &mut parameters.impact_1_start_lat.as_mut().unwrap().value,
            "IMPACT_1_STOP_LAT" => &mut parameters.impact_1_stop_lat.as_mut().unwrap().value,
            "IMPACT_2_START_LAT" => &mut parameters.impact_2_start_lat.as_mut().unwrap().value,
            "IMPACT_2_STOP_LAT" => &mut parameters.impact_2_stop_lat.as_mut().unwrap().value,
            "IMPACT_3_START_LAT" => &mut parameters.impact_3_start_lat.as_mut().unwrap().value,
            "IMPACT_3_STOP_LAT" => &mut parameters.impact_3_stop_lat.as_mut().unwrap().value,
            "IMPACT_1_CROSS_TRACK" => &mut parameters.impact_1_cross_track.as_mut().unwrap().value,
            "IMPACT_2_CROSS_TRACK" => &mut parameters.impact_2_cross_track.as_mut().unwrap().value,
            _ => &mut parameters.impact_3_cross_track.as_mut().unwrap().value,
        };
        *target = if field.contains("LAT") {
            91.0
        } else if field.contains("LON") {
            181.0
        } else if field.contains("CROSS_TRACK") {
            f64::INFINITY
        } else if field.contains("CONFIDENCE") {
            101.0
        } else {
            2.0
        };
        assert_rejects(&message, field);
    }
}

#[test]
fn ground_impact_dependencies_and_confidence_order_are_enforced() {
    let mut cases = Vec::new();

    let mut missing_frame = complete_ground_impact_parameters();
    missing_frame.impact_ref_frame = None;
    cases.push((missing_frame, "IMPACT_REF_FRAME"));

    let mut partial_interval = complete_ground_impact_parameters();
    partial_interval.impact_1_stop_lat = None;
    cases.push((partial_interval, "IMPACT_1_STOP_LAT"));

    let mut skipped_interval = complete_ground_impact_parameters();
    skipped_interval.impact_1_confidence = None;
    skipped_interval.impact_1_start_lon = None;
    skipped_interval.impact_1_start_lat = None;
    skipped_interval.impact_1_stop_lon = None;
    skipped_interval.impact_1_stop_lat = None;
    skipped_interval.impact_1_cross_track = None;
    cases.push((skipped_interval, "IMPACT_1_*"));

    let mut unordered = complete_ground_impact_parameters();
    unordered.impact_2_confidence.as_mut().unwrap().value = 50.0;
    cases.push((unordered, "IMPACT_*_CONFIDENCE"));

    for (parameters, field) in cases {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message.body.segment.data.ground_impact_parameters = Some(parameters);
        assert_rejects(&message, field);
    }
}

fn complete_spacecraft_parameters() -> RdmSpacecraftParameters {
    RdmSpacecraftParameters {
        wet_mass: Some(Mass::new(1.0, None).unwrap()),
        dry_mass: Some(Mass::new(1.0, None).unwrap()),
        solar_rad_area: Some(Area::new(1.0, None).unwrap()),
        solar_rad_coeff: Some(NonNegativeDouble::new(1.0).unwrap()),
        drag_area: Some(Area::new(1.0, None).unwrap()),
        drag_coeff: Some(NonNegativeDouble::new(1.0).unwrap()),
        rcs: Some(Area::new(1.0, None).unwrap()),
        ballistic_coeff: Some(BallisticCoeff::new(1.0, None)),
        thrust_acceleration: Some(Ms2::new(1.0)),
        ..Default::default()
    }
}

#[test]
fn edited_rdm_spacecraft_values_are_revalidated_before_output() {
    for field in [
        "WET_MASS",
        "DRY_MASS",
        "SOLAR_RAD_AREA",
        "SOLAR_RAD_COEFF",
        "DRAG_AREA",
        "DRAG_COEFF",
        "RCS",
        "BALLISTIC_COEFF",
        "THRUST_ACCELERATION",
    ] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message.body.segment.data.spacecraft_parameters = Some(complete_spacecraft_parameters());
        let parameters = message
            .body
            .segment
            .data
            .spacecraft_parameters
            .as_mut()
            .unwrap();
        let target = match field {
            "WET_MASS" => &mut parameters.wet_mass.as_mut().unwrap().value,
            "DRY_MASS" => &mut parameters.dry_mass.as_mut().unwrap().value,
            "SOLAR_RAD_AREA" => &mut parameters.solar_rad_area.as_mut().unwrap().value,
            "SOLAR_RAD_COEFF" => &mut parameters.solar_rad_coeff.as_mut().unwrap().value,
            "DRAG_AREA" => &mut parameters.drag_area.as_mut().unwrap().value,
            "DRAG_COEFF" => &mut parameters.drag_coeff.as_mut().unwrap().value,
            "RCS" => &mut parameters.rcs.as_mut().unwrap().value,
            "BALLISTIC_COEFF" => &mut parameters.ballistic_coeff.as_mut().unwrap().value,
            _ => &mut parameters.thrust_acceleration.as_mut().unwrap().value,
        };
        *target = if field == "THRUST_ACCELERATION" {
            f64::NAN
        } else {
            -1.0
        };
        assert_rejects(&message, field);
    }
}

fn complete_od_parameters() -> OdParameters {
    OdParameters {
        recommended_od_span: Some(DayInterval::new(1.0, None).unwrap()),
        actual_od_span: Some(DayInterval::new(1.0, None).unwrap()),
        obs_available: Some(PositiveInteger::new(1).unwrap()),
        obs_used: Some(PositiveInteger::new(1).unwrap()),
        tracks_available: Some(PositiveInteger::new(1).unwrap()),
        tracks_used: Some(PositiveInteger::new(1).unwrap()),
        residuals_accepted: Some(Percentage::new(50.0, None).unwrap()),
        weighted_rms: Some(NonNegativeDouble::new(1.0).unwrap()),
        ..Default::default()
    }
}

#[test]
fn edited_od_values_are_revalidated_before_output() {
    for field in [
        "RECOMMENDED_OD_SPAN",
        "ACTUAL_OD_SPAN",
        "RESIDUALS_ACCEPTED",
        "WEIGHTED_RMS",
    ] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message.body.segment.data.od_parameters = Some(complete_od_parameters());
        let parameters = message.body.segment.data.od_parameters.as_mut().unwrap();
        let target = match field {
            "RECOMMENDED_OD_SPAN" => &mut parameters.recommended_od_span.as_mut().unwrap().value,
            "ACTUAL_OD_SPAN" => &mut parameters.actual_od_span.as_mut().unwrap().value,
            "RESIDUALS_ACCEPTED" => &mut parameters.residuals_accepted.as_mut().unwrap().value,
            _ => &mut parameters.weighted_rms.as_mut().unwrap().value,
        };
        *target = if field == "RESIDUALS_ACCEPTED" {
            101.0
        } else {
            -1.0
        };
        assert_rejects(&message, field);
    }

    for field in [
        "OBS_AVAILABLE",
        "OBS_USED",
        "TRACKS_AVAILABLE",
        "TRACKS_USED",
    ] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message.body.segment.data.od_parameters = Some(complete_od_parameters());
        let parameters = message.body.segment.data.od_parameters.as_mut().unwrap();
        match field {
            "OBS_AVAILABLE" => parameters.obs_available.as_mut().unwrap().value = 0,
            "OBS_USED" => parameters.obs_used.as_mut().unwrap().value = 0,
            "TRACKS_AVAILABLE" => parameters.tracks_available.as_mut().unwrap().value = 0,
            _ => parameters.tracks_used.as_mut().unwrap().value = 0,
        }
        assert_rejects(&message, field);
    }
}

#[test]
fn completed_optional_blocks_accept_valid_boundaries() {
    let mut message = Rdm::from_kvn(KVN).unwrap();

    let mut ground = complete_ground_impact_parameters();
    ground.probability_of_impact.as_mut().unwrap().value = 0.0;
    ground.probability_of_burn_up.as_mut().unwrap().value = 1.0;
    ground.nominal_impact_lon.as_mut().unwrap().value = -180.0;
    ground.nominal_impact_lat.as_mut().unwrap().value = -90.0;
    ground.impact_1_confidence.as_mut().unwrap().value = 0.0;
    ground.impact_1_start_lon.as_mut().unwrap().value = 180.0;
    ground.impact_1_start_lat.as_mut().unwrap().value = 90.0;
    ground.impact_2_confidence.as_mut().unwrap().value = 50.0;
    ground.impact_3_confidence.as_mut().unwrap().value = 100.0;
    message.body.segment.data.ground_impact_parameters = Some(ground);

    let mut spacecraft = complete_spacecraft_parameters();
    spacecraft.wet_mass.as_mut().unwrap().value = 0.0;
    spacecraft.solar_rad_coeff.as_mut().unwrap().value = 0.0;
    spacecraft.thrust_acceleration.as_mut().unwrap().value = -1.0;
    message.body.segment.data.spacecraft_parameters = Some(spacecraft);

    let mut od = complete_od_parameters();
    od.recommended_od_span.as_mut().unwrap().value = 0.0;
    od.residuals_accepted.as_mut().unwrap().value = 100.0;
    od.weighted_rms.as_mut().unwrap().value = 0.0;
    message.body.segment.data.od_parameters = Some(od);

    let xml = message.to_xml().unwrap();
    validate_xml("optional block boundaries", &xml);
    assert_eq!(Rdm::from_xml(&xml).unwrap(), message);
    assert_eq!(Rdm::from_kvn(&message.to_kvn().unwrap()).unwrap(), message);

    let invalid_xml = mutated(
        &xml,
        "<NOMINAL_IMPACT_LAT units=\"deg\">-90</NOMINAL_IMPACT_LAT>",
        "<NOMINAL_IMPACT_LAT units=\"deg\">91</NOMINAL_IMPACT_LAT>",
    );
    assert_ne!(invalid_xml, xml);
    crate::common::assert_validation_field(
        &Rdm::from_xml(&invalid_xml).unwrap_err(),
        "NOMINAL_IMPACT_LAT",
    );
}

/// `OpmCovarianceMatrix::validate` is shared with OPM and OMM, which both route it. RDM checked
/// only the covariance/state-vector dependency and never the entries themselves, so it accepted
/// values the sibling families rejected. The XSD oracle cannot catch the NaN case — libxml2
/// accepts NaN against `xsd:double` — so this rule needs a direct test.
#[test]
fn rdm_routes_the_shared_covariance_validator() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut message = Rdm::from_kvn(KVN).unwrap();
        message
            .body
            .segment
            .data
            .covariance_matrix
            .as_mut()
            .expect("fixture has a covariance matrix")
            .cx_x
            .value = bad;
        assert_rejects(&message, "CX_X");
    }

    // A later entry, so the fix is not passing on the first element alone.
    let mut message = Rdm::from_kvn(KVN).unwrap();
    message
        .body
        .segment
        .data
        .covariance_matrix
        .as_mut()
        .unwrap()
        .cz_dot_z_dot
        .value = f64::NAN;
    assert_rejects(&message, "CZ_DOT_Z_DOT");
}

/// Populates the nominal-impact triple that RDM §3.5.10 requires alongside an altitude.
fn ground_impact_with_altitude(value: f64) -> Rdm {
    use ccsds_ndm::types::{AltitudeRequired, LatLonUnits, LatitudeRequired, LongitudeRequired};
    let mut message = Rdm::from_kvn(KVN).unwrap();
    let parameters = message
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_mut()
        .expect("fixture has ground impact parameters");
    parameters.impact_ref_frame = Some("ITRF2000".to_owned());
    parameters.nominal_impact_lon = Some(LongitudeRequired {
        value: 10.0,
        units: LatLonUnits::Deg,
    });
    parameters.nominal_impact_lat = Some(LatitudeRequired {
        value: 20.0,
        units: LatLonUnits::Deg,
    });
    parameters.nominal_impact_alt = Some(AltitudeRequired {
        value,
        units: ccsds_ndm::types::LengthUnits::M,
    });
    message
}

/// RDM states no range for `NOMINAL_IMPACT_ALT` and permits non-Earth body-fixed frames, while
/// the common 4.0 XSD's `altRange` is Earth-derived. The model preserves the book-valid value and
/// P3 enforces only finiteness; XML generation refuses the conversion rather than altering the
/// value to fit.
#[test]
fn nominal_impact_altitude_separates_semantics_from_xml_representability() {
    // Outside the XSD range but book-valid: kept by the model, KVN writes it, XML refuses.
    for outside in [9000.0, -431.0] {
        let message = ground_impact_with_altitude(outside);
        message.validate().expect("the book imposes no range");
        assert!(
            message.to_kvn().is_ok(),
            "KVN can represent {outside} and must not refuse it"
        );

        let error = message
            .to_xml()
            .expect_err("the 4.0 XML edition cannot represent this altitude");
        assert!(error.to_string().contains("NOMINAL_IMPACT_ALT"), "{error}");

        let mut output = Vec::new();
        crate::common::assert_validation_field(
            &message.write_xml_to(&mut output).unwrap_err(),
            "NOMINAL_IMPACT_ALT",
        );
        assert!(output.is_empty(), "streaming wrote bytes for {outside}");
    }

    // Non-finite is not a number the standard can express in either notation.
    let message = ground_impact_with_altitude(f64::NAN);
    assert_rejects(&message, "NOMINAL_IMPACT_ALT");

    // Both XSD boundaries are accepted and reach the reference schema.
    for boundary in [-430.5, 8848.0] {
        let message = ground_impact_with_altitude(boundary);
        message.validate().unwrap();
        validate_xml("RDM altitude boundary", &message.to_xml().unwrap());
    }
}

#[test]
fn kitchen_sink_roundtrip() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = COMPREHENSIVE_TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T09:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2023-01-06T19:45:33
REENTRY_WINDOW_START = 2023-01-06T11:45:33
REENTRY_WINDOW_END = 2023-01-06T22:12:56
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.75
EPOCH = 2023-01-01T09:30:12
X = 4000.000000 [km]
Y = 4000.000000 [km]
Z = 4000.000000 [km]
X_DOT = 7.000000 [km/s]
Y_DOT = 7.000000 [km/s]
Z_DOT = 7.000000 [km/s]
COV_REF_FRAME = RTN
CX_X = 0.10000 [km**2]
CY_X = 0.10000 [km**2]
CY_Y = 0.10000 [km**2]
CZ_X = 0.10000 [km**2]
CZ_Y = 0.10000 [km**2]
CZ_Z = 0.10000 [km**2]
CX_DOT_X = 0.02000 [km**2/s]
CX_DOT_Y = 0.02000 [km**2/s]
CX_DOT_Z = 0.02000 [km**2/s]
CX_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_X = 0.02000 [km**2/s]
CY_DOT_Y = 0.02000 [km**2/s]
CY_DOT_Z = 0.02000 [km**2/s]
CY_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
CZ_DOT_X = 0.02000 [km**2/s]
CZ_DOT_Y = 0.02000 [km**2/s]
CZ_DOT_Z = 0.02000 [km**2/s]
CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
WET_MASS = 3582 [kg]
DRAG_AREA = 23.3565 [m**2]
DRAG_COEFF = 2.2634
ACTUAL_OD_SPAN = 3.4554 [d]
TRACKS_AVAILABLE = 18
TRACKS_USED = 17
USER_DEFINED_TEST = VALUE
"#;
    let rdm = Rdm::from_kvn(kvn).expect("parse kvn");
    let generated = rdm.to_kvn().expect("generate kvn");
    let rdm2 = Rdm::from_kvn(&generated).expect("parse generated kvn");

    assert_eq!(rdm2, rdm);
}

#[test]
fn rdm_xml_roundtrip_minimal() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let xml = rdm.to_xml().unwrap();
    let rdm2 = Rdm::from_xml(&xml).unwrap();
    assert_eq!(rdm2, rdm);
}

#[test]
fn test_rdm_full_roundtrip_all_blocks() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = COMPREHENSIVE_TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = YES
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T09:00:00
REF_FRAME = EME2000
ORBIT_LIFETIME = 5.5 [d]
REENTRY_ALTITUDE = 80.0 [km]
NOMINAL_REENTRY_EPOCH = 2023-01-06T19:45:33
REENTRY_WINDOW_START = 2023-01-06T11:45:33
REENTRY_WINDOW_END = 2023-01-06T22:12:56
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.75
EPOCH = 2023-01-01T09:30:12
X = 4000.000000 [km]
Y = 4000.000000 [km]
Z = 4000.000000 [km]
X_DOT = 7.000000 [km/s]
Y_DOT = 7.000000 [km/s]
Z_DOT = 7.000000 [km/s]
COV_REF_FRAME = RTN
CX_X = 0.10000 [km**2]
CY_X = 0.10000 [km**2]
CY_Y = 0.10000 [km**2]
CZ_X = 0.10000 [km**2]
CZ_Y = 0.10000 [km**2]
CZ_Z = 0.10000 [km**2]
CX_DOT_X = 0.02000 [km**2/s]
CX_DOT_Y = 0.02000 [km**2/s]
CX_DOT_Z = 0.02000 [km**2/s]
CX_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_X = 0.02000 [km**2/s]
CY_DOT_Y = 0.02000 [km**2/s]
CY_DOT_Z = 0.02000 [km**2/s]
CY_DOT_X_DOT = 0.00600 [km**2/s**2]
CY_DOT_Y_DOT = 0.00600 [km**2/s**2]
CZ_DOT_X = 0.02000 [km**2/s]
CZ_DOT_Y = 0.02000 [km**2/s]
CZ_DOT_Z = 0.02000 [km**2/s]
CZ_DOT_X_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Y_DOT = 0.00400 [km**2/s**2]
CZ_DOT_Z_DOT = 0.00400 [km**2/s**2]
WET_MASS = 3582 [kg]
DRAG_AREA = 23.3565 [m**2]
DRAG_COEFF = 2.2634
ACTUAL_OD_SPAN = 3.4554 [d]
TRACKS_AVAILABLE = 18
TRACKS_USED = 17
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();

    assert!(rdm.body.segment.data.state_vector.is_some());
    assert!(rdm.body.segment.data.covariance_matrix.is_some());

    let kvn2 = rdm.to_kvn().unwrap();
    let rdm2 = Rdm::from_kvn(&kvn2).unwrap();

    assert_eq!(rdm2, rdm);
}

#[test]
fn test_rdm_basic_kvn_roundtrip() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-11-13T12:00:00
ORIGINATOR = TEST
MESSAGE_ID = RDM-001
OBJECT_NAME = TEST-SAT
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = TEST-CENTER
TIME_SYSTEM = TAI
EPOCH_TZERO = 2023-11-13T00:00:00
ORBIT_LIFETIME = 2 [d]
REENTRY_ALTITUDE = 80 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    assert_eq!(rdm.version, "1.0");
    assert_eq!(rdm.header.message_id, "RDM-001");
    assert_eq!(rdm.body.segment.metadata.object_name, "TEST-SAT");
    let kvn2 = rdm.to_kvn().unwrap();
    assert_eq!(Rdm::from_kvn(&kvn2).unwrap(), rdm);
}

#[test]
fn test_rdm_metadata_optional_fields_kvn_roundtrip() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CATALOG_NAME = CATALOG123
OBJECT_DESIGNATOR = DES456
OBJECT_TYPE = DEBRIS
OBJECT_OWNER = OWNER789
OBJECT_OPERATOR = OPERATOR012
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
REF_FRAME = TEME
REF_FRAME_EPOCH = 2023-01-01T12:00:00
EPHEMERIS_NAME = EPHEM_TEST
GRAVITY_MODEL = JGM-3: 20D 20O
ATMOSPHERIC_MODEL = JACCHIA-71
SOLAR_FLUX_PREDICTION = MEASURED
N_BODY_PERTURBATIONS = MOON,SUN,VENUS
SOLAR_RAD_PRESSURE = YES
EARTH_TIDES = NONE
INTRACK_THRUST = NO
DRAG_PARAMETERS_SOURCE = ESTIMATED
DRAG_PARAMETERS_ALTITUDE = 250.5 [km]
REENTRY_UNCERTAINTY_METHOD = COVARIANCE
REENTRY_DISINTEGRATION = BREAK-UP
IMPACT_UNCERTAINTY_METHOD = STATISTICAL
PREVIOUS_MESSAGE_ID = MSG-PREV-001
PREVIOUS_MESSAGE_EPOCH = 2022-12-25T00:00:00
NEXT_MESSAGE_EPOCH = 2023-01-08T00:00:00
ORBIT_LIFETIME = 10 [d]
REENTRY_ALTITUDE = 120 [km]
"#;
    let mut rdm = Rdm::from_kvn(kvn).unwrap();
    let kvn2 = rdm.to_kvn().unwrap();
    // KVN emits this altitude in its mandated kilometers without the optional unit label.
    rdm.body
        .segment
        .metadata
        .drag_parameters_altitude
        .as_mut()
        .unwrap()
        .units = None;
    assert_eq!(Rdm::from_kvn(&kvn2).unwrap(), rdm);
}

#[test]
fn test_rdm_ground_impact_all_probabilities() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.25
PROBABILITY_OF_BURN_UP = 0.60
PROBABILITY_OF_BREAK_UP = 0.35
PROBABILITY_OF_LAND_IMPACT = 0.15
PROBABILITY_OF_CASUALTY = 0.001
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let g = rdm
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap();
    assert!((g.probability_of_impact.as_ref().unwrap().value - 0.25).abs() < 1e-9);
    assert!((g.probability_of_burn_up.as_ref().unwrap().value - 0.60).abs() < 1e-9);
    assert!((g.probability_of_break_up.as_ref().unwrap().value - 0.35).abs() < 1e-9);
    assert!((g.probability_of_land_impact.as_ref().unwrap().value - 0.15).abs() < 1e-9);
    assert!((g.probability_of_casualty.as_ref().unwrap().value - 0.001).abs() < 1e-9);

    let kvn2 = rdm.to_kvn().unwrap();
    assert_eq!(Rdm::from_kvn(&kvn2).unwrap(), rdm);
}

#[test]
fn test_rdm_ground_impact_nominal_and_windows() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
PROBABILITY_OF_IMPACT = 0.5
NOMINAL_IMPACT_EPOCH = 2023-01-06T15:30:00
IMPACT_WINDOW_START = 2023-01-06T12:00:00
IMPACT_WINDOW_END = 2023-01-06T18:00:00
IMPACT_REF_FRAME = EFG
NOMINAL_IMPACT_LON = -120.5
NOMINAL_IMPACT_LAT = 35.2
NOMINAL_IMPACT_ALT = 0.0 [m]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let g = rdm
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap();
    assert!(g.nominal_impact_epoch.is_some());
    assert!(g.impact_window_start.is_some());
    assert!(g.impact_window_end.is_some());
    assert_eq!(g.impact_ref_frame.as_deref(), Some("EFG"));
    assert!((g.nominal_impact_lon.as_ref().unwrap().value - (-120.5)).abs() < 1e-9);
    assert!((g.nominal_impact_lat.as_ref().unwrap().value - 35.2).abs() < 1e-9);

    let kvn2 = rdm.to_kvn().unwrap();
    assert_eq!(Rdm::from_kvn(&kvn2).unwrap(), rdm);
}

#[test]
fn test_rdm_ground_impact_confidence_intervals_1() {
    let kvn = r#"CCSDS_RDM_VERS = 1.0
CREATION_DATE = 2023-01-01T00:00:00
ORIGINATOR = TEST
MESSAGE_ID = TEST-001
OBJECT_NAME = TEST
INTERNATIONAL_DESIGNATOR = 2023-001A
CONTROLLED_REENTRY = NO
CENTER_NAME = EARTH
TIME_SYSTEM = UTC
EPOCH_TZERO = 2023-01-01T00:00:00
ORBIT_LIFETIME = 5 [d]
REENTRY_ALTITUDE = 80 [km]
IMPACT_1_CONFIDENCE = 68.3 [%]
IMPACT_1_START_LON = -125.0
IMPACT_1_START_LAT = 30.0
IMPACT_1_STOP_LON = -115.0
IMPACT_1_STOP_LAT = 40.0
IMPACT_1_CROSS_TRACK = 50.0 [km]
"#;
    let rdm = Rdm::from_kvn(kvn).unwrap();
    let g = rdm
        .body
        .segment
        .data
        .ground_impact_parameters
        .as_ref()
        .unwrap();
    assert!((g.impact_1_confidence.as_ref().unwrap().value - 68.3).abs() < 1e-9);
    assert!((g.impact_1_start_lon.as_ref().unwrap().value - (-125.0)).abs() < 1e-9);
    assert!((g.impact_1_start_lat.as_ref().unwrap().value - 30.0).abs() < 1e-9);
    assert!((g.impact_1_stop_lon.as_ref().unwrap().value - (-115.0)).abs() < 1e-9);
    assert!((g.impact_1_stop_lat.as_ref().unwrap().value - 40.0).abs() < 1e-9);
    assert!((g.impact_1_cross_track.as_ref().unwrap().value - 50.0).abs() < 1e-9);

    let kvn2 = rdm.to_kvn().unwrap();
    assert_eq!(Rdm::from_kvn(&kvn2).unwrap(), rdm);
}
