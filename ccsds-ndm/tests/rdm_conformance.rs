use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ccsds_ndm::common::{GroundImpactParameters, OdParameters, RdmSpacecraftParameters};
use ccsds_ndm::messages::rdm::Rdm;
use ccsds_ndm::types::{
    Area, BallisticCoeff, DayInterval, Distance, LatitudeRequired, LongitudeRequired, Mass, Ms2,
    NonNegativeDouble, Percentage, PercentageRequired, PositionUnits, PositiveInteger, Probability,
};
use ccsds_ndm::{Ndm, Validate};
use tempfile::NamedTempFile;

const KVN: &str = include_str!("../data/kvn/rdm_c2.kvn");
const XML: &str = include_str!("../data/xml/rdm_c4.xml");

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn rdm_kvn_rejects_unknown_duplicate_reordered_and_misplaced_content() {
    let object_name = "OBJECT_NAME = SPACEOBJECT";
    let designator = "INTERNATIONAL_DESIGNATOR = 2018-099B";
    let lifetime = "ORBIT_LIFETIME = 5.5 [d]";
    for (label, source) in [
        (
            "duplicate keyword",
            KVN.replace(object_name, &format!("{object_name}\n{object_name}")),
        ),
        (
            "reordered metadata keywords",
            KVN.replace(
                &format!("{object_name}\n{designator}"),
                &format!("{designator}\n{object_name}"),
            ),
        ),
        (
            "unknown keyword",
            KVN.replace(lifetime, &format!("{lifetime}\nUNKNOWN = value")),
        ),
        (
            "comment inside a logical block",
            KVN.replace(lifetime, &format!("{lifetime}\nCOMMENT misplaced")),
        ),
        ("trailing assignment", format!("{KVN}UNKNOWN = value\n")),
        (
            "non-ASCII content",
            KVN.replace(object_name, &format!("{object_name} €")),
        ),
    ] {
        assert!(Rdm::from_kvn(&source).is_err(), "accepted {label}");
    }
}

#[test]
fn rdm_xml_rejects_unknown_nested_content_attributes_and_ordering_errors() {
    let lifetime = "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>";
    let altitude = "<REENTRY_ALTITUDE units=\"km\">80.0</REENTRY_ALTITUDE>";
    for (label, source) in [
        (
            "unknown atmospheric child",
            XML.replace(
                "<atmosphericReentryParameters>",
                "<atmosphericReentryParameters><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown covariance child",
            XML.replace(
                "<covarianceMatrix>",
                "<covarianceMatrix><UNKNOWN>1</UNKNOWN>",
            ),
        ),
        (
            "unknown container attribute",
            XML.replace(
                "<groundImpactParameters>",
                "<groundImpactParameters unexpected=\"value\">",
            ),
        ),
        (
            "unknown leaf attribute",
            XML.replace(
                lifetime,
                "<ORBIT_LIFETIME units=\"d\" unexpected=\"value\">5.5</ORBIT_LIFETIME>",
            ),
        ),
        (
            "duplicate element",
            XML.replace(lifetime, &format!("{lifetime}{lifetime}")),
        ),
        (
            "reordered elements",
            XML.replace(
                &format!("{lifetime}\n{altitude}"),
                &format!("{altitude}\n{lifetime}"),
            ),
        ),
    ] {
        assert!(Rdm::from_xml(&source).is_err(), "accepted {label}");
    }
}

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
    assert!(message.to_kvn().is_err());
    assert!(message.to_kvn().is_err());
    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
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
        assert!(message.to_kvn().is_err(), "KVN accepted {value}");
        assert!(message.to_xml().is_err(), "XML accepted {value}");

        let mut kvn = Vec::new();
        assert!(message.write_kvn_to(&mut kvn).is_err());
        assert!(kvn.is_empty(), "KVN wrote bytes for {value}");

        let mut xml = Vec::new();
        assert!(message.write_xml_to(&mut xml).is_err());
        assert!(xml.is_empty(), "XML wrote bytes for {value}");
    }

    for value in ["-1", "0", "NaN", "INF", "-INF"] {
        let kvn = KVN.replace(
            "ORBIT_LIFETIME = 5.5 [d]",
            &format!("ORBIT_LIFETIME = {value} [d]"),
        );
        assert!(Rdm::from_kvn(&kvn).is_err(), "KVN parsed {value}");

        let xml = XML.replace(
            "<ORBIT_LIFETIME units=\"d\">5.5</ORBIT_LIFETIME>",
            &format!("<ORBIT_LIFETIME units=\"d\">{value}</ORBIT_LIFETIME>"),
        );
        assert!(Rdm::from_xml(&xml).is_err(), "XML parsed {value}");
    }
}

fn assert_rdm_rejects(message: &Rdm, field: &str) {
    let error = message
        .validate()
        .expect_err("invalid edited value accepted");
    assert!(error.to_string().contains(field), "{field}: {error}");
    assert!(message.to_kvn().is_err(), "KVN accepted invalid {field}");
    assert!(message.to_xml().is_err(), "XML accepted invalid {field}");

    let mut output = Vec::new();
    assert!(message.write_kvn_to(&mut output).is_err());
    assert!(output.is_empty(), "KVN wrote bytes for invalid {field}");
    assert!(message.write_xml_to(&mut output).is_err());
    assert!(output.is_empty(), "XML wrote bytes for invalid {field}");
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
        assert_rdm_rejects(&message, field);
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
        assert_rdm_rejects(&message, field);
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
        assert_rdm_rejects(&message, field);
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
        assert_rdm_rejects(&message, field);
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
        assert_rdm_rejects(&message, field);
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

    let invalid_xml = xml.replace(
        "<NOMINAL_IMPACT_LAT units=\"deg\">-90</NOMINAL_IMPACT_LAT>",
        "<NOMINAL_IMPACT_LAT units=\"deg\">91</NOMINAL_IMPACT_LAT>",
    );
    assert_ne!(invalid_xml, xml);
    assert!(Rdm::from_xml(&invalid_xml).is_err());
}

#[test]
fn every_shipped_rdm_fixture_preserves_the_typed_model_and_generates_valid_xml() {
    for name in ["rdm_c1.kvn", "rdm_c2.kvn"] {
        let source = fs::read_to_string(repository_path(&format!("data/kvn/{name}"))).unwrap();
        let message = Rdm::from_kvn(&source).unwrap();
        let kvn = message.to_kvn().unwrap();
        assert_eq!(Rdm::from_kvn(&kvn).unwrap(), message, "{name} KVN model");
        let xml = message.to_xml().unwrap();
        assert_eq!(Rdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }

    for name in ["rdm_c3.xml", "rdm_c4.xml"] {
        let source = fs::read_to_string(repository_path(&format!("data/xml/{name}"))).unwrap();
        let message = Rdm::from_xml(&source).unwrap();
        let xml = message.to_xml().unwrap();
        assert_eq!(Rdm::from_xml(&xml).unwrap(), message, "{name} XML model");
        validate_xml(name, &xml);
    }
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
        assert_rdm_rejects(&message, "CX_X");
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
    assert_rdm_rejects(&message, "CZ_DOT_Z_DOT");
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
        assert!(message.write_xml_to(&mut output).is_err());
        assert!(output.is_empty(), "streaming wrote bytes for {outside}");
    }

    // Non-finite is not a number the standard can express in either notation.
    let message = ground_impact_with_altitude(f64::NAN);
    assert_rdm_rejects(&message, "NOMINAL_IMPACT_ALT");

    // Both XSD boundaries are accepted and reach the reference schema.
    for boundary in [-430.5, 8848.0] {
        let message = ground_impact_with_altitude(boundary);
        message.validate().unwrap();
        validate_xml("RDM altitude boundary", &message.to_xml().unwrap());
    }
}

fn validate_xml(label: &str, xml: &str) {
    let document = NamedTempFile::new().unwrap();
    fs::write(document.path(), xml).unwrap();
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(repository_path("data/xsd/ndmxml-4.0.0-master-4.0.xsd"))
        .arg(document.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{label} generated invalid XML: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
